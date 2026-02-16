//! Workflow graph structure and algorithms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use petgraph::{
    algo::{is_cyclic_directed, toposort},
    graph::{DiGraph, NodeIndex},
    Direction,
};

use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::NodeId;

use super::edge::WorkflowEdge;
use super::node::{NodeType, WorkflowNode};

/// A workflow graph that defines the structure and execution flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    #[serde(skip)]
    graph: DiGraph<WorkflowNode, WorkflowEdge>,
    #[serde(skip)]
    node_map: HashMap<NodeId, NodeIndex>,
    nodes: HashMap<NodeId, WorkflowNode>,
    edges: Vec<(NodeId, NodeId, WorkflowEdge)>,
    metadata: HashMap<String, serde_json::Value>,
    #[serde(skip)]
    dependencies_cache: HashMap<NodeId, Vec<NodeId>>,
    #[serde(skip)]
    dependents_cache: HashMap<NodeId, Vec<NodeId>>,
    #[serde(skip)]
    root_nodes_cache: Option<Vec<NodeId>>,
    #[serde(skip)]
    leaf_nodes_cache: Option<Vec<NodeId>>,
}

impl WorkflowGraph {
    /// Create a new empty workflow graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::with_capacity(16),
            nodes: HashMap::with_capacity(16),
            edges: Vec::with_capacity(16),
            metadata: HashMap::new(),
            dependencies_cache: HashMap::with_capacity(16),
            dependents_cache: HashMap::with_capacity(16),
            root_nodes_cache: None,
            leaf_nodes_cache: None,
        }
    }

    fn invalidate_caches(&mut self) {
        self.dependencies_cache.clear();
        self.dependents_cache.clear();
        self.root_nodes_cache = None;
        self.leaf_nodes_cache = None;
    }

    /// Rebuild the graph structure from serialized data
    pub fn rebuild_graph(&mut self) -> GraphBitResult<()> {
        self.graph = DiGraph::new();
        self.node_map.clear();
        self.invalidate_caches();

        self.node_map.reserve(self.nodes.len());

        for (node_id, node) in &self.nodes {
            let graph_index = self.graph.add_node(node.clone());
            self.node_map.insert(node_id.clone(), graph_index);
        }

        for (from, to, edge) in &self.edges {
            let from_index = self
                .node_map
                .get(from)
                .ok_or_else(|| GraphBitError::graph(format!("Source node {from} not found")))?;

            let to_index = self
                .node_map
                .get(to)
                .ok_or_else(|| GraphBitError::graph(format!("Target node {to} not found")))?;

            self.graph.add_edge(*from_index, *to_index, edge.clone());
        }

        Ok(())
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: WorkflowNode) -> GraphBitResult<()> {
        let node_id = node.id.clone();

        if self.nodes.contains_key(&node_id) {
            let incoming_name = node.name.clone();
            let existing_name = self
                .nodes
                .get(&node_id)
                .map(|n| n.name.clone())
                .unwrap_or_else(|| "<unknown>".to_string());
            return Err(GraphBitError::graph(format!(
                "Node already exists: id={node_id} (existing name='{existing_name}', incoming name='{incoming_name}'). Hint: create a fresh Node instance; do not add the same Node object twice."
            )));
        }

        let graph_index = self.graph.add_node(node.clone());
        self.node_map.insert(node_id.clone(), graph_index);
        self.nodes.insert(node_id, node);

        self.invalidate_caches();

        Ok(())
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, edge: WorkflowEdge) -> GraphBitResult<()> {
        let from_index = self
            .node_map
            .get(&from)
            .ok_or_else(|| GraphBitError::graph(format!("Source node {from} not found")))?;

        let to_index = self
            .node_map
            .get(&to)
            .ok_or_else(|| GraphBitError::graph(format!("Target node {to} not found")))?;

        self.graph.add_edge(*from_index, *to_index, edge.clone());
        self.edges.push((from, to, edge));

        self.invalidate_caches();

        Ok(())
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: &NodeId) -> GraphBitResult<()> {
        let graph_index = self
            .node_map
            .remove(node_id)
            .ok_or_else(|| GraphBitError::graph(format!("Node {node_id} not found")))?;

        self.graph.remove_node(graph_index);
        self.nodes.remove(node_id);

        self.edges
            .retain(|(from, to, _)| from != node_id && to != node_id);

        self.invalidate_caches();

        Ok(())
    }

    /// Get a node by ID
    #[inline]
    pub fn get_node(&self, node_id: &NodeId) -> Option<&WorkflowNode> {
        self.nodes.get(node_id)
    }

    /// Get all nodes
    #[inline]
    pub fn get_nodes(&self) -> &HashMap<NodeId, WorkflowNode> {
        &self.nodes
    }

    /// Get all edges
    #[inline]
    pub fn get_edges(&self) -> &[(NodeId, NodeId, WorkflowEdge)] {
        &self.edges
    }

    /// Check if the graph contains cycles
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }

    /// Get topological ordering of nodes
    pub fn topological_sort(&self) -> GraphBitResult<Vec<NodeId>> {
        let sorted_indices = toposort(&self.graph, None).map_err(|_| {
            GraphBitError::graph("Graph contains cycles - cannot perform topological sort")
        })?;

        let mut sorted_nodes = Vec::with_capacity(sorted_indices.len());
        for index in sorted_indices {
            for (node_id, &node_index) in &self.node_map {
                if node_index == index {
                    sorted_nodes.push(node_id.clone());
                    break;
                }
            }
        }

        Ok(sorted_nodes)
    }

    /// Get dependencies (incoming edges) for a node with caching
    pub fn get_dependencies(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        if let Some(deps) = self.dependencies_cache.get(node_id) {
            return deps.clone();
        }

        let mut dependencies = Vec::new();

        if let Some(&node_index) = self.node_map.get(node_id) {
            let incoming = self
                .graph
                .neighbors_directed(node_index, Direction::Incoming);

            for neighbor_index in incoming {
                for (neighbor_id, &idx) in &self.node_map {
                    if idx == neighbor_index {
                        dependencies.push(neighbor_id.clone());
                        break;
                    }
                }
            }
        }

        self.dependencies_cache
            .insert(node_id.clone(), dependencies.clone());
        dependencies
    }

    /// Get dependents (outgoing edges) for a node with caching
    pub fn get_dependents(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        if let Some(deps) = self.dependents_cache.get(node_id) {
            return deps.clone();
        }

        let mut dependents = Vec::new();

        if let Some(&node_index) = self.node_map.get(node_id) {
            let outgoing = self
                .graph
                .neighbors_directed(node_index, Direction::Outgoing);

            for neighbor_index in outgoing {
                for (neighbor_id, &idx) in &self.node_map {
                    if idx == neighbor_index {
                        dependents.push(neighbor_id.clone());
                        break;
                    }
                }
            }
        }

        self.dependents_cache
            .insert(node_id.clone(), dependents.clone());
        dependents
    }

    /// Get root nodes (nodes with no dependencies) with caching
    pub fn get_root_nodes(&mut self) -> Vec<NodeId> {
        if let Some(ref roots) = self.root_nodes_cache {
            return roots.clone();
        }

        let node_ids: Vec<NodeId> = self.nodes.keys().cloned().collect();
        let roots: Vec<NodeId> = node_ids
            .into_iter()
            .filter(|node_id| self.get_dependencies(node_id).is_empty())
            .collect();

        self.root_nodes_cache = Some(roots.clone());
        roots
    }

    /// Get leaf nodes (nodes with no dependents) with caching
    pub fn get_leaf_nodes(&mut self) -> Vec<NodeId> {
        if let Some(ref leaves) = self.leaf_nodes_cache {
            return leaves.clone();
        }

        let node_ids: Vec<NodeId> = self.nodes.keys().cloned().collect();
        let leaves: Vec<NodeId> = node_ids
            .into_iter()
            .filter(|node_id| self.get_dependents(node_id).is_empty())
            .collect();

        self.leaf_nodes_cache = Some(leaves.clone());
        leaves
    }

    /// Check if a node is ready to execute (all dependencies completed)
    pub fn is_node_ready(
        &mut self,
        node_id: &NodeId,
        completed_nodes: &std::collections::HashSet<NodeId>,
    ) -> bool {
        let dependencies = self.get_dependencies(node_id);
        dependencies.iter().all(|dep| completed_nodes.contains(dep))
    }

    /// Get the next executable nodes (optimized version)
    pub fn get_next_executable_nodes(
        &mut self,
        completed_nodes: &std::collections::HashSet<NodeId>,
        running_nodes: &std::collections::HashSet<NodeId>,
    ) -> Vec<NodeId> {
        let mut executable = Vec::with_capacity(8);

        let node_ids: Vec<NodeId> = self.nodes.keys().cloned().collect();

        for node_id in node_ids {
            if !completed_nodes.contains(&node_id)
                && !running_nodes.contains(&node_id)
                && self.is_node_ready(&node_id, completed_nodes)
            {
                executable.push(node_id);
            }
        }

        executable
    }

    /// Validate the graph structure
    pub fn validate(&self) -> GraphBitResult<()> {
        if self.has_cycles() {
            return Err(GraphBitError::graph("Workflow graph contains cycles"));
        }

        for (from, to, _) in &self.edges {
            if !self.nodes.contains_key(from) {
                return Err(GraphBitError::graph(format!(
                    "Edge references non-existent source node: {from}"
                )));
            }
            if !self.nodes.contains_key(to) {
                return Err(GraphBitError::graph(format!(
                    "Edge references non-existent target node: {to}"
                )));
            }
        }

        for node in self.nodes.values() {
            node.validate()?;
        }

        {
            let mut agent_index: HashMap<String, Vec<(NodeId, String)>> = HashMap::new();
            for node in self.nodes.values() {
                if let NodeType::Agent { agent_id, .. } = &node.node_type {
                    agent_index
                        .entry(agent_id.to_string())
                        .or_default()
                        .push((node.id.clone(), node.name.clone()));
                }
            }
            let mut duplicates: Vec<(String, Vec<(NodeId, String)>)> = Vec::new();
            for (aid, entries) in agent_index.into_iter() {
                if entries.len() > 1 {
                    duplicates.push((aid, entries));
                }
            }
            if !duplicates.is_empty() {
                let mut parts: Vec<String> = Vec::new();
                for (aid, entries) in duplicates {
                    let detail = entries
                        .into_iter()
                        .map(|(id, name)| format!("{{id={id}, name='{name}'}}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    parts.push(format!("agent_id='{aid}' used by: [{detail}]"));
                }
                return Err(GraphBitError::graph(format!(
                    "Duplicate agent_id detected. Each agent_id must be unique across the workflow. Conflicts: {}",
                    parts.join("; ")
                )));
            }
        }

        if self
            .metadata
            .get("enforce_unique_node_names")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            let mut name_index: HashMap<String, Vec<NodeId>> = HashMap::new();
            for node in self.nodes.values() {
                name_index
                    .entry(node.name.clone())
                    .or_default()
                    .push(node.id.clone());
            }
            let mut dup_names: Vec<(String, Vec<NodeId>)> = Vec::new();
            for (name, ids) in name_index.into_iter() {
                if ids.len() > 1 {
                    dup_names.push((name, ids));
                }
            }
            if !dup_names.is_empty() {
                let mut parts: Vec<String> = Vec::new();
                for (name, ids) in dup_names {
                    let ids_str = ids
                        .into_iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    parts.push(format!("name='{name}' used by node ids: [{ids_str}]"));
                }
                return Err(GraphBitError::graph(format!(
                    "Duplicate node names not allowed (enforce_unique_node_names=true). Conflicts: {}",
                    parts.join("; ")
                )));
            }
        }

        Ok(())
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// Get number of nodes
    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of edges
    #[inline]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get node ID by name
    pub fn get_node_id_by_name(&self, name: &str) -> Option<NodeId> {
        self.nodes
            .values()
            .find(|node| node.name == name)
            .map(|node| node.id.clone())
    }
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self::new()
    }
}
