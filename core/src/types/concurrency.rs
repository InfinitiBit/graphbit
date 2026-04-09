//! Concurrency control types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::ids::NodeId;
use crate::errors::GraphBitResult;

/// Simplified configuration for basic concurrency control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Global maximum concurrent tasks
    pub global_max_concurrency: usize,
    /// Per-node-type concurrency limits
    pub node_type_limits: HashMap<String, usize>,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        let mut node_type_limits = HashMap::with_capacity(8);
        node_type_limits.insert("agent".to_string(), 4);
        node_type_limits.insert("http_request".to_string(), 8);
        node_type_limits.insert("transform".to_string(), 16);
        node_type_limits.insert("condition".to_string(), 32);
        node_type_limits.insert("delay".to_string(), 1);

        Self {
            global_max_concurrency: 16,
            node_type_limits,
        }
    }
}

impl ConcurrencyConfig {
    /// Get concurrency limit for a specific node type
    pub fn get_node_type_limit(&self, node_type: &str) -> usize {
        self.node_type_limits
            .get(node_type)
            .copied()
            .unwrap_or(self.global_max_concurrency / 4)
    }
}

/// Enhanced concurrency manager that eliminates global semaphore bottleneck
pub struct ConcurrencyManager {
    /// Per-node-type concurrency limits and current counts (using atomic counters)
    node_type_limits: Arc<RwLock<HashMap<String, NodeTypeConcurrency>>>,
    /// Configuration
    config: Arc<RwLock<ConcurrencyConfig>>,
    /// Performance statistics
    stats: Arc<RwLock<ConcurrencyStats>>,
}

/// Atomic concurrency tracking per node type
struct NodeTypeConcurrency {
    /// Maximum allowed concurrent tasks
    max_concurrent: usize,
    /// Current number of running tasks (atomic for lock-free access)
    current_count: Arc<std::sync::atomic::AtomicUsize>,
    /// Wait queue for when at capacity
    wait_queue: Arc<tokio::sync::Notify>,
}

impl ConcurrencyManager {
    /// Create a new enhanced concurrency manager
    pub fn new(config: ConcurrencyConfig) -> Self {
        let mut node_type_limits = HashMap::with_capacity(config.node_type_limits.len() + 4);

        // Pre-create concurrency tracking for known node types
        for (node_type, limit) in &config.node_type_limits {
            node_type_limits.insert(
                node_type.clone(),
                NodeTypeConcurrency {
                    max_concurrent: *limit,
                    current_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
                    wait_queue: Arc::new(tokio::sync::Notify::new()),
                },
            );
        }

        // Add default node types with dynamic limits based on global max
        let default_limit = config.global_max_concurrency / 2;
        for node_type in ["agent", "http_request", "transform", "condition"] {
            if !node_type_limits.contains_key(node_type) {
                node_type_limits.insert(
                    node_type.to_string(),
                    NodeTypeConcurrency {
                        max_concurrent: default_limit,
                        current_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
                        wait_queue: Arc::new(tokio::sync::Notify::new()),
                    },
                );
            }
        }

        Self {
            node_type_limits: Arc::new(RwLock::new(node_type_limits)),
            config: Arc::new(RwLock::new(config)),
            stats: Arc::new(RwLock::new(ConcurrencyStats::default())),
        }
    }

    /// Acquire permits for executing a task (no global semaphore bottleneck)
    pub async fn acquire_permits(
        &self,
        task_info: &TaskInfo,
    ) -> GraphBitResult<ConcurrencyPermits> {
        let start_time = std::time::Instant::now();

        // Get or create node type concurrency tracking
        let (current_count, wait_queue, max_concurrent) = {
            let config = self.config.read().await;
            let mut limits = self.node_type_limits.write().await;

            let node_concurrency = limits
                .entry(task_info.node_type.clone())
                .or_insert_with(|| {
                    let limit = config.get_node_type_limit(&task_info.node_type);
                    NodeTypeConcurrency {
                        max_concurrent: limit,
                        current_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
                        wait_queue: Arc::new(tokio::sync::Notify::new()),
                    }
                });

            (
                Arc::clone(&node_concurrency.current_count),
                Arc::clone(&node_concurrency.wait_queue),
                node_concurrency.max_concurrent,
            )
        };

        // Fast path: try to acquire without waiting
        loop {
            let current = current_count.load(std::sync::atomic::Ordering::Acquire);
            if current < max_concurrent {
                // Try to increment atomically
                match current_count.compare_exchange(
                    current,
                    current + 1,
                    std::sync::atomic::Ordering::AcqRel,
                    std::sync::atomic::Ordering::Acquire,
                ) {
                    Ok(_) => break,     // Successfully acquired
                    Err(_) => continue, // Retry - another thread modified the count
                }
            }
            wait_queue.notified().await;
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_permit_acquisitions += 1;
            stats.total_wait_time_ms += start_time.elapsed().as_millis() as u64;
            stats.current_active_tasks += 1;
            stats.peak_active_tasks = stats.peak_active_tasks.max(stats.current_active_tasks);
        }

        Ok(ConcurrencyPermits {
            stats: Arc::clone(&self.stats),
            current_count,
            wait_queue,
        })
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> ConcurrencyStats {
        self.stats.read().await.clone()
    }

    /// Get available permits for debugging
    pub async fn get_available_permits(&self) -> HashMap<String, usize> {
        let mut permits = HashMap::new();
        let limits = self.node_type_limits.read().await;

        for (node_type, concurrency) in limits.iter() {
            let current = concurrency
                .current_count
                .load(std::sync::atomic::Ordering::Acquire);
            let available = concurrency.max_concurrent.saturating_sub(current);
            permits.insert(node_type.clone(), available);
        }

        permits
    }
}

/// Simplified information about a task for concurrency control
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Type of the node being executed
    pub node_type: String,
    /// Task identifier for tracking
    pub task_id: NodeId,
}

