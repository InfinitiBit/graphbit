//! In-memory vector index for semantic search over memories.

use tokio::sync::RwLock;

use crate::embeddings::EmbeddingService;
use crate::errors::GraphBitResult;

use super::types::MemoryId;

/// A single entry in the vector index.
#[derive(Debug, Clone)]
struct VectorEntry {
    memory_id: MemoryId,
    embedding: Vec<f32>,
}

/// In-memory vector index backed by brute-force cosine similarity.
///
/// Suitable for moderate memory counts (thousands). For larger datasets a
/// purpose-built ANN index should replace this implementation.
pub struct VectorIndex {
    entries: RwLock<Vec<VectorEntry>>,
}

impl VectorIndex {
    /// Create a new, empty vector index.
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
        }
    }

    /// Insert an embedding for the given memory.
    pub async fn insert(&self, memory_id: MemoryId, embedding: Vec<f32>) {
        let mut entries = self.entries.write().await;
        entries.push(VectorEntry {
            memory_id,
            embedding,
        });
    }

    /// Search for the `top_k` most similar entries to `query_embedding`,
    /// returning `(MemoryId, similarity_score)` pairs above `threshold`.
    pub async fn search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
        threshold: f64,
    ) -> GraphBitResult<Vec<(MemoryId, f64)>> {
        let entries = self.entries.read().await;

        let mut scored: Vec<(MemoryId, f64)> = entries
            .iter()
            .filter_map(|entry| {
                let sim = EmbeddingService::cosine_similarity(query_embedding, &entry.embedding)
                    .ok()?;
                let sim_f64 = f64::from(sim);
                if sim_f64 >= threshold {
                    Some((entry.memory_id.clone(), sim_f64))
                } else {
                    None
                }
            })
            .collect();

        // Sort descending by score.
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);

        Ok(scored)
    }

    /// Remove entries for a specific memory.
    pub async fn remove(&self, memory_id: &MemoryId) {
        let mut entries = self.entries.write().await;
        entries.retain(|e| &e.memory_id != memory_id);
    }

    /// Replace the embedding for an existing memory.
    pub async fn update(&self, memory_id: &MemoryId, embedding: Vec<f32>) {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.iter_mut().find(|e| &e.memory_id == memory_id) {
            entry.embedding = embedding;
        } else {
            entries.push(VectorEntry {
                memory_id: memory_id.clone(),
                embedding,
            });
        }
    }

    /// Remove all entries from the index.
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_index_operations() {
        let index = VectorIndex::new();

        let id1 = MemoryId::new();
        let id2 = MemoryId::new();

        // Insert two vectors
        index.insert(id1.clone(), vec![1.0, 0.0, 0.0]).await;
        index.insert(id2.clone(), vec![0.0, 1.0, 0.0]).await;

        // Search with a vector close to id1
        let results = index
            .search(&[0.9, 0.1, 0.0], 10, 0.0)
            .await
            .expect("search ok");
        assert_eq!(results.len(), 2);
        // The first result should be closer to id1
        assert_eq!(results[0].0, id1);
        assert!(results[0].1 > results[1].1);

        // Remove id1
        index.remove(&id1).await;
        let results = index
            .search(&[1.0, 0.0, 0.0], 10, 0.0)
            .await
            .expect("search ok");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, id2);
    }

    #[tokio::test]
    async fn test_vector_index_threshold() {
        let index = VectorIndex::new();

        let id1 = MemoryId::new();
        index.insert(id1.clone(), vec![1.0, 0.0, 0.0]).await;

        // Orthogonal vector should have ~0 similarity
        let results = index
            .search(&[0.0, 1.0, 0.0], 10, 0.5)
            .await
            .expect("search ok");
        assert!(
            results.is_empty(),
            "Orthogonal vector should be below threshold 0.5"
        );

        // Identical vector should have similarity 1.0
        let results = index
            .search(&[1.0, 0.0, 0.0], 10, 0.99)
            .await
            .expect("search ok");
        assert_eq!(results.len(), 1);
        assert!((results[0].1 - 1.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_vector_index_update() {
        let index = VectorIndex::new();
        let id = MemoryId::new();

        index.insert(id.clone(), vec![1.0, 0.0, 0.0]).await;

        // Update embedding
        index.update(&id, vec![0.0, 1.0, 0.0]).await;

        // Now id should be similar to [0, 1, 0] rather than [1, 0, 0]
        let results = index
            .search(&[0.0, 1.0, 0.0], 10, 0.5)
            .await
            .expect("search ok");
        assert_eq!(results.len(), 1);
        assert!((results[0].1 - 1.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_vector_index_clear() {
        let index = VectorIndex::new();
        index.insert(MemoryId::new(), vec![1.0, 0.0]).await;
        index.insert(MemoryId::new(), vec![0.0, 1.0]).await;

        index.clear().await;

        let results = index
            .search(&[1.0, 0.0], 10, 0.0)
            .await
            .expect("search ok");
        assert!(results.is_empty());
    }
}
