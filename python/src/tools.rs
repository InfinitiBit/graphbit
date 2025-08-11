//! Python bindings for GraphBit tool calling functionality
//!
//! This module provides Python bindings for the tool calling system,
//! including tool registration, execution, and management.

use crate::errors::to_py_runtime_error;
use crate::runtime::get_runtime;
use graphbit_core::llm::LlmToolCall;
use graphbit_core::tools::{
    ToolExecutionStats, ToolInfo, ToolManager, ToolMetadata, ToolResult,
    execute_global_tool, get_global_tool_definitions, get_global_tool_manager as core_get_global_tool_manager, register_global_tool,
};
use graphbit_core::GraphBitResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

/// Convert serde_json::Value to Python object (simplified version)
fn json_to_python(py: Python<'_>, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(n.to_string().to_object(py))
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                py_list.append(json_to_python(py, item)?)?;
            }
            Ok(py_list.to_object(py))
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, value) in obj {
                py_dict.set_item(key, json_to_python(py, value)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}

/// Convert Python object to serde_json::Value (simplified version)
fn python_to_json(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = obj.extract::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            Ok(Value::Number(n))
        } else {
            Err(PyValueError::new_err("Invalid float value"))
        }
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(python_to_json(&item)?);
        }
        Ok(Value::Array(arr))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract()?;
            map.insert(key_str, python_to_json(&value)?);
        }
        Ok(Value::Object(map))
    } else {
        // Fallback to string representation
        Ok(Value::String(obj.str()?.to_str()?.to_string()))
    }
}

/// Python wrapper for ToolResult
#[pyclass(name = "ToolResult")]
#[derive(Debug, Clone)]
pub struct PyToolResult {
    inner: ToolResult,
}

