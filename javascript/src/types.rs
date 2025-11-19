//! Core type definitions for JavaScript bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

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

/// Agent message
#[napi(object)]
pub struct AgentMessage {
    /// Message role (user, assistant, system)
    pub role: String,
    /// Message content
    pub content: String,
    /// Optional message metadata
    pub metadata: Option<String>,
}

/// Node execution result
#[napi(object)]
pub struct NodeExecutionResult {
    /// Node ID
    pub node_id: String,
    /// Execution success status
    pub success: bool,
    /// Output data (JSON string)
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: i64,
    /// Retry count
    pub retry_count: u32,
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

