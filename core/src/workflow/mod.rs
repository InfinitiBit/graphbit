//! Workflow execution engine for `GraphBit`
//!
//! This module provides the main workflow execution capabilities,
//! orchestrating agents and managing the execution flow.

mod concurrent;
mod executor;
mod helpers;
mod node_execution;
pub mod template;

pub use executor::WorkflowExecutor;

use crate::errors::GraphBitResult;
use crate::graph::{WorkflowEdge, WorkflowGraph, WorkflowNode};
use crate::types::{NodeId, WorkflowId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique workflow identifier
    pub id: WorkflowId,
    /// Workflow name
    pub name: String,
    /// Workflow description
    pub description: String,
    /// The workflow graph
    pub graph: WorkflowGraph,
    /// Workflow metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: WorkflowId::new(),
            name: name.into(),
            description: description.into(),
            graph: WorkflowGraph::new(),
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Add a node to the workflow
    pub fn add_node(&mut self, node: WorkflowNode) -> GraphBitResult<NodeId> {
        let node_id = node.id.clone();
        self.graph.add_node(node)?;
        Ok(node_id)
    }

    /// Connect two nodes with an edge
    pub fn connect_nodes(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge: WorkflowEdge,
    ) -> GraphBitResult<()> {
        self.graph.add_edge(from, to, edge)
    }

    /// Validate the workflow
    pub fn validate(&self) -> GraphBitResult<()> {
        self.graph.validate()
    }

    /// Set workflow metadata
    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }
}

/// Builder for creating workflows with fluent API
pub struct WorkflowBuilder {
    workflow: Workflow,
}

impl WorkflowBuilder {
    /// Start building a new workflow
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            workflow: Workflow::new(name, ""),
        }
    }

    /// Set workflow description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.workflow.description = description.into();
        self
    }

    /// Add a node to the workflow
    pub fn add_node(mut self, node: WorkflowNode) -> GraphBitResult<(Self, NodeId)> {
        let node_id = self.workflow.add_node(node)?;
        Ok((self, node_id))
    }

    /// Connect two nodes
    pub fn connect(
        mut self,
        from: NodeId,
        to: NodeId,
        edge: WorkflowEdge,
    ) -> GraphBitResult<Self> {
        self.workflow.connect_nodes(from, to, edge)?;
        Ok(self)
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.workflow.set_metadata(key, value);
        self
    }

    /// Build the workflow
    pub fn build(self) -> GraphBitResult<Workflow> {
        self.workflow.validate()?;
        Ok(self.workflow)
    }
}
