//! Tests for memory types

use super::helpers::*;
use graphbit_core::memory::types::*;

#[test]
fn test_memory_id_creation() {
    let id1 = MemoryId::new();
    let id2 = MemoryId::new();

    assert_ne!(id1, id2, "Each new MemoryId should be unique");
    assert!(!id1.as_uuid().is_nil(), "MemoryId should not be nil");
}

#[test]
fn test_memory_id_from_string() {
    let id_str = "550e8400-e29b-41d4-a716-446655440000";
    let id = MemoryId::from_string(id_str).unwrap();

    assert!(
        !id.as_uuid().is_nil(),
        "MemoryId from string should not be nil"
    );

    // Test deterministic generation
    let id2 = MemoryId::from_string(id_str).unwrap();
    assert_eq!(id, id2, "Same string should generate same MemoryId");
}

#[test]
fn test_memory_id_serialization() {
    let id = MemoryId::new();
    let serialized = serde_json::to_string(&id).unwrap();
    let deserialized: MemoryId = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        id, deserialized,
        "MemoryId should serialize and deserialize correctly"
    );
}

#[test]
fn test_memory_type_variants() {
    let types = vec![
        MemoryType::Working,
        MemoryType::Factual,
        MemoryType::Episodic,
        MemoryType::Semantic,
    ];

    for memory_type in types {
        let serialized = serde_json::to_string(&memory_type).unwrap();
        let deserialized: MemoryType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(memory_type, deserialized);
    }
}

#[test]
fn test_memory_metadata_creation() {
    let metadata = MemoryMetadata::new();

    assert!(
        metadata.tags.is_empty(),
        "New metadata should have empty tags"
    );
    assert_eq!(
        metadata.source, "unknown",
        "Default source should be 'unknown'"
    );
    assert!(metadata.custom.is_empty(), "Custom fields should be empty");
    assert!(metadata.image_data.is_none(), "Image data should be None");
}

#[test]
fn test_memory_metadata_with_tags() {
    let tags = vec!["important".to_string(), "user-preference".to_string()];
    let metadata = MemoryMetadata::with_tags(tags.clone());

    assert_eq!(metadata.tags, tags, "Tags should be set correctly");
}

#[test]
fn test_memory_metadata_add_tag() {
    let mut metadata = MemoryMetadata::new();
    metadata.add_tag("important".to_string());
    metadata.add_tag("user-preference".to_string());

    assert_eq!(metadata.tags.len(), 2);
    assert!(metadata.tags.contains(&"important".to_string()));
    assert!(metadata.tags.contains(&"user-preference".to_string()));
}

#[test]
fn test_memory_metadata_set_source() {
    let source = "conversation";
    let mut metadata = MemoryMetadata::new();
    metadata.set_source(source.to_string());

    assert_eq!(metadata.source, source, "Source should be set correctly");
}

#[test]
fn test_memory_metadata_add_custom() {
    let mut metadata = MemoryMetadata::new();
    metadata.add_custom("key1".to_string(), serde_json::json!("value1"));
    metadata.add_custom("key2".to_string(), serde_json::json!(42));

    assert_eq!(metadata.custom.len(), 2, "Should have 2 custom fields");
    assert_eq!(
        metadata.custom.get("key1").unwrap(),
        &serde_json::json!("value1")
    );
    assert_eq!(metadata.custom.get("key2").unwrap(), &serde_json::json!(42));
}

#[test]
fn test_memory_metadata_set_image() {
    let image_data = "base64encodedimage";
    let description = "A test image";
    let mut metadata = MemoryMetadata::new();
    metadata.set_image(image_data.to_string(), Some(description.to_string()));

    assert_eq!(metadata.image_data, Some(image_data.to_string()));
    assert_eq!(metadata.image_description, Some(description.to_string()));
}

#[test]
fn test_memory_entry_creation() {
    let content = "Test memory content";
    // MemoryEntry::new() requires 3 parameters: content, memory_type, session_id
    let entry = MemoryEntry::new(content.to_string(), MemoryType::Working, None);

    assert_eq!(entry.content, content);
    assert_eq!(entry.memory_type, MemoryType::Working);
    assert!(
        entry.embedding.is_none(),
        "New entry should not have embedding"
    );
    assert_eq!(entry.access_count, 0, "Access count should start at 0");
    assert_eq!(
        entry.importance_score, 0.5,
        "Default importance should be 0.5"
    );
    assert!(
        entry.session_id.is_none(),
        "Session ID should be None by default"
    );
    assert!(
        entry.related_memories.is_empty(),
        "Related memories should be empty"
    );
}

#[test]
fn test_memory_entry_with_session() {
    let session_id = "session_123";
    // Pass session_id as third parameter to new()
    let entry = MemoryEntry::new(
        "Test".to_string(),
        MemoryType::Working,
        Some(session_id.to_string()),
    );

    assert_eq!(entry.session_id, Some(session_id.to_string()));
}

#[test]
fn test_memory_entry_with_importance() {
    // Use with_importance() static constructor for custom importance
    let entry = MemoryEntry::with_importance("Test".to_string(), MemoryType::Factual, 0.9, None);

    assert_eq!(entry.importance_score, 0.9);
}

