//! Helper functions for workflow execution.

use std::collections::HashSet;

use crate::errors::GraphBitResult;
use crate::graph::{NodeType, WorkflowGraph, WorkflowNode};
use crate::workflow::Workflow;

/// Extract agent IDs from a workflow
pub fn extract_agent_ids_from_workflow(workflow: &Workflow) -> Vec<String> {
    let mut agent_ids = HashSet::new();

    for node in workflow.graph.get_nodes().values() {
        if let NodeType::Agent { agent_id, .. } = &node.node_type {
            agent_ids.insert(agent_id.to_string());
        }
    }

    agent_ids.into_iter().collect()
}

/// Collect nodes in executable order
pub fn collect_executable_nodes(graph: &WorkflowGraph) -> GraphBitResult<Vec<WorkflowNode>> {
    let nodes: Vec<WorkflowNode> = graph.get_nodes().values().cloned().collect();
    Ok(nodes)
}

/// Create batches that strictly respect dependencies
pub async fn create_dependency_batches(
    graph: &WorkflowGraph,
) -> GraphBitResult<Vec<Vec<WorkflowNode>>> {
    let mut graph_clone = graph.clone();
    let mut completed: HashSet<crate::types::NodeId> = HashSet::new();
    let mut remaining: HashSet<crate::types::NodeId> = graph_clone.get_nodes().keys().cloned().collect();
    let mut batches: Vec<Vec<WorkflowNode>> = Vec::new();

    while !remaining.is_empty() {
        let mut ready_ids: Vec<crate::types::NodeId> = Vec::new();
        for nid in remaining.iter() {
            let deps = graph_clone.get_dependencies(nid);
            if deps.iter().all(|d| completed.contains(d)) {
                ready_ids.push(nid.clone());
            }
        }

        if ready_ids.is_empty() {
            return Err(crate::errors::GraphBitError::workflow_execution(
                "No dependency-ready nodes found; graph may be cyclic or invalid".to_string(),
            ));
        }

        let mut batch: Vec<WorkflowNode> = Vec::with_capacity(ready_ids.len());
        for nid in &ready_ids {
            if let Some(node) = graph_clone.get_nodes().get(nid) {
                batch.push(node.clone());
            }
        }
        batches.push(batch);

        for nid in ready_ids {
            completed.insert(nid.clone());
            remaining.remove(&nid);
        }
    }

    Ok(batches)
}
