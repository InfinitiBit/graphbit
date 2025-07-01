//! Agent integration tests
//!
//! Tests for agent functionality in real-world scenarios,
//! testing actual agent configurations and LLM interactions.

use graphbit_core::agents::*;
use graphbit_core::llm::LlmConfig;
use graphbit_core::types::*;

#[tokio::test]
async fn test_agent_config_validation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let config = AgentConfig::new("Test Agent", "A test agent for validation", llm_config);

    assert_eq!(config.name, "Test Agent");
    assert_eq!(config.description, "A test agent for validation");
}

#[tokio::test]
async fn test_agent_message_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();
    let sender = AgentId::new();
    let content = MessageContent::Text("Hello, world!".to_string());

    let message = AgentMessage::new(sender.clone(), Some(agent_id.clone()), content.clone());

    assert_eq!(message.sender, sender);
    assert_eq!(message.recipient, Some(agent_id));
    match &message.content {
        MessageContent::Text(text) => assert_eq!(text, "Hello, world!"),
        _ => panic!("Expected text content"),
    }
    assert!(message.timestamp > chrono::DateTime::from_timestamp(0, 0).unwrap());
}

#[tokio::test]
async fn test_agent_message_with_metadata() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();
    let content = MessageContent::Text("Test message".to_string());

    let message = AgentMessage::new(agent_id, None, content)
        .with_metadata("priority".to_string(), serde_json::json!("high"))
        .with_metadata("type".to_string(), serde_json::json!("request"));

    assert_eq!(
        message.metadata.get("priority"),
        Some(&serde_json::json!("high"))
    );
    assert_eq!(
        message.metadata.get("type"),
        Some(&serde_json::json!("request"))
    );
}

#[tokio::test]
async fn test_agent_message_ids_unique() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id1 = AgentId::new();
    let agent_id2 = AgentId::new();

    assert_ne!(agent_id1, agent_id2, "Agent IDs should be unique");
}

#[tokio::test]
async fn test_agent_message_timestamps() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();
    let content = MessageContent::Text("First message".to_string());

    let message1 = AgentMessage::new(agent_id.clone(), None, content.clone());

    // Small delay to ensure different timestamps
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let message2 = AgentMessage::new(agent_id, None, content);

    assert!(
        message2.timestamp > message1.timestamp,
        "Timestamps should be ordered"
    );
}

#[tokio::test]
async fn test_agent_message_content_types() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();

    // Test text content
    let text_content = MessageContent::Text("Text message".to_string());
    let text_message = AgentMessage::new(agent_id.clone(), None, text_content.clone());
    match &text_message.content {
        MessageContent::Text(text) => assert_eq!(text, "Text message"),
        _ => panic!("Expected text content"),
    }

    // Test data content
    let json_data = serde_json::json!({"key": "value", "number": 42});
    let data_content = MessageContent::Data(json_data.clone());
    let data_message = AgentMessage::new(agent_id, None, data_content.clone());
    match &data_message.content {
        MessageContent::Data(data) => assert_eq!(data, &json_data),
        _ => panic!("Expected data content"),
    }
}

#[tokio::test]
async fn test_agent_capabilities() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let config = AgentConfig::new(
        "Capable Agent",
        "An agent with specific capabilities",
        llm_config,
    )
    .with_capabilities(vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::Custom("translation".to_string()),
    ]);

    assert!(config
        .capabilities
        .contains(&AgentCapability::TextProcessing));
    assert!(config.capabilities.contains(&AgentCapability::DataAnalysis));
    assert!(config
        .capabilities
        .contains(&AgentCapability::Custom("translation".to_string())));
    assert_eq!(config.capabilities.len(), 3);
}

#[tokio::test]
async fn test_agent_builder_basic() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let config = AgentConfig::new(
        "Builder Agent",
        "An agent created via builder pattern",
        llm_config,
    );

    assert_eq!(config.name, "Builder Agent");
    assert_eq!(config.description, "An agent created via builder pattern");
    assert!(config.capabilities.is_empty());
}

#[tokio::test]
async fn test_agent_builder_with_config() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let config = AgentConfig::new(
        "Configured Agent",
        "An agent with advanced configuration",
        llm_config,
    )
    .with_capabilities(vec![AgentCapability::Custom(
        "advanced-reasoning".to_string(),
    )])
    .with_max_tokens(2000)
    .with_temperature(0.8);

    assert_eq!(config.name, "Configured Agent");
    assert!(config
        .capabilities
        .contains(&AgentCapability::Custom("advanced-reasoning".to_string())));
    assert_eq!(config.max_tokens, Some(2000));
    assert_eq!(config.temperature, Some(0.8));
}

#[tokio::test]
async fn test_agent_unicode_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();
    let unicode_text = "Hello, 世界! 🌍 Здравствуй мир! مرحبا بالعالم!";
    let content = MessageContent::Text(unicode_text.to_string());

    let message = AgentMessage::new(agent_id, None, content.clone());

    match &message.content {
        MessageContent::Text(text) => {
            assert_eq!(text, unicode_text);
            assert!(text.contains("世界"));
            assert!(text.contains("🌍"));
            assert!(text.contains("мир"));
            assert!(text.contains("العالم"));
        }
        _ => panic!("Expected text content"),
    }
}

#[tokio::test]
async fn test_agent_large_message() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let agent_id = AgentId::new();

    // Create a large message (but not excessively large for tests)
    let large_text = "x".repeat(10000);
    let content = MessageContent::Text(large_text.clone());

    let message = AgentMessage::new(agent_id, None, content);

    match &message.content {
        MessageContent::Text(text) => {
            assert_eq!(text.len(), 10000);
            assert!(text.chars().all(|c| c == 'x'));
        }
        _ => panic!("Expected text content"),
    }
}

#[tokio::test]
async fn test_agent_error_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test that agent configuration validates required fields
    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());

    // Test empty name - should still work but name should be valid
    let config = AgentConfig::new("", "Valid description", llm_config.clone());
    assert_eq!(config.name, "");

    // Test empty description - should work
    let config2 = AgentConfig::new("Valid Name", "", llm_config);
    assert_eq!(config2.description, "");
}
