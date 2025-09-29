//! Python bindings for LLM response types
//!
//! This module provides PyO3 bindings for GraphBit's LLM response types,
//! enabling comprehensive LLM tracing and observability from Python.

use graphbit_core::llm::{FinishReason, LlmResponse, LlmToolCall, LlmUsage};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json;

/// Python wrapper for LlmUsage struct
/// 
/// Provides token usage statistics for LLM requests including prompt tokens,
/// completion tokens, and total token counts.
#[pyclass(name = "LlmUsage")]
#[derive(Debug, Clone)]
pub struct PyLlmUsage {
    pub(crate) inner: LlmUsage,
}

#[pymethods]
impl PyLlmUsage {
    /// Create new usage statistics
    #[new]
    #[pyo3(signature = (prompt_tokens, completion_tokens))]
    fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            inner: LlmUsage::new(prompt_tokens, completion_tokens),
        }
    }

    /// Create empty usage statistics
    #[staticmethod]
    fn empty() -> Self {
        Self {
            inner: LlmUsage::empty(),
        }
    }

    /// Number of tokens in the prompt
    #[getter]
    fn prompt_tokens(&self) -> u32 {
        self.inner.prompt_tokens
    }

    /// Number of tokens in the completion
    #[getter]
    fn completion_tokens(&self) -> u32 {
        self.inner.completion_tokens
    }

    /// Total number of tokens
    #[getter]
    fn total_tokens(&self) -> u32 {
        self.inner.total_tokens
    }

    /// Add usage statistics from another LlmUsage instance
    fn add(&mut self, other: &PyLlmUsage) {
        self.inner.add(&other.inner);
    }

    /// Add two usage instances together (returns new instance)
    fn __add__(&self, other: &PyLlmUsage) -> PyLlmUsage {
        PyLlmUsage {
            inner: self.inner.clone() + other.inner.clone(),
        }
    }

    /// Add usage statistics in place
    fn __iadd__(&mut self, other: &PyLlmUsage) {
        self.inner += other.inner.clone();
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "LlmUsage(prompt_tokens={}, completion_tokens={}, total_tokens={})",
            self.inner.prompt_tokens, self.inner.completion_tokens, self.inner.total_tokens
        )
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }

    /// Convert to dictionary for JSON serialization
    fn to_dict<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("prompt_tokens", self.inner.prompt_tokens)?;
        dict.set_item("completion_tokens", self.inner.completion_tokens)?;
        dict.set_item("total_tokens", self.inner.total_tokens)?;
        Ok(dict)
    }

    /// Create from dictionary
    #[staticmethod]
    fn from_dict(data: &Bound<'_, PyDict>) -> PyResult<Self> {
        let prompt_tokens: u32 = data.get_item("prompt_tokens")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'prompt_tokens'"))?
            .extract()?;
        let completion_tokens: u32 = data.get_item("completion_tokens")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'completion_tokens'"))?
            .extract()?;
        
        Ok(Self::new(prompt_tokens, completion_tokens))
    }
}

impl From<LlmUsage> for PyLlmUsage {
    fn from(usage: LlmUsage) -> Self {
        Self { inner: usage }
    }
}

impl From<PyLlmUsage> for LlmUsage {
    fn from(py_usage: PyLlmUsage) -> Self {
        py_usage.inner
    }
}

/// Python wrapper for FinishReason enum
/// 
/// Represents the reason why the LLM stopped generating tokens.
#[pyclass(name = "FinishReason")]
#[derive(Debug, Clone)]
pub struct PyFinishReason {
    pub(crate) inner: FinishReason,
}

#[pymethods]
impl PyFinishReason {
    /// Create Stop finish reason
    #[staticmethod]
    fn stop() -> Self {
        Self {
            inner: FinishReason::Stop,
        }
    }

    /// Create Length finish reason
    #[staticmethod]
    fn length() -> Self {
        Self {
            inner: FinishReason::Length,
        }
    }

    /// Create ToolCalls finish reason
    #[staticmethod]
    fn tool_calls() -> Self {
        Self {
            inner: FinishReason::ToolCalls,
        }
    }

