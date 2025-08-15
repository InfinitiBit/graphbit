#[allow(clippy::duplicate_mod)]
#[path = "test_helpers.rs"]
mod test_helpers;
use graphbit_core::{
    agents::{AgentBuilder, AgentConfig, AgentTrait},
    llm::LlmConfig,
    types::{AgentCapability, AgentId, AgentMessage, MessageContent, WorkflowContext, WorkflowId},
};
use std::sync::Arc;
use test_helpers::*;

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_creation() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "test_key".to_string(),
        model: "test_model".to_string(),
        base_url: None,
        organization: None,
    };

    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("A test agent")
        .capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis,
        ])
        .build()
        .await
        .unwrap();

    assert_eq!(agent.config().name, "test_agent");
    assert_eq!(agent.config().description, "A test agent");
    assert!(agent.has_capability(&AgentCapability::TextProcessing));
    assert!(agent.has_capability(&AgentCapability::DataAnalysis));
}

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_messaging() {
    let agent = create_test_agent().await;

    let message = AgentMessage::new(
        AgentId::new(),
        None,
        MessageContent::Text("Hello agent".to_string()),
    );
    let mut context = WorkflowContext::new(WorkflowId::new());
    let response = agent.process_message(message, &mut context).await;
    assert!(response.is_ok());
}

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_llm_interaction() {
    if !has_openai_key() {
        return;
    }

    let agent = create_test_agent().await;
    let message = AgentMessage::new(
        AgentId::new(),
        None,
        MessageContent::Text("What is 2+2?".to_string()),
    );
    let mut context = WorkflowContext::new(WorkflowId::new());
    let response = agent.process_message(message, &mut context).await.unwrap();

    match response.content {
        MessageContent::Text(text) => assert!(!text.is_empty()),
        _ => panic!("Expected text response"),
    }
}

#[test]
fn test_agent_config_capabilities() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "key".into(),
        model: "model".into(),
        base_url: None,
        organization: None,
    };
    let config = AgentConfig::new("name", "desc", llm_config)
        .with_capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis,
        ])
        .with_system_prompt("system")
        .with_max_tokens(128)
        .with_temperature(0.5);

    assert_eq!(config.capabilities.len(), 2);
    assert_eq!(config.system_prompt, "system");
    assert_eq!(config.max_tokens, Some(128));
    assert_eq!(config.temperature, Some(0.5));
}

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_error_handling() {
    let agent = create_test_agent().await;

    // Test empty message
    let empty_message =
        AgentMessage::new(AgentId::new(), None, MessageContent::Text("".to_string()));
    let mut context = WorkflowContext::new(WorkflowId::new());
    let result = agent.process_message(empty_message, &mut context).await;
    assert!(result.is_err());
}

