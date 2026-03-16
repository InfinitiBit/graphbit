//! Memory layer for `GraphBit`
//!
//! Provides LLM-driven fact extraction from conversations, vector-based semantic
//! search, SQLite-backed persistent storage, and scoped memory isolation.

pub mod types;
pub mod vector;

pub use types::{
    Memory, MemoryAction, MemoryConfig, MemoryDecision, MemoryHistory, MemoryId, MemoryScope,
    ScoredMemory,
};
