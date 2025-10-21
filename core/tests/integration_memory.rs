//! Integration tests for the GraphBit memory system
//!
//! These tests verify the end-to-end functionality of the memory system,
//! including all four memory types, retrieval, decay, and cross-memory operations.

use chrono::Utc;
use graphbit_core::memory::*;

#[tokio::test]
async fn test_memory_manager_basic_workflow() {
    let mut manager = MemoryManager::with_defaults();

    // Start a session (synchronous)
    manager.start_session("test_session_1".to_string());

    // Store working memory
    let working_id = manager
        .store_working("User asked about the weather".to_string())
        .await
        .expect("Failed to store working memory");

    assert!(
        !working_id.as_uuid().is_nil(),
        "Working memory ID should be valid"
    );

    // Store a fact
    let fact_id = manager
        .store_fact("user_location".to_string(), "San Francisco".to_string())
        .await
        .expect("Failed to store fact");

    assert!(!fact_id.as_uuid().is_nil(), "Fact ID should be valid");

    // Get stats
    let stats = manager.get_stats().await;
    assert_eq!(stats.working_count, 1, "Should have 1 working memory");
    assert_eq!(stats.factual_count, 1, "Should have 1 factual memory");
    assert_eq!(stats.total_memories, 2, "Should have 2 total memories");
    assert_eq!(stats.current_session, Some("test_session_1".to_string()));
}

#[tokio::test]
async fn test_memory_retrieval() {
    let mut manager = MemoryManager::with_defaults();

    // Store various memories
    manager.start_session("session_1".to_string());
    manager
        .store_working("Discussion about Rust programming".to_string())
        .await
        .unwrap();
    manager
        .store_fact("favorite_language".to_string(), "Rust".to_string())
        .await
        .unwrap();

    // Retrieve memories
    let query = MemoryQuery::new("Rust".to_string());
    let results = manager
        .retrieve(query)
        .await
        .expect("Failed to retrieve memories");

    assert!(!results.is_empty(), "Should find memories about Rust");
    assert!(results.len() >= 1, "Should find at least one memory");
}

#[tokio::test]
async fn test_episodic_memory_workflow() {
    let mut manager = MemoryManager::with_defaults();

    // Start an episode (synchronous, requires title)
    manager.start_episode("Weather Conversation".to_string());

    // Add events to the episode
    manager.add_to_episode("User greeted the agent".to_string());
    manager.add_to_episode("User asked about the weather".to_string());
    manager.add_to_episode("Agent provided weather information".to_string());

    // End the episode
    let episode_id = manager.end_episode().await.expect("Failed to end episode");

    assert!(episode_id.is_some(), "Episode ID should be returned");

    let stats = manager.get_stats().await;
    assert_eq!(
        stats.episodic_count, 1,
        "Should have 1 episodic memory (the episode)"
    );
}

#[tokio::test]
async fn test_semantic_memory_workflow() {
    let mut manager = MemoryManager::with_defaults();

    // Store a concept (requires name and description)
    let concept = SemanticConcept::new(
        "machine_learning".to_string(),
        "A field of AI focused on learning from data".to_string(),
    );
    let concept_id = manager
        .store_concept(concept)
        .await
        .expect("Failed to store concept");

    assert!(!concept_id.as_uuid().is_nil(), "Concept ID should be valid");

    // Reinforce the concept (only takes name parameter)
    manager
        .reinforce_concept("machine_learning")
        .await
        .expect("Failed to reinforce concept");

    let stats = manager.get_stats().await;
    assert_eq!(stats.semantic_count, 1, "Should have 1 semantic memory");
}

