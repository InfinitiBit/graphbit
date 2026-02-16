//! Memory layer for `GraphBit`
//!
//! Provides LLM-driven fact extraction from conversations, vector-based semantic
//! search, SQLite-backed persistent storage, and scoped memory isolation.

pub mod processor;
pub mod service;
pub mod store;
pub mod types;
pub mod vector;

pub use service::MemoryService;
pub use types::{
    Memory, MemoryAction, MemoryConfig, MemoryDecision, MemoryHistory, MemoryId, MemoryScope,
    ScoredMemory,
};
