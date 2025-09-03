//! Comprehensive serialization unit tests
//!
//! Tests for JSON serialization/deserialization, schema validation,
//! backward compatibility, and edge cases in data serialization.

use graphbit_core::{
    errors::GraphBitError,
    graph::{EdgeType, NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
    types::{
        AgentId, AgentMessage, MessageContent, NodeId, RetryConfig, WorkflowContext, WorkflowId,
        WorkflowState,
    },
};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_workflow_graph_serialization_roundtrip() {
    let mut graph = WorkflowGraph::new();

    // Create complex graph with various node types
    let agent_node = WorkflowNode::new(
        "Agent",
        "AI Agent",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Process: {input}".to_string(),
        },
    )
    .with_config("temperature".to_string(), json!(0.7))
    .with_config("max_tokens".to_string(), json!(1000))
    .with_timeout(30)
    .with_tags(vec!["ai".to_string(), "processing".to_string()]);

    let condition_node = WorkflowNode::new(
        "Condition",
        "Check condition",
        NodeType::Condition {
            expression: "value > 10".to_string(),
        },
    );

    let transform_node = WorkflowNode::new(
        "Transform",
        "Transform data",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let http_node = WorkflowNode::new(
        "HTTP",
        "HTTP Request",
        NodeType::HttpRequest {
            url: "https://api.example.com/data".to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert("Authorization".to_string(), "Bearer token".to_string());
                headers
            },
        },
    );

    let delay_node = WorkflowNode::new(
        "Delay",
        "Wait",
        NodeType::Delay {
            duration_seconds: 5,
        },
    );

    let doc_loader_node = WorkflowNode::new(
        "DocLoader",
        "Load document",
        NodeType::DocumentLoader {
            document_type: "pdf".to_string(),
            source_path: "/path/to/document.pdf".to_string(),
            encoding: Some("utf-8".to_string()),
        },
    );

    // Store node IDs for edge creation
    let agent_id = agent_node.id.clone();
    let condition_id = condition_node.id.clone();
    let transform_id = transform_node.id.clone();
    let http_id = http_node.id.clone();
    let delay_id = delay_node.id.clone();
    let doc_loader_id = doc_loader_node.id.clone();

    // Add nodes to graph
    graph.add_node(agent_node).unwrap();
    graph.add_node(condition_node).unwrap();
    graph.add_node(transform_node).unwrap();
    graph.add_node(http_node).unwrap();
    graph.add_node(delay_node).unwrap();
    graph.add_node(doc_loader_node).unwrap();

    // Add various edge types
    graph
        .add_edge(
            agent_id.clone(),
            condition_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            condition_id.clone(),
            transform_id.clone(),
            WorkflowEdge::conditional("result == true")
                .with_metadata("priority".to_string(), json!(1)),
        )
        .unwrap();
    graph
        .add_edge(
            condition_id.clone(),
            http_id.clone(),
            WorkflowEdge::conditional("result == false"),
        )
        .unwrap();
    graph
        .add_edge(
            transform_id.clone(),
            delay_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            http_id.clone(),
            doc_loader_id.clone(),
            WorkflowEdge::data_flow()
                .with_transform("json_extract")
                .with_metadata("timeout".to_string(), json!(30)),
        )
        .unwrap();

    // Serialize the graph
    let serialized = serde_json::to_string_pretty(&graph).unwrap();

    // Deserialize back
    let mut deserialized: WorkflowGraph = serde_json::from_str(&serialized).unwrap();
    deserialized.rebuild_graph().unwrap();

    // Verify structure is preserved
    assert_eq!(deserialized.node_count(), 6);
    assert_eq!(deserialized.edge_count(), 5);

    // Verify specific nodes
    let agent_node = deserialized.get_node(&agent_id).unwrap();
    assert_eq!(agent_node.name, "Agent");
    assert!(matches!(agent_node.node_type, NodeType::Agent { .. }));
    assert_eq!(agent_node.config.get("temperature").unwrap(), &json!(0.7));
    assert_eq!(agent_node.timeout_seconds, Some(30));
    assert_eq!(agent_node.tags.len(), 2);

    let http_node = deserialized.get_node(&http_id).unwrap();
    if let NodeType::HttpRequest {
        url,
        method,
        headers,
    } = &http_node.node_type
    {
        assert_eq!(url, "https://api.example.com/data");
        assert_eq!(method, "POST");
        assert_eq!(headers.len(), 2);
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
    } else {
        panic!("Expected HttpRequest node type");
    }

    // Verify edges
    let edges: Vec<_> = deserialized
        .get_edges()
        .iter()
        .filter(|(from, _, _)| *from == condition_id)
        .collect();
    assert_eq!(edges.len(), 2);

    for (_, _, edge) in edges {
        assert!(matches!(edge.edge_type, EdgeType::Conditional));
        assert!(edge.condition.is_some());
    }
}