#[tokio::test]
async fn test_session_isolation() {
    let mut manager = MemoryManager::with_defaults();

    // Session 1
    manager.start_session("session_1".to_string());
    manager
        .store_working("Session 1 memory".to_string())
        .await
        .unwrap();
    let session1_count = manager.end_session().await.unwrap();
    assert_eq!(session1_count, 1);

    // Session 2
    manager.start_session("session_2".to_string());
    manager
        .store_working("Session 2 memory".to_string())
        .await
        .unwrap();

    // Query for session 2 only using empty query string
    // This tests that session filtering works without requiring a search term
    let query = MemoryQuery::new("".to_string()).with_session_id("session_2".to_string());
    let results = manager.retrieve(query).await.unwrap();

    assert_eq!(results.len(), 1, "Should only find session 2 memory");
    assert!(results[0].entry.content.contains("Session 2"));
    assert_eq!(
        results[0].similarity, 1.0,
        "Empty query should return similarity of 1.0"
    );
}

#[tokio::test]
async fn test_memory_type_filtering() {
    let mut manager = MemoryManager::with_defaults();

    // Store different types
    manager.start_session("session_1".to_string());
    manager
        .store_working("Working memory".to_string())
        .await
        .unwrap();
    manager
        .store_fact("key".to_string(), "Factual memory".to_string())
        .await
        .unwrap();

    manager.start_episode("Test Episode".to_string());
    manager.add_to_episode("Episodic memory".to_string());
    manager.end_episode().await.unwrap();

    let concept = SemanticConcept::new("concept".to_string(), "A test concept".to_string());
    manager.store_concept(concept).await.unwrap();

    // Query for working memory only
    let query = MemoryQuery::new("memory".to_string()).with_memory_type(MemoryType::Working);
    let results = manager.retrieve(query).await.unwrap();

    assert!(
        results
            .iter()
            .all(|r| r.entry.memory_type == MemoryType::Working),
        "All results should be working memory"
    );
}

#[tokio::test]
async fn test_memory_removal() {
    let manager = MemoryManager::with_defaults();

    // Store a memory
    let memory_id = manager
        .store_fact("temp_key".to_string(), "temp_value".to_string())
        .await
        .unwrap();

    // Verify it exists
    let retrieved = manager.get_memory(&memory_id).await;
    assert!(retrieved.is_some(), "Memory should exist");

    // Remove it
    let removed = manager.remove_memory(&memory_id).await.unwrap();
    assert!(removed, "Memory should be removed");

    // Verify it's gone
    let retrieved_after = manager.get_memory(&memory_id).await;
    assert!(
        retrieved_after.is_none(),
        "Memory should not exist after removal"
    );
}

#[tokio::test]
async fn test_memory_decay() {
    let mut manager = MemoryManager::with_defaults();

    // Store some memories
    manager
        .store_fact("key1".to_string(), "value1".to_string())
        .await
        .unwrap();
    manager
        .store_fact("key2".to_string(), "value2".to_string())
        .await
        .unwrap();

    // Run decay (should not remove recent memories with default config)
    let decay_stats = manager.run_decay().await.unwrap();

    // With default config and recent memories, they should be protected (not forgotten)
    assert_eq!(
        decay_stats.forgotten, 0,
        "Recent memories should not be removed"
    );
    assert_eq!(
        decay_stats.total_checked, 2,
        "Should have checked 2 memories"
    );
    // Recent memories are protected, not retained (protected is a separate counter)
    assert_eq!(
        decay_stats.protected, 2,
        "Both memories should be protected"
    );

    let stats = manager.get_stats().await;
    assert_eq!(stats.factual_count, 2, "Both memories should still exist");
}

#[tokio::test]
async fn test_clear_all_memories() {
    let mut manager = MemoryManager::with_defaults();

    // Store various memories
    manager.start_session("session_1".to_string());
    manager.store_working("Working".to_string()).await.unwrap();
    manager
        .store_fact("key".to_string(), "value".to_string())
        .await
        .unwrap();

    manager.start_episode("Test Episode".to_string());
    manager.add_to_episode("Event".to_string());
    manager.end_episode().await.unwrap();

    let concept = SemanticConcept::new("concept".to_string(), "A test concept".to_string());
    manager.store_concept(concept).await.unwrap();

    // Verify memories exist
    let stats_before = manager.get_stats().await;
    assert!(
        stats_before.total_memories > 0,
        "Should have memories before clear"
    );

    // Clear all
    manager.clear_all().await.unwrap();

    // Verify all cleared
    let stats_after = manager.get_stats().await;
    assert_eq!(
        stats_after.total_memories, 0,
        "All memories should be cleared"
    );
}

