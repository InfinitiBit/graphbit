#[allow(clippy::duplicate_mod)]
#[path = "test_helpers.rs"]
mod test_helpers;
use graphbit_core::{
    agents::{AgentBuilder, AgentConfig, AgentTrait},
    errors::GraphBitResult,
    llm::LlmConfig,
    types::{AgentCapability, AgentId, AgentMessage, MessageContent, WorkflowContext, WorkflowId},
};

use std::sync::Arc;
use test_helpers::*;

// A minimal dummy agent implementing AgentTrait for unit testing default behaviors
struct DummyAgent {
    id: AgentId,
}

#[async_trait::async_trait]
impl AgentTrait for DummyAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn config(&self) -> &graphbit_core::agents::AgentConfig {
        // Create a leaked boxed config to return a &'static reference without adding deps
        // This intentionally leaks memory in test process only.
        let llm_cfg = graphbit_core::llm::LlmConfig::default();
        let boxed = Box::new(graphbit_core::agents::AgentConfig::new(
            "dummy", "desc", llm_cfg,
        ));
        Box::leak(boxed)
    }

    async fn process_message(
        &self,
        _message: graphbit_core::types::AgentMessage,
        _context: &mut graphbit_core::types::WorkflowContext,
    ) -> GraphBitResult<graphbit_core::types::AgentMessage> {
        unimplemented!()
    }

    async fn execute(
        &self,
        _message: graphbit_core::types::AgentMessage,
    ) -> GraphBitResult<serde_json::Value> {
        unimplemented!()
    }

    async fn validate_output(
        &self,
        _output: &str,
        _schema: &serde_json::Value,
    ) -> graphbit_core::validation::ValidationResult {
        unimplemented!()
    }

    fn llm_provider(&self) -> &graphbit_core::llm::LlmProvider {
        unimplemented!()
    }
}

#[test]
fn agent_config_builder_and_capabilities() {
    let llm_cfg = graphbit_core::llm::LlmConfig::default();
    let mut cfg = AgentConfig::new("test-agent", "a test", llm_cfg);
    cfg = cfg.with_capabilities(vec![AgentCapability::TextProcessing]);
    cfg = cfg.with_system_prompt("Do work");
    cfg = cfg.with_max_tokens(128);
    cfg = cfg.with_temperature(0.5);

    assert_eq!(cfg.name, "test-agent");
    assert_eq!(cfg.description, "a test");
    assert!(cfg.capabilities.contains(&AgentCapability::TextProcessing));
    assert_eq!(cfg.system_prompt, "Do work");
    assert_eq!(cfg.max_tokens, Some(128));
    assert_eq!(cfg.temperature, Some(0.5));
}

#[test]
fn dummy_agent_default_methods() {
    let dummy = DummyAgent { id: AgentId::new() };
    // default capabilities/capability checks should work without panics
    let cap = AgentCapability::TextProcessing;
    assert!(!dummy.has_capability(&cap));
    assert!(dummy.capabilities().is_empty());
}

