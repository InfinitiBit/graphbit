//! Thread-safe tool registry for GraphBit Python bindings

use crate::tools::result::ToolResult;
use graphbit_core::llm::LlmTool;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Metadata for a registered tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
    pub return_type: String,
    pub created_at: u64,
    pub call_count: u64,
    pub total_duration_ms: u64,
    pub last_called_at: Option<u64>,
}

impl ToolMetadata {
    pub fn new(
        name: String,
        description: String,
        parameters_schema: serde_json::Value,
        return_type: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            name,
            description,
            parameters_schema,
            return_type,
            created_at: timestamp,
            call_count: 0,
            total_duration_ms: 0,
            last_called_at: None,
        }
    }

    pub fn record_call(&mut self, duration_ms: u64) {
        self.call_count += 1;
        self.total_duration_ms += duration_ms;
        self.last_called_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        );
    }

    pub fn average_duration_ms(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.call_count as f64
        }
    }
}

/// Thread-safe registry for managing tools
#[pyclass]
#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, PyObject>>>,
    metadata: Arc<RwLock<HashMap<String, ToolMetadata>>>,
    execution_history: Arc<RwLock<Vec<ToolResult>>>,
}

#[pymethods]
impl ToolRegistry {
    /// Create a new tool registry
    #[new]
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            execution_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create a proxy to the global tool registry
    #[staticmethod]
    pub fn new_global_proxy() -> Self {
        // Get the global registry from the decorator module
        use crate::tools::decorator::get_global_registry;
        let global_registry = get_global_registry();
        let registry_guard = global_registry.lock().unwrap();

        // Return a clone that shares the same underlying data
        Self {
            tools: registry_guard.tools.clone(),
            metadata: registry_guard.metadata.clone(),
            execution_history: registry_guard.execution_history.clone(),
        }
    }

