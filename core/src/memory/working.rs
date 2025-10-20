//! Working memory implementation for short-term session-based context
//!
//! Working memory stores temporary information for the current conversation session,
//! automatically clearing when sessions end.

use super::storage::MemoryStorage;
use super::types::{MemoryEntry, MemoryId, MemoryMetadata, MemoryType};
use crate::errors::GraphBitResult;
use std::collections::HashMap;

/// Working memory manager for session-based short-term storage
#[derive(Debug)]
pub struct WorkingMemory {
    /// Current session ID
    current_session: Option<String>,
    /// Session metadata
    session_metadata: HashMap<String, serde_json::Value>,
}

impl WorkingMemory {
    /// Create a new working memory instance
    pub fn new() -> Self {
        Self {
            current_session: None,
            session_metadata: HashMap::with_capacity(8),
        }
    }

    /// Start a new session
    pub fn start_session(&mut self, session_id: String) {
        self.current_session = Some(session_id);
        self.session_metadata.clear();
    }

    /// End the current session and clear its memories
    pub fn end_session(&mut self, storage: &mut dyn MemoryStorage) -> GraphBitResult<usize> {
        if let Some(ref session_id) = self.current_session {
            let memories = storage.list_by_session(session_id);
            let count = memories.len();
            storage.clear_session(session_id);
            self.current_session = None;
            self.session_metadata.clear();
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Get current session ID
    pub fn get_session_id(&self) -> Option<&String> {
        self.current_session.as_ref()
    }

    /// Store a working memory entry
    pub fn store(
        &self,
        content: String,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let session_id = self.current_session.clone();
        let entry = MemoryEntry::new(content, MemoryType::Working, session_id);
        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Store a working memory with custom metadata
    pub fn store_with_metadata(
        &self,
        content: String,
        metadata: MemoryMetadata,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let session_id = self.current_session.clone();
        let mut entry = MemoryEntry::new(content, MemoryType::Working, session_id);
        entry.metadata = metadata;
        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Get all working memories for current session
    pub fn get_session_memories(&self, storage: &dyn MemoryStorage) -> Vec<MemoryEntry> {
        if let Some(ref session_id) = self.current_session {
            storage
                .list_by_session(session_id)
                .into_iter()
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get session context as a formatted string
    pub fn get_session_context(&self, storage: &dyn MemoryStorage) -> String {
        let memories = self.get_session_memories(storage);

        if memories.is_empty() {
            return String::from("No working memory available.");
        }

        let mut context = String::with_capacity(memories.len() * 100);
        context.push_str("Working Memory Context:\n");

        for (i, memory) in memories.iter().enumerate() {
            context.push_str(&format!("{}. {}\n", i + 1, memory.content));
        }

        context
    }

    /// Set session metadata
    pub fn set_session_metadata(&mut self, key: String, value: serde_json::Value) {
        self.session_metadata.insert(key, value);
    }

    /// Get session metadata
    pub fn get_session_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.session_metadata.get(key)
    }

    /// Get all session metadata
    pub fn get_all_session_metadata(&self) -> HashMap<String, serde_json::Value> {
        self.session_metadata.clone()
    }

    /// Clear all session metadata
    pub fn clear_session_metadata(&mut self) {
        self.session_metadata.clear();
    }

    /// Get count of working memories in current session
    pub fn count_session_memories(&self, storage: &dyn MemoryStorage) -> usize {
        if let Some(ref session_id) = self.current_session {
            storage.list_by_session(session_id).len()
        } else {
            0
        }
    }

    /// Clear all working memories in current session
    pub fn clear_session_memories(&mut self, storage: &mut dyn MemoryStorage) -> GraphBitResult<usize> {
        if let Some(ref session_id) = self.current_session {
            let memories = storage.list_by_session(session_id);
            let count = memories.len();
            // Collect IDs first to avoid borrow checker issues
            let ids: Vec<_> = memories.iter().map(|m| m.id.clone()).collect();
            for id in ids {
                storage.delete(&id)?;
            }
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Check if session is active
    pub fn is_session_active(&self) -> bool {
        self.current_session.is_some()
    }
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::storage::InMemoryStorage;

    #[test]
    fn test_working_memory_session_lifecycle() {
        let mut working = WorkingMemory::new();
        let mut storage = InMemoryStorage::new();

        assert!(!working.is_session_active());

        working.start_session("session1".to_string());
        assert!(working.is_session_active());
        assert_eq!(working.get_session_id(), Some(&"session1".to_string()));

        let id = working
            .store("Test memory".to_string(), &mut storage)
            .unwrap();
        assert_eq!(working.count_session_memories(&storage), 1);

        let ended = working.end_session(&mut storage).unwrap();
        assert_eq!(ended, 1);
        assert!(!working.is_session_active());
        assert_eq!(storage.count(), 0);
    }

    #[test]
    fn test_working_memory_context() {
        let mut working = WorkingMemory::new();
        let mut storage = InMemoryStorage::new();

        working.start_session("session1".to_string());
        working
            .store("First memory".to_string(), &mut storage)
            .unwrap();
        working
            .store("Second memory".to_string(), &mut storage)
            .unwrap();

        let context = working.get_session_context(&storage);
        assert!(context.contains("First memory"));
        assert!(context.contains("Second memory"));
    }

    #[test]
    fn test_session_metadata() {
        let mut working = WorkingMemory::new();

        working.set_session_metadata("user_id".to_string(), serde_json::json!("user123"));
        working.set_session_metadata("language".to_string(), serde_json::json!("en"));

        assert_eq!(
            working.get_session_metadata("user_id"),
            Some(&serde_json::json!("user123"))
        );
        assert_eq!(
            working.get_session_metadata("language"),
            Some(&serde_json::json!("en"))
        );

        working.clear_session_metadata();
        assert_eq!(working.get_session_metadata("user_id"), None);
    }
}
