//! Workflow node for GraphBit Python bindings

use graphbit_core::{
    graph::{NodeType, WorkflowNode},
    types::AgentId,
};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Node {
    pub(crate) inner: WorkflowNode,
}

#[pymethods]
impl Node {
    #[staticmethod]
    #[pyo3(signature = (name, prompt, agent_id=None, output_name=None))]
    fn agent(
        name: String,
        prompt: String,
        agent_id: Option<String>,
        output_name: Option<String>,
    ) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Name cannot be empty",
            ));
        }
        if prompt.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Prompt cannot be empty",
            ));
        }

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
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid ID: {}", e))
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

        Ok(Self { inner: node })
    }

    #[staticmethod]
    fn transform(name: String, transformation: String) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Transform name cannot be empty",
            ));
        }
        if transformation.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Transform transformation cannot be empty",
            ));
        }

        Ok(Self {
            inner: WorkflowNode::new(
                name.clone(),
                format!("Transform: {}", name),
                NodeType::Transform { transformation },
            ),
        })
    }

    #[staticmethod]
    fn condition(name: String, expression: String) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Condition name cannot be empty",
            ));
        }
        if expression.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Condition expression cannot be empty",
            ));
        }

        Ok(Self {
            inner: WorkflowNode::new(
                name.clone(),
                format!("Condition: {}", name),
                NodeType::Condition { expression },
            ),
        })
    }

    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }
}
