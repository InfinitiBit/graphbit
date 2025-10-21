//! Tests for memory manager

use graphbit_core::memory::decay::DecayConfig;
use graphbit_core::memory::manager::{MemoryConfig, MemoryManager};
use graphbit_core::memory::types::*;

#[tokio::test]
async fn test_memory_manager_creation() {
    let manager = MemoryManager::new(MemoryConfig::default(), None);

    let stats = manager.get_stats().await;
    assert_eq!(stats.total_memories, 0);
}

#[tokio::test]
async fn test_memory_manager_with_defaults() {
    let manager = MemoryManager::with_defaults();

    let stats = manager.get_stats().await;
    assert_eq!(stats.total_memories, 0);
}

#[tokio::test]
async fn test_memory_manager_start_session() {
    let mut manager = MemoryManager::with_defaults();

    let session_id = "test_session";
    manager.start_session(session_id.to_string());

    let stats = manager.get_stats().await;
    assert_eq!(stats.current_session, Some(session_id.to_string()));
}

#[tokio::test]
async fn test_memory_manager_end_session() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_session("session_1".to_string());
    manager
        .store_working("Test memory".to_string())
        .await
        .unwrap();

    let count = manager.end_session().await.unwrap();

    assert_eq!(count, 1);

    let stats = manager.get_stats().await;
    assert!(stats.current_session.is_none());
}

#[tokio::test]
async fn test_memory_manager_store_working() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_session("session_1".to_string());
    let memory_id = manager
        .store_working("Working memory content".to_string())
        .await
        .unwrap();

    assert!(!memory_id.as_uuid().is_nil());

    let stats = manager.get_stats().await;
    assert_eq!(stats.working_count, 1);
}

#[tokio::test]
async fn test_memory_manager_store_fact() {
    let manager = MemoryManager::with_defaults();

    let memory_id = manager
        .store_fact("user_name".to_string(), "Alice".to_string())
        .await
        .unwrap();

    assert!(!memory_id.as_uuid().is_nil());

    let stats = manager.get_stats().await;
    assert_eq!(stats.factual_count, 1);
}

#[tokio::test]
async fn test_memory_manager_start_episode() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_episode("Test Episode".to_string());
    manager.add_to_episode("Episode event".to_string());

    let memory_id = manager.end_episode().await.unwrap();

    assert!(memory_id.is_some());

    let stats = manager.get_stats().await;
    assert_eq!(stats.episodic_count, 1);
}

#[tokio::test]
async fn test_memory_manager_end_episode() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_episode("Test Episode".to_string());
    manager.add_to_episode("Event 1".to_string());
    manager.add_to_episode("Event 2".to_string());

    let episode_id = manager.end_episode().await.unwrap();

    assert!(episode_id.is_some());
}

#[tokio::test]
async fn test_memory_manager_store_concept() {
    let mut manager = MemoryManager::with_defaults();

    let concept = graphbit_core::memory::semantic::SemanticConcept::new(
        "rust".to_string(),
        "A programming language".to_string(),
    );
    let memory_id = manager.store_concept(concept).await.unwrap();

    assert!(!memory_id.as_uuid().is_nil());

    let stats = manager.get_stats().await;
    assert_eq!(stats.semantic_count, 1);
}

#[tokio::test]
async fn test_memory_manager_reinforce_concept() {
    let mut manager = MemoryManager::with_defaults();

    let concept = graphbit_core::memory::semantic::SemanticConcept::new(
        "learning".to_string(),
        "The process of acquiring knowledge".to_string(),
    );
    manager.store_concept(concept).await.unwrap();

    manager.reinforce_concept("learning").await.unwrap();

    // Concept should be reinforced
    let stats = manager.get_stats().await;
    assert_eq!(stats.semantic_count, 1);
}

