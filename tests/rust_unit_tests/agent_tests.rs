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
