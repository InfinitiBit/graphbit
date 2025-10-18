//! Memory manager for GraphBit Python bindings

use super::config::MemoryConfig;
use super::query::MemoryQuery;
use super::types::{MemoryEntry, MemoryStats};
use crate::errors::to_py_error;
use graphbit_core::memory::manager::MemoryManager as CoreMemoryManager;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Memory manager for stateful agent memory
#[pyclass]
pub struct MemoryManager {
    pub(crate) inner: Arc<RwLock<CoreMemoryManager>>,
    runtime: tokio::runtime::Handle,
}

#[pymethods]
impl MemoryManager {
    /// Create a new memory manager with default configuration
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<MemoryConfig>) -> PyResult<Self> {
        let runtime = tokio::runtime::Handle::try_current().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "No tokio runtime found. Please ensure you're running in an async context.",
            )
        })?;

        let core_config = config.map(|c| c.inner).unwrap_or_default();
        let manager = CoreMemoryManager::new(core_config, None);

        Ok(Self {
            inner: Arc::new(RwLock::new(manager)),
            runtime,
        })
    }

    /// Create a memory manager with default settings
    #[staticmethod]
    fn with_defaults() -> PyResult<Self> {
        Self::new(None)
    }

    /// Start a new session
    fn start_session(&self, session_id: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.start_session(session_id);
            Ok(())
        })
    }

    /// End the current session
    fn end_session(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.end_session().await.map_err(to_py_error)
        })
    }

    /// Store a working memory
    fn store_working(&self, content: String) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager
                .store_working(content)
                .await
                .map(|id| id.to_string())
                .map_err(to_py_error)
        })
    }

    /// Store a factual memory
    fn store_fact(&self, key: String, value: String) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager
                .store_fact(key, value)
                .await
                .map(|id| id.to_string())
                .map_err(to_py_error)
        })
    }

    /// Add content to the current episode
    fn add_to_episode(&self, content: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.add_to_episode(content);
            Ok(())
        })
    }

    /// End the current episode and store it
    fn end_episode(&self) -> PyResult<Option<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .end_episode()
                .await
                .map(|opt_id| opt_id.map(|id| id.to_string()))
                .map_err(to_py_error)
        })
    }

    /// Retrieve memories matching a query
    fn retrieve(&self, query: MemoryQuery) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        let core_query = query.inner;

        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager
                .retrieve(core_query)
                .await
                .map(|results| {
                    results
                        .into_iter()
                        .map(|r| MemoryEntry::from(r.entry))
                        .collect()
                })
                .map_err(to_py_error)
        })
    }

    /// Get a specific memory by ID
    fn get_memory(&self, memory_id: String) -> PyResult<Option<MemoryEntry>> {
        let inner = self.inner.clone();

        self.runtime.block_on(async move {
            let id = graphbit_core::memory::types::MemoryId::from_string(&memory_id).map_err(
                |e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid memory ID: {}",
                        e
                    ))
                },
            )?;

            let manager = inner.read().await;
            Ok(manager.get_memory(&id).await.map(MemoryEntry::from))
        })
    }

    /// Remove a memory by ID
    fn remove_memory(&self, memory_id: String) -> PyResult<bool> {
        let inner = self.inner.clone();

        self.runtime.block_on(async move {
            let id = graphbit_core::memory::types::MemoryId::from_string(&memory_id).map_err(
                |e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid memory ID: {}",
                        e
                    ))
                },
            )?;

            let manager = inner.read().await;
            manager.remove_memory(&id).await.map_err(to_py_error)
        })
    }

    /// Get working memory context for LLM injection
    fn get_working_context(&self) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_working_context().await)
        })
    }

    /// Get current session ID
    fn get_current_session(&self) -> PyResult<Option<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_current_session().await)
        })
    }

    /// Get memory statistics
    fn get_stats(&self) -> PyResult<MemoryStats> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(MemoryStats::from(manager.get_stats().await))
        })
    }

    /// Run memory decay
    fn run_decay(&self) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.run_decay().await.map(|_| ()).map_err(to_py_error)
        })
    }

    /// Clear all memories
    fn clear_all(&self) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager.clear_all().await.map_err(to_py_error)
        })
    }

    /// String representation
    fn __repr__(&self) -> String {
        "MemoryManager()".to_string()
    }
}

