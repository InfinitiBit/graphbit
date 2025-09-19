//! Advanced graph functionality unit tests
//!
//! Tests for complex graph operations, caching, serialization,
//! and edge cases not covered in basic graph tests.

use graphbit_core::{
    graph::{EdgeType, NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
    types::{AgentId, NodeId, RetryConfig},
};
use serde_json::json;
use std::collections::HashSet;
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

#[test]
fn test_workflow_graph_uncovered_functions_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Test get_edges method
    assert_eq!(graph.get_edges().len(), 0);

    // Add nodes and edges to test get_edges
    let node1 = WorkflowNode::new("node1", "Test node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "test prompt".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Test node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "test prompt".to_string(),
    });
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    let edge = WorkflowEdge::data_flow();
    graph.add_edge(node1_id.clone(), node2_id.clone(), edge).unwrap();

    // Test get_edges returns the edge
    let edges = graph.get_edges();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, node1_id);
    assert_eq!(edges[0].1, node2_id);

    // Test has_cycles method
    assert!(!graph.has_cycles());

    // Create a cycle to test has_cycles returns true
    let node3 = WorkflowNode::new("node3", "Test node 3", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "test prompt".to_string(),
    });
    let node3_id = node3.id.clone();
    graph.add_node(node3).unwrap();

    // Add edges to create a cycle: node1 -> node2 -> node3 -> node1
    graph.add_edge(node2_id.clone(), node3_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node3_id.clone(), node1_id.clone(), WorkflowEdge::data_flow()).unwrap();

    assert!(graph.has_cycles());
}

#[test]
fn test_workflow_graph_next_executable_nodes_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Create a complex graph structure
    let node1 = WorkflowNode::new("start", "Start node", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "start prompt".to_string(),
    });
    let node2 = WorkflowNode::new("middle1", "Middle node 1", NodeType::Transform {
        transformation: "transform1".to_string(),
    });
    let node3 = WorkflowNode::new("middle2", "Middle node 2", NodeType::Transform {
        transformation: "transform2".to_string(),
    });
    let node4 = WorkflowNode::new("end", "End node", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "end prompt".to_string(),
    });

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();
    let node3_id = node3.id.clone();
    let node4_id = node4.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();
    graph.add_node(node4).unwrap();

    // Create dependencies: node1 -> node2, node1 -> node3, node2 -> node4, node3 -> node4
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node1_id.clone(), node3_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node2_id.clone(), node4_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node3_id.clone(), node4_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test get_next_executable_nodes with empty completed set
    let completed = HashSet::new();
    let failed = HashSet::new();
    let executable = graph.get_next_executable_nodes(&completed, &failed);
    assert_eq!(executable.len(), 1);
    assert!(executable.contains(&node1_id));

    // Test with node1 completed
    let mut completed = HashSet::new();
    completed.insert(node1_id.clone());
    let executable = graph.get_next_executable_nodes(&completed, &failed);
    assert_eq!(executable.len(), 2);
    assert!(executable.contains(&node2_id));
    assert!(executable.contains(&node3_id));

    // Test with node1, node2, node3 completed
    completed.insert(node2_id.clone());
    completed.insert(node3_id.clone());
    let executable = graph.get_next_executable_nodes(&completed, &failed);
    assert_eq!(executable.len(), 1);
    assert!(executable.contains(&node4_id));

    // Test with all nodes completed
    completed.insert(node4_id.clone());
    let executable = graph.get_next_executable_nodes(&completed, &failed);
    assert_eq!(executable.len(), 0);
}

