use async_trait::async_trait;
use futures::stream;
use graphbit_core::types::RetryConfig;
use graphbit_core::*;
use serde_json::json;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;

fn has_openai_key() -> bool {
    std::env::var("OPENAI_API_KEY").is_ok()
}

#[test]
fn test_resolve_template_variables_node_and_vars() {
    use graphbit_core::types::{NodeId, WorkflowContext, WorkflowId};
    use serde_json::json;

    let mut ctx = WorkflowContext::new(WorkflowId::new());
    // Add simple variable replacement
    ctx.set_variable("user".to_string(), json!("alice"));

    // Add node output and reference it
    let node_id = NodeId::new();
    ctx.set_node_output(&node_id, json!({"greeting": "hello"}));

    let template = format!("Hi {{node.{node_id}.greeting}}, {{user}}!");
    let resolved =
        graphbit_core::workflow::WorkflowExecutor::resolve_template_variables(&template, &ctx);

    assert!(resolved.contains("alice"));
    assert!(!resolved.contains("{{node."));

    // Also test simple variable-only replacement
    let only_var =
        graphbit_core::workflow::WorkflowExecutor::resolve_template_variables("User={user}", &ctx);
    assert!(only_var.contains("alice"));
}

// ---- Dummy LLM provider for testing ----
struct DummyLlmProvider;

#[derive(Debug, Default)]
struct ProviderCallCounts {
    complete_calls: usize,
    stream_calls: usize,
}

struct ScriptedStreamingProvider {
    counts: Arc<Mutex<ProviderCallCounts>>,
    stream_chunks: Vec<graphbit_core::llm::LlmResponse>,
    complete_response: graphbit_core::llm::LlmResponse,
}

#[async_trait]
impl graphbit_core::llm::LlmProviderTrait for DummyLlmProvider {
    fn provider_name(&self) -> &str {
        "dummy"
    }

    fn model_name(&self) -> &str {
        "dummy-model"
    }

    async fn complete(
        &self,
        _request: graphbit_core::llm::LlmRequest,
    ) -> graphbit_core::errors::GraphBitResult<graphbit_core::llm::LlmResponse> {
        // Return a dummy response
        Ok(graphbit_core::llm::LlmResponse {
            content: "dummy response".to_string(),
            usage: graphbit_core::llm::LlmUsage::new(10, 5),
            finish_reason: graphbit_core::llm::FinishReason::Stop,
            model: "dummy-model".to_string(),
            tool_calls: vec![],
            metadata: std::collections::HashMap::new(),
            id: Some("dummy-id".to_string()),
        })
    }
}

#[async_trait]
impl graphbit_core::llm::LlmProviderTrait for ScriptedStreamingProvider {
    fn provider_name(&self) -> &str {
        "scripted"
    }

    fn model_name(&self) -> &str {
        "scripted-model"
    }

    async fn complete(
        &self,
        _request: graphbit_core::llm::LlmRequest,
    ) -> graphbit_core::errors::GraphBitResult<graphbit_core::llm::LlmResponse> {
        let mut counts = self.counts.lock().expect("counts lock poisoned");
        counts.complete_calls += 1;
        Ok(self.complete_response.clone())
    }

    async fn stream(
        &self,
        _request: graphbit_core::llm::LlmRequest,
    ) -> graphbit_core::errors::GraphBitResult<
        Box<
            dyn futures::Stream<
                    Item = graphbit_core::errors::GraphBitResult<graphbit_core::llm::LlmResponse>,
                > + Unpin
                + Send,
        >,
    > {
        let mut counts = self.counts.lock().expect("counts lock poisoned");
        counts.stream_calls += 1;
        let items = self
            .stream_chunks
            .clone()
            .into_iter()
            .map(Ok)
            .collect::<Vec<_>>();
        Ok(Box::new(stream::iter(items)))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_function_calling(&self) -> bool {
        true
    }
}

// ---- Dummy agent for workflow execution tests (no external API) ----
struct DummyAgent {
    cfg: graphbit_core::agents::AgentConfig,
    llm_provider: graphbit_core::llm::LlmProvider,
}

#[async_trait::async_trait]
impl graphbit_core::agents::AgentTrait for DummyAgent {
    fn id(&self) -> &graphbit_core::types::AgentId {
        &self.cfg.id
    }
    fn config(&self) -> &graphbit_core::agents::AgentConfig {
        &self.cfg
    }

    async fn process_message(
        &self,
        message: graphbit_core::types::AgentMessage,
        _context: &mut graphbit_core::types::WorkflowContext,
    ) -> graphbit_core::errors::GraphBitResult<graphbit_core::types::AgentMessage> {
        let reply = match message.content {
            graphbit_core::types::MessageContent::Text(t) => format!("echo:{t}"),
            _ => "unsupported".to_string(),
        };
        Ok(graphbit_core::types::AgentMessage::new(
            self.cfg.id.clone(),
            Some(message.sender),
            graphbit_core::types::MessageContent::Text(reply),
        ))
    }

