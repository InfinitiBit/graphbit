//! Tests for factual memory

use graphbit_core::memory::factual::FactualMemory;
use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;

#[test]
fn test_factual_memory_creation() {
    let _factual = FactualMemory::new();
    // Just verify creation succeeds
}

#[test]
fn test_factual_memory_store() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "user_name";
    let value = "Alice";

    let memory_id = factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    // Verify the memory was stored
    let stored = storage.get(&memory_id);
    assert!(stored.is_some());

    let entry = stored.unwrap();
    assert!(entry.content.contains(key));
    assert!(entry.content.contains(value));
    assert_eq!(entry.memory_type, MemoryType::Factual);
}

#[test]
fn test_factual_memory_get() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "favorite_color";
    let value = "blue";

    factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    let retrieved = factual.get_fact(key, &storage);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
}

#[test]
fn test_factual_memory_get_nonexistent() {
    let storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let retrieved = factual.get_fact("nonexistent_key", &storage);
    assert!(
        retrieved.is_none(),
        "Getting non-existent fact should return None"
    );
}

#[test]
fn test_factual_memory_update() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "user_age";

    // Store initial value
    factual
        .store_fact(key.to_string(), "25".to_string(), &mut storage)
        .unwrap();

    // Update value
    let updated = factual
        .update_fact(key, "26".to_string(), &mut storage)
        .unwrap();
    assert!(updated, "Update should return true");

    let retrieved = factual.get_fact(key, &storage).unwrap();
    assert_eq!(retrieved, "26");
}

#[test]
fn test_factual_memory_list_facts() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store multiple facts
    factual
        .store_fact("name".to_string(), "Bob".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("age".to_string(), "30".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("city".to_string(), "NYC".to_string(), &mut storage)
        .unwrap();

    let all_facts = factual.list_facts(&storage);
    assert_eq!(all_facts.len(), 3, "Should retrieve all facts");
}

#[test]
fn test_factual_memory_get_all_facts() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store multiple facts
    factual
        .store_fact("name".to_string(), "Bob".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("age".to_string(), "30".to_string(), &mut storage)
        .unwrap();

    let all_facts = factual.get_all_facts(&storage);
    assert_eq!(all_facts.len(), 2);
    assert_eq!(all_facts.get("name"), Some(&"Bob".to_string()));
    assert_eq!(all_facts.get("age"), Some(&"30".to_string()));
}

#[test]
fn test_factual_memory_delete() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "temp_fact";
    factual
        .store_fact(key.to_string(), "value".to_string(), &mut storage)
        .unwrap();

    let deleted = factual.delete_fact(key, &mut storage).unwrap();
    assert!(deleted, "Delete should return true for existing fact");

    let retrieved = factual.get_fact(key, &storage);
    assert!(retrieved.is_none(), "Fact should be deleted");
}

#[test]
fn test_factual_memory_delete_nonexistent() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let deleted = factual.delete_fact("nonexistent", &mut storage).unwrap();
    assert!(!deleted, "Delete should return false for non-existent fact");
}

#[test]
fn test_factual_memory_has_fact() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store a fact
    factual
        .store_fact(
            "test_key".to_string(),
            "test_value".to_string(),
            &mut storage,
        )
        .unwrap();

    assert!(factual.has_fact("test_key", &storage));
    assert!(!factual.has_fact("nonexistent", &storage));
}

#[test]
fn test_factual_memory_count_facts() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    assert_eq!(factual.count_facts(&storage), 0);

    factual
        .store_fact("fact1".to_string(), "value1".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("fact2".to_string(), "value2".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("fact3".to_string(), "value3".to_string(), &mut storage)
        .unwrap();

    assert_eq!(factual.count_facts(&storage), 3);
}

#[test]
fn test_factual_memory_key_value_format() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "setting";
    let value = "enabled";

    let memory_id = factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();

    // Content should be in key: value format
    assert!(stored.content.contains(key));
    assert!(stored.content.contains(value));
    assert!(stored.content.contains(":"));
}

#[test]
fn test_factual_memory_metadata() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let memory_id = factual
        .store_fact("key".to_string(), "value".to_string(), &mut storage)
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();

    assert_eq!(stored.metadata.source, "factual");
    assert!(stored.metadata.tags.contains(&"fact".to_string()));
}

#[test]
fn test_factual_memory_importance() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let memory_id = factual
        .store_fact(
            "important_key".to_string(),
            "important_value".to_string(),
            &mut storage,
        )
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();

    // Factual memories should have higher importance (default 0.8)
    assert!(stored.importance_score >= 0.7);
}

