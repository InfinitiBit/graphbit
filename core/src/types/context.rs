//! Workflow execution context and state.

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::execution::WorkflowExecutionStats;
use super::ids::{NodeId, WorkflowId};

/// Workflow execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Workflow ID
    pub workflow_id: WorkflowId,
    /// Current execution state
    pub state: WorkflowState,
    /// Shared variables accessible by all agents
    pub variables: HashMap<String, serde_json::Value>,
    /// Node outputs for automatic data flow
    pub node_outputs: HashMap<String, serde_json::Value>,
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
            node_outputs: HashMap::with_capacity(8),
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
            let duration = chrono::Utc::now().signed_duration_since(self.started_at);
            Some(duration.num_milliseconds() as u64)
        }
    }

    /// Store a node's output in the context for automatic data flow
    #[inline]
    pub fn set_node_output(&mut self, node_id: &NodeId, output: serde_json::Value) {
        self.node_outputs.insert(node_id.to_string(), output);
    }

    /// Store a node's output in the context for automatic data flow using the node name as key
    #[inline]
    pub fn set_node_output_by_name(&mut self, node_name: &str, output: serde_json::Value) {
        self.node_outputs.insert(node_name.to_string(), output);
    }

    /// Get a node's output from the context
    #[inline]
    pub fn get_node_output(&self, node_id: &str) -> Option<&serde_json::Value> {
        self.node_outputs.get(node_id)
    }

    /// Get a nested value from a node's output using dot notation
    pub fn get_nested_output(&self, reference: &str) -> Option<&serde_json::Value> {
        let parts: Vec<&str> = reference.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let node_output = self.get_node_output(parts[0])?;
        if parts.len() == 1 {
            return Some(node_output);
        }

        let mut current = node_output;
        for part in &parts[1..] {
            current = current.get(part)?;
        }
        Some(current)
    }
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            workflow_id: WorkflowId::default(),
            state: WorkflowState::Pending,
            variables: HashMap::with_capacity(16),
            node_outputs: HashMap::with_capacity(16),
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
    Running {
        /// ID of the currently executing node
        current_node: NodeId,
    },
    /// Workflow is paused
    Paused {
        /// ID of the node where execution paused
        current_node: NodeId,
        /// Reason for pausing
        reason: String,
    },
    /// Workflow completed successfully
    Completed,
    /// Workflow failed
    Failed {
        /// Error message describing the failure
        error: String,
    },
    /// Workflow was cancelled
    Cancelled,
}

impl WorkflowState {
    /// Check if the workflow is in a terminal state
    #[inline]
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed { .. } | Self::Cancelled
        )
    }

    /// Check if the workflow is currently running
    #[inline]
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    /// Check if the workflow is paused
    #[inline]
    pub fn is_paused(&self) -> bool {
        matches!(self, Self::Paused { .. })
    }
}
