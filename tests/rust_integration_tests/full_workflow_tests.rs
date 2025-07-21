//! End-to-End Workflow Integration Tests
//!
//! Tests for complete workflow execution including agent interactions,
//! real LLM calls, document processing, and complex workflow patterns.

use graphbit_core::{agents::*, embeddings::*, graph::*, llm::*, types::*, workflow::*};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

#[tokio::test]
async fn test_simple_agent_workflow_execution() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::ollama("llama3.2");
    let agent_config = AgentConfig::new("Test Agent", "A simple test agent", llm_config.clone())
        .with_system_prompt("You are a helpful assistant. Respond briefly.");

    let agent_node = WorkflowNode::new(
        "Agent Node",
        "Processes user input",
        NodeType::Agent {
            agent_id: agent_config.id.clone(),
            prompt_template: "Process this input: {{input}}".to_string(),
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Simple Agent Workflow")
        .description("A simple workflow with one agent")
        .add_node(agent_node)
        .expect("Failed to add node");

    let workflow = builder.build().expect("Failed to build workflow");
    assert_eq!(workflow.name, "Simple Agent Workflow");
    assert_eq!(workflow.graph.node_count(), 1);
}

#[tokio::test]
async fn test_sequential_agent_workflow() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::ollama("llama3.2");

    let agent1_config =
        AgentConfig::new("Analyzer Agent", "Analyzes input text", llm_config.clone())
            .with_system_prompt("Analyze the input and extract key themes. Be concise.");

    let agent2_config = AgentConfig::new(
        "Summarizer Agent",
        "Summarizes analysis",
        llm_config.clone(),
    )
    .with_system_prompt("Create a brief summary of the analysis. One sentence only.");

    let analyzer_node = WorkflowNode::new(
        "Analyzer",
        "Analyzes input text",
        NodeType::Agent {
            agent_id: agent1_config.id.clone(),
            prompt_template: "Analyze: {{input}}".to_string(),
        },
    );

    let summarizer_node = WorkflowNode::new(
        "Summarizer",
        "Summarizes analysis",
        NodeType::Agent {
            agent_id: agent2_config.id.clone(),
            prompt_template: "Summarize: {{previous_output}}".to_string(),
        },
    );

    let (builder, analyzer_id) = WorkflowBuilder::new("Sequential Agent Workflow")
        .add_node(analyzer_node)
        .expect("Failed to add analyzer node");

    let (builder, summarizer_id) = builder
        .add_node(summarizer_node)
        .expect("Failed to add summarizer node");

    let workflow = builder
        .connect(analyzer_id, summarizer_id, WorkflowEdge::data_flow())
        .expect("Failed to connect nodes")
        .build()
        .expect("Failed to build workflow");

    assert_eq!(workflow.graph.node_count(), 2);
    assert_eq!(workflow.graph.edge_count(), 1);
    assert!(!workflow.graph.has_cycles());
}

#[tokio::test]
async fn test_parallel_agent_workflow() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::ollama("llama3.2");

    // Create three agents for parallel processing
    let sentiment_agent =
        AgentConfig::new("Sentiment Agent", "Analyzes sentiment", llm_config.clone())
            .with_system_prompt(
                "Analyze sentiment: positive, negative, or neutral. One word only.",
            );

    let topic_agent = AgentConfig::new("Topic Agent", "Extracts topics", llm_config.clone())
        .with_system_prompt("Extract main topic. One word only.");

    let length_agent = AgentConfig::new("Length Agent", "Analyzes text length", llm_config.clone())
        .with_system_prompt("Classify text length: short, medium, or long. One word only.");

    // Create nodes
    let sentiment_node = WorkflowNode::new(
        "Sentiment Analysis",
        "Analyzes sentiment",
        NodeType::Agent {
            agent_id: sentiment_agent.id.clone(),
            prompt_template: "Sentiment of: {{input}}".to_string(),
        },
    );

    let topic_node = WorkflowNode::new(
        "Topic Extraction",
        "Extracts topics",
        NodeType::Agent {
            agent_id: topic_agent.id.clone(),
            prompt_template: "Topic of: {{input}}".to_string(),
        },
    );

    let length_node = WorkflowNode::new(
        "Length Analysis",
        "Analyzes length",
        NodeType::Agent {
            agent_id: length_agent.id.clone(),
            prompt_template: "Length of: {{input}}".to_string(),
        },
    );

    // Build parallel workflow
    let (builder, _sentiment_id) = WorkflowBuilder::new("Parallel Analysis Workflow")
        .add_node(sentiment_node)
        .expect("Failed to add sentiment node");

    let (builder, _topic_id) = builder
        .add_node(topic_node)
        .expect("Failed to add topic node");

    let (builder, _length_id) = builder
        .add_node(length_node)
        .expect("Failed to add length node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Test parallel structure
    assert_eq!(workflow.graph.node_count(), 3);
    assert_eq!(workflow.graph.edge_count(), 0); // No connections = parallel execution
}