impl TaskInfo {
    /// Create task info for an agent node
    pub fn agent_task(_agent_id: super::ids::AgentId, task_id: NodeId) -> Self {
        Self {
            node_type: "agent".to_string(),
            task_id,
        }
    }

    /// Create task info for an HTTP request node
    pub fn http_task(task_id: NodeId) -> Self {
        Self {
            node_type: "http_request".to_string(),
            task_id,
        }
    }

    /// Create task info for a transform node
    pub fn transform_task(task_id: NodeId) -> Self {
        Self {
            node_type: "transform".to_string(),
            task_id,
        }
    }

    /// Create task info for a condition node
    pub fn condition_task(task_id: NodeId) -> Self {
        Self {
            node_type: "condition".to_string(),
            task_id,
        }
    }

    /// Create task info for a delay node
    pub fn delay_task(task_id: NodeId, _duration_ms: u64) -> Self {
        Self {
            node_type: "delay".to_string(),
            task_id,
        }
    }

    /// Create task info from a node type - optimized helper
    pub fn from_node_type(node_type: &crate::graph::NodeType, task_id: &NodeId) -> Self {
        use crate::graph::NodeType;
        let type_str = match node_type {
            NodeType::Agent { .. } => "agent",
            NodeType::HttpRequest { .. } => "http_request",
            NodeType::Transform { .. } => "transform",
            NodeType::Condition { .. } => "condition",
            NodeType::Delay { .. } => "delay",
            NodeType::DocumentLoader { .. } => "document_loader",
            _ => "generic",
        };

        Self {
            node_type: type_str.to_string(),
            task_id: task_id.clone(),
        }
    }
}

/// Enhanced permits with atomic cleanup
pub struct ConcurrencyPermits {
    stats: Arc<RwLock<ConcurrencyStats>>,
    current_count: Arc<std::sync::atomic::AtomicUsize>,
    wait_queue: Arc<tokio::sync::Notify>,
}

impl Drop for ConcurrencyPermits {
    fn drop(&mut self) {
        // Atomically decrement count
        self.current_count
            .fetch_sub(1, std::sync::atomic::Ordering::AcqRel);

        // Notify waiting tasks
        self.wait_queue.notify_one();

        // Update statistics
        if let Ok(mut stats) = self.stats.try_write() {
            stats.current_active_tasks = stats.current_active_tasks.saturating_sub(1);
        }
    }
}

/// Simplified statistics for concurrency management
#[derive(Debug, Clone, Default)]
pub struct ConcurrencyStats {
    /// Total number of permit acquisitions
    pub total_permit_acquisitions: u64,
    /// Total time spent waiting for permits (milliseconds)
    pub total_wait_time_ms: u64,
    /// Current number of active tasks
    pub current_active_tasks: usize,
    /// Peak number of concurrent active tasks
    pub peak_active_tasks: usize,
    /// Number of permit acquisition failures
    pub permit_failures: u64,
    /// Average wait time per permit acquisition
    pub avg_wait_time_ms: f64,
}

impl ConcurrencyStats {
    /// Calculate average wait time
    pub fn calculate_avg_wait_time(&mut self) {
        if self.total_permit_acquisitions > 0 {
            self.avg_wait_time_ms =
                self.total_wait_time_ms as f64 / self.total_permit_acquisitions as f64;
        }
    }

    /// Get utilization percentage (0.0-100.0)
    pub fn get_utilization(&self, max_capacity: usize) -> f64 {
        if max_capacity > 0 {
            (self.current_active_tasks as f64 / max_capacity as f64) * 100.0
        } else {
            0.0
        }
    }
}