#[tokio::test]
async fn test_memory_manager_retrieve() {
    let mut manager = MemoryManager::with_defaults();

    // Store various memories
    manager.start_session("session_1".to_string());
    manager
        .store_working("Working memory about weather".to_string())
        .await
        .unwrap();
    manager
        .store_fact("location".to_string(), "New York".to_string())
        .await
        .unwrap();

    let query = MemoryQuery::new("weather".to_string());
    let results = manager.retrieve(query).await.unwrap();

    assert!(!results.is_empty(), "Should find matching memories");
}

#[tokio::test]
async fn test_memory_manager_retrieve_with_filters() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_session("session_1".to_string());
    manager
        .store_working("Session memory".to_string())
        .await
        .unwrap();
    manager
        .store_fact("key".to_string(), "value".to_string())
        .await
        .unwrap();

    let query = MemoryQuery::new("".to_string()).with_memory_type(MemoryType::Working);

    let results = manager.retrieve(query).await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entry.memory_type, MemoryType::Working);
}

#[tokio::test]
async fn test_memory_manager_get_memory() {
    let manager = MemoryManager::with_defaults();

    let memory_id = manager
        .store_fact("test_key".to_string(), "test_value".to_string())
        .await
        .unwrap();

    let retrieved = manager.get_memory(&memory_id).await;

    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, memory_id);
}