#[test]
fn test_factual_memory_custom_importance() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let memory_id = factual
        .store_fact_with_importance(
            "custom_key".to_string(),
            "custom_value".to_string(),
            0.95,
            &mut storage,
        )
        .unwrap();

    let stored = storage.get(&memory_id).unwrap();
    assert_eq!(stored.importance_score, 0.95);
}

#[test]
fn test_factual_memory_special_characters() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "special_key";
    let value = "value with spaces and symbols!@#$%";

    factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    let retrieved = factual.get_fact(key, &storage);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
}

#[test]
fn test_factual_memory_empty_value() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "empty_key";
    let value = "";

    factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    let retrieved = factual.get_fact(key, &storage);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), "");
}

#[test]
fn test_factual_memory_long_value() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "long_value";
    let value = "a".repeat(1000);

    factual
        .store_fact(key.to_string(), value.clone(), &mut storage)
        .unwrap();

    let retrieved = factual.get_fact(key, &storage).unwrap();
    assert_eq!(retrieved, value);
}

#[test]
fn test_factual_memory_multiple_keys() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let keys = vec!["key1", "key2", "key3", "key4", "key5"];

    for (i, key) in keys.iter().enumerate() {
        factual
            .store_fact(key.to_string(), format!("value{}", i), &mut storage)
            .unwrap();
    }

    // Verify all keys can be retrieved
    for key in keys {
        let fact = factual.get_fact(key, &storage);
        assert!(fact.is_some(), "Key {} should exist", key);
    }
}

#[test]
fn test_factual_memory_update_behavior() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "update_test";

    // Store initial value
    let id1 = factual
        .store_fact(key.to_string(), "value1".to_string(), &mut storage)
        .unwrap();

    // Verify initial value
    assert_eq!(factual.get_fact(key, &storage), Some("value1".to_string()));

    // Update value using update_fact
    let updated = factual
        .update_fact(key, "value2".to_string(), &mut storage)
        .unwrap();
    assert!(updated);

    // Verify updated value
    assert_eq!(factual.get_fact(key, &storage), Some("value2".to_string()));

    // The ID should be the same (in-place update)
    let entry = storage.get(&id1).unwrap();
    assert!(entry.content.contains("value2"));
}

#[test]
fn test_factual_memory_json_value() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    let key = "json_data";
    let value = r#"{"name": "test", "count": 42}"#;

    factual
        .store_fact(key.to_string(), value.to_string(), &mut storage)
        .unwrap();

    let retrieved = factual.get_fact(key, &storage).unwrap();
    assert_eq!(retrieved, value);
}

#[test]
fn test_factual_memory_namespace() {
    let mut storage = InMemoryStorage::new();
    let mut factual = FactualMemory::new();

    // Store facts in different namespaces
    factual.set_namespace("app1".to_string());
    factual
        .store_fact("setting".to_string(), "value1".to_string(), &mut storage)
        .unwrap();

    factual.set_namespace("app2".to_string());
    factual
        .store_fact("setting".to_string(), "value2".to_string(), &mut storage)
        .unwrap();

    // Retrieve from specific namespace
    factual.set_namespace("app1".to_string());
    assert_eq!(
        factual.get_fact("setting", &storage),
        Some("value1".to_string())
    );

    factual.set_namespace("app2".to_string());
    assert_eq!(
        factual.get_fact("setting", &storage),
        Some("value2".to_string())
    );
}

#[test]
fn test_factual_memory_preferences() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    factual
        .store_preference("theme".to_string(), "dark".to_string(), &mut storage)
        .unwrap();

    assert_eq!(
        factual.get_preference("theme", &storage),
        Some("dark".to_string())
    );
}

#[test]
fn test_factual_memory_namespace_isolation() {
    let mut storage = InMemoryStorage::new();
    let mut factual = FactualMemory::new();

    // Store in namespace
    factual.set_namespace("ns1".to_string());
    factual
        .store_fact("key1".to_string(), "value1".to_string(), &mut storage)
        .unwrap();

    // Store without namespace
    factual.clear_namespace();
    factual
        .store_fact("key2".to_string(), "value2".to_string(), &mut storage)
        .unwrap();

    // Count should only include namespace facts
    factual.set_namespace("ns1".to_string());
    assert_eq!(factual.count_facts(&storage), 1);

    // Without namespace, should see all facts (both namespaced and non-namespaced)
    factual.clear_namespace();
    assert_eq!(factual.count_facts(&storage), 2);
}