#[test]
fn test_workflow_graph_node_readiness_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Create nodes
    let node1 = WorkflowNode::new("node1", "Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt1".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt2".to_string(),
    });
    let node3 = WorkflowNode::new("node3", "Node 3", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt3".to_string(),
    });

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();
    let node3_id = node3.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Create dependencies: node1 -> node2 -> node3
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node2_id.clone(), node3_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test is_node_ready with empty completed set
    let completed = HashSet::new();
    assert!(graph.is_node_ready(&node1_id, &completed));
    assert!(!graph.is_node_ready(&node2_id, &completed));
    assert!(!graph.is_node_ready(&node3_id, &completed));

    // Test with node1 completed
    let mut completed = HashSet::new();
    completed.insert(node1_id.clone());
    assert!(graph.is_node_ready(&node1_id, &completed)); // Already completed
    assert!(graph.is_node_ready(&node2_id, &completed)); // Dependencies met
    assert!(!graph.is_node_ready(&node3_id, &completed)); // Dependencies not met

    // Test with node1 and node2 completed
    completed.insert(node2_id.clone());
    assert!(graph.is_node_ready(&node3_id, &completed)); // Dependencies met
}

#[test]
fn test_workflow_node_builder_comprehensive_coverage() {
    // Test all builder methods for comprehensive coverage
    let retry_config = RetryConfig::new(3);
    let input_schema = json!({"type": "object", "properties": {"input": {"type": "string"}}});
    let output_schema = json!({"type": "object", "properties": {"output": {"type": "string"}}});

    let node = WorkflowNode::new("test_node", "Test description", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "test prompt".to_string(),
    })
        .with_config("key1".to_string(), json!("value1"))
        .with_config("key2".to_string(), json!(42))
        .with_input_schema(input_schema.clone())
        .with_output_schema(output_schema.clone())
        .with_retry_config(retry_config.clone())
        .with_timeout(300)
        .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);

    // Verify all properties were set correctly
    assert_eq!(node.name, "test_node");
    assert_eq!(node.description, "Test description");
    assert!(matches!(node.node_type, NodeType::Agent { .. }));
    assert_eq!(node.config.get("key1"), Some(&json!("value1")));
    assert_eq!(node.config.get("key2"), Some(&json!(42)));
    assert_eq!(node.input_schema, Some(input_schema));
    assert_eq!(node.output_schema, Some(output_schema));
    assert_eq!(node.retry_config.max_attempts, 3);
    assert_eq!(node.timeout_seconds, Some(300));
    assert_eq!(node.tags, vec!["tag1".to_string(), "tag2".to_string()]);
}

#[test]
fn test_workflow_edge_builder_comprehensive_coverage() {
    // Test all edge builder methods
    let edge = WorkflowEdge::data_flow()
        .with_transform("transform_script".to_string())
        .with_metadata("priority".to_string(), json!("high"))
        .with_metadata("weight".to_string(), json!(0.8));

    assert!(matches!(edge.edge_type, EdgeType::DataFlow));
    assert_eq!(edge.transform, Some("transform_script".to_string()));
    assert_eq!(edge.metadata.get("priority"), Some(&json!("high")));
    assert_eq!(edge.metadata.get("weight"), Some(&json!(0.8)));

    // Test control flow edge
    let control_edge = WorkflowEdge::control_flow()
        .with_metadata("control_type".to_string(), json!("sequence"));

    assert!(matches!(control_edge.edge_type, EdgeType::ControlFlow));
    assert_eq!(control_edge.metadata.get("control_type"), Some(&json!("sequence")));

    // Test conditional edge
    let conditional_edge = WorkflowEdge::conditional("x > 10")
        .with_transform("filter_transform".to_string())
        .with_metadata("condition_type".to_string(), json!("numeric"));

    assert!(matches!(conditional_edge.edge_type, EdgeType::Conditional));
    assert_eq!(conditional_edge.condition, Some("x > 10".to_string()));
    assert_eq!(conditional_edge.transform, Some("filter_transform".to_string()));
    assert_eq!(conditional_edge.metadata.get("condition_type"), Some(&json!("numeric")));
}

