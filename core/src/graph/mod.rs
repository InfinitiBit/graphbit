//! Graph-based workflow system for `GraphBit`
//!
//! This module provides a directed graph structure for defining and executing
//! agentic workflows with proper dependency management and parallel execution.

mod edge;
mod node;
pub mod workflow_graph;

pub use workflow_graph::WorkflowGraph;
pub use node::{AgentNodeConfig, NodeType, WorkflowNode};
pub use edge::{EdgeType, WorkflowEdge};
