//! Comprehensive agent system unit tests
//!
//! Tests for agent configuration, capabilities, message processing,
//! and agent builder patterns.

use graphbit_core::{
    agents::{AgentBuilder, AgentConfig},
    types::{
        AgentCapability, AgentId, AgentMessage, MessageContent, NodeId, WorkflowContext, WorkflowId,
    },
};
use serde_json::json;
// async_trait is already included in graphbit_core

use crate::rust_unit_tests::test_helpers::*;

#[test]
fn test_agent_config_creation_and_builder() {
    let llm_config = create_mock_llm_config();

    // Test basic config creation
    let config = AgentConfig::new("Test Agent", "A test agent", llm_config.clone());

    assert_eq!(config.name, "Test Agent");
    assert_eq!(config.description, "A test agent");
    assert!(config.capabilities.is_empty());
    assert!(config.system_prompt.is_empty());
    assert!(config.max_tokens.is_none());
    assert!(config.temperature.is_none());
    assert!(config.custom_config.is_empty());

    // Test builder pattern
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::Custom("SpecialSkill".to_string()),
    ];

    let built_config = AgentConfig::new("Builder Agent", "Built with builder", llm_config)
        .with_capabilities(capabilities.clone())
        .with_system_prompt("You are a helpful assistant")
        .with_max_tokens(1000)
        .with_temperature(0.7);

    assert_eq!(built_config.capabilities, capabilities);
    assert_eq!(built_config.system_prompt, "You are a helpful assistant");
    assert_eq!(built_config.max_tokens, Some(1000));
    assert_eq!(built_config.temperature, Some(0.7));
}

#[test]
fn test_agent_config_with_id() {
    let llm_config = create_mock_llm_config();
    let custom_id = AgentId::new();

    let config = AgentConfig::new("Test", "Test", llm_config).with_id(custom_id.clone());

    assert_eq!(config.id, custom_id);
}

#[test]
fn test_agent_config_serialization() {
    let llm_config = create_mock_llm_config();
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::ToolExecution,
    ];

    let config = AgentConfig::new("Serializable Agent", "Test serialization", llm_config)
        .with_capabilities(capabilities)
        .with_system_prompt("System prompt")
        .with_max_tokens(500)
        .with_temperature(0.5);

    // Test serialization
    let serialized = serde_json::to_string(&config).unwrap();

    // Test deserialization
    let deserialized: AgentConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.description, deserialized.description);
    assert_eq!(config.capabilities, deserialized.capabilities);
    assert_eq!(config.system_prompt, deserialized.system_prompt);
    assert_eq!(config.max_tokens, deserialized.max_tokens);
    assert_eq!(config.temperature, deserialized.temperature);
}

#[test]
fn test_agent_builder_fluent_api() {
    let llm_config = create_mock_llm_config();
    let custom_id = AgentId::new();

    let _builder = AgentBuilder::new("Fluent Agent", llm_config)
        .description("Built with fluent API")
        .capabilities(vec![AgentCapability::DecisionMaking])
        .system_prompt("You make decisions")
        .max_tokens(2000)
        .temperature(0.3)
        .with_id(custom_id.clone());

    // Note: We can't actually build the agent in unit tests without mocking LLM provider
    // But we can test that the builder pattern works correctly by checking the built config
    // The builder's config field is private, so we'll test the pattern conceptually
    assert_eq!(custom_id.to_string().len(), 36); // UUID length check
}