#[tokio::test]
async fn test_memory_manager_get_nonexistent_memory() {
    let manager = MemoryManager::with_defaults();

    let fake_id = MemoryId::new();
    let retrieved = manager.get_memory(&fake_id).await;

    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_memory_manager_remove_memory() {
    let manager = MemoryManager::with_defaults();

    let memory_id = manager
        .store_fact("temp".to_string(), "value".to_string())
        .await
        .unwrap();

    let removed = manager.remove_memory(&memory_id).await.unwrap();
    assert!(removed);

    let retrieved = manager.get_memory(&memory_id).await;
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_memory_manager_get_current_session() {
    let mut manager = MemoryManager::with_defaults();

    assert!(manager.get_current_session().await.is_none());

    manager.start_session("session_1".to_string());

    assert_eq!(
        manager.get_current_session().await,
        Some("session_1".to_string())
    );
}

#[tokio::test]
async fn test_memory_manager_run_decay() {
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

    let stats = manager.run_decay().await.unwrap();

    // With default config and recent memories, nothing should be removed
    assert_eq!(stats.forgotten, 0);
}

#[tokio::test]
async fn test_memory_manager_clear_all() {
    let mut manager = MemoryManager::with_defaults();

    // Store various memories
    manager.start_session("session_1".to_string());
    manager.store_working("Working".to_string()).await.unwrap();
    manager
        .store_fact("key".to_string(), "value".to_string())
        .await
        .unwrap();

    manager.clear_all().await.unwrap();

    let stats = manager.get_stats().await;
    assert_eq!(stats.total_memories, 0);
}

#[tokio::test]
async fn test_memory_manager_stats() {
    let mut manager = MemoryManager::with_defaults();

    manager.start_session("session_1".to_string());
    manager
        .store_working("Working 1".to_string())
        .await
        .unwrap();
    manager
        .store_working("Working 2".to_string())
        .await
        .unwrap();
    manager
        .store_fact("key1".to_string(), "value1".to_string())
        .await
        .unwrap();

    manager.start_episode("Test Episode".to_string());
    manager.add_to_episode("Event 1".to_string());
    manager.end_episode().await.unwrap();

    let concept = graphbit_core::memory::semantic::SemanticConcept::new(
        "concept1".to_string(),
        "A test concept".to_string(),
    );
    manager.store_concept(concept).await.unwrap();

    let stats = manager.get_stats().await;

    assert_eq!(stats.working_count, 2);
    assert_eq!(stats.factual_count, 1);
    assert_eq!(stats.episodic_count, 1);
    assert_eq!(stats.semantic_count, 1);
    assert_eq!(stats.total_memories, 5);
    assert_eq!(stats.current_session, Some("session_1".to_string()));
}

#[tokio::test]
async fn test_memory_manager_config_capacities() {
    use std::collections::HashMap;

    let mut capacities = HashMap::new();
    capacities.insert(MemoryType::Working, 10);
    capacities.insert(MemoryType::Factual, 100);

    let config = MemoryConfig {
        enable_working: true,
        enable_factual: true,
        enable_episodic: true,
        enable_semantic: true,
        capacities,
        decay_config: DecayConfig::default(),
        auto_embed: false,
    };

    let _manager = MemoryManager::new(config, None);

    // Just verify creation with custom config succeeds
}

#[tokio::test]
async fn test_memory_manager_disabled_memory_types() {
    let config = MemoryConfig {
        enable_working: false,
        enable_factual: true,
        enable_episodic: false,
        enable_semantic: false,
        capacities: std::collections::HashMap::new(),
        decay_config: DecayConfig::default(),
        auto_embed: false,
    };

    let mut manager = MemoryManager::new(config, None);

    // Working memory should fail
    manager.start_session("session_1".to_string());
    let result = manager.store_working("Test".to_string()).await;
    assert!(result.is_err(), "Working memory should be disabled");

    // Factual should work
    let result = manager
        .store_fact("key".to_string(), "value".to_string())
        .await;
    assert!(result.is_ok(), "Factual memory should be enabled");
}

#[tokio::test]
async fn test_memory_manager_multiple_sessions() {
    let mut manager = MemoryManager::with_defaults();

    // Session 1
    manager.start_session("session_1".to_string());
    manager
        .store_working("Session 1 memory".to_string())
        .await
        .unwrap();
    manager.end_session().await.unwrap();

    // Session 2
    manager.start_session("session_2".to_string());
    manager
        .store_working("Session 2 memory".to_string())
        .await
        .unwrap();

    let stats = manager.get_stats().await;
    assert_eq!(stats.current_session, Some("session_2".to_string()));
}

#[tokio::test]
async fn test_memory_manager_cross_memory_retrieval() {
    let mut manager = MemoryManager::with_defaults();

    // Store memories of different types with similar content
    manager.start_session("session_1".to_string());
    manager
        .store_working("Information about Rust programming".to_string())
        .await
        .unwrap();
    manager
        .store_fact(
            "language".to_string(),
            "Rust programming language".to_string(),
        )
        .await
        .unwrap();

    manager.start_episode("Rust Discussion".to_string());
    manager.add_to_episode("Discussed Rust programming".to_string());
    manager.end_episode().await.unwrap();

    let concept = graphbit_core::memory::semantic::SemanticConcept::new(
        "Rust".to_string(),
        "A systems programming language".to_string(),
    );
    manager.store_concept(concept).await.unwrap();

    let query = MemoryQuery::new("Rust".to_string());
    let results = manager.retrieve(query).await.unwrap();

    // Should find memories across all types
    assert!(
        results.len() >= 2,
        "Should find memories from multiple types"
    );
}

#[tokio::test]
async fn test_memory_manager_concurrent_operations() {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let manager = Arc::new(RwLock::new(MemoryManager::with_defaults()));

    let mut handles = vec![];

    // Spawn multiple tasks performing operations concurrently
    for i in 0..5 {
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
        handle.await.unwrap().unwrap();
    }

    let stats = manager.read().await.get_stats().await;
    assert_eq!(stats.factual_count, 5);
}

#[tokio::test]
async fn test_memory_manager_empty_query() {
    let manager = MemoryManager::with_defaults();

    manager
        .store_fact("key".to_string(), "value".to_string())
        .await
        .unwrap();

    let query = MemoryQuery::new("".to_string());
    let results = manager.retrieve(query).await.unwrap();

    // Empty query should still return results
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_memory_manager_large_content() {
    let manager = MemoryManager::with_defaults();

    let large_content = "a".repeat(10000);
    let memory_id = manager
        .store_fact("large".to_string(), large_content.clone())
        .await
        .unwrap();

    let retrieved = manager.get_memory(&memory_id).await.unwrap();
    assert!(retrieved.content.contains(&large_content));
}