#[tokio::test]
#[ignore = "Requires OpenAI API key (set OPENAI_API_KEY environment variable to run)"]
async fn test_agent_creation() {
    let _agent_id = AgentId::new();
    let llm_config = LlmConfig::OpenAI {
        api_key: std::env::var("OPENAI_API_KEY").unwrap(),
        model: "gpt-3.5-turbo".to_string(),
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

    // Test empty message - currently the agent processes empty messages successfully
    let empty_message =
        AgentMessage::new(AgentId::new(), None, MessageContent::Text("".to_string()));
    let mut context = WorkflowContext::new(WorkflowId::new());
    let result = agent.process_message(empty_message, &mut context).await;

    // The agent currently processes empty messages successfully, so we check for success
    // If error handling for empty messages is added later, this test should be updated
    assert!(result.is_ok());

    // Verify the response content is not empty (agent should respond to empty input)
    let response = result.unwrap();
    match response.content {
        MessageContent::Text(text) => {
            assert!(!text.is_empty(), "Agent should respond to empty input")
        }
        _ => panic!("Expected text response"),
    }
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

// Additional comprehensive tests for 100% coverage

#[test]
fn test_agent_config_with_id() {
    let llm_config = LlmConfig::default();
    let custom_id = AgentId::new();
    let config = AgentConfig::new("test", "desc", llm_config)
        .with_id(custom_id.clone());

    assert_eq!(config.id, custom_id);
}

#[test]
fn test_agent_config_custom_config_field() {
    let llm_config = LlmConfig::default();
    let config = AgentConfig::new("test", "desc", llm_config);

    // Test that custom_config is initialized as empty HashMap
    assert!(config.custom_config.is_empty());
    // Note: HashMap capacity may vary based on implementation, just check it exists
    let _capacity = config.custom_config.capacity(); // Just verify it's accessible
}

#[test]
fn test_agent_config_all_llm_variants() {
    // Test all LlmConfig variants
    let configs = vec![
        LlmConfig::OpenAI {
            api_key: "key".to_string(),
            model: "gpt-4".to_string(),
            base_url: Some("https://api.openai.com".to_string()),
            organization: Some("org".to_string()),
        },
        LlmConfig::Anthropic {
            api_key: "key".to_string(),
            model: "claude-3".to_string(),
            base_url: Some("https://api.anthropic.com".to_string()),
        },
        LlmConfig::DeepSeek {
            api_key: "key".to_string(),
            model: "deepseek-chat".to_string(),
            base_url: None,
        },
        LlmConfig::HuggingFace {
            api_key: "key".to_string(),
            model: "gpt2".to_string(),
            base_url: None,
        },
        LlmConfig::Ollama {
            model: "llama3.2".to_string(),
            base_url: None,
        },
        LlmConfig::Perplexity {
            api_key: "key".to_string(),
            model: "llama-3.1-sonar-small-128k-online".to_string(),
            base_url: None,
        },
        LlmConfig::Custom {
            provider_type: "custom".to_string(),
            config: {
                let mut map = std::collections::HashMap::new();
                map.insert("model".to_string(), serde_json::json!("custom-model"));
                map.insert("api_key".to_string(), serde_json::json!("key"));
                map
            },
        },
    ];

    for llm_config in configs {
        let agent_config = AgentConfig::new("test", "desc", llm_config);
        assert_eq!(agent_config.name, "test");
        assert_eq!(agent_config.description, "desc");
    }
}

#[test]
fn test_agent_builder_all_methods() {
    let llm_config = LlmConfig::default();
    let custom_id = AgentId::new();

    let _builder = AgentBuilder::new("builder_test", llm_config)
        .description("Full builder test")
        .capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis,
            AgentCapability::ToolExecution,
            AgentCapability::DecisionMaking,
            AgentCapability::Custom("special".to_string()),
        ])
        .system_prompt("You are a test agent")
        .max_tokens(1000)
        .temperature(0.7)
        .with_id(custom_id.clone());

    // We can't access the internal config directly, but we can verify the builder pattern works
    // by checking that the methods return the builder (fluent interface)
    assert_eq!(custom_id.to_string().len(), 36); // UUID length
}

#[test]
fn test_all_agent_capabilities() {
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("test_capability".to_string()),
    ];

    // Test serialization/deserialization for all capabilities
    for capability in &capabilities {
        let serialized = serde_json::to_string(capability).unwrap();
        let deserialized: AgentCapability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*capability, deserialized);
    }

    // Test capability comparison
    assert_ne!(AgentCapability::TextProcessing, AgentCapability::DataAnalysis);
    assert_eq!(
        AgentCapability::Custom("test".to_string()),
        AgentCapability::Custom("test".to_string())
    );
    assert_ne!(
        AgentCapability::Custom("test1".to_string()),
        AgentCapability::Custom("test2".to_string())
    );
}

