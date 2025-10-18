//! Semantic memory implementation for pattern-based knowledge
//!
//! Semantic memory stores general knowledge built from patterns and insights
//! over time, with graph connections between related concepts.

use super::storage::MemoryStorage;
use super::types::{MemoryEntry, MemoryId, MemoryMetadata, MemoryType};
use crate::errors::GraphBitResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A semantic concept or insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConcept {
    /// Concept ID
    pub id: String,
    /// Concept name/title
    pub name: String,
    /// Concept description
    pub description: String,
    /// Related concept IDs
    pub related_concepts: Vec<String>,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Number of times this concept has been reinforced
    pub reinforcement_count: u32,
}

impl SemanticConcept {
    /// Create a new semantic concept
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            related_concepts: Vec::with_capacity(4),
            confidence: 0.5,
            reinforcement_count: 1,
        }
    }

    /// Reinforce this concept (increases confidence)
    pub fn reinforce(&mut self) {
        self.reinforcement_count += 1;
        // Increase confidence with diminishing returns
        let boost = 0.1 * (1.0 - self.confidence);
        self.confidence = (self.confidence + boost).min(1.0);
    }

    /// Add a related concept
    pub fn add_relation(&mut self, concept_id: String) {
        if !self.related_concepts.contains(&concept_id) {
            self.related_concepts.push(concept_id);
        }
    }
}

/// Relationship between semantic concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    /// Source concept ID
    pub from_concept: String,
    /// Target concept ID
    pub to_concept: String,
    /// Relationship type (e.g., "is_a", "part_of", "related_to")
    pub relation_type: String,
    /// Strength of the relationship (0.0-1.0)
    pub strength: f32,
}

/// Semantic memory manager for pattern-based knowledge
#[derive(Debug)]
pub struct SemanticMemory {
    /// Concept graph (concept_id -> related concept IDs)
    concept_graph: HashMap<String, Vec<String>>,
}

impl SemanticMemory {
    /// Create a new semantic memory instance
    pub fn new() -> Self {
        Self {
            concept_graph: HashMap::with_capacity(100),
        }
    }