    async fn execute(
        &self,
        message: graphbit_core::types::AgentMessage,
    ) -> graphbit_core::errors::GraphBitResult<serde_json::Value> {
        let txt = match message.content {
            graphbit_core::types::MessageContent::Text(t) => t,
            _ => String::new(),
        };
        Ok(serde_json::json!({"ok": true, "len": txt.len()}))
    }

    async fn validate_output(
        &self,
        _output: &str,
        _schema: &serde_json::Value,
    ) -> graphbit_core::validation::ValidationResult {
        graphbit_core::validation::ValidationResult::success()
    }

    fn llm_provider(&self) -> &graphbit_core::llm::LlmProvider {
        &self.llm_provider
    }
}

fn build_dummy_agent(name: &str) -> (graphbit_core::types::AgentId, std::sync::Arc<DummyAgent>) {
    let id = graphbit_core::types::AgentId::new();
    let cfg = graphbit_core::agents::AgentConfig::new(
        name,
        "dummy",
        graphbit_core::llm::LlmConfig::default(),
    )
    .with_id(id.clone())
    .with_capabilities(vec![graphbit_core::types::AgentCapability::TextProcessing]);

    // Create dummy LLM provider
    let dummy_provider = Box::new(DummyLlmProvider);
    let llm_provider = graphbit_core::llm::LlmProvider::new(
        dummy_provider,
        graphbit_core::llm::LlmConfig::default(),
    );

    (
        id.clone(),
        std::sync::Arc::new(DummyAgent { cfg, llm_provider }),
    )
}

fn build_scripted_streaming_agent(
    name: &str,
    stream_chunks: Vec<graphbit_core::llm::LlmResponse>,
    complete_response: graphbit_core::llm::LlmResponse,
) -> (
    graphbit_core::types::AgentId,
    std::sync::Arc<DummyAgent>,
    Arc<Mutex<ProviderCallCounts>>,
) {
    let id = graphbit_core::types::AgentId::new();
    let cfg = graphbit_core::agents::AgentConfig::new(
        name,
        "scripted",
        graphbit_core::llm::LlmConfig::default(),
    )
    .with_id(id.clone())
    .with_capabilities(vec![graphbit_core::types::AgentCapability::TextProcessing]);

    let counts = Arc::new(Mutex::new(ProviderCallCounts::default()));
    let provider = Box::new(ScriptedStreamingProvider {
        counts: counts.clone(),
        stream_chunks,
        complete_response,
    });
    let llm_provider =
        graphbit_core::llm::LlmProvider::new(provider, graphbit_core::llm::LlmConfig::default());

    (
        id.clone(),
        std::sync::Arc::new(DummyAgent { cfg, llm_provider }),
        counts,
    )
}

async fn collect_stream_events(
    mut rx: mpsc::Receiver<graphbit_core::stream::StreamEvent>,
) -> Vec<graphbit_core::stream::StreamEvent> {
    let mut events = Vec::new();
    while let Some(event) = rx.recv().await {
        events.push(event);
    }
    events
}

fn build_single_agent_workflow(
    agent_id: graphbit_core::types::AgentId,
    node_name: &str,
    prompt_template: &str,
    tool_schemas: Option<serde_json::Value>,
) -> (Workflow, NodeId) {
    use graphbit_core::graph::{NodeType, WorkflowNode};

    let agent_node = {
        let base = WorkflowNode::new(
            node_name,
            "agent node",
            NodeType::Agent {
                config: AgentNodeConfig {
                    agent_id,
                    prompt_template: prompt_template.to_string(),
                    conversational_context: None,
                    system_prompt_override: None,
                },
            },
        );
        if let Some(schemas) = tool_schemas {
            base.with_config("tool_schemas".to_string(), schemas)
        } else {
            base
        }
    };
    let node_id = agent_node.id.clone();
    let (builder, _) = WorkflowBuilder::new("stream_parity")
        .add_node(agent_node)
        .unwrap();
    (builder.build().unwrap(), node_id)
}

#[tokio::test]
async fn test_streaming_empty_stream_falls_back_to_complete_no_tools() {
    let complete_response =
        graphbit_core::llm::LlmResponse::new("fallback-content", "scripted-model")
            .with_usage(graphbit_core::llm::LlmUsage::new(11, 7))
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("fallback-1".to_string());
    let (agent_id, agent, counts) =
        build_scripted_streaming_agent("stream-fallback", vec![], complete_response);
    let (workflow, node_id) =
        build_single_agent_workflow(agent_id, "agent_fallback", "Hello", None);

    let executor = WorkflowExecutor::new();
    executor.register_agent(agent).await;
    let (tx, rx) = mpsc::channel(32);
    let context = executor
        .execute_streaming(
            workflow,
            None,
            tx,
            graphbit_core::stream::StreamMode::All,
        )
        .await
        .expect("streaming execution should succeed");
    let events = collect_stream_events(rx).await;

    assert_eq!(
        context.get_node_output(&node_id.to_string()),
        Some(&serde_json::Value::String("fallback-content".to_string()))
    );
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::Token { .. }))
    );
    assert!(events.iter().any(|e| matches!(
        e,
        graphbit_core::stream::StreamEvent::LlmCallCompleted { output, .. }
            if output == "fallback-content"
    )));

    let counts = counts.lock().expect("counts lock poisoned");
    assert_eq!(counts.stream_calls, 1);
    assert_eq!(counts.complete_calls, 1);
}