#[test]
fn test_workflow_graph_caching_behavior() {
    let mut graph = WorkflowGraph::new();

    // Add nodes to test caching behavior
    let node1 = WorkflowNode::new("node1", "Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt1".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt2".to_string(),
    });
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test that caching methods work correctly by calling them multiple times
    let roots1 = graph.get_root_nodes();
    let roots2 = graph.get_root_nodes();
    assert_eq!(roots1, roots2);
    assert_eq!(roots1.len(), 1);
    assert!(roots1.contains(&node1_id));

    let leaves1 = graph.get_leaf_nodes();
    let leaves2 = graph.get_leaf_nodes();
    assert_eq!(leaves1, leaves2);
    assert_eq!(leaves1.len(), 1);
    assert!(leaves1.contains(&node2_id));

    let deps1 = graph.get_dependencies(&node2_id);
    let deps2 = graph.get_dependencies(&node2_id);
    assert_eq!(deps1, deps2);
    assert_eq!(deps1.len(), 1);
    assert!(deps1.contains(&node1_id));

    let dependents1 = graph.get_dependents(&node1_id);
    let dependents2 = graph.get_dependents(&node1_id);
    assert_eq!(dependents1, dependents2);
    assert_eq!(dependents1.len(), 1);
    assert!(dependents1.contains(&node2_id));
}

#[test]
fn test_workflow_graph_edge_count_and_node_count() {
    let mut graph = WorkflowGraph::new();

    // Test empty graph
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);

    // Add nodes
    let node1 = WorkflowNode::new("node1", "Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt1".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt2".to_string(),
    });
    let node3 = WorkflowNode::new("node3", "Node 3", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "prompt3".to_string(),
    });

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();
    let node3_id = node3.id.clone();

    graph.add_node(node1).unwrap();
    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 0);

    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 0);

    // Add edges
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 1);

    graph.add_edge(node2_id.clone(), node3_id.clone(), WorkflowEdge::control_flow()).unwrap();
    graph.add_edge(node1_id.clone(), node3_id.clone(), WorkflowEdge::conditional("test")).unwrap();
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);

    // Remove a node (should also remove associated edges)
    graph.remove_node(&node2_id).unwrap();
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1); // Only node1 -> node3 edge should remain
}

// Additional comprehensive tests for 100% coverage

#[test]
fn test_workflow_graph_default_implementation() {
    let graph = WorkflowGraph::default();

    // Verify default state
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(!graph.has_cycles());
    assert!(graph.get_nodes().is_empty());
    assert!(graph.get_edges().is_empty());
}

#[test]
fn test_workflow_graph_invalidate_caches() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Populate caches by calling methods
    let _deps = graph.get_dependencies(&node2_id);
    let _dependents = graph.get_dependents(&node1_id);
    let _roots = graph.get_root_nodes();
    let _leaves = graph.get_leaf_nodes();

    // Add another node to trigger cache invalidation
    let node3 = WorkflowNode::new("node3", "Third node", NodeType::Split);
    graph.add_node(node3).unwrap();

    // Verify caches are properly invalidated and rebuilt
    let roots = graph.get_root_nodes();
    assert_eq!(roots.len(), 2); // node1 and node3 are roots
}

#[test]
fn test_workflow_graph_metadata_operations() {
    let mut graph = WorkflowGraph::new();

    // Test setting and getting metadata
    graph.set_metadata("version".to_string(), serde_json::json!("1.0"));
    graph.set_metadata("author".to_string(), serde_json::json!("test"));

    assert_eq!(graph.get_metadata("version"), Some(&serde_json::json!("1.0")));
    assert_eq!(graph.get_metadata("author"), Some(&serde_json::json!("test")));
    assert_eq!(graph.get_metadata("nonexistent"), None);
}

#[test]
fn test_workflow_graph_node_id_by_name() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("unique_node", "First unique node", NodeType::Split);
    let node2 = WorkflowNode::new("another_node", "Another unique node", NodeType::Join);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Test finding nodes by name
    assert_eq!(graph.get_node_id_by_name("unique_node"), Some(node1_id));
    assert_eq!(graph.get_node_id_by_name("another_node"), Some(node2_id));
    assert_eq!(graph.get_node_id_by_name("nonexistent"), None);
}