#[pymethods]
impl PyToolResult {
    /// Check if the tool execution was successful
    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }

    /// Get the result data as a Python object
    #[getter]
    fn data(&self, py: Python<'_>) -> PyResult<PyObject> {
        json_to_python(py, &self.inner.data)
    }

    /// Get the execution time in milliseconds
    #[getter]
    fn execution_time_ms(&self) -> u64 {
        self.inner.execution_time_ms
    }

    /// Get the tool name that was executed
    #[getter]
    fn tool_name(&self) -> String {
        self.inner.tool_name.clone()
    }

    fn __str__(&self) -> String {
        format!(
            "ToolResult(success={}, tool_name='{}', execution_time={}ms)",
            self.inner.success, self.inner.tool_name, self.inner.execution_time_ms
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

/// Python wrapper for ToolInfo
#[pyclass(name = "ToolInfo")]
#[derive(Debug, Clone)]
pub struct PyToolInfo {
    inner: ToolInfo,
}

#[pymethods]
impl PyToolInfo {
    /// Get the tool name
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Get the tool description
    #[getter]
    fn description(&self) -> String {
        self.inner.description.clone()
    }

    /// Get the tool parameters schema as a Python object
    #[getter]
    fn parameters(&self, py: Python<'_>) -> PyResult<PyObject> {
        json_to_python(py, &self.inner.parameters)
    }

    /// Get the tool category
    #[getter]
    fn category(&self) -> String {
        self.inner.category.clone()
    }

    /// Get the tool version
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    /// Check if the tool is enabled
    #[getter]
    fn enabled(&self) -> bool {
        self.inner.enabled
    }

    fn __str__(&self) -> String {
        format!(
            "ToolInfo(name='{}', category='{}', enabled={})",
            self.inner.name, self.inner.category, self.inner.enabled
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

/// Python wrapper for ToolExecutionStats
#[pyclass(name = "ToolExecutionStats")]
#[derive(Debug, Clone)]
pub struct PyToolExecutionStats {
    inner: ToolExecutionStats,
}

#[pymethods]
impl PyToolExecutionStats {
    /// Get total number of tool calls
    #[getter]
    fn total_calls(&self) -> u64 {
        self.inner.total_calls
    }

    /// Get number of successful tool calls
    #[getter]
    fn successful_calls(&self) -> u64 {
        self.inner.successful_calls
    }

    /// Get number of failed tool calls
    #[getter]
    fn failed_calls(&self) -> u64 {
        self.inner.failed_calls
    }

    /// Get total execution time in milliseconds
    #[getter]
    fn total_execution_time_ms(&self) -> u64 {
        self.inner.total_execution_time_ms
    }

    /// Get call counts per tool as a Python dictionary
    #[getter]
    fn tool_call_counts(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        for (tool_name, count) in &self.inner.tool_call_counts {
            dict.set_item(tool_name, count)?;
        }
        Ok(dict.to_object(py))
    }

    /// Get success rate as a percentage
    #[getter]
    fn success_rate(&self) -> f64 {
        if self.inner.total_calls == 0 {
            0.0
        } else {
            (self.inner.successful_calls as f64 / self.inner.total_calls as f64) * 100.0
        }
    }

    /// Get average execution time in milliseconds
    #[getter]
    fn average_execution_time_ms(&self) -> f64 {
        if self.inner.total_calls == 0 {
            0.0
        } else {
            self.inner.total_execution_time_ms as f64 / self.inner.total_calls as f64
        }
    }

    fn __str__(&self) -> String {
        format!(
            "ToolExecutionStats(total_calls={}, success_rate={:.1}%, avg_time={:.1}ms)",
            self.inner.total_calls,
            self.success_rate(),
            self.average_execution_time_ms()
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

/// Python wrapper for ToolManager
#[pyclass(name = "ToolManager")]
#[derive(Debug, Clone)]
pub struct PyToolManager {
    inner: ToolManager,
}

#[pymethods]
impl PyToolManager {
    /// Create a new tool manager
    #[new]
    fn new() -> Self {
        Self {
            inner: ToolManager::new(),
        }
    }

    /// Register a Python function as a tool
    #[pyo3(signature = (name, description, parameters, function, category=None, version=None, enabled=None))]
    fn register_tool(
        &self,
        py: Python<'_>,
        name: String,
        description: String,
        parameters: PyObject,
        function: PyObject,
        category: Option<String>,
        version: Option<String>,
        enabled: Option<bool>,
    ) -> PyResult<()> {
        // Convert Python parameters to JSON
        let parameters_json = python_to_json(&parameters.bind(py))?;

        // Create a wrapper function that calls the Python function
        let function_wrapper = {
            let function = function.clone_ref(py);
            Box::new(move |params: Value| -> GraphBitResult<Value> {
                Python::with_gil(|py| {
                    let py_params = json_to_python(py, &params)
                        .map_err(|e| graphbit_core::GraphBitError::config(format!("Failed to convert params: {}", e)))?;
                    
                    let result = function.call1(py, (py_params,))
                        .map_err(|e| graphbit_core::GraphBitError::config(format!("Python function call failed: {}", e)))?;
                    
                    let json_result = python_to_json(&result.bind(py))
                        .map_err(|e| graphbit_core::GraphBitError::config(format!("Failed to convert result: {}", e)))?;
                    
                    Ok(json_result)
                })
            })
        };

        // Create tool metadata
        let mut metadata = ToolMetadata::new(name, description, parameters_json, function_wrapper);

        if let Some(category) = category {
            metadata = metadata.with_category(category);
        }

        if let Some(version) = version {
            metadata = metadata.with_version(version);
        }

        if let Some(enabled) = enabled {
            metadata = metadata.with_enabled(enabled);
        }

        // Register the tool
        self.inner.register_tool(metadata).map_err(to_py_runtime_error)?;

        Ok(())
    }

    /// Execute a tool call
    fn execute_tool(&self, tool_name: String, parameters: PyObject, py: Python<'_>) -> PyResult<PyToolResult> {
        // Convert Python parameters to JSON
        let parameters_json = python_to_json(&parameters.bind(py))?;

        // Create tool call
        let tool_call = LlmToolCall {
            id: format!("py_call_{}", uuid::Uuid::new_v4()),
            name: tool_name,
            parameters: parameters_json,
        };

        // Execute the tool
        let result = self.inner.execute_tool(&tool_call).map_err(to_py_runtime_error)?;

        Ok(PyToolResult { inner: result })
    }

    /// List all registered tools
    fn list_tools(&self) -> PyResult<Vec<String>> {
        self.inner.list_tools().map_err(to_py_runtime_error)
    }

    fn __str__(&self) -> PyResult<String> {
        let tool_count = self.list_tools()?.len();
        Ok(format!("ToolManager(registered_tools={})", tool_count))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }
}

/// Register a tool globally
#[pyfunction]
#[pyo3(signature = (name, description, parameters, function, category=None, version=None, enabled=None))]
pub fn register_tool(
    py: Python<'_>,
    name: String,
    description: String,
    parameters: PyObject,
    function: PyObject,
    category: Option<String>,
    version: Option<String>,
    enabled: Option<bool>,
) -> PyResult<()> {
    // Convert Python parameters to JSON
    let parameters_json = python_to_json(&parameters.bind(py))?;

    // Create a wrapper function that calls the Python function
    let function_wrapper = {
        let function = function.clone_ref(py);
        Box::new(move |params: Value| -> GraphBitResult<Value> {
            Python::with_gil(|py| {
                let py_params = json_to_python(py, &params)
                    .map_err(|e| graphbit_core::GraphBitError::config(format!("Failed to convert params: {}", e)))?;
                
                let result = function.call1(py, (py_params,))
                    .map_err(|e| graphbit_core::GraphBitError::config(format!("Python function call failed: {}", e)))?;
                
                let json_result = python_to_json(&result.bind(py))
                    .map_err(|e| graphbit_core::GraphBitError::config(format!("Failed to convert result: {}", e)))?;
                
                Ok(json_result)
            })
        })
    };

    // Create tool metadata
    let mut metadata = ToolMetadata::new(name.clone(), description, parameters_json, function_wrapper);

    if let Some(category) = category {
        metadata = metadata.with_category(category);
    }

    if let Some(version) = version {
        metadata = metadata.with_version(version);
    }

    if let Some(enabled) = enabled {
        metadata = metadata.with_enabled(enabled);
    }

    // Register the tool globally
    register_global_tool(metadata).map_err(to_py_runtime_error)?;

    info!("Tool '{}' registered globally", name);
    Ok(())
}

/// Execute a tool globally
#[pyfunction]
pub fn execute_tool(py: Python<'_>, tool_name: String, parameters: PyObject) -> PyResult<PyToolResult> {
    // Convert Python parameters to JSON
    let parameters_json = python_to_json(&parameters.bind(py))?;

    // Create tool call
    let tool_call = LlmToolCall {
        id: format!("py_global_call_{}", uuid::Uuid::new_v4()),
        name: tool_name,
        parameters: parameters_json,
    };

    // Execute the tool
    let result = execute_global_tool(&tool_call).map_err(to_py_runtime_error)?;

    Ok(PyToolResult { inner: result })
}

/// Get all global tool definitions
#[pyfunction]
pub fn get_tool_definitions(py: Python<'_>) -> PyResult<PyObject> {
    let definitions = get_global_tool_definitions().map_err(to_py_runtime_error)?;
    
    let py_list = PyList::empty(py);
    for definition in definitions {
        let py_tool = PyDict::new(py);
        py_tool.set_item("name", definition.name)?;
        py_tool.set_item("description", definition.description)?;
        py_tool.set_item("parameters", json_to_python(py, &definition.parameters)?)?;
        py_list.append(py_tool)?;
    }
    
    Ok(py_list.to_object(py))
}

/// Get the global tool manager
#[pyfunction]
pub fn get_tool_manager() -> PyToolManager {
    PyToolManager {
        inner: core_get_global_tool_manager().clone(),
    }
}