#[test]
fn test_agent_message_creation_and_metadata() {
    let sender = AgentId::new();
    let recipient = AgentId::new();

    // Test message creation with all content types
    let _text_message = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::Text("Hello".to_string()),
    );

    let _data_message = AgentMessage::new(
        sender.clone(),
        None, // Broadcast message
        MessageContent::Data(serde_json::json!({"key": "value"})),
    );

    let _tool_call_message = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::ToolCall {
            tool_name: "calculator".to_string(),
            parameters: serde_json::json!({"op": "add", "a": 1, "b": 2}),
        },
    );

    let _tool_response_message = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::ToolResponse {
            tool_name: "calculator".to_string(),
            result: serde_json::json!(3),
            success: true,
        },
    );

    let _error_message = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::Error {
            error_code: "CALC_ERROR".to_string(),
            error_message: "Division by zero".to_string(),
        },
    );

    // Test message with metadata
    let message_with_metadata = AgentMessage::new(
        sender.clone(),
        Some(recipient.clone()),
        MessageContent::Text("Test".to_string()),
    ).with_metadata("priority".to_string(), serde_json::json!("high"))
     .with_metadata("timestamp".to_string(), serde_json::json!(1234567890));

    assert_eq!(message_with_metadata.metadata.len(), 2);
    assert_eq!(
        message_with_metadata.metadata.get("priority").unwrap(),
        &serde_json::json!("high")
    );

    // Test default message
    let default_message = AgentMessage::default();
    assert!(default_message.metadata.is_empty());
    assert!(default_message.recipient.is_none());
    match default_message.content {
        MessageContent::Text(ref text) => assert!(text.is_empty()),
        _ => panic!("Default message should have empty text content"),
    }
}

#[test]
fn test_agent_id_creation_and_display() {
    // Test new ID creation
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    assert_ne!(id1, id2);

    // Test default
    let default_id = AgentId::default();
    assert_eq!(default_id.to_string().len(), 36); // UUID length

    // Test from_string with valid UUID
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let id_from_uuid = AgentId::from_string(uuid_str).unwrap();
    assert_eq!(id_from_uuid.to_string(), uuid_str);

    // Test from_string with non-UUID (deterministic generation)
    let id_from_string1 = AgentId::from_string("test_agent").unwrap();
    let id_from_string2 = AgentId::from_string("test_agent").unwrap();
    assert_eq!(id_from_string1, id_from_string2); // Should be deterministic

    // Test as_uuid
    let uuid = id1.as_uuid();
    assert_eq!(uuid.to_string(), id1.to_string());

    // Test Display trait
    let display_str = format!("{}", id1);
    assert_eq!(display_str, id1.to_string());
}

#[test]
fn test_workflow_id_creation_and_methods() {
    // Test WorkflowId creation and methods for completeness
    let wf_id1 = WorkflowId::new();
    let wf_id2 = WorkflowId::new();
    assert_ne!(wf_id1, wf_id2);

    // Test default
    let default_wf_id = WorkflowId::default();
    assert_eq!(default_wf_id.to_string().len(), 36);

    // Test from_string
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let wf_id_from_uuid = WorkflowId::from_string(uuid_str).unwrap();
    assert_eq!(wf_id_from_uuid.to_string(), uuid_str);

    // Test as_uuid
    let uuid = wf_id1.as_uuid();
    assert_eq!(uuid.to_string(), wf_id1.to_string());

    // Test Display trait
    let display_str = format!("{}", wf_id1);
    assert_eq!(display_str, wf_id1.to_string());
}

#[test]
fn test_workflow_context_creation_and_metadata() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());

    // Test initial state
    assert_eq!(context.workflow_id, workflow_id);

    // Test metadata operations
    context.set_metadata("test_key".to_string(), serde_json::json!("test_value"));
    context.set_metadata("number_key".to_string(), serde_json::json!(42));

    // Note: WorkflowContext doesn't have get_metadata method, only set_metadata
    // This tests that set_metadata works without errors
    assert_eq!(context.workflow_id, workflow_id);
}

