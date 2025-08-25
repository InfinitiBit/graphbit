//! Sequential tool execution engine for GraphBit Python bindings
//!
//! This module provides the core execution engine that handles sequential tool calls,
//! stores results, and integrates with the LLM agent system.

use crate::tools::registry::ToolRegistry;
use crate::tools::result::{ToolResult, ToolResultCollection};
use graphbit_core::llm::LlmToolCall;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::{debug, error, info, warn};

/// Custom error types for tool execution
#[derive(Debug)]
pub enum ToolExecutionError {
    RegistryLockError(String),
    ContextLockError(String),
    ToolNotFound(String),
    InvalidParameters(String),
    ExecutionTimeout(String),
    MaxCallsExceeded(String),
    ValidationError(String),
    SerializationError(String),
}

impl std::fmt::Display for ToolExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolExecutionError::RegistryLockError(msg) => write!(f, "Registry lock error: {}", msg),
            ToolExecutionError::ContextLockError(msg) => write!(f, "Context lock error: {}", msg),
            ToolExecutionError::ToolNotFound(msg) => write!(f, "Tool not found: {}", msg),
            ToolExecutionError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            ToolExecutionError::ExecutionTimeout(msg) => write!(f, "Execution timeout: {}", msg),
            ToolExecutionError::MaxCallsExceeded(msg) => write!(f, "Max calls exceeded: {}", msg),
            ToolExecutionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ToolExecutionError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ToolExecutionError {}

impl From<ToolExecutionError> for PyErr {
    fn from(err: ToolExecutionError) -> Self {
        match err {
            ToolExecutionError::RegistryLockError(msg) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg)
            }
            ToolExecutionError::ContextLockError(msg) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg)
            }
            ToolExecutionError::ToolNotFound(msg) => {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(msg)
            }
            ToolExecutionError::InvalidParameters(msg) => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(msg)
            }
            ToolExecutionError::ExecutionTimeout(msg) => {
                PyErr::new::<pyo3::exceptions::PyTimeoutError, _>(msg)
            }
            ToolExecutionError::MaxCallsExceeded(msg) => {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg)
            }
            ToolExecutionError::ValidationError(msg) => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(msg)
            }
            ToolExecutionError::SerializationError(msg) => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(msg)
            }
        }
    }
}

/// Sequential tool execution engine
#[pyclass]
pub struct ToolExecutor {
    registry: Arc<Mutex<ToolRegistry>>,
    execution_context: Arc<Mutex<ExecutionContext>>,
    config: ExecutorConfig,
}

/// Configuration for tool execution
#[pyclass]
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Maximum execution time per tool in milliseconds
    #[pyo3(get, set)]
    pub max_execution_time_ms: u64,

    /// Maximum number of sequential tool calls
    #[pyo3(get, set)]
    pub max_tool_calls: usize,

    /// Whether to continue execution if a tool fails
    #[pyo3(get, set)]
    pub continue_on_error: bool,

    /// Whether to store execution results
    #[pyo3(get, set)]
    pub store_results: bool,

    /// Whether to enable detailed logging
    #[pyo3(get, set)]
    pub enable_logging: bool,
}

#[pymethods]
impl ExecutorConfig {
    /// Create a new executor configuration
    #[new]
    #[pyo3(signature = (
        max_execution_time_ms=30000,
        max_tool_calls=10,
        continue_on_error=false,
        store_results=true,
        enable_logging=true
    ))]
    pub fn new(
        max_execution_time_ms: u64,
        max_tool_calls: usize,
        continue_on_error: bool,
        store_results: bool,
        enable_logging: bool,
    ) -> Self {
        Self {
            max_execution_time_ms,
            max_tool_calls,
            continue_on_error,
            store_results,
            enable_logging,
        }
    }

    /// Create a production configuration
    #[staticmethod]
    pub fn production() -> Self {
        Self {
            max_execution_time_ms: 60000, // 60 seconds
            max_tool_calls: 20,
            continue_on_error: false,
            store_results: true,
            enable_logging: true,
        }
    }

    /// Create a development configuration
    #[staticmethod]
    pub fn development() -> Self {
        Self {
            max_execution_time_ms: 10000, // 10 seconds
            max_tool_calls: 5,
            continue_on_error: true,
            store_results: true,
            enable_logging: true,
        }
    }

    /// String representation
    pub fn __repr__(&self) -> String {
        format!(
            "ExecutorConfig(max_time={}ms, max_calls={}, continue_on_error={})",
            self.max_execution_time_ms, self.max_tool_calls, self.continue_on_error
        )
    }
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self::new(30000, 10, false, true, true)
    }
}

