//! Python bindings for semantic memory types

use graphbit_core::memory::types::MemoryEntry;
use pyo3::prelude::*;
use std::collections::HashMap;

/// Python wrapper for SemanticConcept
#[pyclass]
#[derive(Clone)]
pub struct SemanticConcept {
    /// Concept ID
    #[pyo3(get)]
    pub id: String,
    /// Concept name
    #[pyo3(get)]
    pub name: String,
    /// Concept description
    #[pyo3(get)]
    pub description: String,
    /// Confidence score (0.0-1.0)
    #[pyo3(get)]
    pub confidence: f32,
    /// Number of reinforcements
    #[pyo3(get)]
    pub reinforcement_count: u32,
    /// Related concept names
    #[pyo3(get)]
    pub related_concepts: Vec<String>,
}

#[pymethods]
impl SemanticConcept {
    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "SemanticConcept(name='{}', confidence={:.2}, reinforcements={})",
            self.name, self.confidence, self.reinforcement_count
        )
    }

    /// Get all properties as a dictionary
    fn to_dict(&self) -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("id".to_string(), self.id.clone());
        dict.insert("name".to_string(), self.name.clone());
        dict.insert("description".to_string(), self.description.clone());
        dict.insert("confidence".to_string(), self.confidence.to_string());
        dict.insert(
            "reinforcement_count".to_string(),
            self.reinforcement_count.to_string(),
        );
        dict.insert(
            "related_concepts".to_string(),
            format!("{:?}", self.related_concepts),
        );
        dict
    }
}

impl SemanticConcept {
    /// Create from MemoryEntry
    pub fn from_memory_entry(entry: &MemoryEntry) -> Option<Self> {
        // Extract concept metadata
        let name = entry
            .metadata
            .tags
            .iter()
            .find(|tag| *tag != "concept")
            .cloned()?;

        let description = entry.content.clone();

        let confidence = entry
            .metadata
            .custom
            .get("confidence")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5) as f32;

        let reinforcement_count = entry
            .metadata
            .custom
            .get("reinforcement_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;

        let concept_id = entry
            .metadata
            .custom
            .get("concept_id")
            .and_then(|v| v.as_str())
            .unwrap_or(&entry.id.to_string())
            .to_string();

        // Get related concept names from related memories
        let related_concepts = Vec::new(); // TODO: Extract from entry.related_memories

        Some(Self {
            id: concept_id,
            name,
            description,
            confidence,
            reinforcement_count,
            related_concepts,
        })
    }
}

