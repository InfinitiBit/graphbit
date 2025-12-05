//! Core type definitions for JavaScript bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

// =================================================================
// ID Types
// =================================================================

/// Unique identifier for agents
#[napi(object)]
pub struct AgentId {
    /// The UUID as a string
    pub uuid: String,
}

impl From<graphbit_core::types::AgentId> for AgentId {
    fn from(id: graphbit_core::types::AgentId) -> Self {
        Self {
            uuid: id.to_string(),
        }
    }
}

impl From<AgentId> for graphbit_core::types::AgentId {
    fn from(id: AgentId) -> Self {
        graphbit_core::types::AgentId::from_string(&id.uuid)
            .unwrap_or_else(|_| graphbit_core::types::AgentId::new())
    }
}

/// Unique identifier for workflows
#[napi(object)]
pub struct WorkflowId {
    /// The UUID as a string
    pub uuid: String,
}

impl From<graphbit_core::types::WorkflowId> for WorkflowId {
    fn from(id: graphbit_core::types::WorkflowId) -> Self {
        Self {
            uuid: id.to_string(),
        }
    }
}

impl From<WorkflowId> for graphbit_core::types::WorkflowId {
    fn from(id: WorkflowId) -> Self {
        graphbit_core::types::WorkflowId::from_string(&id.uuid)
            .unwrap_or_else(|_| graphbit_core::types::WorkflowId::new())
    }
}

/// Unique identifier for workflow nodes
#[napi(object)]
pub struct NodeId {
    /// The UUID as a string
    pub uuid: String,
}

impl From<graphbit_core::types::NodeId> for NodeId {
    fn from(id: graphbit_core::types::NodeId) -> Self {
        Self {
            uuid: id.to_string(),
        }
    }
}

impl From<NodeId> for graphbit_core::types::NodeId {
    fn from(id: NodeId) -> Self {
        graphbit_core::types::NodeId::from_string(&id.uuid)
            .unwrap_or_else(|_| graphbit_core::types::NodeId::new())
    }
}

// =================================================================
// Enums
// =================================================================

/// Workflow execution state
#[napi]
pub enum WorkflowState {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently running
    Running,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed with an error
    Failed,
    /// Workflow was cancelled
    Cancelled,
}

impl From<graphbit_core::types::WorkflowState> for WorkflowState {
    fn from(state: graphbit_core::types::WorkflowState) -> Self {
        match state {
            graphbit_core::types::WorkflowState::Pending => WorkflowState::Pending,
            graphbit_core::types::WorkflowState::Running { .. } => WorkflowState::Running,
            graphbit_core::types::WorkflowState::Paused { .. } => WorkflowState::Pending, // Map Paused to Pending for JS
            graphbit_core::types::WorkflowState::Completed => WorkflowState::Completed,
            graphbit_core::types::WorkflowState::Failed { .. } => WorkflowState::Failed,
            graphbit_core::types::WorkflowState::Cancelled => WorkflowState::Cancelled,
        }
    }
}

/// Agent capability
#[napi]
pub enum AgentCapability {
    /// Text processing capability
    TextProcessing,
    /// Data analysis capability
    DataAnalysis,
    /// Tool execution capability
    ToolExecution,
    /// Decision making capability
    DecisionMaking,
}

impl From<AgentCapability> for graphbit_core::types::AgentCapability {
    fn from(cap: AgentCapability) -> Self {
        match cap {
            AgentCapability::TextProcessing => graphbit_core::types::AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis => graphbit_core::types::AgentCapability::DataAnalysis,
            AgentCapability::ToolExecution => graphbit_core::types::AgentCapability::ToolExecution,
            AgentCapability::DecisionMaking => graphbit_core::types::AgentCapability::DecisionMaking,
        }
    }
}

impl From<graphbit_core::types::AgentCapability> for AgentCapability {
    fn from(cap: graphbit_core::types::AgentCapability) -> Self {
        match cap {
            graphbit_core::types::AgentCapability::TextProcessing => AgentCapability::TextProcessing,
            graphbit_core::types::AgentCapability::DataAnalysis => AgentCapability::DataAnalysis,
            graphbit_core::types::AgentCapability::ToolExecution => AgentCapability::ToolExecution,
            graphbit_core::types::AgentCapability::DecisionMaking => AgentCapability::DecisionMaking,
            graphbit_core::types::AgentCapability::Custom(_) => AgentCapability::TextProcessing, // Default for custom
        }
    }
}

/// Workflow execution statistics
#[napi(object)]
pub struct WorkflowExecutionStats {
    /// Total number of nodes
    pub total_nodes: i32,
    /// Number of successful nodes
    pub successful_nodes: i32,
    /// Number of failed nodes
    pub failed_nodes: i32,
    /// Average execution time per node in milliseconds
    pub avg_execution_time_ms: f64,
    /// Maximum concurrent nodes
    pub max_concurrent_nodes: i32,
    /// Total execution time in milliseconds
    pub total_execution_time_ms: f64,
    /// Peak memory usage in MB (if available)
    pub peak_memory_usage_mb: Option<f64>,
}

impl From<graphbit_core::types::WorkflowExecutionStats> for WorkflowExecutionStats {
    fn from(stats: graphbit_core::types::WorkflowExecutionStats) -> Self {
        Self {
            total_nodes: stats.total_nodes as i32,
            successful_nodes: stats.successful_nodes as i32,
            failed_nodes: stats.failed_nodes as i32,
            avg_execution_time_ms: stats.avg_execution_time_ms,
            max_concurrent_nodes: stats.max_concurrent_nodes as i32,
            total_execution_time_ms: stats.total_execution_time_ms as f64,
            peak_memory_usage_mb: stats.peak_memory_usage_mb,
        }
    }
}