#[test]
fn test_workflow_context_serialization() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());

    // Add complex metadata
    context.set_metadata("string_value".to_string(), json!("test"));
    context.set_metadata("number_value".to_string(), json!(42));
    context.set_metadata("boolean_value".to_string(), json!(true));
    context.set_metadata("array_value".to_string(), json!([1, 2, 3]));
    context.set_metadata(
        "object_value".to_string(),
        json!({
            "nested": {
                "key": "value",
                "number": 123
            }
        }),
    );

    // Add node outputs
    let node_id1 = NodeId::new();
    let node_id2 = NodeId::new();

    context.set_node_output(
        &node_id1,
        json!({
            "result": "success",
            "data": {
                "processed_items": 10,
                "errors": []
            }
        }),
    );

    context.set_node_output(
        &node_id2,
        json!({
            "status": "completed",
            "output": "Final result"
        }),
    );

    // Set workflow state
    context.state = WorkflowState::Running {
        current_node: node_id1.clone(),
    };

    // Serialize
    let serialized = serde_json::to_string_pretty(&context).unwrap();

    // Deserialize
    let deserialized: WorkflowContext = serde_json::from_str(&serialized).unwrap();

    // Verify structure
    assert_eq!(deserialized.workflow_id, workflow_id);
    assert_eq!(deserialized.metadata.len(), 5);
    assert_eq!(deserialized.node_outputs.len(), 2);

    // Verify metadata
    assert_eq!(
        deserialized.metadata.get("string_value").unwrap(),
        &json!("test")
    );
    assert_eq!(
        deserialized.metadata.get("number_value").unwrap(),
        &json!(42)
    );
    assert_eq!(
        deserialized.metadata.get("boolean_value").unwrap(),
        &json!(true)
    );

    // Verify node outputs
    let node_output = deserialized
        .node_outputs
        .get(&node_id1.to_string())
        .unwrap();
    assert_eq!(node_output.get("result").unwrap(), &json!("success"));

    let data_section = node_output.get("data").unwrap();
    assert_eq!(data_section.get("processed_items").unwrap(), &json!(10));

    // Verify state
    match &deserialized.state {
        WorkflowState::Running { current_node } => {
            assert_eq!(*current_node, node_id1);
        }
        _ => panic!("Expected Running state"),
    }
}

