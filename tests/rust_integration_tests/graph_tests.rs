//! Graph integration tests
//!
//! Tests for workflow graph functionality including node management,
//! edge creation, graph validation, and traversal algorithms.

use graphbit_core::{graph::NodeType, types::AgentId, WorkflowEdge, WorkflowGraph, WorkflowNode};

#[tokio::test]
async fn test_create_empty_graph() {
    let graph = WorkflowGraph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[tokio::test]
async fn test_add_single_node() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "test-node",
        "A test node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node_id = node.id.clone();
    graph.add_node(node).unwrap();

    assert_eq!(graph.node_count(), 1);
    assert!(graph.get_node(&node_id).is_some());
}

#[tokio::test]
async fn test_add_multiple_nodes() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "First prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    assert_eq!(graph.node_count(), 2);
    assert!(graph.get_node(&node1_id).is_some());
    assert!(graph.get_node(&node2_id).is_some());
}

#[tokio::test]
async fn test_duplicate_node_id() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "Test Node",
        "A test node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    // Create another node with same ID
    let mut duplicate_node = node.clone();
    duplicate_node.name = "Duplicate Node".to_string();

    graph.add_node(node).unwrap();
    let result = graph.add_node(duplicate_node);

    assert!(result.is_err());
    assert_eq!(graph.node_count(), 1);
}

#[tokio::test]
async fn test_remove_node() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "Existing Node",
        "A node that exists",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node_id = node.id.clone();
    graph.add_node(node).unwrap();

    assert_eq!(graph.node_count(), 1);

    graph.remove_node(&node_id).unwrap();
    assert_eq!(graph.node_count(), 0);
    assert!(graph.get_node(&node_id).is_none());
}

#[tokio::test]
async fn test_add_edge() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "First prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    let edge = WorkflowEdge::data_flow();

    graph.add_edge(node1_id, node2_id, edge).unwrap();

    assert_eq!(graph.edge_count(), 1);
}

#[tokio::test]
async fn test_get_single_node_no_edges() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "Single Node",
        "A single node graph",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node_id = node.id.clone();
    graph.add_node(node).unwrap();

    let retrieved_node = graph.get_node(&node_id).unwrap();
    assert_eq!(retrieved_node.name, "Single Node");
    assert_eq!(retrieved_node.description, "A single node graph");
}

#[tokio::test]
async fn test_get_multiple_nodes_with_edges() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "First prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    let edge = WorkflowEdge::data_flow();
    graph
        .add_edge(node1_id.clone(), node2_id.clone(), edge)
        .unwrap();

    let nodes = graph.get_nodes();
    assert_eq!(nodes.len(), 2);
    assert!(nodes.contains_key(&node1_id));
    assert!(nodes.contains_key(&node2_id));
}

#[tokio::test]
async fn test_large_graph() {
    let mut graph = WorkflowGraph::new();

    // Add 100 nodes
    let mut node_ids = Vec::new();
    for i in 0..100 {
        let node = WorkflowNode::new(
            format!("Node {i}"),
            format!("Node {i} description"),
            NodeType::Agent {
                agent_id: AgentId::new(),
                prompt_template: format!("Prompt {i}"),
            },
        );

        node_ids.push(node.id.clone());
        graph.add_node(node).unwrap();
    }

    // Add edges between consecutive nodes
    for i in 0..99 {
        let edge = WorkflowEdge::data_flow();
        graph
            .add_edge(node_ids[i].clone(), node_ids[i + 1].clone(), edge)
            .unwrap();
    }

    assert_eq!(graph.node_count(), 100);
    assert_eq!(graph.edge_count(), 99);
}

#[tokio::test]
async fn test_self_loop() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "Self Loop Node",
        "A node with self loop",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Self loop prompt".to_string(),
        },
    );

    let node_id = node.id.clone();
    graph.add_node(node).unwrap();

    let edge = WorkflowEdge::conditional("self_loop".to_string());

    graph
        .add_edge(node_id.clone(), node_id.clone(), edge)
        .unwrap();

    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.edge_count(), 1);
}

#[tokio::test]
async fn test_complex_graph() {
    let mut graph = WorkflowGraph::new();

    // Create nodes
    let names = ["Input", "Process", "Filter", "Output"];
    let mut node_ids = Vec::new();

    for (i, _name) in names.iter().enumerate() {
        let node = WorkflowNode::new(
            format!("Node {}", names[i]),
            format!("Node {} description", names[i]),
            NodeType::Agent {
                agent_id: AgentId::new(),
                prompt_template: format!("Prompt for {}", names[i]),
            },
        );

        node_ids.push(node.id.clone());
        graph.add_node(node).unwrap();
    }

    // Create a complex edge pattern
    for i in 0..3 {
        let edge = WorkflowEdge::data_flow();
        graph
            .add_edge(node_ids[i].clone(), node_ids[i + 1].clone(), edge)
            .unwrap();
    }

    // Add a conditional edge back from Filter to Process
    let edge1 = WorkflowEdge::conditional("retry".to_string());
    graph
        .add_edge(node_ids[2].clone(), node_ids[1].clone(), edge1)
        .unwrap();

    // Add control flow edge from Input to Output (bypass)
    let edge2 = WorkflowEdge::control_flow();
    graph
        .add_edge(node_ids[0].clone(), node_ids[3].clone(), edge2)
        .unwrap();

    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 5);
}

#[tokio::test]
async fn test_node_with_config() {
    let mut graph = WorkflowGraph::new();

    let node = WorkflowNode::new(
        "Configured Node",
        "A node with configuration",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    )
    .with_config("priority".to_string(), serde_json::json!(1))
    .with_config("category".to_string(), serde_json::json!("processing"));

    let node_id = node.id.clone();
    graph.add_node(node).unwrap();

    let retrieved_node = graph.get_node(&node_id).unwrap();
    assert_eq!(
        retrieved_node.config.get("priority"),
        Some(&serde_json::json!(1))
    );
    assert_eq!(
        retrieved_node.config.get("category"),
        Some(&serde_json::json!("processing"))
    );
}

#[tokio::test]
async fn test_edge_with_metadata() {
    let mut graph = WorkflowGraph::new();

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "First prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let node1_id = node1.id.clone();
    let node2_id = node2.id.clone();

    graph.add_node(node1).unwrap();
    graph.add_node(node2).unwrap();

    let edge = WorkflowEdge::data_flow()
        .with_metadata("weight".to_string(), serde_json::json!(0.8))
        .with_metadata("type".to_string(), serde_json::json!("priority"));

    graph.add_edge(node1_id, node2_id, edge).unwrap();

    let edges = graph.get_edges();
    assert_eq!(edges.len(), 1);

    let edge = &edges[0].2;
    assert_eq!(edge.metadata.get("weight"), Some(&serde_json::json!(0.8)));
    assert_eq!(
        edge.metadata.get("type"),
        Some(&serde_json::json!("priority"))
    );
}
