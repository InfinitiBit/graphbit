//! Workflow execution integration tests
//!
//! Tests for complete workflow execution scenarios including
//! graph validation, node execution, error handling, and state management.

use graphbit_core::{
    graph::{EdgeType, NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
    types::{AgentId, NodeId, RetryConfig, WorkflowContext, WorkflowId, WorkflowState},
};
use serde_json::json;

#[allow(clippy::duplicate_mod)]
#[path = "../rust_unit_tests/test_helpers.rs"]
mod test_helpers;

#[tokio::test]
async fn test_simple_linear_workflow_execution() {
    let mut graph = WorkflowGraph::new();

    // Create a simple linear workflow: Input -> Transform -> Output
    let input_node = WorkflowNode::new(
        "Input",
        "Input node",
        NodeType::Custom {
            function_name: "input_processor".to_string(),
        },
    );
    let transform_node = WorkflowNode::new(
        "Transform",
        "Transform node",
        NodeType::Transform {
            transformation: "uppercase".to_string(),
        },
    );
    let output_node = WorkflowNode::new(
        "Output",
        "Output node",
        NodeType::Custom {
            function_name: "output_processor".to_string(),
        },
    );

    let input_id = input_node.id.clone();
    let transform_id = transform_node.id.clone();
    let output_id = output_node.id.clone();

    graph.add_node(input_node).unwrap();
    graph.add_node(transform_node).unwrap();
    graph.add_node(output_node).unwrap();

    graph
        .add_edge(
            input_id.clone(),
            transform_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            transform_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the graph
    graph.validate().unwrap();

    // Create execution context
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());
    context.set_metadata("input_data".to_string(), json!("hello world"));

    // Verify graph structure
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);

    let root_nodes = graph.get_root_nodes();
    assert_eq!(root_nodes.len(), 1);
    assert!(root_nodes.contains(&input_id));

    let leaf_nodes = graph.get_leaf_nodes();
    assert_eq!(leaf_nodes.len(), 1);
    assert!(leaf_nodes.contains(&output_id));

    // Test topological ordering
    let topo_order = graph.topological_sort().unwrap();
    assert_eq!(topo_order.len(), 3);

    let input_pos = topo_order.iter().position(|id| *id == input_id).unwrap();
    let transform_pos = topo_order
        .iter()
        .position(|id| *id == transform_id)
        .unwrap();
    let output_pos = topo_order.iter().position(|id| *id == output_id).unwrap();

    assert!(input_pos < transform_pos);
    assert!(transform_pos < output_pos);
}

#[tokio::test]
async fn test_parallel_workflow_execution() {
    let mut graph = WorkflowGraph::new();

    // Create parallel workflow: Input -> [Process1, Process2] -> Join -> Output
    let input_node = WorkflowNode::new("Input", "Input", NodeType::Split);
    let process1_node = WorkflowNode::new(
        "Process1",
        "Process 1",
        NodeType::Transform {
            transformation: "process1".to_string(),
        },
    );
    let process2_node = WorkflowNode::new(
        "Process2",
        "Process 2",
        NodeType::Transform {
            transformation: "process2".to_string(),
        },
    );
    let join_node = WorkflowNode::new("Join", "Join results", NodeType::Join);
    let output_node = WorkflowNode::new(
        "Output",
        "Final output",
        NodeType::Custom {
            function_name: "output".to_string(),
        },
    );

    let input_id = input_node.id.clone();
    let process1_id = process1_node.id.clone();
    let process2_id = process2_node.id.clone();
    let join_id = join_node.id.clone();
    let output_id = output_node.id.clone();

    // Add nodes
    graph.add_node(input_node).unwrap();
    graph.add_node(process1_node).unwrap();
    graph.add_node(process2_node).unwrap();
    graph.add_node(join_node).unwrap();
    graph.add_node(output_node).unwrap();

    // Add edges for parallel execution
    graph
        .add_edge(
            input_id.clone(),
            process1_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            input_id.clone(),
            process2_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            process1_id.clone(),
            join_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            process2_id.clone(),
            join_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            join_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the graph
    graph.validate().unwrap();

    // Test parallel structure
    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 5);

    let dependencies_join = graph.get_dependencies(&join_id);
    assert_eq!(dependencies_join.len(), 2);
    assert!(dependencies_join.contains(&process1_id));
    assert!(dependencies_join.contains(&process2_id));

    let dependents_input = graph.get_dependents(&input_id);
    assert_eq!(dependents_input.len(), 2);
    assert!(dependents_input.contains(&process1_id));
    assert!(dependents_input.contains(&process2_id));
}

