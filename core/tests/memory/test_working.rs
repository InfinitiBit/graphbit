//! Tests for working memory

use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;
use graphbit_core::memory::working::WorkingMemory;

#[test]
fn test_working_memory_creation() {
    let working = WorkingMemory::new();

    assert!(
        working.get_session_id().is_none(),
        "Should have no active session initially"
    );
}

#[test]
fn test_working_memory_start_session() {
    let mut working = WorkingMemory::new();

    let session_id = "test_session_1";
    working.start_session(session_id.to_string());

    assert_eq!(working.get_session_id(), Some(&session_id.to_string()));
}

#[test]
fn test_working_memory_store() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    let session_id = "test_session";
    working.start_session(session_id.to_string());

    let content = "Remember this for the session";
    let memory_id = working.store(content.to_string(), &mut storage).unwrap();

    // Verify the memory was stored
    let stored = storage.get(&memory_id);
    assert!(stored.is_some());

    let entry = stored.unwrap();
    assert_eq!(entry.content, content);
    assert_eq!(entry.memory_type, MemoryType::Working);
    assert_eq!(entry.session_id, Some(session_id.to_string()));
}

#[test]
fn test_working_memory_store_without_session() {
    let mut storage = InMemoryStorage::new();
    let working = WorkingMemory::new();

    // Storing without session should succeed but have no session_id
    let memory_id = working
        .store("Test content".to_string(), &mut storage)
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();
    assert_eq!(stored.session_id, None);
}

#[test]
fn test_working_memory_get_session_context() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    let session_id = "test_session";
    working.start_session(session_id.to_string());

    // Store multiple memories
    working.store("Memory 1".to_string(), &mut storage).unwrap();
    working.store("Memory 2".to_string(), &mut storage).unwrap();
    working.store("Memory 3".to_string(), &mut storage).unwrap();

    let context = working.get_session_context(&storage);

    assert!(context.contains("Memory 1"));
    assert!(context.contains("Memory 2"));
    assert!(context.contains("Memory 3"));
}

#[test]
fn test_working_memory_get_session_context_without_session() {
    let storage = InMemoryStorage::new();
    let working = WorkingMemory::new();

    let context = working.get_session_context(&storage);

    assert_eq!(context, "No working memory available.");
}

#[test]
fn test_working_memory_end_session() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    let session_id = "test_session";
    working.start_session(session_id.to_string());

    // Store some memories
    working.store("Memory 1".to_string(), &mut storage).unwrap();
    working.store("Memory 2".to_string(), &mut storage).unwrap();

    let count = working.end_session(&mut storage).unwrap();

    assert_eq!(count, 2, "Should return count of cleared memories");
    assert!(
        working.get_session_id().is_none(),
        "Session should be cleared"
    );
}

#[test]
fn test_working_memory_end_session_without_active() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    let count = working.end_session(&mut storage).unwrap();

    assert_eq!(count, 0, "Should return 0 when no active session");
}

#[test]
fn test_working_memory_session_isolation() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    // First session
    working.start_session("session_1".to_string());
    working
        .store("Session 1 Memory".to_string(), &mut storage)
        .unwrap();
    working.end_session(&mut storage).unwrap();

    // Second session
    working.start_session("session_2".to_string());
    working
        .store("Session 2 Memory".to_string(), &mut storage)
        .unwrap();

    let memories = working.get_session_memories(&storage);

    assert_eq!(
        memories.len(),
        1,
        "Should only see current session memories"
    );
    assert_eq!(memories[0].content, "Session 2 Memory");
}

#[test]
fn test_working_memory_multiple_sessions_sequential() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    // Session 1
    working.start_session("session_1".to_string());
    working
        .store("S1 Memory 1".to_string(), &mut storage)
        .unwrap();
    working
        .store("S1 Memory 2".to_string(), &mut storage)
        .unwrap();
    let count1 = working.end_session(&mut storage).unwrap();
    assert_eq!(count1, 2);

    // Session 2
    working.start_session("session_2".to_string());
    working
        .store("S2 Memory 1".to_string(), &mut storage)
        .unwrap();
    let count2 = working.end_session(&mut storage).unwrap();
    assert_eq!(count2, 1);

    // Session 3
    working.start_session("session_3".to_string());
    working
        .store("S3 Memory 1".to_string(), &mut storage)
        .unwrap();
    working
        .store("S3 Memory 2".to_string(), &mut storage)
        .unwrap();
    working
        .store("S3 Memory 3".to_string(), &mut storage)
        .unwrap();
    let count3 = working.end_session(&mut storage).unwrap();
    assert_eq!(count3, 3);
}

