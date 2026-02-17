//! Node execution and workflow statistics types.

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ids::NodeId;

/// Agent capability types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentCapability {
    /// Text processing capability
    TextProcessing,
    /// Data analysis capability
    DataAnalysis,
    /// Tool execution capability
    ToolExecution,
    /// Decision making capability
    DecisionMaking,
    /// Custom capability
    Custom(String),
}

/// Node execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecutionResult {
    /// Whether the execution was successful
    pub success: bool,
    /// Output data from the node
    pub output: serde_json::Value,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Execution metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Timestamp when execution started
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when execution completed
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of retries attempted (if retry logic is used)
    pub retry_count: u32,
    /// ID of the node that was executed
    pub node_id: NodeId,
}

impl NodeExecutionResult {
    /// Create a successful execution result
    pub fn success(output: serde_json::Value, node_id: NodeId) -> Self {
        Self {
            success: true,
            output,
            error: None,
            metadata: HashMap::with_capacity(4),
            duration_ms: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            retry_count: 0,
            node_id,
        }
    }

    /// Create a failed execution result
    pub fn failure(error: String, node_id: NodeId) -> Self {
        Self {
            success: false,
            output: serde_json::Value::Null,
            error: Some(error),
            metadata: HashMap::with_capacity(4),
            duration_ms: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            retry_count: 0,
            node_id,
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set execution duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set retry count
    pub fn with_retry_count(mut self, retry_count: u32) -> Self {
        self.retry_count = retry_count;
        self
    }

    /// Mark the result as completed
    #[inline]
    pub fn mark_completed(mut self) -> Self {
        self.completed_at = Some(chrono::Utc::now());
        self
    }
}

impl Default for NodeExecutionResult {
    fn default() -> Self {
        Self {
            success: false,
            output: serde_json::Value::Null,
            error: None,
            metadata: HashMap::with_capacity(4),
            duration_ms: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            retry_count: 0,
            node_id: NodeId::new(),
        }
    }
}

/// Workflow execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionStats {
    /// Total number of nodes executed
    pub total_nodes: usize,
    /// Number of nodes that completed successfully
    pub successful_nodes: usize,
    /// Number of nodes that failed
    pub failed_nodes: usize,
    /// Average execution time per node in milliseconds
    pub avg_execution_time_ms: f64,
    /// Maximum concurrent nodes executed at once
    pub max_concurrent_nodes: usize,
    /// Total execution time for the entire workflow
    pub total_execution_time_ms: u64,
    /// Memory usage statistics (if available)
    pub peak_memory_usage_mb: Option<f64>,
    /// Number of semaphore acquisitions
    pub semaphore_acquisitions: u64,
    /// Average wait time for semaphore acquisition
    pub avg_semaphore_wait_ms: f64,
}