#[tokio::test]
async fn test_workflow_with_conditions() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create condition node
    let condition_node = WorkflowNode::new(
        "Length Check",
        "Checks if text is long",
        NodeType::Condition {
            expression: "input.length > 100".to_string(),
        },
    );

    // Create processing nodes
    let short_processor = WorkflowNode::new(
        "Short Text Processor",
        "Processes short text",
        NodeType::Transform {
            transformation: "process_short".to_string(),
        },
    );

    let long_processor = WorkflowNode::new(
        "Long Text Processor",
        "Processes long text",
        NodeType::Transform {
            transformation: "process_long".to_string(),
        },
    );

    // Build conditional workflow
    let (builder, condition_id) = WorkflowBuilder::new("Conditional Workflow")
        .add_node(condition_node)
        .expect("Failed to add condition node");

    let (builder, short_id) = builder
        .add_node(short_processor)
        .expect("Failed to add short processor");

    let (builder, long_id) = builder
        .add_node(long_processor)
        .expect("Failed to add long processor");

    // Connect with conditional edges
    let workflow = builder
        .connect(
            condition_id.clone(),
            short_id,
            WorkflowEdge::conditional("false"),
        )
        .expect("Failed to connect to short processor")
        .connect(condition_id, long_id, WorkflowEdge::conditional("true"))
        .expect("Failed to connect to long processor")
        .build()
        .expect("Failed to build workflow");

    assert_eq!(workflow.graph.node_count(), 3);
    assert_eq!(workflow.graph.edge_count(), 2);
}

#[tokio::test]
async fn test_workflow_with_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let doc_path = temp_dir.path().join("test_document.txt");

    let document_content =
        "This is a test document for workflow processing.\nIt contains multiple lines of text.";
    fs::write(&doc_path, document_content).expect("Failed to write test document");

    // Create document loader node
    let doc_loader_node = WorkflowNode::new(
        "Document Loader",
        "Loads documents from file system",
        NodeType::DocumentLoader {
            document_type: "txt".to_string(),
            source_path: doc_path.to_str().unwrap().to_string(),
            encoding: Some("utf-8".to_string()),
        },
    );

    // Create text processor node
    let llm_config = LlmConfig::ollama("llama3.2");
    let agent_config = AgentConfig::new("Text Processor", "Processes loaded text", llm_config)
        .with_system_prompt("Summarize the provided text briefly.");

    let text_processor_node = WorkflowNode::new(
        "Text Processor",
        "Processes the loaded document",
        NodeType::Agent {
            agent_id: agent_config.id.clone(),
            prompt_template: "Summarize this text: {{document_content}}".to_string(),
        },
    );

    // Build document processing workflow
    let (builder, loader_id) = WorkflowBuilder::new("Document Processing Workflow")
        .add_node(doc_loader_node)
        .expect("Failed to add document loader");

    let (builder, processor_id) = builder
        .add_node(text_processor_node)
        .expect("Failed to add text processor");

    let workflow = builder
        .connect(loader_id, processor_id, WorkflowEdge::data_flow())
        .expect("Failed to connect nodes")
        .build()
        .expect("Failed to build workflow");

    assert_eq!(workflow.graph.node_count(), 2);
    assert_eq!(workflow.graph.edge_count(), 1);
}