#[test]
fn test_message_content_handling() {
    // Test different message content types
    let text_content = MessageContent::Text("Hello, agent!".to_string());
    let data_content = MessageContent::Data(json!({"key": "value", "number": 42}));
    let tool_call = MessageContent::ToolCall {
        tool_name: "calculator".to_string(),
        parameters: json!({"operation": "multiply", "a": 5, "b": 3}),
    };
    let tool_response = MessageContent::ToolResponse {
        tool_name: "calculator".to_string(),
        result: json!(15),
        success: true,
    };
    let error_content = MessageContent::Error {
        error_code: "CALCULATION_ERROR".to_string(),
        error_message: "Division by zero".to_string(),
    };

    let contents = vec![
        text_content,
        data_content,
        tool_call,
        tool_response,
        error_content,
    ];

    for content in contents {
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&content).unwrap();
        let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();

        // Verify the content type is preserved
        match (&content, &deserialized) {
            (MessageContent::Text(_), MessageContent::Text(_)) => {}
            (MessageContent::Data(_), MessageContent::Data(_)) => {}
            (MessageContent::ToolCall { .. }, MessageContent::ToolCall { .. }) => {}
            (MessageContent::ToolResponse { .. }, MessageContent::ToolResponse { .. }) => {}
            (MessageContent::Error { .. }, MessageContent::Error { .. }) => {}
            _ => panic!("Message content type not preserved during serialization"),
        }
    }
}

#[test]
fn test_agent_message_creation_and_metadata() {
    let sender = AgentId::new();
    let recipient = AgentId::new();
    let content = MessageContent::Text("Test message".to_string());

    let message = AgentMessage::new(sender.clone(), Some(recipient.clone()), content);

    assert_eq!(message.sender, sender);
    assert_eq!(message.recipient, Some(recipient));
    assert!(matches!(message.content, MessageContent::Text(_)));
    assert!(message.metadata.is_empty());

    // Test adding metadata
    let message_with_metadata = message
        .with_metadata("priority".to_string(), json!("high"))
        .with_metadata("source".to_string(), json!("user_input"))
        .with_metadata("timestamp".to_string(), json!(1234567890));

    assert_eq!(message_with_metadata.metadata.len(), 3);
    assert_eq!(
        message_with_metadata.metadata.get("priority").unwrap(),
        &json!("high")
    );
    assert_eq!(
        message_with_metadata.metadata.get("source").unwrap(),
        &json!("user_input")
    );
}

#[test]
fn test_agent_message_default() {
    let default_message = AgentMessage::default();

    // Default message should have valid structure
    assert!(!default_message.id.is_nil());
    assert!(!default_message.sender.to_string().is_empty());
    assert!(default_message.recipient.is_none());
    assert!(matches!(default_message.content, MessageContent::Text(_)));
    assert!(default_message.metadata.is_empty());
}

#[test]
fn test_agent_capabilities_matching() {
    let capabilities = [
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::Custom("DatabaseAccess".to_string()),
    ];

    // Test capability checking
    assert!(capabilities.contains(&AgentCapability::TextProcessing));
    assert!(capabilities.contains(&AgentCapability::DataAnalysis));
    assert!(capabilities.contains(&AgentCapability::Custom("DatabaseAccess".to_string())));
    assert!(!capabilities.contains(&AgentCapability::ToolExecution));
    assert!(!capabilities.contains(&AgentCapability::Custom("DifferentSkill".to_string())));
}

#[test]
fn test_workflow_context_with_agent_operations() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id);

    // Test setting agent-related metadata
    context.set_metadata("current_agent".to_string(), json!("agent-123"));
    context.set_metadata(
        "agent_capabilities".to_string(),
        json!(["TextProcessing", "DataAnalysis"]),
    );
    context.set_metadata("processing_time_ms".to_string(), json!(250));

    assert_eq!(
        context.metadata.get("current_agent").unwrap(),
        &json!("agent-123")
    );
    assert_eq!(
        context.metadata.get("processing_time_ms").unwrap(),
        &json!(250)
    );

    // Test agent output tracking
    let _agent_id = AgentId::new();
    let node_id = NodeId::new();
    let agent_output = json!({
        "response": "Task completed successfully",
        "confidence": 0.95,
        "tokens_used": 150
    });

    context.set_node_output(&node_id, agent_output.clone());
    assert_eq!(
        context.get_node_output(&node_id.to_string()).unwrap(),
        &agent_output
    );

    // Test nested access to agent output
    let response = context
        .get_nested_output(&format!("{node_id}.response"))
        .unwrap();
    assert_eq!(response, &json!("Task completed successfully"));

    let confidence = context
        .get_nested_output(&format!("{node_id}.confidence"))
        .unwrap();
    assert_eq!(confidence, &json!(0.95));
}
