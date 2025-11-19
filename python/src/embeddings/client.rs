//! Embedding client for GraphBit Python bindings

use graphbit_core::embeddings::{
    EmbeddingBatchRequest, EmbeddingInput, EmbeddingRequest, EmbeddingService,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::Arc;

use super::config::EmbeddingConfig;
use crate::errors::to_py_runtime_error;
use crate::runtime::get_runtime;

/// Python client for generating text embeddings using various providers
#[pyclass]
pub struct EmbeddingClient {
    service: Arc<EmbeddingService>,
}

#[pymethods]
impl EmbeddingClient {
    #[new]
    fn new(config: EmbeddingConfig) -> PyResult<Self> {
        let service = Arc::new(EmbeddingService::new(config.inner).map_err(to_py_runtime_error)?);
        Ok(Self { service })
    }

    /// Generate embedding for a single text
    ///
    /// CRITICAL: This method releases the GIL during execution, enabling true parallelism
    /// from Python threads. Multiple threads can call this method simultaneously.
    fn embed(&self, py: Python<'_>, text: String) -> PyResult<Vec<f32>> {
        // Validate input
        if text.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Text input cannot be empty",
            ));
        }

        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        // CRITICAL FIX: Release GIL during async execution
        // This enables multiple Python threads to execute embed() in parallel
        py.allow_threads(|| {
            rt.block_on(async move {
                let response = service
                    .embed_text(&text)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(response)
            })
        })
    }

    /// Generate embeddings for multiple texts
    ///
    /// CRITICAL: This method releases the GIL during execution, enabling true parallelism
    /// from Python threads. Multiple threads can call this method simultaneously.
    fn embed_many(&self, py: Python<'_>, texts: Vec<String>) -> PyResult<Vec<Vec<f32>>> {
        // Validate input
        if texts.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Text list cannot be empty",
            ));
        }

        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        // CRITICAL FIX: Release GIL during async execution
        // This enables multiple Python threads to execute embed_many() in parallel
        py.allow_threads(|| {
            rt.block_on(async move {
                let response = service
                    .embed_texts(&texts)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(response)
            })
        })
    }

    /// Process a batch of embedding requests with lock-free parallel execution
    ///
    /// This method exposes GraphBit's lock-free parallel embedding engine to Python,
    /// enabling 10-50x speedup for batch embedding generation compared to sequential processing.
    ///
    /// # Arguments
    /// * `texts_batch` - List of text lists, where each inner list is a batch of texts
    /// * `max_concurrency` - Maximum number of concurrent requests (default: 10)
    /// * `timeout_ms` - Timeout for the entire batch in milliseconds (optional)
    ///
    /// # Returns
    /// Dictionary with:
    /// - `embeddings`: List of embedding lists (Vec<Vec<Vec<f32>>>)
    /// - `stats`: Batch processing statistics
    /// - `duration_ms`: Total processing time
    ///
    /// # Example
    /// ```python
    /// # Process 100 documents in parallel with 10 concurrent requests
    /// texts_batch = [chunks[i:i+10] for i in range(0, 100, 10)]
    /// result = client.embed_batch_parallel(texts_batch, max_concurrency=10)
    /// embeddings = result['embeddings']  # List of 10 embedding batches
    /// stats = result['stats']  # Processing statistics
    /// ```
    #[pyo3(signature = (texts_batch, max_concurrency=None, timeout_ms=None))]
    fn embed_batch_parallel(
        &self,
        py: Python<'_>,
        texts_batch: Vec<Vec<String>>,
        max_concurrency: Option<usize>,
        timeout_ms: Option<u64>,
    ) -> PyResult<Py<PyDict>> {
        // Validate input
        if texts_batch.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Batch cannot be empty",
            ));
        }

        // Build batch request
        let requests: Vec<EmbeddingRequest> = texts_batch
            .into_iter()
            .map(|texts| EmbeddingRequest {
                input: EmbeddingInput::Multiple(texts),
                user: None,
                params: HashMap::new(),
            })
            .collect();

        let batch_request = EmbeddingBatchRequest {
            requests,
            max_concurrency,
            timeout_ms,
        };

        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        // CRITICAL: Release GIL during lock-free parallel execution
        // This enables true parallelism with atomic operations for concurrency control
        let batch_response = py.allow_threads(|| {
            rt.block_on(async move {
                service
                    .process_batch(batch_request)
                    .await
                    .map_err(to_py_runtime_error)
            })
        })?;

        // Convert response to Python dictionary
        let result_dict = PyDict::new(py);

        // Extract embeddings from successful responses
        let mut all_embeddings: Vec<Vec<Vec<f32>>> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (idx, response_result) in batch_response.responses.into_iter().enumerate() {
            match response_result {
                Ok(response) => {
                    all_embeddings.push(response.embeddings);
                }
                Err(e) => {
                    errors.push(format!("Batch {}: {}", idx, e));
                    all_embeddings.push(Vec::new()); // Empty embeddings for failed batch
                }
            }
        }

        result_dict.set_item("embeddings", all_embeddings)?;
        result_dict.set_item("errors", errors)?;
        result_dict.set_item("duration_ms", batch_response.total_duration_ms)?;

        // Add statistics
        let stats_dict = PyDict::new(py);
        stats_dict.set_item(
            "successful_requests",
            batch_response.stats.successful_requests,
        )?;
        stats_dict.set_item("failed_requests", batch_response.stats.failed_requests)?;
        stats_dict.set_item(
            "avg_response_time_ms",
            batch_response.stats.avg_response_time_ms,
        )?;
        stats_dict.set_item("total_embeddings", batch_response.stats.total_embeddings)?;
        stats_dict.set_item("total_tokens", batch_response.stats.total_tokens)?;

        result_dict.set_item("stats", stats_dict)?;

        Ok(result_dict.unbind())
    }

    #[staticmethod]
    fn similarity(a: Vec<f32>, b: Vec<f32>) -> PyResult<f32> {
        EmbeddingService::cosine_similarity(&a, &b).map_err(to_py_runtime_error)
    }
}
