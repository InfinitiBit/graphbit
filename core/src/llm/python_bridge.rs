//! Python bridge provider for calling Python LLM implementations from Rust
//!
//! This module provides a bridge to call Python-based LLM implementations
//! (like HuggingFace) from Rust code, enabling seamless integration of
//! Python ML libraries with the Rust workflow execution system.

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
    /// Extracts content from the HuggingFace-style response format:
    /// response.choices[0].message.content
    fn parse_python_response(&self, py: Python, result: PyObject) -> GraphBitResult<LlmResponse> {
        // Extract content from HuggingFace response format
        // The response follows the OpenAI-compatible format:
        // response.choices[0].message.content
        // Some models (like Kimi) may use reasoning_content instead
        let message = result
            .getattr(py, "choices")
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "python_bridge",
                    format!("Failed to get 'choices' attribute: {e}"),
                )
            })?
            .call_method1(py, "__getitem__", (0,))
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "python_bridge",
                    format!("Failed to get first choice: {e}"),
                )
            })?
            .getattr(py, "message")
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "python_bridge",
                    format!("Failed to get 'message' attribute: {e}"),
                )
            })?;

        // Try to get content, fallback to reasoning_content if content is empty
        let mut content: String = message
            .getattr(py, "content")
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "python_bridge",
                    format!("Failed to get 'content' attribute: {e}"),
                )
            })?
            .extract(py)
            .map_err(|e| {
                GraphBitError::llm_provider(
                    "python_bridge",
                    format!("Failed to extract content as string: {e}"),
                )
            })?;

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

        // Try to extract usage information if available
        let usage = result
            .getattr(py, "usage")
            .ok()
            .and_then(|usage_obj| {
                let prompt_tokens = usage_obj
                    .getattr(py, "prompt_tokens")
                    .ok()?
                    .extract::<u32>(py)
                    .ok()?;
                let completion_tokens = usage_obj
                    .getattr(py, "completion_tokens")
                    .ok()?
                    .extract::<u32>(py)
                    .ok()?;
                Some(LlmUsage::new(prompt_tokens, completion_tokens))
            });

        // Try to extract finish reason if available
        let finish_reason = result
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
                _ => None,
            })
            .unwrap_or(FinishReason::Stop);

        let mut response = LlmResponse::new(content, &self.model).with_finish_reason(finish_reason);

        if let Some(usage) = usage {
            response = response.with_usage(usage);
        }

        Ok(response)
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
        // Execute Python call in a blocking context
        // We use block_in_place instead of spawn_blocking to avoid GIL deadlocks
        // block_in_place runs the blocking code on the current thread, which is safer for Python GIL
        let python_instance = Arc::clone(&self.python_instance);
        let model = self.model.clone();

        // Use block_in_place to run blocking Python code without moving to a different thread
        // This avoids potential GIL deadlocks that can occur with spawn_blocking
        tokio::task::block_in_place(|| {
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
                    dict.set_item("role", role_str)
                        .map_err(|e| {
                            GraphBitError::llm_provider(
                                "python_bridge",
                                format!("Failed to set role: {e}"),
                            )
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
                        GraphBitError::llm_provider(
                            "python_bridge",
                            format!("Python call failed: {e}"),
                        )
                    })?;

                // Parse the response
                let provider = PythonBridgeProvider {
                    python_instance: python_instance.clone(),
                    model,
                };
                provider.parse_python_response(py, result)
            })
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

