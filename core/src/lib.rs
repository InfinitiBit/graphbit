//! # `GraphBit` Core Library
//!
//! The core library provides the foundational types, traits, and algorithms
//! for building and executing agentic workflows in `GraphBit`.

// Use jemalloc as the global allocator for better performance
// Disable for Python bindings to avoid TLS block allocation issues
// Also disable on Windows where jemalloc support is problematic
#[cfg(all(not(feature = "python"), unix))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod agents;
pub mod document_loader;
pub mod embeddings;
pub mod errors;
pub mod graph;
pub mod llm;
pub mod text_splitter;
pub mod types;
pub mod validation;
pub mod workflow;

#[cfg(test)]
mod workflow_tests;

// Re-export important types for convenience - only keep what's actually used
pub use agents::{Agent, AgentBuilder, AgentConfig, AgentTrait};
pub use document_loader::{DocumentContent, DocumentLoader, DocumentLoaderConfig};
pub use embeddings::{
    EmbeddingConfig, EmbeddingProvider, EmbeddingRequest, EmbeddingResponse, EmbeddingService,
};
pub use errors::{GraphBitError, GraphBitResult};
pub use graph::{NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode};
pub use llm::{LlmConfig, LlmProvider, LlmResponse};
pub use text_splitter::{
    CharacterSplitter, RecursiveSplitter, SentenceSplitter, SplitterStrategy, TextChunk,
    TextSplitterConfig, TextSplitterFactory, TextSplitterTrait, TokenSplitter,
};
pub use types::{
    AgentCapability, AgentId, AgentMessage, MessageContent, NodeExecutionResult, NodeId,
    WorkflowContext, WorkflowExecutionStats, WorkflowId, WorkflowState,
};
pub use validation::ValidationResult;
pub use workflow::{Workflow, WorkflowBuilder, WorkflowExecutor};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the `GraphBit` core library with default configuration
pub fn init() -> GraphBitResult<()> {
    // Use try_init to avoid panicking if a global subscriber is already set
    let _ = tracing_subscriber::fmt::try_init();
    tracing::info!("GraphBit Core v{} initialized", VERSION);
    Ok(())
}
