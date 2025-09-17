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

// Additional tests to achieve 100% coverage for graph.rs

#[test]
fn test_graph_rebuild_functionality() {
    let mut graph = WorkflowGraph::new();

    // Add nodes and edges
    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test rebuild_graph functionality
    graph.rebuild_graph().unwrap();

    // Verify graph structure is maintained
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.get_node(&node1_id).is_some());
    assert!(graph.get_node(&node2_id).is_some());
}

#[test]
fn test_graph_rebuild_after_deserialization() {
    let mut graph = WorkflowGraph::new();

    // Add nodes and edges
    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.set_metadata("version".to_string(), serde_json::Value::String("1.0".to_string()));

    // Test serialization
    let serialized = serde_json::to_string(&graph).unwrap();
    assert!(!serialized.is_empty());

    // Test deserialization
    let mut deserialized: WorkflowGraph = serde_json::from_str(&serialized).unwrap();

    // After deserialization, need to rebuild the graph structure
    deserialized.rebuild_graph().unwrap();

    // Verify structure is maintained
    assert_eq!(deserialized.node_count(), 2);
    assert_eq!(deserialized.edge_count(), 1);
    assert!(deserialized.get_node(&node1_id).is_some());
    assert!(deserialized.get_node(&node2_id).is_some());
    assert_eq!(deserialized.get_metadata("version"), Some(&serde_json::Value::String("1.0".to_string())));
}

#[test]
fn test_graph_remove_node_functionality() {
    let mut graph = WorkflowGraph::new();

    // Add nodes
    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);

    // Remove node1
    graph.remove_node(&node1_id).unwrap();

    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0); // Edge should be removed too
    assert!(graph.get_node(&node1_id).is_none());
    assert!(graph.get_node(&node2_id).is_some());

    // Try to remove non-existent node
    let result = graph.remove_node(&node1_id);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_graph_metadata_operations() {
    let mut graph = WorkflowGraph::new();

    // Test metadata operations
    graph.set_metadata("version".to_string(), serde_json::Value::String("1.0".to_string()));
    graph.set_metadata("author".to_string(), serde_json::Value::String("test".to_string()));
    graph.set_metadata("count".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));

    assert_eq!(graph.get_metadata("version"), Some(&serde_json::Value::String("1.0".to_string())));
    assert_eq!(graph.get_metadata("author"), Some(&serde_json::Value::String("test".to_string())));
    assert_eq!(graph.get_metadata("count"), Some(&serde_json::Value::Number(serde_json::Number::from(42))));
    assert_eq!(graph.get_metadata("nonexistent"), None);
}

#[test]
fn test_graph_node_lookup_by_name() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("UniqueNode", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("AnotherNode", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Test node lookup by name
    assert_eq!(graph.get_node_id_by_name("UniqueNode"), Some(node1_id));
    assert_eq!(graph.get_node_id_by_name("AnotherNode"), Some(node2_id));
    assert_eq!(graph.get_node_id_by_name("NonExistent"), None);
}

#[test]
fn test_graph_accessor_methods() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test accessor methods
    let nodes = graph.get_nodes();
    assert_eq!(nodes.len(), 2);
    assert!(nodes.contains_key(&node1_id));
    assert!(nodes.contains_key(&node2_id));

    let edges = graph.get_edges();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, node1_id);
    assert_eq!(edges[0].1, node2_id);

    // Test count methods
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_workflow_node_builder_methods() {
    let node = WorkflowNode::new("TestNode", "Test description", NodeType::Split)
        .with_config("param1".to_string(), serde_json::Value::String("value1".to_string()))
        .with_config("param2".to_string(), serde_json::Value::Number(serde_json::Number::from(42)))
        .with_input_schema(json!({"type": "object", "properties": {"input": {"type": "string"}}}))
        .with_output_schema(json!({"type": "object", "properties": {"output": {"type": "string"}}}))
        .with_retry_config(RetryConfig::default())
        .with_timeout(300)
        .with_tags(vec!["test".to_string(), "example".to_string()]);

    assert_eq!(node.name, "TestNode");
    assert_eq!(node.description, "Test description");
    assert_eq!(node.config.len(), 2);
    assert_eq!(node.config.get("param1"), Some(&serde_json::Value::String("value1".to_string())));
    assert_eq!(node.config.get("param2"), Some(&serde_json::Value::Number(serde_json::Number::from(42))));
    assert!(node.input_schema.is_some());
    assert!(node.output_schema.is_some());
    assert_eq!(node.timeout_seconds, Some(300));
    assert_eq!(node.tags, vec!["test".to_string(), "example".to_string()]);
}

