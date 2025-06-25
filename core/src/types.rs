//! Core type definitions for GraphBit
//!
//! This module contains all the fundamental types used throughout the
//! GraphBit agentic workflow automation framework.

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Import error types from the errors module
use crate::errors::GraphBitResult;

// Common timeout constants to avoid magic numbers
/// Default timeout for operations (30 seconds)
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
/// Default recovery timeout for circuit breakers (1 minute)
pub const DEFAULT_RECOVERY_TIMEOUT_MS: u64 = 60_000;
/// Default failure window for circuit breakers (5 minutes)
pub const DEFAULT_FAILURE_WINDOW_MS: u64 = 300_000;

/// Unique identifier for agents
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    /// Create a new random agent ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an agent ID from a string
    /// If the string is a valid UUID, it's used directly
    /// Otherwise, a deterministic UUID is generated from the string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        // First try to parse as UUID
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self(uuid));
        }

        // If not a UUID, generate a deterministic UUID from the string
        // Using UUID v5 with a namespace to ensure deterministic generation
        let namespace = Uuid::NAMESPACE_DNS; // Different namespace for agent IDs
        let uuid = Uuid::new_v5(&namespace, s.as_bytes());
        Ok(Self(uuid))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AgentId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AgentId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for workflows
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkflowId(pub Uuid);

impl WorkflowId {
    /// Create a new random workflow ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a workflow ID from a string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for WorkflowId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WorkflowId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for workflow nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    /// Create a new random node ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a node ID from a string
    /// If the string is a valid UUID, it's used directly
    /// Otherwise, a deterministic UUID is generated from the string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        // First try to parse as UUID
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self(uuid));
        }

        // If not a UUID, generate a deterministic UUID from the string
        // Using UUID v5 with a namespace to ensure deterministic generation
        let namespace = Uuid::NAMESPACE_OID; // Standard namespace for object identifiers
        let uuid = Uuid::new_v5(&namespace, s.as_bytes());
        Ok(Self(uuid))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NodeId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Message structure for agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Unique message ID
    pub id: Uuid,
    /// ID of the sending agent
    pub sender: AgentId,
    /// ID of the receiving agent (None for broadcast)
    pub recipient: Option<AgentId>,
    /// Message content
    pub content: MessageContent,
    /// Message metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Timestamp when message was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AgentMessage {
    /// Create a new agent message
    pub fn new(sender: AgentId, recipient: Option<AgentId>, content: MessageContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add metadata to the message
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl Default for AgentMessage {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            sender: AgentId::new(),
            recipient: None,
            content: MessageContent::Text("".to_string()),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Different types of message content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    /// Plain text message
    Text(String),
    /// Structured data message
    Data(serde_json::Value),
    /// Tool call request
    ToolCall {
        tool_name: String,
        parameters: serde_json::Value,
    },
    /// Tool call response
    ToolResponse {
        tool_name: String,
        result: serde_json::Value,
        success: bool,
    },
    /// Error message
    Error {
        error_code: String,
        error_message: String,
    },
}

/// Workflow execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Workflow ID
    pub workflow_id: WorkflowId,
    /// Current execution state
    pub state: WorkflowState,
    /// Shared variables accessible by all agents
    pub variables: HashMap<String, serde_json::Value>,
    /// Execution metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Start time of the workflow execution
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// End time of the workflow execution (if completed)
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Execution statistics
    pub stats: Option<WorkflowExecutionStats>,
}

impl WorkflowContext {
    /// Create a new workflow context
    pub fn new(workflow_id: WorkflowId) -> Self {
        Self {
            workflow_id,
            state: WorkflowState::Pending,
            variables: HashMap::with_capacity(8),
            metadata: HashMap::with_capacity(4),
            started_at: chrono::Utc::now(),
            completed_at: None,
            stats: None,
        }
    }

    /// Set a variable in the context
    #[inline]
    pub fn set_variable(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
    }

    /// Get a variable from the context
    #[inline]
    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }

    /// Set metadata in the context
    #[inline]
    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    /// Mark workflow as completed
    #[inline]
    pub fn complete(&mut self) {
        self.state = WorkflowState::Completed;
        self.completed_at = Some(chrono::Utc::now());
    }

    /// Mark workflow as failed
    #[inline]
    pub fn fail(&mut self, error: String) {
        self.state = WorkflowState::Failed { error };
        self.completed_at = Some(chrono::Utc::now());
    }

    /// Set execution statistics
    #[inline]
    pub fn set_stats(&mut self, stats: WorkflowExecutionStats) {
        self.stats = Some(stats);
    }

    /// Get execution statistics
    #[inline]
    pub fn get_stats(&self) -> Option<&WorkflowExecutionStats> {
        self.stats.as_ref()
    }

    /// Calculate and return execution duration in milliseconds
    pub fn execution_duration_ms(&self) -> Option<u64> {
        if let Some(completed_at) = self.completed_at {
            let duration = completed_at.signed_duration_since(self.started_at);
            Some(duration.num_milliseconds() as u64)
        } else {
            // If not completed, return current duration
            let duration = chrono::Utc::now().signed_duration_since(self.started_at);
            Some(duration.num_milliseconds() as u64)
        }
    }
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            workflow_id: WorkflowId::default(),
            state: WorkflowState::Pending,
            variables: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(8),
            started_at: chrono::Utc::now(),
            completed_at: None,
            stats: None,
        }
    }
}