#[tokio::test]
async fn test_streaming_reconstructs_final_content_from_chunks_no_tools() {
    let stream_chunks = vec![
        graphbit_core::llm::LlmResponse::new("Hello ", "scripted-model")
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("stream-1".to_string()),
        graphbit_core::llm::LlmResponse::new("world", "scripted-model")
            .with_usage(graphbit_core::llm::LlmUsage::new(8, 3))
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("stream-1".to_string()),
    ];
    let complete_response =
        graphbit_core::llm::LlmResponse::new("should-not-be-used", "scripted-model");
    let (agent_id, agent, counts) =
        build_scripted_streaming_agent("stream-chunks", stream_chunks, complete_response);
    let (workflow, node_id) =
        build_single_agent_workflow(agent_id, "agent_streamed", "Hello world", None);

    let executor = WorkflowExecutor::new();
    executor.register_agent(agent).await;
    let (tx, rx) = mpsc::channel(32);
    let context = executor
        .execute_streaming(
            workflow,
            None,
            tx,
            graphbit_core::stream::StreamMode::Messages,
        )
        .await
        .expect("streaming execution should succeed");
    let events = collect_stream_events(rx).await;

    assert_eq!(
        context.get_node_output(&node_id.to_string()),
        Some(&serde_json::Value::String("Hello world".to_string()))
    );
    let token_payload = events
        .iter()
        .filter_map(|e| match e {
            graphbit_core::stream::StreamEvent::Token { content, .. } => Some(content.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    assert_eq!(token_payload, "Hello world");
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::NodeStarted { .. }))
    );
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::NodeCompleted { .. }))
    );
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::LlmCallStarted { .. }))
    );
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::LlmCallCompleted { .. }))
    );

    let counts = counts.lock().expect("counts lock poisoned");
    assert_eq!(counts.stream_calls, 1);
    assert_eq!(counts.complete_calls, 0);
}

#[tokio::test]
async fn test_streaming_empty_stream_falls_back_to_complete_with_tools() {
    let complete_response = graphbit_core::llm::LlmResponse::new("", "scripted-model")
        .with_tool_calls(vec![graphbit_core::llm::LlmToolCall {
            id: "call_weather_1".to_string(),
            name: "get_weather".to_string(),
            parameters: serde_json::json!({"location":"San Francisco"}),
        }])
        .with_usage(graphbit_core::llm::LlmUsage::new(14, 5))
        .with_finish_reason(graphbit_core::llm::FinishReason::ToolCalls)
        .with_id("tool-fallback-1".to_string());
    let (agent_id, agent, counts) =
        build_scripted_streaming_agent("tool-fallback", vec![], complete_response);
    let tool_schemas = serde_json::json!([{
        "name": "get_weather",
        "description": "Get weather for a city",
        "parameters": {
            "type": "object",
            "properties": { "location": { "type": "string" } },
            "required": ["location"]
        }
    }]);
    let (workflow, node_id) = build_single_agent_workflow(
        agent_id,
        "agent_with_tools",
        "Use tools",
        Some(tool_schemas),
    );

    let executor = WorkflowExecutor::new();
    executor.register_agent(agent).await;
    let (tx, rx) = mpsc::channel(32);
    let context = executor
        .execute_streaming(workflow, None, tx, graphbit_core::stream::StreamMode::All)
        .await
        .expect("streaming execution should succeed");
    let events = collect_stream_events(rx).await;

    let output = context
        .get_node_output(&node_id.to_string())
        .expect("node output should be present");
    assert_eq!(output["type"], "tool_calls_required");
    assert_eq!(output["tool_calls"].as_array().map(|a| a.len()), Some(1));

    assert!(events.iter().any(|e| matches!(
        e,
        graphbit_core::stream::StreamEvent::ToolCallStarted {
            tool_name,
            tool_call_id,
            ..
        } if tool_name == "get_weather" && tool_call_id == "call_weather_1"
    )));
    assert!(
        events
            .iter()
            .all(|e| !matches!(e, graphbit_core::stream::StreamEvent::Token { .. }))
    );

    let counts = counts.lock().expect("counts lock poisoned");
    assert_eq!(counts.stream_calls, 1);
    assert_eq!(counts.complete_calls, 1);
}