// Removed state management test; Agent has no public state API

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_concurrent_execution() {
    let agent = create_test_agent().await;
    let agent = Arc::new(agent);

    let mut handles = vec![];

    // Spawn multiple concurrent message processing tasks
    for i in 0..5 {
        let agent_clone = Arc::clone(&agent);
        let handle = tokio::spawn(async move {
            let message = AgentMessage::new(
                AgentId::new(),
                None,
                MessageContent::Text(format!("Message {i}")),
            );
            let mut context = WorkflowContext::new(WorkflowId::new());
            agent_clone.process_message(message, &mut context).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_agent_validation() {
    // Validate AgentConfig fields without building network-dependent agent
    let llm_config = LlmConfig::OpenAI {
        api_key: "key".into(),
        model: "model".into(),
        base_url: None,
        organization: None,
    };
    let config = AgentConfig::new("name", "desc", llm_config)
        .with_max_tokens(64)
        .with_temperature(0.2);
    assert_eq!(config.max_tokens, Some(64));
    assert_eq!(config.temperature, Some(0.2));
}

#[test]
fn test_agent_capability_management() {
    // Test all capability variants
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("code_generation".to_string()),
        AgentCapability::Custom("image_processing".to_string()),
        AgentCapability::Custom("web_search".to_string()),
        AgentCapability::Custom("file_operations".to_string()),
    ];

    let llm_config = LlmConfig::OpenAI {
        api_key: "key".into(),
        model: "model".into(),
        base_url: None,
        organization: None,
    };

    let config = AgentConfig::new("test_agent", "Test agent", llm_config)
        .with_capabilities(capabilities.clone());

    assert_eq!(config.capabilities.len(), 8);
    for capability in &capabilities {
        assert!(config.capabilities.contains(capability));
    }
}

#[test]
fn test_agent_message_variants() {
    let agent_id = AgentId::new();

    // Test text message
    let text_message = AgentMessage::new(
        agent_id.clone(),
        None,
        MessageContent::Text("Hello".to_string()),
    );
    assert_eq!(text_message.sender, agent_id);
    assert!(text_message.recipient.is_none());
    match text_message.content {
        MessageContent::Text(ref text) => assert_eq!(text, "Hello"),
        _ => panic!("Expected text content"),
    }

    // Test structured message
    let structured_data = serde_json::json!({"key": "value", "number": 42});
    let structured_message = AgentMessage::new(
        agent_id.clone(),
        Some(AgentId::new()),
        MessageContent::Data(structured_data.clone()),
    );
    match structured_message.content {
        MessageContent::Data(ref data) => assert_eq!(data, &structured_data),
        _ => panic!("Expected structured content"),
    }

    // Test tool call message
    let tool_call_message = AgentMessage::new(
        agent_id.clone(),
        None,
        MessageContent::ToolCall {
            tool_name: "calculator".to_string(),
            parameters: serde_json::json!({"operation": "add", "a": 1, "b": 2}),
        },
    );
    match tool_call_message.content {
        MessageContent::ToolCall {
            ref tool_name,
            ref parameters,
        } => {
            assert_eq!(tool_name, "calculator");
            assert_eq!(parameters["operation"], "add");
        }
        _ => panic!("Expected tool call content"),
    }
}

#[test]
fn test_agent_config_builder_edge_cases() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "key".into(),
        model: "model".into(),
        base_url: None,
        organization: None,
    };

    // Test with empty capabilities
    let config =
        AgentConfig::new("agent", "description", llm_config.clone()).with_capabilities(vec![]);
    assert!(config.capabilities.is_empty());

    // Test with extreme values
    let config = AgentConfig::new("agent", "description", llm_config.clone())
        .with_max_tokens(1)
        .with_temperature(0.0);
    assert_eq!(config.max_tokens, Some(1));
    assert_eq!(config.temperature, Some(0.0));

    let config = AgentConfig::new("agent", "description", llm_config.clone())
        .with_max_tokens(4096)
        .with_temperature(2.0);
    assert_eq!(config.max_tokens, Some(4096));
    assert_eq!(config.temperature, Some(2.0));

    // Test with very long system prompt
    let long_prompt = "a".repeat(1000);
    let config =
        AgentConfig::new("agent", "description", llm_config).with_system_prompt(&long_prompt);
    assert_eq!(config.system_prompt, long_prompt);
}

#[test]
fn test_agent_id_generation() {
    let id1 = AgentId::new();
    let id2 = AgentId::new();

    // IDs should be unique
    assert_ne!(id1, id2);

    // IDs should be consistent when cloned
    let id1_clone = id1.clone();
    assert_eq!(id1, id1_clone);
}

#[tokio::test]
async fn test_agent_builder_validation() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "test_key".to_string(),
        model: "test_model".to_string(),
        base_url: None,
        organization: None,
    };

    // Test builder with minimal configuration
    let _builder = AgentBuilder::new("minimal_agent", llm_config.clone());
    // Should be able to build with minimal config (though it may fail at runtime without real API)

    // Test builder with full configuration
    let _builder = AgentBuilder::new("full_agent", llm_config)
        .description("Full featured agent")
        .capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis,
            AgentCapability::ToolExecution,
        ])
        .system_prompt("You are a helpful assistant")
        .max_tokens(512)
        .temperature(0.7);

    // Builder should be configured correctly
    // Note: We can't easily test the build() method without a real API key
}