#[test]
fn test_working_memory_restart_session() {
    let mut working = WorkingMemory::new();

    // Start first session
    working.start_session("session_1".to_string());

    // Start new session without ending first (should replace)
    working.start_session("session_2".to_string());

    assert_eq!(working.get_session_id(), Some(&"session_2".to_string()));
}

#[test]
fn test_working_memory_importance_score() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    working.start_session("session_1".to_string());
    let memory_id = working
        .store("Important information".to_string(), &mut storage)
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();

    // Working memory should have default importance
    assert_eq!(stored.importance_score, 0.5);
}

#[test]
fn test_working_memory_count_session_memories() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    assert_eq!(working.count_session_memories(&storage), 0);

    working.start_session("session_1".to_string());
    working.store("Memory 1".to_string(), &mut storage).unwrap();
    working.store("Memory 2".to_string(), &mut storage).unwrap();
    working.store("Memory 3".to_string(), &mut storage).unwrap();

    assert_eq!(working.count_session_memories(&storage), 3);
}

#[test]
fn test_working_memory_is_session_active() {
    let mut working = WorkingMemory::new();

    assert!(!working.is_session_active());

    working.start_session("session_1".to_string());
    assert!(working.is_session_active());

    let mut storage = InMemoryStorage::new();
    working.end_session(&mut storage).unwrap();
    assert!(!working.is_session_active());
}

#[test]
fn test_working_memory_session_metadata() {
    let mut working = WorkingMemory::new();

    working.start_session("session_1".to_string());

    // Set metadata
    working.set_session_metadata("user_id".to_string(), serde_json::json!("user123"));
    working.set_session_metadata("timestamp".to_string(), serde_json::json!(1234567890));

    // Get metadata
    assert_eq!(
        working.get_session_metadata("user_id"),
        Some(&serde_json::json!("user123"))
    );
    assert_eq!(
        working.get_session_metadata("timestamp"),
        Some(&serde_json::json!(1234567890))
    );

    // Clear metadata
    working.clear_session_metadata();
    assert_eq!(working.get_session_metadata("user_id"), None);
}

#[test]
fn test_working_memory_with_metadata() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    working.start_session("session_1".to_string());

    let mut metadata = MemoryMetadata::new();
    metadata.set_source("custom_source".to_string());
    metadata.add_tag("important".to_string());

    let memory_id = working
        .store_with_metadata("Test content".to_string(), metadata, &mut storage)
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();
    assert_eq!(stored.metadata.source, "custom_source");
    assert!(stored.metadata.tags.contains(&"important".to_string()));
}

#[test]
fn test_working_memory_get_session_memories() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    working.start_session("session_1".to_string());
    working.store("Memory 1".to_string(), &mut storage).unwrap();
    working.store("Memory 2".to_string(), &mut storage).unwrap();

    let memories = working.get_session_memories(&storage);
    assert_eq!(memories.len(), 2);
    assert!(memories.iter().any(|m| m.content == "Memory 1"));
    assert!(memories.iter().any(|m| m.content == "Memory 2"));
}

#[test]
fn test_working_memory_session_cleared_on_end() {
    let mut storage = InMemoryStorage::new();
    let mut working = WorkingMemory::new();

    working.start_session("session_1".to_string());
    working.store("Memory 1".to_string(), &mut storage).unwrap();
    working.store("Memory 2".to_string(), &mut storage).unwrap();

    assert_eq!(storage.count_by_type(MemoryType::Working), 2);

    working.end_session(&mut storage).unwrap();

    // Memories should be cleared from storage
    assert_eq!(storage.count_by_type(MemoryType::Working), 0);
}