#[test]
fn test_workflow_graph_edge_operations_comprehensive() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Test different edge types
    let data_edge = WorkflowEdge::data_flow();
    let control_edge = WorkflowEdge::control_flow();
    let conditional_edge = WorkflowEdge::conditional("x > 0");

    // Test edge with transform and metadata
    let complex_edge = WorkflowEdge::data_flow()
        .with_transform("json.parse")
        .with_metadata("priority".to_string(), serde_json::json!(1));

    graph.add_edge(node1_id.clone(), node2_id.clone(), data_edge).unwrap();

    // Verify edge was added
    assert_eq!(graph.edge_count(), 1);
    let edges = graph.get_edges();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, node1_id);
    assert_eq!(edges[0].1, node2_id);

    // Test edge types
    assert!(matches!(control_edge.edge_type, EdgeType::ControlFlow));
    assert!(matches!(conditional_edge.edge_type, EdgeType::Conditional));
    assert_eq!(conditional_edge.condition, Some("x > 0".to_string()));

    // Test complex edge properties
    assert_eq!(complex_edge.transform, Some("json.parse".to_string()));
    assert_eq!(complex_edge.metadata.get("priority"), Some(&serde_json::json!(1)));
}

#[test]
fn test_workflow_graph_remove_node_comprehensive() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);
    let node3 = WorkflowNode::new("node3", "Third node", NodeType::Split);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();
    let node3_id = node3.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Add edges
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node2_id.clone(), node3_id.clone(), WorkflowEdge::data_flow()).unwrap();

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);

    // Remove middle node
    graph.remove_node(&node2_id).unwrap();

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 0); // Both edges should be removed
    assert!(graph.get_node(&node2_id).is_none());

    // Test removing non-existent node
    let result = graph.remove_node(&NodeId::new());
    assert!(result.is_err());
}

#[test]
fn test_workflow_graph_validation_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Test empty graph validation
    assert!(graph.validate().is_ok());

    // Create nodes that form a cycle
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);
    let node3 = WorkflowNode::new("node3", "Third node", NodeType::Split);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();
    let node3_id = node3.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_node(node3).unwrap();

    // Create a cycle: node1 -> node2 -> node3 -> node1
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node2_id.clone(), node3_id.clone(), WorkflowEdge::data_flow()).unwrap();
    graph.add_edge(node3_id.clone(), node1_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Validation should fail due to cycle
    let result = graph.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cycles"));

    // Test has_cycles method
    assert!(graph.has_cycles());

    // Test topological_sort with cycles
    let topo_result = graph.topological_sort();
    assert!(topo_result.is_err());
    assert!(topo_result.unwrap_err().to_string().contains("cycles"));
}

#[test]
fn test_workflow_node_builder_comprehensive() {
    let agent_id = AgentId::new();
    let node_id = NodeId::new();

    // Test comprehensive node builder
    let mut node = WorkflowNode::new(
        "test_node",
        "A test node for comprehensive testing",
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "Hello {{input}}".to_string(),
        },
    );

    // Set the node ID manually for testing
    node.id = node_id.clone();

    node = node
    .with_config("temperature".to_string(), serde_json::json!(0.7))
    .with_config("max_tokens".to_string(), serde_json::json!(100))
    .with_input_schema(serde_json::json!({
        "type": "object",
        "properties": {
            "input": {"type": "string"}
        }
    }))
    .with_output_schema(serde_json::json!({
        "type": "object",
        "properties": {
            "output": {"type": "string"}
        }
    }))
    .with_retry_config(RetryConfig::new(3).with_exponential_backoff(1000, 2.0, 10000))
    .with_timeout(30)
    .with_tags(vec!["ai".to_string(), "agent".to_string()]);

    // Verify all properties were set
    assert_eq!(node.id, node_id);
    assert_eq!(node.name, "test_node");
    assert!(matches!(node.node_type, NodeType::Agent { .. }));
    assert_eq!(node.config.get("temperature"), Some(&serde_json::json!(0.7)));
    assert_eq!(node.config.get("max_tokens"), Some(&serde_json::json!(100)));
    assert!(node.input_schema.is_some());
    assert!(node.output_schema.is_some());
    assert_eq!(node.retry_config.max_attempts, 3);
    assert_eq!(node.timeout_seconds, Some(30));
    assert_eq!(node.tags, vec!["ai".to_string(), "agent".to_string()]);

    // Test validation
    assert!(node.validate().is_ok());
}

