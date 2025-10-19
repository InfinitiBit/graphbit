//! Tests for semantic memory

use graphbit_core::memory::semantic::{SemanticConcept, SemanticMemory};
use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;

#[test]
fn test_semantic_memory_creation() {
    let _semantic = SemanticMemory::new();

    // Just verify creation succeeds
}

#[test]
fn test_semantic_concept_creation() {
    let concept = SemanticConcept::new(
        "programming".to_string(),
        "The art of writing code".to_string(),
    );

    assert_eq!(concept.name, "programming");
    assert_eq!(concept.description, "The art of writing code");
    assert_eq!(concept.confidence, 0.5);
    assert_eq!(concept.reinforcement_count, 1);
    assert!(concept.related_concepts.is_empty());
}

#[test]
fn test_semantic_concept_reinforce() {
    let mut concept = SemanticConcept::new("rust".to_string(), "A systems language".to_string());

    let initial_confidence = concept.confidence;
    let initial_count = concept.reinforcement_count;

    concept.reinforce();

    assert_eq!(concept.reinforcement_count, initial_count + 1);
    assert!(concept.confidence > initial_confidence);
}

#[test]
fn test_semantic_concept_add_relation() {
    let mut concept = SemanticConcept::new("rust".to_string(), "A language".to_string());

    concept.add_relation("programming".to_string());
    concept.add_relation("systems".to_string());
    concept.add_relation("programming".to_string()); // Duplicate

    assert_eq!(concept.related_concepts.len(), 2);
}

#[test]
fn test_semantic_memory_store_concept() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let concept = SemanticConcept::new("rust".to_string(), "A systems language".to_string());
    let memory_id = semantic.store_concept(concept, &mut storage).unwrap();

    // Verify the memory was stored
    let stored = storage.get(&memory_id).unwrap();
    assert!(stored.content.contains("rust"));
    assert_eq!(stored.memory_type, MemoryType::Semantic);
}

#[test]
fn test_semantic_memory_get_concept() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let concept = SemanticConcept::new("python".to_string(), "A dynamic language".to_string());
    semantic.store_concept(concept, &mut storage).unwrap();

    let retrieved = semantic.get_concept("python", &storage);
    assert!(retrieved.is_some());

    let entry = retrieved.unwrap();
    assert!(entry.content.contains("python"));
}

#[test]
fn test_semantic_memory_get_nonexistent_concept() {
    let storage = InMemoryStorage::new();
    let semantic = SemanticMemory::new();

    let retrieved = semantic.get_concept("nonexistent", &storage);
    assert!(retrieved.is_none());
}

#[test]
fn test_semantic_memory_reinforce_concept() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let concept = SemanticConcept::new(
        "learning".to_string(),
        "The process of acquiring knowledge".to_string(),
    );
    semantic.store_concept(concept, &mut storage).unwrap();

    // Reinforce the concept
    let result = semantic
        .reinforce_concept("learning", &mut storage)
        .unwrap();
    assert!(result, "Should successfully reinforce existing concept");

    let retrieved = semantic.get_concept("learning", &storage).unwrap();
    // Check that reinforcement count increased
    let count = retrieved
        .metadata
        .custom
        .get("reinforcement_count")
        .unwrap();
    assert_eq!(count.as_u64().unwrap(), 2);
}

#[test]
fn test_semantic_memory_reinforce_nonexistent() {
    let mut storage = InMemoryStorage::new();
    let semantic = SemanticMemory::new();

    let result = semantic
        .reinforce_concept("nonexistent", &mut storage)
        .unwrap();
    assert!(!result, "Should return false for nonexistent concept");
}

#[test]
fn test_semantic_memory_list_concepts() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    // Store multiple concepts
    semantic
        .store_concept(
            SemanticConcept::new("ai".to_string(), "Artificial Intelligence".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("ml".to_string(), "Machine Learning".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("nlp".to_string(), "Natural Language Processing".to_string()),
            &mut storage,
        )
        .unwrap();

    let all_concepts = semantic.list_concepts(&storage);

    assert_eq!(all_concepts.len(), 3);
}

#[test]
fn test_semantic_memory_count_concepts() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    assert_eq!(semantic.count_concepts(&storage), 0);

    semantic
        .store_concept(
            SemanticConcept::new("concept1".to_string(), "Description 1".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("concept2".to_string(), "Description 2".to_string()),
            &mut storage,
        )
        .unwrap();

    assert_eq!(semantic.count_concepts(&storage), 2);
}

#[test]
fn test_semantic_concept_with_relations() {
    let mut concept = SemanticConcept::new("programming".to_string(), "Writing code".to_string());
    concept.add_relation("coding".to_string());
    concept.add_relation("software".to_string());

    assert_eq!(concept.related_concepts.len(), 2);
}

#[test]
fn test_semantic_memory_metadata() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let concept = SemanticConcept::new("test".to_string(), "Test concept".to_string());
    let memory_id = semantic.store_concept(concept, &mut storage).unwrap();

    let stored = storage.get(&memory_id).unwrap();

    assert_eq!(stored.metadata.source, "semantic");
    assert!(stored.metadata.tags.contains(&"concept".to_string()));
    assert!(stored.metadata.tags.contains(&"test".to_string()));
}

