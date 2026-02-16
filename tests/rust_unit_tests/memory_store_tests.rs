use std::collections::HashMap;

use graphbit_core::memory::{
    store::MetadataStore, Memory, MemoryAction, MemoryHistory, MemoryId, MemoryScope,
};

fn make_memory(content: &str, user_id: Option<&str>) -> Memory {
    Memory {
        id: MemoryId::new(),
        content: content.to_string(),
        scope: MemoryScope {
            user_id: user_id.map(String::from),
            agent_id: None,
            run_id: None,
        },
        metadata: HashMap::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        hash: format!("hash_{content}"),
    }
}

#[tokio::test]
async fn test_metadata_store_insert_and_get() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("User lives in Munich", Some("user1"));
    let id = memory.id.clone();

    store
        .insert_memory(&memory)
        .await
        .expect("insert should succeed");

    let fetched = store
        .get_memory(&id)
        .await
        .expect("get should succeed")
        .expect("memory should exist");
    assert_eq!(fetched.content, "User lives in Munich");
    assert_eq!(fetched.scope.user_id.as_deref(), Some("user1"));
}

#[tokio::test]
async fn test_metadata_store_update() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("User lives in Munich", Some("user1"));
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    store
        .update_memory(&id, "User lives in Berlin", "def456")
        .await
        .expect("update should succeed");

    let updated = store
        .get_memory(&id)
        .await
        .expect("get ok")
        .expect("should exist");
    assert_eq!(updated.content, "User lives in Berlin");
    assert_eq!(updated.hash, "def456");
}

#[tokio::test]
async fn test_metadata_store_update_nonexistent() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let result = store
        .update_memory(&MemoryId::new(), "content", "hash")
        .await;
    assert!(result.is_err(), "Updating non-existent memory should fail");
}

#[tokio::test]
async fn test_metadata_store_delete() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("To be deleted", None);
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    store.delete_memory(&id).await.expect("delete ok");

    let gone = store.get_memory(&id).await.expect("get ok");
    assert!(gone.is_none());
}

#[tokio::test]
async fn test_metadata_store_get_nonexistent() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let result = store
        .get_memory(&MemoryId::new())
        .await
        .expect("get should not error");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_metadata_store_scope_filtering() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    for (user, content) in &[("alice", "Fact A"), ("bob", "Fact B"), ("alice", "Fact C")] {
        let memory = make_memory(content, Some(user));
        store.insert_memory(&memory).await.expect("insert ok");
    }

    // Filter by Alice
    let alice_scope = MemoryScope {
        user_id: Some("alice".to_string()),
        ..Default::default()
    };
    let alice_memories = store
        .get_all_memories(&alice_scope)
        .await
        .expect("get_all ok");
    assert_eq!(alice_memories.len(), 2);

    // Filter by Bob
    let bob_scope = MemoryScope {
        user_id: Some("bob".to_string()),
        ..Default::default()
    };
    let bob_memories = store
        .get_all_memories(&bob_scope)
        .await
        .expect("get_all ok");
    assert_eq!(bob_memories.len(), 1);

    // No filter (all)
    let all = store
        .get_all_memories(&MemoryScope::default())
        .await
        .expect("get_all ok");
    assert_eq!(all.len(), 3);
}

#[tokio::test]
async fn test_metadata_store_delete_all_by_scope() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    for (user, content) in &[("alice", "Fact A"), ("bob", "Fact B"), ("alice", "Fact C")] {
        let memory = make_memory(content, Some(user));
        store.insert_memory(&memory).await.expect("insert ok");
    }

    let alice_scope = MemoryScope {
        user_id: Some("alice".to_string()),
        ..Default::default()
    };
    store
        .delete_all_memories(&alice_scope)
        .await
        .expect("delete_all ok");

    let remaining = store
        .get_all_memories(&MemoryScope::default())
        .await
        .expect("get_all ok");
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].scope.user_id.as_deref(), Some("bob"));
}

#[tokio::test]
async fn test_metadata_store_history() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("Initial", None);
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    store
        .insert_history(&MemoryHistory {
            memory_id: id.clone(),
            old_content: String::new(),
            new_content: "First version".to_string(),
            action: MemoryAction::Add,
            timestamp: chrono::Utc::now(),
        })
        .await
        .expect("insert_history ok");

    store
        .insert_history(&MemoryHistory {
            memory_id: id.clone(),
            old_content: "First version".to_string(),
            new_content: "Second version".to_string(),
            action: MemoryAction::Update,
            timestamp: chrono::Utc::now(),
        })
        .await
        .expect("insert_history ok");

    let history = store.get_history(&id).await.expect("get_history ok");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].action, MemoryAction::Add);
    assert_eq!(history[1].action, MemoryAction::Update);
    assert_eq!(history[0].new_content, "First version");
    assert_eq!(history[1].old_content, "First version");
    assert_eq!(history[1].new_content, "Second version");
}

#[tokio::test]
async fn test_metadata_store_history_empty() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("No history", None);
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    let history = store.get_history(&id).await.expect("get_history ok");
    assert!(history.is_empty());
}

#[tokio::test]
async fn test_metadata_store_delete_cascades_history() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let memory = make_memory("Will be deleted", None);
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    store
        .insert_history(&MemoryHistory {
            memory_id: id.clone(),
            old_content: String::new(),
            new_content: "First version".to_string(),
            action: MemoryAction::Add,
            timestamp: chrono::Utc::now(),
        })
        .await
        .expect("insert_history ok");

    // Delete the memory — history should cascade
    store.delete_memory(&id).await.expect("delete ok");

    // History should be gone due to ON DELETE CASCADE
    let history = store.get_history(&id).await.expect("get_history ok");
    assert!(history.is_empty());
}

#[tokio::test]
async fn test_metadata_store_metadata_roundtrip() {
    let store = MetadataStore::new(":memory:").expect("should create in-memory store");

    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), serde_json::json!("conversation"));
    metadata.insert("confidence".to_string(), serde_json::json!(0.95));

    let memory = Memory {
        id: MemoryId::new(),
        content: "With metadata".to_string(),
        scope: MemoryScope::default(),
        metadata,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        hash: "h".to_string(),
    };
    let id = memory.id.clone();
    store.insert_memory(&memory).await.expect("insert ok");

    let fetched = store
        .get_memory(&id)
        .await
        .expect("get ok")
        .expect("should exist");
    assert_eq!(
        fetched.metadata.get("source"),
        Some(&serde_json::json!("conversation"))
    );
    assert_eq!(
        fetched.metadata.get("confidence"),
        Some(&serde_json::json!(0.95))
    );
}