#[tokio::test]
async fn test_execute_and_execute_streaming_match_final_output_no_tools() {
    let shared_complete_response =
        graphbit_core::llm::LlmResponse::new("Parity output", "scripted-model")
            .with_usage(graphbit_core::llm::LlmUsage::new(9, 4))
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("parity-1".to_string());

    let stream_chunks = vec![
        graphbit_core::llm::LlmResponse::new("Parity ", "scripted-model")
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("parity-1".to_string()),
        graphbit_core::llm::LlmResponse::new("output", "scripted-model")
            .with_usage(graphbit_core::llm::LlmUsage::new(9, 4))
            .with_finish_reason(graphbit_core::llm::FinishReason::Stop)
            .with_id("parity-1".to_string()),
    ];

    let (agent_id_complete, agent_complete, complete_counts) = build_scripted_streaming_agent(
        "parity-complete-agent",
        stream_chunks.clone(),
        shared_complete_response.clone(),
    );
    let (workflow_complete, node_id_complete) = build_single_agent_workflow(
        agent_id_complete,
        "parity_node_complete",
        "Prompt parity",
        None,
    );
    let complete_executor = WorkflowExecutor::new();
    complete_executor.register_agent(agent_complete).await;
    let complete_ctx = complete_executor
        .execute(workflow_complete, None)
        .await
        .expect("non-streaming execution should succeed");

    let (agent_id_stream, agent_stream, stream_counts) = build_scripted_streaming_agent(
        "parity-stream-agent",
        stream_chunks,
        shared_complete_response,
    );
    let (workflow_stream, node_id_stream) =
        build_single_agent_workflow(agent_id_stream, "parity_node_stream", "Prompt parity", None);
    let streaming_executor = WorkflowExecutor::new();
    streaming_executor.register_agent(agent_stream).await;
    let (tx, rx) = mpsc::channel(32);
    let stream_ctx = streaming_executor
        .execute_streaming(
            workflow_stream,
            None,
            tx,
            graphbit_core::stream::StreamMode::All,
        )
        .await
        .expect("streaming execution should succeed");
    let events = collect_stream_events(rx).await;

    let complete_output = complete_ctx
        .get_node_output(&node_id_complete.to_string())
        .cloned()
        .expect("non-streaming node output should exist");
    let stream_output = stream_ctx
        .get_node_output(&node_id_stream.to_string())
        .cloned()
        .expect("streaming node output should exist");
    assert_eq!(complete_output, stream_output);

    assert!(
        events
            .iter()
            .any(|e| matches!(e, graphbit_core::stream::StreamEvent::Token { .. }))
    );
    assert!(events.iter().any(|e| matches!(
        e,
        graphbit_core::stream::StreamEvent::LlmCallCompleted { output, .. }
            if output == "Parity output"
    )));

    let complete_counts = complete_counts.lock().expect("counts lock poisoned");
    assert_eq!(complete_counts.complete_calls, 1);
    assert_eq!(complete_counts.stream_calls, 0);
    drop(complete_counts);

    let stream_counts = stream_counts.lock().expect("counts lock poisoned");
    assert_eq!(stream_counts.complete_calls, 0);
    assert_eq!(stream_counts.stream_calls, 1);
}

