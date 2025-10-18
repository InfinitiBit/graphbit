//! Storage abstraction layer for the memory system
//!
//! This module provides storage backends for persisting and retrieving memories,
//! including in-memory storage with LRU caching for performance.

use super::types::{MemoryEntry, MemoryId, MemoryType};
use crate::errors::GraphBitResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for memory storage backends
pub trait MemoryStorage: Send + Sync {
    /// Store a memory entry
    fn store(&mut self, entry: MemoryEntry) -> GraphBitResult<()>;

    /// Retrieve a memory by ID
    fn get(&self, id: &MemoryId) -> Option<&MemoryEntry>;

    /// Retrieve a mutable reference to a memory by ID
    fn get_mut(&mut self, id: &MemoryId) -> Option<&mut MemoryEntry>;

    /// Delete a memory by ID
    fn delete(&mut self, id: &MemoryId) -> GraphBitResult<bool>;

    /// List all memories of a specific type
    fn list_by_type(&self, memory_type: MemoryType) -> Vec<&MemoryEntry>;

    /// List all memories in a session
    fn list_by_session(&self, session_id: &str) -> Vec<&MemoryEntry>;

    /// Get all memories
    fn list_all(&self) -> Vec<&MemoryEntry>;

    /// Count memories by type
    fn count_by_type(&self, memory_type: MemoryType) -> usize;

    /// Get total memory count
    fn count(&self) -> usize;

    /// Clear all memories
    fn clear(&mut self);

    /// Clear memories of a specific type
    fn clear_type(&mut self, memory_type: MemoryType);

    /// Clear memories in a session
    fn clear_session(&mut self, session_id: &str);
}

/// In-memory storage implementation with HashMap
pub struct InMemoryStorage {
    /// Main storage map
    memories: HashMap<MemoryId, MemoryEntry>,
    /// Index by memory type for fast filtering
    type_index: HashMap<MemoryType, Vec<MemoryId>>,
    /// Index by session ID for fast session queries
    session_index: HashMap<String, Vec<MemoryId>>,
    /// Maximum capacity per memory type
    capacity_limits: HashMap<MemoryType, usize>,
}

impl InMemoryStorage {
    /// Create a new in-memory storage with default capacities
    pub fn new() -> Self {
        let mut capacity_limits = HashMap::with_capacity(4);
        for mem_type in MemoryType::all() {
            capacity_limits.insert(mem_type, mem_type.default_capacity());
        }

        Self {
            memories: HashMap::with_capacity(1000),
            type_index: HashMap::with_capacity(4),
            session_index: HashMap::with_capacity(16),
            capacity_limits,
        }
    }

    /// Create a new in-memory storage with custom capacities
    pub fn with_capacities(capacities: HashMap<MemoryType, usize>) -> Self {
        let total_capacity: usize = capacities.values().sum();
        
        Self {
            memories: HashMap::with_capacity(total_capacity),
            type_index: HashMap::with_capacity(4),
            session_index: HashMap::with_capacity(16),
            capacity_limits: capacities,
        }
    }

    /// Set capacity limit for a memory type
    pub fn set_capacity(&mut self, memory_type: MemoryType, capacity: usize) {
        self.capacity_limits.insert(memory_type, capacity);
    }

    /// Get capacity limit for a memory type
    pub fn get_capacity(&self, memory_type: MemoryType) -> usize {
        self.capacity_limits
            .get(&memory_type)
            .copied()
            .unwrap_or(memory_type.default_capacity())
    }

    /// Check if adding a memory would exceed capacity
    fn would_exceed_capacity(&self, memory_type: MemoryType) -> bool {
        let current_count = self.count_by_type(memory_type);
        let limit = self.get_capacity(memory_type);
        current_count >= limit
    }

