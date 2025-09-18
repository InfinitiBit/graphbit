//! Graph-based workflow system for `GraphBit`
//!
//! This module provides a directed graph structure for defining and executing
//! agentic workflows with proper dependency management and parallel execution.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::{NodeId, RetryConfig};
use petgraph::{
    algo::{is_cyclic_directed, toposort},
    graph::{DiGraph, NodeIndex},
    Direction,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A workflow graph that defines the structure and execution flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    /// Graph structure
    #[serde(skip)]
    graph: DiGraph<WorkflowNode, WorkflowEdge>,
    /// Mapping from `NodeId` to graph indices
    #[serde(skip)]
    node_map: HashMap<NodeId, NodeIndex>,
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
            nodes: HashMap::with_capacity(16),    // Pre-allocate capacity
            edges: Vec::with_capacity(16),        // Pre-allocate capacity
            metadata: HashMap::new(),
            dependencies_cache: HashMap::with_capacity(16),
            dependents_cache: HashMap::with_capacity(16),
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

    /// Rebuild the graph structure from serialized data
    /// This must be called after deserialization since graph and `node_map` are not serialized
    pub fn rebuild_graph(&mut self) -> GraphBitResult<()> {
        // Clear existing graph structures
        self.graph = DiGraph::new();
        self.node_map.clear();
        self.invalidate_caches();

        // Pre-allocate capacity based on existing data
        self.node_map.reserve(self.nodes.len());

        // Re-add all nodes to the graph
        for (node_id, node) in &self.nodes {
            let graph_index = self.graph.add_node(node.clone());
            self.node_map.insert(node_id.clone(), graph_index);
        }

        // Re-add all edges to the graph
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
        self.edges.push((from, to, edge));

        // Invalidate caches since graph structure changed
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

        // Remove edges involving this node
        self.edges
            .retain(|(from, to, _)| from != node_id && to != node_id);

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
            // Find the NodeId for this graph index
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
                // Find the NodeId for this neighbor
                for (neighbor_id, &idx) in &self.node_map {
                    if idx == neighbor_index {
                        dependencies.push(neighbor_id.clone());
                        break;
                    }
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
                // Find the NodeId for this neighbor
                for (neighbor_id, &idx) in &self.node_map {
                    if idx == neighbor_index {
                        dependents.push(neighbor_id.clone());
                        break;
                    }
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

        // Enforce unique agent IDs across all agent nodes
        {
            use std::collections::HashMap;
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
            .and_then(|v| v.as_bool())
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

/// A node in the workflow graph representing a single execution unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    /// Unique identifier for the node
    pub id: NodeId,
    /// Human-readable name
    pub name: String,
    /// Description of what this node does
    pub description: String,
    /// Type of the node
    pub node_type: NodeType,
    /// Configuration for the node
    pub config: HashMap<String, serde_json::Value>,
    /// Input schema for validation
    pub input_schema: Option<serde_json::Value>,
    /// Output schema for validation
    pub output_schema: Option<serde_json::Value>,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl WorkflowNode {
    /// Create a new workflow node
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        node_type: NodeType,
    ) -> Self {
        Self {
            id: NodeId::new(),
            name: name.into(),
            description: description.into(),
            node_type,
            config: HashMap::with_capacity(8), // Pre-allocate for config parameters
            input_schema: None,
            output_schema: None,
            retry_config: RetryConfig::default(),
            timeout_seconds: None,
            tags: Vec::new(),
        }
    }

    /// Set node configuration
    pub fn with_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.insert(key, value);
        self
    }

    /// Set input schema
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Set output schema
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Validate the node configuration
    pub fn validate(&self) -> GraphBitResult<()> {
        // Validate node type specific requirements
        match &self.node_type {
            NodeType::Agent { agent_id, .. } => {
                if agent_id.to_string().is_empty() {
                    return Err(GraphBitError::graph(
                        "Agent node must have a valid agent_id",
                    ));
                }
            }
            NodeType::Condition { expression } => {
                if expression.is_empty() {
                    return Err(GraphBitError::graph(
                        "Condition node must have an expression",
                    ));
                }
            }
            NodeType::Transform { transformation } => {
                if transformation.is_empty() {
                    return Err(GraphBitError::graph(
                        "Transform node must have a transformation",
                    ));
                }
            }
            NodeType::DocumentLoader {
                document_type,
                source_path,
                ..
            } => {
                if document_type.is_empty() {
                    return Err(GraphBitError::graph(
                        "DocumentLoader node must have a document_type",
                    ));
                }
                if source_path.is_empty() {
                    return Err(GraphBitError::graph(
                        "DocumentLoader node must have a source_path",
                    ));
                }
                // Validate supported document types
                let supported_types = ["pdf", "txt", "docx", "json", "csv", "xml", "html"];
                if !supported_types.contains(&document_type.to_lowercase().as_str()) {
                    return Err(GraphBitError::graph(format!(
                        "Unsupported document type: {document_type}. Supported types: {supported_types:?}"
                    )));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Types of workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    /// Agent execution node
    Agent {
        /// Unique identifier for the agent
        agent_id: crate::types::AgentId,
        /// Template for the prompt to send to the agent
        prompt_template: String,
    },
    /// Conditional branching node
    Condition {
        /// Boolean expression to evaluate
        expression: String,
    },
    /// Data transformation node
    Transform {
        /// Transformation logic to apply
        transformation: String,
    },
    /// Parallel execution splitter
    Split,
    /// Parallel execution joiner
    Join,
    /// Delay/wait node
    Delay {
        /// Duration to wait in seconds
        duration_seconds: u64,
    },
    /// HTTP request node
    HttpRequest {
        /// Target URL for the request
        url: String,
        /// HTTP method (GET, POST, etc.)
        method: String,
        /// HTTP headers to include
        headers: HashMap<String, String>,
    },
    /// Custom function node
    Custom {
        /// Name of the custom function to execute
        function_name: String,
    },
    /// Document loading node
    DocumentLoader {
        /// Type of document to load
        document_type: String,
        /// Path to the source document
        source_path: String,
        /// Optional encoding specification
        encoding: Option<String>,
    },
}

/// An edge in the workflow graph representing data flow and dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    /// Type of the edge
    pub edge_type: EdgeType,
    /// Condition for edge traversal
    pub condition: Option<String>,
    /// Data transformation applied to values flowing through this edge
    pub transform: Option<String>,
    /// Edge metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl WorkflowEdge {
    /// Create a new data flow edge
    pub fn data_flow() -> Self {
        Self {
            edge_type: EdgeType::DataFlow,
            condition: None,
            transform: None,
            metadata: HashMap::with_capacity(4), // Pre-allocate for metadata
        }
    }

    /// Create a new control flow edge
    pub fn control_flow() -> Self {
        Self {
            edge_type: EdgeType::ControlFlow,
            condition: None,
            transform: None,
            metadata: HashMap::with_capacity(4), // Pre-allocate for metadata
        }
    }

    /// Create a conditional edge
    pub fn conditional(condition: impl Into<String>) -> Self {
        Self {
            edge_type: EdgeType::Conditional,
            condition: Some(condition.into()),
            transform: None,
            metadata: HashMap::with_capacity(4), // Pre-allocate for metadata
        }
    }

    /// Add a transformation to the edge
    pub fn with_transform(mut self, transform: impl Into<String>) -> Self {
        self.transform = Some(transform.into());
        self
    }

    /// Add metadata to the edge
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Types of edges in the workflow graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    /// Data flows from one node to another
    DataFlow,
    /// Control dependency (execution order)
    ControlFlow,
    /// Conditional edge (only traversed if condition is true)
    Conditional,
    /// Error handling edge
    ErrorHandling,
}