#[test]
fn test_workflow_node_validation_comprehensive() {
    let agent_id = AgentId::new();
    let node_id = NodeId::new();

    // Test valid agent node
    let valid_agent_node = WorkflowNode::new(
        "agent_node",
        "A valid agent node",
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "Hello".to_string(),
        },
    );
    assert!(valid_agent_node.validate().is_ok());

    // Agent nodes with valid UUIDs are always valid, so test a different validation case
    // Test that agent node validation passes for valid configurations
    assert!(valid_agent_node.validate().is_ok());

    // Test HTTP request node validation
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let valid_http_node = WorkflowNode::new(
        "http_node",
        "A valid HTTP request node",
        NodeType::HttpRequest {
            url: "https://api.example.com".to_string(),
            method: "POST".to_string(),
            headers: headers.clone(),
        },
    );
    assert!(valid_http_node.validate().is_ok());

    // HTTP nodes don't have URL validation in the current implementation
    let http_node_any_url = WorkflowNode::new(
        "http_node",
        "An HTTP request node with any URL",
        NodeType::HttpRequest {
            url: "not-a-url".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
        },
    );
    assert!(http_node_any_url.validate().is_ok());

    // Test transform node validation
    let valid_transform_node = WorkflowNode::new(
        "transform_node",
        "A valid transform node",
        NodeType::Transform {
            transformation: "data.map(x => x * 2)".to_string(),
        },
    );
    assert!(valid_transform_node.validate().is_ok());

    let invalid_transform_node = WorkflowNode::new(
        "transform_node",
        "An invalid transform node",
        NodeType::Transform {
            transformation: "".to_string(),
        },
    );
    let result = invalid_transform_node.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("transformation"));

    // Test condition node validation
    let valid_condition_node = WorkflowNode::new(
        "condition_node",
        "A valid condition node",
        NodeType::Condition {
            expression: "value > 10".to_string(),
        },
    );
    assert!(valid_condition_node.validate().is_ok());

    let invalid_condition_node = WorkflowNode::new(
        "condition_node",
        "An invalid condition node",
        NodeType::Condition {
            expression: "".to_string(),
        },
    );
    let result = invalid_condition_node.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expression"));
}

