//! Memory retrieval system with vector-based semantic search
//!
//! This module provides efficient memory retrieval using embeddings and
//! similarity scoring for semantic search capabilities.

use super::storage::MemoryStorage;
use super::types::{MemoryEntry, MemoryId, MemoryQuery};
use crate::embeddings::EmbeddingService;
use crate::errors::{GraphBitError, GraphBitResult};
use std::sync::Arc;

/// Result of a memory retrieval operation
#[derive(Debug, Clone)]
pub struct RetrievalResult {
    /// The retrieved memory entry
    pub entry: MemoryEntry,
    /// Similarity score (0.0-1.0)
    pub similarity: f32,
    /// Related memories (if requested)
    pub related: Vec<MemoryEntry>,
}

impl RetrievalResult {
    /// Create a new retrieval result
    pub fn new(entry: MemoryEntry, similarity: f32) -> Self {
        Self {
            entry,
            similarity,
            related: Vec::new(),
        }
    }

    /// Add related memories
    pub fn with_related(mut self, related: Vec<MemoryEntry>) -> Self {
        self.related = related;
        self
    }
}

/// Memory retrieval engine
pub struct MemoryRetriever {
    /// Embedding service for generating query embeddings
    #[allow(dead_code)]
    embedding_service: Option<Arc<EmbeddingService>>,
}

// Manual Debug implementation since EmbeddingService doesn't implement Debug
impl std::fmt::Debug for MemoryRetriever {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryRetriever")
            .field("embedding_service", &self.embedding_service.is_some())
            .finish()
    }
}

impl MemoryRetriever {
    /// Create a new memory retriever
    pub fn new(embedding_service: Option<Arc<EmbeddingService>>) -> Self {
        Self { embedding_service }
    }

    /// Retrieve memories matching a query
    pub async fn retrieve(
        &self,
        query: &MemoryQuery,
        storage: &dyn MemoryStorage,
    ) -> GraphBitResult<Vec<RetrievalResult>> {
        // Get candidate memories based on filters
        let candidates = self.get_candidates(query, storage);

        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        // If we have an embedding service, use semantic search
        if let Some(ref service) = self.embedding_service {
            self.semantic_search(query, candidates, service).await
        } else {
            // Fall back to keyword-based search
            self.keyword_search(query, candidates)
        }
    }

    /// Get candidate memories based on filters
    fn get_candidates<'a>(
        &self,
        query: &MemoryQuery,
        storage: &'a dyn MemoryStorage,
    ) -> Vec<&'a MemoryEntry> {
        let mut candidates = Vec::with_capacity(100);

        // Filter by memory types
        if let Some(ref types) = query.memory_types {
            for mem_type in types {
                candidates.extend(storage.list_by_type(*mem_type));
            }
        } else {
            candidates = storage.list_all();
        }

        // Filter by session if specified
        if let Some(ref session_id) = query.session_id {
            candidates.retain(|entry| {
                entry.session_id.as_ref().map_or(false, |sid| sid == session_id)
            });
        }

        // Filter by tags if specified
        if let Some(ref tags) = query.tags {
            candidates.retain(|entry| {
                tags.iter().any(|tag| entry.metadata.tags.contains(tag))
            });
        }

        candidates
    }

    /// Perform semantic search using embeddings
    async fn semantic_search(
        &self,
        query: &MemoryQuery,
        candidates: Vec<&MemoryEntry>,
        service: &EmbeddingService,
    ) -> GraphBitResult<Vec<RetrievalResult>> {
        // Generate query embedding
        let query_embedding = service
            .embed_text(&query.query)
            .await
            .map_err(|e| GraphBitError::memory(format!("Failed to generate query embedding: {}", e)))?;

        // Calculate similarities and collect results
        let mut results: Vec<RetrievalResult> = candidates
            .into_iter()
            .filter_map(|entry| {
                // Skip entries without embeddings
                let entry_embedding = entry.embedding.as_ref()?;

                // Calculate cosine similarity
                let similarity = Self::cosine_similarity(&query_embedding, entry_embedding);

                // Filter by minimum similarity
                if similarity >= query.min_similarity {
                    Some(RetrievalResult::new(entry.clone(), similarity))
                } else {
                    None
                }
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit results
        results.truncate(query.limit);

        // Add related memories if requested
        if query.include_related {
            // This would be implemented by looking up related_memories IDs
            // For now, we'll leave it as a placeholder
        }

        Ok(results)
    }

    /// Perform keyword-based search (fallback when no embeddings)
    fn keyword_search(
        &self,
        query: &MemoryQuery,
        candidates: Vec<&MemoryEntry>,
    ) -> GraphBitResult<Vec<RetrievalResult>> {
        let query_lower = query.query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        let mut results: Vec<RetrievalResult> = candidates
            .into_iter()
            .filter_map(|entry| {
                let content_lower = entry.content.to_lowercase();

                // Calculate simple keyword match score
                let matches = query_words
                    .iter()
                    .filter(|word| content_lower.contains(*word))
                    .count();

                if matches > 0 {
                    let similarity = matches as f32 / query_words.len() as f32;
                    if similarity >= query.min_similarity {
                        Some(RetrievalResult::new(entry.clone(), similarity))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit results
        results.truncate(query.limit);

        Ok(results)
    }

    /// Calculate cosine similarity between two vectors
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        (dot_product / (magnitude_a * magnitude_b)).clamp(0.0, 1.0)
    }

    /// Retrieve a specific memory by ID and record access
    pub fn get_by_id(
        &self,
        id: &MemoryId,
        storage: &mut dyn MemoryStorage,
    ) -> Option<MemoryEntry> {
        if let Some(entry) = storage.get_mut(id) {
            entry.record_access();
            Some(entry.clone())
        } else {
            None
        }
    }

    /// Retrieve related memories for a given memory
    pub fn get_related(
        &self,
        memory_id: &MemoryId,
        storage: &dyn MemoryStorage,
        limit: usize,
    ) -> Vec<MemoryEntry> {
        if let Some(entry) = storage.get(memory_id) {
            entry
                .related_memories
                .iter()
                .take(limit)
                .filter_map(|id| storage.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find memories similar to a given memory (using its embedding)
    pub async fn find_similar(
        &self,
        memory_id: &MemoryId,
        storage: &dyn MemoryStorage,
        limit: usize,
        min_similarity: f32,
    ) -> GraphBitResult<Vec<RetrievalResult>> {
        let entry = storage
            .get(memory_id)
            .ok_or_else(|| GraphBitError::memory(format!("Memory not found: {}", memory_id)))?;

        let query_embedding = entry
            .embedding
            .as_ref()
            .ok_or_else(|| GraphBitError::memory("Memory has no embedding"))?;

        let candidates = storage.list_by_type(entry.memory_type);

        let mut results: Vec<RetrievalResult> = candidates
            .into_iter()
            .filter(|candidate| candidate.id != *memory_id) // Exclude self
            .filter_map(|candidate| {
                let candidate_embedding = candidate.embedding.as_ref()?;
                let similarity = Self::cosine_similarity(query_embedding, candidate_embedding);

                if similarity >= min_similarity {
                    Some(RetrievalResult::new(candidate.clone(), similarity))
                } else {
                    None
                }
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit results
        results.truncate(limit);

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((MemoryRetriever::cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert!((MemoryRetriever::cosine_similarity(&a, &b) - 0.0).abs() < 0.001);

        let a = vec![1.0, 1.0, 0.0];
        let b = vec![1.0, 1.0, 0.0];
        assert!((MemoryRetriever::cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
    }
}

