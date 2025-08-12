//! Workflow node for GraphBit Python bindings

use graphbit_core::{
    graph::{NodeType, WorkflowNode},
    types::{AgentId, ToolDefinition},
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
    #[pyo3(signature = (name, prompt, agent_id=None, tools=None))]
    fn agent(
        name: String,
        prompt: String,
        agent_id: Option<String>,
        tools: Option<Vec<PyObject>>,
    ) -> PyResult<Self> {
        // Validate required parameters
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Agent name cannot be empty",
            ));
        }
        if prompt.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Agent prompt cannot be empty",
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

        // Convert Python tools to Rust ToolDefinition objects
        let rust_tools = if let Some(py_tools) = tools {
            let mut converted_tools = Vec::new();
            Python::with_gil(|py| -> PyResult<()> {
                for py_tool in py_tools {
                    // Extract tool information from the Python object
                    let tool_name: String = py_tool.getattr(py, "_tool_name")?.extract(py)?;
                    let tool_description: String =
                        py_tool.getattr(py, "_tool_description")?.extract(py)?;

                    // Convert Python dict to JSON value
                    let py_params = py_tool.getattr(py, "_tool_parameters")?;
                    let json_module = py.import("json")?;
                    let dumps_fn = json_module.getattr("dumps")?;
                    let params_str = dumps_fn.call1((py_params,))?.extract::<String>()?;
                    let tool_parameters: serde_json::Value = serde_json::from_str(&params_str)
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Invalid tool parameters JSON: {}",
                                e
                            ))
                        })?;

                    let tool_category: Option<String> = py_tool
                        .getattr(py, "_tool_category")
                        .ok()
                        .and_then(|attr| attr.extract(py).ok());
                    let tool_version: Option<String> = py_tool
                        .getattr(py, "_tool_version")
                        .ok()
                        .and_then(|attr| attr.extract(py).ok());
                    let tool_enabled: bool = py_tool.getattr(py, "_tool_enabled")?.extract(py)?;

                    let tool_def = ToolDefinition {
                        name: tool_name,
                        description: tool_description,
                        parameters: tool_parameters,
                        category: tool_category,
                        version: tool_version,
                        enabled: tool_enabled,
                    };
                    converted_tools.push(tool_def);
                }
                Ok(())
            })?;
            Some(converted_tools)
        } else {
            None
        };

        let node = WorkflowNode::new(
            name.clone(),
            format!("Agent: {}", name),
            NodeType::Agent {
                agent_id: AgentId::from_string(&id).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid ID: {}", e))
                })?,
                prompt_template: prompt,
                tools: rust_tools,
            },
        );

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
