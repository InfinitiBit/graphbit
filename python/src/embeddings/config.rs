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
            },
        })
    }

    #[staticmethod]
    fn huggingface(api_key: String, model: String) -> PyResult<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        Ok(Self {
            inner: CoreEmbeddingConfig {
                provider: EmbeddingProvider::HuggingFace,
                api_key,
                model,
                base_url: None,
                timeout_seconds: None,
                max_batch_size: None,
                extra_params: HashMap::new(),
            },
        })
    }
}
