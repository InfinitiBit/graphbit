//! Memory types for GraphBit Python bindings

use graphbit_core::memory::manager::MemoryStats as CoreMemoryStats;
use graphbit_core::memory::types::{MemoryEntry as CoreMemoryEntry, MemoryType as CoreMemoryType};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Memory type enumeration
#[pyclass(eq, eq_int)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemoryType {
    /// Short-term session-based memory
    Working,
    /// Long-term structured facts
    Factual,
    /// Conversation history
    Episodic,
    /// Pattern-based knowledge
    Semantic,
}

#[pymethods]
impl MemoryType {
    /// String representation
    fn __repr__(&self) -> String {
        format!("MemoryType.{:?}", self)
    }

    /// String conversion
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<MemoryType> for CoreMemoryType {
    fn from(mt: MemoryType) -> Self {
        match mt {
            MemoryType::Working => CoreMemoryType::Working,
            MemoryType::Factual => CoreMemoryType::Factual,
            MemoryType::Episodic => CoreMemoryType::Episodic,
            MemoryType::Semantic => CoreMemoryType::Semantic,
        }
    }
}

impl From<CoreMemoryType> for MemoryType {
    fn from(mt: CoreMemoryType) -> Self {
        match mt {
            CoreMemoryType::Working => MemoryType::Working,
            CoreMemoryType::Factual => MemoryType::Factual,
            CoreMemoryType::Episodic => MemoryType::Episodic,
            CoreMemoryType::Semantic => MemoryType::Semantic,
        }
    }
}

/// Memory entry containing stored information
#[pyclass]
#[derive(Clone)]
pub struct MemoryEntry {
    pub(crate) inner: CoreMemoryEntry,
}

#[pymethods]
impl MemoryEntry {
    /// Get the memory ID
    #[getter]
    fn id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Get the memory type
    #[getter]
    fn memory_type(&self) -> MemoryType {
        self.inner.memory_type.clone().into()
    }

    /// Get the content
    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    /// Get the importance score
    #[getter]
    fn importance(&self) -> f32 {
        self.inner.importance_score
    }

    /// Get the timestamp
    #[getter]
    fn timestamp(&self) -> String {
        self.inner.created_at.to_rfc3339()
    }

    /// Get the session ID if available
    #[getter]
    fn session_id(&self) -> Option<String> {
        self.inner.session_id.clone()
    }

    /// Get tags
    #[getter]
    fn tags(&self) -> Vec<String> {
        self.inner.metadata.tags.clone()
    }

    /// Get source
    #[getter]
    fn source(&self) -> String {
        self.inner.metadata.source.clone()
    }

    /// Get custom metadata as JSON strings
    #[getter]
    fn custom(&self) -> HashMap<String, String> {
        self.inner
            .metadata
            .custom
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "MemoryEntry(id={}, type={:?}, content={})",
            self.id(),
            self.memory_type(),
            &self.content()[..self.content().len().min(50)]
        )
    }
}

impl From<CoreMemoryEntry> for MemoryEntry {
    fn from(entry: CoreMemoryEntry) -> Self {
        Self { inner: entry }
    }
}

/// Memory statistics
#[pyclass]
#[derive(Clone)]
pub struct MemoryStats {
    pub(crate) inner: CoreMemoryStats,
}

#[pymethods]
impl MemoryStats {
    /// Total number of memories
    #[getter]
    fn total_memories(&self) -> usize {
        self.inner.total_memories
    }

    /// Number of working memories
    #[getter]
    fn working_count(&self) -> usize {
        self.inner.working_count
    }

    /// Number of factual memories
    #[getter]
    fn factual_count(&self) -> usize {
        self.inner.factual_count
    }

    /// Number of episodic memories
    #[getter]
    fn episodic_count(&self) -> usize {
        self.inner.episodic_count
    }

    /// Number of semantic memories
    #[getter]
    fn semantic_count(&self) -> usize {
        self.inner.semantic_count
    }

    /// Current session ID
    #[getter]
    fn current_session(&self) -> Option<String> {
        self.inner.current_session.clone()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "MemoryStats(total={}, working={}, factual={}, episodic={}, semantic={})",
            self.total_memories(),
            self.working_count(),
            self.factual_count(),
            self.episodic_count(),
            self.semantic_count()
        )
    }
}

impl From<CoreMemoryStats> for MemoryStats {
    fn from(stats: CoreMemoryStats) -> Self {
        Self { inner: stats }
    }
}
