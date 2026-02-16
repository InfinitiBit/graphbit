//! Workflow graph nodes and node types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::{NodeId, RetryConfig};

/// Types of workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    /// Agent execution node
    Agent {
        /// Unique identifier for the agent
        agent_id: crate::types::AgentId,
        /// Template for the prompt to send to the agent
        prompt_template: String,
    },
    /// Conditional branching node
    Condition {
        /// Boolean expression to evaluate
        expression: String,
    },
    /// Data transformation node
    Transform {
        /// Transformation logic to apply
        transformation: String,
    },
    /// Parallel execution splitter
    Split,
    /// Parallel execution joiner
    Join,
    /// Delay/wait node
    Delay {
        /// Duration to wait in seconds
        duration_seconds: u64,
    },
    /// HTTP request node
    HttpRequest {
        /// Target URL for the request
        url: String,
        /// HTTP method (GET, POST, etc.)
        method: String,
        /// HTTP headers to include
        headers: HashMap<String, String>,
    },
    /// Custom function node
    Custom {
        /// Name of the custom function to execute
        function_name: String,
    },
    /// Document loading node
    DocumentLoader {
        /// Type of document to load
        document_type: String,
        /// Path to the source document
        source_path: String,
        /// Optional encoding specification
        encoding: Option<String>,
    },
}

/// A node in the workflow graph representing a single execution unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    /// Unique identifier for the node
    pub id: NodeId,
    /// Human-readable name
    pub name: String,
    /// Description of what this node does
    pub description: String,
    /// Type of the node
    pub node_type: NodeType,
    /// Configuration for the node
    pub config: HashMap<String, serde_json::Value>,
    /// Input schema for validation
    pub input_schema: Option<serde_json::Value>,
    /// Output schema for validation
    pub output_schema: Option<serde_json::Value>,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl WorkflowNode {
    /// Create a new workflow node
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        node_type: NodeType,
    ) -> Self {
        Self {
            id: NodeId::new(),
            name: name.into(),
            description: description.into(),
            node_type,
            config: HashMap::with_capacity(8),
            input_schema: None,
            output_schema: None,
            retry_config: RetryConfig::default(),
            timeout_seconds: None,
            tags: Vec::new(),
        }
    }

    /// Set node configuration
    pub fn with_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.insert(key, value);
        self
    }

    /// Set input schema
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Set output schema
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Validate the node configuration
    pub fn validate(&self) -> GraphBitResult<()> {
        match &self.node_type {
            NodeType::Agent { agent_id, .. } => {
                if agent_id.to_string().is_empty() {
                    return Err(GraphBitError::graph(
                        "Agent node must have a valid agent_id",
                    ));
                }
            }
            NodeType::Condition { expression } => {
                if expression.is_empty() {
                    return Err(GraphBitError::graph(
                        "Condition node must have an expression",
                    ));
                }
            }
            NodeType::Transform { transformation } => {
                if transformation.is_empty() {
                    return Err(GraphBitError::graph(
                        "Transform node must have a transformation",
                    ));
                }
            }
            NodeType::DocumentLoader {
                document_type,
                source_path,
                ..
            } => {
                if document_type.is_empty() {
                    return Err(GraphBitError::graph(
                        "DocumentLoader node must have a document_type",
                    ));
                }
                if source_path.is_empty() {
                    return Err(GraphBitError::graph(
                        "DocumentLoader node must have a source_path",
                    ));
                }
                let supported_types = ["pdf", "txt", "docx", "json", "csv", "xml", "html"];
                if !supported_types.contains(&document_type.to_lowercase().as_str()) {
                    return Err(GraphBitError::graph(format!(
                        "Unsupported document type: {document_type}. Supported types: {supported_types:?}"
                    )));
                }
            }
            _ => {}
        }

        Ok(())
    }
}