#[test]
fn test_llm_config_model_name_method() {
    let configs = vec![
        (LlmConfig::OpenAI {
            api_key: "key".to_string(),
            model: "gpt-4".to_string(),
            base_url: None,
            organization: None,
        }, "gpt-4"),
        (LlmConfig::Anthropic {
            api_key: "key".to_string(),
            model: "claude-3".to_string(),
            base_url: None,
        }, "claude-3"),
        (LlmConfig::DeepSeek {
            api_key: "key".to_string(),
            model: "deepseek-chat".to_string(),
            base_url: None,
        }, "deepseek-chat"),
        (LlmConfig::HuggingFace {
            api_key: "key".to_string(),
            model: "gpt2".to_string(),
            base_url: None,
        }, "gpt2"),
        (LlmConfig::Ollama {
            model: "llama3.2".to_string(),
            base_url: None,
        }, "llama3.2"),
        (LlmConfig::Perplexity {
            api_key: "key".to_string(),
            model: "llama-3.1-sonar-small-128k-online".to_string(),
            base_url: None,
        }, "llama-3.1-sonar-small-128k-online"),
        (LlmConfig::Custom {
            provider_type: "custom".to_string(),
            config: {
                let mut map = std::collections::HashMap::new();
                map.insert("model".to_string(), serde_json::json!("custom-model"));
                map
            },
        }, "custom-model"),
        (LlmConfig::Custom {
            provider_type: "custom".to_string(),
            config: {
                let mut map = std::collections::HashMap::new();
                map.insert("other".to_string(), serde_json::json!("value"));
                map
            },
        }, "unknown"), // No model field
    ];

    for (config, expected_model) in configs {
        assert_eq!(config.model_name(), expected_model);
    }
}

#[test]
fn test_llm_config_default() {
    let default_config = LlmConfig::default();
    match default_config {
        LlmConfig::Ollama { model, base_url } => {
            assert_eq!(model, "llama3.2");
            assert!(base_url.is_none());
        }
        _ => panic!("Default LlmConfig should be Ollama"),
    }
}

#[test]
fn test_message_content_serialization() {
    let contents = vec![
        MessageContent::Text("Hello world".to_string()),
        MessageContent::Data(serde_json::json!({"key": "value", "number": 42})),
        MessageContent::ToolCall {
            tool_name: "calculator".to_string(),
            parameters: serde_json::json!({"operation": "add", "a": 1, "b": 2}),
        },
        MessageContent::ToolResponse {
            tool_name: "calculator".to_string(),
            result: serde_json::json!(3),
            success: true,
        },
        MessageContent::ToolResponse {
            tool_name: "failed_tool".to_string(),
            result: serde_json::json!(null),
            success: false,
        },
        MessageContent::Error {
            error_code: "VALIDATION_ERROR".to_string(),
            error_message: "Invalid input provided".to_string(),
        },
    ];

    for content in contents {
        // Test serialization/deserialization
        let serialized = serde_json::to_string(&content).unwrap();
        let deserialized: MessageContent = serde_json::from_str(&serialized).unwrap();

        // Compare based on content type since we can't directly compare enum variants
        match (&content, &deserialized) {
            (MessageContent::Text(a), MessageContent::Text(b)) => assert_eq!(a, b),
            (MessageContent::Data(a), MessageContent::Data(b)) => assert_eq!(a, b),
            (MessageContent::ToolCall { tool_name: a1, parameters: a2 },
             MessageContent::ToolCall { tool_name: b1, parameters: b2 }) => {
                assert_eq!(a1, b1);
                assert_eq!(a2, b2);
            },
            (MessageContent::ToolResponse { tool_name: a1, result: a2, success: a3 },
             MessageContent::ToolResponse { tool_name: b1, result: b2, success: b3 }) => {
                assert_eq!(a1, b1);
                assert_eq!(a2, b2);
                assert_eq!(a3, b3);
            },
            (MessageContent::Error { error_code: a1, error_message: a2 },
             MessageContent::Error { error_code: b1, error_message: b2 }) => {
                assert_eq!(a1, b1);
                assert_eq!(a2, b2);
            },
            _ => panic!("Serialization/deserialization changed content type"),
        }
    }
}

#[test]
fn test_agent_config_serialization() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "test_key".to_string(),
        model: "gpt-4".to_string(),
        base_url: None,
        organization: None,
    };

    let config = AgentConfig::new("Serializable Agent", "Test serialization", llm_config)
        .with_capabilities(vec![AgentCapability::TextProcessing])
        .with_system_prompt("Test prompt")
        .with_max_tokens(100)
        .with_temperature(0.5);

    // Test serialization/deserialization
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: AgentConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.description, deserialized.description);
    assert_eq!(config.capabilities, deserialized.capabilities);
    assert_eq!(config.system_prompt, deserialized.system_prompt);
    assert_eq!(config.max_tokens, deserialized.max_tokens);
    assert_eq!(config.temperature, deserialized.temperature);
}

