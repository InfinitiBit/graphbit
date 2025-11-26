//! Python bridge provider for calling Python embedding implementations
//!
//! This provider wraps a Python object that implements the embedding interface
//! and forwards requests to it, converting between Rust and Python types.

use crate::embeddings::{
    EmbeddingConfig, EmbeddingProviderTrait, EmbeddingRequest, EmbeddingResponse, EmbeddingUsage,
};
use crate::errors::{GraphBitError, GraphBitResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyList;

/// Python bridge provider for calling Python embedding implementations
///
/// This provider wraps a Python object that implements the embedding interface
/// and forwards requests to it, converting between Rust and Python types.
#[cfg(feature = "python")]
pub struct PythonBridgeEmbeddingProvider {
    python_instance: Arc<PyObject>,
    model: String,
}

#[cfg(feature = "python")]
impl PythonBridgeEmbeddingProvider {
    /// Create a new Python bridge embedding provider
    pub fn new(config: EmbeddingConfig) -> GraphBitResult<Self> {
        let python_instance = config.python_instance.ok_or_else(|| {
            GraphBitError::config("PythonBridge provider requires python_instance to be set")
        })?;

        Ok(Self {
            python_instance,
            model: config.model,
        })
    }

    /// Parse Python response to EmbeddingResponse
    fn parse_python_response(
        &self,
        py: Python<'_>,
        result: PyObject,
    ) -> GraphBitResult<EmbeddingResponse> {
        // The Python embedding method should return a list of floats (single embedding)
        // or a list of lists of floats (multiple embeddings)

        let result_bound = result.bind(py);
        let embeddings: Vec<Vec<f32>> = if let Ok(list) = result_bound.downcast::<PyList>() {
            if list.is_empty() {
                return Err(GraphBitError::llm("Empty embedding response from Python"));
            }

            // Check if it's a single embedding (list of floats) or multiple (list of lists)
            let first_item = list.get_item(0).map_err(|e| {
                GraphBitError::llm(format!(
                    "Failed to get first item from Python response: {e}"
                ))
            })?;

            if first_item.downcast::<PyList>().is_ok() {
                // Multiple embeddings (list of lists)
                list.iter()
                    .map(|item| {
                        item.downcast::<PyList>()
                            .map_err(|e| {
                                GraphBitError::llm(format!("Invalid embedding format: {e}"))
                            })?
                            .iter()
                            .map(|v| {
                                v.extract::<f32>().map_err(|e| {
                                    GraphBitError::llm(format!("Failed to extract float: {e}"))
                                })
                            })
                            .collect::<Result<Vec<f32>, _>>()
                    })
                    .collect::<Result<Vec<Vec<f32>>, _>>()?
            } else {
                // Single embedding (list of floats)
                let embedding: Vec<f32> = list
                    .iter()
                    .map(|v| {
                        v.extract::<f32>().map_err(|e| {
                            GraphBitError::llm(format!("Failed to extract float: {e}"))
                        })
                    })
                    .collect::<Result<Vec<f32>, _>>()?;
                vec![embedding]
            }
        } else {
            return Err(GraphBitError::llm(
                "Python embedding response must be a list",
            ));
        };

        // Estimate token usage (Python providers typically don't provide this)
        let total_chars: usize = embeddings.iter().map(|e| e.len()).sum();
        let estimated_tokens = (total_chars / 4) as u32;

        let usage = EmbeddingUsage {
            prompt_tokens: estimated_tokens,
            total_tokens: estimated_tokens,
        };

        Ok(EmbeddingResponse {
            embeddings,
            model: self.model.clone(),
            usage,
            metadata: HashMap::new(),
        })
    }
}

#[cfg(feature = "python")]
#[async_trait]
impl EmbeddingProviderTrait for PythonBridgeEmbeddingProvider {
    fn provider_name(&self) -> &str {
        "python_bridge"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn generate_embeddings(
        &self,
        request: EmbeddingRequest,
    ) -> GraphBitResult<EmbeddingResponse> {
        let python_instance = Arc::clone(&self.python_instance);
        let model = self.model.clone();

        Python::with_gil(|py| {
            // Convert input to appropriate format
            let texts = request.input.as_texts();

            // Call Python method: embed(model, text, **kwargs) for single text
            // or embed_many(model, texts, **kwargs) for multiple texts
            let result = if texts.len() == 1 {
                // Single text - call embed(model, text)
                python_instance
                    .call_method(py, "embed", (model.clone(), texts[0]), None)
                    .map_err(|e| GraphBitError::llm(format!("Python embed call failed: {e}")))?
            } else {
                // Multiple texts - call embed_many(model, texts)
                let texts_vec: Vec<String> = texts.iter().map(|s| s.to_string()).collect();
                python_instance
                    .call_method(py, "embed_many", (model.clone(), texts_vec), None)
                    .map_err(|e| {
                        GraphBitError::llm(format!("Python embed_many call failed: {e}"))
                    })?
            };

            // Parse the response
            self.parse_python_response(py, result)
        })
    }

    async fn get_embedding_dimensions(&self) -> GraphBitResult<usize> {
        // Make a test request to determine dimensions
        let test_request = EmbeddingRequest {
            input: crate::embeddings::EmbeddingInput::Single("test".to_string()),
            user: None,
            params: HashMap::new(),
        };
        let response = self.generate_embeddings(test_request).await?;
        Ok(response
            .embeddings
            .first()
            .map(Vec::<f32>::len)
            .unwrap_or(768))
    }

    fn max_batch_size(&self) -> usize {
        100 // Conservative default
    }
}

// Stub implementation when python feature is not enabled
#[cfg(not(feature = "python"))]
pub struct PythonBridgeEmbeddingProvider;

#[cfg(not(feature = "python"))]
impl PythonBridgeEmbeddingProvider {
    /// Create a new Python bridge embedding provider (stub)
    pub fn new(_config: EmbeddingConfig) -> GraphBitResult<Self> {
        Err(GraphBitError::config(
            "Python bridge embedding provider requires 'python' feature to be enabled",
        ))
    }
}

#[cfg(not(feature = "python"))]
#[async_trait]
impl EmbeddingProviderTrait for PythonBridgeEmbeddingProvider {
    fn provider_name(&self) -> &str {
        "python_bridge"
    }

    fn model_name(&self) -> &str {
        "unavailable"
    }

    async fn generate_embeddings(
        &self,
        _request: EmbeddingRequest,
    ) -> GraphBitResult<EmbeddingResponse> {
        Err(GraphBitError::config(
            "Python bridge embedding provider requires 'python' feature to be enabled",
        ))
    }

    async fn get_embedding_dimensions(&self) -> GraphBitResult<usize> {
        Err(GraphBitError::config(
            "Python bridge embedding provider requires 'python' feature to be enabled",
        ))
    }
}
