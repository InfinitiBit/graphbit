//! Workflow implementation for GraphBit Node.js bindings

use super::node::Node;
use crate::errors::to_napi_error;
use graphbit_core::{graph::WorkflowEdge, types::NodeId, workflow::Workflow as CoreWorkflow};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Workflow {
    pub(crate) inner: CoreWorkflow,
}

#[napi]
impl Workflow {
    /// Create a new workflow
    #[napi(constructor)]
    pub fn new(name: String) -> Self {
        Self {
            inner: CoreWorkflow::new(name, "Node.js workflow"),
        }
    }

    /// Add a node to the workflow
    #[napi]
    pub fn add_node(&mut self, node: &Node) -> Result<String> {
        let node_id = self
            .inner
            .add_node(node.inner.clone())
            .map_err(to_napi_error)?;
        Ok(node_id.to_string())
    }

    /// Connect two nodes in the workflow
    #[napi]
    pub fn connect(&mut self, from_id: String, to_id: String) -> Result<()> {
        // First try to look up by name, if that fails try to parse as UUID
        let from_node_id = if let Some(id) = self.inner.graph.get_node_id_by_name(&from_id) {
            id
        } else if let Ok(id) = NodeId::from_string(&from_id) {
            id
        } else {
            return Err(Error::new(
                Status::InvalidArg,
                format!("Source node {} not found", from_id),
            ));
        };

        let to_node_id = if let Some(id) = self.inner.graph.get_node_id_by_name(&to_id) {
            id
        } else if let Ok(id) = NodeId::from_string(&to_id) {
            id
        } else {
            return Err(Error::new(
                Status::InvalidArg,
                format!("Target node {} not found", to_id),
            ));
        };

        self.inner
            .connect_nodes(from_node_id, to_node_id, WorkflowEdge::data_flow())
            .map_err(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("not found") || error_msg.contains("Target node") {
                    Error::new(Status::InvalidArg, error_msg)
                } else {
                    to_napi_error(e)
                }
            })?;
        Ok(())
    }

    /// Validate the workflow
    #[napi]
    pub fn validate(&self) -> Result<()> {
        self.inner.validate().map_err(to_napi_error)?;
        Ok(())
    }

    /// Get the workflow name
    #[napi(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Get the number of nodes in the workflow
    #[napi(getter)]
    pub fn node_count(&self) -> u32 {
        self.inner.graph.node_count() as u32
    }

    /// Get the number of edges in the workflow
    #[napi(getter)]
    pub fn edge_count(&self) -> u32 {
        self.inner.graph.edge_count() as u32
    }
}