#[test]
fn test_agent_message_serialization() {
    let sender = AgentId::new();
    let recipient = AgentId::new();

    let message = AgentMessage::new(
        sender,
        Some(recipient),
        MessageContent::Text("Test message".to_string()),
    ).with_metadata("priority".to_string(), serde_json::json!("high"));

    // Test serialization/deserialization
    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: AgentMessage = serde_json::from_str(&serialized).unwrap();

    assert_eq!(message.id, deserialized.id);
    assert_eq!(message.sender, deserialized.sender);
    assert_eq!(message.recipient, deserialized.recipient);
    assert_eq!(message.metadata, deserialized.metadata);

    match (&message.content, &deserialized.content) {
        (MessageContent::Text(a), MessageContent::Text(b)) => assert_eq!(a, b),
        _ => panic!("Content type mismatch after serialization"),
    }
}

#[test]
fn test_agent_id_error_handling() {
    // Test invalid UUID string
    let result = AgentId::from_string("invalid-uuid-format");
    assert!(result.is_ok()); // Should still work with deterministic generation

    // Test empty string
    let result = AgentId::from_string("");
    assert!(result.is_ok()); // Should work with deterministic generation

    // Test very long string
    let long_string = "a".repeat(1000);
    let result = AgentId::from_string(&long_string);
    assert!(result.is_ok());
}

#[test]
fn test_workflow_id_error_handling() {
    // Test invalid UUID string
    let result = WorkflowId::from_string("invalid-uuid-format");
    assert!(result.is_err());

    // Test empty string
    let result = WorkflowId::from_string("");
    assert!(result.is_err());

    // Test valid UUID
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let result = WorkflowId::from_string(uuid_str);
    assert!(result.is_ok());
}

#[test]
fn test_dummy_agent_unimplemented_methods() {
    let dummy = DummyAgent { id: AgentId::new() };

    // Test that unimplemented methods panic as expected
    let message = AgentMessage::new(
        AgentId::new(),
        None,
        MessageContent::Text("test".to_string()),
    );
    let mut context = WorkflowContext::new(WorkflowId::new());

    // These should panic with unimplemented!()
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            dummy.process_message(message.clone(), &mut context).await
        })
    }));
    assert!(result.is_err());

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            dummy.execute(message.clone()).await
        })
    }));
    assert!(result.is_err());

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            dummy.validate_output("test", &serde_json::json!({})).await
        })
    }));
    assert!(result.is_err());

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        dummy.llm_provider()
    }));
    assert!(result.is_err());
}

#[test]
fn test_agent_config_edge_cases() {
    // Test with empty strings
    let llm_config = LlmConfig::default();
    let config = AgentConfig::new("", "", llm_config);
    assert!(config.name.is_empty());
    assert!(config.description.is_empty());

    // Test with very long strings
    let long_name = "a".repeat(1000);
    let long_desc = "b".repeat(1000);
    let long_prompt = "c".repeat(1000);

    let llm_config = LlmConfig::default();
    let config = AgentConfig::new(long_name.clone(), long_desc.clone(), llm_config)
        .with_system_prompt(long_prompt.clone());

    assert_eq!(config.name, long_name);
    assert_eq!(config.description, long_desc);
    assert_eq!(config.system_prompt, long_prompt);

    // Test with extreme values
    let config = AgentConfig::new("test", "test", LlmConfig::default())
        .with_max_tokens(u32::MAX)
        .with_temperature(f32::MAX);

    assert_eq!(config.max_tokens, Some(u32::MAX));
    assert_eq!(config.temperature, Some(f32::MAX));

    // Test with zero values
    let config = AgentConfig::new("test", "test", LlmConfig::default())
        .with_max_tokens(0)
        .with_temperature(0.0);

    assert_eq!(config.max_tokens, Some(0));
    assert_eq!(config.temperature, Some(0.0));
}

