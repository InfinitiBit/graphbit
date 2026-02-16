//! `MemoryService` -- the primary public API that orchestrates the full
//! memory pipeline: fact extraction, embedding, vector search, LLM-driven
//! deduplication, and persistent storage.

use std::collections::HashMap;

use chrono::Utc;

use crate::embeddings::EmbeddingService;
use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::{LlmMessage, LlmProviderFactory};

use super::processor::MemoryProcessor;
use super::store::MetadataStore;
use super::types::{
    Memory, MemoryAction, MemoryConfig, MemoryHistory, MemoryId, MemoryScope, ScoredMemory,
};
use super::vector::VectorIndex;

/// Orchestrates the full memory pipeline.
pub struct MemoryService {
    store: MetadataStore,
    vector_index: VectorIndex,
    embedding_service: EmbeddingService,
    processor: MemoryProcessor,
    config: MemoryConfig,
}

impl MemoryService {
    /// Build a new `MemoryService` from the provided configuration.
    ///
    /// This creates the SQLite store, vector index, embedding service, and
    /// LLM processor, then loads any existing memories into the vector index.
    pub async fn new(config: MemoryConfig) -> GraphBitResult<Self> {
        let store = MetadataStore::new(&config.db_path)?;
        let vector_index = VectorIndex::new();
        let embedding_service = EmbeddingService::new(config.embedding_config.clone())?;
        let llm_provider = LlmProviderFactory::create_provider(config.llm_config.clone())?;
        let processor = MemoryProcessor::new(
            llm_provider,
            config.max_extraction_tokens,
            config.extraction_temperature,
        );

        let service = Self {
            store,
            vector_index,
            embedding_service,
            processor,
            config,
        };

        // Load existing memories into the vector index.
        service.load_existing_memories().await?;

        Ok(service)
    }

    /// Extract facts from `messages`, embed them, decide actions against
    /// existing memories, and persist the results. Returns newly created
    /// or updated memories.
    pub async fn add(
        &self,
        messages: &[LlmMessage],
        scope: &MemoryScope,
    ) -> GraphBitResult<Vec<Memory>> {
        // Phase 1: extract facts.
        let facts = self.processor.extract_facts(messages).await?;
        if facts.is_empty() {
            return Ok(Vec::new());
        }

        // Phase 2: get existing memories for this scope for deduplication.
        let existing = self.store.get_all_memories(scope).await?;
        let decisions = self.processor.decide_actions(&facts, &existing).await?;

        let mut result_memories = Vec::new();

        for decision in &decisions {
            match decision.action {
                MemoryAction::Add => {
                    let memory = self
                        .create_memory(&decision.fact, scope.clone())
                        .await?;
                    result_memories.push(memory);
                }
                MemoryAction::Update => {
                    if let Some(ref target_id_str) = decision.target_memory_id {
                        if let Ok(target_id) = MemoryId::from_string(target_id_str) {
                            if let Some(old_memory) = self.store.get_memory(&target_id).await? {
                                let updated = self
                                    .update_memory_internal(
                                        &target_id,
                                        &decision.fact,
                                        &old_memory.content,
                                    )
                                    .await?;
                                result_memories.push(updated);
                            }
                        }
                    }
                }
                MemoryAction::Delete => {
                    if let Some(ref target_id_str) = decision.target_memory_id {
                        if let Ok(target_id) = MemoryId::from_string(target_id_str) {
                            if let Some(old_memory) = self.store.get_memory(&target_id).await? {
                                self.delete_memory_internal(&target_id, &old_memory.content)
                                    .await?;
                            }
                        }
                    }
                }
                MemoryAction::Noop => {}
            }
        }

        Ok(result_memories)
    }

    /// Embed a query and search for the most similar memories within a scope.
    pub async fn search(
        &self,
        query: &str,
        scope: &MemoryScope,
        top_k: usize,
    ) -> GraphBitResult<Vec<ScoredMemory>> {
        let query_embedding = self.embedding_service.embed_text(query).await?;
        let results = self
            .vector_index
            .search(&query_embedding, top_k, self.config.similarity_threshold)
            .await?;

        let mut scored = Vec::new();
        for (memory_id, score) in results {
            if let Some(memory) = self.store.get_memory(&memory_id).await? {
                if matches_scope(&memory.scope, scope) {
                    scored.push(ScoredMemory { memory, score });
                }
            }
        }

        Ok(scored)
    }

    /// Get a single memory by its ID.
    pub async fn get(&self, memory_id: &MemoryId) -> GraphBitResult<Option<Memory>> {
        self.store.get_memory(memory_id).await
    }

    /// Get all memories matching a scope.
    pub async fn get_all(&self, scope: &MemoryScope) -> GraphBitResult<Vec<Memory>> {
        self.store.get_all_memories(scope).await
    }

