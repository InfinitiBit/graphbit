//! Graph workflow core implementation.
//!
//! Contains the graph structure and adjacency management for workflow execution.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::NodeId;
use petgraph::{
    Direction,
    algo::{is_cyclic_directed, toposort},
    graph::{DiGraph, NodeIndex},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::{NodeType, WorkflowEdge, WorkflowNode};

/// A workflow graph that defines the structure and execution flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    /// Graph structure
    #[serde(skip)]
    graph: DiGraph<WorkflowNode, WorkflowEdge>,
    /// Mapping from `NodeId` to graph indices
    #[serde(skip)]
    node_map: HashMap<NodeId, NodeIndex>,
    /// Reverse mapping from graph index to `NodeId`
    #[serde(skip)]
    index_to_id: HashMap<NodeIndex, NodeId>,
    /// Serializable representation of nodes
    nodes: HashMap<NodeId, WorkflowNode>,
    /// Serializable representation of edges
    edges: Vec<(NodeId, NodeId, WorkflowEdge)>,
    /// Graph metadata
    metadata: HashMap<String, serde_json::Value>,
    /// Cached adjacency information for performance
    #[serde(skip)]
    dependencies_cache: HashMap<NodeId, Vec<NodeId>>,
    #[serde(skip)]
    dependents_cache: HashMap<NodeId, Vec<NodeId>>,
    /// Cached outgoing adjacency lists (source -> successors)
    #[serde(skip)]
    outgoing: HashMap<NodeId, Vec<NodeId>>,
    /// Cached incoming adjacency lists (target -> predecessors)
    #[serde(skip)]
    incoming: HashMap<NodeId, Vec<NodeId>>,
    /// Cached node name lookup (first-wins when duplicate names exist)
    #[serde(skip)]
    name_to_id: HashMap<String, NodeId>,
    /// Cached root and leaf nodes
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
            node_map: HashMap::with_capacity(16), // Pre-allocate capacity
            index_to_id: HashMap::with_capacity(16),
            nodes: HashMap::with_capacity(16),    // Pre-allocate capacity
            edges: Vec::with_capacity(16),        // Pre-allocate capacity
            metadata: HashMap::new(),
            dependencies_cache: HashMap::with_capacity(16),
            dependents_cache: HashMap::with_capacity(16),
            outgoing: HashMap::with_capacity(16),
            incoming: HashMap::with_capacity(16),
            name_to_id: HashMap::with_capacity(16),
            root_nodes_cache: None,
            leaf_nodes_cache: None,
        }
    }

    /// Invalidate caches when graph structure changes
    fn invalidate_caches(&mut self) {
        self.dependencies_cache.clear();
        self.dependents_cache.clear();
        self.root_nodes_cache = None;
        self.leaf_nodes_cache = None;
    }

    /// Refresh outgoing/incoming adjacency maps from canonical `edges`.
    fn refresh_adjacency_from_edges(&mut self) {
        self.outgoing.clear();
        self.incoming.clear();
        self.outgoing.reserve(self.nodes.len());
        self.incoming.reserve(self.nodes.len());

        for (from, to, _) in &self.edges {
            self.outgoing
                .entry(from.clone())
                .or_default()
                .push(to.clone());
            self.incoming
                .entry(to.clone())
                .or_default()
                .push(from.clone());
        }
    }

    /// Rebuild `name_to_id` deterministically using first-wins semantics.
    fn rebuild_name_map_from_nodes(&mut self) {
        self.name_to_id.clear();
        self.name_to_id.reserve(self.nodes.len());

        let mut ordered: Vec<(&NodeId, &WorkflowNode)> = self.nodes.iter().collect();
        ordered.sort_by(|(a, _), (b, _)| a.to_string().cmp(&b.to_string()));

        for (id, node) in ordered {
            self.name_to_id
                .entry(node.name.clone())
                .or_insert_with(|| id.clone());
        }
    }

    /// Rebuild petgraph structures and node id/index maps from `nodes` + `edges`.
    fn rebuild_petgraph_and_id_maps(&mut self) -> GraphBitResult<()> {
        self.graph = DiGraph::new();
        self.node_map.clear();
        self.index_to_id.clear();

        self.node_map.reserve(self.nodes.len());
        self.index_to_id.reserve(self.nodes.len());

        for (node_id, node) in &self.nodes {
            let graph_index = self.graph.add_node(node.clone());
            self.node_map.insert(node_id.clone(), graph_index);
            self.index_to_id.insert(graph_index, node_id.clone());
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

    /// Rebuild the graph structure from serialized data
    /// This must be called after deserialization since `graph` and `node_map` are not serialized
    pub fn rebuild_graph(&mut self) -> GraphBitResult<()> {
        self.rebuild_petgraph_and_id_maps()?;
        self.refresh_adjacency_from_edges();
        self.rebuild_name_map_from_nodes();
        self.invalidate_caches();
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
        self.index_to_id.insert(graph_index, node_id.clone());
        self.name_to_id
            .entry(node.name.clone())
            .or_insert_with(|| node_id.clone());
        self.nodes.insert(node_id, node);

        // Invalidate caches since graph structure changed
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
        self.edges.push((from.clone(), to.clone(), edge));
        self.outgoing.entry(from.clone()).or_default().push(to.clone());
        self.incoming.entry(to.clone()).or_default().push(from.clone());

        // Invalidate caches since graph structure changed
        self.invalidate_caches();

        Ok(())
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: &NodeId) -> GraphBitResult<()> {
        if self.nodes.remove(node_id).is_none() {
            return Err(GraphBitError::graph(format!("Node {node_id} not found")));
        }

        // Remove edges involving this node
        self.edges
            .retain(|(from, to, _)| from != node_id && to != node_id);

        // Update adjacency maps incrementally for the removed node.
        // Remove any outgoing entries for this node and drop this node from predecessors.
        if let Some(successors) = self.outgoing.remove(node_id) {
            for successor in successors {
                if let Some(predecessors) = self.incoming.get_mut(&successor) {
                    predecessors.retain(|source| source != node_id);
                    if predecessors.is_empty() {
                        self.incoming.remove(&successor);
                    }
                }
            }
        }

        // Remove any incoming entries for this node and drop this node from successors.
        if let Some(predecessors) = self.incoming.remove(node_id) {
            for predecessor in predecessors {
                if let Some(successors) = self.outgoing.get_mut(&predecessor) {
                    successors.retain(|target| target != node_id);
                    if successors.is_empty() {
                        self.outgoing.remove(&predecessor);
                    }
                }
            }
        }

        // Rebuild full graph/index structures to avoid stale NodeIndex entries
        self.rebuild_petgraph_and_id_maps()?;
        self.refresh_adjacency_from_edges();
        self.rebuild_name_map_from_nodes();

        // Invalidate caches since graph structure changed
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

        // Pre-allocate with known capacity
        let mut sorted_nodes = Vec::with_capacity(sorted_indices.len());
        for index in sorted_indices {
            let node_id = self.index_to_id.get(&index).ok_or_else(|| {
                GraphBitError::graph(format!(
                    "Missing NodeId mapping for graph index {:?} during topological sort",
                    index
                ))
            })?;
            sorted_nodes.push(node_id.clone());
        }

        Ok(sorted_nodes)
    }

    /// Get dependencies (incoming edges) for a node with caching
    pub fn get_dependencies(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        // Check cache first
        if let Some(deps) = self.dependencies_cache.get(node_id) {
            return deps.clone();
        }

        let mut dependencies = Vec::new();

        if let Some(&node_index) = self.node_map.get(node_id) {
            let incoming = self
                .graph
                .neighbors_directed(node_index, Direction::Incoming);

            for neighbor_index in incoming {
                if let Some(neighbor_id) = self.index_to_id.get(&neighbor_index) {
                    dependencies.push(neighbor_id.clone());
                }
            }
        }

        // Cache the result
        self.dependencies_cache
            .insert(node_id.clone(), dependencies.clone());
        dependencies
    }

    /// Get dependents (outgoing edges) for a node with caching
    pub fn get_dependents(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        // Check cache first
        if let Some(deps) = self.dependents_cache.get(node_id) {
            return deps.clone();
        }

        let mut dependents = Vec::new();

        if let Some(&node_index) = self.node_map.get(node_id) {
            let outgoing = self
                .graph
                .neighbors_directed(node_index, Direction::Outgoing);

            for neighbor_index in outgoing {
                if let Some(neighbor_id) = self.index_to_id.get(&neighbor_index) {
                    dependents.push(neighbor_id.clone());
                }
            }
        }

        // Cache the result
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
        // Pre-allocate with estimated capacity
        let mut executable = Vec::with_capacity(8);

        // Collect node IDs first to avoid borrow conflicts
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
        // Check for cycles
        if self.has_cycles() {
            return Err(GraphBitError::graph("Workflow graph contains cycles"));
        }

        // Check that all edge endpoints exist
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

        // Validate individual nodes
        for node in self.nodes.values() {
            node.validate()?;
        }

        // Condition nodes: direct successors must have unique names (routing by name)
        for node in self.nodes.values() {
            if matches!(node.node_type, NodeType::Condition { .. }) {
                let mut seen: HashSet<&str> = HashSet::new();
                for (from, to, _) in &self.edges {
                    if from == &node.id {
                        if let Some(succ) = self.nodes.get(to) {
                            if !seen.insert(succ.name.as_str()) {
                                return Err(GraphBitError::graph(format!(
                                    "Condition node '{}' has duplicate successor name '{}'",
                                    node.name, succ.name
                                )));
                            }
                        }
                    }
                }
            }
        }

        // Enforce unique agent IDs across all agent nodes
        {
            use std::collections::HashMap;
            let mut agent_index: HashMap<String, Vec<(NodeId, String)>> = HashMap::new();
            for node in self.nodes.values() {
                if let NodeType::Agent { config } = &node.node_type {
                    agent_index
                        .entry(config.agent_id.to_string())
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
                // Build a helpful error message listing conflicts
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

        // Optionally enforce unique node names if metadata flag is set
        if self
            .metadata
            .get("enforce_unique_node_names")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            use std::collections::HashMap;
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

    /// Get node ID by node name in O(1).
    ///
    /// If duplicate node names exist, this returns the canonical first-wins
    /// entry in the internal name index.
    pub fn get_node_id_by_name(&self, name: &str) -> Option<NodeId> {
        self.name_to_id.get(name).cloned()
    }

    /// Direct successors along control/data edges (`from` → `to`).
    pub fn direct_successors(&self, from: &NodeId) -> Vec<NodeId> {
        self.outgoing.get(from).cloned().unwrap_or_default()
    }

    /// All nodes reachable from `start` (including `start`) following outgoing edges.
    pub fn forward_reachable_from(&self, start: &NodeId) -> HashSet<NodeId> {
        let mut visited = HashSet::new();
        let mut stack = vec![start.clone()];
        while let Some(n) = stack.pop() {
            if !visited.insert(n.clone()) {
                continue;
            }
            for s in self.direct_successors(&n) {
                stack.push(s);
            }
        }
        visited
    }
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_reachable_from_small_dag() {
        let mut graph = WorkflowGraph::new();
        let a = WorkflowNode::new(
            "a",
            "",
            NodeType::Transform {
                transformation: "a".to_string(),
            },
        );
        let b = WorkflowNode::new(
            "b",
            "",
            NodeType::Transform {
                transformation: "b".to_string(),
            },
        );
        let c = WorkflowNode::new(
            "c",
            "",
            NodeType::Transform {
                transformation: "c".to_string(),
            },
        );
        let d = WorkflowNode::new(
            "d",
            "",
            NodeType::Transform {
                transformation: "d".to_string(),
            },
        );

        let a_id = a.id.clone();
        let b_id = b.id.clone();
        let c_id = c.id.clone();
        let d_id = d.id.clone();

        graph.add_node(a).expect("add node a");
        graph.add_node(b).expect("add node b");
        graph.add_node(c).expect("add node c");
        graph.add_node(d).expect("add node d");

        graph
            .add_edge(a_id.clone(), b_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge a->b");
        graph
            .add_edge(a_id.clone(), c_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge a->c");
        graph
            .add_edge(c_id.clone(), d_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge c->d");

        let reachable = graph.forward_reachable_from(&a_id);
        assert!(reachable.contains(&a_id));
        assert!(reachable.contains(&b_id));
        assert!(reachable.contains(&c_id));
        assert!(reachable.contains(&d_id));
        assert_eq!(reachable.len(), 4);
    }

    #[test]
    fn test_remove_node_keeps_indexes_and_lookups_consistent() {
        let mut graph = WorkflowGraph::new();
        let a = WorkflowNode::new(
            "a",
            "",
            NodeType::Transform {
                transformation: "a".to_string(),
            },
        );
        let b = WorkflowNode::new(
            "b",
            "",
            NodeType::Transform {
                transformation: "b".to_string(),
            },
        );
        let c = WorkflowNode::new(
            "c",
            "",
            NodeType::Transform {
                transformation: "c".to_string(),
            },
        );

        let a_id = a.id.clone();
        let b_id = b.id.clone();
        let c_id = c.id.clone();

        graph.add_node(a).expect("add node a");
        graph.add_node(b).expect("add node b");
        graph.add_node(c).expect("add node c");
        graph
            .add_edge(a_id.clone(), b_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge a->b");
        graph
            .add_edge(b_id.clone(), c_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge b->c");

        graph.remove_node(&b_id).expect("remove b");

        assert!(graph.get_node(&b_id).is_none());
        assert!(graph.get_node_id_by_name("b").is_none());
        assert!(graph.get_dependencies(&c_id).is_empty());
        assert!(graph.direct_successors(&a_id).is_empty());

        let sorted = graph.topological_sort().expect("topological sort after removal");
        assert!(sorted.contains(&a_id));
        assert!(sorted.contains(&c_id));
        assert_eq!(sorted.len(), 2);
    }

    #[test]
    fn test_add_edge_updates_adjacency_incrementally() {
        let mut graph = WorkflowGraph::new();
        let a = WorkflowNode::new(
            "a",
            "",
            NodeType::Transform {
                transformation: "a".to_string(),
            },
        );
        let b = WorkflowNode::new(
            "b",
            "",
            NodeType::Transform {
                transformation: "b".to_string(),
            },
        );

        let a_id = a.id.clone();
        let b_id = b.id.clone();

        graph.add_node(a).expect("add node a");
        graph.add_node(b).expect("add node b");

        assert!(graph.get_dependencies(&b_id).is_empty());
        assert!(graph.get_dependents(&a_id).is_empty());
        assert!(graph.direct_successors(&a_id).is_empty());

        graph
            .add_edge(a_id.clone(), b_id.clone(), WorkflowEdge::data_flow())
            .expect("add edge a->b");

        assert_eq!(graph.get_dependencies(&b_id), vec![a_id.clone()]);
        assert_eq!(graph.get_dependents(&a_id), vec![b_id.clone()]);
        assert_eq!(graph.direct_successors(&a_id), vec![b_id.clone()]);
    }

    #[test]
    fn test_get_node_id_by_name_duplicate_names_first_wins_then_fallback() {
        let mut graph = WorkflowGraph::new();
        let n1 = WorkflowNode::new(
            "dup",
            "",
            NodeType::Transform {
                transformation: "one".to_string(),
            },
        );
        let n2 = WorkflowNode::new(
            "dup",
            "",
            NodeType::Transform {
                transformation: "two".to_string(),
            },
        );
        let n1_id = n1.id.clone();
        let n2_id = n2.id.clone();

        graph.add_node(n1).expect("add first dup");
        graph.add_node(n2).expect("add second dup");

        let winner = graph
            .get_node_id_by_name("dup")
            .expect("winner should exist");
        assert!(winner == n1_id || winner == n2_id);

        graph
            .remove_node(&winner)
            .expect("remove canonical duplicate name");
        let fallback = graph
            .get_node_id_by_name("dup")
            .expect("fallback should exist after removing winner");
        assert_ne!(fallback, winner);
    }
}