#[test]
fn test_memory_entry_with_embedding() {
    let mut entry = MemoryEntry::new("Test".to_string(), MemoryType::Factual, None);
    let embedding = create_test_embedding(128);

    // Set embedding field directly (no builder method)
    entry.embedding = Some(embedding.clone());

    assert_eq!(entry.embedding, Some(embedding));
}

#[test]
fn test_memory_entry_with_metadata() {
    let mut metadata = MemoryMetadata::new();
    metadata.add_tag("tag1".to_string());
    metadata.set_source("test".to_string());

    let mut entry = MemoryEntry::new("Test".to_string(), MemoryType::Factual, None);
    // Set metadata field directly (no builder method)
    entry.metadata = metadata.clone();

    assert_eq!(entry.metadata.tags, metadata.tags);
    assert_eq!(entry.metadata.source, metadata.source);
}

#[test]
fn test_memory_entry_add_relation() {
    let mut entry = MemoryEntry::new("Test".to_string(), MemoryType::Semantic, None);
    let related_id = MemoryId::new();

    // Use add_relation() method
    entry.add_relation(related_id.clone());

    assert_eq!(entry.related_memories.len(), 1);
    assert_eq!(entry.related_memories[0], related_id);
}

#[test]
fn test_memory_entry_record_access() {
    let mut entry = MemoryEntry::new("Test".to_string(), MemoryType::Factual, None);
    let initial_access = entry.last_accessed;
    let initial_importance = entry.importance_score;

    std::thread::sleep(std::time::Duration::from_millis(10));
    // Use record_access() method (not increment_access)
    entry.record_access();

    assert_eq!(entry.access_count, 1);
    assert!(
        entry.last_accessed > initial_access,
        "Last accessed should be updated"
    );
    assert!(
        entry.importance_score >= initial_importance,
        "Importance should be boosted on access"
    );
}

#[test]
fn test_memory_entry_calculate_decay() {
    use chrono::Utc;

    let entry = MemoryEntry::new("Test".to_string(), MemoryType::Working, None);

    // calculate_decay() takes DateTime<Utc>, not DecayConfig
    let now = Utc::now();
    let decay_score = entry.calculate_decay(now);

    assert!(
        decay_score >= 0.0 && decay_score <= 1.0,
        "Decay score should be between 0 and 1"
    );
    // Fresh memory with default importance (0.5) should score around 0.4
    // Formula: (1.0 * 0.3 + 1.0 * 0.5 + 0.0 * 0.2) * 0.5 = 0.8 * 0.5 = 0.4
    assert!(
        (decay_score - 0.4).abs() < 0.01,
        "Fresh memory should score ~0.4, got {}",
        decay_score
    );
}

#[test]
fn test_memory_entry_serialization() {
    // Use with_importance() for custom importance
    let entry = MemoryEntry::with_importance(
        "Test content".to_string(),
        MemoryType::Factual,
        0.8,
        Some("session_1".to_string()),
    );

    let serialized = serde_json::to_string(&entry).unwrap();
    let deserialized: MemoryEntry = serde_json::from_str(&serialized).unwrap();

    assert_eq!(entry.id, deserialized.id);
    assert_eq!(entry.content, deserialized.content);
    assert_eq!(entry.memory_type, deserialized.memory_type);
    assert_eq!(entry.importance_score, deserialized.importance_score);
}

#[test]
fn test_memory_query_creation() {
    let query = MemoryQuery::new("test query".to_string());

    assert_eq!(query.query, "test query");
    assert_eq!(query.limit, 10, "Default limit should be 10");
    assert_eq!(
        query.min_similarity, 0.5,
        "Default min_similarity should be 0.5"
    );
    assert!(
        query.memory_types.is_none(),
        "Memory types should be None by default"
    );
    assert!(
        query.session_id.is_none(),
        "Session ID should be None by default"
    );
    assert!(
        !query.include_related,
        "Include related should be false by default"
    );
}

#[test]
fn test_memory_query_with_limit() {
    let query = MemoryQuery::new("test".to_string()).with_limit(20);

    assert_eq!(query.limit, 20);
}

#[test]
fn test_memory_query_with_min_similarity() {
    let query = MemoryQuery::new("test".to_string()).with_min_similarity(0.7);

    assert_eq!(query.min_similarity, 0.7);
}

#[test]
fn test_memory_query_with_memory_type() {
    let query = MemoryQuery::new("test".to_string()).with_memory_type(MemoryType::Factual);

    assert_eq!(query.memory_types, Some(vec![MemoryType::Factual]));
}

#[test]
fn test_memory_query_with_session_id() {
    let session_id = "session_123";
    let query = MemoryQuery::new("test".to_string()).with_session_id(session_id.to_string());

    assert_eq!(query.session_id, Some(session_id.to_string()));
}

#[test]
fn test_memory_query_with_tags() {
    let tags = vec!["tag1".to_string(), "tag2".to_string()];
    let query = MemoryQuery::new("test".to_string()).with_tags(tags.clone());

    assert_eq!(query.tags, Some(tags));
}

#[test]
fn test_memory_query_with_related() {
    let query = MemoryQuery::new("test".to_string()).with_related();

    assert!(query.include_related);
}

// DecayConfig tests removed - DecayConfig is part of the decay module, not types module
// These tests should be in test_decay.rs instead