/// Execution context for tracking tool calls and results
#[derive(Debug, Clone)]
struct ExecutionContext {
    current_execution_id: String,
    tool_call_count: usize,
    results: ToolResultCollection,
    variables: HashMap<String, Value>,
    start_time: Option<Instant>,
}

impl ExecutionContext {
    fn new() -> Self {
        Self {
            current_execution_id: uuid::Uuid::new_v4().to_string(),
            tool_call_count: 0,
            results: ToolResultCollection::new(),
            variables: HashMap::new(),
            start_time: None,
        }
    }

    fn reset(&mut self) {
        self.current_execution_id = uuid::Uuid::new_v4().to_string();
        self.tool_call_count = 0;
        self.results = ToolResultCollection::new();
        self.variables.clear();
        self.start_time = None;
    }

    fn start_execution(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn add_result(&mut self, result: ToolResult) {
        self.results.add(result);
        self.tool_call_count += 1;
    }

    fn set_variable(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    fn get_variable(&self, key: &str) -> Option<&Value> {
        self.variables.get(key)
    }
}

#[pymethods]
impl ToolExecutor {
    /// Create a new tool executor
    #[new]
    #[pyo3(signature = (registry=None, config=None))]
    pub fn new(registry: Option<&ToolRegistry>, config: Option<ExecutorConfig>) -> Self {
        let reg = Arc::new(Mutex::new(
            registry
                .map(|r| r.clone())
                .unwrap_or_else(ToolRegistry::new),
        ));

        let conf = config.unwrap_or_default();

        Self {
            registry: reg,
            execution_context: Arc::new(Mutex::new(ExecutionContext::new())),
            config: conf,
        }
    }

    /// Execute a sequence of tool calls
    pub fn execute_tools(
        &self,
        tool_calls: &Bound<'_, PyList>,
        py: Python<'_>,
    ) -> PyResult<ToolResultCollection> {
        // Validate configuration first
        self.validate_config()?;

        let start_time = Instant::now();

        // Validate input
        if tool_calls.is_empty() {
            return Ok(ToolResultCollection::new());
        }

        // Reset execution context
        {
            let mut context = self.execution_context.lock().map_err(|e| {
                ToolExecutionError::ContextLockError(format!(
                    "Failed to acquire context lock: {}",
                    e
                ))
            })?;
            context.reset();
            context.start_execution();
        }

        if self.config.enable_logging {
            info!(
                "Starting tool execution sequence with {} tools",
                tool_calls.len()
            );
        }

        let mut results = ToolResultCollection::new();
        let mut execution_count = 0;

        for (index, tool_call_item) in tool_calls.iter().enumerate() {
            // Check execution limits
            if let Err(e) = self.check_execution_limits(execution_count, start_time) {
                if self.config.enable_logging {
                    warn!("{}", e);
                }
                break;
            }

            // Parse and validate tool call
            let tool_call = match self.parse_tool_call(&tool_call_item, py) {
                Ok(call) => call,
                Err(e) => {
                    let error_result = ToolResult::failure(
                        "unknown_tool".to_string(),
                        "invalid_params".to_string(),
                        format!("Failed to parse tool call: {}", e),
                        0,
                    );
                    results.add(error_result);

                    if !self.config.continue_on_error {
                        break;
                    }
                    continue;
                }
            };

            // Validate tool call
            if let Err(e) = self.validate_tool_call(&tool_call) {
                let error_result = ToolResult::failure(
                    tool_call.name.clone(),
                    serde_json::to_string(&tool_call.parameters).unwrap_or_default(),
                    format!("Tool call validation failed: {}", e),
                    0,
                );
                results.add(error_result);

                if !self.config.continue_on_error {
                    break;
                }
                continue;
            }

            if self.config.enable_logging {
                debug!("Executing tool {}: {}", index + 1, tool_call.name);
            }

            // Execute the tool
            let result = self.execute_single_tool_internal(tool_call, py)?;

            // Store result
            if self.config.store_results {
                results.add(result.clone());

                let mut context = self.execution_context.lock().map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to acquire context lock: {}",
                        e
                    ))
                })?;
                context.add_result(result.clone());

                // Store result as variable for next tools
                let var_name = format!("tool_result_{}", execution_count + 1);
                let result_value = serde_json::to_value(&result.output).unwrap_or(Value::Null);
                context.set_variable(var_name, result_value);
            }

