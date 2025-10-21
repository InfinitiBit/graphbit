//! Tests for memory storage

use super::helpers::*;
use graphbit_core::memory::storage::*;
use graphbit_core::memory::types::*;

#[test]
fn test_in_memory_storage_creation() {
    let storage = InMemoryStorage::new();

    assert_eq!(storage.count(), 0, "New storage should be empty");
}

#[test]
fn test_in_memory_storage_with_capacities() {
    let mut capacities = std::collections::HashMap::new();
    capacities.insert(MemoryType::Working, 50);
    capacities.insert(MemoryType::Factual, 100);

    let storage = InMemoryStorage::with_capacities(capacities);

    assert_eq!(storage.count(), 0, "New storage should be empty");
    assert_eq!(storage.get_capacity(MemoryType::Working), 50);
    assert_eq!(storage.get_capacity(MemoryType::Factual), 100);
}

#[test]
fn test_storage_store_and_get() {
    let mut storage = InMemoryStorage::new();
    let entry = create_test_entry("Test content", MemoryType::Factual);
    let id = entry.id.clone();

    storage.store(entry.clone()).unwrap();

    let retrieved = storage.get(&id).unwrap();
    assert_eq!(retrieved.id, id);
    assert_eq!(retrieved.content, "Test content");
}

#[test]
fn test_storage_get_nonexistent() {
    let storage = InMemoryStorage::new();
    let id = MemoryId::new();

    let result = storage.get(&id);
    assert!(
        result.is_none(),
        "Getting non-existent memory should return None"
    );
}

#[test]
fn test_storage_delete() {
    let mut storage = InMemoryStorage::new();
    let entry = create_test_entry("Test content", MemoryType::Factual);
    let id = entry.id.clone();

    storage.store(entry).unwrap();
    assert_eq!(storage.count(), 1);

    let deleted = storage.delete(&id).unwrap();
    assert!(deleted, "Delete should return true for existing entry");
    assert_eq!(storage.count(), 0, "Storage should be empty after deletion");

    let deleted_again = storage.delete(&id).unwrap();
    assert!(
        !deleted_again,
        "Delete should return false for non-existent entry"
    );
}

#[test]
fn test_storage_list_by_type() {
    let mut storage = InMemoryStorage::new();

    // Store different types
    let working = create_test_entry("Working memory", MemoryType::Working);
    let factual = create_test_entry("Factual memory", MemoryType::Factual);
    let episodic = create_test_entry("Episodic memory", MemoryType::Episodic);

    storage.store(working).unwrap();
    storage.store(factual).unwrap();
    storage.store(episodic).unwrap();

    let working_memories = storage.list_by_type(MemoryType::Working);
    assert_eq!(working_memories.len(), 1);
    assert_eq!(working_memories[0].content, "Working memory");

    let factual_memories = storage.list_by_type(MemoryType::Factual);
    assert_eq!(factual_memories.len(), 1);
    assert_eq!(factual_memories[0].content, "Factual memory");
}

#[test]
fn test_storage_list_by_session() {
    let mut storage = InMemoryStorage::new();
    let session_id = "session_123";

    // Create entries with session IDs
    let entry1 = MemoryEntry::new(
        "Memory 1".to_string(),
        MemoryType::Working,
        Some(session_id.to_string()),
    );
    let entry2 = MemoryEntry::new(
        "Memory 2".to_string(),
        MemoryType::Working,
        Some(session_id.to_string()),
    );
    let entry3 = MemoryEntry::new(
        "Memory 3".to_string(),
        MemoryType::Working,
        Some("other_session".to_string()),
    );

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();
    storage.store(entry3).unwrap();

    let session_memories = storage.list_by_session(session_id);
    assert_eq!(
        session_memories.len(),
        2,
        "Should retrieve 2 memories for session"
    );
}

#[test]
fn test_storage_list_all() {
    let mut storage = InMemoryStorage::new();

    for i in 0..5 {
        let entry = create_test_entry(&format!("Memory {}", i), MemoryType::Factual);
        storage.store(entry).unwrap();
    }

    let all_memories = storage.list_all();
    assert_eq!(all_memories.len(), 5, "Should retrieve all 5 memories");
}

#[test]
fn test_storage_clear() {
    let mut storage = InMemoryStorage::new();

    for i in 0..3 {
        let entry = create_test_entry(&format!("Memory {}", i), MemoryType::Factual);
        storage.store(entry).unwrap();
    }

    assert_eq!(storage.count(), 3);

    storage.clear();

    assert_eq!(storage.count(), 0, "Storage should be empty after clear");
}

#[test]
fn test_storage_update() {
    let mut storage = InMemoryStorage::new();
    let mut entry = create_test_entry("Original content", MemoryType::Factual);
    let id = entry.id.clone();

    storage.store(entry.clone()).unwrap();

    // Update the entry
    entry.content = "Updated content".to_string();
    entry.importance_score = 0.9;

    storage.store(entry).unwrap();

    let retrieved = storage.get(&id).unwrap();
    assert_eq!(retrieved.content, "Updated content");
    assert_eq!(retrieved.importance_score, 0.9);
    assert_eq!(storage.count(), 1, "Should still have only 1 entry");
}

#[test]
fn test_storage_capacity_management() {
    let mut capacities = std::collections::HashMap::new();
    capacities.insert(MemoryType::Factual, 5);
    let mut storage = InMemoryStorage::with_capacities(capacities);

    // Store more entries than capacity - should evict least important
    for i in 0..10 {
        let entry = create_test_entry(&format!("Memory {}", i), MemoryType::Factual);
        storage.store(entry).unwrap();
    }

    let count = storage.count_by_type(MemoryType::Factual);
    assert!(
        count <= 5,
        "Storage should not exceed capacity, got {}",
        count
    );
}