    /// Create ContentFilter finish reason
    #[staticmethod]
    fn content_filter() -> Self {
        Self {
            inner: FinishReason::ContentFilter,
        }
    }

    /// Create Error finish reason
    #[staticmethod]
    fn error() -> Self {
        Self {
            inner: FinishReason::Error,
        }
    }

    /// Create Other finish reason with custom message
    #[staticmethod]
    fn other(reason: String) -> Self {
        Self {
            inner: FinishReason::Other(reason),
        }
    }

    /// Check if the response finished naturally
    fn is_natural_stop(&self) -> bool {
        self.inner.is_natural_stop()
    }

    /// Check if the response was truncated
    fn is_truncated(&self) -> bool {
        self.inner.is_truncated()
    }

    /// Check if there was an error
    fn is_error(&self) -> bool {
        self.inner.is_error()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("FinishReason({})", self.inner)
    }

    /// String representation
    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    /// Convert to string for serialization
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl From<FinishReason> for PyFinishReason {
    fn from(reason: FinishReason) -> Self {
        Self { inner: reason }
    }
}

impl From<PyFinishReason> for FinishReason {
    fn from(py_reason: PyFinishReason) -> Self {
        py_reason.inner
    }
}

/// Python wrapper for LlmToolCall struct
///
/// Represents a tool/function call made by the LLM.
#[pyclass(name = "LlmToolCall")]
#[derive(Debug, Clone)]
pub struct PyLlmToolCall {
    pub(crate) inner: LlmToolCall,
}

#[pymethods]
impl PyLlmToolCall {
    /// Create new tool call
    #[new]
    fn new(id: String, name: String, parameters: String) -> PyResult<Self> {
        let params_value: serde_json::Value = serde_json::from_str(&parameters)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON parameters: {}", e)))?;

        Ok(Self {
            inner: LlmToolCall {
                id,
                name,
                parameters: params_value,
            },
        })
    }

    /// Tool call ID
    #[getter]
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    /// Tool/function name
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Tool parameters as JSON string
    #[getter]
    fn parameters(&self) -> String {
        serde_json::to_string(&self.inner.parameters).unwrap_or_else(|_| "{}".to_string())
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "LlmToolCall(id='{}', name='{}', parameters={})",
            self.inner.id, self.inner.name, self.parameters()
        )
    }

    /// Convert to dictionary
    fn to_dict<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("id", &self.inner.id)?;
        dict.set_item("name", &self.inner.name)?;
        dict.set_item("parameters", self.parameters())?;
        Ok(dict)
    }
}

impl From<LlmToolCall> for PyLlmToolCall {
    fn from(tool_call: LlmToolCall) -> Self {
        Self { inner: tool_call }
    }
}

impl From<PyLlmToolCall> for LlmToolCall {
    fn from(py_tool_call: PyLlmToolCall) -> Self {
        py_tool_call.inner
    }
}

/// Python wrapper for LlmResponse struct
///
/// Represents a complete response from an LLM provider including content,
/// usage statistics, finish reason, and metadata.
#[pyclass(name = "LlmResponse")]
#[derive(Debug, Clone)]
pub struct PyLlmResponse {
    pub(crate) inner: LlmResponse,
}

#[pymethods]
impl PyLlmResponse {
    /// Create new LLM response
    #[new]
    fn new(content: String, model: String) -> Self {
        Self {
            inner: LlmResponse::new(content, model),
        }
    }

    /// The generated content
    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    /// Tool calls made by the LLM
    #[getter]
    fn tool_calls(&self) -> Vec<PyLlmToolCall> {
        self.inner
            .tool_calls
            .iter()
            .map(|tc| PyLlmToolCall::from(tc.clone()))
            .collect()
    }

    /// Usage statistics
    #[getter]
    fn usage(&self) -> PyLlmUsage {
        PyLlmUsage::from(self.inner.usage.clone())
    }