#[test]
fn test_workflow_node_validation_additional_types() {
    let node_id = NodeId::new();

    // Test delay node validation
    let valid_delay_node = WorkflowNode::new(
        "delay_node",
        "A valid delay node",
        NodeType::Delay {
            duration_seconds: 5,
        },
    );
    assert!(valid_delay_node.validate().is_ok());

    // Delay nodes don't have specific validation, so test that they pass
    let delay_node_zero = WorkflowNode::new(
        "delay_node",
        "A delay node with zero duration",
        NodeType::Delay {
            duration_seconds: 0,
        },
    );
    assert!(delay_node_zero.validate().is_ok());

    // Test document loader node validation
    let valid_doc_node = WorkflowNode::new(
        "doc_node",
        "A valid document loader node",
        NodeType::DocumentLoader {
            document_type: "pdf".to_string(),
            source_path: "/path/to/document.pdf".to_string(),
            encoding: Some("utf-8".to_string()),
        },
    );
    assert!(valid_doc_node.validate().is_ok());

    let invalid_doc_node = WorkflowNode::new(
        "doc_node",
        "An invalid document loader node",
        NodeType::DocumentLoader {
            document_type: "unsupported_type".to_string(),
            source_path: "/path/to/document.pdf".to_string(),
            encoding: None,
        },
    );
    let result = invalid_doc_node.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported document type"));

    let invalid_doc_node2 = WorkflowNode::new(
        "doc_node",
        "Another invalid document loader node",
        NodeType::DocumentLoader {
            document_type: "pdf".to_string(),
            source_path: "".to_string(),
            encoding: None,
        },
    );
    let result2 = invalid_doc_node2.validate();
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("source_path"));

    // Test custom node validation
    let valid_custom_node = WorkflowNode::new(
        "custom_node",
        "A valid custom node",
        NodeType::Custom {
            function_name: "my_function".to_string(),
        },
    );
    assert!(valid_custom_node.validate().is_ok());

    // Custom nodes don't have specific validation, so test that they pass
    let custom_node_empty = WorkflowNode::new(
        "custom_node",
        "A custom node with empty function name",
        NodeType::Custom {
            function_name: "".to_string(),
        },
    );
    assert!(custom_node_empty.validate().is_ok());

    // Test Split and Join nodes (should always be valid)
    let split_node = WorkflowNode::new(
        "split_node",
        "A split node",
        NodeType::Split,
    );
    assert!(split_node.validate().is_ok());

    let join_node = WorkflowNode::new(
        "join_node",
        "A join node",
        NodeType::Join,
    );
    assert!(join_node.validate().is_ok());
}

#[test]
fn test_workflow_graph_serialization_deserialization() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();
    graph.add_edge(node1_id.clone(), node2_id.clone(), WorkflowEdge::data_flow()).unwrap();

    // Test serialization and deserialization
    let serialized = serde_json::to_string(&graph).unwrap();
    let mut deserialized: WorkflowGraph = serde_json::from_str(&serialized).unwrap();

    // Rebuild the graph after deserialization
    deserialized.rebuild_graph().unwrap();

    // Verify the deserialized graph is functional
    assert_eq!(deserialized.node_count(), 2);
    assert_eq!(deserialized.edge_count(), 1);
    assert!(!deserialized.has_cycles());

    let topo_sort = deserialized.topological_sort().unwrap();
    assert_eq!(topo_sort.len(), 2);

    // Verify nodes exist
    assert!(deserialized.get_node(&node1_id).is_some());
    assert!(deserialized.get_node(&node2_id).is_some());
}

#[test]
fn test_workflow_graph_add_edge_error_cases() {
    let mut graph = WorkflowGraph::new();
    let node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let node1_id = node1.id.clone();

    graph.add_node(node1).unwrap();

    let nonexistent_id = NodeId::new();

    // Test adding edge with non-existent from node
    let result = graph.add_edge(nonexistent_id.clone(), node1_id.clone(), WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));

    // Test adding edge with non-existent to node
    let result = graph.add_edge(node1_id.clone(), nonexistent_id.clone(), WorkflowEdge::data_flow());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_workflow_graph_duplicate_node_error() {
    let mut graph = WorkflowGraph::new();
    let node_id = NodeId::new();
    let mut node1 = WorkflowNode::new("node1", "First node", NodeType::Split);
    let mut node2 = WorkflowNode::new("node2", "Second node", NodeType::Join);

    // Set the same ID for both nodes
    node1.id = node_id.clone();
    node2.id = node_id.clone();

    // Add first node
    assert!(graph.add_node(node1).is_ok());

    // Try to add second node with same ID
    let result = graph.add_node(node2);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));
}

