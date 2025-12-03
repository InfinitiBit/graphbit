//! Python bridge provider for calling Python LLM implementations from Rust
//!
//! This module provides a bridge to call Python-based LLM implementations
//! (like HuggingFace) from Rust code, enabling seamless integration of
//! Python ML libraries with the Rust workflow execution system.
//!
//! # Response Format Support
//!
//! The `parse_python_response()` function has been refactored to support multiple
//! Python LLM provider response formats with graceful fallbacks:
//!
//! 1. **OpenAI-compatible format** (default, preserves backward compatibility):
//!    ```json
//!    {
//!      "choices": [{"message": {"content": "...", "reasoning_content": "..."}, "finish_reason": "stop"}],
//!      "usage": {"prompt_tokens": N, "completion_tokens": N}
//!    }
//!    ```
//!
//! 2. **Direct HuggingFace transformers format**:
//!    ```json
//!    [{"generated_text": "..."}]
//!    ```
//!
//! 3. **Simple text formats**:
//!    - Plain string: `"generated text"`
//!    - Dict with text: `{"text": "..."}`
//!    - Dict with generated_text: `{"generated_text": "..."}`
//!
//! # Error Handling
//!
//! The refactored implementation provides:
//! - **Graceful fallbacks**: Missing metadata (usage, finish_reason) doesn't cause failures
//! - **Detailed error messages**: When all formats fail, the error includes:
//!   - List of attempted formats and why each failed
//!   - Python object type information
//!   - Debug representation of the response structure
//! - **Type safety**: Handles both u32 and i64 for token counts (Python int compatibility)
//!
//! # Design Principles
//!
//! - **Backward compatibility**: Existing HuggingFace Inference API integration continues working
//! - **Extensibility**: New formats can be added by implementing new `try_*_format()` methods
//! - **Fail gracefully**: Text extraction is prioritized over metadata extraction
//! - **Debug-friendly**: Error messages help developers understand what went wrong

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{FinishReason, LlmRequest, LlmResponse, LlmUsage};
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};

/// Python bridge provider for calling Python LLM implementations
///
/// This provider wraps a Python object that implements the LLM interface
/// and forwards requests to it, converting between Rust and Python types.
#[cfg(feature = "python")]
pub struct PythonBridgeProvider {
    python_instance: Arc<PyObject>,
    model: String,
}

#[cfg(feature = "python")]
impl PythonBridgeProvider {
    /// Create a new Python bridge provider
    ///
    /// # Arguments
    /// * `python_instance` - A Python object that implements the LLM interface (must have a `chat` method)
    /// * `model` - The model name to use
    ///
    /// # Returns
    /// A new `PythonBridgeProvider` instance
    pub fn new(python_instance: Arc<PyObject>, model: String) -> GraphBitResult<Self> {
        Ok(Self {
            python_instance,
            model,
        })
    }