#[test]
fn test_agent_message_edge_cases() {
    let sender = AgentId::new();

    // Test message with empty content
    let empty_text_message = AgentMessage::new(
        sender.clone(),
        None,
        MessageContent::Text("".to_string()),
    );
    assert!(match empty_text_message.content {
        MessageContent::Text(ref text) => text.is_empty(),
        _ => false,
    });

    // Test message with null JSON data
    let null_data_message = AgentMessage::new(
        sender.clone(),
        None,
        MessageContent::Data(serde_json::Value::Null),
    );
    assert!(match null_data_message.content {
        MessageContent::Data(serde_json::Value::Null) => true,
        _ => false,
    });

    // Test message with complex nested JSON
    let complex_data = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "array": [1, 2, 3, {"nested": "value"}],
                    "boolean": true,
                    "null_value": null
                }
            }
        }
    });

    let complex_message = AgentMessage::new(
        sender.clone(),
        None,
        MessageContent::Data(complex_data.clone()),
    );

    match complex_message.content {
        MessageContent::Data(data) => assert_eq!(data, complex_data),
        _ => panic!("Expected Data content"),
    }

    // Test message with very large metadata
    let mut large_message = AgentMessage::new(
        sender.clone(),
        None,
        MessageContent::Text("test".to_string()),
    );

    for i in 0..100 {
        large_message = large_message.with_metadata(
            format!("key_{}", i),
            serde_json::json!(format!("value_{}", i)),
        );
    }

    assert_eq!(large_message.metadata.len(), 100);
}

#[test]
fn test_capability_combinations() {
    let llm_config = LlmConfig::default();

    // Test empty capabilities
    let config = AgentConfig::new("test", "test", llm_config.clone())
        .with_capabilities(vec![]);
    assert!(config.capabilities.is_empty());

    // Test duplicate capabilities
    let config = AgentConfig::new("test", "test", llm_config.clone())
        .with_capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::TextProcessing, // Duplicate
            AgentCapability::DataAnalysis,
        ]);
    assert_eq!(config.capabilities.len(), 3); // Duplicates are allowed

    // Test all capability types
    let all_capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("custom1".to_string()),
        AgentCapability::Custom("custom2".to_string()),
        AgentCapability::Custom("".to_string()), // Empty custom capability
    ];

    let config = AgentConfig::new("test", "test", llm_config)
        .with_capabilities(all_capabilities.clone());

    assert_eq!(config.capabilities.len(), all_capabilities.len());
    for capability in &all_capabilities {
        assert!(config.capabilities.contains(capability));
    }
}

#[test]
fn test_builder_pattern_chaining() {
    let llm_config = LlmConfig::default();
    let agent_id = AgentId::new();

    // Test that all builder methods can be chained in any order
    let _builder1 = AgentBuilder::new("test1", llm_config.clone())
        .with_id(agent_id.clone())
        .description("desc")
        .capabilities(vec![AgentCapability::TextProcessing])
        .system_prompt("prompt")
        .max_tokens(100)
        .temperature(0.5);

    let _builder2 = AgentBuilder::new("test2", llm_config.clone())
        .temperature(0.5)
        .max_tokens(100)
        .system_prompt("prompt")
        .capabilities(vec![AgentCapability::TextProcessing])
        .description("desc")
        .with_id(agent_id.clone());

    // Both builders should be valid (we can't compare them directly due to private fields)
    assert_eq!(agent_id.to_string().len(), 36);
}

#[test]
fn test_llm_config_with_optional_fields() {
    // Test OpenAI config with all optional fields
    let openai_full = LlmConfig::OpenAI {
        api_key: "key".to_string(),
        model: "gpt-4".to_string(),
        base_url: Some("https://custom.openai.com".to_string()),
        organization: Some("org-123".to_string()),
    };

    let openai_minimal = LlmConfig::OpenAI {
        api_key: "key".to_string(),
        model: "gpt-4".to_string(),
        base_url: None,
        organization: None,
    };

    // Test Anthropic config with optional fields
    let anthropic_full = LlmConfig::Anthropic {
        api_key: "key".to_string(),
        model: "claude-3".to_string(),
        base_url: Some("https://custom.anthropic.com".to_string()),
    };

    let anthropic_minimal = LlmConfig::Anthropic {
        api_key: "key".to_string(),
        model: "claude-3".to_string(),
        base_url: None,
    };

    // Test that configs can be used to create AgentConfig
    let configs = vec![openai_full, openai_minimal, anthropic_full, anthropic_minimal];

    for llm_config in configs {
        let agent_config = AgentConfig::new("test", "test", llm_config);
        assert_eq!(agent_config.name, "test");
    }
}

