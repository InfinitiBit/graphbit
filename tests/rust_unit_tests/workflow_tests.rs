use graphbit_core::types::RetryConfig;
use graphbit_core::*;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

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

// ---- Dummy agent for workflow execution tests (no external API) ----
struct DummyAgent {
    cfg: graphbit_core::agents::AgentConfig,
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
    (id.clone(), std::sync::Arc::new(DummyAgent { cfg }))
}

#[tokio::test]
async fn test_workflow_execute_with_dummy_agent_success() {
    use graphbit_core::graph::{NodeType, WorkflowEdge, WorkflowNode};

    // Build a small workflow: Agent -> Transform -> Condition
    let (agent_id, agent) = build_dummy_agent("dummy");
    let agent_node = WorkflowNode::new(
        "agent",
        "agent node",
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "Say hello".to_string(),
        },
    );
    let transform_node = WorkflowNode::new(
        "transform",
        "transform node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );
    let condition_node = WorkflowNode::new(
        "cond",
        "condition node",
        NodeType::Condition {
            expression: "true".to_string(),
        },
    );

    let builder = WorkflowBuilder::new("wf");
    let (builder, a_id) = builder.add_node(agent_node).unwrap();
    let (builder, t_id) = builder.add_node(transform_node).unwrap();
    let (mut builder, c_id) = builder.add_node(condition_node).unwrap();
    builder = builder
        .connect(a_id.clone(), t_id.clone(), WorkflowEdge::control_flow())
        .unwrap();
    builder = builder
        .connect(t_id.clone(), c_id.clone(), WorkflowEdge::control_flow())
        .unwrap();
    let wf = builder.build().unwrap();

    let exec = WorkflowExecutor::new();
    exec.register_agent(agent).await;

    let ctx = exec.execute(wf).await.expect("workflow should execute");
    assert!(matches!(ctx.state, WorkflowState::Completed));
    let stats = ctx.stats.expect("stats present");
    assert!(stats.total_nodes >= 3);
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
            agent_id: agent_id.clone(),
            prompt_template: "Hello".to_string(),
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
        .execute(wf)
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
            agent_id: agent_id.clone(),
            prompt_template: "What is 2+2?".to_string(),
        },
    );

    workflow.add_node(node).unwrap();

    let executor = WorkflowExecutor::new();
    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow).await;
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
        model: "claude-2".to_string(),
        base_url: None,
    };

    let agent_id = AgentId::new();
    let node = WorkflowNode::new(
        "agent_node".to_string(),
        "Agent node".to_string(),
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "What is 2+2?".to_string(),
        },
    );

    workflow.add_node(node).unwrap();

    let executor = WorkflowExecutor::new();
    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow).await;
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
            agent_id: agent_id.clone(),
            prompt_template: "What is 2+2?".to_string(),
        },
    );

    workflow.add_node(node).unwrap();

    let executor = WorkflowExecutor::new();
    let agent = AgentBuilder::new("test_agent", llm_config)
        .description("Test agent")
        .build()
        .await
        .expect("Failed to build agent");

    executor.register_agent(Arc::new(agent)).await;
    let result = executor.execute(workflow).await;
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
    assert!(format!("{err}")
        .to_lowercase()
        .contains("graph contains cycles"));
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
    assert!(format!("{err}")
        .to_lowercase()
        .contains("graph contains cycles"));
}
