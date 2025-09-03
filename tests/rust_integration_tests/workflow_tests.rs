// Workflow integration tests
//
// Tests for workflow functionality, structure validation, and execution flows
// without using simulated agents.

use graphbit_core::{
    errors::GraphBitResult, graph::NodeType, llm::LlmConfig, types::AgentId, AgentConfig,
    WorkflowBuilder, WorkflowEdge, WorkflowNode,
};

#[test]
fn test_agent_id_uniqueness() {
    use graphbit_core::types::AgentId;
    let id1 = AgentId::new();
    let id2 = AgentId::new();
    assert_ne!(
        id1, id2,
        "AgentId::new() did not generate unique IDs. This is a bug."
    );
}

#[tokio::test]
async fn test_simple_workflow_creation() -> GraphBitResult<()> {
    let agent_id = AgentId::new();

    // Create agent config
    let llm_config = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let agent_config = AgentConfig {
        id: agent_id,
        name: "Agent 1".to_string(),
        description: "First agent".to_string(),
        capabilities: vec![],
        system_prompt: "Simple prompt".to_string(),
        llm_config,
        max_tokens: None,
        temperature: None,
        custom_config: std::collections::HashMap::with_capacity(4),
    };

    // Create workflow node
    let node = WorkflowNode::new(
        "Agent 1",
        "First agent",
        NodeType::Agent {
            agent_id: agent_config.id.clone(),
            prompt_template: agent_config.system_prompt.clone(),
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Simple Workflow").add_node(node)?;

    let workflow = builder.build()?;
    assert_eq!(workflow.name, "Simple Workflow");
    Ok(())
}

#[tokio::test]
async fn test_multi_node_workflow() -> GraphBitResult<()> {
    let agent_id1 = AgentId::new();
    let agent_id2 = AgentId::new();

    // Create first agent config
    let llm_config1 = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let agent_config1 = AgentConfig {
        id: agent_id1,
        name: "Agent 1".to_string(),
        description: "First agent".to_string(),
        capabilities: vec![],
        system_prompt: "First prompt".to_string(),
        llm_config: llm_config1,
        max_tokens: None,
        temperature: None,
        custom_config: std::collections::HashMap::with_capacity(4),
    };

    // Create second agent config
    let llm_config2 = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let agent_config2 = AgentConfig {
        id: agent_id2,
        name: "Agent 2".to_string(),
        description: "Second agent".to_string(),
        capabilities: vec![],
        system_prompt: "Second prompt".to_string(),
        llm_config: llm_config2,
        max_tokens: None,
        temperature: None,
        custom_config: std::collections::HashMap::with_capacity(4),
    };

    // Create workflow nodes
    let node1 = WorkflowNode::new(
        "Agent 1",
        "First agent",
        NodeType::Agent {
            agent_id: agent_config1.id.clone(),
            prompt_template: agent_config1.system_prompt.clone(),
        },
    );

    let node2 = WorkflowNode::new(
        "Agent 2",
        "Second agent",
        NodeType::Agent {
            agent_id: agent_config2.id.clone(),
            prompt_template: agent_config2.system_prompt.clone(),
        },
    );

    let (builder, _node1_id) = WorkflowBuilder::new("Multi-Node Workflow").add_node(node1)?;

    let (builder, _node2_id) = builder.add_node(node2)?;

    let workflow = builder.build()?;
    assert_eq!(workflow.name, "Multi-Node Workflow");
    Ok(())
}

#[tokio::test]
async fn test_workflow_with_connections() -> GraphBitResult<()> {
    let agent_id1 = AgentId::new();
    let agent_id2 = AgentId::new();

    // Create first agent config
    let llm_config1 = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let agent_config1 = AgentConfig {
        id: agent_id1,
        name: "Agent 1".to_string(),
        description: "First agent".to_string(),
        capabilities: vec![],
        system_prompt: "First prompt".to_string(),
        llm_config: llm_config1,
        max_tokens: None,
        temperature: None,
        custom_config: std::collections::HashMap::with_capacity(4),
    };

    // Create second agent config
    let llm_config2 = LlmConfig::openai(super::get_test_api_key(), super::get_test_model());
    let agent_config2 = AgentConfig {
        id: agent_id2,
        name: "Agent 2".to_string(),
        description: "Second agent".to_string(),
        capabilities: vec![],
        system_prompt: "Second prompt".to_string(),
        llm_config: llm_config2,
        max_tokens: None,
        temperature: None,
        custom_config: std::collections::HashMap::with_capacity(4),
    };

    // Create workflow nodes
    let node1 = WorkflowNode::new(
        "Agent 1",
        "First agent",
        NodeType::Agent {
            agent_id: agent_config1.id.clone(),
            prompt_template: agent_config1.system_prompt.clone(),
        },
    );

    let node2 = WorkflowNode::new(
        "Agent 2",
        "Second agent",
        NodeType::Agent {
            agent_id: agent_config2.id.clone(),
            prompt_template: agent_config2.system_prompt.clone(),
        },
    );

    let (builder, node1_id) = WorkflowBuilder::new("Connected Workflow").add_node(node1)?;

    let (builder, node2_id) = builder.add_node(node2)?;

    let edge = WorkflowEdge::data_flow();
    let builder = builder.connect(node1_id, node2_id, edge)?;

    let workflow = builder.build()?;
    assert_eq!(workflow.name, "Connected Workflow");
    Ok(())
}

#[tokio::test]
async fn test_workflow_complex_graph() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let _agent_id1 = AgentId::new();
    let _agent_id2 = AgentId::new();


    // Create start node
    let start_node = WorkflowNode::new(
        "Start",
        "Starting node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Start processing".to_string(),
        },
    );

    // Create left branch node
    let left_node = WorkflowNode::new(
        "Left",
        "Left branch",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Process left branch".to_string(),
        },
    );

    // Create right branch node
    let right_node = WorkflowNode::new(
        "Right",
        "Right branch",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Process right branch".to_string(),
        },
    );

    // Create end node
    let end_node = WorkflowNode::new(
        "End",
        "Merging node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Merge results".to_string(),
        },
    );

    // Build complex workflow
    let (builder, start_id) = WorkflowBuilder::new("Complex Workflow")
        .add_node(start_node)
        .expect("Failed to add start node");

    let (builder, left_id) = builder
        .add_node(left_node)
        .expect("Failed to add left node");

    let (builder, right_id) = builder
        .add_node(right_node)
        .expect("Failed to add right node");

    let (builder, end_id) = builder.add_node(end_node).expect("Failed to add end node");

    // Create diamond pattern: start -> left/right -> end
    let edge = WorkflowEdge::data_flow();

    let builder = builder
        .connect(start_id.clone(), left_id.clone(), edge.clone())
        .expect("Failed to connect start to left")
        .connect(start_id, right_id.clone(), edge.clone())
        .expect("Failed to connect start to right")
        .connect(left_id, end_id.clone(), edge.clone())
        .expect("Failed to connect left to end")
        .connect(right_id, end_id, edge)
        .expect("Failed to connect right to end");

    let workflow = builder.build().expect("Failed to build workflow");

    // Verify the diamond structure
    assert_eq!(workflow.graph.get_nodes().len(), 4);
    assert_eq!(workflow.graph.get_edges().len(), 4);
}