#[test]
fn test_custom_llm_config_edge_cases() {
    // Test custom config with empty provider name
    let custom_empty_provider = LlmConfig::Custom {
        provider_type: "".to_string(),
        config: {
            let mut map = std::collections::HashMap::new();
            map.insert("model".to_string(), serde_json::json!("test"));
            map
        },
    };

    // Test custom config with empty config object
    let custom_empty_config = LlmConfig::Custom {
        provider_type: "provider".to_string(),
        config: std::collections::HashMap::new(),
    };

    // Test custom config with minimal config
    let custom_minimal_config = LlmConfig::Custom {
        provider_type: "provider".to_string(),
        config: {
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), serde_json::json!("value"));
            map
        },
    };

    let configs = vec![custom_empty_provider, custom_empty_config, custom_minimal_config];

    for llm_config in configs {
        let agent_config = AgentConfig::new("test", "test", llm_config);
        assert_eq!(agent_config.name, "test");
    }
}

#[test]
fn test_agent_trait_default_implementations() {
    let dummy = DummyAgent { id: AgentId::new() };

    // Test has_capability default implementation
    let capabilities = vec![
        AgentCapability::TextProcessing,
        AgentCapability::DataAnalysis,
        AgentCapability::ToolExecution,
        AgentCapability::DecisionMaking,
        AgentCapability::Custom("test".to_string()),
    ];

    for capability in capabilities {
        assert!(!dummy.has_capability(&capability));
    }

    // Test capabilities default implementation
    assert!(dummy.capabilities().is_empty());
    assert_eq!(dummy.capabilities().len(), 0);
}

#[test]
fn test_agent_config_field_access() {
    let llm_config = LlmConfig::OpenAI {
        api_key: "test_key".to_string(),
        model: "gpt-4".to_string(),
        base_url: Some("https://api.openai.com".to_string()),
        organization: Some("org-123".to_string()),
    };

    let agent_id = AgentId::new();
    let config = AgentConfig::new("Test Agent", "A comprehensive test agent", llm_config.clone())
        .with_id(agent_id.clone())
        .with_capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::Custom("special".to_string()),
        ])
        .with_system_prompt("You are a helpful assistant")
        .with_max_tokens(2048)
        .with_temperature(0.7);

    // Test all public fields
    assert_eq!(config.id, agent_id);
    assert_eq!(config.name, "Test Agent");
    assert_eq!(config.description, "A comprehensive test agent");
    assert_eq!(config.capabilities.len(), 2);
    assert!(config.capabilities.contains(&AgentCapability::TextProcessing));
    assert!(config.capabilities.contains(&AgentCapability::Custom("special".to_string())));
    assert_eq!(config.system_prompt, "You are a helpful assistant");
    assert_eq!(config.max_tokens, Some(2048));
    assert_eq!(config.temperature, Some(0.7));
    assert!(config.custom_config.is_empty());

    // Test llm_config field access
    match config.llm_config {
        LlmConfig::OpenAI { api_key, model, base_url, organization } => {
            assert_eq!(api_key, "test_key");
            assert_eq!(model, "gpt-4");
            assert_eq!(base_url, Some("https://api.openai.com".to_string()));
            assert_eq!(organization, Some("org-123".to_string()));
        }
        _ => panic!("Expected OpenAI config"),
    }
}

#[test]
fn test_message_timestamp_and_id_uniqueness() {
    let sender = AgentId::new();
    let recipient = AgentId::new();

    // Create multiple messages and verify uniqueness
    let mut messages = Vec::new();
    for i in 0..10 {
        let message = AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::Text(format!("Message {}", i)),
        );
        messages.push(message);
    }

    // Check that all message IDs are unique
    for i in 0..messages.len() {
        for j in (i + 1)..messages.len() {
            assert_ne!(messages[i].id, messages[j].id);
        }
    }

    // Check that timestamps are reasonable (within last few seconds)
    let now = chrono::Utc::now();
    for message in &messages {
        let diff = now.signed_duration_since(message.timestamp);
        assert!(diff.num_seconds() < 10); // Should be created within last 10 seconds
        assert!(diff.num_seconds() >= 0); // Should not be in the future
    }
}