/// Message content type
#[napi]
pub enum MessageContentType {
    /// Plain text content
    Text,
    /// Image content
    Image,
    /// Tool call
    ToolCall,
    /// Tool result
    ToolResult,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_state_conversion() {
        let state = WorkflowState::from(graphbit_core::types::WorkflowState::Pending);
        matches!(state, WorkflowState::Pending);
    }
}

// =================================================================
// Reliability Configuration
// =================================================================

/// Types of errors that can potentially be retried
#[napi(js_name = "RetryableErrorType")]
pub enum JsRetryableErrorType {
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

/// Retry configuration for node execution
#[napi(object, js_name = "RetryConfig")]
pub struct JsRetryConfig {
    /// Maximum number of retry attempts (0 means no retries)
    pub max_attempts: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: f64,
    /// Backoff multiplier for exponential backoff (e.g., 2.0 for doubling)
    pub backoff_multiplier: f64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: f64,
    /// Jitter factor to add randomness (0.0 to 1.0)
    pub jitter_factor: f64,
    /// Types of errors that should trigger retries
    pub retryable_errors: Vec<JsRetryableErrorType>,
}

/// Circuit breaker configuration
#[napi(object)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Time in milliseconds to wait before trying again when circuit is open
    pub recovery_timeout_ms: f64,
    /// Number of successful calls needed to close the circuit
    pub success_threshold: u32,
    /// Time window for counting failures in milliseconds
    pub failure_window_ms: f64,
}

impl From<CircuitBreakerConfig> for graphbit_core::types::CircuitBreakerConfig {
    fn from(config: CircuitBreakerConfig) -> Self {
        Self {
            failure_threshold: config.failure_threshold,
            recovery_timeout_ms: config.recovery_timeout_ms as u64,
            success_threshold: config.success_threshold,
            failure_window_ms: config.failure_window_ms as u64,
        }
    }
}

impl From<graphbit_core::types::CircuitBreakerConfig> for CircuitBreakerConfig {
    fn from(config: graphbit_core::types::CircuitBreakerConfig) -> Self {
        Self {
            failure_threshold: config.failure_threshold,
            recovery_timeout_ms: config.recovery_timeout_ms as f64,
            success_threshold: config.success_threshold,
            failure_window_ms: config.failure_window_ms as f64,
        }
    }
}

/// Circuit breaker state
#[napi]
pub enum CircuitBreakerState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service has recovered
    HalfOpen,
}

impl From<graphbit_core::types::CircuitBreakerState> for CircuitBreakerState {
    fn from(state: graphbit_core::types::CircuitBreakerState) -> Self {
        match state {
            graphbit_core::types::CircuitBreakerState::Closed => CircuitBreakerState::Closed,
            graphbit_core::types::CircuitBreakerState::Open { .. } => CircuitBreakerState::Open,
            graphbit_core::types::CircuitBreakerState::HalfOpen => CircuitBreakerState::HalfOpen,
        }
    }
}

/// Message structure for agent communication
#[napi(object)]
pub struct AgentMessage {
    /// Unique message ID
    pub id: String,
    /// ID of the sending agent
    pub sender: AgentId,
    /// ID of the receiving agent (None for broadcast)
    pub recipient: Option<AgentId>,
    /// Message content
    #[napi(ts_type = "any")]
    pub content: serde_json::Value,
    /// Message metadata
    #[napi(ts_type = "Record<string, any>")]
    pub metadata: serde_json::Value,
    /// Timestamp when message was created
    pub timestamp: String,
}

impl From<graphbit_core::types::AgentMessage> for AgentMessage {
    fn from(msg: graphbit_core::types::AgentMessage) -> Self {
        Self {
            id: msg.id.to_string(),
            sender: msg.sender.into(),
            recipient: msg.recipient.map(|id| id.into()),
            content: serde_json::to_value(&msg.content).unwrap_or(serde_json::Value::Null),
            metadata: serde_json::to_value(&msg.metadata).unwrap_or(serde_json::Value::Null),
            timestamp: msg.timestamp.to_rfc3339(),
        }
    }
}

/// Node execution result
#[napi(object)]
pub struct NodeExecutionResult {
    /// Whether the execution was successful
    pub success: bool,
    /// Output data from the node
    #[napi(ts_type = "any")]
    pub output: serde_json::Value,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Execution metadata
    #[napi(ts_type = "Record<string, any>")]
    pub metadata: serde_json::Value,
    /// Execution duration in milliseconds
    pub duration_ms: f64,
    /// Timestamp when execution started
    pub started_at: String,
    /// Timestamp when execution completed
    pub completed_at: Option<String>,
    /// Number of retries attempted
    pub retry_count: u32,
    /// ID of the node that was executed
    pub node_id: NodeId,
}

impl From<graphbit_core::types::NodeExecutionResult> for NodeExecutionResult {
    fn from(result: graphbit_core::types::NodeExecutionResult) -> Self {
        Self {
            success: result.success,
            output: result.output,
            error: result.error,
            metadata: serde_json::to_value(&result.metadata).unwrap_or(serde_json::Value::Null),
            duration_ms: result.duration_ms as f64,
            started_at: result.started_at.to_rfc3339(),
            completed_at: result.completed_at.map(|t| t.to_rfc3339()),
            retry_count: result.retry_count,
            node_id: result.node_id.into(),
        }
    }
}
