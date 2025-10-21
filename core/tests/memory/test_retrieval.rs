//! Tests for memory retrieval

use graphbit_core::memory::retrieval::MemoryRetriever;
use graphbit_core::memory::storage::{InMemoryStorage, MemoryStorage};
use graphbit_core::memory::types::*;

fn create_test_entry(content: &str, memory_type: MemoryType) -> MemoryEntry {
    MemoryEntry::new(content.to_string(), memory_type, None)
}

#[test]
fn test_memory_retriever_creation() {
    let _retriever = MemoryRetriever::new(None);

    // Just verify creation succeeds
}

#[tokio::test]
async fn test_retrieval_basic_query() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store some memories
    let entry1 = create_test_entry("The weather is sunny today", MemoryType::Working);
    let entry2 = create_test_entry("I like programming in Rust", MemoryType::Factual);

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();

    let query = MemoryQuery::new("weather".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert!(!results.is_empty(), "Should find matching memories");
}

#[tokio::test]
async fn test_retrieval_with_limit() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store multiple memories
    for i in 0..10 {
        let entry = create_test_entry(&format!("Memory {}", i), MemoryType::Factual);
        storage.store(entry).unwrap();
    }

    let query = MemoryQuery::new("Memory".to_string()).with_limit(5);
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert!(results.len() <= 5, "Should respect limit");
}

#[tokio::test]
async fn test_retrieval_by_memory_type() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store different types
    storage
        .store(create_test_entry("Working 1", MemoryType::Working))
        .unwrap();
    storage
        .store(create_test_entry("Factual 1", MemoryType::Factual))
        .unwrap();
    storage
        .store(create_test_entry("Working 2", MemoryType::Working))
        .unwrap();

    let query = MemoryQuery::new("".to_string()).with_memory_type(MemoryType::Working);

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert_eq!(results.len(), 2, "Should only return Working memories");
    assert!(results
        .iter()
        .all(|r| r.entry.memory_type == MemoryType::Working));
}

#[tokio::test]
async fn test_retrieval_by_session() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let session_id = "session_123";

    // Store memories with different sessions
    let mut entry1 = create_test_entry("Session memory 1", MemoryType::Working);
    entry1.session_id = Some(session_id.to_string());
    let mut entry2 = create_test_entry("Session memory 2", MemoryType::Working);
    entry2.session_id = Some(session_id.to_string());
    let mut entry3 = create_test_entry("Other session", MemoryType::Working);
    entry3.session_id = Some("other_session".to_string());

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();
    storage.store(entry3).unwrap();

    let query = MemoryQuery::new("".to_string()).with_session_id(session_id.to_string());

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert_eq!(
        results.len(),
        2,
        "Should only return memories from specified session"
    );
}

#[tokio::test]
async fn test_retrieval_by_tags() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store memories with tags
    let mut entry1 = create_test_entry("Tagged memory 1", MemoryType::Factual);
    entry1.metadata.add_tag("important".to_string());
    entry1.metadata.add_tag("user".to_string());

    let mut entry2 = create_test_entry("Tagged memory 2", MemoryType::Factual);
    entry2.metadata.add_tag("important".to_string());

    let entry3 = create_test_entry("Untagged memory", MemoryType::Factual);

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();
    storage.store(entry3).unwrap();

    let query = MemoryQuery::new("".to_string()).with_tags(vec!["important".to_string()]);

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert_eq!(
        results.len(),
        2,
        "Should return memories with specified tag"
    );
}

#[tokio::test]
async fn test_retrieval_empty_storage() {
    let storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let query = MemoryQuery::new("anything".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert!(
        results.is_empty(),
        "Should return empty results for empty storage"
    );
}

#[tokio::test]
async fn test_retrieval_no_matches() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    storage
        .store(create_test_entry("Test memory", MemoryType::Factual))
        .unwrap();

    let query =
        MemoryQuery::new("nonexistent query".to_string()).with_memory_type(MemoryType::Episodic); // Different type

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert!(
        results.is_empty(),
        "Should return empty results when no matches"
    );
}