#[test]
fn test_workflow_graph_get_edges_accessor_method() {
    let mut graph = WorkflowGraph::new();

    // Create test nodes
    let node1 = WorkflowNode::new("node1", "Test Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 1".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Test Node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 2".to_string(),
    });
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    // Add nodes to graph
    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Initially no edges
    let edges = graph.get_edges();
    assert_eq!(edges.len(), 0);

    // Add an edge
    let edge = WorkflowEdge::data_flow();
    graph.add_edge(node1_id.clone(), node2_id.clone(), edge).unwrap();

    // Test get_edges() method - this covers the uncovered lines 183-185
    let edges = graph.get_edges();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, node1_id);
    assert_eq!(edges[0].1, node2_id);
    // Check edge type by matching instead of equality since EdgeType doesn't implement PartialEq
    match &edges[0].2.edge_type {
        EdgeType::DataFlow => {}, // Expected
        _ => panic!("Expected DataFlow edge type"),
    }

    // Add another edge
    let conditional_edge = WorkflowEdge::conditional("test_condition");
    let node3 = WorkflowNode::new("node3", "Test Node 3", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 3".to_string(),
    });
    let node3_id = node3.id.clone();
    graph.add_node(node3).unwrap();
    graph.add_edge(node2_id.clone(), node3_id.clone(), conditional_edge).unwrap();

    // Verify multiple edges
    let edges = graph.get_edges();
    assert_eq!(edges.len(), 2);

    // Verify edge details by checking each edge type
    let mut has_data_flow = false;
    let mut has_conditional = false;
    for (_, _, edge) in edges {
        match edge.edge_type {
            EdgeType::DataFlow => has_data_flow = true,
            EdgeType::Conditional => has_conditional = true,
            _ => {},
        }
    }
    assert!(has_data_flow, "Should have DataFlow edge");
    assert!(has_conditional, "Should have Conditional edge");
}

#[test]
fn test_workflow_graph_validation_non_existent_nodes() {
    let mut graph = WorkflowGraph::new();

    // Create a valid node
    let node1 = WorkflowNode::new("node1", "Test Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 1".to_string(),
    });
    let node1_id = node1.id.clone();
    graph.add_node(node1).unwrap();

    // Create fake node IDs that don't exist in the graph
    let fake_node_id1 = NodeId::new();
    let fake_node_id2 = NodeId::new();

    // Test validation by trying to add edges with non-existent nodes
    // Since edges field is private, we'll test through the add_edge method
    let edge = WorkflowEdge::data_flow();

    // Try to add edge with non-existent source node - should trigger error
    let result = graph.add_edge(fake_node_id1.clone(), node1_id.clone(), edge.clone());
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("not found") || error.to_string().contains("does not exist"));

    // Try to add edge with non-existent target node - should trigger error
    let result = graph.add_edge(node1_id.clone(), fake_node_id2.clone(), edge);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("not found") || error.to_string().contains("does not exist"));
}

#[test]
fn test_workflow_graph_edge_validation_comprehensive() {
    let mut graph = WorkflowGraph::new();

    // Test validation with empty graph
    let result = graph.validate();
    assert!(result.is_ok()); // Empty graph should be valid

    // Create nodes
    let node1 = WorkflowNode::new("node1", "Test Node 1", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 1".to_string(),
    });
    let node2 = WorkflowNode::new("node2", "Test Node 2", NodeType::Agent {
        agent_id: AgentId::new(),
        prompt_template: "Test prompt 2".to_string(),
    });
    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    // Add valid edge
    let edge = WorkflowEdge::control_flow();
    graph.add_edge(node1_id.clone(), node2_id.clone(), edge).unwrap();

    // Graph should be valid
    let result = graph.validate();
    assert!(result.is_ok());

    // Test cycle detection by adding a back edge
    let back_edge = WorkflowEdge::data_flow();
    graph.add_edge(node2_id.clone(), node1_id.clone(), back_edge).unwrap();

    // Graph should now have a cycle and be invalid
    let result = graph.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = error.to_string();
    println!("Cycle detection error: {}", error_msg);
    // Check for various possible cycle error messages
    assert!(error_msg.contains("cycle") ||
            error_msg.contains("circular") ||
            error_msg.contains("dependency") ||
            error_msg.contains("loop"));
}