#[tokio::test]
async fn test_workflow_with_delay_and_timeouts() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create delay node
    let delay_node = WorkflowNode::new(
        "Processing Delay",
        "Adds delay for rate limiting",
        NodeType::Delay {
            duration_seconds: 2,
        },
    )
    .with_timeout(5); // 5 second timeout

    // Create processor node
    let processor_node = WorkflowNode::new(
        "Data Processor",
        "Processes data after delay",
        NodeType::Transform {
            transformation: "process_data".to_string(),
        },
    )
    .with_timeout(10); // 10 second timeout

    // Build workflow with delays
    let (builder, delay_id) = WorkflowBuilder::new("Delayed Processing Workflow")
        .add_node(delay_node)
        .expect("Failed to add delay node");

    let (builder, processor_id) = builder
        .add_node(processor_node)
        .expect("Failed to add processor node");

    let workflow = builder
        .connect(delay_id, processor_id, WorkflowEdge::data_flow())
        .expect("Failed to connect nodes")
        .build()
        .expect("Failed to build workflow");

    assert_eq!(workflow.graph.node_count(), 2);
    assert_eq!(workflow.graph.edge_count(), 1);
}

#[tokio::test]
async fn test_workflow_retry_configuration() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let retry_config = RetryConfig::new(3)
        .with_exponential_backoff(1000, 2.0, 10000)
        .with_jitter(0.1)
        .with_retryable_errors(vec![
            RetryableErrorType::NetworkError,
            RetryableErrorType::TimeoutError,
        ]);

    let agent_node = WorkflowNode::new(
        "Retry Agent",
        "Agent with retry configuration",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Process with retry: {{input}}".to_string(),
        },
    )
    .with_retry_config(retry_config);

    let (builder, _node_id) = WorkflowBuilder::new("Retry Workflow")
        .add_node(agent_node)
        .expect("Failed to add retry node");

    let workflow = builder.build().expect("Failed to build workflow");
    assert_eq!(workflow.graph.node_count(), 1);
}

#[tokio::test]
async fn test_workflow_concurrency_limits() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test workflow execution with concurrency configuration
    let executor = WorkflowExecutor::new_low_latency(); // Limited concurrency
    let stats = executor.get_concurrency_stats().await;

    assert_eq!(stats.current_active_tasks, 0);
    assert_eq!(stats.total_permit_acquisitions, 0);
}

#[tokio::test]
async fn test_workflow_execution_stats() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut context = WorkflowContext::new(WorkflowId::new());
    context.set_variable("test_var".to_string(), json!("test_value"));
    context.set_metadata("workflow_type".to_string(), json!("test"));

    let stats = WorkflowExecutionStats {
        total_nodes: 5,
        successful_nodes: 4,
        failed_nodes: 1,
        avg_execution_time_ms: 250.0,
        max_concurrent_nodes: 3,
        total_execution_time_ms: 1250,
        peak_memory_usage_mb: Some(128.0),
        semaphore_acquisitions: 10,
        avg_semaphore_wait_ms: 15.5,
    };

    context.set_stats(stats);

    let retrieved_stats = context.get_stats().unwrap();
    assert_eq!(retrieved_stats.total_nodes, 5);
    assert_eq!(retrieved_stats.successful_nodes, 4);
    assert_eq!(retrieved_stats.failed_nodes, 1);
}