#[tokio::test]
async fn test_retrieval_keyword_search() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store memories with embeddings
    let mut entry1 = create_test_entry("Memory with embedding 1", MemoryType::Factual);
    entry1.embedding = Some(vec![0.1; 128]);
    let mut entry2 = create_test_entry("Memory with embedding 2", MemoryType::Factual);
    entry2.embedding = Some(vec![0.2; 128]);

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();

    let query = MemoryQuery::new("embedding".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    // Without embedding provider, should still return results based on content
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_retrieval_similarity_scoring() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store memories
    storage
        .store(create_test_entry("Exact match query", MemoryType::Factual))
        .unwrap();
    storage
        .store(create_test_entry(
            "Partial query match",
            MemoryType::Factual,
        ))
        .unwrap();
    storage
        .store(create_test_entry("No match here", MemoryType::Factual))
        .unwrap();

    let query = MemoryQuery::new("query".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    // Results should be ordered by similarity
    if results.len() > 1 {
        assert!(
            results[0].similarity >= results[1].similarity,
            "Results should be ordered by similarity"
        );
    }
}

#[tokio::test]
async fn test_retrieval_min_similarity_filter() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    storage
        .store(create_test_entry("Test memory", MemoryType::Factual))
        .unwrap();

    let query = MemoryQuery::new("test".to_string()).with_min_similarity(0.9); // Very high threshold

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    // All results should meet minimum similarity
    for result in results {
        assert!(result.similarity >= 0.9);
    }
}

#[tokio::test]
async fn test_retrieval_with_related_memories() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store related memories
    let entry1 = create_test_entry("Main memory", MemoryType::Semantic);
    let entry2 = create_test_entry("Related memory", MemoryType::Semantic);
    let id2 = entry2.id.clone();

    let mut entry1_with_relation = entry1;
    entry1_with_relation.add_relation(id2.clone());

    storage.store(entry1_with_relation).unwrap();
    storage.store(entry2).unwrap();

    let query = MemoryQuery::new("Main".to_string());

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    // Should include related memories
    assert!(!results.is_empty());
}

#[tokio::test]
async fn test_retrieval_multiple_filters() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let session_id = "test_session";

    // Store various memories
    let mut entry1 = create_test_entry("Matching memory", MemoryType::Working);
    entry1.metadata.add_tag("important".to_string());
    entry1.session_id = Some(session_id.to_string());

    let mut entry2 = create_test_entry("Non-matching type", MemoryType::Factual);
    entry2.metadata.add_tag("important".to_string());
    entry2.session_id = Some(session_id.to_string());

    let mut entry3 = create_test_entry("Non-matching session", MemoryType::Working);
    entry3.metadata.add_tag("important".to_string());
    entry3.session_id = Some("other_session".to_string());

    storage.store(entry1).unwrap();
    storage.store(entry2).unwrap();
    storage.store(entry3).unwrap();

    let query = MemoryQuery::new("".to_string())
        .with_memory_type(MemoryType::Working)
        .with_session_id(session_id.to_string())
        .with_tags(vec!["important".to_string()]);

    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert_eq!(results.len(), 1, "Should match all filters");
}

#[tokio::test]
async fn test_retrieval_result_structure() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let entry = create_test_entry("Test memory", MemoryType::Factual);
    storage.store(entry).unwrap();

    let query = MemoryQuery::new("Test".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert!(!results.is_empty());

    let result = &results[0];
    assert!(result.similarity >= 0.0 && result.similarity <= 1.0);
    assert_eq!(result.entry.content, "Test memory");
}

#[test]
fn test_retrieval_get_by_id() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let entry = create_test_entry("Test memory", MemoryType::Factual);
    let id = entry.id.clone();
    storage.store(entry).unwrap();

    let retrieved = retriever.get_by_id(&id, &mut storage);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().content, "Test memory");
}

#[test]
fn test_retrieval_get_by_id_nonexistent() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    let fake_id = MemoryId::new();
    let retrieved = retriever.get_by_id(&fake_id, &mut storage);
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_retrieval_empty_query() {
    let mut storage = InMemoryStorage::new();
    let retriever = MemoryRetriever::new(None);

    // Store some memories
    storage
        .store(create_test_entry("Memory 1", MemoryType::Factual))
        .unwrap();
    storage
        .store(create_test_entry("Memory 2", MemoryType::Factual))
        .unwrap();

    // Empty query should return all memories (up to limit)
    let query = MemoryQuery::new("".to_string());
    let results = retriever.retrieve(&query, &storage).await.unwrap();

    assert_eq!(results.len(), 2);
}
