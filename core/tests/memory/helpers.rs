//! Test helpers for memory system tests

use graphbit_core::memory::types::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Create a test memory entry
#[allow(dead_code)]
pub fn create_test_entry(content: &str, memory_type: MemoryType) -> MemoryEntry {
    MemoryEntry::new(content.to_string(), memory_type, None)
}

/// Create a test memory entry with metadata
#[allow(dead_code)]
pub fn create_test_entry_with_metadata(
    content: &str,
    memory_type: MemoryType,
    tags: Vec<String>,
    source: &str,
) -> MemoryEntry {
    let mut entry = MemoryEntry::new(content.to_string(), memory_type, None);
    entry.metadata.tags = tags;
    entry.metadata.source = source.to_string();
    entry
}

/// Create a test memory query
#[allow(dead_code)]
pub fn create_test_query(query: &str) -> MemoryQuery {
    MemoryQuery::new(query.to_string())
}

/// Create a test embedding provider (mock)
#[allow(dead_code)]
pub fn create_test_embedding_provider(
) -> Option<Arc<dyn graphbit_core::embeddings::EmbeddingProviderTrait>> {
    // Return None for tests that don't need actual embeddings
    None
}

/// Run async test
#[allow(dead_code)]
pub fn run_async<F>(future: F) -> F::Output
where
    F: std::future::Future,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

/// Create test embedding vector
#[allow(dead_code)]
pub fn create_test_embedding(size: usize) -> Vec<f32> {
    (0..size).map(|i| (i as f32) / (size as f32)).collect()
}

/// Calculate cosine similarity between two vectors
#[allow(dead_code)]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Create a test session ID
#[allow(dead_code)]
pub fn create_test_session_id() -> String {
    format!("test_session_{}", uuid::Uuid::new_v4())
}

/// Wait for a short duration (for testing time-based features)
#[allow(dead_code)]
pub async fn wait_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}
