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

    /// Add a node to the graph
    ///
    /// # Arguments
    /// * `node` - The workflow node to add
    ///
    /// # Returns
    /// The node ID as a string
    ///
    /// # Example
    /// ```javascript
    /// const graph = new WorkflowGraph();
    /// const node = {
    ///   id: "node1",
    ///   name: "My Node",
    ///   description: "A test node",
    ///   nodeType: "Agent"
    /// };
    /// const nodeId = await graph.addNode(node);
    /// ```
    #[napi]
    pub async fn add_node(&self, node: WorkflowNode) -> Result<String> {
        let mut graph = self.inner.lock().await;

        // Parse the node ID as a UUID
        let node_id = graphbit_core::types::NodeId::from_string(&node.id)
            .map_err(|e| Error::from_reason(format!("Invalid node ID: {}", e)))?;

        // Determine the node type from the string
        let node_type = match node.node_type.as_str() {
            "Agent" => CoreNodeType::Agent {
                agent_id: graphbit_core::types::AgentId::new(),
                prompt_template: String::new(),
            },
            "Condition" => CoreNodeType::Condition {
                expression: String::new(),
            },
            "Transform" => CoreNodeType::Transform {
                transformation: String::new(),
            },
            "Split" => CoreNodeType::Split,
            "Join" => CoreNodeType::Join,
            "Delay" => CoreNodeType::Delay {
                duration_seconds: 0,
            },
            _ => return Err(Error::from_reason(format!("Unknown node type: {}", node.node_type))),
        };

        // Create the core WorkflowNode
        let mut core_node = CoreWorkflowNode::new(node.name, node.description, node_type);
        core_node.id = node_id.clone();

        graph.add_node(core_node)
            .map_err(crate::errors::to_napi_error)?;

        Ok(node.id)
    }

    /// Add an edge between two nodes in the graph
    ///
    /// # Arguments
    /// * `edge` - The workflow edge to add
    ///
    /// # Example
    /// ```javascript
    /// const graph = new WorkflowGraph();
    /// const edge = {
    ///   fromNode: "node1",
    ///   toNode: "node2",
    ///   condition: null
    /// };
    /// await graph.addEdge(edge);
    /// ```
    #[napi]
    pub async fn add_edge(&self, edge: WorkflowEdge) -> Result<()> {
        let mut graph = self.inner.lock().await;

        // Parse the node IDs
        let from_id = graphbit_core::types::NodeId::from_string(&edge.from_node)
            .map_err(|e| Error::from_reason(format!("Invalid from_node ID: {}", e)))?;
        let to_id = graphbit_core::types::NodeId::from_string(&edge.to_node)
            .map_err(|e| Error::from_reason(format!("Invalid to_node ID: {}", e)))?;

        // Create a data flow edge (default)
        let mut core_edge = CoreWorkflowEdge::data_flow();
        core_edge.condition = edge.condition;

        graph.add_edge(from_id, to_id, core_edge)
            .map_err(crate::errors::to_napi_error)?;

        Ok(())
    }
}