    /// Store a semantic concept
    pub fn store_concept(
        &mut self,
        concept: SemanticConcept,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let mut metadata = MemoryMetadata::new();
        metadata.set_source("semantic".to_string());
        metadata.add_tag("concept".to_string());
        metadata.add_tag(concept.name.clone());
        
        // Store concept metadata
        metadata.add_custom("concept_id".to_string(), serde_json::json!(concept.id));
        metadata.add_custom("confidence".to_string(), serde_json::json!(concept.confidence));
        metadata.add_custom(
            "reinforcement_count".to_string(),
            serde_json::json!(concept.reinforcement_count),
        );

        // Format content
        let content = format!(
            "Concept: {}\nDescription: {}\nConfidence: {:.2}\nReinforcements: {}",
            concept.name, concept.description, concept.confidence, concept.reinforcement_count
        );

        let mut entry = MemoryEntry::with_importance(
            content,
            MemoryType::Semantic,
            concept.confidence, // Use confidence as importance
            None,
        );
        entry.metadata = metadata;

        // Store related concepts in the entry
        for related_id in &concept.related_concepts {
            if let Ok(memory_id) = MemoryId::from_string(related_id) {
                entry.add_relation(memory_id);
            }
        }

        // Update concept graph
        self.concept_graph
            .insert(concept.id.clone(), concept.related_concepts.clone());

        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Retrieve a concept by name
    pub fn get_concept(&self, name: &str, storage: &dyn MemoryStorage) -> Option<MemoryEntry> {
        let concepts = storage.list_by_type(MemoryType::Semantic);
        
        concepts
            .into_iter()
            .find(|c| c.metadata.tags.contains(&name.to_string()))
            .cloned()
    }

    /// Reinforce an existing concept
    pub fn reinforce_concept(
        &self,
        name: &str,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<bool> {
        let concepts = storage.list_by_type(MemoryType::Semantic);

        // Find the concept ID first (to avoid borrowing issues)
        let mut concept_id_to_update: Option<MemoryId> = None;
        for concept in concepts {
            if concept.metadata.tags.contains(&name.to_string()) {
                concept_id_to_update = Some(concept.id.clone());
                break;
            }
        }

        // Now update the concept
        if let Some(concept_id) = concept_id_to_update {
            if let Some(entry) = storage.get_mut(&concept_id) {
                // Increase reinforcement count
                if let Some(count) = entry.metadata.custom.get("reinforcement_count") {
                    if let Some(count_val) = count.as_u64() {
                        let new_count = count_val + 1;
                        entry.metadata.add_custom(
                            "reinforcement_count".to_string(),
                            serde_json::json!(new_count),
                        );

                        // Increase confidence
                        if let Some(conf) = entry.metadata.custom.get("confidence") {
                            if let Some(conf_val) = conf.as_f64() {
                                let boost = 0.1 * (1.0 - conf_val);
                                let new_conf = (conf_val + boost).min(1.0);
                                entry.metadata.add_custom(
                                    "confidence".to_string(),
                                    serde_json::json!(new_conf),
                                );
                                entry.importance_score = new_conf as f32;
                            }
                        }
                    }
                }

                entry.record_access();
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Connect two concepts
    pub fn connect_concepts(
        &mut self,
        concept1_name: &str,
        concept2_name: &str,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<bool> {
        // Find both concepts
        let concepts = storage.list_by_type(MemoryType::Semantic);
        let mut concept1_id: Option<MemoryId> = None;
        let mut concept2_id: Option<MemoryId> = None;

        for concept in concepts {
            if concept.metadata.tags.contains(&concept1_name.to_string()) {
                concept1_id = Some(concept.id.clone());
            }
            if concept.metadata.tags.contains(&concept2_name.to_string()) {
                concept2_id = Some(concept.id.clone());
            }
        }

        if let (Some(id1), Some(id2)) = (concept1_id, concept2_id) {
            // Add bidirectional relationship
            if let Some(entry1) = storage.get_mut(&id1) {
                entry1.add_relation(id2.clone());
            }
            if let Some(entry2) = storage.get_mut(&id2) {
                entry2.add_relation(id1);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get related concepts
    pub fn get_related_concepts(
        &self,
        name: &str,
        storage: &dyn MemoryStorage,
    ) -> Vec<MemoryEntry> {
        if let Some(concept) = self.get_concept(name, storage) {
            concept
                .related_memories
                .iter()
                .filter_map(|id| storage.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// List all concepts
    pub fn list_concepts(&self, storage: &dyn MemoryStorage) -> Vec<MemoryEntry> {
        storage
            .list_by_type(MemoryType::Semantic)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Count concepts
    pub fn count_concepts(&self, storage: &dyn MemoryStorage) -> usize {
        storage.count_by_type(MemoryType::Semantic)
    }

    /// Get concept graph structure
    pub fn get_concept_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.concept_graph
    }

    /// Find concepts by confidence threshold
    pub fn get_high_confidence_concepts(
        &self,
        min_confidence: f32,
        storage: &dyn MemoryStorage,
    ) -> Vec<MemoryEntry> {
        let concepts = storage.list_by_type(MemoryType::Semantic);
        
        concepts
            .into_iter()
            .filter(|c| {
                c.metadata
                    .custom
                    .get("confidence")
                    .and_then(|v| v.as_f64())
                    .map(|conf| conf >= min_confidence as f64)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::storage::InMemoryStorage;

    #[test]
    fn test_semantic_concept_creation() {
        let mut concept = SemanticConcept::new(
            "Machine Learning".to_string(),
            "AI technique for pattern recognition".to_string(),
        );

        assert_eq!(concept.confidence, 0.5);
        assert_eq!(concept.reinforcement_count, 1);

        concept.reinforce();
        assert_eq!(concept.reinforcement_count, 2);
        assert!(concept.confidence > 0.5);
    }

    #[test]
    fn test_semantic_memory_storage() {
        let mut semantic = SemanticMemory::new();
        let mut storage = InMemoryStorage::new();

        let concept = SemanticConcept::new(
            "GraphBit".to_string(),
            "Agentic workflow framework".to_string(),
        );

        semantic.store_concept(concept, &mut storage).unwrap();
        assert_eq!(semantic.count_concepts(&storage), 1);

        let retrieved = semantic.get_concept("GraphBit", &storage);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_concept_reinforcement() {
        let mut semantic = SemanticMemory::new();
        let mut storage = InMemoryStorage::new();

        let concept = SemanticConcept::new("Test".to_string(), "Test concept".to_string());
        semantic.store_concept(concept, &mut storage).unwrap();

        let reinforced = semantic.reinforce_concept("Test", &mut storage).unwrap();
        assert!(reinforced);
    }

    #[test]
    fn test_concept_connections() {
        let mut semantic = SemanticMemory::new();
        let mut storage = InMemoryStorage::new();

        let concept1 = SemanticConcept::new("AI".to_string(), "Artificial Intelligence".to_string());
        let concept2 = SemanticConcept::new("ML".to_string(), "Machine Learning".to_string());

        semantic.store_concept(concept1, &mut storage).unwrap();
        semantic.store_concept(concept2, &mut storage).unwrap();

        let connected = semantic
            .connect_concepts("AI", "ML", &mut storage)
            .unwrap();
        assert!(connected);

        let related = semantic.get_related_concepts("AI", &storage);
        assert_eq!(related.len(), 1);
    }
}