/// Workflow execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum WorkflowState {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently running
    Running { current_node: NodeId },
    /// Workflow is paused
    Paused {
        current_node: NodeId,
        reason: String,
    },
    /// Workflow completed successfully
    Completed,
    /// Workflow failed
    Failed { error: String },
    /// Workflow was cancelled
    Cancelled,
}

impl WorkflowState {
    /// Check if the workflow is in a terminal state
    #[inline]
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            WorkflowState::Completed | WorkflowState::Failed { .. } | WorkflowState::Cancelled
        )
    }

    /// Check if the workflow is currently running
    #[inline]
    pub fn is_running(&self) -> bool {
        matches!(self, WorkflowState::Running { .. })
    }

    /// Check if the workflow is paused
    #[inline]
    pub fn is_paused(&self) -> bool {
        matches!(self, WorkflowState::Paused { .. })
    }
}

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
}

impl NodeExecutionResult {
    /// Create a successful execution result
    pub fn success(output: serde_json::Value) -> Self {
        Self {
            success: true,
            output,
            error: None,
            metadata: HashMap::with_capacity(4),
            duration_ms: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            retry_count: 0,
        }
    }

    /// Create a failed execution result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: serde_json::Value::Null,
            error: Some(error),
            metadata: HashMap::with_capacity(4),
            duration_ms: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            retry_count: 0,
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set the execution duration
    #[inline]
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set the retry count
    #[inline]
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

/// Retry configuration for node execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (0 means no retries)
    pub max_attempts: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Backoff multiplier for exponential backoff (e.g., 2.0 for doubling)
    pub backoff_multiplier: f64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Jitter factor to add randomness (0.0 to 1.0)
    pub jitter_factor: f64,
    /// Types of errors that should trigger retries
    pub retryable_errors: Vec<RetryableErrorType>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: DEFAULT_TIMEOUT_MS,
            jitter_factor: 0.1,
            retryable_errors: vec![
                RetryableErrorType::NetworkError,
                RetryableErrorType::TimeoutError,
                RetryableErrorType::TemporaryUnavailable,
                RetryableErrorType::InternalServerError,
            ],
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: DEFAULT_TIMEOUT_MS,
            jitter_factor: 0.1,
            retryable_errors: vec![
                RetryableErrorType::NetworkError,
                RetryableErrorType::TimeoutError,
                RetryableErrorType::TemporaryUnavailable,
                RetryableErrorType::InternalServerError,
            ],
        }
    }

    /// Configure exponential backoff
    pub fn with_exponential_backoff(
        mut self,
        initial_delay_ms: u64,
        multiplier: f64,
        max_delay_ms: u64,
    ) -> Self {
        self.initial_delay_ms = initial_delay_ms;
        self.backoff_multiplier = multiplier;
        self.max_delay_ms = max_delay_ms;
        self
    }

    /// Set jitter factor
    #[inline]
    pub fn with_jitter(mut self, jitter_factor: f64) -> Self {
        self.jitter_factor = jitter_factor.clamp(0.0, 1.0);
        self
    }

    /// Set retryable error types
    #[inline]
    pub fn with_retryable_errors(mut self, errors: Vec<RetryableErrorType>) -> Self {
        self.retryable_errors = errors;
        self
    }

    /// Calculate delay for a given attempt with exponential backoff and jitter
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0;
        }

        let base_delay = (self.initial_delay_ms as f64
            * self.backoff_multiplier.powi(attempt as i32 - 1))
        .min(self.max_delay_ms as f64);

        // Add jitter to prevent thundering herd
        let jitter = if self.jitter_factor > 0.0 {
            let max_jitter = base_delay * self.jitter_factor;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(-max_jitter..=max_jitter)
        } else {
            0.0
        };

        ((base_delay + jitter).max(0.0) as u64).min(self.max_delay_ms)
    }

    /// Check if an error should trigger a retry
    #[inline]
    pub fn should_retry(&self, error: &crate::errors::GraphBitError, attempt: u32) -> bool {
        if attempt >= self.max_attempts {
            return false;
        }

        let error_type = RetryableErrorType::from_error(error);
        self.retryable_errors.contains(&error_type)
    }
}

/// Types of errors that can potentially be retried
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RetryableErrorType {
    /// Network connectivity issues
    NetworkError,
    /// Request timeout errors
    TimeoutError,
    /// Rate limiting from external services
    RateLimitError,
    /// Temporary service unavailability
    TemporaryUnavailable,
    /// Internal server errors (5xx)
    InternalServerError,
    /// Authentication/authorization that might be temporary
    AuthenticationError,
    /// Resource conflicts that might resolve
    ResourceConflict,
    /// All other errors (use with caution)
    Other,
}

