//! Workflow node for GraphBit Node.js bindings

use crate::validation::validate_non_empty_string;
use graphbit_core::{
    graph::{NodeType, WorkflowNode},
    types::AgentId,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Node {
    pub(crate) inner: WorkflowNode,
}

#[napi]
impl Node {
    /// Create an agent node
    #[napi(factory)]
    pub fn agent(
        name: String,
        prompt: String,
        agent_id: Option<String>,
        output_name: Option<String>,
    ) -> Result<Node> {
        // Validate required parameters
        validate_non_empty_string(&name, "name")?;
        validate_non_empty_string(&prompt, "prompt")?;

        let id = agent_id.unwrap_or_else(|| {
            format!(
                "agent_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            )
        });

        let mut node = WorkflowNode::new(
            name.clone(),
            format!("Agent: {}", name),
            NodeType::Agent {
                agent_id: AgentId::from_string(&id).map_err(|e| {
                    Error::new(Status::InvalidArg, format!("Invalid agent ID: {}", e))
                })?,
                prompt_template: prompt,
            },
        );

        // Store output name in metadata if provided
        if let Some(output_name) = output_name {
            node.config.insert(
                "output_name".to_string(),
                serde_json::Value::String(output_name),
            );
        }

        Ok(Node { inner: node })
    }

    /// Create a transform node
    #[napi(factory)]
    pub fn transform(name: String, transformation: String) -> Result<Node> {
        validate_non_empty_string(&name, "name")?;
        validate_non_empty_string(&transformation, "transformation")?;

        let node = WorkflowNode::new(
            name.clone(),
            format!("Transform: {}", name),
            NodeType::Transform { transformation },
        );

        Ok(Node { inner: node })
    }

    /// Create an input node
    #[napi(factory)]
    pub fn input(name: String, input_schema: Option<String>) -> Result<Node> {
        validate_non_empty_string(&name, "name")?;

        let mut node = WorkflowNode::new(
            name.clone(),
            format!("Input: {}", name),
            NodeType::Transform {
                transformation: "Input node".to_string(),
            },
        );

        // Store input schema in metadata if provided
        if let Some(schema) = input_schema {
            node.config.insert(
                "input_schema".to_string(),
                serde_json::Value::String(schema),
            );
        }

        Ok(Node { inner: node })
    }

    /// Create an output node
    #[napi(factory)]
    pub fn output(name: String, output_schema: Option<String>) -> Result<Node> {
        validate_non_empty_string(&name, "name")?;

        let mut node = WorkflowNode::new(
            name.clone(),
            format!("Output: {}", name),
            NodeType::Transform {
                transformation: "Output node".to_string(),
            },
        );

        // Store output schema in metadata if provided
        if let Some(schema) = output_schema {
            node.config.insert(
                "output_schema".to_string(),
                serde_json::Value::String(schema),
            );
        }

        Ok(Node { inner: node })
    }

    /// Get the node name
    #[napi(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Get the node description
    #[napi(getter)]
    pub fn description(&self) -> String {
        self.inner.description.clone()
    }

    /// Get the node ID as a string
    #[napi(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }
}
