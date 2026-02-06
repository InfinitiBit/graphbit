//! Memory configuration for GraphBit Python bindings

use pyo3::prelude::*;

use crate::embeddings::EmbeddingConfig;
use crate::llm::LlmConfig;

/// Configuration for the memory subsystem.
///
/// Wraps the core `MemoryConfig` and accepts Python-side `LlmConfig` and
/// `EmbeddingConfig` objects.
#[pyclass]
#[derive(Clone)]
pub struct MemoryConfig {
    pub(crate) inner: graphbit_core::memory::MemoryConfig,
}

#[pymethods]
impl MemoryConfig {
    /// Create a new memory configuration.
    ///
    /// # Arguments
    /// * `llm_config` - LLM provider configuration (e.g. `LlmConfig.openai(...)`)
    /// * `embedding_config` - Embedding provider configuration (e.g. `EmbeddingConfig.openai(...)`)
    /// * `db_path` - Optional path to the SQLite database file. Defaults to `"graphbit_memory.db"`.
    /// * `similarity_threshold` - Optional minimum cosine-similarity threshold (0.0-1.0).
    #[new]
    #[pyo3(signature = (llm_config, embedding_config, db_path=None, similarity_threshold=None))]
    fn new(
        llm_config: LlmConfig,
        embedding_config: EmbeddingConfig,
        db_path: Option<String>,
        similarity_threshold: Option<f64>,
    ) -> Self {
        let mut config =
            graphbit_core::memory::MemoryConfig::new(llm_config.inner, embedding_config.inner);

        if let Some(path) = db_path {
            config = config.with_db_path(path);
        }
        if let Some(threshold) = similarity_threshold {
            config = config.with_similarity_threshold(threshold);
        }

        Self { inner: config }
    }
}
