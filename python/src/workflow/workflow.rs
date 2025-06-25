//! Workflow implementation for GraphBit Python bindings

use super::node::Node;
use graphbit_core::{graph::WorkflowEdge, types::NodeId, workflow::Workflow as CoreWorkflow};
use pyo3::prelude::*;

#[pyclass]
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
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
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
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }
}