            execution_count += 1;

            // Check if we should continue on error
            if !result.success && !self.config.continue_on_error {
                error!(
                    "Tool execution failed and continue_on_error is false: {}",
                    result.error.unwrap_or_else(|| "Unknown error".to_string())
                );
                break;
            }
        }

        if self.config.enable_logging {
            info!(
                "Tool execution sequence completed. Executed {} tools in {}ms",
                execution_count,
                start_time.elapsed().as_millis()
            );
        }

        Ok(results)
    }

    /// Get execution results
    pub fn get_results(&self) -> PyResult<ToolResultCollection> {
        let context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        Ok(context.results.clone())
    }

    /// Get execution variables
    pub fn get_variables(&self) -> PyResult<String> {
        let context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        serde_json::to_string(&context.variables).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize variables: {}",
                e
            ))
        })
    }

    /// Set execution variable
    pub fn set_variable(&self, key: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let mut context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        let json_value = self.python_to_json_value(value)?;
        context.set_variable(key, json_value);

        Ok(())
    }

    /// Get execution variable
    pub fn get_variable(&self, key: &str) -> PyResult<Option<String>> {
        let context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        if let Some(value) = context.get_variable(key) {
            Ok(Some(value.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Clear execution context
    pub fn clear_context(&self) -> PyResult<()> {
        let mut context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        context.reset();
        Ok(())
    }

    /// Get execution statistics
    pub fn get_execution_stats(&self) -> PyResult<String> {
        let context = self.execution_context.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire context lock: {}",
                e
            ))
        })?;

        let elapsed_ms = context
            .start_time
            .map(|start| start.elapsed().as_millis() as u64)
            .unwrap_or(0);

        let stats = serde_json::json!({
            "execution_id": context.current_execution_id,
            "tool_calls": context.tool_call_count,
            "elapsed_ms": elapsed_ms,
            "success_rate": context.results.success_rate(),
            "total_results": context.results.count(),
            "successful_results": context.results.success_count(),
            "failed_results": context.results.failure_count()
        });

        serde_json::to_string_pretty(&stats).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize stats: {}",
                e
            ))
        })
    }

    /// String representation
    pub fn __repr__(&self) -> String {
        format!("ToolExecutor(config={})", self.config.__repr__())
    }
}