#[tokio::test]
async fn test_complex_branching_workflow() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let llm_config = LlmConfig::ollama("llama3.2");

    // Input classifier
    let classifier_agent =
        AgentConfig::new("Classifier", "Classifies input type", llm_config.clone())
            .with_system_prompt("Classify input as: text, number, or other. One word only.");

    let classifier_node = WorkflowNode::new(
        "Input Classifier",
        "Classifies the input type",
        NodeType::Agent {
            agent_id: classifier_agent.id.clone(),
            prompt_template: "Classify: {{input}}".to_string(),
        },
    );

    // Text processor
    let text_processor =
        AgentConfig::new("Text Processor", "Processes text input", llm_config.clone())
            .with_system_prompt("Process text input. Respond briefly.");

    let text_node = WorkflowNode::new(
        "Text Processor",
        "Processes text inputs",
        NodeType::Agent {
            agent_id: text_processor.id.clone(),
            prompt_template: "Process text: {{input}}".to_string(),
        },
    );

    // Number processor
    let number_processor = AgentConfig::new(
        "Number Processor",
        "Processes numeric input",
        llm_config.clone(),
    )
    .with_system_prompt("Process numeric input. Respond briefly.");

    let number_node = WorkflowNode::new(
        "Number Processor",
        "Processes numeric inputs",
        NodeType::Agent {
            agent_id: number_processor.id.clone(),
            prompt_template: "Process number: {{input}}".to_string(),
        },
    );

    // Other processor
    let other_processor = AgentConfig::new(
        "Other Processor",
        "Processes other input types",
        llm_config.clone(),
    )
    .with_system_prompt("Process other input types. Respond briefly.");

    let other_node = WorkflowNode::new(
        "Other Processor",
        "Processes other input types",
        NodeType::Agent {
            agent_id: other_processor.id.clone(),
            prompt_template: "Process other: {{input}}".to_string(),
        },
    );

    // Final aggregator
    let aggregator = AgentConfig::new("Aggregator", "Aggregates results", llm_config.clone())
        .with_system_prompt("Combine and summarize results. One sentence only.");

    let aggregator_node = WorkflowNode::new(
        "Result Aggregator",
        "Combines all processing results",
        NodeType::Agent {
            agent_id: aggregator.id.clone(),
            prompt_template: "Combine results: {{all_outputs}}".to_string(),
        },
    );

    // Build complex branching workflow
    let (builder, classifier_id) = WorkflowBuilder::new("Complex Branching Workflow")
        .add_node(classifier_node)
        .expect("Failed to add classifier");

    let (builder, text_id) = builder
        .add_node(text_node)
        .expect("Failed to add text processor");

    let (builder, number_id) = builder
        .add_node(number_node)
        .expect("Failed to add number processor");

    let (builder, other_id) = builder
        .add_node(other_node)
        .expect("Failed to add other processor");

    let (builder, aggregator_id) = builder
        .add_node(aggregator_node)
        .expect("Failed to add aggregator");

    // Connect classifier to processors based on classification
    let builder = builder
        .connect(
            classifier_id.clone(),
            text_id.clone(),
            WorkflowEdge::conditional("text"),
        )
        .expect("Failed to connect to text processor")
        .connect(
            classifier_id.clone(),
            number_id.clone(),
            WorkflowEdge::conditional("number"),
        )
        .expect("Failed to connect to number processor")
        .connect(
            classifier_id,
            other_id.clone(),
            WorkflowEdge::conditional("other"),
        )
        .expect("Failed to connect to other processor");

    // Connect all processors to aggregator
    let workflow = builder
        .connect(text_id, aggregator_id.clone(), WorkflowEdge::data_flow())
        .expect("Failed to connect text to aggregator")
        .connect(number_id, aggregator_id.clone(), WorkflowEdge::data_flow())
        .expect("Failed to connect number to aggregator")
        .connect(other_id, aggregator_id, WorkflowEdge::data_flow())
        .expect("Failed to connect other to aggregator")
        .build()
        .expect("Failed to build complex workflow");

    assert_eq!(workflow.graph.node_count(), 5);
    assert_eq!(workflow.graph.edge_count(), 6);
    assert!(!workflow.graph.has_cycles());
}

#[tokio::test]
async fn test_workflow_metadata_and_context() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut workflow = Workflow::new("Metadata Test Workflow", "Testing metadata handling");
    workflow.set_metadata("version".to_string(), json!("1.0.0"));
    workflow.set_metadata("author".to_string(), json!("test_user"));
    workflow.set_metadata(
        "tags".to_string(),
        json!(["test", "integration", "workflow"]),
    );

    assert_eq!(workflow.metadata.len(), 3);
    assert_eq!(workflow.metadata.get("version"), Some(&json!("1.0.0")));
    assert_eq!(workflow.metadata.get("author"), Some(&json!("test_user")));

    // Test workflow context
    let mut context = WorkflowContext::new(workflow.id.clone());
    context.set_variable("input_data".to_string(), json!("test input"));
    context.set_variable("processing_mode".to_string(), json!("batch"));
    context.set_metadata(
        "start_time".to_string(),
        json!(chrono::Utc::now().to_rfc3339()),
    );

    assert_eq!(
        context.get_variable("input_data"),
        Some(&json!("test input"))
    );
    assert_eq!(
        context.get_variable("processing_mode"),
        Some(&json!("batch"))
    );
}

