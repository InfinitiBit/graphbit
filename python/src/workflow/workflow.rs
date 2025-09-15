//! Workflow implementation for GraphBit Python bindings

use super::node::Node;
use crate::errors::to_py_runtime_error;
use graphbit_core::{graph::WorkflowEdge, types::NodeId, workflow::Workflow as CoreWorkflow};
use pyo3::prelude::*;

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
        // First try to look up by name, if that fails try to parse as UUID
        let from_node_id = if let Some(id) = self.inner.graph.get_node_id_by_name(&from_id) {
            id
        } else if let Ok(id) = NodeId::from_string(&from_id) {
            id
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Source node {} not found",
                from_id
            )));
        };

        let to_node_id = if let Some(id) = self.inner.graph.get_node_id_by_name(&to_id) {
            id
        } else if let Ok(id) = NodeId::from_string(&to_id) {
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