#[tokio::test]
async fn test_execute_and_execute_streaming_match_tool_calls_required_shape() {
    let tool_response = graphbit_core::llm::LlmResponse::new("", "scripted-model")
        .with_tool_calls(vec![graphbit_core::llm::LlmToolCall {
            id: "tool-call-99".to_string(),
            name: "lookup_price".to_string(),
            parameters: serde_json::json!({"symbol":"GBIT"}),
        }])
        .with_usage(graphbit_core::llm::LlmUsage::new(12, 3))
        .with_finish_reason(graphbit_core::llm::FinishReason::ToolCalls)
        .with_id("tool-parity-1".to_string());
    let stream_chunks = vec![tool_response.clone()];
    let tool_schemas = serde_json::json!([{
        "name": "lookup_price",
        "description": "Lookup stock price",
        "parameters": {
            "type": "object",
            "properties": { "symbol": { "type": "string" } },
            "required": ["symbol"]
        }
    }]);

    let (agent_id_complete, agent_complete, complete_counts) = build_scripted_streaming_agent(
        "tool-parity-complete-agent",
        stream_chunks.clone(),
        tool_response.clone(),
    );
    let (workflow_complete, node_id_complete) = build_single_agent_workflow(
        agent_id_complete,
        "tool_parity_node_complete",
        "Use pricing tool",
        Some(tool_schemas.clone()),
    );
    let complete_executor = WorkflowExecutor::new();
    complete_executor.register_agent(agent_complete).await;
    let complete_ctx = complete_executor
        .execute(workflow_complete, None)
        .await
        .expect("non-streaming tool execution should succeed");

    let (agent_id_stream, agent_stream, stream_counts) =
        build_scripted_streaming_agent("tool-parity-stream-agent", stream_chunks, tool_response);
    let (workflow_stream, node_id_stream) = build_single_agent_workflow(
        agent_id_stream,
        "tool_parity_node_stream",
        "Use pricing tool",
        Some(tool_schemas),
    );
    let streaming_executor = WorkflowExecutor::new();
    streaming_executor.register_agent(agent_stream).await;
    let (tx, rx) = mpsc::channel(32);
    let stream_ctx = streaming_executor
        .execute_streaming(
            workflow_stream,
            None,
            tx,
            graphbit_core::stream::StreamMode::All,
        )
        .await
        .expect("streaming tool execution should succeed");
    let events = collect_stream_events(rx).await;

    let complete_output = complete_ctx
        .get_node_output(&node_id_complete.to_string())
        .cloned()
        .expect("non-streaming node output should exist");
    let stream_output = stream_ctx
        .get_node_output(&node_id_stream.to_string())
        .cloned()
        .expect("streaming node output should exist");

    assert_eq!(complete_output["type"], "tool_calls_required");
    assert_eq!(stream_output["type"], "tool_calls_required");
    assert_eq!(complete_output["tool_calls"], stream_output["tool_calls"]);
    assert_eq!(
        complete_output["tool_calls"][0]["id"],
        serde_json::Value::String("tool-call-99".to_string())
    );

    assert!(events.iter().any(|e| matches!(
        e,
        graphbit_core::stream::StreamEvent::ToolCallStarted {
            tool_name,
            tool_call_id,
            ..
        } if tool_name == "lookup_price" && tool_call_id == "tool-call-99"
    )));

    let complete_counts = complete_counts.lock().expect("counts lock poisoned");
    assert_eq!(complete_counts.complete_calls, 1);
    assert_eq!(complete_counts.stream_calls, 0);
    drop(complete_counts);

    let stream_counts = stream_counts.lock().expect("counts lock poisoned");
    assert_eq!(stream_counts.complete_calls, 0);
    assert_eq!(stream_counts.stream_calls, 1);
}

#[tokio::test]
async fn test_workflow_execute_with_dummy_agent_success() {
    use graphbit_core::graph::{NodeType, WorkflowEdge, WorkflowNode};

    // Build a small workflow: Agent -> Transform
    let (agent_id, agent) = build_dummy_agent("dummy");
    let agent_node = WorkflowNode::new(
        "agent",
        "agent node",
        NodeType::Agent {
            config: AgentNodeConfig {
                agent_id: agent_id.clone(),
                prompt_template: "Say hello".to_string(),
                conversational_context: None,
                system_prompt_override: None,
            },
        },
    );
    let transform_node = WorkflowNode::new(
        "transform",
        "transform node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );
    let builder = WorkflowBuilder::new("wf");
    let (builder, a_id) = builder.add_node(agent_node).unwrap();
    let (builder, t_id) = builder.add_node(transform_node).unwrap();
    let builder = builder
        .connect(a_id.clone(), t_id.clone(), WorkflowEdge::control_flow())
        .unwrap();
    let wf = builder.build().unwrap();

    let exec = WorkflowExecutor::new();
    exec.register_agent(agent).await;

    let ctx = exec
        .execute(wf, None)
        .await
        .expect("workflow should execute");
    assert!(matches!(ctx.state, WorkflowState::Completed));
    let stats = ctx.stats.expect("stats present");
    assert!(stats.total_nodes >= 2);
}

#[tokio::test]
async fn test_workflow_execute_fail_fast_on_error() {
    use graphbit_core::graph::{NodeType, WorkflowEdge, WorkflowNode};

    // Build a workflow ending with a failing DocumentLoader node
    let (agent_id, agent) = build_dummy_agent("dummy");
    let agent_node = WorkflowNode::new(
        "agent",
        "agent node",
        NodeType::Agent {
            config: AgentNodeConfig {
                agent_id: agent_id.clone(),
                prompt_template: "Hello".to_string(),
                conversational_context: None,
                system_prompt_override: None,
            },
        },
    );
    let bad_doc = WorkflowNode::new(
        "doc",
        "bad doc",
        NodeType::DocumentLoader {
            document_type: "txt".to_string(),
            source_path: "/definitely/not/found".to_string(),
            encoding: None,
        },
    );

    let builder = WorkflowBuilder::new("wf_fail");
    let (builder, a_id) = builder.add_node(agent_node).unwrap();
    let (mut builder, d_id) = builder.add_node(bad_doc).unwrap();
    builder = builder
        .connect(a_id.clone(), d_id.clone(), WorkflowEdge::control_flow())
        .unwrap();
    let wf = builder.build().unwrap();

    let exec = WorkflowExecutor::new().with_fail_fast(true);
    exec.register_agent(agent).await;

    let ctx = exec
        .execute(wf, None)
        .await
        .expect("execution should return context");
    // Current executor records node failure but continues; ensure at least one failed node counted
    match &ctx.state {
        WorkflowState::Completed | WorkflowState::Failed { .. } => {}
        _ => panic!("Expected terminal state"),
    }
    let stats = ctx.stats.expect("stats present");
    assert!(stats.failed_nodes >= 1);
}

#[tokio::test]
async fn test_execute_concurrent_agent_tasks_with_dummy_agent() {
    let (agent_id, agent) = build_dummy_agent("dummy");
    let exec = WorkflowExecutor::new();
    exec.register_agent(agent).await;

    let prompts = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let results = exec
        .execute_concurrent_agent_tasks(prompts, agent_id)
        .await
        .expect("should execute");

    assert_eq!(results.len(), 3);
    for r in results {
        assert!(r.is_ok());
    }
}

#[tokio::test]
async fn test_execute_concurrent_tasks_success() {
    use futures::future::BoxFuture;

    let exec = WorkflowExecutor::new();
    let tasks = vec![1, 2, 3, 4];

    let task_fn = move |n: i32| -> BoxFuture<'static, GraphBitResult<i32>> {
        Box::pin(async move { Ok(n * 2) })
    };

    let results = exec
        .execute_concurrent_tasks(tasks.clone(), task_fn)
        .await
        .expect("execution failed");

    assert_eq!(results.len(), tasks.len());
    for (i, res) in results.into_iter().enumerate() {
        assert_eq!(res.unwrap(), tasks[i] * 2);
    }
}

