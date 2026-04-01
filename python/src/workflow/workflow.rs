//! Workflow implementation for GraphBit Python bindings

use super::node::Node;
use crate::errors::to_py_runtime_error;
use graphbit_core::{graph::WorkflowEdge, types::NodeId, workflow::Workflow as CoreWorkflow};
use pyo3::prelude::*;
use uuid::Uuid;

/// A workflow definition containing nodes and their execution flow
#[pyclass]
#[derive(Clone)]
pub struct Workflow {
    pub(crate) inner: CoreWorkflow,
}

#[pymethods]
impl Workflow {
    #[new]
    fn new(name: String) -> Self {
        Self {
            inner: CoreWorkflow::new(name, "Fast workflow"),
        }
    }

    fn add_node(&mut self, node: Node) -> PyResult<String> {
        let node_id = self
            .inner
            .add_node(node.inner)
            .map_err(to_py_runtime_error)?;
        Ok(node_id.to_string())
    }

    fn connect(&mut self, from_id: String, to_id: String) -> PyResult<()> {
        // If the string is a literal UUID (`add_node` return values), parse it first so a node
        // *name* that equals another node's UUID cannot hijack resolution. Otherwise resolve by
        // human-readable node name. (NodeId::from_string accepts any string via v5 — do not use
        // that path for names.)
        let from_node_id = if Uuid::parse_str(from_id.trim()).is_ok() {
            NodeId::from_string(&from_id).map_err(to_py_runtime_error)?
        } else if let Some(id) = self.inner.graph.get_node_id_by_name(&from_id) {
            id
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Source node {} not found",
                from_id
            )));
        };

        let to_node_id = if Uuid::parse_str(to_id.trim()).is_ok() {
            NodeId::from_string(&to_id).map_err(to_py_runtime_error)?
        } else if let Some(id) = self.inner.graph.get_node_id_by_name(&to_id) {
            id
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Target node {} not found",
                to_id
            )));
        };

        self.inner
            .connect_nodes(from_node_id, to_node_id, WorkflowEdge::data_flow())
            .map_err(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("not found") || error_msg.contains("Target node") {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(error_msg)
                } else {
                    to_py_runtime_error(e)
                }
            })?;
        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.inner.validate().map_err(to_py_runtime_error)?;
        Ok(())
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Set graph-level metadata key to a boolean value
    /// Exposes core graph.set_metadata for Python tests and configuration
    fn set_graph_metadata(&mut self, key: String, value: bool) -> PyResult<()> {
        self.inner
            .graph
            .set_metadata(key, serde_json::Value::Bool(value));
        Ok(())
    }
}
