//! Tool execution result management for GraphBit Python bindings

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Result of a tool execution
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool name that was executed
    #[pyo3(get)]
    pub tool_name: String,

    /// Input parameters passed to the tool
    #[pyo3(get)]
    pub input_params: String,

    /// Output result from the tool execution
    #[pyo3(get)]
    pub output: String,

    /// Execution status
    #[pyo3(get)]
    pub success: bool,

    /// Error message if execution failed
    #[pyo3(get)]
    pub error: Option<String>,

    /// Execution duration in milliseconds
    #[pyo3(get)]
    pub duration_ms: u64,

    /// Timestamp when the tool was executed
    #[pyo3(get)]
    pub timestamp: u64,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[pymethods]
impl ToolResult {
    /// Create a new successful tool result
    #[new]
    #[pyo3(signature = (tool_name, input_params, output, duration_ms=0))]
    pub fn new(tool_name: String, input_params: String, output: String, duration_ms: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            tool_name,
            input_params,
            output,
            success: true,
            error: None,
            duration_ms,
            timestamp,
            metadata: HashMap::new(),
        }
    }

    /// Create a new failed tool result
    #[staticmethod]
    #[pyo3(signature = (tool_name, input_params, error, duration_ms=0))]
    pub fn failure(
        tool_name: String,
        input_params: String,
        error: String,
        duration_ms: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            tool_name,
            input_params,
            output: String::new(),
            success: false,
            error: Some(error),
            duration_ms,
            timestamp,
            metadata: HashMap::new(),
        }
    }

    /// Check if the tool execution was successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if the tool execution failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }

    /// Get the error message if any
    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }

    /// Get execution duration as a Python timedelta-compatible value
    pub fn get_duration(&self) -> f64 {
        self.duration_ms as f64 / 1000.0
    }

    /// Add metadata to the result
    pub fn add_metadata(&mut self, key: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let json_value = python_to_json_value(value)?;
        self.metadata.insert(key, json_value);
        Ok(())
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<String> {
        self.metadata.get(key).map(|v| v.to_string())
    }

    /// Convert to JSON string representation
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize tool result: {}",
                e
            ))
        })
    }

    /// Create from JSON string
    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        serde_json::from_str(json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to deserialize tool result: {}",
                e
            ))
        })
    }

    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!(
            "ToolResult(tool_name='{}', success={}, duration_ms={})",
            self.tool_name, self.success, self.duration_ms
        )
    }

    /// String representation for display
    pub fn __str__(&self) -> String {
        if self.success {
            format!(
                "✓ {} -> {}",
                self.tool_name,
                if self.output.len() > 50 {
                    format!("{}...", &self.output[..50])
                } else {
                    self.output.clone()
                }
            )
        } else {
            format!(
                "✗ {} -> Error: {}",
                self.tool_name,
                self.error.as_ref().unwrap_or(&"Unknown error".to_string())
            )
        }
    }
}

/// Helper function to convert Python values to JSON
fn python_to_json_value(value: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
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
    } else {
        // Fallback to string representation
        Ok(serde_json::Value::String(value.to_string()))
    }
}

/// Collection of tool results for batch operations
#[pyclass]
#[derive(Debug, Clone)]
pub struct ToolResultCollection {
    results: Vec<ToolResult>,
}

#[pymethods]
impl ToolResultCollection {
    /// Create a new empty collection
    #[new]
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Add a result to the collection
    pub fn add(&mut self, result: ToolResult) {
        self.results.push(result);
    }

    /// Get all results
    pub fn get_all(&self) -> Vec<ToolResult> {
        self.results.clone()
    }

    /// Get successful results only
    pub fn get_successful(&self) -> Vec<ToolResult> {
        self.results.iter().filter(|r| r.success).cloned().collect()
    }

    /// Get failed results only
    pub fn get_failed(&self) -> Vec<ToolResult> {
        self.results
            .iter()
            .filter(|r| !r.success)
            .cloned()
            .collect()
    }

    /// Get total count
    pub fn count(&self) -> usize {
        self.results.len()
    }

    /// Get success count
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.success).count()
    }

    /// Get failure count
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.success).count()
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            (self.success_count() as f64 / self.results.len() as f64) * 100.0
        }
    }

    /// Get total execution time
    pub fn total_duration_ms(&self) -> u64 {
        self.results.iter().map(|r| r.duration_ms).sum()
    }

    /// Clear all results
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// String representation
    pub fn __repr__(&self) -> String {
        format!(
            "ToolResultCollection(count={}, success_rate={:.1}%)",
            self.count(),
            self.success_rate()
        )
    }
}

impl Default for ToolResultCollection {
    fn default() -> Self {
        Self::new()
    }
}