#[test]
fn test_semantic_memory_importance() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let mut concept = SemanticConcept::new(
        "important_concept".to_string(),
        "Very important".to_string(),
    );
    concept.confidence = 0.9;
    let memory_id = semantic.store_concept(concept, &mut storage).unwrap();

    let stored = storage.get(&memory_id).unwrap();

    // Importance should match confidence
    assert_eq!(stored.importance_score, 0.9);
}

#[test]
fn test_semantic_memory_confidence_bounds() {
    let mut concept = SemanticConcept::new("max_confidence".to_string(), "Test".to_string());
    concept.confidence = 0.95;

    // Reinforce multiple times
    for _ in 0..10 {
        concept.reinforce();
    }

    assert!(
        concept.confidence <= 1.0,
        "Confidence should not exceed 1.0"
    );
    assert!(concept.confidence >= 0.95, "Confidence should increase");
}

#[test]
fn test_semantic_memory_connect_concepts() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    // Store two concepts
    semantic
        .store_concept(
            SemanticConcept::new("rust".to_string(), "A language".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("programming".to_string(), "Writing code".to_string()),
            &mut storage,
        )
        .unwrap();

    // Connect them
    let result = semantic
        .connect_concepts("rust", "programming", &mut storage)
        .unwrap();
    assert!(result, "Should successfully connect concepts");

    // Verify connection
    let related = semantic.get_related_concepts("rust", &storage);
    assert_eq!(related.len(), 1);
}

#[test]
fn test_semantic_memory_get_related_concepts() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    // Store three concepts
    semantic
        .store_concept(
            SemanticConcept::new("rust".to_string(), "A language".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("python".to_string(), "Another language".to_string()),
            &mut storage,
        )
        .unwrap();
    semantic
        .store_concept(
            SemanticConcept::new("java".to_string(), "Yet another language".to_string()),
            &mut storage,
        )
        .unwrap();

    // Connect rust to both python and java
    semantic
        .connect_concepts("rust", "python", &mut storage)
        .unwrap();
    semantic
        .connect_concepts("rust", "java", &mut storage)
        .unwrap();

    let related = semantic.get_related_concepts("rust", &storage);
    assert_eq!(related.len(), 2);
}

#[test]
fn test_semantic_memory_get_high_confidence_concepts() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let mut high_conf = SemanticConcept::new("high".to_string(), "High confidence".to_string());
    high_conf.confidence = 0.9;
    semantic.store_concept(high_conf, &mut storage).unwrap();

    let mut low_conf = SemanticConcept::new("low".to_string(), "Low confidence".to_string());
    low_conf.confidence = 0.3;
    semantic.store_concept(low_conf, &mut storage).unwrap();

    let high_concepts = semantic.get_high_confidence_concepts(0.7, &storage);
    assert_eq!(high_concepts.len(), 1);
}

#[test]
fn test_semantic_memory_concept_serialization() {
    let concept =
        SemanticConcept::new("serializable".to_string(), "Test serialization".to_string());

    let serialized = serde_json::to_string(&concept).unwrap();
    let deserialized: SemanticConcept = serde_json::from_str(&serialized).unwrap();

    assert_eq!(concept.name, deserialized.name);
    assert_eq!(concept.description, deserialized.description);
    assert_eq!(concept.confidence, deserialized.confidence);
}

#[test]
fn test_semantic_memory_special_characters_in_name() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let concept = SemanticConcept::new(
        "concept-with-dashes_and_underscores".to_string(),
        "Special chars".to_string(),
    );
    semantic.store_concept(concept, &mut storage).unwrap();

    let retrieved = semantic.get_concept("concept-with-dashes_and_underscores", &storage);
    assert!(retrieved.is_some());
}

#[test]
fn test_semantic_memory_long_concept_name() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let long_name = "a".repeat(200);
    let concept = SemanticConcept::new(long_name.clone(), "Long name test".to_string());
    semantic.store_concept(concept, &mut storage).unwrap();

    let retrieved = semantic.get_concept(&long_name, &storage);
    assert!(retrieved.is_some());
}

