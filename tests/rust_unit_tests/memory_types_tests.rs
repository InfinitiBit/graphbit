use graphbit_core::memory::{
    MemoryAction, MemoryConfig, MemoryId, MemoryScope,
};
use std::collections::HashMap;

#[test]
fn test_memory_id_creation() {
    let id1 = MemoryId::new();
    let id2 = MemoryId::new();
    assert_ne!(id1, id2, "Two fresh IDs should be unique");

    let id_str = id1.to_string();
    let parsed = MemoryId::from_string(&id_str).expect("should parse valid UUID");
    assert_eq!(id1, parsed);
}

#[test]
fn test_memory_id_default() {
    let id = MemoryId::default();
    // Default should produce a valid UUID
    let _ = id.as_uuid();
    assert!(!id.to_string().is_empty());
}

#[test]
fn test_memory_id_from_invalid_string() {
    let result = MemoryId::from_string("not-a-uuid");
    assert!(result.is_err());
}

#[test]
fn test_memory_id_display() {
    let id = MemoryId::new();
    let display = format!("{id}");
    // UUID format: 8-4-4-4-12 hex chars
    assert_eq!(display.len(), 36);
    assert_eq!(display.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn test_memory_id_equality_and_hash() {
    let id = MemoryId::new();
    let cloned = id.clone();
    assert_eq!(id, cloned);

    // Test HashMap usage (requires Hash + Eq)
    let mut map = HashMap::new();
    map.insert(id.clone(), "value");
    assert_eq!(map.get(&id), Some(&"value"));
}

#[test]
fn test_memory_scope_default() {
    let scope = MemoryScope::default();
    assert!(scope.user_id.is_none());
    assert!(scope.agent_id.is_none());
    assert!(scope.run_id.is_none());
}

#[test]
fn test_memory_scope_with_fields() {
    let scope = MemoryScope {
        user_id: Some("user1".to_string()),
        agent_id: Some("agent1".to_string()),
        run_id: Some("run1".to_string()),
    };
    assert_eq!(scope.user_id.as_deref(), Some("user1"));
    assert_eq!(scope.agent_id.as_deref(), Some("agent1"));
    assert_eq!(scope.run_id.as_deref(), Some("run1"));
}

#[test]
fn test_memory_action_display() {
    assert_eq!(MemoryAction::Add.to_string(), "ADD");
    assert_eq!(MemoryAction::Update.to_string(), "UPDATE");
    assert_eq!(MemoryAction::Delete.to_string(), "DELETE");
    assert_eq!(MemoryAction::Noop.to_string(), "NOOP");
}

#[test]
fn test_memory_action_from_str_lossy() {
    assert_eq!(MemoryAction::from_str_lossy("ADD"), MemoryAction::Add);
    assert_eq!(MemoryAction::from_str_lossy("add"), MemoryAction::Add);
    assert_eq!(MemoryAction::from_str_lossy("Add"), MemoryAction::Add);
    assert_eq!(MemoryAction::from_str_lossy("UPDATE"), MemoryAction::Update);
    assert_eq!(MemoryAction::from_str_lossy("update"), MemoryAction::Update);
    assert_eq!(MemoryAction::from_str_lossy("DELETE"), MemoryAction::Delete);
    assert_eq!(MemoryAction::from_str_lossy("delete"), MemoryAction::Delete);
    assert_eq!(MemoryAction::from_str_lossy("unknown"), MemoryAction::Noop);
    assert_eq!(MemoryAction::from_str_lossy(""), MemoryAction::Noop);
}

#[test]
fn test_memory_action_equality() {
    assert_eq!(MemoryAction::Add, MemoryAction::Add);
    assert_ne!(MemoryAction::Add, MemoryAction::Update);
    assert_ne!(MemoryAction::Delete, MemoryAction::Noop);
}

#[test]
fn test_memory_config_builder() {
    let llm_config = graphbit_core::llm::LlmConfig::openai("test-key", "gpt-4o-mini");
    let embedding_config = graphbit_core::embeddings::EmbeddingConfig {
        provider: graphbit_core::embeddings::EmbeddingProvider::OpenAI,
        api_key: "test-key".to_string(),
        model: "text-embedding-3-small".to_string(),
        base_url: None,
        timeout_seconds: None,
        max_batch_size: None,
        extra_params: HashMap::new(),
        python_instance: None,
    };

    let config = MemoryConfig::new(llm_config, embedding_config)
        .with_db_path(":memory:")
        .with_similarity_threshold(0.8)
        .with_max_extraction_tokens(2000)
        .with_extraction_temperature(0.2);

    assert_eq!(config.db_path, ":memory:");
    assert!((config.similarity_threshold - 0.8).abs() < f64::EPSILON);
    assert_eq!(config.max_extraction_tokens, 2000);
    assert!((config.extraction_temperature - 0.2).abs() < f32::EPSILON);
}

#[test]
fn test_memory_config_defaults() {
    let llm_config = graphbit_core::llm::LlmConfig::openai("key", "model");
    let embedding_config = graphbit_core::embeddings::EmbeddingConfig {
        provider: graphbit_core::embeddings::EmbeddingProvider::OpenAI,
        api_key: "key".to_string(),
        model: "model".to_string(),
        base_url: None,
        timeout_seconds: None,
        max_batch_size: None,
        extra_params: HashMap::new(),
        python_instance: None,
    };

    let config = MemoryConfig::new(llm_config, embedding_config);
    assert_eq!(config.db_path, "graphbit_memory.db");
    assert!((config.similarity_threshold - 0.7).abs() < f64::EPSILON);
    assert_eq!(config.max_extraction_tokens, 1500);
    assert!((config.extraction_temperature - 0.1).abs() < f32::EPSILON);
}

#[test]
fn test_memory_config_threshold_clamping() {
    let llm_config = graphbit_core::llm::LlmConfig::openai("key", "model");
    let embedding_config = graphbit_core::embeddings::EmbeddingConfig {
        provider: graphbit_core::embeddings::EmbeddingProvider::OpenAI,
        api_key: "key".to_string(),
        model: "model".to_string(),
        base_url: None,
        timeout_seconds: None,
        max_batch_size: None,
        extra_params: HashMap::new(),
        python_instance: None,
    };

    // Threshold above 1.0 should be clamped
    let config = MemoryConfig::new(llm_config.clone(), embedding_config.clone())
        .with_similarity_threshold(1.5);
    assert!((config.similarity_threshold - 1.0).abs() < f64::EPSILON);

    // Threshold below 0.0 should be clamped
    let config = MemoryConfig::new(llm_config.clone(), embedding_config.clone())
        .with_similarity_threshold(-0.5);
    assert!(config.similarity_threshold.abs() < f64::EPSILON);

    // Temperature above 1.0 should be clamped
    let config = MemoryConfig::new(llm_config, embedding_config)
        .with_extraction_temperature(2.0);
    assert!((config.extraction_temperature - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_memory_action_serialization() {
    let action = MemoryAction::Add;
    let json = serde_json::to_string(&action).expect("serialize ok");
    let deserialized: MemoryAction = serde_json::from_str(&json).expect("deserialize ok");
    assert_eq!(action, deserialized);
}

#[test]
fn test_memory_scope_serialization() {
    let scope = MemoryScope {
        user_id: Some("user1".to_string()),
        agent_id: None,
        run_id: Some("run1".to_string()),
    };
    let json = serde_json::to_string(&scope).expect("serialize ok");
    let deserialized: MemoryScope = serde_json::from_str(&json).expect("deserialize ok");
    assert_eq!(deserialized.user_id.as_deref(), Some("user1"));
    assert!(deserialized.agent_id.is_none());
    assert_eq!(deserialized.run_id.as_deref(), Some("run1"));
}

#[test]
fn test_memory_id_serialization() {
    let id = MemoryId::new();
    let json = serde_json::to_string(&id).expect("serialize ok");
    let deserialized: MemoryId = serde_json::from_str(&json).expect("deserialize ok");
    assert_eq!(id, deserialized);
}
