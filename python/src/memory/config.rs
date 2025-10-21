//! Memory configuration for GraphBit Python bindings

use graphbit_core::memory::manager::MemoryConfig as CoreMemoryConfig;
use pyo3::prelude::*;

/// Configuration for the memory system
#[pyclass]
#[derive(Clone)]
pub struct MemoryConfig {
    pub(crate) inner: CoreMemoryConfig,
}

#[pymethods]
impl MemoryConfig {
    /// Create a new memory configuration with default settings
    #[new]
    fn new() -> Self {
        Self {
            inner: CoreMemoryConfig::default(),
        }
    }

    /// Create a configuration with all memory types enabled
    #[staticmethod]
    fn with_defaults() -> Self {
        Self {
            inner: CoreMemoryConfig::default(),
        }
    }

    /// Create a minimal configuration (only working memory)
    #[staticmethod]
    fn minimal() -> Self {
        let mut config = CoreMemoryConfig::default();
        config.enable_factual = false;
        config.enable_episodic = false;
        config.enable_semantic = false;
        Self { inner: config }
    }

    /// Enable or disable working memory
    #[setter]
    fn set_enable_working(&mut self, value: bool) {
        self.inner.enable_working = value;
    }

    /// Get working memory enabled status
    #[getter]
    fn enable_working(&self) -> bool {
        self.inner.enable_working
    }

    /// Enable or disable factual memory
    #[setter]
    fn set_enable_factual(&mut self, value: bool) {
        self.inner.enable_factual = value;
    }

    /// Get factual memory enabled status
    #[getter]
    fn enable_factual(&self) -> bool {
        self.inner.enable_factual
    }

    /// Enable or disable episodic memory
    #[setter]
    fn set_enable_episodic(&mut self, value: bool) {
        self.inner.enable_episodic = value;
    }

    /// Get episodic memory enabled status
    #[getter]
    fn enable_episodic(&self) -> bool {
        self.inner.enable_episodic
    }

    /// Enable or disable semantic memory
    #[setter]
    fn set_enable_semantic(&mut self, value: bool) {
        self.inner.enable_semantic = value;
    }

    /// Get semantic memory enabled status
    #[getter]
    fn enable_semantic(&self) -> bool {
        self.inner.enable_semantic
    }

    /// Set auto embedding enabled
    #[setter]
    fn set_auto_embed(&mut self, value: bool) {
        self.inner.auto_embed = value;
    }

    /// Get auto embedding enabled status
    #[getter]
    fn auto_embed(&self) -> bool {
        self.inner.auto_embed
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "MemoryConfig(working={}, factual={}, episodic={}, semantic={}, auto_embed={})",
            self.enable_working(),
            self.enable_factual(),
            self.enable_episodic(),
            self.enable_semantic(),
            self.auto_embed()
        )
    }
}