#[tokio::test]
async fn test_conditional_workflow_execution() {
    let mut graph = WorkflowGraph::new();

    // Create conditional workflow: Input -> Condition -> [TruePath, FalsePath] -> Output
    let input_node = WorkflowNode::new(
        "Input",
        "Input",
        NodeType::Custom {
            function_name: "input".to_string(),
        },
    );
    let condition_node = WorkflowNode::new(
        "Condition",
        "Check condition",
        NodeType::Condition {
            expression: "value > 10".to_string(),
        },
    );
    let true_path_node = WorkflowNode::new(
        "TruePath",
        "True path",
        NodeType::Transform {
            transformation: "true_transform".to_string(),
        },
    );
    let false_path_node = WorkflowNode::new(
        "FalsePath",
        "False path",
        NodeType::Transform {
            transformation: "false_transform".to_string(),
        },
    );
    let output_node = WorkflowNode::new(
        "Output",
        "Output",
        NodeType::Custom {
            function_name: "output".to_string(),
        },
    );

    let input_id = input_node.id.clone();
    let condition_id = condition_node.id.clone();
    let true_path_id = true_path_node.id.clone();
    let false_path_id = false_path_node.id.clone();
    let output_id = output_node.id.clone();

    // Add nodes
    graph.add_node(input_node).unwrap();
    graph.add_node(condition_node).unwrap();
    graph.add_node(true_path_node).unwrap();
    graph.add_node(false_path_node).unwrap();
    graph.add_node(output_node).unwrap();

    // Add conditional edges
    graph
        .add_edge(
            input_id.clone(),
            condition_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            condition_id.clone(),
            true_path_id.clone(),
            WorkflowEdge::conditional("true"),
        )
        .unwrap();
    graph
        .add_edge(
            condition_id.clone(),
            false_path_id.clone(),
            WorkflowEdge::conditional("false"),
        )
        .unwrap();
    graph
        .add_edge(
            true_path_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            false_path_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the graph
    graph.validate().unwrap();

    // Test conditional structure
    let condition_dependents = graph.get_dependents(&condition_id);
    assert_eq!(condition_dependents.len(), 2);
    assert!(condition_dependents.contains(&true_path_id));
    assert!(condition_dependents.contains(&false_path_id));

    // Test edge conditions
    let edges: Vec<_> = graph
        .get_edges()
        .iter()
        .filter(|(from, _, _)| *from == condition_id)
        .collect();
    assert_eq!(edges.len(), 2);

    for (_, _, edge) in edges {
        assert!(matches!(edge.edge_type, EdgeType::Conditional));
        assert!(edge.condition.is_some());
        let condition = edge.condition.as_ref().unwrap();
        assert!(condition == "true" || condition == "false");
    }
}

#[tokio::test]
async fn test_workflow_with_agent_nodes() {
    let mut graph = WorkflowGraph::new();

    // Create workflow with agent nodes
    let agent_id = AgentId::new();
    let input_node = WorkflowNode::new(
        "Input",
        "Input",
        NodeType::Custom {
            function_name: "input".to_string(),
        },
    );
    let agent_node = WorkflowNode::new(
        "Agent",
        "AI Agent",
        NodeType::Agent {
            agent_id: agent_id.clone(),
            prompt_template: "Process this data: {input}".to_string(),
        },
    );
    let output_node = WorkflowNode::new(
        "Output",
        "Output",
        NodeType::Custom {
            function_name: "output".to_string(),
        },
    );

    let input_id = input_node.id.clone();
    let agent_node_id = agent_node.id.clone();
    let output_id = output_node.id.clone();

    // Configure agent node with additional settings
    let agent_node = agent_node
        .with_config("temperature".to_string(), json!(0.7))
        .with_config("max_tokens".to_string(), json!(1000))
        .with_timeout(30)
        .with_tags(vec!["ai".to_string(), "processing".to_string()]);

    graph.add_node(input_node).unwrap();
    graph.add_node(agent_node).unwrap();
    graph.add_node(output_node).unwrap();

    graph
        .add_edge(
            input_id.clone(),
            agent_node_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            agent_node_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the graph
    graph.validate().unwrap();

    // Test agent node configuration
    let agent_node = graph.get_node(&agent_node_id).unwrap();
    assert!(matches!(agent_node.node_type, NodeType::Agent { .. }));
    assert_eq!(agent_node.config.get("temperature").unwrap(), &json!(0.7));
    assert_eq!(agent_node.config.get("max_tokens").unwrap(), &json!(1000));
    assert_eq!(agent_node.timeout_seconds, Some(30));
    assert_eq!(agent_node.tags.len(), 2);
}

#[tokio::test]
async fn test_workflow_error_handling_and_recovery() {
    let mut graph = WorkflowGraph::new();

    // Create workflow with potential failure points
    let input_node = WorkflowNode::new(
        "Input",
        "Input",
        NodeType::Custom {
            function_name: "input".to_string(),
        },
    );
    let risky_node = WorkflowNode::new(
        "RiskyOperation",
        "Might fail",
        NodeType::Custom {
            function_name: "risky_operation".to_string(),
        },
    )
    .with_retry_config(RetryConfig::new(3).with_exponential_backoff(1000, 2.0, 10000));
    let fallback_node = WorkflowNode::new(
        "Fallback",
        "Fallback operation",
        NodeType::Custom {
            function_name: "fallback".to_string(),
        },
    );
    let output_node = WorkflowNode::new(
        "Output",
        "Output",
        NodeType::Custom {
            function_name: "output".to_string(),
        },
    );

    let input_id = input_node.id.clone();
    let risky_id = risky_node.id.clone();
    let fallback_id = fallback_node.id.clone();
    let output_id = output_node.id.clone();

    graph.add_node(input_node).unwrap();
    graph.add_node(risky_node).unwrap();
    graph.add_node(fallback_node).unwrap();
    graph.add_node(output_node).unwrap();

    // Primary path
    graph
        .add_edge(
            input_id.clone(),
            risky_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            risky_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Fallback path - using control_flow for error handling
    graph
        .add_edge(
            input_id.clone(),
            fallback_id.clone(),
            WorkflowEdge::control_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            fallback_id.clone(),
            output_id.clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the graph
    graph.validate().unwrap();

    // Test retry configuration
    let risky_node = graph.get_node(&risky_id).unwrap();
    assert_eq!(risky_node.retry_config.max_attempts, 3);
    assert_eq!(risky_node.retry_config.initial_delay_ms, 1000);
    assert_eq!(risky_node.retry_config.backoff_multiplier, 2.0);
    assert_eq!(risky_node.retry_config.max_delay_ms, 10000);

    // Test error handling edge
    let control_edges: Vec<_> = graph
        .get_edges()
        .iter()
        .filter(|(from, _, edge)| {
            *from == input_id && matches!(edge.edge_type, EdgeType::ControlFlow)
        })
        .collect();
    assert_eq!(control_edges.len(), 1);
}

#[tokio::test]
async fn test_workflow_state_transitions() {
    let workflow_id = WorkflowId::new();
    let mut context = WorkflowContext::new(workflow_id.clone());

    // Test initial state
    assert!(matches!(context.state, WorkflowState::Pending));

    // Test state transitions
    let node_id = NodeId::new();
    context.state = WorkflowState::Running {
        current_node: node_id.clone(),
    };

    match &context.state {
        WorkflowState::Running { current_node } => {
            assert_eq!(*current_node, node_id);
        }
        _ => panic!("Expected Running state"),
    }

    // Test completion
    context.state = WorkflowState::Completed;

    match &context.state {
        WorkflowState::Completed => {
            // State is completed
        }
        _ => panic!("Expected Completed state"),
    }

    // Test failure state
    context.state = WorkflowState::Failed {
        error: "Processing failed".to_string(),
    };

    match &context.state {
        WorkflowState::Failed { error } => {
            assert_eq!(error, "Processing failed");
        }
        _ => panic!("Expected Failed state"),
    }
}

#[tokio::test]
async fn test_complex_workflow_validation() {
    let mut graph = WorkflowGraph::new();

    // Create a complex workflow with multiple patterns
    let nodes: Vec<_> = (0..10)
        .map(|i| {
            WorkflowNode::new(
                format!("Node{i}"),
                format!("Node {i} description"),
                match i % 4 {
                    0 => NodeType::Split,
                    1 => NodeType::Transform {
                        transformation: format!("transform{i}"),
                    },
                    2 => NodeType::Condition {
                        expression: format!("condition{i}"),
                    },
                    3 => NodeType::Join,
                    _ => unreachable!(),
                },
            )
        })
        .collect();

    let node_ids: Vec<_> = nodes.iter().map(|n| n.id.clone()).collect();

    // Add all nodes
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    // Create complex edge patterns
    // Linear chain: 0 -> 1 -> 2 -> 3
    for i in 0..3 {
        graph
            .add_edge(
                node_ids[i].clone(),
                node_ids[i + 1].clone(),
                WorkflowEdge::data_flow(),
            )
            .unwrap();
    }

    // Parallel branches: 4 -> [5, 6] -> 7
    graph
        .add_edge(
            node_ids[4].clone(),
            node_ids[5].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            node_ids[4].clone(),
            node_ids[6].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            node_ids[5].clone(),
            node_ids[7].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            node_ids[6].clone(),
            node_ids[7].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Additional connections: 3 -> 8 -> 9
    graph
        .add_edge(
            node_ids[3].clone(),
            node_ids[8].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();
    graph
        .add_edge(
            node_ids[8].clone(),
            node_ids[9].clone(),
            WorkflowEdge::data_flow(),
        )
        .unwrap();

    // Validate the complex graph
    graph.validate().unwrap();

    // Test graph properties
    assert_eq!(graph.node_count(), 10);
    assert!(!graph.has_cycles());

    let topo_order = graph.topological_sort().unwrap();
    assert_eq!(topo_order.len(), 10);

    // Verify topological ordering constraints
    let positions: std::collections::HashMap<_, _> = topo_order
        .iter()
        .enumerate()
        .map(|(i, id)| (id, i))
        .collect();

    // Check linear chain ordering
    for i in 0..3 {
        assert!(positions[&node_ids[i]] < positions[&node_ids[i + 1]]);
    }

    // Check parallel branch ordering
    assert!(positions[&node_ids[4]] < positions[&node_ids[5]]);
    assert!(positions[&node_ids[4]] < positions[&node_ids[6]]);
    assert!(positions[&node_ids[5]] < positions[&node_ids[7]]);
    assert!(positions[&node_ids[6]] < positions[&node_ids[7]]);
}