    /// Response metadata as dictionary (JSON strings)
    #[getter]
    fn metadata<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.inner.metadata {
            let json_str = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
            dict.set_item(key, json_str)?;
        }
        Ok(dict)
    }

    /// Finish reason
    #[getter]
    fn finish_reason(&self) -> PyFinishReason {
        PyFinishReason::from(self.inner.finish_reason.clone())
    }

    /// Model used for generation
    #[getter]
    fn model(&self) -> String {
        self.inner.model.clone()
    }

    /// Response ID (if provided by the API)
    #[getter]
    fn id(&self) -> Option<String> {
        self.inner.id.clone()
    }

    /// Check if the response contains tool calls
    fn has_tool_calls(&self) -> bool {
        self.inner.has_tool_calls()
    }

    /// Check if the response was truncated due to length
    fn is_truncated(&self) -> bool {
        self.inner.is_truncated()
    }

    /// Get the total token count
    fn total_tokens(&self) -> u32 {
        self.inner.total_tokens()
    }



    /// Set usage statistics (builder pattern)
    fn with_usage(&mut self, usage: &PyLlmUsage) -> PyResult<()> {
        self.inner = self.inner.clone().with_usage(usage.inner.clone());
        Ok(())
    }

    /// Add tool calls (builder pattern)
    fn with_tool_calls(&mut self, tool_calls: Vec<PyLlmToolCall>) -> PyResult<()> {
        let rust_tool_calls: Vec<LlmToolCall> = tool_calls
            .into_iter()
            .map(|tc| tc.inner)
            .collect();
        self.inner = self.inner.clone().with_tool_calls(rust_tool_calls);
        Ok(())
    }

    /// Set finish reason (builder pattern)
    fn with_finish_reason(&mut self, finish_reason: &PyFinishReason) -> PyResult<()> {
        self.inner = self.inner.clone().with_finish_reason(finish_reason.inner.clone());
        Ok(())
    }

    /// Set response ID (builder pattern)
    fn with_id(&mut self, id: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_id(id);
        Ok(())
    }

    /// Add metadata (builder pattern) - accepts JSON string
    fn with_metadata(&mut self, key: String, value: String) -> PyResult<()> {
        let json_value: serde_json::Value = serde_json::from_str(&value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON value: {}", e)))?;
        self.inner = self.inner.clone().with_metadata(key, json_value);
        Ok(())
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "LlmResponse(content='{}...', model='{}', usage={}, finish_reason={})",
            if self.inner.content.len() > 50 {
                format!("{}...", &self.inner.content[..50])
            } else {
                self.inner.content.clone()
            },
            self.inner.model,
            format!("LlmUsage(total={})", self.inner.usage.total_tokens),
            self.inner.finish_reason
        )
    }

    /// Convert to dictionary for JSON serialization
    fn to_dict<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("content", &self.inner.content)?;
        dict.set_item("model", &self.inner.model)?;
        dict.set_item("id", &self.inner.id)?;

        // Convert tool calls
        let tool_calls_list = PyList::empty(py);
        for tool_call in &self.inner.tool_calls {
            let tc_dict = PyLlmToolCall::from(tool_call.clone()).to_dict(py)?;
            tool_calls_list.append(tc_dict)?;
        }
        dict.set_item("tool_calls", tool_calls_list)?;

        // Add usage
        dict.set_item("usage", PyLlmUsage::from(self.inner.usage.clone()).to_dict(py)?)?;

        // Add finish reason
        dict.set_item("finish_reason", self.inner.finish_reason.to_string())?;

        // Add metadata
        let metadata_dict = PyDict::new(py);
        for (key, value) in &self.inner.metadata {
            let json_str = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
            metadata_dict.set_item(key, json_str)?;
        }
        dict.set_item("metadata", metadata_dict)?;

        Ok(dict)
    }
}

impl From<LlmResponse> for PyLlmResponse {
    fn from(response: LlmResponse) -> Self {
        Self { inner: response }
    }
}

impl From<PyLlmResponse> for LlmResponse {
    fn from(py_response: PyLlmResponse) -> Self {
        py_response.inner
    }
}