#[test]
fn test_agent_message_serialization() {
    let sender = AgentId::new();
    let recipient = AgentId::new();

    // Test different message content types
    let messages = vec![
        AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::Text("Hello, world!".to_string()),
        )
        .with_metadata("priority".to_string(), json!("high"))
        .with_metadata("timestamp".to_string(), json!(1234567890)),
        AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::Data(json!({
                "type": "analysis_result",
                "data": {
                    "score": 0.95,
                    "categories": ["positive", "confident"],
                    "details": {
                        "tokens": 150,
                        "processing_time": 250
                    }
                }
            })),
        ),
        AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::ToolCall {
                tool_name: "calculator".to_string(),
                parameters: json!({
                    "operation": "multiply",
                    "operands": [15, 7]
                }),
            },
        ),
        AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::ToolResponse {
                tool_name: "calculator".to_string(),
                result: json!(105),
                success: true,
            },
        ),
        AgentMessage::new(
            sender.clone(),
            Some(recipient.clone()),
            MessageContent::Error {
                error_code: "CALCULATION_ERROR".to_string(),
                error_message: "Division by zero attempted".to_string(),
            },
        ),
    ];

    for message in messages {
        // Serialize
        let serialized = serde_json::to_string_pretty(&message).unwrap();

        // Deserialize
        let deserialized: AgentMessage = serde_json::from_str(&serialized).unwrap();

        // Verify structure
        assert_eq!(deserialized.sender, message.sender);
        assert_eq!(deserialized.recipient, message.recipient);
        assert_eq!(deserialized.metadata.len(), message.metadata.len());

        // Verify content type is preserved
        match (&message.content, &deserialized.content) {
            (MessageContent::Text(_), MessageContent::Text(_)) => {}
            (MessageContent::Data(_), MessageContent::Data(_)) => {}
            (MessageContent::ToolCall { .. }, MessageContent::ToolCall { .. }) => {}
            (MessageContent::ToolResponse { .. }, MessageContent::ToolResponse { .. }) => {}
            (MessageContent::Error { .. }, MessageContent::Error { .. }) => {}
            _ => panic!("Message content type not preserved"),
        }
    }
}

#[test]
fn test_error_serialization_with_context() {
    let errors = vec![
        GraphBitError::Configuration {
            message: "Missing API key".to_string(),
        },
        GraphBitError::LlmProvider {
            provider: "openai".to_string(),
            message: "Rate limit exceeded".to_string(),
        },
        GraphBitError::Agent {
            agent_id: "agent-123".to_string(),
            message: "Processing failed".to_string(),
        },
        GraphBitError::Validation {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        },
        GraphBitError::RateLimit {
            provider: "anthropic".to_string(),
            retry_after_seconds: 120,
        },
        GraphBitError::Network {
            message: "Connection timeout".to_string(),
        },
        GraphBitError::Serialization {
            message: "Invalid JSON format".to_string(),
        },
        GraphBitError::Internal {
            message: "Unexpected internal error".to_string(),
        },
    ];

    for error in errors {
        // Serialize
        let serialized = serde_json::to_string_pretty(&error).unwrap();

        // Deserialize
        let deserialized: GraphBitError = serde_json::from_str(&serialized).unwrap();

        // Verify error properties are preserved
        assert_eq!(error.to_string(), deserialized.to_string());
        assert_eq!(error.is_retryable(), deserialized.is_retryable());
        assert_eq!(error.retry_delay(), deserialized.retry_delay());
    }
}

#[test]
fn test_retry_config_serialization() {
    let configs = vec![
        RetryConfig::new(3),
        RetryConfig::new(5).with_exponential_backoff(100, 2.0, 10000),
        RetryConfig::new(10)
            .with_exponential_backoff(50, 1.5, 5000)
            .with_jitter(0.2),
    ];

    for config in configs {
        // Serialize
        let serialized = serde_json::to_string_pretty(&config).unwrap();

        // Deserialize
        let deserialized: RetryConfig = serde_json::from_str(&serialized).unwrap();

        // Verify all fields are preserved
        assert_eq!(config.max_attempts, deserialized.max_attempts);
        assert_eq!(config.initial_delay_ms, deserialized.initial_delay_ms);
        assert_eq!(config.backoff_multiplier, deserialized.backoff_multiplier);
        assert_eq!(config.max_delay_ms, deserialized.max_delay_ms);
        assert_eq!(config.jitter_factor, deserialized.jitter_factor);

        // Verify delay calculation works the same
        for attempt in 0..5 {
            let original_delay = config.calculate_delay(attempt);
            let deserialized_delay = deserialized.calculate_delay(attempt);

            // Allow for jitter variation
            let diff = (original_delay as i64 - deserialized_delay as i64).abs();
            let max_jitter = (config.initial_delay_ms as f64
                * config.jitter_factor
                * 2.0_f64.powi(attempt as i32)) as u64;
            assert!(diff <= max_jitter as i64);
        }
    }
}