#[test]
fn test_semantic_memory_concept_graph() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    let mut concept = SemanticConcept::new("test".to_string(), "Test concept".to_string());
    concept.add_relation("related1".to_string());
    concept.add_relation("related2".to_string());

    let concept_id = concept.id.clone();
    semantic.store_concept(concept, &mut storage).unwrap();

    let graph = semantic.get_concept_graph();
    assert!(graph.contains_key(&concept_id));
    assert_eq!(graph.get(&concept_id).unwrap().len(), 2);
}

#[test]
fn test_semantic_memory_multiple_reinforcements() {
    let mut storage = InMemoryStorage::new();
    let mut semantic = SemanticMemory::new();

    semantic
        .store_concept(
            SemanticConcept::new("test".to_string(), "Test".to_string()),
            &mut storage,
        )
        .unwrap();

    // Reinforce multiple times
    for _ in 0..5 {
        semantic.reinforce_concept("test", &mut storage).unwrap();
    }

    let retrieved = semantic.get_concept("test", &storage).unwrap();
    let count = retrieved
        .metadata
        .custom
        .get("reinforcement_count")
        .unwrap();
    assert_eq!(count.as_u64().unwrap(), 6); // 1 initial + 5 reinforcements
}

// ============================================================================
// MemoryManager Semantic Methods Tests (Phase 3)
// ============================================================================

use graphbit_core::memory::manager::MemoryManager;

#[tokio::test]
async fn test_manager_store_concept_success() {
    let mut manager = MemoryManager::with_defaults();

    let concept = SemanticConcept::new("AI".to_string(), "Artificial Intelligence".to_string());
    let result = manager.store_concept(concept).await;

    assert!(result.is_ok(), "Should successfully store concept");
    let memory_id = result.unwrap();
    assert!(!memory_id.to_string().is_empty());
}

#[tokio::test]
async fn test_manager_get_concept_existing() {
    let mut manager = MemoryManager::with_defaults();

    // Store a concept
    let concept = SemanticConcept::new("Rust".to_string(), "A systems language".to_string());
    manager.store_concept(concept).await.unwrap();

    // Retrieve it
    let retrieved = manager.get_concept("Rust").await;
    assert!(retrieved.is_some(), "Should retrieve existing concept");

    let entry = retrieved.unwrap();
    assert!(entry.content.contains("Rust"));
    assert!(entry.content.contains("systems language"));
}

#[tokio::test]
async fn test_manager_get_concept_nonexistent() {
    let manager = MemoryManager::with_defaults();

    // Try to get non-existent concept
    let retrieved = manager.get_concept("NonExistent").await;
    assert!(
        retrieved.is_none(),
        "Should return None for non-existent concept"
    );
}

#[tokio::test]
async fn test_manager_reinforce_concept_existing() {
    let mut manager = MemoryManager::with_defaults();

    // Store a concept
    let concept = SemanticConcept::new("Python".to_string(), "A programming language".to_string());
    manager.store_concept(concept).await.unwrap();

    // Reinforce it
    let result = manager.reinforce_concept("Python").await;
    assert!(result.is_ok());
    assert!(result.unwrap(), "Should return true for existing concept");

    // Verify reinforcement increased
    let retrieved = manager.get_concept("Python").await.unwrap();
    let count = retrieved
        .metadata
        .custom
        .get("reinforcement_count")
        .unwrap()
        .as_u64()
        .unwrap();
    assert_eq!(count, 2); // 1 initial + 1 reinforcement
}

#[tokio::test]
async fn test_manager_reinforce_concept_nonexistent() {
    let mut manager = MemoryManager::with_defaults();

    // Try to reinforce non-existent concept
    let result = manager.reinforce_concept("NonExistent").await;
    assert!(result.is_ok());
    assert!(
        !result.unwrap(),
        "Should return false for non-existent concept"
    );
}

#[tokio::test]
async fn test_manager_connect_concepts_both_exist() {
    let mut manager = MemoryManager::with_defaults();

    // Store two concepts
    let concept1 = SemanticConcept::new("AI".to_string(), "Artificial Intelligence".to_string());
    let concept2 = SemanticConcept::new("ML".to_string(), "Machine Learning".to_string());
    manager.store_concept(concept1).await.unwrap();
    manager.store_concept(concept2).await.unwrap();

    // Connect them
    let result = manager.connect_concepts("AI", "ML").await;
    assert!(result.is_ok());
    assert!(result.unwrap(), "Should successfully connect concepts");

    // Verify connection
    let related = manager.get_related_concepts("AI").await;
    assert_eq!(related.len(), 1, "AI should have 1 related concept");
}

