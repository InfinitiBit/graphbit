//! Memory manager coordinating all memory types
//!
//! This module provides the central MemoryManager that coordinates working,
//! factual, episodic, and semantic memory systems.

use super::decay::{DecayConfig, DecayManager, DecayStats};
use super::episodic::EpisodicMemory;
use super::factual::FactualMemory;
use super::retrieval::{MemoryRetriever, RetrievalResult};
use super::semantic::SemanticMemory;
use super::storage::{create_shared_storage_with_capacities, SharedStorage};
use super::types::{MemoryEntry, MemoryId, MemoryQuery, MemoryType};
use super::working::WorkingMemory;
use crate::embeddings::EmbeddingService;
use crate::errors::GraphBitResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for the memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Enable working memory
    pub enable_working: bool,
    /// Enable factual memory
    pub enable_factual: bool,
    /// Enable episodic memory
    pub enable_episodic: bool,
    /// Enable semantic memory
    pub enable_semantic: bool,
    /// Maximum memories per type
    pub capacities: HashMap<MemoryType, usize>,
    /// Decay configuration
    pub decay_config: DecayConfig,
    /// Enable automatic embedding generation
    pub auto_embed: bool,
}

impl MemoryConfig {
    /// Create a new memory configuration with defaults
    pub fn new() -> Self {
        let mut capacities = HashMap::with_capacity(4);
        for mem_type in MemoryType::all() {
            capacities.insert(mem_type, mem_type.default_capacity());
        }

        Self {
            enable_working: true,
            enable_factual: true,
            enable_episodic: true,
            enable_semantic: true,
            capacities,
            decay_config: DecayConfig::new(),
            auto_embed: true,
        }
    }

    /// Create a minimal configuration (only working memory)
    pub fn minimal() -> Self {
        let mut capacities = HashMap::with_capacity(1);
        capacities.insert(MemoryType::Working, 50);

        Self {
            enable_working: true,
            enable_factual: false,
            enable_episodic: false,
            enable_semantic: false,
            capacities,
            decay_config: DecayConfig::disabled(),
            auto_embed: false,
        }
    }

    /// Create a full-featured configuration
    pub fn full() -> Self {
        let mut capacities = HashMap::with_capacity(4);
        capacities.insert(MemoryType::Working, 200);
        capacities.insert(MemoryType::Factual, 2000);
        capacities.insert(MemoryType::Episodic, 1000);
        capacities.insert(MemoryType::Semantic, 500);

        Self {
            enable_working: true,
            enable_factual: true,
            enable_episodic: true,
            enable_semantic: true,
            capacities,
            decay_config: DecayConfig::conservative(),
            auto_embed: true,
        }
    }

    /// Set capacity for a memory type
    pub fn set_capacity(&mut self, memory_type: MemoryType, capacity: usize) {
        self.capacities.insert(memory_type, capacity);
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Central memory manager coordinating all memory types
pub struct MemoryManager {
    /// Configuration
    config: MemoryConfig,
    /// Shared storage backend
    storage: SharedStorage,
    /// Working memory
    working: WorkingMemory,
    /// Factual memory
    factual: FactualMemory,
    /// Episodic memory
    episodic: EpisodicMemory,
    /// Semantic memory
    semantic: SemanticMemory,
    /// Memory retriever
    retriever: MemoryRetriever,
    /// Decay manager
    decay_manager: DecayManager,
    /// Embedding service (optional)
    #[allow(dead_code)]
    embedding_service: Option<Arc<EmbeddingService>>,
}

// Manual Debug implementation since SharedStorage contains a trait object
impl std::fmt::Debug for MemoryManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryManager")
            .field("config", &self.config)
            .field("storage", &"<SharedStorage>")
            .field("working", &self.working)
            .field("factual", &self.factual)
            .field("episodic", &self.episodic)
            .field("semantic", &self.semantic)
            .field("retriever", &self.retriever)
            .field("decay_manager", &self.decay_manager)
            .field("embedding_service", &self.embedding_service.is_some())
            .finish()
    }
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(config: MemoryConfig, embedding_service: Option<Arc<EmbeddingService>>) -> Self {
        let storage = create_shared_storage_with_capacities(config.capacities.clone());
        let retriever = MemoryRetriever::new(embedding_service.clone());
        let decay_manager = DecayManager::new(config.decay_config.clone());

        Self {
            config,
            storage,
            working: WorkingMemory::new(),
            factual: FactualMemory::new(),
            episodic: EpisodicMemory::new(),
            semantic: SemanticMemory::new(),
            retriever,
            decay_manager,
            embedding_service,
        }
    }

    /// Create a memory manager with default configuration
    pub fn with_defaults() -> Self {
        Self::new(MemoryConfig::default(), None)
    }

    /// Create a memory manager with embedding support
    pub fn with_embeddings(embedding_service: Arc<EmbeddingService>) -> Self {
        Self::new(MemoryConfig::default(), Some(embedding_service))
    }

    // Working Memory Methods
    
    /// Start a new working memory session
    pub fn start_session(&mut self, session_id: String) {
        if self.config.enable_working {
            self.working.start_session(session_id);
        }
    }

    /// End the current working memory session
    pub async fn end_session(&mut self) -> GraphBitResult<usize> {
        if self.config.enable_working {
            let mut storage = self.storage.write().await;
            self.working.end_session(&mut **storage)
        } else {
            Ok(0)
        }
    }

