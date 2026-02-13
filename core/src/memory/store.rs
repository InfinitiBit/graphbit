//! SQLite-backed metadata store for persistent memory storage.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::errors::{GraphBitError, GraphBitResult};

use super::types::{Memory, MemoryAction, MemoryHistory, MemoryId, MemoryScope};

/// Persistent metadata store backed by SQLite.
///
/// The connection is wrapped in `Arc<Mutex>` so that it can be shared across
/// async tasks and moved into `spawn_blocking` closures.
pub struct MetadataStore {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl MetadataStore {
    /// Open (or create) the database at `db_path`.
    /// Pass `":memory:"` for an in-memory database.
    pub fn new(db_path: &str) -> GraphBitResult<Self> {
        let conn = rusqlite::Connection::open(db_path)?;
        // Enable foreign key enforcement (must be set per-connection).
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_schema_sync()?;
        Ok(store)
    }

    /// Create the required tables if they do not already exist.
    fn init_schema_sync(&self) -> GraphBitResult<()> {
        let conn = self
            .conn
            .try_lock()
            .map_err(|_| GraphBitError::memory("Failed to acquire database lock during init"))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memories (
                id          TEXT PRIMARY KEY,
                content     TEXT NOT NULL,
                user_id     TEXT,
                agent_id    TEXT,
                run_id      TEXT,
                hash        TEXT NOT NULL,
                metadata    TEXT NOT NULL DEFAULT '{}',
                created_at  TEXT NOT NULL,
                updated_at  TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS memory_history (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                memory_id   TEXT NOT NULL,
                old_content TEXT NOT NULL DEFAULT '',
                new_content TEXT NOT NULL DEFAULT '',
                action      TEXT NOT NULL,
                timestamp   TEXT NOT NULL,
                FOREIGN KEY (memory_id) REFERENCES memories(id) ON DELETE CASCADE
            );
            CREATE INDEX IF NOT EXISTS idx_memories_user_id  ON memories(user_id);
            CREATE INDEX IF NOT EXISTS idx_memories_agent_id ON memories(agent_id);
            CREATE INDEX IF NOT EXISTS idx_memories_run_id   ON memories(run_id);
            CREATE INDEX IF NOT EXISTS idx_memories_hash     ON memories(hash);
            CREATE INDEX IF NOT EXISTS idx_history_memory_id ON memory_history(memory_id);",
        )?;

        Ok(())
    }

