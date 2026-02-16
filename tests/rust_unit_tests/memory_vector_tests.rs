use graphbit_core::memory::{vector::VectorIndex, MemoryId};

#[tokio::test]
async fn test_vector_index_insert_and_search() {
    let index = VectorIndex::new();

    let id1 = MemoryId::new();
    let id2 = MemoryId::new();

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
}

#[tokio::test]
async fn test_vector_index_remove() {
    let index = VectorIndex::new();

    let id1 = MemoryId::new();
    let id2 = MemoryId::new();

    index.insert(id1.clone(), vec![1.0, 0.0, 0.0]).await;
    index.insert(id2.clone(), vec![0.0, 1.0, 0.0]).await;

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
async fn test_vector_index_update_nonexistent_inserts() {
    let index = VectorIndex::new();
    let id = MemoryId::new();

    // Update on a non-existent ID should insert
    index.update(&id, vec![1.0, 0.0, 0.0]).await;

    let results = index
        .search(&[1.0, 0.0, 0.0], 10, 0.5)
        .await
        .expect("search ok");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, id);
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

#[tokio::test]
async fn test_vector_index_top_k_limit() {
    let index = VectorIndex::new();

    // Insert 5 similar vectors
    for i in 0..5 {
        let mut v = vec![0.0; 3];
        v[0] = 1.0 - (i as f32 * 0.1);
        v[1] = i as f32 * 0.1;
        index.insert(MemoryId::new(), v).await;
    }

    // Request top_k=2
    let results = index
        .search(&[1.0, 0.0, 0.0], 2, 0.0)
        .await
        .expect("search ok");
    assert_eq!(results.len(), 2);
    // Results should be sorted descending by score
    assert!(results[0].1 >= results[1].1);
}

#[tokio::test]
async fn test_vector_index_empty_search() {
    let index = VectorIndex::new();

    let results = index
        .search(&[1.0, 0.0, 0.0], 10, 0.0)
        .await
        .expect("search ok");
    assert!(results.is_empty());
}

#[tokio::test]
async fn test_vector_index_remove_nonexistent() {
    let index = VectorIndex::new();
    let id = MemoryId::new();

    // Removing a non-existent ID should not panic
    index.remove(&id).await;

    let results = index
        .search(&[1.0, 0.0, 0.0], 10, 0.0)
        .await
        .expect("search ok");
    assert!(results.is_empty());
}
