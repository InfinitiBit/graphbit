//! Error handling integration tests
//!
//! Tests for real error handling scenarios in GraphBit,
//! focusing on validation and actual error conditions.

use graphbit_core::*;

#[tokio::test]
async fn test_workflow_validation_errors() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test duplicate node IDs
    let node1 = graph::WorkflowNode::new(
        "Node 1",
        "First node",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node2 = graph::WorkflowNode::new(
        "Node 2",
        "Second node with same ID",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let mut workflow = workflow::Workflow::new("Duplicate ID Workflow", "Testing duplicate IDs");
    workflow.add_node(node1).expect("Failed to add first node");

    // Manually set the same ID to test duplicate detection
    let mut node2_with_same_id = node2;
    node2_with_same_id.id = workflow.graph.get_nodes().keys().next().unwrap().clone();

    let result = workflow.add_node(node2_with_same_id);
    assert!(result.is_err(), "Should fail to add node with duplicate ID");

    // Test edge to nonexistent node
    let valid_node = graph::WorkflowNode::new(
        "Valid Node",
        "A valid node",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let mut workflow2 = workflow::Workflow::new("Invalid Edge Workflow", "Testing invalid edges");
    workflow2
        .add_node(valid_node)
        .expect("Failed to add valid node");

    let edge = graph::WorkflowEdge::data_flow();
    let valid_id = workflow2.graph.get_nodes().keys().next().unwrap().clone();
    let nonexistent_id = types::NodeId::new();

    let result = workflow2.connect_nodes(valid_id, nonexistent_id, edge);
    assert!(
        result.is_err(),
        "Should fail to connect to nonexistent node"
    );
}

#[tokio::test]
async fn test_circular_dependency_detection() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let node1 = graph::WorkflowNode::new(
        "Node 1",
        "First node",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node2 = graph::WorkflowNode::new(
        "Node 2",
        "Second node",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node3 = graph::WorkflowNode::new(
        "Node 3",
        "Third node",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let mut workflow =
        workflow::Workflow::new("Circular Workflow", "Testing circular dependencies");

    workflow.add_node(node1).expect("Failed to add node1");
    workflow.add_node(node2).expect("Failed to add node2");
    workflow.add_node(node3).expect("Failed to add node3");

    let node_ids: Vec<_> = workflow.graph.get_nodes().keys().cloned().collect();
    let node1_id = &node_ids[0];
    let node2_id = &node_ids[1];
    let node3_id = &node_ids[2];

    // Create circular dependency: 1 -> 2 -> 3 -> 1
    workflow
        .connect_nodes(
            node1_id.clone(),
            node2_id.clone(),
            graph::WorkflowEdge::data_flow(),
        )
        .expect("Failed to connect node1 to node2");
    workflow
        .connect_nodes(
            node2_id.clone(),
            node3_id.clone(),
            graph::WorkflowEdge::data_flow(),
        )
        .expect("Failed to connect node2 to node3");
    workflow
        .connect_nodes(
            node3_id.clone(),
            node1_id.clone(),
            graph::WorkflowEdge::data_flow(),
        )
        .expect("Failed to connect node3 to node1");

    let result = workflow.validate();
    assert!(
        result.is_err(),
        "Should fail to validate workflow with circular dependencies"
    );
}

#[tokio::test]
async fn test_invalid_configuration_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test with invalid LLM configuration
    let invalid_llm_config = llm::LlmConfig::openai("", "");

    let result = agents::AgentConfig::new(
        "Test Agent",
        "An agent with invalid config",
        invalid_llm_config,
    );

    // Should not panic, but the config should have empty values
    assert_eq!(result.name, "Test Agent");
    assert_eq!(result.description, "An agent with invalid config");
}

#[tokio::test]
async fn test_error_message_formatting() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test different error types have proper formatting
    let graph_error = errors::GraphBitError::graph("Test graph error");
    assert!(graph_error.to_string().contains("Test graph error"));

    let validation_error = errors::GraphBitError::validation("field", "Test validation error");
    assert!(validation_error
        .to_string()
        .contains("Test validation error"));

    let io_error = errors::GraphBitError::Io {
        message: "File not found".to_string(),
    };
    assert!(io_error.to_string().contains("File not found"));
}

#[tokio::test]
async fn test_error_propagation_chain() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test that errors propagate correctly through the system
    let node = graph::WorkflowNode::new(
        "Error Node",
        "A node that will cause errors",
        graph::NodeType::Agent {
            agent_id: types::AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let mut workflow = workflow::Workflow::new("Error Workflow", "Testing error propagation");
    workflow.add_node(node).expect("Failed to add node");

    // Add an edge to a nonexistent node to trigger validation error
    let node_ids: Vec<_> = workflow.graph.get_nodes().keys().cloned().collect();
    let existing_id = &node_ids[0];
    let nonexistent_id = types::NodeId::new();

    let result = workflow.connect_nodes(
        existing_id.clone(),
        nonexistent_id,
        graph::WorkflowEdge::data_flow(),
    );

    assert!(
        result.is_err(),
        "Should fail when connecting to nonexistent node"
    );
}

#[tokio::test]
async fn test_agent_config_validation() {
    let llm_config = llm::LlmConfig::openai(super::get_test_api_key(), super::get_test_model());

    let result = agents::AgentConfig::new("Test Agent", "An agent with invalid config", llm_config);

    assert_eq!(result.name, "Test Agent");
    assert_eq!(result.description, "An agent with invalid config");
}

#[tokio::test]
async fn test_error_conversion() {
    // Test validation error with both field and message parameters
    let validation_error = errors::GraphBitError::validation("field", "Test validation error");
    assert!(matches!(
        validation_error,
        errors::GraphBitError::Validation { .. }
    ));

    // Test IO error using the Io variant
    let io_error = errors::GraphBitError::Io {
        message: "File not found".to_string(),
    };
    assert!(matches!(io_error, errors::GraphBitError::Io { .. }));
}