    /// Insert a new memory.
    pub async fn insert_memory(&self, memory: &Memory) -> GraphBitResult<()> {
        let id = memory.id.to_string();
        let content = memory.content.clone();
        let user_id = memory.scope.user_id.clone();
        let agent_id = memory.scope.agent_id.clone();
        let run_id = memory.scope.run_id.clone();
        let hash = memory.hash.clone();
        let metadata = serde_json::to_string(&memory.metadata)?;
        let created_at = memory.created_at.to_rfc3339();
        let updated_at = memory.updated_at.to_rfc3339();

        let conn_arc = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || -> GraphBitResult<()> {
            let conn = conn_arc.blocking_lock();
            conn.execute(
                "INSERT INTO memories (id, content, user_id, agent_id, run_id, hash, metadata, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![id, content, user_id, agent_id, run_id, hash, metadata, created_at, updated_at],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Get a single memory by ID.
    pub async fn get_memory(&self, memory_id: &MemoryId) -> GraphBitResult<Option<Memory>> {
        let id = memory_id.to_string();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<Option<Memory>> {
            let conn = conn_arc.blocking_lock();
            let mut stmt =
                conn.prepare("SELECT id, content, user_id, agent_id, run_id, hash, metadata, created_at, updated_at FROM memories WHERE id = ?1")?;
            let mut rows = stmt.query(rusqlite::params![id])?;
            if let Some(row) = rows.next()? {
                Ok(Some(row_to_memory(row)?))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Get all memories matching the given scope.
    pub async fn get_all_memories(&self, scope: &MemoryScope) -> GraphBitResult<Vec<Memory>> {
        let user_id = scope.user_id.clone();
        let agent_id = scope.agent_id.clone();
        let run_id = scope.run_id.clone();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<Vec<Memory>> {
            let conn = conn_arc.blocking_lock();
            let (where_clause, params) = build_scope_filter(&user_id, &agent_id, &run_id);
            let sql = format!(
                "SELECT id, content, user_id, agent_id, run_id, hash, metadata, created_at, updated_at FROM memories{}",
                where_clause
            );
            let mut stmt = conn.prepare(&sql)?;
            let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::types::ToSql).collect();
            let mut rows = stmt.query(param_refs.as_slice())?;
            let mut memories = Vec::new();
            while let Some(row) = rows.next()? {
                memories.push(row_to_memory(row)?);
            }
            Ok(memories)
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Update content and metadata of an existing memory.
    pub async fn update_memory(
        &self,
        memory_id: &MemoryId,
        content: &str,
        hash: &str,
    ) -> GraphBitResult<()> {
        let id = memory_id.to_string();
        let content = content.to_string();
        let hash = hash.to_string();
        let updated_at = Utc::now().to_rfc3339();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<()> {
            let conn = conn_arc.blocking_lock();
            let changed = conn.execute(
                "UPDATE memories SET content = ?1, hash = ?2, updated_at = ?3 WHERE id = ?4",
                rusqlite::params![content, hash, updated_at, id],
            )?;
            if changed == 0 {
                return Err(GraphBitError::memory(format!(
                    "Memory not found: {id}"
                )));
            }
            Ok(())
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Delete a single memory by ID.
    pub async fn delete_memory(&self, memory_id: &MemoryId) -> GraphBitResult<()> {
        let id = memory_id.to_string();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<()> {
            let conn = conn_arc.blocking_lock();
            conn.execute("DELETE FROM memories WHERE id = ?1", rusqlite::params![id])?;
            Ok(())
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Delete all memories matching the given scope.
    pub async fn delete_all_memories(&self, scope: &MemoryScope) -> GraphBitResult<()> {
        let user_id = scope.user_id.clone();
        let agent_id = scope.agent_id.clone();
        let run_id = scope.run_id.clone();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<()> {
            let conn = conn_arc.blocking_lock();
            let (where_clause, params) = build_scope_filter(&user_id, &agent_id, &run_id);
            let sql = format!("DELETE FROM memories{where_clause}");
            let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::types::ToSql).collect();
            conn.execute(&sql, param_refs.as_slice())?;
            Ok(())
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Record a history entry for a memory mutation.
    pub async fn insert_history(&self, history: &MemoryHistory) -> GraphBitResult<()> {
        let memory_id = history.memory_id.to_string();
        let old_content = history.old_content.clone();
        let new_content = history.new_content.clone();
        let action = history.action.to_string();
        let timestamp = history.timestamp.to_rfc3339();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<()> {
            let conn = conn_arc.blocking_lock();
            conn.execute(
                "INSERT INTO memory_history (memory_id, old_content, new_content, action, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![memory_id, old_content, new_content, action, timestamp],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }

    /// Get the full history for a specific memory.
    pub async fn get_history(&self, memory_id: &MemoryId) -> GraphBitResult<Vec<MemoryHistory>> {
        let id = memory_id.to_string();
        let conn_arc = Arc::clone(&self.conn);

        tokio::task::spawn_blocking(move || -> GraphBitResult<Vec<MemoryHistory>> {
            let conn = conn_arc.blocking_lock();
            let mut stmt = conn.prepare(
                "SELECT memory_id, old_content, new_content, action, timestamp
                 FROM memory_history WHERE memory_id = ?1 ORDER BY timestamp ASC",
            )?;
            let mut rows = stmt.query(rusqlite::params![id])?;
            let mut entries = Vec::new();
            while let Some(row) = rows.next()? {
                entries.push(row_to_history(row)?);
            }
            Ok(entries)
        })
        .await
        .map_err(|e| GraphBitError::memory(format!("Join error: {e}")))?
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn row_to_memory(row: &rusqlite::Row<'_>) -> GraphBitResult<Memory> {
    let id_str: String = row.get(0)?;
    let content: String = row.get(1)?;
    let user_id: Option<String> = row.get(2)?;
    let agent_id: Option<String> = row.get(3)?;
    let run_id: Option<String> = row.get(4)?;
    let hash: String = row.get(5)?;
    let metadata_json: String = row.get(6)?;
    let created_at_str: String = row.get(7)?;
    let updated_at_str: String = row.get(8)?;

    let id = MemoryId(Uuid::parse_str(&id_str).map_err(|e| {
        GraphBitError::memory(format!("Invalid UUID in database: {e}"))
    })?);

    let metadata: HashMap<String, serde_json::Value> =
        serde_json::from_str(&metadata_json).unwrap_or_default();

    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    Ok(Memory {
        id,
        content,
        scope: MemoryScope {
            user_id,
            agent_id,
            run_id,
        },
        metadata,
        created_at,
        updated_at,
        hash,
    })
}

fn row_to_history(row: &rusqlite::Row<'_>) -> GraphBitResult<MemoryHistory> {
    let memory_id_str: String = row.get(0)?;
    let old_content: String = row.get(1)?;
    let new_content: String = row.get(2)?;
    let action_str: String = row.get(3)?;
    let timestamp_str: String = row.get(4)?;

    let memory_id = MemoryId(Uuid::parse_str(&memory_id_str).map_err(|e| {
        GraphBitError::memory(format!("Invalid UUID in history: {e}"))
    })?);

    let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    Ok(MemoryHistory {
        memory_id,
        old_content,
        new_content,
        action: MemoryAction::from_str_lossy(&action_str),
        timestamp,
    })
}

/// Build a SQL WHERE clause + params from optional scope fields.
fn build_scope_filter(
    user_id: &Option<String>,
    agent_id: &Option<String>,
    run_id: &Option<String>,
) -> (String, Vec<String>) {
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    if let Some(uid) = user_id {
        params.push(uid.clone());
        conditions.push(format!("user_id = ?{}", params.len()));
    }
    if let Some(aid) = agent_id {
        params.push(aid.clone());
        conditions.push(format!("agent_id = ?{}", params.len()));
    }
    if let Some(rid) = run_id {
        params.push(rid.clone());
        conditions.push(format!("run_id = ?{}", params.len()));
    }

    if conditions.is_empty() {
        (String::new(), params)
    } else {
        (format!(" WHERE {}", conditions.join(" AND ")), params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metadata_store_crud() {
        let store = MetadataStore::new(":memory:").expect("should create in-memory store");

        // Insert
        let id = MemoryId::new();
        let memory = Memory {
            id: id.clone(),
            content: "User lives in Munich".to_string(),
            scope: MemoryScope {
                user_id: Some("user1".to_string()),
                agent_id: None,
                run_id: None,
            },
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            hash: "abc123".to_string(),
        };
        store
            .insert_memory(&memory)
            .await
            .expect("insert should succeed");

        // Get
        let fetched = store
            .get_memory(&id)
            .await
            .expect("get should succeed")
            .expect("memory should exist");
        assert_eq!(fetched.content, "User lives in Munich");
        assert_eq!(fetched.scope.user_id.as_deref(), Some("user1"));

        // Update
        store
            .update_memory(&id, "User lives in Berlin", "def456")
            .await
            .expect("update should succeed");
        let updated = store
            .get_memory(&id)
            .await
            .expect("get should succeed")
            .expect("memory should exist");
        assert_eq!(updated.content, "User lives in Berlin");

        // Delete
        store
            .delete_memory(&id)
            .await
            .expect("delete should succeed");
        let gone = store
            .get_memory(&id)
            .await
            .expect("get should succeed");
        assert!(gone.is_none());
    }

    #[tokio::test]
    async fn test_metadata_store_scope_filtering() {
        let store = MetadataStore::new(":memory:").expect("should create in-memory store");

        // Insert memories for different users
        for (user, content) in &[("alice", "Fact A"), ("bob", "Fact B"), ("alice", "Fact C")] {
            let memory = Memory {
                id: MemoryId::new(),
                content: content.to_string(),
                scope: MemoryScope {
                    user_id: Some(user.to_string()),
                    agent_id: None,
                    run_id: None,
                },
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                hash: format!("hash_{content}"),
            };
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

        // Delete all for Alice
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
        let id = MemoryId::new();

        // Insert a parent memory so the FK constraint is satisfied.
        let memory = Memory {
            id: id.clone(),
            content: "Initial".to_string(),
            scope: MemoryScope::default(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            hash: "h".to_string(),
        };
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
    }
}