#[tokio::test]
async fn test_manager_connect_concepts_from_missing() {
    let mut manager = MemoryManager::with_defaults();

    // Store only one concept
    let concept = SemanticConcept::new("ML".to_string(), "Machine Learning".to_string());
    manager.store_concept(concept).await.unwrap();

    // Try to connect with non-existent concept
    let result = manager.connect_concepts("NonExistent", "ML").await;
    assert!(result.is_ok());
    assert!(
        !result.unwrap(),
        "Should return false when from concept doesn't exist"
    );
}

#[tokio::test]
async fn test_manager_connect_concepts_to_missing() {
    let mut manager = MemoryManager::with_defaults();

    // Store only one concept
    let concept = SemanticConcept::new("AI".to_string(), "Artificial Intelligence".to_string());
    manager.store_concept(concept).await.unwrap();

    // Try to connect to non-existent concept
    let result = manager.connect_concepts("AI", "NonExistent").await;
    assert!(result.is_ok());
    assert!(
        !result.unwrap(),
        "Should return false when to concept doesn't exist"
    );
}

#[tokio::test]
async fn test_manager_get_related_concepts_existing() {
    let mut manager = MemoryManager::with_defaults();

    // Store three concepts
    let concept1 = SemanticConcept::new("Programming".to_string(), "Writing code".to_string());
    let concept2 = SemanticConcept::new("Rust".to_string(), "A language".to_string());
    let concept3 = SemanticConcept::new("Python".to_string(), "Another language".to_string());
    manager.store_concept(concept1).await.unwrap();
    manager.store_concept(concept2).await.unwrap();
    manager.store_concept(concept3).await.unwrap();

    // Connect Programming to both languages
    manager
        .connect_concepts("Programming", "Rust")
        .await
        .unwrap();
    manager
        .connect_concepts("Programming", "Python")
        .await
        .unwrap();

    // Get related concepts
    let related = manager.get_related_concepts("Programming").await;
    assert_eq!(
        related.len(),
        2,
        "Programming should have 2 related concepts"
    );
}

#[tokio::test]
async fn test_manager_get_related_concepts_none() {
    let mut manager = MemoryManager::with_defaults();

    // Store a concept with no relationships
    let concept = SemanticConcept::new("Isolated".to_string(), "No connections".to_string());
    manager.store_concept(concept).await.unwrap();

    // Get related concepts
    let related = manager.get_related_concepts("Isolated").await;
    assert_eq!(related.len(), 0, "Should have no related concepts");
}

#[tokio::test]
async fn test_manager_semantic_concept_graph() {
    let mut manager = MemoryManager::with_defaults();

    // Build a concept graph: AI -> ML -> DL
    let ai = SemanticConcept::new("AI".to_string(), "Artificial Intelligence".to_string());
    let ml = SemanticConcept::new("ML".to_string(), "Machine Learning".to_string());
    let dl = SemanticConcept::new("DL".to_string(), "Deep Learning".to_string());

    manager.store_concept(ai).await.unwrap();
    manager.store_concept(ml).await.unwrap();
    manager.store_concept(dl).await.unwrap();

    manager.connect_concepts("AI", "ML").await.unwrap();
    manager.connect_concepts("ML", "DL").await.unwrap();

    // Verify graph structure
    let ai_related = manager.get_related_concepts("AI").await;
    assert_eq!(ai_related.len(), 1);

    let ml_related = manager.get_related_concepts("ML").await;
    assert_eq!(ml_related.len(), 2); // Connected to both AI and DL
}

#[tokio::test]
async fn test_manager_semantic_memory_disabled() {
    use graphbit_core::memory::manager::MemoryConfig;

    let mut config = MemoryConfig::minimal();
    config.enable_semantic = false;

    let mut manager = MemoryManager::new(config, None);

    // Try to store concept when semantic is disabled
    let concept = SemanticConcept::new("Test".to_string(), "Should fail".to_string());
    let result = manager.store_concept(concept).await;
    assert!(
        result.is_err(),
        "Should fail when semantic memory is disabled"
    );

    // Try to get concept
    let retrieved = manager.get_concept("Test").await;
    assert!(
        retrieved.is_none(),
        "Should return None when semantic is disabled"
    );

    // Try to reinforce
    let reinforced = manager.reinforce_concept("Test").await.unwrap();
    assert!(!reinforced, "Should return false when semantic is disabled");

    // Try to connect
    let connected = manager.connect_concepts("A", "B").await.unwrap();
    assert!(!connected, "Should return false when semantic is disabled");

    // Try to get related
    let related = manager.get_related_concepts("Test").await;
    assert_eq!(
        related.len(),
        0,
        "Should return empty vec when semantic is disabled"
    );
}