#[test]
fn test_workflow_edge_builder_methods() {
    let edge = WorkflowEdge::data_flow()
        .with_transform("x * 2".to_string())
        .with_metadata("priority".to_string(), serde_json::Value::Number(serde_json::Number::from(1)))
        .with_metadata("description".to_string(), serde_json::Value::String("test edge".to_string()));

    assert!(matches!(edge.edge_type, EdgeType::DataFlow));
    assert_eq!(edge.transform, Some("x * 2".to_string()));
    assert_eq!(edge.metadata.len(), 2);
    assert_eq!(edge.metadata.get("priority"), Some(&serde_json::Value::Number(serde_json::Number::from(1))));
    assert_eq!(edge.metadata.get("description"), Some(&serde_json::Value::String("test edge".to_string())));

    let control_edge = WorkflowEdge::control_flow();
    assert!(matches!(control_edge.edge_type, EdgeType::ControlFlow));
    assert!(control_edge.condition.is_none());
    assert!(control_edge.transform.is_none());

    let conditional_edge = WorkflowEdge::conditional("x > 10");
    assert!(matches!(conditional_edge.edge_type, EdgeType::Conditional));
    assert_eq!(conditional_edge.condition, Some("x > 10".to_string()));
}

#[test]
fn test_graph_validation_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Test empty graph validation
    graph.validate().unwrap();

    // Add nodes with different types
    let agent_node = WorkflowNode::new("Agent", "Agent node", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Execute task: {{input}}".to_string(),
    });
    let transform_node = WorkflowNode::new("Transform", "Transform node", NodeType::Transform {
        transformation: "x + 1".to_string(),
    });
    let condition_node = WorkflowNode::new("Condition", "Condition node", NodeType::Condition {
        expression: "x > 0".to_string(),
    });
    let delay_node = WorkflowNode::new("Delay", "Delay node", NodeType::Delay {
        duration_seconds: 5,
    });

    let agent_id = agent_node.id.clone();
    let transform_id = transform_node.id.clone();
    let condition_id = condition_node.id.clone();
    let delay_id = delay_node.id.clone();

    graph.add_node(agent_node).unwrap();
    graph.add_node(transform_node).unwrap();
    graph.add_node(condition_node).unwrap();
    graph.add_node(delay_node).unwrap();

    // Create a valid workflow
    graph.add_edge(agent_id.clone(), transform_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(transform_id.clone(), condition_id.clone(), WorkflowEdge::control_flow()).unwrap();
    graph.add_edge(condition_id.clone(), delay_id.clone(), WorkflowEdge::conditional("result == true")).unwrap();

    // Should validate successfully
    graph.validate().unwrap();

    // Test cycle detection by adding a cycle
    graph.add_edge(delay_id.clone(), agent_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Should fail validation due to cycle
    let result = graph.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cycle"));
}

#[test]
fn test_node_validation_edge_cases() {
    // Test agent node with valid agent_id
    let agent_node = WorkflowNode::new("Agent", "Agent node", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Execute: {{input}}".to_string(),
    });

    // Should validate successfully with valid agent_id
    agent_node.validate().unwrap();

    // Test transform node with empty transformation
    let transform_node = WorkflowNode::new("Transform", "Transform node", NodeType::Transform {
        transformation: "".to_string(),
    });

    let result = transform_node.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("transformation"));

    // Test condition node with empty expression
    let condition_node = WorkflowNode::new("Condition", "Condition node", NodeType::Condition {
        expression: "".to_string(),
    });

    let result = condition_node.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expression"));

    // Test delay node with zero duration (should validate successfully as there's no validation for this)
    let delay_node = WorkflowNode::new("Delay", "Delay node", NodeType::Delay {
        duration_seconds: 0,
    });

    // Delay nodes don't have duration validation, so this should pass
    delay_node.validate().unwrap();
}