#[test]
fn test_factual_memory_with_namespace_constructor() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::with_namespace("test_ns".to_string());

    factual
        .store_fact("key".to_string(), "value".to_string(), &mut storage)
        .unwrap();

    // Should be able to retrieve with same namespace
    assert_eq!(factual.get_fact("key", &storage), Some("value".to_string()));

    // Different namespace should not see it
    let other_factual = FactualMemory::with_namespace("other_ns".to_string());
    assert_eq!(other_factual.get_fact("key", &storage), None);
}

// ============================================================================
// CRUD Operations Tests (Phase 2)
// ============================================================================

#[test]
fn test_get_fact_existing() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store a fact
    factual
        .store_fact("username".to_string(), "alice".to_string(), &mut storage)
        .unwrap();

    // Retrieve it
    let value = factual.get_fact("username", &storage);
    assert_eq!(value, Some("alice".to_string()));
}

#[test]
fn test_get_fact_nonexistent_returns_none() {
    let storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Try to get non-existent fact
    let value = factual.get_fact("nonexistent", &storage);
    assert_eq!(value, None);
}

#[test]
fn test_update_fact_existing_returns_true() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store a fact
    factual
        .store_fact("status".to_string(), "active".to_string(), &mut storage)
        .unwrap();

    // Update it
    let result = factual
        .update_fact("status", "inactive".to_string(), &mut storage)
        .unwrap();
    assert!(result, "update_fact should return true for existing fact");

    // Verify the update
    let value = factual.get_fact("status", &storage);
    assert_eq!(value, Some("inactive".to_string()));
}

#[test]
fn test_update_fact_nonexistent_returns_false() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Try to update non-existent fact
    let result = factual
        .update_fact("nonexistent", "value".to_string(), &mut storage)
        .unwrap();
    assert!(
        !result,
        "update_fact should return false for non-existent fact"
    );
}

#[test]
fn test_delete_fact_existing_returns_true() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store a fact
    factual
        .store_fact("temp".to_string(), "data".to_string(), &mut storage)
        .unwrap();

    // Delete it
    let result = factual.delete_fact("temp", &mut storage).unwrap();
    assert!(result, "delete_fact should return true for existing fact");

    // Verify it's gone
    let value = factual.get_fact("temp", &storage);
    assert_eq!(value, None);
}

#[test]
fn test_delete_fact_nonexistent_returns_false() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Try to delete non-existent fact
    let result = factual.delete_fact("nonexistent", &mut storage).unwrap();
    assert!(
        !result,
        "delete_fact should return false for non-existent fact"
    );
}

#[test]
fn test_list_facts_empty() {
    let storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // List facts on empty storage
    let facts = factual.list_facts(&storage);
    assert_eq!(
        facts.len(),
        0,
        "list_facts should return empty vec for empty storage"
    );
}

#[test]
fn test_list_facts_multiple() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store multiple facts
    factual
        .store_fact("key1".to_string(), "value1".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("key2".to_string(), "value2".to_string(), &mut storage)
        .unwrap();
    factual
        .store_fact("key3".to_string(), "value3".to_string(), &mut storage)
        .unwrap();

    // List all facts
    let facts = factual.list_facts(&storage);
    assert_eq!(facts.len(), 3, "list_facts should return all 3 facts");

    // Verify all facts are present (order may vary)
    let facts_map: std::collections::HashMap<_, _> = facts.into_iter().collect();
    assert_eq!(facts_map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(facts_map.get("key2"), Some(&"value2".to_string()));
    assert_eq!(facts_map.get("key3"), Some(&"value3".to_string()));
}

#[test]
fn test_has_fact_existing_returns_true() {
    let mut storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Store a fact
    factual
        .store_fact("exists".to_string(), "yes".to_string(), &mut storage)
        .unwrap();

    // Check if it exists
    let exists = factual.has_fact("exists", &storage);
    assert!(exists, "has_fact should return true for existing fact");
}

#[test]
fn test_has_fact_nonexistent_returns_false() {
    let storage = InMemoryStorage::new();
    let factual = FactualMemory::new();

    // Check if non-existent fact exists
    let exists = factual.has_fact("nonexistent", &storage);
    assert!(
        !exists,
        "has_fact should return false for non-existent fact"
    );
}