#[test]
fn test_storage_eviction_by_importance() {
    let mut capacities = std::collections::HashMap::new();
    capacities.insert(MemoryType::Factual, 3);
    let mut storage = InMemoryStorage::with_capacities(capacities);

    // Store entries with different importance scores
    let entry1 =
        MemoryEntry::with_importance("Low importance".to_string(), MemoryType::Factual, 0.1, None);
    let entry2 = MemoryEntry::with_importance(
        "High importance".to_string(),
        MemoryType::Factual,
        0.9,
        None,
    );
    let entry3 = MemoryEntry::with_importance(
        "Medium importance".to_string(),
        MemoryType::Factual,
        0.5,
        None,
    );
    let id2 = entry2.id.clone();

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();
    storage.store(entry3).unwrap();

    // Store a new entry, should evict least important (entry1)
    let entry4 =
        MemoryEntry::with_importance("New entry".to_string(), MemoryType::Factual, 0.6, None);
    storage.store(entry4).unwrap();

    assert_eq!(storage.count_by_type(MemoryType::Factual), 3);
    // High importance entry should still be there
    assert!(
        storage.get(&id2).is_some(),
        "High importance entry should be retained"
    );
}

#[test]
fn test_storage_list_by_type_empty() {
    let storage = InMemoryStorage::new();

    let memories = storage.list_by_type(MemoryType::Semantic);
    assert!(
        memories.is_empty(),
        "Should return empty vec for non-existent type"
    );
}

#[test]
fn test_storage_list_by_session_empty() {
    let storage = InMemoryStorage::new();

    let memories = storage.list_by_session("nonexistent_session");
    assert!(
        memories.is_empty(),
        "Should return empty vec for non-existent session"
    );
}

#[test]
fn test_storage_multiple_types() {
    let mut storage = InMemoryStorage::new();

    // Store multiple entries of each type
    for _ in 0..2 {
        storage
            .store(create_test_entry("Working", MemoryType::Working))
            .unwrap();
        storage
            .store(create_test_entry("Factual", MemoryType::Factual))
            .unwrap();
        storage
            .store(create_test_entry("Episodic", MemoryType::Episodic))
            .unwrap();
        storage
            .store(create_test_entry("Semantic", MemoryType::Semantic))
            .unwrap();
    }

    assert_eq!(storage.count(), 8);
    assert_eq!(storage.list_by_type(MemoryType::Working).len(), 2);
    assert_eq!(storage.list_by_type(MemoryType::Factual).len(), 2);
    assert_eq!(storage.list_by_type(MemoryType::Episodic).len(), 2);
    assert_eq!(storage.list_by_type(MemoryType::Semantic).len(), 2);
}

#[test]
fn test_storage_clear_type() {
    let mut storage = InMemoryStorage::new();

    // Store different types
    storage
        .store(create_test_entry("Working 1", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("Working 2", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("Factual 1", MemoryType::Factual))
        .unwrap();
    storage
        .store(create_test_entry("Episodic 1", MemoryType::Episodic))
        .unwrap();

    assert_eq!(storage.count(), 4);

    // Clear only working memories
    storage.clear_type(MemoryType::Working);

    assert_eq!(storage.count(), 2, "Should have 2 memories left");
    assert_eq!(storage.count_by_type(MemoryType::Working), 0);
    assert_eq!(storage.count_by_type(MemoryType::Factual), 1);
    assert_eq!(storage.count_by_type(MemoryType::Episodic), 1);
}

#[test]
fn test_storage_clear_session() {
    let mut storage = InMemoryStorage::new();
    let session1 = "session_1";
    let session2 = "session_2";

    // Store memories in different sessions
    storage
        .store(MemoryEntry::new(
            "Session 1 Memory 1".to_string(),
            MemoryType::Working,
            Some(session1.to_string()),
        ))
        .unwrap();
    storage
        .store(MemoryEntry::new(
            "Session 1 Memory 2".to_string(),
            MemoryType::Working,
            Some(session1.to_string()),
        ))
        .unwrap();
    storage
        .store(MemoryEntry::new(
            "Session 2 Memory 1".to_string(),
            MemoryType::Working,
            Some(session2.to_string()),
        ))
        .unwrap();

    assert_eq!(storage.count(), 3);

    // Clear session 1
    storage.clear_session(session1);

    assert_eq!(storage.count(), 1, "Should have 1 memory left");
    assert_eq!(storage.list_by_session(session1).len(), 0);
    assert_eq!(storage.list_by_session(session2).len(), 1);
}

#[test]
fn test_storage_get_mut() {
    let mut storage = InMemoryStorage::new();
    let entry = create_test_entry("Original", MemoryType::Factual);
    let id = entry.id.clone();

    storage.store(entry).unwrap();

    // Get mutable reference and modify
    if let Some(entry_mut) = storage.get_mut(&id) {
        entry_mut.content = "Modified".to_string();
        entry_mut.importance_score = 0.8;
    }

    // Verify changes
    let retrieved = storage.get(&id).unwrap();
    assert_eq!(retrieved.content, "Modified");
    assert_eq!(retrieved.importance_score, 0.8);
}

#[test]
fn test_storage_count_by_type() {
    let mut storage = InMemoryStorage::new();

    storage
        .store(create_test_entry("W1", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("W2", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("W3", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("F1", MemoryType::Factual))
        .unwrap();
    storage
        .store(create_test_entry("F2", MemoryType::Factual))
        .unwrap();

    assert_eq!(storage.count_by_type(MemoryType::Working), 3);
    assert_eq!(storage.count_by_type(MemoryType::Factual), 2);
    assert_eq!(storage.count_by_type(MemoryType::Episodic), 0);
    assert_eq!(storage.count_by_type(MemoryType::Semantic), 0);
}
