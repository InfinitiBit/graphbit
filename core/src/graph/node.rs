use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::{AgentId, RetryConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the workflow graph representing a single execution unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    /// Unique identifier for the node
    pub id: crate::types::NodeId,
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

impl AgentNodeConfig {
    /// Create a new agent node configuration with required fields
    pub fn new(agent_id: AgentId, prompt_template: impl Into<String>) -> Self {
        Self {
            agent_id,
            prompt_template: prompt_template.into(),
            conversational_context: None,
            system_prompt_override: None,
        }
    }

    /// Add conversational context string to the configuration
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.conversational_context = Some(context.into());
        self
    }

    /// Add a system prompt override to the configuration
    pub fn with_system_prompt_override(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt_override = Some(system_prompt.into());
        self
    }
}

impl WorkflowNode {
    /// Create a new workflow node
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        node_type: NodeType,
    ) -> Self {
        Self {
            id: crate::types::NodeId::new(),
            name: name.into(),
            description: description.into(),
            node_type,
            config: HashMap::with_capacity(8), // Pre-allocate for config parameters
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
        // Validate node type specific requirements
        match &self.node_type {
            NodeType::Agent { config } => {
                if config.agent_id.to_string().is_empty() {
                    return Err(GraphBitError::graph(
                        "Agent node must have a valid agent_id",
                    ));
                }

                // Production-grade validation for prompt templates
                Self::validate_template_syntax(&config.prompt_template, "prompt_template")?;
                if let Some(ctx) = &config.conversational_context {
                    Self::validate_template_syntax(ctx, "conversational_context")?;
                }
                if let Some(sys) = &config.system_prompt_override {
                    Self::validate_template_syntax(sys, "system_prompt_override")?;
                }
            }
            NodeType::Condition { handler_id } => {
                if handler_id.trim().is_empty() {
                    return Err(GraphBitError::graph(
                        "Condition node must have a non-empty handler_id",
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
                // Validate supported document types
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

    /// Basic syntax validation for template strings (checks for balanced braces)
    fn validate_template_syntax(template: &str, field_name: &str) -> GraphBitResult<()> {
        let mut brace_level = 0;
        let bytes = template.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
                brace_level += 1;
                i += 2;
            } else if i + 1 < bytes.len() && bytes[i] == b'}' && bytes[i + 1] == b'}' {
                brace_level -= 1;
                i += 2;
                if brace_level < 0 {
                    return Err(GraphBitError::graph(format!(
                        "Invalid template syntax in {field_name}: unexpected closing braces '}}' at byte offset {i}"
                    )));
                }
            } else {
                i += 1;
            }
        }

        if brace_level != 0 {
            return Err(GraphBitError::graph(format!(
                "Invalid template syntax in {field_name}: unbalanced braces (level {brace_level})"
            )));
        }

        Ok(())
    }
}

/// Configuration for an Agent execution node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNodeConfig {
    /// Unique identifier for the agent
    pub agent_id: AgentId,
    /// Template for the prompt to send to the agent
    pub prompt_template: String,
    /// Optional conversational context template
    pub conversational_context: Option<String>,
    /// Optional system prompt override (Node-level wins over Agent-level)
    pub system_prompt_override: Option<String>,
}

/// Types of workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    /// Agent execution node
    Agent {
        /// Flatten into the outer object to maintain JSON backwards compatibility
        #[serde(flatten)]
        config: AgentNodeConfig,
    },
    /// Conditional branch: parent output and workflow snapshot are passed to a runtime-registered handler; handler returns the next node name (string).
    Condition {
        /// Opaque id matching an entry in `WorkflowExecutor`'s conditional handler map.
        handler_id: String,
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