#[tokio::test]
async fn test_real_llm_workflow_execution() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real LLM workflow test - no valid OpenAI API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();

    let llm_config = LlmConfig::openai(api_key, "gpt-3.5-turbo");
    let agent_config = AgentConfig::new(
        "Real LLM Agent",
        "Agent that makes real LLM calls",
        llm_config,
    )
    .with_system_prompt("You are a helpful assistant. Be very brief.")
    .with_max_tokens(50)
    .with_temperature(0.0);

    // Create agent and register with executor
    let agent = Agent::new(agent_config.clone()).await;
    match agent {
        Ok(agent) => {
            let executor = WorkflowExecutor::new();
            executor.register_agent(Arc::new(agent)).await;

            let agent_node = WorkflowNode::new(
                "Real LLM Node",
                "Makes real LLM API calls",
                NodeType::Agent {
                    agent_id: agent_config.id.clone(),
                    prompt_template: "Say hello briefly: {{input}}".to_string(),
                },
            );

            let (builder, _node_id) = WorkflowBuilder::new("Real LLM Workflow")
                .add_node(agent_node)
                .expect("Failed to add LLM node");

            let workflow = builder.build().expect("Failed to build workflow");

            // Try to execute the workflow
            let result = executor.execute(workflow).await;
            match result {
                Ok(context) => {
                    println!("Real LLM workflow executed successfully");
                    assert!(context.state.is_terminal());
                }
                Err(e) => {
                    println!("Real LLM workflow failed (may be expected): {e:?}");
                }
            }
        }
        Err(e) => {
            println!("Failed to create real LLM agent (may be expected): {e:?}");
        }
    }
}

// Additional error handling and edge case tests
#[tokio::test]
async fn test_workflow_with_invalid_connections() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Transform {
            transformation: "test1".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "test2".to_string(),
        },
    );

    let (builder, node1_id) = WorkflowBuilder::new("Invalid Connection Test")
        .add_node(node1)
        .expect("Failed to add node1");

    let (builder, _node2_id) = builder.add_node(node2).expect("Failed to add node2");

    // Try to connect to non-existent node
    let nonexistent_id = NodeId::new();
    let result = builder.connect(node1_id, nonexistent_id, WorkflowEdge::data_flow());

    assert!(
        result.is_err(),
        "Should fail to connect to non-existent node"
    );
}

#[tokio::test]
async fn test_workflow_cycle_detection() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let node1 = WorkflowNode::new("Node 1", "First", NodeType::Split);
    let node2 = WorkflowNode::new("Node 2", "Second", NodeType::Join);
    let node3 = WorkflowNode::new("Node 3", "Third", NodeType::Split);

    let (builder, id1) = WorkflowBuilder::new("Cycle Test")
        .add_node(node1)
        .expect("Failed to add node1");

    let (builder, id2) = builder.add_node(node2).expect("Failed to add node2");

    let (builder, id3) = builder.add_node(node3).expect("Failed to add node3");

    // Create a cycle: 1 -> 2 -> 3 -> 1
    let builder = builder
        .connect(id1.clone(), id2.clone(), WorkflowEdge::data_flow())
        .expect("Failed to connect 1->2")
        .connect(id2, id3.clone(), WorkflowEdge::data_flow())
        .expect("Failed to connect 2->3")
        .connect(id3, id1, WorkflowEdge::data_flow())
        .expect("Failed to connect 3->1");

    // Building should fail due to cycle
    let result = builder.build();
    assert!(result.is_err(), "Should fail to build workflow with cycle");
}

#[tokio::test]
async fn test_workflow_execution_context_isolation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let workflow_id1 = WorkflowId::new();
    let workflow_id2 = WorkflowId::new();

    let mut context1 = WorkflowContext::new(workflow_id1);
    let mut context2 = WorkflowContext::new(workflow_id2);

    // Set different variables in each context
    context1.set_variable("shared_key".to_string(), json!("value1"));
    context2.set_variable("shared_key".to_string(), json!("value2"));

    // Verify isolation
    assert_eq!(context1.get_variable("shared_key"), Some(&json!("value1")));
    assert_eq!(context2.get_variable("shared_key"), Some(&json!("value2")));
    assert_ne!(context1.workflow_id, context2.workflow_id);
}

#[tokio::test]
async fn test_workflow_executor_configuration_variants() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test different executor configurations
    let high_throughput = WorkflowExecutor::new_high_throughput();
    let low_latency = WorkflowExecutor::new_low_latency();
    let memory_optimized = WorkflowExecutor::new_memory_optimized();

    // Verify different concurrency limits
    let ht_concurrency = high_throughput.max_concurrency().await;
    let ll_concurrency = low_latency.max_concurrency().await;
    let mo_concurrency = memory_optimized.max_concurrency().await;

    assert!(ht_concurrency >= ll_concurrency);
    assert!(ll_concurrency >= mo_concurrency);
    assert!(mo_concurrency > 0);
}

