//! Memory system for stateful AI agents
//!
//! This module provides a comprehensive memory system that enables agents to remember,
//! learn, and evolve across interactions. It implements four types of memory:
//!
//! - **Working Memory**: Short-term session-based context for current conversations
//! - **Factual Memory**: Long-term structured knowledge (facts, preferences, settings)
//! - **Episodic Memory**: Records of specific past conversations and interactions
//! - **Semantic Memory**: General knowledge built from patterns and insights over time
//!
//! # Features
//!
//! - LLM-based intelligent memory extraction
//! - Vector-based semantic search with embeddings
//! - Time and importance-based memory decay
//! - Graph connections between related memories
//! - Sub-50ms retrieval performance with in-memory caching
//! - Multimodal support (text and images)
//! - Comprehensive observability and monitoring
//!
//! # Example
//!
//! ```rust
//! use graphbit_core::memory::{MemoryManager, MemoryConfig, MemoryQuery};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a memory manager
//! let mut manager = MemoryManager::with_defaults();
//!
//! // Start a session
//! manager.start_session("session_123".to_string());
//!
//! // Store working memory
//! manager.store_working("User prefers dark mode".to_string()).await?;
//!
//! // Store a fact
//! manager.store_fact("theme".to_string(), "dark".to_string()).await?;
//!
//! // Retrieve memories
//! let query = MemoryQuery::new("user preferences".to_string());
//! let results = manager.retrieve(query).await?;
//!
//! // Get statistics
//! let stats = manager.get_stats().await;
//! println!("Total memories: {}", stats.total_memories);
//! # Ok(())
//! # }
//! ```

pub mod decay;
pub mod episodic;
pub mod extraction;
pub mod factual;
pub mod manager;
pub mod retrieval;
pub mod semantic;
pub mod storage;
pub mod tools;
pub mod types;
pub mod working;

// Re-export main types for convenience
pub use decay::{DecayConfig, DecayManager, DecayStats};
pub use episodic::{Episode, EpisodicMemory};
pub use extraction::{ExtractionConfig, MemoryExtractor};
pub use factual::FactualMemory;
pub use manager::{MemoryConfig, MemoryManager, MemoryStats};
pub use retrieval::{MemoryRetriever, RetrievalResult};
pub use semantic::{ConceptRelation, SemanticConcept, SemanticMemory};
pub use storage::{InMemoryStorage, MemoryStorage, SharedStorage};
pub use types::{MemoryEntry, MemoryId, MemoryMetadata, MemoryQuery, MemoryType};
pub use working::WorkingMemory;