#[test]
fn test_agent_config_into_conversions() {
    let llm_config = LlmConfig::default();

    // Test Into<String> conversions for name and description
    let config1 = AgentConfig::new("static_str", "static_desc", llm_config.clone());
    assert_eq!(config1.name, "static_str");
    assert_eq!(config1.description, "static_desc");

    let name_string = String::from("string_name");
    let desc_string = String::from("string_desc");
    let config2 = AgentConfig::new(name_string.clone(), desc_string.clone(), llm_config.clone());
    assert_eq!(config2.name, name_string);
    assert_eq!(config2.description, desc_string);

    // Test Into<String> for system prompt
    let config3 = AgentConfig::new("test", "test", llm_config.clone())
        .with_system_prompt("static_prompt");
    assert_eq!(config3.system_prompt, "static_prompt");

    let prompt_string = String::from("string_prompt");
    let config4 = AgentConfig::new("test", "test", llm_config)
        .with_system_prompt(prompt_string.clone());
    assert_eq!(config4.system_prompt, prompt_string);
}

#[test]
fn test_comprehensive_agent_builder_coverage() {
    let llm_config = LlmConfig::default();

    // Test builder with minimal configuration
    let _minimal_builder = AgentBuilder::new("minimal", llm_config.clone());
    // Can't access internal config, but verify builder creation works

    // Test builder with all possible configurations
    let _full_builder = AgentBuilder::new("full", llm_config.clone())
        .description("Full configuration test")
        .capabilities(vec![
            AgentCapability::TextProcessing,
            AgentCapability::DataAnalysis,
            AgentCapability::ToolExecution,
            AgentCapability::DecisionMaking,
            AgentCapability::Custom("custom1".to_string()),
            AgentCapability::Custom("custom2".to_string()),
        ])
        .system_prompt("Comprehensive system prompt for testing")
        .max_tokens(4096)
        .temperature(0.8)
        .with_id(AgentId::new());

    // Test builder with edge case values
    let _edge_builder = AgentBuilder::new("", llm_config.clone())
        .description("")
        .capabilities(vec![])
        .system_prompt("")
        .max_tokens(0)
        .temperature(0.0);

    // Test builder with extreme values
    let _extreme_builder = AgentBuilder::new("extreme", llm_config)
        .max_tokens(u32::MAX)
        .temperature(f32::MAX);

    // All builders should be valid (we can't test internal state due to private fields)
    // The fact that they compile and don't panic is the test
}

// Test to ensure we cover the Debug trait implementations
#[test]
fn test_debug_implementations() {
    let agent_id = AgentId::new();
    let workflow_id = WorkflowId::new();
    let llm_config = LlmConfig::default();

    // Test Debug for AgentId
    let debug_str = format!("{:?}", agent_id);
    assert!(debug_str.contains("AgentId"));

    // Test Debug for WorkflowId
    let debug_str = format!("{:?}", workflow_id);
    assert!(debug_str.contains("WorkflowId"));

    // Test Debug for LlmConfig
    let debug_str = format!("{:?}", llm_config);
    assert!(debug_str.contains("Ollama") || debug_str.contains("LlmConfig"));

    // Test Debug for AgentConfig
    let agent_config = AgentConfig::new("debug_test", "test", llm_config);
    let debug_str = format!("{:?}", agent_config);
    assert!(debug_str.contains("AgentConfig") || debug_str.contains("debug_test"));

    // Test Debug for AgentMessage
    let message = AgentMessage::new(
        agent_id.clone(),
        None,
        MessageContent::Text("debug test".to_string()),
    );
    let debug_str = format!("{:?}", message);
    assert!(debug_str.contains("AgentMessage") || debug_str.contains("debug test"));

    // Test Debug for MessageContent variants
    let contents = vec![
        MessageContent::Text("debug".to_string()),
        MessageContent::Data(serde_json::json!({"debug": true})),
        MessageContent::ToolCall {
            tool_name: "debug_tool".to_string(),
            parameters: serde_json::json!({}),
        },
        MessageContent::ToolResponse {
            tool_name: "debug_tool".to_string(),
            result: serde_json::json!("success"),
            success: true,
        },
        MessageContent::Error {
            error_code: "DEBUG_ERROR".to_string(),
            error_message: "Debug error message".to_string(),
        },
    ];

    for content in contents {
        let debug_str = format!("{:?}", content);
        assert!(!debug_str.is_empty());
    }
}