#[tokio::test]
async fn test_workflow_metadata_preservation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let (builder, node1_id) = WorkflowBuilder::new("Metadata Workflow")
        .add_node(node1)
        .expect("Failed to add first node");

    let (builder, node2_id) = builder.add_node(node2).expect("Failed to add second node");

    let edge = WorkflowEdge::data_flow()
        .with_metadata("test_key".to_string(), serde_json::json!("test_value"));

    let builder = builder
        .connect(node1_id, node2_id, edge)
        .expect("Failed to connect nodes");

    let workflow = builder.build().expect("Failed to build workflow");

    // Verify metadata is preserved
    let edges = workflow.graph.get_edges();
    assert_eq!(edges.len(), 1);
    assert_eq!(
        edges[0].2.metadata.get("test_key"),
        Some(&serde_json::json!("test_value"))
    );
}

#[tokio::test]
async fn test_workflow_validation_cycles() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let node1 = WorkflowNode::new(
        "Node 1",
        "First node",
        NodeType::Agent {
            agent_id: AgentId::new(),
            prompt_template: "Test prompt".to_string(),
        },
    );

    let node2 = WorkflowNode::new(
        "Node 2",
        "Second node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let (builder, node1_id) = WorkflowBuilder::new("Cycle Workflow")
        .add_node(node1)
        .expect("Failed to add first node");

    let (builder, node2_id) = builder.add_node(node2).expect("Failed to add second node");

    let edge = WorkflowEdge::data_flow();

    // Create cycle: node1 -> node2 -> node1
    let builder = builder
        .connect(node1_id.clone(), node2_id.clone(), edge.clone())
        .expect("Failed to connect node1 to node2")
        .connect(node2_id, node1_id, edge)
        .expect("Failed to connect node2 to node1");

    let workflow_result = builder.build();

    // Workflow builder should detect cycles during build
    assert!(
        workflow_result.is_err(),
        "Workflow with cycles should fail to build"
    );
}

#[tokio::test]
async fn test_workflow_execution_with_delay() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let delay_node = WorkflowNode::new(
        "Delay Node",
        "A node with delay",
        NodeType::Delay {
            duration_seconds: 1,
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Delay Workflow")
        .add_node(delay_node)
        .expect("Failed to add delay node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Test that workflow with delay nodes can be created
    assert_eq!(workflow.graph.get_nodes().len(), 1);
    assert!(
        workflow.validate().is_ok(),
        "Workflow with delay should be valid"
    );
}

#[tokio::test]
async fn test_workflow_execution_with_transform() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let transform_node = WorkflowNode::new(
        "Transform Node",
        "A transformation node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Transform Workflow")
        .add_node(transform_node)
        .expect("Failed to add transform node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Test that workflow with transform nodes can be created
    assert_eq!(workflow.graph.get_nodes().len(), 1);
    assert!(
        workflow.validate().is_ok(),
        "Workflow with transform should be valid"
    );
}

#[tokio::test]
async fn test_workflow_execution_with_conditions() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let condition_node = WorkflowNode::new(
        "Condition Node",
        "A conditional node",
        NodeType::Condition {
            expression: "length > 5".to_string(),
        },
    );

    let (builder, _node_id) = WorkflowBuilder::new("Condition Workflow")
        .add_node(condition_node)
        .expect("Failed to add condition node");

    let workflow = builder.build().expect("Failed to build workflow");

    // Test that workflow with condition nodes can be created
    assert_eq!(workflow.graph.get_nodes().len(), 1);
    assert!(
        workflow.validate().is_ok(),
        "Workflow with condition should be valid"
    );
}
