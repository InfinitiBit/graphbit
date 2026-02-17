//! Graph-based workflow system for `GraphBit`
//!
//! This module provides a directed graph structure for defining and executing
//! agentic workflows with proper dependency management and parallel execution.

mod edge;
mod graph;
mod node;

pub use edge::{EdgeType, WorkflowEdge};
pub use graph::WorkflowGraph;
pub use node::{NodeType, WorkflowNode};
