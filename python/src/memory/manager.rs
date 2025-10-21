//! Memory manager for GraphBit Python bindings

use super::config::MemoryConfig;
use super::decay::DecayStats;
use super::query::MemoryQuery;
use super::semantic::SemanticConcept;
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
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl MemoryManager {
    /// Create a new memory manager with default configuration
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<MemoryConfig>) -> PyResult<Self> {
        // Create a new tokio runtime for this memory manager
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create tokio runtime: {}",
                e
            ))
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

    /// Get all working memory items for current session
    fn get_all_items(&self) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_all_items().await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Count working memory items in current session
    fn count_items(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.count_items().await)
        })
    }

    /// Clear all working memory items in current session
    fn clear_items(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.clear_items().await.map_err(to_py_error)
        })
    }

    /// Set context variable for current session
    fn set_context(&self, key: String, value: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.set_context(key, value);
            Ok(())
        })
    }

    /// Get context variable for current session
    fn get_context(&self, key: String) -> PyResult<Option<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_context(&key))
        })
    }

    /// Get all context variables for current session
    fn get_all_context(&self) -> PyResult<std::collections::HashMap<String, String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_all_context())
        })
    }

    /// Clear all context variables for current session
    fn clear_context(&self) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.clear_context();
            Ok(())
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

    /// Retrieve a fact by key
    fn get_fact(&self, key: String) -> PyResult<Option<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_fact(&key).await)
        })
    }

    /// Update an existing fact
    fn update_fact(&self, key: String, value: String) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .update_fact(&key, value)
                .await
                .map_err(to_py_error)
        })
    }

    /// Delete a fact by key
    fn delete_fact(&self, key: String) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .delete_fact(&key)
                .await
                .map_err(to_py_error)
        })
    }

    /// List all facts as (key, value) tuples
    fn list_facts(&self) -> PyResult<Vec<(String, String)>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.list_facts().await)
        })
    }

    /// Check if a fact exists
    fn has_fact(&self, key: String) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.has_fact(&key).await)
        })
    }

    /// Store a fact with custom importance
    fn store_fact_with_importance(
        &self,
        key: String,
        value: String,
        importance: f32,
    ) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager
                .store_fact_with_importance(key, value, importance)
                .await
                .map(|id| id.to_string())
                .map_err(to_py_error)
        })
    }

    /// Get all facts as a dictionary
    fn get_all_facts(&self) -> PyResult<std::collections::HashMap<String, String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_all_facts().await)
        })
    }

    /// Count facts in current namespace
    fn count_facts(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.count_facts().await)
        })
    }

    /// Store a user preference
    fn store_preference(&self, key: String, value: String) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            manager
                .store_preference(key, value)
                .await
                .map(|id| id.to_string())
                .map_err(to_py_error)
        })
    }

    /// Get a user preference
    fn get_preference(&self, key: String) -> PyResult<Option<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_preference(&key).await)
        })
    }

    /// Search facts by pattern with optional importance filtering
    #[pyo3(signature = (pattern, min_importance=None, max_results=None))]
    fn search_facts(
        &self,
        pattern: String,
        min_importance: Option<f32>,
        max_results: Option<usize>,
    ) -> PyResult<Vec<(String, String)>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager
                .search_facts(&pattern, min_importance, max_results)
                .await)
        })
    }

    /// Get facts by importance range
    #[pyo3(signature = (min_importance, max_importance=None))]
    fn get_facts_by_importance(
        &self,
        min_importance: f32,
        max_importance: Option<f32>,
    ) -> PyResult<Vec<(String, String)>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager
                .get_facts_by_importance(min_importance, max_importance)
                .await)
        })
    }

    // Semantic Memory Methods

    /// Store a semantic concept
    fn store_concept(&self, name: String, description: String) -> PyResult<String> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            let concept =
                graphbit_core::memory::semantic::SemanticConcept::new(name, description);
            manager
                .store_concept(concept)
                .await
                .map(|id| id.to_string())
                .map_err(to_py_error)
        })
    }

    /// Get a concept by name
    fn get_concept(&self, name: String) -> PyResult<Option<super::semantic::SemanticConcept>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager
                .get_concept(&name)
                .await
                .and_then(|entry| super::semantic::SemanticConcept::from_memory_entry(&entry)))
        })
    }

    /// Reinforce a concept (increases confidence)
    fn reinforce_concept(&self, name: String) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .reinforce_concept(&name)
                .await
                .map_err(to_py_error)
        })
    }

    /// Connect two concepts
    fn connect_concepts(&self, from_concept: String, to_concept: String) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .connect_concepts(&from_concept, &to_concept)
                .await
                .map_err(to_py_error)
        })
    }

    /// Get related concepts
    fn get_related_concepts(&self, name: String) -> PyResult<Vec<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_related_concepts(&name).await;

            // Extract concept names from entries
            let names: Vec<String> = entries
                .iter()
                .filter_map(|entry| {
                    entry
                        .metadata
                        .tags
                        .iter()
                        .find(|tag| *tag != "concept")
                        .cloned()
                })
                .collect();

            Ok(names)
        })
    }

    /// List all concepts
    fn list_concepts(&self) -> PyResult<Vec<super::semantic::SemanticConcept>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.list_concepts().await;

            // Convert MemoryEntry to SemanticConcept
            let concepts: Vec<super::semantic::SemanticConcept> = entries
                .iter()
                .filter_map(|entry| super::semantic::SemanticConcept::from_memory_entry(entry))
                .collect();

            Ok(concepts)
        })
    }

    /// Count total number of concepts
    fn count_concepts(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.count_concepts().await)
        })
    }

    /// Get the concept relationship graph
    fn get_concept_graph(&self) -> PyResult<std::collections::HashMap<String, Vec<String>>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_concept_graph().await)
        })
    }

    /// Get concepts with confidence above threshold
    fn get_high_confidence_concepts(
        &self,
        min_confidence: f32,
    ) -> PyResult<Vec<super::semantic::SemanticConcept>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_high_confidence_concepts(min_confidence).await;

            // Convert MemoryEntry to SemanticConcept
            let concepts: Vec<super::semantic::SemanticConcept> = entries
                .iter()
                .filter_map(|entry| super::semantic::SemanticConcept::from_memory_entry(entry))
                .collect();

            Ok(concepts)
        })
    }

    /// Calculate similarity between two concepts
    ///
    /// Returns a similarity score from 0.0 (completely different) to 1.0 (very similar).
    /// Similarity is based on shared relationships and confidence score proximity.
    ///
    /// Args:
    ///     concept1_name: Name of the first concept
    ///     concept2_name: Name of the second concept
    ///
    /// Returns:
    ///     Similarity score (0.0-1.0)
    fn calculate_similarity(&self, concept1_name: String, concept2_name: String) -> PyResult<f32> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager
                .calculate_similarity(&concept1_name, &concept2_name)
                .await)
        })
    }

    /// Search for concepts matching a pattern
    ///
    /// Searches for concepts whose names contain the given pattern (case-insensitive).
    /// Results can be filtered by minimum confidence and limited to a maximum number.
    ///
    /// Args:
    ///     pattern: Substring to search for in concept names (case-insensitive)
    ///     min_confidence: Optional minimum confidence threshold (0.0-1.0)
    ///     max_results: Optional limit on number of results to return
    ///
    /// Returns:
    ///     List of SemanticConcept objects matching the criteria, sorted by confidence (descending)
    fn search_concepts(
        &self,
        pattern: String,
        min_confidence: Option<f32>,
        max_results: Option<usize>,
    ) -> PyResult<Vec<SemanticConcept>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager
                .search_concepts(&pattern, min_confidence, max_results)
                .await;

            // Convert MemoryEntry to SemanticConcept
            let concepts: Vec<SemanticConcept> = entries
                .iter()
                .filter_map(|entry| SemanticConcept::from_memory_entry(entry))
                .collect();

            Ok(concepts)
        })
    }

    // Episodic Memory Methods

    /// Start recording a new episode
    fn start_episode(&self, title: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.start_episode(title);
            Ok(())
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

    /// Add a participant to the current episode
    fn add_participant(&self, participant: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.add_participant(participant);
            Ok(())
        })
    }

    /// Set the outcome of the current episode
    fn set_outcome(&self, outcome: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.set_outcome(outcome);
            Ok(())
        })
    }

    /// Add a tag to the current episode
    fn add_tag(&self, tag: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager.add_tag(tag);
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

    /// Get episodes by participant
    fn get_episodes_by_participant(&self, participant: String) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_episodes_by_participant(&participant).await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Get episodes by tag
    fn get_episodes_by_tag(&self, tag: String) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_episodes_by_tag(&tag).await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Get episodes by time range
    fn get_episodes_by_timerange(
        &self,
        start: String,
        end: String,
    ) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            // Parse ISO 8601 datetime strings
            let start_dt = chrono::DateTime::parse_from_rfc3339(&start)
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid start datetime: {}",
                        e
                    ))
                })?
                .with_timezone(&chrono::Utc);

            let end_dt = chrono::DateTime::parse_from_rfc3339(&end)
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid end datetime: {}",
                        e
                    ))
                })?
                .with_timezone(&chrono::Utc);

            let manager = inner.read().await;
            let entries = manager.get_episodes_by_timerange(start_dt, end_dt).await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Get recent episodes
    fn get_recent_episodes(&self, limit: usize) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            let entries = manager.get_recent_episodes(limit).await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Count total episodes
    fn count_episodes(&self) -> PyResult<usize> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.count_episodes().await)
        })
    }

    /// Check if currently recording an episode
    fn is_recording(&self) -> PyResult<bool> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.is_recording())
        })
    }

    /// Search episodes with multiple filters
    #[pyo3(signature = (pattern, participants=None, tags=None, start_time=None, end_time=None, max_results=None))]
    fn search_episodes(
        &self,
        pattern: String,
        participants: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        start_time: Option<String>,
        end_time: Option<String>,
        max_results: Option<usize>,
    ) -> PyResult<Vec<MemoryEntry>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            // Parse datetime strings if provided
            let start_dt = if let Some(ref start_str) = start_time {
                Some(
                    chrono::DateTime::parse_from_rfc3339(start_str)
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Invalid start_time: {}",
                                e
                            ))
                        })?
                        .with_timezone(&chrono::Utc),
                )
            } else {
                None
            };

            let end_dt = if let Some(ref end_str) = end_time {
                Some(
                    chrono::DateTime::parse_from_rfc3339(end_str)
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Invalid end_time: {}",
                                e
                            ))
                        })?
                        .with_timezone(&chrono::Utc),
                )
            } else {
                None
            };

            let manager = inner.read().await;
            let entries = manager
                .search_episodes(&pattern, participants, tags, start_dt, end_dt, max_results)
                .await;
            Ok(entries.into_iter().map(MemoryEntry::from).collect())
        })
    }

    /// Get summaries of recent episodes
    fn get_episode_summaries(&self, limit: usize) -> PyResult<Vec<String>> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let manager = inner.read().await;
            Ok(manager.get_episode_summaries(limit).await)
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
            let id =
                graphbit_core::memory::types::MemoryId::from_string(&memory_id).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid memory ID: {}",
                        e
                    ))
                })?;

            let manager = inner.read().await;
            Ok(manager.get_memory(&id).await.map(MemoryEntry::from))
        })
    }

    /// Remove a memory by ID
    fn remove_memory(&self, memory_id: String) -> PyResult<bool> {
        let inner = self.inner.clone();

        self.runtime.block_on(async move {
            let id =
                graphbit_core::memory::types::MemoryId::from_string(&memory_id).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid memory ID: {}",
                        e
                    ))
                })?;

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

    /// Run memory decay and return statistics
    fn run_decay(&self) -> PyResult<DecayStats> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut manager = inner.write().await;
            manager
                .run_decay()
                .await
                .map(DecayStats::from)
                .map_err(to_py_error)
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
