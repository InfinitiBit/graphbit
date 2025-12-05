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
    pub retry_config: Option<crate::types::JsRetryConfig>,
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
    ///   nodeType: "Agent",
    ///   retryConfig: {
    ///     maxAttempts: 3,
    ///     initialDelayMs: 1000,
    ///     backoffMultiplier: 2.0,
    ///     maxDelayMs: 5000,
    ///     jitterFactor: 0.1,
    ///     retryableErrors: ["NetworkError", "TimeoutError"]
    ///   }
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
        
        if let Some(retry_config) = node.retry_config {
            let core_retry_config = graphbit_core::types::RetryConfig {
                max_attempts: retry_config.max_attempts,
                initial_delay_ms: retry_config.initial_delay_ms as u64,
                backoff_multiplier: retry_config.backoff_multiplier,
                max_delay_ms: retry_config.max_delay_ms as u64,
                jitter_factor: retry_config.jitter_factor,
                retryable_errors: retry_config.retryable_errors.into_iter().map(|e| match e {
                    crate::types::JsRetryableErrorType::NetworkError => graphbit_core::types::RetryableErrorType::NetworkError,
                    crate::types::JsRetryableErrorType::TimeoutError => graphbit_core::types::RetryableErrorType::TimeoutError,
                    crate::types::JsRetryableErrorType::RateLimitError => graphbit_core::types::RetryableErrorType::RateLimitError,
                    crate::types::JsRetryableErrorType::TemporaryUnavailable => graphbit_core::types::RetryableErrorType::TemporaryUnavailable,
                    crate::types::JsRetryableErrorType::InternalServerError => graphbit_core::types::RetryableErrorType::InternalServerError,
                    crate::types::JsRetryableErrorType::AuthenticationError => graphbit_core::types::RetryableErrorType::AuthenticationError,
                    crate::types::JsRetryableErrorType::ResourceConflict => graphbit_core::types::RetryableErrorType::ResourceConflict,
                    crate::types::JsRetryableErrorType::Other => graphbit_core::types::RetryableErrorType::Other,
                }).collect(),
            };
            core_node = core_node.with_retry_config(core_retry_config);
        }

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

    /// Get a node by ID
    #[napi]
    pub async fn get_node(&self, id: String) -> Result<Option<WorkflowNode>> {
        let graph = self.inner.lock().await;
        let node_id = graphbit_core::types::NodeId::from_string(&id)
            .map_err(|e| Error::from_reason(format!("Invalid node ID: {}", e)))?;
        
        if let Some(node) = graph.get_node(&node_id) {
            Ok(Some(node_to_napi(node)))
        } else {
            Ok(None)
        }
    }

    /// Get all nodes in the graph
    #[napi]
    pub async fn get_nodes(&self) -> Result<Vec<WorkflowNode>> {
        let graph = self.inner.lock().await;
        let nodes = graph.get_nodes();
        Ok(nodes.iter().map(|(_, node)| node_to_napi(node)).collect())
    }

    /// Get the topological sort of the graph
    #[napi]
    pub async fn topological_sort(&self) -> Result<Vec<String>> {
        let graph = self.inner.lock().await;
        let sorted = graph.topological_sort()
            .map_err(crate::errors::to_napi_error)?;
        Ok(sorted.iter().map(|id| id.to_string()).collect())
    }

    /// Check if the graph has cycles
    #[napi]
    pub async fn has_cycles(&self) -> Result<bool> {
        let graph = self.inner.lock().await;
        Ok(graph.has_cycles())
    }

    /// Get direct dependencies of a node
    #[napi]
    pub async fn get_dependencies(&self, node_id: String) -> Result<Vec<String>> {
        let mut graph = self.inner.lock().await;
        let id = graphbit_core::types::NodeId::from_string(&node_id)
            .map_err(|e| Error::from_reason(format!("Invalid node ID: {}", e)))?;
        
        let deps = graph.get_dependencies(&id);
        Ok(deps.iter().map(|id| id.to_string()).collect())
    }

    /// Get nodes that directly depend on a node
    #[napi]
    pub async fn get_dependents(&self, node_id: String) -> Result<Vec<String>> {
        let mut graph = self.inner.lock().await;
        let id = graphbit_core::types::NodeId::from_string(&node_id)
            .map_err(|e| Error::from_reason(format!("Invalid node ID: {}", e)))?;
        
        let deps = graph.get_dependents(&id);
        Ok(deps.iter().map(|id| id.to_string()).collect())
    }

    /// Get nodes with no incoming edges
    #[napi]
    pub async fn get_root_nodes(&self) -> Result<Vec<String>> {
        let mut graph = self.inner.lock().await;
        let roots = graph.get_root_nodes();
        Ok(roots.iter().map(|id| id.to_string()).collect())
    }

    /// Get nodes with no outgoing edges
    #[napi]
    pub async fn get_leaf_nodes(&self) -> Result<Vec<String>> {
        let mut graph = self.inner.lock().await;
        let leaves = graph.get_leaf_nodes();
        Ok(leaves.iter().map(|id| id.to_string()).collect())
    }
}

fn node_to_napi(node: &CoreWorkflowNode) -> WorkflowNode {
    let node_type = match &node.node_type {
        CoreNodeType::Agent { .. } => "Agent",
        CoreNodeType::Condition { .. } => "Condition",
        CoreNodeType::Transform { .. } => "Transform",
        CoreNodeType::Split => "Split",
        CoreNodeType::Join => "Join",
        CoreNodeType::Delay { .. } => "Delay",
        CoreNodeType::HttpRequest { .. } => "HttpRequest",
        CoreNodeType::Custom { .. } => "Custom",
        CoreNodeType::DocumentLoader { .. } => "DocumentLoader",
    };

    WorkflowNode {
        id: node.id.to_string(),
        name: node.name.clone(),
        description: node.description.clone(),
        node_type: node_type.to_string(),
        retry_config: Some(crate::types::JsRetryConfig {
            max_attempts: node.retry_config.max_attempts,
            initial_delay_ms: node.retry_config.initial_delay_ms as f64,
            backoff_multiplier: node.retry_config.backoff_multiplier,
            max_delay_ms: node.retry_config.max_delay_ms as f64,
            jitter_factor: node.retry_config.jitter_factor,
            retryable_errors: node.retry_config.retryable_errors.iter().map(|e| match e {
                graphbit_core::types::RetryableErrorType::NetworkError => crate::types::JsRetryableErrorType::NetworkError,
                graphbit_core::types::RetryableErrorType::TimeoutError => crate::types::JsRetryableErrorType::TimeoutError,
                graphbit_core::types::RetryableErrorType::RateLimitError => crate::types::JsRetryableErrorType::RateLimitError,
                graphbit_core::types::RetryableErrorType::TemporaryUnavailable => crate::types::JsRetryableErrorType::TemporaryUnavailable,
                graphbit_core::types::RetryableErrorType::InternalServerError => crate::types::JsRetryableErrorType::InternalServerError,
                graphbit_core::types::RetryableErrorType::AuthenticationError => crate::types::JsRetryableErrorType::AuthenticationError,
                graphbit_core::types::RetryableErrorType::ResourceConflict => crate::types::JsRetryableErrorType::ResourceConflict,
                graphbit_core::types::RetryableErrorType::Other => crate::types::JsRetryableErrorType::Other,
            }).collect(),
        }),
    }
}