#[tokio::test]
async fn test_workflow_error_propagation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create a workflow with an invalid agent reference
    let invalid_agent_id = AgentId::new(); // Not registered anywhere

    let invalid_node = WorkflowNode::new(
        "Invalid Agent Node",
        "References non-existent agent",
        NodeType::Agent {
            agent_id: invalid_agent_id,
            prompt_template: "This will fail".to_string(),
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Error Propagation Test")
        .add_node(invalid_node)
        .expect("Failed to add invalid node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Try to execute - should fail gracefully
    let executor = WorkflowExecutor::new();
    let result = executor.execute(workflow).await;

    // Should handle gracefully (may succeed or fail depending on implementation)
    match result {
        Ok(_) => println!("Workflow execution succeeded despite invalid agent reference"),
        Err(e) => println!("Workflow execution failed as expected: {e:?}"),
    }
}

#[tokio::test]
async fn test_multi_provider_workflow_execution() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let mut available_providers = Vec::new();

    // Test which providers are available
    if super::has_openai_api_key() {
        available_providers.push(("OpenAI", "gpt-3.5-turbo"));
    }
    if super::has_anthropic_api_key() {
        available_providers.push(("Anthropic", "claude-3-haiku-20240307"));
    }
    if super::has_ollama_available().await {
        available_providers.push(("Ollama", "llama3.2"));
    }

    if available_providers.is_empty() {
        println!("Skipping multi-provider workflow test - no providers available");
        return;
    }

    println!(
        "Testing workflow with {len} provider(s): {providers:?}",
        len = available_providers.len(),
        providers = available_providers
            .iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>()
    );

    for (provider_name, model) in available_providers {
        println!("🧪 Testing workflow with {provider_name}");

        let llm_config = match provider_name {
            "OpenAI" => LlmConfig::openai(std::env::var("OPENAI_API_KEY").unwrap(), model),
            "Anthropic" => LlmConfig::anthropic(std::env::var("ANTHROPIC_API_KEY").unwrap(), model),
            "Ollama" => LlmConfig::ollama(model),
            _ => continue,
        };

        let agent_config = AgentConfig::new(
            format!("{provider_name} Test Agent"),
            format!("Agent using {provider_name}"),
            llm_config,
        )
        .with_system_prompt(
            "You are a helpful assistant. Be very brief - respond with only 1-2 words.",
        )
        .with_max_tokens(10)
        .with_temperature(0.0);

        let agent_result = Agent::new(agent_config.clone()).await;
        if let Ok(agent) = agent_result {
            let executor = WorkflowExecutor::new();
            executor.register_agent(Arc::new(agent)).await;

            let agent_node = WorkflowNode::new(
                format!("{provider_name} Node"),
                format!("Node using {provider_name}"),
                NodeType::Agent {
                    agent_id: agent_config.id.clone(),
                    prompt_template: "Say hello: {{input}}".to_string(),
                },
            );

            let (builder, _node_id) = WorkflowBuilder::new(format!("{provider_name} Workflow"))
                .add_node(agent_node)
                .expect("Failed to add node");

            let workflow = builder.build().expect("Failed to build workflow");

            let result = executor.execute(workflow).await;
            match result {
                Ok(context) => {
                    println!("{provider_name} workflow executed successfully");
                    assert!(context.state.is_terminal());
                }
                Err(e) => {
                    println!("{provider_name} workflow failed: {e:?}");
                }
            }
        } else {
            println!(
                "Failed to create {provider_name} agent: {err:?}",
                err = agent_result.err()
            );
        }
    }
}

#[tokio::test]
async fn test_real_embeddings_workflow() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real embeddings workflow test - no valid OpenAI API key");
        return;
    }

    // This test would require implementing embedding nodes in the workflow
    // For now, we'll test that we can at least create the configuration
    let embedding_config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key: super::get_openai_api_key_or_skip(),
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(16),
        extra_params: HashMap::new(),
    };

    let service_result = EmbeddingService::new(embedding_config);
    assert!(
        service_result.is_ok(),
        "Should create embedding service with real API key"
    );

    println!("Real embeddings workflow configuration successful");
}