#[test]
fn test_graph_ready_nodes_functionality() {
    let mut graph = WorkflowGraph::new();

    // Create a complex graph: A -> B -> D, A -> C -> D
    let node_a = WorkflowNode::new("A", "Node A", NodeType::Split);
    let node_b = WorkflowNode::new("B", "Node B", NodeType::Transform { transformation: "b_transform".to_string() });
    let node_c = WorkflowNode::new("C", "Node C", NodeType::Transform { transformation: "c_transform".to_string() });
    let node_d = WorkflowNode::new("D", "Node D", NodeType::Join);

    let id_a = node_a.id.clone();
    let id_b = node_b.id.clone();
    let id_c = node_c.id.clone();
    let id_d = node_d.id.clone();

    graph.add_node(node_a).unwrap();
    graph.add_node(node_b).unwrap();
    graph.add_node(node_c).unwrap();
    graph.add_node(node_d).unwrap();

    graph.add_edge(id_a.clone(), id_b.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(id_a.clone(), id_c.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(id_b.clone(), id_d.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(id_c.clone(), id_d.clone(), WorkflowEdge::data_flow()).unwrap();

    // Initially, only A should be ready (no dependencies)
    let mut completed = std::collections::HashSet::new();
    let running = std::collections::HashSet::new();
    let ready = graph.get_next_executable_nodes(&completed, &running);
    assert_eq!(ready.len(), 1);
    assert!(ready.contains(&id_a));

    // After A completes, B and C should be ready
    completed.insert(id_a.clone());
    let ready = graph.get_next_executable_nodes(&completed, &running);
    assert_eq!(ready.len(), 2);
    assert!(ready.contains(&id_b));
    assert!(ready.contains(&id_c));

    // After B completes, D should not be ready yet (still waiting for C)
    completed.insert(id_b.clone());
    let ready = graph.get_next_executable_nodes(&completed, &running);
    // D requires both B and C to be completed, so it should not be ready yet
    assert!(!ready.contains(&id_d));

    // After C completes, D should be ready
    completed.insert(id_c.clone());
    let ready = graph.get_next_executable_nodes(&completed, &running);
    assert_eq!(ready.len(), 1);
    assert!(ready.contains(&id_d));
}

#[test]
fn test_graph_add_duplicate_node() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("TestNode", "First node", NodeType::Split);
    let node1_id = node1.id.clone();

    // Add node successfully
    graph.add_node(node1).unwrap();
    assert_eq!(graph.node_count(), 1);

    // Try to add node with same ID
    let duplicate_node = WorkflowNode {
        id: node1_id.clone(),
        name: "DifferentName".to_string(),
        description: "Different description".to_string(),
        node_type: NodeType::Join,
        config: HashMap::new(),
        input_schema: None,
        output_schema: None,
        retry_config: RetryConfig::default(),
        timeout_seconds: None,
        tags: Vec::new(),
    };

    let result = graph.add_node(duplicate_node);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));
    assert_eq!(graph.node_count(), 1); // Should still be 1
}

#[test]
fn test_graph_add_edge_missing_nodes() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node1_id = node1.id.clone();
    let fake_node_id = NodeId::new();

    graph.add_node(node1).unwrap();

    // Try to add edge from existing node to non-existent node
    let result = graph.add_edge(node1_id.clone(), fake_node_id.clone(), WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Target node"));

    // Try to add edge from non-existent node to existing node
    let result = graph.add_edge(fake_node_id.clone(), node1_id.clone(), WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Source node"));
}

#[test]
fn test_graph_topological_sort_with_cycles() {
    let mut graph = WorkflowGraph::new();

    // Create nodes
    let node1 = WorkflowNode::new("Node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("Node2", "Second node", NodeType::Join);
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Create a cycle
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node2_id.clone(), node1_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Topological sort should fail
    let result = graph.topological_sort();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cycles"));

    // has_cycles should return true
    assert!(graph.has_cycles());
}

#[test]
fn test_graph_leaf_nodes_caching() {
    let mut graph = WorkflowGraph::new();

    // Create a linear chain: A -> B -> C
    let node_a = WorkflowNode::new("A", "Node A", NodeType::Split);
    let node_b = WorkflowNode::new("B", "Node B", NodeType::Transform { transformation: "test".to_string() });
    let node_c = WorkflowNode::new("C", "Node C", NodeType::Join);

    let id_a = node_a.id.clone();
    let id_b = node_b.id.clone();
    let id_c = node_c.id.clone();

    graph.add_node(node_a).unwrap();
    graph.add_node(node_b).unwrap();
    graph.add_node(node_c).unwrap();

    graph.add_edge(id_a.clone(), id_b.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(id_b.clone(), id_c.clone(), WorkflowEdge::data_flow()).unwrap();

    // Get leaf nodes (should be C)
    let leaves1 = graph.get_leaf_nodes();
    assert_eq!(leaves1.len(), 1);
    assert!(leaves1.contains(&id_c));

    // Get leaf nodes again (should use cache)
    let leaves2 = graph.get_leaf_nodes();
    assert_eq!(leaves1, leaves2);

    // Add another leaf node
    let node_d = WorkflowNode::new("D", "Node D", NodeType::Split);
    let id_d = node_d.id.clone();
    graph.add_node(node_d).unwrap();

    // Cache should be invalidated, now should have C and D as leaves
    let leaves3 = graph.get_leaf_nodes();
    assert_eq!(leaves3.len(), 2);
    assert!(leaves3.contains(&id_c));
    assert!(leaves3.contains(&id_d));
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
