use async_trait::async_trait;
use graphbit_core::{
    agents::{AgentConfig, AgentTrait},
    graph::{AgentNodeConfig, NodeType, WorkflowNode},
    llm::{
        FinishReason, LlmConfig, LlmProvider, LlmProviderTrait, LlmRequest, LlmResponse, LlmRole,
        LlmUsage,
    },
    types::{AgentId, AgentMessage, MessageContent, WorkflowContext},
    validation::ValidationResult,
    workflow::{Workflow, WorkflowExecutor},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock LLM Provider to capture requests
struct MockLlmProvider {
    captured_requests: Arc<Mutex<Vec<LlmRequest>>>,
}

#[async_trait]
impl LlmProviderTrait for MockLlmProvider {
    fn provider_name(&self) -> &str {
        "mock"
    }
    fn model_name(&self) -> &str {
        "mock-model"
    }
    async fn complete(&self, request: LlmRequest) -> graphbit_core::GraphBitResult<LlmResponse> {
        self.captured_requests.lock().unwrap().push(request);
        Ok(LlmResponse {
            content: "Mock response".to_string(),
            usage: LlmUsage::empty(),
            finish_reason: FinishReason::Stop,
            model: "mock-model".to_string(),
            tool_calls: vec![],
            metadata: HashMap::new(),
            id: None,
        })
    }
}

// Manual Agent implementation for testing
struct TestAgent {
    config: AgentConfig,
    provider: LlmProvider,
}

#[async_trait]
impl AgentTrait for TestAgent {
    fn id(&self) -> &AgentId {
        &self.config.id
    }
    fn config(&self) -> &AgentConfig {
        &self.config
    }
    fn llm_provider(&self) -> &LlmProvider {
        &self.provider
    }

    async fn process_message(
        &self,
        _m: AgentMessage,
        _c: &mut WorkflowContext,
    ) -> graphbit_core::GraphBitResult<AgentMessage> {
        Ok(AgentMessage::new(
            self.config.id.clone(),
            None,
            MessageContent::Text("Mock".to_string()),
        ))
    }

    async fn execute(
        &self,
        _message: AgentMessage,
    ) -> graphbit_core::GraphBitResult<serde_json::Value> {
        Ok(serde_json::json!({"mock": true}))
    }

    async fn validate_output(
        &self,
        _output: &str,
        _schema: &serde_json::Value,
    ) -> ValidationResult {
        ValidationResult::success()
    }
}

#[tokio::test]
async fn test_system_prompt_inheritance_and_override() {
    let captured_requests = Arc::new(Mutex::new(Vec::new()));

    let agent_id = AgentId::new();
    let llm_config = LlmConfig::default();
    let mock_provider_trait = MockLlmProvider {
        captured_requests: captured_requests.clone(),
    };

    let agent_config = AgentConfig::new("Test Agent", "desc", llm_config.clone())
        .with_id(agent_id.clone())
        .with_system_prompt("Agent Default System Prompt");

    let llm_provider = LlmProvider::new(Box::new(mock_provider_trait), llm_config);
    let agent = Arc::new(TestAgent {
        config: agent_config,
        provider: llm_provider,
    });

    let executor = WorkflowExecutor::new();
    executor.register_agent(agent).await;

    // SCENARIO 1: Node with NO override
    let node_no_override = WorkflowNode::new(
        "NoOverride",
        "desc",
        NodeType::Agent {
            config: AgentNodeConfig::new(agent_id.clone(), "User Prompt 1"),
        },
    );

    let mut workflow1 = Workflow::new("WF1", "desc");
    workflow1.add_node(node_no_override).unwrap();
    executor.execute(workflow1, None).await.unwrap();

    {
        let requests = captured_requests.lock().unwrap();
        let last_req = requests.last().expect("Should have captured a request");
        let system_msg = last_req
            .messages
            .iter()
            .find(|m| matches!(m.role, LlmRole::System));
        assert!(system_msg.is_some(), "System message missing in request 1");
        assert_eq!(system_msg.unwrap().content, "Agent Default System Prompt");
    }

    // SCENARIO 2: Node WITH override
    let node_with_override = WorkflowNode::new(
        "WithOverride",
        "desc",
        NodeType::Agent {
            config: AgentNodeConfig::new(agent_id.clone(), "User Prompt 2")
                .with_system_prompt_override("Node Specific System Prompt"),
        },
    );

    let mut workflow2 = Workflow::new("WF2", "desc");
    workflow2.add_node(node_with_override).unwrap();
    executor.execute(workflow2, None).await.unwrap();

    {
        let requests = captured_requests.lock().unwrap();
        let last_req = requests.last().expect("Should have captured a request");
        let system_msg = last_req
            .messages
            .iter()
            .find(|m| matches!(m.role, LlmRole::System));
        assert!(system_msg.is_some(), "System message missing in request 2");
        assert_eq!(system_msg.unwrap().content, "Node Specific System Prompt");
    }
}