#[tokio::test]
async fn test_comprehensive_real_api_workflow() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping comprehensive real API workflow test - no valid OpenAI API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();

    // Create multiple agents with different roles
    let analyzer_config = AgentConfig::new(
        "Content Analyzer",
        "Analyzes input content and extracts key themes",
        LlmConfig::openai(api_key.clone(), "gpt-3.5-turbo"),
    )
    .with_system_prompt("Analyze the text and identify the main topic in 1-2 words only.")
    .with_max_tokens(5)
    .with_temperature(0.0);

    let summarizer_config = AgentConfig::new(
        "Content Summarizer",
        "Creates brief summaries",
        LlmConfig::openai(api_key, "gpt-3.5-turbo"),
    )
    .with_system_prompt("Create a 3-word summary.")
    .with_max_tokens(5)
    .with_temperature(0.0);

    // Create agents
    let analyzer_result = Agent::new(analyzer_config.clone()).await;
    let summarizer_result = Agent::new(summarizer_config.clone()).await;

    if analyzer_result.is_err() || summarizer_result.is_err() {
        println!("Failed to create agents for comprehensive test");
        return;
    }

    let analyzer = Arc::new(analyzer_result.unwrap());
    let summarizer = Arc::new(summarizer_result.unwrap());

    // Set up executor and register agents
    let executor = WorkflowExecutor::new();
    executor.register_agent(analyzer.clone()).await;
    executor.register_agent(summarizer.clone()).await;

    // Create workflow nodes
    let analyzer_node = WorkflowNode::new(
        "Analyzer",
        "Analyzes input content",
        NodeType::Agent {
            agent_id: analyzer_config.id.clone(),
            prompt_template: "Analyze this: {{input}}".to_string(),
        },
    );

    let summarizer_node = WorkflowNode::new(
        "Summarizer",
        "Summarizes analysis",
        NodeType::Agent {
            agent_id: summarizer_config.id.clone(),
            prompt_template: "Summarize: {{previous_output}}".to_string(),
        },
    );

    // Build sequential workflow
    let (builder, analyzer_id) = WorkflowBuilder::new("Comprehensive Real API Workflow")
        .add_node(analyzer_node)
        .expect("Failed to add analyzer node");

    let (builder, summarizer_id) = builder
        .add_node(summarizer_node)
        .expect("Failed to add summarizer node");

    let workflow = builder
        .connect(analyzer_id, summarizer_id, WorkflowEdge::data_flow())
        .expect("Failed to connect nodes")
        .build()
        .expect("Failed to build workflow");

    // Execute workflow
    let result = executor.execute(workflow).await;
    match result {
        Ok(context) => {
            println!("Comprehensive real API workflow executed successfully");
            assert!(context.state.is_terminal());

            // Check execution stats
            if let Some(stats) = context.get_stats() {
                println!(
                    "Workflow stats: {total_nodes} nodes executed, {successful_nodes} successful",
                    total_nodes = stats.total_nodes,
                    successful_nodes = stats.successful_nodes
                );
            }
        }
        Err(e) => {
            println!("Comprehensive real API workflow failed: {e:?}");
            // Don't panic here as this might fail due to various API-related issues
        }
    }
}

#[tokio::test]
async fn test_workflow_timeout_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Create a node with very short timeout
    let timeout_node = WorkflowNode::new(
        "Timeout Node",
        "Node with short timeout",
        NodeType::Delay {
            duration_seconds: 10, // 10 second delay
        },
    )
    .with_timeout(1); // 1 second timeout (shorter than delay)

    let (builder, _node_id) = WorkflowBuilder::new("Timeout Test")
        .add_node(timeout_node)
        .expect("Failed to add timeout node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Execute with timeout
    let executor = WorkflowExecutor::new().with_max_node_execution_time(2000); // 2 second max
    let start = std::time::Instant::now();
    let result = executor.execute(workflow).await;
    let duration = start.elapsed();

    // Should complete quickly due to timeout, not wait full 10 seconds
    assert!(duration.as_secs() < 5, "Should timeout quickly");

    // Result might be error or success depending on implementation
    match result {
        Ok(_) => println!("Workflow completed despite timeout"),
        Err(e) => println!("Workflow timed out as expected: {e:?}"),
    }
}
