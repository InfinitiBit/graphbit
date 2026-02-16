//! # `GraphBit` Core Library
//!
//! The core library provides the foundational types, traits, and algorithms
//! for building and executing agentic workflows in `GraphBit`.

// Memory allocator configuration - optimized per platform
// Disabled for Python bindings to avoid TLS block allocation issues

// Linux: jemalloc
#[cfg(all(not(feature = "python"), target_os = "linux"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

// macOS: mimalloc
#[cfg(all(not(feature = "python"), target_os = "macos"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Windows: mimalloc
#[cfg(all(not(feature = "python"), target_os = "windows"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Other Unix systems: jemalloc (broad compatibility)
#[cfg(all(
    not(feature = "python"),
    unix,
    not(any(target_os = "linux", target_os = "macos"))
))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod agents;
pub mod document_loader;
pub mod embeddings;
pub mod errors;
pub mod graph;
pub mod llm;
pub mod memory;
pub mod text_splitter;
pub mod types;
pub mod validation;
pub mod workflow;

// Re-export important types for convenience - only keep what's actually used
pub use agents::{Agent, AgentBuilder, AgentConfig, AgentTrait};
pub use document_loader::{DocumentContent, DocumentLoader, DocumentLoaderConfig};
pub use embeddings::{
    EmbeddingConfig, EmbeddingProvider, EmbeddingRequest, EmbeddingResponse, EmbeddingService,
};
pub use errors::{GraphBitError, GraphBitResult};
pub use graph::{NodeType, WorkflowEdge, WorkflowGraph, WorkflowNode};
pub use llm::{LlmConfig, LlmProvider, LlmResponse};
pub use memory::{
    Memory, MemoryConfig, MemoryHistory, MemoryId, MemoryScope, MemoryService, ScoredMemory,
};
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
///
/// Note: Tracing/logging is NOT initialized here - the bindings (Python/JavaScript)
/// control logging setup and it's disabled by default for cleaner output.
/// To enable logging from Python: `graphbit.init(enable_tracing=True, log_level='info')`
pub fn init() -> GraphBitResult<()> {
    // Tracing is intentionally NOT initialized here.
    // The Python/JS bindings handle tracing setup, disabled by default.
    // This keeps output clean unless explicitly enabled by the user.
    Ok(())
}
