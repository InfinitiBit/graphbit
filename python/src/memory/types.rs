//! Python wrapper types for memory data objects.

use pyo3::prelude::*;
use std::collections::HashMap;

/// A stored memory fact exposed to Python.
#[pyclass]
#[derive(Clone)]
pub struct PyMemory {
    /// Unique identifier (UUID string).
    #[pyo3(get)]
    pub id: String,
    /// The fact / content of the memory.
    #[pyo3(get)]
    pub content: String,
    /// User-level scope (if set).
    #[pyo3(get)]
    pub user_id: Option<String>,
    /// Agent-level scope (if set).
    #[pyo3(get)]
    pub agent_id: Option<String>,
    /// Run-level scope (if set).
    #[pyo3(get)]
    pub run_id: Option<String>,
    /// ISO-8601 creation timestamp.
    #[pyo3(get)]
    pub created_at: String,
    /// ISO-8601 last-update timestamp.
    #[pyo3(get)]
    pub updated_at: String,
    /// JSON-serialised metadata.
    #[pyo3(get)]
    pub metadata: HashMap<String, String>,
}

#[pymethods]
impl PyMemory {
    fn __repr__(&self) -> String {
        format!("Memory(id='{}', content='{}')", self.id, self.content)
    }
}

impl From<graphbit_core::memory::Memory> for PyMemory {
    fn from(m: graphbit_core::memory::Memory) -> Self {
        let metadata: HashMap<String, String> = m
            .metadata
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect();

        Self {
            id: m.id.to_string(),
            content: m.content,
            user_id: m.scope.user_id,
            agent_id: m.scope.agent_id,
            run_id: m.scope.run_id,
            created_at: m.created_at.to_rfc3339(),
            updated_at: m.updated_at.to_rfc3339(),
            metadata,
        }
    }
}

/// A memory with its similarity score from a search.
#[pyclass]
#[derive(Clone)]
pub struct PyScoredMemory {
    /// The memory.
    #[pyo3(get)]
    pub memory: PyMemory,
    /// Cosine similarity score (0.0-1.0).
    #[pyo3(get)]
    pub score: f64,
}

#[pymethods]
impl PyScoredMemory {
    fn __repr__(&self) -> String {
        format!(
            "ScoredMemory(id='{}', score={:.4})",
            self.memory.id, self.score
        )
    }
}

impl From<graphbit_core::memory::ScoredMemory> for PyScoredMemory {
    fn from(s: graphbit_core::memory::ScoredMemory) -> Self {
        Self {
            memory: PyMemory::from(s.memory),
            score: s.score,
        }
    }
}

/// A historical record of a memory mutation.
#[pyclass]
#[derive(Clone)]
pub struct PyMemoryHistory {
    /// Which memory was affected (UUID string).
    #[pyo3(get)]
    pub memory_id: String,
    /// Content before the change.
    #[pyo3(get)]
    pub old_content: String,
    /// Content after the change.
    #[pyo3(get)]
    pub new_content: String,
    /// Action type (ADD, UPDATE, DELETE, NOOP).
    #[pyo3(get)]
    pub action: String,
    /// ISO-8601 timestamp.
    #[pyo3(get)]
    pub timestamp: String,
}

#[pymethods]
impl PyMemoryHistory {
    fn __repr__(&self) -> String {
        format!(
            "MemoryHistory(memory_id='{}', action='{}')",
            self.memory_id, self.action
        )
    }
}

impl From<graphbit_core::memory::MemoryHistory> for PyMemoryHistory {
    fn from(h: graphbit_core::memory::MemoryHistory) -> Self {
        Self {
            memory_id: h.memory_id.to_string(),
            old_content: h.old_content,
            new_content: h.new_content,
            action: h.action.to_string(),
            timestamp: h.timestamp.to_rfc3339(),
        }
    }
}
