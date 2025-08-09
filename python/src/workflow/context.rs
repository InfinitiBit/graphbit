use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[pyclass]
#[derive(Clone)]
pub struct WorkflowContext {
    pub(crate) inner: graphbit_core::types::WorkflowContext,
}

#[pymethods]
impl WorkflowContext {
    #[new]
    fn new() -> Self {
        Self {
            inner: graphbit_core::types::WorkflowContext::new(
                graphbit_core::types::WorkflowId::new(),
            ),
        }
    }

    /// Set a variable in the context
    fn set_variable(&mut self, key: String, value: PyObject, py: Python<'_>) -> PyResult<()> {
        // Convert Python object to JSON value
        let json_value = if let Ok(s) = value.extract::<String>(py) {
            JsonValue::String(s)
        } else if let Ok(b) = value.extract::<bool>(py) {
            JsonValue::Bool(b)
        } else if let Ok(i) = value.extract::<i64>(py) {
            JsonValue::Number(serde_json::Number::from(i))
        } else if let Ok(f) = value.extract::<f64>(py) {
            JsonValue::Number(
                serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)),
            )
        } else {
            // Try to serialize as JSON string
            let json_str = format!("{}", value);
            JsonValue::String(json_str)
        };

        self.inner.variables.insert(key, json_value);
        Ok(())
    }

    /// Get a variable from the context
    fn get_variable(&self, key: &str, py: Python<'_>) -> PyResult<Option<PyObject>> {
        match self.inner.variables.get(key) {
            Some(value) => {
                let py_obj = json_to_python(value, py)?;
                Ok(Some(py_obj))
            }
            None => Ok(None),
        }
    }

    /// Get a node's output from the context
    fn get_node_output(&self, node_id: &str, py: Python<'_>) -> PyResult<Option<PyObject>> {
        match self.inner.get_node_output(node_id) {
            Some(value) => {
                let py_obj = json_to_python(value, py)?;
                Ok(Some(py_obj))
            }
            None => Ok(None),
        }
    }

    /// Get a nested value from a node's output using dot notation
    fn get_nested_output(&self, reference: &str, py: Python<'_>) -> PyResult<Option<PyObject>> {
        match self.inner.get_nested_output(reference) {
            Some(value) => {
                let py_obj = json_to_python(value, py)?;
                Ok(Some(py_obj))
            }
            None => Ok(None),
        }
    }

    /// Convert the context variables to a Python dictionary
    fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        for (k, v) in &self.inner.variables {
            let py_value = json_to_python(v, py)?;
            dict.set_item(k, py_value)?;
        }
        Ok(dict.into())
    }

    /// Get all node outputs as a dictionary
    fn get_all_node_outputs(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        for (k, v) in &self.inner.node_outputs {
            let py_value = json_to_python(v, py)?;
            dict.set_item(k, py_value)?;
        }
        Ok(dict.into())
    }

    /// Get the workflow ID
    fn get_workflow_id(&self) -> String {
        self.inner.workflow_id.to_string()
    }

    /// Get the workflow state
    fn get_state(&self) -> String {
        format!("{:?}", self.inner.state)
    }

    /// Check if the workflow is completed
    fn is_completed(&self) -> bool {
        matches!(
            self.inner.state,
            graphbit_core::types::WorkflowState::Completed
        )
    }

    /// Check if the workflow has failed
    fn is_failed(&self) -> bool {
        matches!(
            self.inner.state,
            graphbit_core::types::WorkflowState::Failed { .. }
        )
    }
}

/// Helper function to convert JSON values to Python objects
fn json_to_python(value: &JsonValue, py: Python<'_>) -> PyResult<PyObject> {
    match value {
        JsonValue::String(s) => Ok(s.clone().into_py(py)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(n.to_string().into_py(py))
            }
        }
        JsonValue::Bool(b) => Ok(b.into_py(py)),
        JsonValue::Null => Ok(py.None()),
        JsonValue::Array(arr) => {
            let py_list = pyo3::types::PyList::empty(py);
            for item in arr {
                let py_item = json_to_python(item, py)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into())
        }
        JsonValue::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                let py_value = json_to_python(v, py)?;
                py_dict.set_item(k, py_value)?;
            }
            Ok(py_dict.into())
        }
    }
}
