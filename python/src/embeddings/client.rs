//! Embedding client for GraphBit Python bindings

use graphbit_core::embeddings::EmbeddingService;
use pyo3::prelude::*;
use std::sync::Arc;

use crate::runtime::get_runtime;
use super::config::EmbeddingConfig;

#[pyclass]
pub struct EmbeddingClient {
    service: Arc<EmbeddingService>,
}

#[pymethods]
impl EmbeddingClient {
    #[new]
    fn new(config: EmbeddingConfig) -> PyResult<Self> {
        let service = Arc::new(EmbeddingService::new(config.inner).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
        })?);
        Ok(Self { service })
    }

    fn embed(&self, text: String) -> PyResult<Vec<f32>> {
        let service = Arc::clone(&self.service);
        
        get_runtime().block_on(async move {
            let response = service.embed_text(&text).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok(response)
        })
    }

    fn embed_many(&self, texts: Vec<String>) -> PyResult<Vec<Vec<f32>>> {
        let service = Arc::clone(&self.service);
        
        get_runtime().block_on(async move {
            let response = service.embed_texts(&texts).await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok(response)
        })
    }

    #[staticmethod]
    fn similarity(a: Vec<f32>, b: Vec<f32>) -> PyResult<f32> {
        EmbeddingService::cosine_similarity(&a, &b)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
} 