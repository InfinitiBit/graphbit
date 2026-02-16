//! Memory client for GraphBit Python bindings

use std::sync::Arc;

use pyo3::prelude::*;

use crate::errors::to_py_runtime_error;
use crate::runtime::get_runtime;

use super::config::MemoryConfig;
use super::types::{PyMemory, PyMemoryHistory, PyScoredMemory};

/// High-level memory client for Python consumers.
///
/// Wraps the core `MemoryService` and exposes synchronous methods that release
/// the GIL during async execution.
#[pyclass]
pub struct MemoryClient {
    service: Arc<graphbit_core::memory::MemoryService>,
}

#[pymethods]
impl MemoryClient {
    /// Create a new memory client.
    ///
    /// This opens the SQLite database, loads existing memories into the
    /// vector index, and prepares the LLM and embedding providers.
    #[new]
    fn new(py: Python<'_>, config: MemoryConfig) -> PyResult<Self> {
        let cfg = config.inner;
        let rt = get_runtime();

        let service = py.allow_threads(|| {
            rt.block_on(async move {
                graphbit_core::memory::MemoryService::new(cfg)
                    .await
                    .map_err(to_py_runtime_error)
            })
        })?;

        Ok(Self {
            service: Arc::new(service),
        })
    }

    /// Extract and store facts from conversation messages.
    ///
    /// # Arguments
    /// * `messages` - List of `(role, content)` tuples.
    /// * `user_id` - Optional user-level scope.
    /// * `agent_id` - Optional agent-level scope.
    /// * `run_id` - Optional run-level scope.
    ///
    /// # Returns
    /// A list of newly created or updated `PyMemory` objects.
    #[pyo3(signature = (messages, user_id=None, agent_id=None, run_id=None))]
    fn add(
        &self,
        py: Python<'_>,
        messages: Vec<(String, String)>,
        user_id: Option<String>,
        agent_id: Option<String>,
        run_id: Option<String>,
    ) -> PyResult<Vec<PyMemory>> {
        let llm_messages = messages_to_llm(messages);
        let scope = graphbit_core::memory::MemoryScope {
            user_id,
            agent_id,
            run_id,
        };

        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let memories = service
                    .add(&llm_messages, &scope)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(memories.into_iter().map(PyMemory::from).collect())
            })
        })
    }

    /// Search for memories similar to a query.
    ///
    /// # Arguments
    /// * `query` - The search query text.
    /// * `user_id` / `agent_id` / `run_id` - Optional scope filters.
    /// * `top_k` - Maximum number of results (default 10).
    #[pyo3(signature = (query, user_id=None, agent_id=None, run_id=None, top_k=None))]
    fn search(
        &self,
        py: Python<'_>,
        query: String,
        user_id: Option<String>,
        agent_id: Option<String>,
        run_id: Option<String>,
        top_k: Option<usize>,
    ) -> PyResult<Vec<PyScoredMemory>> {
        let scope = graphbit_core::memory::MemoryScope {
            user_id,
            agent_id,
            run_id,
        };
        let k = top_k.unwrap_or(10);

        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let results = service
                    .search(&query, &scope, k)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(results.into_iter().map(PyScoredMemory::from).collect())
            })
        })
    }

    /// Get a single memory by its ID.
    fn get(&self, py: Python<'_>, memory_id: String) -> PyResult<Option<PyMemory>> {
        let id = parse_memory_id(&memory_id)?;
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let mem = service.get(&id).await.map_err(to_py_runtime_error)?;
                Ok(mem.map(PyMemory::from))
            })
        })
    }

    /// Get all memories matching the given scope.
    #[pyo3(signature = (user_id=None, agent_id=None, run_id=None))]
    fn get_all(
        &self,
        py: Python<'_>,
        user_id: Option<String>,
        agent_id: Option<String>,
        run_id: Option<String>,
    ) -> PyResult<Vec<PyMemory>> {
        let scope = graphbit_core::memory::MemoryScope {
            user_id,
            agent_id,
            run_id,
        };
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let memories = service
                    .get_all(&scope)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(memories.into_iter().map(PyMemory::from).collect())
            })
        })
    }

    /// Update a memory's content.
    fn update(&self, py: Python<'_>, memory_id: String, content: String) -> PyResult<PyMemory> {
        let id = parse_memory_id(&memory_id)?;
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let mem = service
                    .update(&id, &content)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(PyMemory::from(mem))
            })
        })
    }

    /// Delete a single memory by its ID.
    fn delete(&self, py: Python<'_>, memory_id: String) -> PyResult<()> {
        let id = parse_memory_id(&memory_id)?;
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move { service.delete(&id).await.map_err(to_py_runtime_error) })
        })
    }

    /// Delete all memories matching the given scope.
    #[pyo3(signature = (user_id=None, agent_id=None, run_id=None))]
    fn delete_all(
        &self,
        py: Python<'_>,
        user_id: Option<String>,
        agent_id: Option<String>,
        run_id: Option<String>,
    ) -> PyResult<()> {
        let scope = graphbit_core::memory::MemoryScope {
            user_id,
            agent_id,
            run_id,
        };
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(
                async move { service.delete_all(&scope).await.map_err(to_py_runtime_error) },
            )
        })
    }

    /// Get the mutation history for a memory.
    fn history(&self, py: Python<'_>, memory_id: String) -> PyResult<Vec<PyMemoryHistory>> {
        let id = parse_memory_id(&memory_id)?;
        let service = Arc::clone(&self.service);
        let rt = get_runtime();

        py.allow_threads(|| {
            rt.block_on(async move {
                let entries = service
                    .history(&id)
                    .await
                    .map_err(to_py_runtime_error)?;
                Ok(entries.into_iter().map(PyMemoryHistory::from).collect())
            })
        })
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn messages_to_llm(messages: Vec<(String, String)>) -> Vec<graphbit_core::llm::LlmMessage> {
    messages
        .into_iter()
        .map(|(role, content)| match role.as_str() {
            "system" => graphbit_core::llm::LlmMessage::system(content),
            "assistant" => graphbit_core::llm::LlmMessage::assistant(content),
            _ => graphbit_core::llm::LlmMessage::user(content),
        })
        .collect()
}

fn parse_memory_id(s: &str) -> PyResult<graphbit_core::memory::MemoryId> {
    graphbit_core::memory::MemoryId::from_string(s).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid memory ID '{s}': {e}"))
    })
}