    /// Evict least important memory of a given type
    fn evict_least_important(&mut self, memory_type: MemoryType) -> GraphBitResult<()> {
        let type_memories = self.list_by_type(memory_type);
        
        if type_memories.is_empty() {
            return Ok(());
        }

        // Find memory with lowest decay score (least important)
        let now = chrono::Utc::now();
        let to_evict = type_memories
            .iter()
            .min_by(|a, b| {
                let a_score = a.calculate_decay(now);
                let b_score = b.calculate_decay(now);
                a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|entry| entry.id.clone());

        if let Some(id) = to_evict {
            self.delete(&id)?;
        }

        Ok(())
    }

    /// Update indices when storing a memory
    fn update_indices(&mut self, entry: &MemoryEntry) {
        // Update type index
        self.type_index
            .entry(entry.memory_type)
            .or_insert_with(|| Vec::with_capacity(entry.memory_type.default_capacity()))
            .push(entry.id.clone());

        // Update session index if applicable
        if let Some(ref session_id) = entry.session_id {
            self.session_index
                .entry(session_id.clone())
                .or_insert_with(|| Vec::with_capacity(16))
                .push(entry.id.clone());
        }
    }

    /// Remove from indices when deleting a memory
    fn remove_from_indices(&mut self, entry: &MemoryEntry) {
        // Remove from type index
        if let Some(ids) = self.type_index.get_mut(&entry.memory_type) {
            ids.retain(|id| id != &entry.id);
        }

        // Remove from session index
        if let Some(ref session_id) = entry.session_id {
            if let Some(ids) = self.session_index.get_mut(session_id) {
                ids.retain(|id| id != &entry.id);
            }
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStorage for InMemoryStorage {
    fn store(&mut self, entry: MemoryEntry) -> GraphBitResult<()> {
        // Check capacity and evict if necessary
        if self.would_exceed_capacity(entry.memory_type) {
            self.evict_least_important(entry.memory_type)?;
        }

        // Update indices
        self.update_indices(&entry);

        // Store the entry
        self.memories.insert(entry.id.clone(), entry);

        Ok(())
    }

    fn get(&self, id: &MemoryId) -> Option<&MemoryEntry> {
        self.memories.get(id)
    }

    fn get_mut(&mut self, id: &MemoryId) -> Option<&mut MemoryEntry> {
        self.memories.get_mut(id)
    }

    fn delete(&mut self, id: &MemoryId) -> GraphBitResult<bool> {
        if let Some(entry) = self.memories.remove(id) {
            self.remove_from_indices(&entry);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn list_by_type(&self, memory_type: MemoryType) -> Vec<&MemoryEntry> {
        self.type_index
            .get(&memory_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.memories.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn list_by_session(&self, session_id: &str) -> Vec<&MemoryEntry> {
        self.session_index
            .get(session_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.memories.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn list_all(&self) -> Vec<&MemoryEntry> {
        self.memories.values().collect()
    }

    fn count_by_type(&self, memory_type: MemoryType) -> usize {
        self.type_index
            .get(&memory_type)
            .map(|ids| ids.len())
            .unwrap_or(0)
    }

    fn count(&self) -> usize {
        self.memories.len()
    }

    fn clear(&mut self) {
        self.memories.clear();
        self.type_index.clear();
        self.session_index.clear();
    }

    fn clear_type(&mut self, memory_type: MemoryType) {
        if let Some(ids) = self.type_index.remove(&memory_type) {
            for id in ids {
                if let Some(entry) = self.memories.remove(&id) {
                    // Also remove from session index
                    if let Some(ref session_id) = entry.session_id {
                        if let Some(session_ids) = self.session_index.get_mut(session_id) {
                            session_ids.retain(|sid| sid != &id);
                        }
                    }
                }
            }
        }
    }

    fn clear_session(&mut self, session_id: &str) {
        if let Some(ids) = self.session_index.remove(session_id) {
            for id in ids {
                if let Some(entry) = self.memories.remove(&id) {
                    // Also remove from type index
                    if let Some(type_ids) = self.type_index.get_mut(&entry.memory_type) {
                        type_ids.retain(|tid| tid != &id);
                    }
                }
            }
        }
    }
}

/// Thread-safe wrapper for memory storage
pub type SharedStorage = Arc<RwLock<Box<dyn MemoryStorage>>>;

/// Create a new shared in-memory storage
pub fn create_shared_storage() -> SharedStorage {
    Arc::new(RwLock::new(Box::new(InMemoryStorage::new())))
}

/// Create a new shared in-memory storage with custom capacities
pub fn create_shared_storage_with_capacities(
    capacities: HashMap<MemoryType, usize>,
) -> SharedStorage {
    Arc::new(RwLock::new(Box::new(InMemoryStorage::with_capacities(
        capacities,
    ))))
}