impl ToolExecutor {
    /// Execute a single tool call (internal method)
    fn execute_single_tool_internal(
        &self,
        tool_call: LlmToolCall,
        py: Python<'_>,
    ) -> PyResult<ToolResult> {
        let registry = self.registry.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to acquire registry lock: {}",
                e
            ))
        })?;

        // Convert parameters to PyDict
        let params = self.json_to_pydict(&tool_call.parameters, py)?;

        // Execute the tool
        registry.execute_tool(&tool_call.name, &params, py)
    }
    /// Validate execution configuration
    fn validate_config(&self) -> Result<(), ToolExecutionError> {
        if self.config.max_execution_time_ms == 0 {
            return Err(ToolExecutionError::ValidationError(
                "Max execution time must be greater than 0".to_string(),
            ));
        }

        if self.config.max_tool_calls == 0 {
            return Err(ToolExecutionError::ValidationError(
                "Max tool calls must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate tool call parameters
    fn validate_tool_call(&self, tool_call: &LlmToolCall) -> Result<(), ToolExecutionError> {
        if tool_call.name.trim().is_empty() {
            return Err(ToolExecutionError::ValidationError(
                "Tool name cannot be empty".to_string(),
            ));
        }

        if tool_call.id.trim().is_empty() {
            return Err(ToolExecutionError::ValidationError(
                "Tool call ID cannot be empty".to_string(),
            ));
        }

        // Validate parameters is a valid JSON object
        if !tool_call.parameters.is_object() && !tool_call.parameters.is_null() {
            return Err(ToolExecutionError::ValidationError(
                "Tool parameters must be a JSON object or null".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if execution should be stopped due to limits
    fn check_execution_limits(
        &self,
        execution_count: usize,
        start_time: Instant,
    ) -> Result<(), ToolExecutionError> {
        if execution_count >= self.config.max_tool_calls {
            return Err(ToolExecutionError::MaxCallsExceeded(format!(
                "Reached maximum tool call limit: {}",
                self.config.max_tool_calls
            )));
        }

        if start_time.elapsed().as_millis() as u64 > self.config.max_execution_time_ms {
            return Err(ToolExecutionError::ExecutionTimeout(format!(
                "Reached maximum execution time: {}ms",
                self.config.max_execution_time_ms
            )));
        }

        Ok(())
    }

    /// Parse a tool call from Python object
    fn parse_tool_call(
        &self,
        tool_call_obj: &Bound<'_, PyAny>,
        _py: Python<'_>,
    ) -> PyResult<LlmToolCall> {
        // Try to extract as dictionary first
        if let Ok(dict) = tool_call_obj.downcast::<PyDict>() {
            let id = dict
                .get_item("id")?
                .map(|item| item.extract::<String>())
                .transpose()?
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

            let name = dict
                .get_item("name")?
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'name' in tool call")
                })?
                .extract::<String>()?;

            let parameters = dict
                .get_item("parameters")?
                .map(|p| self.python_to_json_value(&p))
                .transpose()?
                .unwrap_or(Value::Object(serde_json::Map::new()));

            return Ok(LlmToolCall {
                id,
                name,
                parameters,
            });
        }

        // Try to extract as JSON string
        if let Ok(json_str) = tool_call_obj.extract::<String>() {
            return serde_json::from_str(&json_str).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to parse tool call JSON: {}",
                    e
                ))
            });
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Tool call must be a dictionary or JSON string",
        ))
    }

    /// Convert JSON value to PyDict
    fn json_to_pydict<'a>(&self, value: &Value, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);

        if let Value::Object(map) = value {
            for (key, val) in map {
                let py_val = self.json_value_to_python(val, py)?;
                dict.set_item(key, py_val)?;
            }
        }

        Ok(dict)
    }

    /// Convert JSON value to Python object
    fn json_value_to_python(&self, value: &Value, py: Python<'_>) -> PyResult<PyObject> {
        match value {
            Value::Null => Ok(py.None()),
            Value::Bool(b) => Ok(b.to_object(py)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.to_object(py))
                } else if let Some(f) = n.as_f64() {
                    Ok(f.to_object(py))
                } else {
                    Ok(py.None())
                }
            }
            Value::String(s) => Ok(s.to_object(py)),
            Value::Array(arr) => {
                let py_list = PyList::empty(py);
                for item in arr {
                    let py_item = self.json_value_to_python(item, py)?;
                    py_list.append(py_item)?;
                }
                Ok(py_list.to_object(py))
            }
            Value::Object(obj) => {
                let py_dict = PyDict::new(py);
                for (key, val) in obj {
                    let py_val = self.json_value_to_python(val, py)?;
                    py_dict.set_item(key, py_val)?;
                }
                Ok(py_dict.to_object(py))
            }
        }
    }

    /// Convert Python value to JSON
    fn python_to_json_value(&self, value: &Bound<'_, PyAny>) -> PyResult<Value> {
        if value.is_none() {
            Ok(Value::Null)
        } else if let Ok(s) = value.extract::<String>() {
            Ok(Value::String(s))
        } else if let Ok(i) = value.extract::<i64>() {
            Ok(Value::Number(serde_json::Number::from(i)))
        } else if let Ok(f) = value.extract::<f64>() {
            if let Some(num) = serde_json::Number::from_f64(f) {
                Ok(Value::Number(num))
            } else {
                Ok(Value::Null)
            }
        } else if let Ok(b) = value.extract::<bool>() {
            Ok(Value::Bool(b))
        } else {
            // Fallback to string representation
            Ok(Value::String(value.to_string()))
        }
    }
}

impl Default for ToolExecutor {
    fn default() -> Self {
        Self::new(None, None)
    }
}