#[test]
fn test_json_schema_validation() {
    // Test that serialized data conforms to expected JSON schema structure

    let node = WorkflowNode::new("Test", "Test node", NodeType::Split)
        .with_config("param1".to_string(), json!("value1"))
        .with_input_schema(json!({
            "type": "object",
            "properties": {
                "input": {"type": "string"}
            },
            "required": ["input"]
        }))
        .with_output_schema(json!({
            "type": "object",
            "properties": {
                "output": {"type": "string"}
            }
        }));

    let serialized = serde_json::to_value(&node).unwrap();

    // Verify required fields are present
    assert!(serialized.get("id").is_some());
    assert!(serialized.get("name").is_some());
    assert!(serialized.get("description").is_some());
    assert!(serialized.get("node_type").is_some());
    assert!(serialized.get("config").is_some());

    // Verify config structure
    let config = serialized.get("config").unwrap();
    assert!(config.is_object());
    assert_eq!(config.get("param1").unwrap(), &json!("value1"));

    // Verify schema fields
    assert!(serialized.get("input_schema").is_some());
    assert!(serialized.get("output_schema").is_some());

    let input_schema = serialized.get("input_schema").unwrap();
    assert_eq!(input_schema.get("type").unwrap(), &json!("object"));
    assert!(input_schema.get("properties").is_some());
    assert!(input_schema.get("required").is_some());
}

#[test]
fn test_backward_compatibility_with_missing_fields() {
    // Test that deserialization works even with missing optional fields

    let minimal_node_json = json!({
            "id": NodeId::new(),
            "name": "Minimal Node",
            "description": "Minimal description",
            // NodeType is internally tagged with `type` in the current serde representation
            "node_type": { "type": "Split" },
            "config": {},
            "tags": [],
            "retry_config": {
                "max_attempts": 1,
                "initial_delay_ms": 0,
                "backoff_multiplier": 1.0,
                "max_delay_ms": 0,
                "jitter_factor": 0.0,
                "retryable_errors": []
            }
    });
    let node: WorkflowNode = serde_json::from_value(minimal_node_json).unwrap();

    assert_eq!(node.name, "Minimal Node");
    assert_eq!(node.description, "Minimal description");
    assert!(matches!(node.node_type, NodeType::Split));
    assert!(node.config.is_empty());
    assert!(node.input_schema.is_none());
    assert!(node.output_schema.is_none());
    assert!(node.timeout_seconds.is_none());
    assert!(node.tags.is_empty());
}

#[test]
fn test_large_data_serialization_performance() {
    // Test serialization performance with large datasets

    let mut graph = WorkflowGraph::new();

    // Create a large graph
    let mut node_ids = vec![];
    for i in 0..1000 {
        let node = WorkflowNode::new(
            format!("Node{i}"),
            format!("Description for node {i}"),
            NodeType::Transform {
                transformation: format!("transform_{i}"),
            },
        )
        .with_config("index".to_string(), json!(i))
        .with_config("data".to_string(), json!(vec![i; 100])); // Large config data

        node_ids.push(node.id.clone());
        graph.add_node(node).unwrap();
    }

    // Add many edges
    for i in 0..999 {
        graph
            .add_edge(
                node_ids[i].clone(),
                node_ids[i + 1].clone(),
                WorkflowEdge::data_flow(),
            )
            .unwrap();
    }

    // Measure serialization time
    let start = std::time::Instant::now();
    let serialized = serde_json::to_string(&graph).unwrap();
    let serialize_duration = start.elapsed();

    // Measure deserialization time
    let start = std::time::Instant::now();
    let mut deserialized: WorkflowGraph = serde_json::from_str(&serialized).unwrap();
    deserialized.rebuild_graph().unwrap();
    let deserialize_duration = start.elapsed();

    // Verify correctness
    assert_eq!(deserialized.node_count(), 1000);
    assert_eq!(deserialized.edge_count(), 999);

    // Performance should be reasonable (adjust thresholds as needed)
    assert!(serialize_duration.as_millis() < 5000); // Less than 5 seconds
    assert!(deserialize_duration.as_millis() < 5000); // Less than 5 seconds

    println!("Serialization took: {serialize_duration:?}");
    println!("Deserialization took: {deserialize_duration:?}");
    println!("Serialized size: {} bytes", serialized.len());
}