    /// Parse Python response to GraphBit response
    ///
    /// Supports multiple response formats from different Python LLM providers:
    /// 1. OpenAI-compatible format: `{"choices": [{"message": {"content": "..."}}], "usage": {...}}`
    /// 2. Direct HuggingFace transformers format: `[{"generated_text": "..."}]`
    /// 3. Simple text formats: plain string, `{"text": "..."}`, or `{"generated_text": "..."}`
    ///
    /// The function tries each format in order and provides detailed error messages if all fail.
    fn parse_python_response(&self, py: Python, result: PyObject) -> GraphBitResult<LlmResponse> {
        // Collect error information for debugging
        let mut error_details = Vec::new();

        // Try OpenAI-compatible format first (preserves backward compatibility)
        if let Some(response) = Self::try_openai_format(py, &result, &self.model) {
            return Ok(response);
        } else {
            error_details.push("OpenAI format: Missing 'choices' attribute or invalid structure");
        }

        // Try direct HuggingFace transformers format: [{"generated_text": "..."}]
        if let Some(response) = Self::try_transformers_format(py, &result, &self.model) {
            return Ok(response);
        } else {
            error_details
                .push("HuggingFace transformers format: Not a list or missing 'generated_text'");
        }

        // Try simple text formats
        if let Some(response) = Self::try_simple_text_format(py, &result, &self.model) {
            return Ok(response);
        } else {
            error_details.push(
                "Simple text format: Not a string, missing 'text', or missing 'generated_text'",
            );
        }

        // All formats failed - construct comprehensive error message
        let python_type = result
            .bind(py)
            .get_type()
            .name()
            .map(|n| n.to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        let debug_repr = result
            .bind(py)
            .repr()
            .map(|r| r.to_string())
            .unwrap_or_else(|_| "<unable to get repr>".to_string());

        Err(GraphBitError::llm_provider(
            "python_bridge",
            format!(
                "Failed to parse Python LLM response. Tried all supported formats:\n\
                 - {}\n\
                 Python object type: {}\n\
                 Python object repr (truncated): {}",
                error_details.join("\n - "),
                python_type,
                if debug_repr.len() > 200 {
                    format!("{}...", &debug_repr[..200])
                } else {
                    debug_repr
                }
            ),
        ))
    }

    /// Try to parse OpenAI-compatible format
    ///
    /// Expected structure:
    /// ```json
    /// {
    ///   "choices": [{"message": {"content": "...", "reasoning_content": "..."}, "finish_reason": "stop"}],
    ///   "usage": {"prompt_tokens": N, "completion_tokens": N}
    /// }
    /// ```
    fn try_openai_format(py: Python, result: &PyObject, model: &str) -> Option<LlmResponse> {
        // Try to get choices array
        let choices = result.getattr(py, "choices").ok()?;
        let first_choice = choices.call_method1(py, "__getitem__", (0,)).ok()?;
        let message = first_choice.getattr(py, "message").ok()?;

        // Extract content (required)
        let mut content: String = message.getattr(py, "content").ok()?.extract(py).ok()?;

        // If content is empty, try reasoning_content (for models like Kimi)
        if content.is_empty() {
            if let Ok(reasoning_content) = message.getattr(py, "reasoning_content") {
                if let Ok(reasoning_str) = reasoning_content.extract::<String>(py) {
                    if !reasoning_str.is_empty() {
                        content = reasoning_str;
                    }
                }
            }
        }

        // Extract metadata safely (these are optional)
        let usage = Self::extract_usage_safely(py, result);
        let finish_reason = Self::extract_finish_reason_safely(py, result);

        let mut response = LlmResponse::new(content, model).with_finish_reason(finish_reason);

        if let Some(usage) = usage {
            response = response.with_usage(usage);
        }

        Some(response)
    }

    /// Try to parse direct HuggingFace transformers format
    ///
    /// Expected structure:
    /// ```json
    /// [{"generated_text": "..."}]
    /// ```
    fn try_transformers_format(py: Python, result: &PyObject, model: &str) -> Option<LlmResponse> {
        // Check if result is a list
        let bound = result.bind(py);
        let list = bound.downcast::<PyList>().ok()?;

        // Get first element
        if list.is_empty() {
            return None;
        }

        let first_item = list.get_item(0).ok()?;

        // Try to extract generated_text
        let content: String = first_item.getattr("generated_text").ok()?.extract().ok()?;

        // Transformers format typically doesn't include usage or finish_reason
        // Use defaults
        Some(LlmResponse::new(content, model).with_finish_reason(FinishReason::Stop))
    }

    /// Try to parse simple text formats
    ///
    /// Supports:
    /// - Plain string: `"generated text"`
    /// - Dict with text: `{"text": "..."}`
    /// - Dict with generated_text: `{"generated_text": "..."}`
    fn try_simple_text_format(py: Python, result: &PyObject, model: &str) -> Option<LlmResponse> {
        // Try direct string extraction
        if let Ok(content) = result.extract::<String>(py) {
            return Some(LlmResponse::new(content, model).with_finish_reason(FinishReason::Stop));
        }

        // Try {"text": "..."}
        if let Ok(text_obj) = result.getattr(py, "text") {
            if let Ok(content) = text_obj.extract::<String>(py) {
                return Some(
                    LlmResponse::new(content, model).with_finish_reason(FinishReason::Stop),
                );
            }
        }

        // Try {"generated_text": "..."} (single dict, not in array)
        if let Ok(text_obj) = result.getattr(py, "generated_text") {
            if let Ok(content) = text_obj.extract::<String>(py) {
                return Some(
                    LlmResponse::new(content, model).with_finish_reason(FinishReason::Stop),
                );
            }
        }

        None
    }

    /// Safely extract usage information from response
    ///
    /// Returns `None` if usage information is missing or malformed.
    fn extract_usage_safely(py: Python, result: &PyObject) -> Option<LlmUsage> {
        let usage_obj = result.getattr(py, "usage").ok()?;

        // Try to extract token counts - handle both u32 and i64 (Python int)
        let prompt_tokens = usage_obj
            .getattr(py, "prompt_tokens")
            .ok()?
            .extract::<i64>(py)
            .ok()
            .and_then(|v| u32::try_from(v).ok())?;

        let completion_tokens = usage_obj
            .getattr(py, "completion_tokens")
            .ok()?
            .extract::<i64>(py)
            .ok()
            .and_then(|v| u32::try_from(v).ok())?;

        Some(LlmUsage::new(prompt_tokens, completion_tokens))
    }

    /// Safely extract finish reason from response
    ///
    /// Returns `FinishReason::Stop` as default if finish_reason is missing or unrecognized.
    fn extract_finish_reason_safely(py: Python, result: &PyObject) -> FinishReason {
        result
            .getattr(py, "choices")
            .ok()
            .and_then(|choices| choices.call_method1(py, "__getitem__", (0,)).ok())
            .and_then(|choice| choice.getattr(py, "finish_reason").ok())
            .and_then(|reason| reason.extract::<String>(py).ok())
            .and_then(|reason_str| match reason_str.as_str() {
                "stop" => Some(FinishReason::Stop),
                "length" => Some(FinishReason::Length),
                "tool_calls" => Some(FinishReason::ToolCalls),
                "content_filter" => Some(FinishReason::ContentFilter),
                _ => Some(FinishReason::Other(reason_str)),
            })
            .unwrap_or(FinishReason::Stop)
    }
}

#[cfg(feature = "python")]
#[async_trait]
impl LlmProviderTrait for PythonBridgeProvider {
    fn provider_name(&self) -> &str {
        "python_bridge"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        // Execute Python call directly without spawn_blocking or block_in_place
        // The Python GIL will be acquired and released as needed
        // The caller (Python bindings) must release the GIL before entering the async runtime
        let python_instance = Arc::clone(&self.python_instance);
        let model = self.model.clone();

        Python::with_gil(|py| {
            // Convert Rust messages to Python format
            let messages = PyList::empty(py);
            for msg in &request.messages {
                let dict = PyDict::new(py);
                // Convert LlmRole to string manually
                let role_str = match msg.role {
                    crate::llm::LlmRole::User => "user",
                    crate::llm::LlmRole::Assistant => "assistant",
                    crate::llm::LlmRole::System => "system",
                    crate::llm::LlmRole::Tool => "tool",
                };
                dict.set_item("role", role_str).map_err(|e| {
                    GraphBitError::llm_provider("python_bridge", format!("Failed to set role: {e}"))
                })?;
                dict.set_item("content", &msg.content).map_err(|e| {
                    GraphBitError::llm_provider(
                        "python_bridge",
                        format!("Failed to set content: {e}"),
                    )
                })?;
                messages.append(dict).map_err(|e| {
                    GraphBitError::llm_provider(
                        "python_bridge",
                        format!("Failed to append message: {e}"),
                    )
                })?;
            }

            // Prepare kwargs for the Python call
            let kwargs = PyDict::new(py);
            if let Some(max_tokens) = request.max_tokens {
                kwargs.set_item("max_tokens", max_tokens).map_err(|e| {
                    GraphBitError::llm_provider(
                        "python_bridge",
                        format!("Failed to set max_tokens: {e}"),
                    )
                })?;
            }
            if let Some(temperature) = request.temperature {
                kwargs.set_item("temperature", temperature).map_err(|e| {
                    GraphBitError::llm_provider(
                        "python_bridge",
                        format!("Failed to set temperature: {e}"),
                    )
                })?;
            }
            if let Some(top_p) = request.top_p {
                kwargs.set_item("top_p", top_p).map_err(|e| {
                    GraphBitError::llm_provider(
                        "python_bridge",
                        format!("Failed to set top_p: {e}"),
                    )
                })?;
            }

            // Call Python method: chat(model, messages, **kwargs)
            let result = python_instance
                .call_method(py, "chat", (model.clone(), messages), Some(&kwargs))
                .map_err(|e| {
                    GraphBitError::llm_provider("python_bridge", format!("Python call failed: {e}"))
                })?;

            // Parse the response
            let provider = PythonBridgeProvider {
                python_instance: python_instance.clone(),
                model,
            };
            provider.parse_python_response(py, result)
        })
    }

    fn supports_function_calling(&self) -> bool {
        false
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}

// Stub implementation when python feature is not enabled
/// Stub implementation of PythonBridgeProvider when python feature is disabled
#[cfg(not(feature = "python"))]
pub struct PythonBridgeProvider;

#[cfg(not(feature = "python"))]
impl PythonBridgeProvider {
    /// Create a new Python bridge provider (stub)
    pub fn new(_python_instance: (), _model: String) -> GraphBitResult<Self> {
        Err(GraphBitError::config(
            "Python bridge provider requires 'python' feature to be enabled",
        ))
    }
}

#[cfg(not(feature = "python"))]
#[async_trait]
impl LlmProviderTrait for PythonBridgeProvider {
    fn provider_name(&self) -> &str {
        "python_bridge"
    }

    fn model_name(&self) -> &str {
        "unavailable"
    }

    async fn complete(&self, _request: LlmRequest) -> GraphBitResult<LlmResponse> {
        Err(GraphBitError::config(
            "Python bridge provider requires 'python' feature to be enabled",
        ))
    }

    fn supports_function_calling(&self) -> bool {
        false
    }

    fn supports_streaming(&self) -> bool {
        false
    }
}
