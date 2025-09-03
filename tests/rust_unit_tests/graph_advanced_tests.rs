//! Advanced graph functionality unit tests
//!
//! Tests for complex graph operations, caching, serialization,
//! and edge cases not covered in basic graph tests.

use graphbit_core::{
    graph::{EdgeType, NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
    types::{AgentId, NodeId, RetryConfig},
};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn workflow_node_and_edge_constructors() {
    let node = WorkflowNode::new("Name", "Desc", NodeType::Split);
    assert_eq!(node.name, "Name");
    assert_eq!(node.description, "Desc");
    assert!(matches!(node.node_type, NodeType::Split));

    let edge = WorkflowEdge::data_flow();
    assert!(matches!(
        edge.edge_type,
        graphbit_core::graph::EdgeType::DataFlow
    ));

    let cond_edge = WorkflowEdge::conditional("x > 0");
    assert!(cond_edge.condition.is_some());
}

#[test]
fn test_graph_cache_invalidation() {
    let mut graph = WorkflowGraph::new();

    // Add nodes
    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Get root nodes (should cache)
    let roots1 = graph.get_root_nodes();
    assert_eq!(roots1.len(), 2);

    // Add edge and verify cache invalidation
    graph
        .add_edge(
            node1_id.clone(),
            node2_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    let roots2 = graph.get_root_nodes();
    assert_eq!(roots2.len(), 1);
    assert!(roots2.contains(&node1_id));
}

#[test]
fn test_graph_dependencies_and_dependents() {
    let mut graph = WorkflowGraph::new();

    // Create a chain: A -> B -> C
    let node_a = WorkflowNode::new("A", "Node A", NodeType::Split);
    let node_b = WorkflowNode::new(
        "B",
        "Node B",
        NodeType::Transform {
            transformation: "test".to_string(),
        },
    );
    let node_c = WorkflowNode::new("C", "Node C", NodeType::Join);

    let id_a = node_a.id.clone();
    let id_b = node_b.id.clone();
    let id_c = node_c.id.clone();

    graph.add_node(node_a).unwrap();
    graph.add_node(node_b).unwrap();
    graph.add_node(node_c).unwrap();

    graph
        .add_edge(id_a.clone(), id_b.clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(id_b.clone(), id_c.clone(), WorkflowEdge::control_flow())
        .unwrap();

    // Test dependencies
    assert!(graph.get_dependencies(&id_a).is_empty());
    assert_eq!(graph.get_dependencies(&id_b), vec![id_a.clone()]);
    assert_eq!(graph.get_dependencies(&id_c), vec![id_b.clone()]);

    // Test dependents
    assert_eq!(graph.get_dependents(&id_a), vec![id_b.clone()]);
    assert_eq!(graph.get_dependents(&id_b), vec![id_c.clone()]);
    assert!(graph.get_dependents(&id_c).is_empty());
}

#[test]
fn test_graph_node_readiness() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("Node1", "First", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second", NodeType::Join);
    let id1 = node1.id.clone();
    let id2 = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph
        .add_edge(id1.clone(), id2.clone(), WorkflowEdge::data_flow())
        .unwrap();

    let mut completed = std::collections::HashSet::new();

    // Node1 should be ready (no dependencies)
    assert!(graph.is_node_ready(&id1, &completed));
    // Node2 should not be ready (depends on Node1)
    assert!(!graph.is_node_ready(&id2, &completed));

    // After completing Node1, Node2 should be ready
    completed.insert(id1.clone());
    assert!(graph.is_node_ready(&id2, &completed));
}

#[test]
fn test_graph_serialization_deserialization() {
    let mut graph = WorkflowGraph::new();

    // Add nodes with complex configurations
    let mut node1 = WorkflowNode::new(
        "Complex Node",
        "Description",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );
    node1 = node1
        .with_config("key1".to_string(), json!("value1"))
        .with_config("key2".to_string(), json!(42))
        .with_input_schema(json!({"type": "object"}))
        .with_output_schema(json!({"type": "string"}))
        .with_timeout(30)
        .with_tags(vec!["test".to_string(), "complex".to_string()]);

    let node1_id = node1.id.clone();
    graph.add_node(node1).unwrap();

    // Serialize
    let serialized = serde_json::to_string(&graph).unwrap();

    // Deserialize
    let mut deserialized: WorkflowGraph = serde_json::from_str(&serialized).unwrap();
    deserialized.rebuild_graph().unwrap();

    // Verify structure is preserved
    assert_eq!(deserialized.node_count(), 1);
    let node = deserialized.get_node(&node1_id).unwrap();
    assert_eq!(node.name, "Complex Node");
    assert_eq!(node.config.get("key1").unwrap(), &json!("value1"));
    assert_eq!(node.config.get("key2").unwrap(), &json!(42));
    assert!(node.input_schema.is_some());
    assert!(node.output_schema.is_some());
    assert_eq!(node.timeout_seconds, Some(30));
    assert_eq!(node.tags.len(), 2);
}

#[test]
fn test_graph_cycle_detection() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("Node1", "First", NodeType::Split);
    let node2 = WorkflowNode::new(
        "Node2",
        "Second",
        NodeType::Transform {
            transformation: "test".to_string(),
        },
    );
    let node3 = WorkflowNode::new("Node3", "Third", NodeType::Join);

    let id1 = node1.id.clone();
    let id2 = node2.id.clone();
    let id3 = node3.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Create a cycle: 1 -> 2 -> 3 -> 1
    graph
        .add_edge(id1.clone(), id2.clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(id2.clone(), id3.clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(id3.clone(), id1.clone(), WorkflowEdge::data_flow())
        .unwrap();

    assert!(graph.has_cycles());
    assert!(graph.validate().is_err());
}

#[test]
fn test_graph_topological_sort() {
    let mut graph = WorkflowGraph::new();

    // Create DAG: A -> B, A -> C, B -> D, C -> D
    let nodes: Vec<_> = (0..4)
        .map(|i| {
            WorkflowNode::new(
                format!("Node{i}"),
                format!("Description {i}"),
                NodeType::Split,
            )
        })
        .collect();

    let ids: Vec<_> = nodes.iter().map(|n| n.id.clone()).collect();

    for node in nodes {
        graph.add_node(node).unwrap();
    }

    graph
        .add_edge(ids[0].clone(), ids[1].clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(ids[0].clone(), ids[2].clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(ids[1].clone(), ids[3].clone(), WorkflowEdge::data_flow())
        .unwrap();
    graph
        .add_edge(ids[2].clone(), ids[3].clone(), WorkflowEdge::data_flow())
        .unwrap();

    let sorted = graph.topological_sort().unwrap();
    assert_eq!(sorted.len(), 4);

    // Verify topological order
    let pos: HashMap<_, _> = sorted.iter().enumerate().map(|(i, id)| (id, i)).collect();
    assert!(pos[&ids[0]] < pos[&ids[1]]);
    assert!(pos[&ids[0]] < pos[&ids[2]]);
    assert!(pos[&ids[1]] < pos[&ids[3]]);
    assert!(pos[&ids[2]] < pos[&ids[3]]);
}

#[test]
fn test_workflow_node_builder_pattern() {
    let node = WorkflowNode::new("Test Node", "Test Description", NodeType::Split)
        .with_config("param1".to_string(), json!("value1"))
        .with_config("param2".to_string(), json!(123))
        .with_input_schema(json!({"type": "object", "properties": {"input": {"type": "string"}}}))
        .with_output_schema(json!({"type": "string"}))
        .with_retry_config(RetryConfig::new(3).with_exponential_backoff(1000, 2.0, 10000))
        .with_timeout(60)
        .with_tags(vec!["test".to_string(), "builder".to_string()]);

    assert_eq!(node.name, "Test Node");
    assert_eq!(node.description, "Test Description");
    assert_eq!(node.config.len(), 2);
    assert!(node.input_schema.is_some());
    assert!(node.output_schema.is_some());
    assert_eq!(node.retry_config.max_attempts, 3);
    assert_eq!(node.timeout_seconds, Some(60));
    assert_eq!(node.tags.len(), 2);
}

#[test]
fn test_workflow_edge_types_and_metadata() {
    let edge = WorkflowEdge::data_flow();
    assert!(matches!(edge.edge_type, EdgeType::DataFlow));
    assert!(edge.condition.is_none());
    assert!(edge.transform.is_none());

    // Create a conditional edge instead since with_condition may not be available on data_flow edges
    let conditional_edge = WorkflowEdge::conditional("x > 0")
        .with_transform("uppercase")
        .with_metadata("priority".to_string(), json!(1))
        .with_metadata("category".to_string(), json!("important"));

    assert_eq!(conditional_edge.condition, Some("x > 0".to_string()));
    assert_eq!(conditional_edge.transform, Some("uppercase".to_string()));
    assert_eq!(conditional_edge.metadata.len(), 2);
    assert_eq!(
        conditional_edge.metadata.get("priority").unwrap(),
        &json!(1)
    );
}

#[test]
fn test_graph_error_conditions() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new("Test", "Test", NodeType::Split);
    let node_id = node.id.clone();

    // Test adding duplicate node
    graph.add_node(node.clone()).unwrap();
    let result = graph.add_node(node);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Node already exists"));

    // Test adding edge with non-existent nodes
    let fake_id = NodeId::new();
    let result = graph.add_edge(node_id.clone(), fake_id.clone(), WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));

    let result = graph.add_edge(fake_id, node_id, WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_node_type_variants() {
    // Test all NodeType variants
    let agent_node = NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "test".to_string(),
    };
    let condition_node = NodeType::Condition {
        expression: "x > 0".to_string(),
    };
    let transform_node = NodeType::Transform {
        transformation: "uppercase".to_string(),
    };
    let split_node = NodeType::Split;
    let join_node = NodeType::Join;
    let delay_node = NodeType::Delay {
        duration_seconds: 5,
    };
    let http_node = NodeType::HttpRequest {
        url: "https://api.example.com".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
    };
    let custom_node = NodeType::Custom {
        function_name: "my_function".to_string(),
    };
    let doc_loader_node = NodeType::DocumentLoader {
        document_type: "pdf".to_string(),
        source_path: "/path/to/doc.pdf".to_string(),
        encoding: Some("utf-8".to_string()),
    };

    // Verify they can be serialized/deserialized
    let types = vec![
        agent_node,
        condition_node,
        transform_node,
        split_node,
        join_node,
        delay_node,
        http_node,
        custom_node,
        doc_loader_node,
    ];

    for node_type in types {
        let serialized = serde_json::to_string(&node_type).unwrap();
        let _deserialized: NodeType = serde_json::from_str(&serialized).unwrap();
    }
}
