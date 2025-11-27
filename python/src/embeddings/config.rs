//! Embedding configuration for GraphBit Python bindings

use crate::validation::validate_api_key;
use graphbit_core::embeddings::{EmbeddingConfig as CoreEmbeddingConfig, EmbeddingProvider};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Configuration for embedding providers and models
#[pyclass]
#[derive(Clone)]
pub struct EmbeddingConfig {
    pub(crate) inner: CoreEmbeddingConfig,
}

#[pymethods]
impl EmbeddingConfig {
    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn openai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenAI")?;

        Ok(Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::OpenAI,
                api_key,
                model: model.unwrap_or_else(|| "text-embedding-3-small".to_string()),
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
                python_instance: None,
            },
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn huggingface(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        Python::with_gil(|py| {
            // Import the Python HuggingFace embeddings class
            let hf_module = py
                .import("graphbit.providers.huggingface.embeddings")
                .map_err(|e| {
                    pyo3::exceptions::PyImportError::new_err(format!(
                        "Failed to import HuggingFace embeddings module: {e}"
                    ))
                })?;
            let hf_class = hf_module.getattr("HuggingfaceEmbeddings").map_err(|e| {
                pyo3::exceptions::PyAttributeError::new_err(format!(
                    "Failed to get HuggingfaceEmbeddings class: {e}"
                ))
            })?;

            // Create instance with API key (token parameter)
            let hf_instance = hf_class.call1((api_key.clone(),)).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "Failed to create HuggingfaceEmbeddings instance: {e}"
                ))
            })?;

            Ok(Self {
                inner: CoreEmbeddingConfig {
                    provider: EmbeddingProvider::PythonBridge,
                    api_key,
                    model: model
                        .unwrap_or_else(|| "sentence-transformers/all-MiniLM-L6-v2".to_string()),
                    base_url: None,
                    timeout_seconds: None,
                    max_batch_size: None,
                    extra_params: HashMap::new(),
                    python_instance: Some(std::sync::Arc::new(hf_instance.into())),
                },
            })
        })
    }
}