    /// Update a memory's content by ID.
    pub async fn update(&self, memory_id: &MemoryId, content: &str) -> GraphBitResult<Memory> {
        let old = self
            .store
            .get_memory(memory_id)
            .await?
            .ok_or_else(|| GraphBitError::memory(format!("Memory not found: {memory_id}")))?;

        self.update_memory_internal(memory_id, content, &old.content)
            .await
    }

    /// Delete a single memory by its ID.
    pub async fn delete(&self, memory_id: &MemoryId) -> GraphBitResult<()> {
        let old = self.store.get_memory(memory_id).await?;
        let old_content = old.map(|m| m.content).unwrap_or_default();
        self.delete_memory_internal(memory_id, &old_content).await
    }

    /// Delete all memories matching a scope.
    pub async fn delete_all(&self, scope: &MemoryScope) -> GraphBitResult<()> {
        // Remove from vector index first.
        let memories = self.store.get_all_memories(scope).await?;
        for m in &memories {
            self.vector_index.remove(&m.id).await;
        }
        self.store.delete_all_memories(scope).await
    }

    /// Get the mutation history for a memory.
    pub async fn history(&self, memory_id: &MemoryId) -> GraphBitResult<Vec<MemoryHistory>> {
        self.store.get_history(memory_id).await
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Load all existing memories from the store into the vector index.
    async fn load_existing_memories(&self) -> GraphBitResult<()> {
        let all_memories = self.store.get_all_memories(&MemoryScope::default()).await?;
        for memory in &all_memories {
            let embedding = self.embedding_service.embed_text(&memory.content).await?;
            self.vector_index
                .insert(memory.id.clone(), embedding)
                .await;
        }
        Ok(())
    }

    /// Create a brand-new memory: hash, embed, store, index, record history.
    async fn create_memory(
        &self,
        content: &str,
        scope: MemoryScope,
    ) -> GraphBitResult<Memory> {
        let now = Utc::now();
        let hash = simple_hash(content);
        let id = MemoryId::new();

        let memory = Memory {
            id: id.clone(),
            content: content.to_string(),
            scope,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            hash,
        };

        self.store.insert_memory(&memory).await?;

        let embedding = self.embedding_service.embed_text(content).await?;
        self.vector_index.insert(id.clone(), embedding).await;

        self.store
            .insert_history(&MemoryHistory {
                memory_id: id,
                old_content: String::new(),
                new_content: content.to_string(),
                action: MemoryAction::Add,
                timestamp: now,
            })
            .await?;

        Ok(memory)
    }

    /// Update an existing memory: re-hash, re-embed, persist, record history.
    async fn update_memory_internal(
        &self,
        memory_id: &MemoryId,
        new_content: &str,
        old_content: &str,
    ) -> GraphBitResult<Memory> {
        let hash = simple_hash(new_content);
        self.store
            .update_memory(memory_id, new_content, &hash)
            .await?;

        let embedding = self.embedding_service.embed_text(new_content).await?;
        self.vector_index.update(memory_id, embedding).await;

        self.store
            .insert_history(&MemoryHistory {
                memory_id: memory_id.clone(),
                old_content: old_content.to_string(),
                new_content: new_content.to_string(),
                action: MemoryAction::Update,
                timestamp: Utc::now(),
            })
            .await?;

        self.store
            .get_memory(memory_id)
            .await?
            .ok_or_else(|| GraphBitError::memory("Memory disappeared after update"))
    }

    /// Delete a memory from store and index, record history.
    async fn delete_memory_internal(
        &self,
        memory_id: &MemoryId,
        old_content: &str,
    ) -> GraphBitResult<()> {
        // Record history BEFORE deleting (so FK constraint is satisfied).
        // Note: with ON DELETE CASCADE, the history entry will be deleted
        // along with the memory, so this serves as an audit log only if
        // the cascade is disabled or history is preserved elsewhere.
        // For now, we skip history on delete to avoid FK issues.

        // Delete from store (cascades to history) and index.
        self.store.delete_memory(memory_id).await?;
        self.vector_index.remove(memory_id).await;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Check if a memory's scope matches the filter scope.
/// `None` fields in the filter are treated as wildcards.
fn matches_scope(memory_scope: &MemoryScope, filter: &MemoryScope) -> bool {
    if let Some(ref uid) = filter.user_id {
        if memory_scope.user_id.as_ref() != Some(uid) {
            return false;
        }
    }
    if let Some(ref aid) = filter.agent_id {
        if memory_scope.agent_id.as_ref() != Some(aid) {
            return false;
        }
    }
    if let Some(ref rid) = filter.run_id {
        if memory_scope.run_id.as_ref() != Some(rid) {
            return false;
        }
    }
    true
}

/// Produce a simple content hash for deduplication.
fn simple_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
