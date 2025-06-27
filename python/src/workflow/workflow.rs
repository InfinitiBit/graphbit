//! Workflow implementation for GraphBit Python bindings

use super::node::Node;
use crate::errors::to_py_runtime_error;
use graphbit_core::{graph::WorkflowEdge, types::NodeId, workflow::Workflow as CoreWorkflow};
use pyo3::prelude::*;

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
        let from_node_id = NodeId::from_string(&from_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid from_id: {}", e))
        })?;
        let to_node_id = NodeId::from_string(&to_id).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid to_id: {}", e))
        })?;

        self.inner
            .connect_nodes(from_node_id, to_node_id, WorkflowEdge::data_flow())
            .map_err(to_py_runtime_error)?;
        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.inner.validate().map_err(to_py_runtime_error)?;
        Ok(())
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }
}