    /// Store a working memory
    pub async fn store_working(&self, content: String) -> GraphBitResult<MemoryId> {
        if !self.config.enable_working {
            return Err(crate::errors::GraphBitError::memory(
                "Working memory is disabled",
            ));
        }

        let mut storage = self.storage.write().await;
        self.working.store(content, &mut **storage)
    }

    /// Get working memory context
    pub async fn get_working_context(&self) -> String {
        if !self.config.enable_working {
            return String::new();
        }

        let storage = self.storage.read().await;
        self.working.get_session_context(&**storage)
    }

    // Factual Memory Methods

    /// Store a fact
    pub async fn store_fact(&self, key: String, value: String) -> GraphBitResult<MemoryId> {
        if !self.config.enable_factual {
            return Err(crate::errors::GraphBitError::memory(
                "Factual memory is disabled",
            ));
        }

        let mut storage = self.storage.write().await;
        self.factual.store_fact(key, value, &mut **storage)
    }

    /// Get a fact
    pub async fn get_fact(&self, key: &str) -> Option<String> {
        if !self.config.enable_factual {
            return None;
        }

        let storage = self.storage.read().await;
        self.factual.get_fact(key, &**storage)
    }

    /// Update a fact
    pub async fn update_fact(&self, key: &str, value: String) -> GraphBitResult<bool> {
        if !self.config.enable_factual {
            return Ok(false);
        }

        let mut storage = self.storage.write().await;
        self.factual.update_fact(key, value, &mut **storage)
    }

    // Episodic Memory Methods

    /// Start recording an episode
    pub fn start_episode(&mut self, title: String) {
        if self.config.enable_episodic {
            self.episodic.start_episode(title);
        }
    }

    /// Add content to current episode
    pub fn add_to_episode(&mut self, content: String) {
        if self.config.enable_episodic {
            self.episodic.add_to_episode(content);
        }
    }

    /// End current episode
    pub async fn end_episode(&mut self) -> GraphBitResult<Option<MemoryId>> {
        if !self.config.enable_episodic {
            return Ok(None);
        }

        let mut storage = self.storage.write().await;
        self.episodic.end_episode(&mut **storage)
    }

    /// Get recent episodes
    pub async fn get_recent_episodes(&self, limit: usize) -> Vec<MemoryEntry> {
        if !self.config.enable_episodic {
            return Vec::new();
        }

        let storage = self.storage.read().await;
        self.episodic.get_recent_episodes(limit, &**storage)
    }

    // Semantic Memory Methods

    /// Store a semantic concept
    pub async fn store_concept(
        &mut self,
        concept: super::semantic::SemanticConcept,
    ) -> GraphBitResult<MemoryId> {
        if !self.config.enable_semantic {
            return Err(crate::errors::GraphBitError::memory(
                "Semantic memory is disabled",
            ));
        }

        let mut storage = self.storage.write().await;
        self.semantic.store_concept(concept, &mut **storage)
    }

    /// Reinforce a concept
    pub async fn reinforce_concept(&mut self, name: &str) -> GraphBitResult<bool> {
        if !self.config.enable_semantic {
            return Ok(false);
        }

        let mut storage = self.storage.write().await;
        self.semantic.reinforce_concept(name, &mut **storage)
    }

    // Retrieval Methods

    /// Retrieve memories matching a query
    pub async fn retrieve(&self, query: MemoryQuery) -> GraphBitResult<Vec<RetrievalResult>> {
        let storage = self.storage.read().await;
        self.retriever.retrieve(&query, &**storage).await
    }

    /// Get a specific memory by ID
    pub async fn get_memory(&self, id: &MemoryId) -> Option<MemoryEntry> {
        let mut storage = self.storage.write().await;
        self.retriever.get_by_id(id, &mut **storage)
    }

    // Decay Methods

    /// Run memory decay
    pub async fn run_decay(&mut self) -> GraphBitResult<DecayStats> {
        let mut storage = self.storage.write().await;
        self.decay_manager.run_decay(&mut **storage)
    }

    /// Check if decay should run
    pub fn should_run_decay(&self) -> bool {
        self.decay_manager.should_run_decay()
    }

    // Statistics and Monitoring

    /// Get memory statistics
    pub async fn get_stats(&self) -> MemoryStats {
        let storage = self.storage.read().await;
        
        MemoryStats {
            total_memories: storage.count(),
            working_count: storage.count_by_type(MemoryType::Working),
            factual_count: storage.count_by_type(MemoryType::Factual),
            episodic_count: storage.count_by_type(MemoryType::Episodic),
            semantic_count: storage.count_by_type(MemoryType::Semantic),
            current_session: self.working.get_session_id().cloned(),
        }
    }

    /// Clear all memories
    pub async fn clear_all(&self) -> GraphBitResult<()> {
        let mut storage = self.storage.write().await;
        storage.clear();
        Ok(())
    }

    /// Remove a specific memory by ID
    pub async fn remove_memory(&self, id: &MemoryId) -> GraphBitResult<bool> {
        let mut storage = self.storage.write().await;
        storage.delete(id)
    }

    /// Get current session ID
    pub async fn get_current_session(&self) -> Option<String> {
        self.working.get_session_id().cloned()
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total number of memories across all types
    pub total_memories: usize,
    /// Number of working memories
    pub working_count: usize,
    /// Number of factual memories
    pub factual_count: usize,
    /// Number of episodic memories
    pub episodic_count: usize,
    /// Number of semantic memories
    pub semantic_count: usize,
    /// Current session ID (if any)
    pub current_session: Option<String>,
}