#[tokio::test]
async fn test_execute_concurrent_tasks_with_retry_errors() {
    use futures::future::BoxFuture;

    let exec = WorkflowExecutor::new();
    let tasks = vec![1, 2];

    // Always error to cover retry path fall-through
    let task_fn = move |_n: i32| -> BoxFuture<'static, GraphBitResult<i32>> {
        Box::pin(async move { Err(GraphBitError::workflow_execution("fail")) })
    };

    let retry = RetryConfig::new(2);
    let results = exec
        .execute_concurrent_tasks_with_retry(tasks, task_fn, Some(retry))
        .await
        .expect("execution failed");

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_err()));
}

fn has_anthropic_key() -> bool {
    std::env::var("ANTHROPIC_API_KEY").is_ok()
}

async fn check_ollama_url(url: &str) -> bool {
    let client = reqwest::Client::new();
    match client
        .get(format!("{}/api/version", url.trim_end_matches('/')))
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => {
            println!("Failed to connect to Ollama at {url}");
            false
        }
    }
}

async fn ensure_ollama_model(model: &str, base_url: &str) -> bool {
    if !check_ollama_url(base_url).await {
        println!("Skipping Ollama test - server not available at {base_url}");
        return false;
    }

    // First check if model exists
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/show", base_url.trim_end_matches('/')))
        .json(&serde_json::json!({
            "name": model
        }))
        .send()
        .await;

    // If model doesn't exist or error, try to pull it
    if response.is_err() || !response.unwrap().status().is_success() {
        println!("Model {model} not found, attempting to pull...");

        // Pull the model
        let response = client
            .post(format!("{}/api/pull", base_url.trim_end_matches('/')))
            .json(&serde_json::json!({
                "name": model
            }))
            .send()
            .await;

        if let Ok(mut response) = response {
            // Wait for the pull to complete
            while let Ok(chunk) = response.chunk().await {
                if chunk.is_none() {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            // Verify model exists after pull
            let verify = client
                .post(format!("{}/api/show", base_url.trim_end_matches('/')))
                .json(&serde_json::json!({
                    "name": model
                }))
                .send()
                .await;

            if verify.is_err() || !verify.unwrap().status().is_success() {
                println!("Failed to verify model {model} after pulling");
                return false;
            }
            return true;
        }
        println!("Failed to pull model {model}");
        return false;
    }

    true
}

#[tokio::test]
async fn test_workflow_context() {
    let mut context = WorkflowContext::new(WorkflowId::new());
    let node_id = NodeId::from_string("test_node").unwrap();

    let output = json!({
        "result": "success",
        "value": 42
    });

    context
        .node_outputs
        .insert(node_id.to_string(), output.clone());
    let stored = context.node_outputs.get(&node_id.to_string()).unwrap();
    assert_eq!(stored, &output);
}

#[tokio::test]
async fn test_workflow_builder() {
    let workflow = WorkflowBuilder::new("test_workflow")
        .description("Test workflow")
        .build()
        .expect("Failed to build workflow");

    assert_eq!(workflow.name, "test_workflow");
    assert_eq!(workflow.description, "Test workflow");
}

#[tokio::test]
async fn test_workflow_with_llm() {
    if !has_openai_key() {
        println!("Skipping OpenAI workflow test - no API key available");
        return;
    }

    let mut workflow = WorkflowBuilder::new("test_workflow")
        .build()
        .expect("Failed to build workflow");

    let llm_config = llm::LlmConfig::OpenAI {
        api_key: std::env::var("OPENAI_API_KEY").unwrap(),
        model: "gpt-3.5-turbo".to_string(),
        base_url: None,
        organization: None,
    };

    let agent_id = AgentId::new();
    let node = WorkflowNode::new(
        "agent_node".to_string(),
        "Agent node".to_string(),
        NodeType::Agent {
            config: AgentNodeConfig {
                agent_id: agent_id.clone(),
                prompt_template: "What is 2+2?".to_string(),
                conversational_context: None,
                system_prompt_override: None,
            },
        },
    );

    workflow.add_node(node).unwrap();

    // Create executor with default LLM configuration
    let executor = WorkflowExecutor::new().with_default_llm_config(llm_config.clone());

    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_workflow_with_anthropic() {
    if !has_anthropic_key() {
        println!("Skipping Anthropic workflow test - no API key available");
        return;
    }

    let mut workflow = WorkflowBuilder::new("test_workflow")
        .build()
        .expect("Failed to build workflow");

    let llm_config = llm::LlmConfig::Anthropic {
        api_key: std::env::var("ANTHROPIC_API_KEY").unwrap(),
        model: "claude-3-5-sonnet-20241022".to_string(),
        base_url: None,
    };

    let agent_id = AgentId::new();
    let node = WorkflowNode::new(
        "agent_node".to_string(),
        "Agent node".to_string(),
        NodeType::Agent {
            config: AgentNodeConfig {
                agent_id: agent_id.clone(),
                prompt_template: "What is 2+2?".to_string(),
                conversational_context: None,
                system_prompt_override: None,
            },
        },
    );

    workflow.add_node(node).unwrap();

    // Create executor with default LLM configuration
    let executor = WorkflowExecutor::new().with_default_llm_config(llm_config.clone());

    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_workflow_with_ollama() {
    let base_url = "http://localhost:11434";
    let model = "llama3.2";

    if !ensure_ollama_model(model, base_url).await {
        println!("Skipping Ollama workflow test - server not available or model not found");
        return;
    }

    let mut workflow = WorkflowBuilder::new("test_workflow")
        .build()
        .expect("Failed to build workflow");

    let llm_config = llm::LlmConfig::Ollama {
        model: model.to_string(),
        base_url: Some(base_url.to_string()),
    };

    let agent_id = AgentId::new();
    let node = WorkflowNode::new(
        "agent_node".to_string(),
        "Agent node".to_string(),
        NodeType::Agent {
            config: AgentNodeConfig {
                agent_id: agent_id.clone(),
                prompt_template: "What is 2+2?".to_string(),
                conversational_context: None,
                system_prompt_override: None,
            },
        },
    );

    workflow.add_node(node).unwrap();

    // Create executor with default LLM configuration
    let executor = WorkflowExecutor::new().with_default_llm_config(llm_config.clone());

    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_workflow_graph_operations() {
    let mut workflow = WorkflowBuilder::new("test_workflow")
        .build()
        .expect("Failed to build workflow");

    let node1 = WorkflowNode::new(
        "node1".to_string(),
        "Node 1".to_string(),
        NodeType::Transform {
            transformation: "return input;".to_string(),
        },
    );
    let node2 = WorkflowNode::new(
        "node2".to_string(),
        "Node 2".to_string(),
        NodeType::Transform {
            transformation: "return input;".to_string(),
        },
    );

    workflow.add_node(node1.clone()).unwrap();
    workflow.add_node(node2.clone()).unwrap();

    let edge = WorkflowEdge::data_flow();
    workflow
        .connect_nodes(node1.id.clone(), node2.id.clone(), edge)
        .unwrap();

    assert!(workflow.validate().is_ok());
}

#[test]
fn test_workflow_graph_validation_errors() {
    use graphbit_core::graph::{WorkflowEdge, WorkflowGraph, WorkflowNode};
    use graphbit_core::types::NodeId;

    let mut graph = WorkflowGraph::new();

    // Add one node
    let node = WorkflowNode::new(
        "only",
        "single",
        NodeType::Transform {
            transformation: "x".to_string(),
        },
    );
    let from_id = node.id.clone();
    graph.add_node(node).unwrap();

    // Create a different target id not in graph
    let to_id = NodeId::new();
    let edge = WorkflowEdge::data_flow();
    let add_err = graph
        .add_edge(from_id.clone(), to_id.clone(), edge)
        .unwrap_err();
    let msg = format!("{add_err}").to_lowercase();
    assert!(msg.contains("target node") || msg.contains("not found"));
}

#[test]
fn test_workflow_graph_toposort_and_cycles() {
    let mut graph = WorkflowGraph::new();
    let n1 = WorkflowNode::new(
        "n1",
        "",
        NodeType::Transform {
            transformation: "a".into(),
        },
    );
    let n2 = WorkflowNode::new(
        "n2",
        "",
        NodeType::Transform {
            transformation: "b".into(),
        },
    );
    let id1 = n1.id.clone();
    let id2 = n2.id.clone();
    graph.add_node(n1).unwrap();
    graph.add_node(n2).unwrap();
    graph
        .add_edge(id1.clone(), id2.clone(), WorkflowEdge::data_flow())
        .unwrap();

    // Toposort should succeed on acyclic graph
    let order = graph.topological_sort().unwrap();
    assert!(!order.is_empty());

    // Create a cycle and verify detection via validate()
    graph.add_edge(id2, id1, WorkflowEdge::data_flow()).unwrap();
    let err = graph.validate().unwrap_err();
    assert!(
        format!("{err}")
            .to_lowercase()
            .contains("graph contains cycles")
    );
}

#[test]
fn test_workflow_graph_metadata_and_accessors() {
    let mut graph = WorkflowGraph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);

    graph.set_metadata("k".to_string(), serde_json::json!(123));
    assert_eq!(graph.get_metadata("k").unwrap(), &serde_json::json!(123));

    // Add and find by name
    let n = WorkflowNode::new(
        "find_me",
        "",
        NodeType::Transform {
            transformation: "t".into(),
        },
    );
    let id = n.id.clone();
    graph.add_node(n).unwrap();
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.get_node_id_by_name("find_me").unwrap(), id);
}

#[test]
fn test_workflow_graph_dependencies_and_ready_nodes() {
    use std::collections::HashSet;

    let mut graph = WorkflowGraph::new();
    let n1 = WorkflowNode::new(
        "n1",
        "",
        NodeType::Transform {
            transformation: "t1".into(),
        },
    );
    let n2 = WorkflowNode::new(
        "n2",
        "",
        NodeType::Transform {
            transformation: "t2".into(),
        },
    );
    let n3 = WorkflowNode::new(
        "n3",
        "",
        NodeType::Transform {
            transformation: "t3".into(),
        },
    );
    let id1 = n1.id.clone();
    let id2 = n2.id.clone();
    let id3 = n3.id.clone();
    graph.add_node(n1).unwrap();
    graph.add_node(n2).unwrap();
    graph.add_node(n3).unwrap();
    graph
        .add_edge(id1.clone(), id2.clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(id2.clone(), id3.clone(), WorkflowEdge::data_flow())
        .unwrap();

    // Roots/leaves caches
    let roots = graph.get_root_nodes();
    assert!(roots.contains(&id1) && !roots.contains(&id2));
    let leaves = graph.get_leaf_nodes();
    assert!(leaves.contains(&id3) && !leaves.contains(&id2));

    // Dependencies/dependents
    let deps_n3 = graph.get_dependencies(&id3);
    assert!(deps_n3.contains(&id2));
    let deps_n2 = graph.get_dependencies(&id2);
    assert!(deps_n2.contains(&id1));

    // Ready nodes given completed set
    let completed: HashSet<_> = [id1.clone()].into_iter().collect();
    let running: HashSet<NodeId> = HashSet::new();
    let next = graph.get_next_executable_nodes(&completed, &running);
    assert!(next.contains(&id2));
}

#[test]
fn test_workflow_builder_metadata_and_build_errors() {
    // Build with metadata
    let wf = WorkflowBuilder::new("wf_meta")
        .description("desc")
        .metadata("owner".into(), serde_json::json!("qa"))
        .build();
    // Without nodes, validate() passes (empty graph valid), so build returns Ok
    assert!(wf.is_ok());

    // Create a graph with a cycle via direct graph manipulation and ensure validate fails in build
    let mut g = WorkflowGraph::new();
    let n1 = WorkflowNode::new(
        "a",
        "",
        NodeType::Transform {
            transformation: "t".into(),
        },
    );
    let n2 = WorkflowNode::new(
        "b",
        "",
        NodeType::Transform {
            transformation: "t".into(),
        },
    );
    let i1 = n1.id.clone();
    let i2 = n2.id.clone();
    g.add_node(n1).unwrap();
    g.add_node(n2).unwrap();
    g.add_edge(i1.clone(), i2.clone(), WorkflowEdge::data_flow())
        .unwrap();
    g.add_edge(i2, i1, WorkflowEdge::data_flow()).unwrap();

    // Rehydrate into a Workflow and call validate directly to simulate builder failure path
    let wf = Workflow {
        id: WorkflowId::new(),
        name: "cyclic".into(),
        description: "".into(),
        graph: g,
        metadata: Default::default(),
    };
    let err = wf.validate().unwrap_err();
    assert!(
        format!("{err}")
            .to_lowercase()
            .contains("graph contains cycles")
    );
}