impl RetryableErrorType {
    /// Determine retry type from error
    pub fn from_error(error: &crate::errors::GraphBitError) -> Self {
        // Simple error type classification based on error message
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("timeout") || error_str.contains("timed out") {
            Self::TimeoutError
        } else if error_str.contains("network") || error_str.contains("connection") {
            Self::NetworkError
        } else if error_str.contains("rate limit") || error_str.contains("too many requests") {
            Self::RateLimitError
        } else if error_str.contains("unavailable") || error_str.contains("service") {
            Self::TemporaryUnavailable
        } else if error_str.contains("internal server error") || error_str.contains("500") {
            Self::InternalServerError
        } else if error_str.contains("auth") || error_str.contains("unauthorized") {
            Self::AuthenticationError
        } else if error_str.contains("conflict") || error_str.contains("409") {
            Self::ResourceConflict
        } else {
            Self::Other
        }
    }
}

/// Circuit breaker configuration for error recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Time in milliseconds to wait before trying again when circuit is open
    pub recovery_timeout_ms: u64,
    /// Number of successful calls needed to close the circuit
    pub success_threshold: u32,
    /// Time window for counting failures in milliseconds
    pub failure_window_ms: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_ms: DEFAULT_RECOVERY_TIMEOUT_MS,
            success_threshold: 3,
            failure_window_ms: DEFAULT_FAILURE_WINDOW_MS,
        }
    }
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open {
        opened_at: chrono::DateTime<chrono::Utc>,
    },
    /// Circuit is half-open, testing if service has recovered
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Configuration
    pub config: CircuitBreakerConfig,
    /// Current state
    pub state: CircuitBreakerState,
    /// Failure count in current window
    pub failure_count: u32,
    /// Success count when half-open
    pub success_count: u32,
    /// Last failure time
    pub last_failure: Option<chrono::DateTime<chrono::Utc>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure: None,
        }
    }

    /// Check if a request should be allowed
    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open { opened_at } => {
                let now = chrono::Utc::now();
                let elapsed = now.signed_duration_since(opened_at).num_milliseconds() as u64;

                if elapsed >= self.config.recovery_timeout_ms {
                    self.state = CircuitBreakerState::HalfOpen;
                    self.success_count = 0;
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    /// Record a successful operation
    #[inline]
    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitBreakerState::Open { .. } => {
                // Should not happen, but reset counts
                self.failure_count = 0;
                self.success_count = 0;
            }
        }
    }

    /// Record a failed operation
    #[inline]
    pub fn record_failure(&mut self) {
        self.last_failure = Some(chrono::Utc::now());

        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitBreakerState::Open {
                        opened_at: chrono::Utc::now(),
                    };
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open {
                    opened_at: chrono::Utc::now(),
                };
                self.failure_count = 1;
                self.success_count = 0;
            }
            CircuitBreakerState::Open { .. } => {
                // Already open, no action needed
            }
        }
    }
}

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
    /// Create a high-throughput configuration
    pub fn high_throughput() -> Self {
        let mut node_type_limits = HashMap::with_capacity(8);
        node_type_limits.insert("agent".to_string(), 4);
        node_type_limits.insert("http_request".to_string(), 8);
        node_type_limits.insert("transform".to_string(), 16);
        node_type_limits.insert("condition".to_string(), 32);
        node_type_limits.insert("delay".to_string(), 1);

        // Increase per-type limits
        node_type_limits.insert("agent".to_string(), 8);
        node_type_limits.insert("http_request".to_string(), 16);
        node_type_limits.insert("transform".to_string(), 32);

        Self {
            global_max_concurrency: 32,
            node_type_limits,
        }
    }

    /// Create a low-latency configuration
    pub fn low_latency() -> Self {
        let mut node_type_limits = HashMap::with_capacity(8);
        node_type_limits.insert("agent".to_string(), 4);
        node_type_limits.insert("http_request".to_string(), 8);
        node_type_limits.insert("transform".to_string(), 16);
        node_type_limits.insert("condition".to_string(), 32);
        node_type_limits.insert("delay".to_string(), 1);

        // Lower limits but prioritize fast execution
        node_type_limits.insert("agent".to_string(), 2);
        node_type_limits.insert("http_request".to_string(), 4);
        node_type_limits.insert("transform".to_string(), 8);

        Self {
            global_max_concurrency: 8,
            node_type_limits,
        }
    }

    /// Create a memory-optimized configuration
    pub fn memory_optimized() -> Self {
        let mut node_type_limits = HashMap::with_capacity(8);
        node_type_limits.insert("agent".to_string(), 4);
        node_type_limits.insert("http_request".to_string(), 8);
        node_type_limits.insert("transform".to_string(), 16);
        node_type_limits.insert("condition".to_string(), 32);
        node_type_limits.insert("delay".to_string(), 1);

        // Conservative limits to reduce memory pressure
        node_type_limits.insert("agent".to_string(), 3);
        node_type_limits.insert("http_request".to_string(), 6);
        node_type_limits.insert("transform".to_string(), 12);

        Self {
            global_max_concurrency: 12,
            node_type_limits,
        }
    }

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
            } else {
                // At capacity, wait for notification
                wait_queue.notified().await;
            }
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
    pub fn agent_task(_agent_id: AgentId, task_id: NodeId) -> Self {
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