#[tokio::test]
async fn test_concurrent_memory_operations() {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let manager = Arc::new(RwLock::new(MemoryManager::with_defaults()));

    let mut handles = vec![];

    // Spawn multiple tasks storing facts concurrently
    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            manager_clone
                .read()
                .await
                .store_fact(format!("key{}", i), format!("value{}", i))
                .await
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap().expect("Failed to store fact");
    }

    let stats = manager.read().await.get_stats().await;
    assert_eq!(
        stats.factual_count, 10,
        "All concurrent stores should succeed"
    );
}

#[tokio::test]
async fn test_memory_entry_decay_calculation() {
    let entry = MemoryEntry::new("Test".to_string(), MemoryType::Working, None);

    let now = Utc::now();
    let decay_score = entry.calculate_decay(now);

    // Decay score should be normalized between 0 and 1
    assert!(
        decay_score >= 0.0 && decay_score <= 1.0,
        "Decay score should be normalized"
    );
    // Fresh memory with default importance (0.5) should score around 0.4
    // Formula: (1.0 * 0.3 + 1.0 * 0.5 + 0.0 * 0.2) * 0.5 = 0.8 * 0.5 = 0.4
    assert!(
        (decay_score - 0.4).abs() < 0.01,
        "Fresh memory should score ~0.4, got {}",
        decay_score
    );
}

#[tokio::test]
async fn test_memory_entry_access_tracking() {
    let mut entry = MemoryEntry::new("Test".to_string(), MemoryType::Factual, None);

    assert_eq!(entry.access_count, 0);

    entry.record_access();
    assert_eq!(entry.access_count, 1);

    entry.record_access();
    assert_eq!(entry.access_count, 2);
}

#[tokio::test]
async fn test_memory_entry_relations() {
    let mut entry1 = MemoryEntry::new("Entry 1".to_string(), MemoryType::Semantic, None);
    let entry2 = MemoryEntry::new("Entry 2".to_string(), MemoryType::Semantic, None);

    let id2 = entry2.id.clone();

    entry1.add_relation(id2.clone());

    assert_eq!(entry1.related_memories.len(), 1);
    assert_eq!(entry1.related_memories[0], id2);

    entry1.remove_relation(&id2);
    assert_eq!(entry1.related_memories.len(), 0);
}

#[tokio::test]
async fn test_memory_stats_accuracy() {
    let mut manager = MemoryManager::with_defaults();

    // Store specific counts of each type
    manager.start_session("session_1".to_string());
    manager.store_working("W1".to_string()).await.unwrap();
    manager.store_working("W2".to_string()).await.unwrap();

    manager
        .store_fact("F1".to_string(), "V1".to_string())
        .await
        .unwrap();
    manager
        .store_fact("F2".to_string(), "V2".to_string())
        .await
        .unwrap();
    manager
        .store_fact("F3".to_string(), "V3".to_string())
        .await
        .unwrap();

    manager.start_episode("Episode 1".to_string());
    manager.add_to_episode("E1".to_string());
    manager.end_episode().await.unwrap();

    let concept = SemanticConcept::new("S1".to_string(), "Semantic concept 1".to_string());
    manager.store_concept(concept).await.unwrap();

    let stats = manager.get_stats().await;

    assert_eq!(stats.working_count, 2);
    assert_eq!(stats.factual_count, 3);
    assert_eq!(stats.episodic_count, 1);
    assert_eq!(stats.semantic_count, 1);
    assert_eq!(stats.total_memories, 7);
}
