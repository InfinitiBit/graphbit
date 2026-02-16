//! Workflow graph edges and edge types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An edge in the workflow graph representing data flow and dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    /// Type of the edge
    pub edge_type: EdgeType,
    /// Condition for edge traversal
    pub condition: Option<String>,
    /// Data transformation applied to values flowing through this edge
    pub transform: Option<String>,
    /// Edge metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl WorkflowEdge {
    /// Create a new data flow edge
    pub fn data_flow() -> Self {
        Self {
            edge_type: EdgeType::DataFlow,
            condition: None,
            transform: None,
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Create a new control flow edge
    pub fn control_flow() -> Self {
        Self {
            edge_type: EdgeType::ControlFlow,
            condition: None,
            transform: None,
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Create a conditional edge
    pub fn conditional(condition: impl Into<String>) -> Self {
        Self {
            edge_type: EdgeType::Conditional,
            condition: Some(condition.into()),
            transform: None,
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Add a transformation to the edge
    pub fn with_transform(mut self, transform: impl Into<String>) -> Self {
        self.transform = Some(transform.into());
        self
    }

    /// Add metadata to the edge
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Types of edges in the workflow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    /// Data flows from one node to another
    DataFlow,
    /// Control dependency (execution order)
    ControlFlow,
    /// Conditional edge (only traversed if condition is true)
    Conditional,
    /// Error handling edge
    ErrorHandling,
}
