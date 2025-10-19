//! Decay statistics for GraphBit Python bindings

use graphbit_core::memory::decay::DecayStats as CoreDecayStats;
use graphbit_core::memory::types::MemoryType as CoreMemoryType;
use pyo3::prelude::*;
use std::collections::HashMap;

use super::types::MemoryType;

/// Statistics from a memory decay run
#[pyclass]
#[derive(Clone)]
pub struct DecayStats {
    pub(crate) inner: CoreDecayStats,
}

#[pymethods]
impl DecayStats {
    /// Total memories checked during decay
    #[getter]
    fn total_checked(&self) -> usize {
        self.inner.total_checked
    }

    /// Number of memories forgotten (removed)
    #[getter]
    fn forgotten(&self) -> usize {
        self.inner.forgotten
    }

    /// Number of memories retained (kept but decayed)
    #[getter]
    fn retained(&self) -> usize {
        self.inner.retained
    }

    /// Number of memories protected from decay (high importance)
    #[getter]
    fn protected(&self) -> usize {
        self.inner.protected
    }

    /// Execution time in milliseconds
    #[getter]
    fn execution_time_ms(&self) -> u64 {
        self.inner.execution_time_ms
    }

    /// Get forgotten count by memory type
    fn forgotten_by_type(&self, memory_type: &MemoryType) -> usize {
        let core_type = match memory_type {
            MemoryType::Working => CoreMemoryType::Working,
            MemoryType::Factual => CoreMemoryType::Factual,
            MemoryType::Episodic => CoreMemoryType::Episodic,
            MemoryType::Semantic => CoreMemoryType::Semantic,
        };
        *self.inner.forgotten_by_type.get(&core_type).unwrap_or(&0)
    }

    /// Get all forgotten counts by type as a dictionary
    fn forgotten_by_type_dict(&self) -> HashMap<String, usize> {
        let mut result = HashMap::new();
        for (mem_type, count) in &self.inner.forgotten_by_type {
            let type_str = match mem_type {
                CoreMemoryType::Working => "Working",
                CoreMemoryType::Factual => "Factual",
                CoreMemoryType::Episodic => "Episodic",
                CoreMemoryType::Semantic => "Semantic",
            };
            result.insert(type_str.to_string(), *count);
        }
        result
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "DecayStats(total_checked={}, forgotten={}, retained={}, protected={}, execution_time_ms={})",
            self.total_checked(),
            self.forgotten(),
            self.retained(),
            self.protected(),
            self.execution_time_ms()
        )
    }
}

impl From<CoreDecayStats> for DecayStats {
    fn from(inner: CoreDecayStats) -> Self {
        Self { inner }
    }
}