    /// Register a tool function with metadata
    pub fn register_tool(
        &self,
        name: String,
        description: String,
        function: PyObject,
        parameters_schema: &Bound<'_, PyDict>,
        return_type: Option<String>,
    ) -> PyResult<()> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Tool name cannot be empty",
            ));
        }

        if description.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Tool description cannot be empty",
            ));
        }

        // Convert parameters schema to JSON
        let schema_json = python_dict_to_json(parameters_schema)?;

        // Create metadata
        let metadata = ToolMetadata::new(
            name.clone(),
            description,
            schema_json,
            return_type.unwrap_or_else(|| "Any".to_string()),
        );

        // Store tool and metadata
        {
            let mut tools = self.tools.write().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to acquire tools lock: {}",
                    e
                ))
            })?;
            tools.insert(name.clone(), function);
        }

        {
            let mut meta = self.metadata.write().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to acquire metadata lock: {}",
                    e
                ))
            })?;
            meta.insert(name, metadata);
        }

        Ok(())
    }

    /// Unregister a tool
    pub fn unregister_tool(&self, name: &str) -> PyResult<bool> {
        let mut tools = self.tools.write().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire tools lock: {}",
                e
            ))
        })?;

        let mut metadata = self.metadata.write().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire metadata lock: {}",
                e
            ))
        })?;

        let removed_tool = tools.remove(name).is_some();
        let removed_meta = metadata.remove(name).is_some();

        Ok(removed_tool || removed_meta)
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> PyResult<bool> {
        let tools = self.tools.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire tools lock: {}",
                e
            ))
        })?;
        Ok(tools.contains_key(name))
    }

    /// Get list of registered tool names
    pub fn list_tools(&self) -> PyResult<Vec<String>> {
        let tools = self.tools.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire tools lock: {}",
                e
            ))
        })?;
        Ok(tools.keys().cloned().collect())
    }

    /// Get tool metadata
    pub fn get_tool_metadata(&self, name: &str) -> PyResult<Option<String>> {
        let metadata = self.metadata.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire metadata lock: {}",
                e
            ))
        })?;

        if let Some(meta) = metadata.get(name) {
            let json = serde_json::to_string(meta).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to serialize metadata: {}",
                    e
                ))
            })?;
            Ok(Some(json))
        } else {
            Ok(None)
        }
    }

    /// Get all tools as LlmTool format for agent integration
    pub fn get_llm_tools(&self) -> PyResult<Vec<String>> {
        let metadata = self.metadata.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire metadata lock: {}",
                e
            ))
        })?;

        let mut llm_tools = Vec::new();
        for meta in metadata.values() {
            let llm_tool = LlmTool::new(
                meta.name.clone(),
                meta.description.clone(),
                meta.parameters_schema.clone(),
            );

            let json = serde_json::to_string(&llm_tool).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to serialize LlmTool: {}",
                    e
                ))
            })?;
            llm_tools.push(json);
        }

        Ok(llm_tools)
    }

    /// Execute a tool by name with parameters
    pub fn execute_tool(
        &self,
        name: &str,
        params: &Bound<'_, PyDict>,
        py: Python<'_>,
    ) -> PyResult<ToolResult> {
        let start_time = Instant::now();

        // Get the tool function
        let function = {
            let tools = self.tools.read().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to acquire tools lock: {}",
                    e
                ))
            })?;

            tools.get(name).map(|f| f.clone_ref(py)).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!("Tool '{}' not found", name))
            })?
        };

        // Convert parameters to string for logging
        let params_str = params.repr()?.to_string();

        // Execute the tool
        let result = match function.call(py, (), Some(params)) {
            Ok(result) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let output = result.to_string();

                // Record execution in metadata
                self.record_tool_execution(name, duration)?;

                let tool_result = ToolResult::new(name.to_string(), params_str, output, duration);

                // Add to execution history
                self.add_to_history(tool_result.clone())?;

                tool_result
            }
            Err(e) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let error_msg = format!("Tool execution failed: {}", e);

                let tool_result =
                    ToolResult::failure(name.to_string(), params_str, error_msg, duration);

                // Add to execution history even for failures
                self.add_to_history(tool_result.clone())?;

                tool_result
            }
        };

        Ok(result)
    }

    /// Get execution history
    pub fn get_execution_history(&self) -> PyResult<Vec<ToolResult>> {
        let history = self.execution_history.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire history lock: {}",
                e
            ))
        })?;
        Ok(history.clone())
    }

    /// Clear execution history
    pub fn clear_history(&self) -> PyResult<()> {
        let mut history = self.execution_history.write().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire history lock: {}",
                e
            ))
        })?;
        history.clear();
        Ok(())
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> PyResult<String> {
        let metadata = self.metadata.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire metadata lock: {}",
                e
            ))
        })?;

        let history = self.execution_history.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire history lock: {}",
                e
            ))
        })?;

        let total_tools = metadata.len();
        let total_executions = history.len();
        let successful_executions = history.iter().filter(|r| r.success).count();
        let success_rate = if total_executions > 0 {
            (successful_executions as f64 / total_executions as f64) * 100.0
        } else {
            0.0
        };

        let stats = serde_json::json!({
            "total_tools": total_tools,
            "total_executions": total_executions,
            "successful_executions": successful_executions,
            "success_rate": success_rate,
            "tools": metadata.values().collect::<Vec<_>>()
        });

        serde_json::to_string_pretty(&stats).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize stats: {}",
                e
            ))
        })
    }

    /// String representation
    pub fn __repr__(&self) -> PyResult<String> {
        let tools = self.tools.read().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire tools lock: {}",
                e
            ))
        })?;
        Ok(format!("ToolRegistry(tools={})", tools.len()))
    }
}

impl ToolRegistry {
    /// Record tool execution in metadata (internal method)
    fn record_tool_execution(&self, name: &str, duration_ms: u64) -> PyResult<()> {
        let mut metadata = self.metadata.write().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire metadata lock: {}",
                e
            ))
        })?;

        if let Some(meta) = metadata.get_mut(name) {
            meta.record_call(duration_ms);
        }

        Ok(())
    }

    /// Add result to execution history (internal method)
    fn add_to_history(&self, result: ToolResult) -> PyResult<()> {
        let mut history = self.execution_history.write().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire history lock: {}",
                e
            ))
        })?;

        history.push(result);

        // Keep only last 1000 executions to prevent memory bloat
        if history.len() > 1000 {
            let len = history.len();
            history.drain(0..len - 1000);
        }

        Ok(())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to convert Python dict to JSON
fn python_dict_to_json(dict: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();

    for (key, value) in dict.iter() {
        let key_str = key.to_string();
        let json_value = python_any_to_json(&value)?;
        map.insert(key_str, json_value);
    }

    Ok(serde_json::Value::Object(map))
}

/// Helper function to convert Python Any to JSON
fn python_any_to_json(value: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if value.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(s) = value.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(i) = value.extract::<i64>() {
        Ok(serde_json::Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = value.extract::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(f) {
            Ok(serde_json::Value::Number(num))
        } else {
            Ok(serde_json::Value::Null)
        }
    } else if let Ok(b) = value.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(dict) = value.downcast::<PyDict>() {
        python_dict_to_json(&dict)
    } else {
        // Fallback to string representation
        Ok(serde_json::Value::String(value.to_string()))
    }
}
