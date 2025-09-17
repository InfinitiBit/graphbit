//! `GraphBit` - Agentic Workflow Automation Framework
//!
//! `GraphBit` is a declarative framework for building agentic workflows that can
//! dynamically adapt to changing requirements and data flows.
//!
//! ## Features
//!
//! - **Dynamic Workflow Graphs**: Build workflows that can adapt at runtime
//! - **Multi-LLM Support**: Integrate with `OpenAI`, `Anthropic`, `Ollama`, and more
//! - **Agent-Based Architecture**: Compose complex workflows from simple agents
//! - **Python Bindings**: Use GraphBit from Python applications
//! - **Validation & Monitoring**: Built-in validation and execution monitoring
//!
//! ## Quick Start
//!
//! ```rust
//! use graphbit_core::{WorkflowGraph, WorkflowNode, NodeType, AgentId};
//!
//! // Create a simple workflow
//! let mut graph = WorkflowGraph::new();
//! let node_type = NodeType::Agent {
//!     agent_id: AgentId::new(),
//!     prompt_template: "You are a helpful assistant".to_string(),
//! };
//! let node = WorkflowNode::new("start", "A starting node", node_type);
//! graph.add_node(node).unwrap();
//! ```
//!
//! For more examples, see the `examples/` directory.

// Re-export core functionality
pub use graphbit_core::*;

// Re-export common types and utilities
pub mod prelude {
    //! Commonly used types and traits
    pub use graphbit_core::{
        agents::{Agent, AgentConfig, AgentTrait},
        embeddings::{EmbeddingProvider, EmbeddingRequest, EmbeddingResponse},
        graph::{NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode},
        llm::{LlmProvider, LlmResponse},
        types::{NodeExecutionResult, WorkflowContext, WorkflowState},
        validation::ValidationResult,
        workflow::{Workflow, WorkflowBuilder, WorkflowExecutor},
        GraphBitResult,
    };
}
