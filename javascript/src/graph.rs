//! Graph bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::graph::{
    WorkflowGraph as CoreWorkflowGraph,
    WorkflowNode as CoreWorkflowNode,
    WorkflowEdge as CoreWorkflowEdge,
    NodeType as CoreNodeType,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Node type enumeration
#[napi]
pub enum NodeType {
    Agent,
    Condition,
    Transform,
    Split,
    Join,
    Delay,
}

impl From<NodeType> for CoreNodeType {
    fn from(nt: NodeType) -> Self {
        match nt {
            NodeType::Agent => CoreNodeType::Agent {
                agent_id: graphbit_core::types::AgentId::new(),
                prompt_template: String::new(),
            },
            NodeType::Condition => CoreNodeType::Condition {
                expression: String::new(),
            },
            NodeType::Transform => CoreNodeType::Transform {
                transformation: String::new(),
            },
            NodeType::Split => CoreNodeType::Split,
            NodeType::Join => CoreNodeType::Join,
            NodeType::Delay => CoreNodeType::Delay {
                duration_seconds: 0,
            },
        }
    }
}

/// Workflow node
#[napi(object)]
pub struct WorkflowNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub node_type: String,
}

/// Workflow edge
#[napi(object)]
pub struct WorkflowEdge {
    pub from_node: String,
    pub to_node: String,
    pub condition: Option<String>,
}

/// Workflow graph
#[napi]
pub struct WorkflowGraph {
    inner: Arc<Mutex<CoreWorkflowGraph>>,
}

#[napi]
impl WorkflowGraph {
    /// Create a new workflow graph
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreWorkflowGraph::new())),
        }
    }

    /// Get the number of nodes in the graph
    #[napi]
    pub async fn node_count(&self) -> Result<u32> {
        let graph = self.inner.lock().await;
        Ok(graph.node_count() as u32)
    }

    /// Get the number of edges in the graph
    #[napi]
    pub async fn edge_count(&self) -> Result<u32> {
        let graph = self.inner.lock().await;
        Ok(graph.edge_count() as u32)
    }

    /// Check if the graph is empty
    #[napi]
    pub async fn is_empty(&self) -> Result<bool> {
        let graph = self.inner.lock().await;
        Ok(graph.node_count() == 0)
    }

    /// Validate the graph structure
    #[napi]
    pub async fn validate(&self) -> Result<bool> {
        let graph = self.inner.lock().await;
        match graph.validate() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

