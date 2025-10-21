//! Factual memory implementation for long-term structured knowledge
//!
//! Factual memory stores persistent facts, preferences, and settings that
//! should be retained across sessions.

use super::storage::MemoryStorage;
use super::types::{MemoryEntry, MemoryId, MemoryMetadata, MemoryType};
use crate::errors::GraphBitResult;

/// Factual memory manager for long-term structured knowledge
#[derive(Debug)]
pub struct FactualMemory {
    /// Namespace for organizing facts (e.g., "user_preferences", "system_config")
    namespace: Option<String>,
}

impl FactualMemory {
    /// Create a new factual memory instance
    pub fn new() -> Self {
        Self { namespace: None }
    }

    /// Create a factual memory with a namespace
    pub fn with_namespace(namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
        }
    }

    /// Set the current namespace
    pub fn set_namespace(&mut self, namespace: String) {
        self.namespace = Some(namespace);
    }

    /// Clear the namespace
    pub fn clear_namespace(&mut self) {
        self.namespace = None;
    }

    /// Store a fact
    pub fn store_fact(
        &self,
        key: String,
        value: String,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let content = format!("{}: {}", key, value);
        let mut metadata = MemoryMetadata::new();
        metadata.set_source("factual".to_string());
        metadata.add_tag("fact".to_string());
        metadata.add_tag(key.clone());

        if let Some(ref ns) = self.namespace {
            metadata.add_tag(format!("namespace:{}", ns));
        }

        let mut entry = MemoryEntry::with_importance(
            content,
            MemoryType::Factual,
            0.8,  // Facts are important by default
            None, // No session ID for factual memories
        );
        entry.metadata = metadata;

        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Store a fact with custom importance
    pub fn store_fact_with_importance(
        &self,
        key: String,
        value: String,
        importance: f32,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let content = format!("{}: {}", key, value);
        let mut metadata = MemoryMetadata::new();
        metadata.set_source("factual".to_string());
        metadata.add_tag("fact".to_string());
        metadata.add_tag(key.clone());

        if let Some(ref ns) = self.namespace {
            metadata.add_tag(format!("namespace:{}", ns));
        }

        let mut entry =
            MemoryEntry::with_importance(content, MemoryType::Factual, importance, None);
        entry.metadata = metadata;

        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Retrieve a fact by key
    pub fn get_fact(&self, key: &str, storage: &dyn MemoryStorage) -> Option<String> {
        let facts = storage.list_by_type(MemoryType::Factual);

        for fact in facts {
            // Check if this fact matches the key
            if fact.metadata.tags.contains(&key.to_string()) {
                // Check namespace if set
                if let Some(ref ns) = self.namespace {
                    let ns_tag = format!("namespace:{}", ns);
                    if !fact.metadata.tags.contains(&ns_tag) {
                        continue;
                    }
                }

                // Extract value from "key: value" format
                if let Some(colon_pos) = fact.content.find(':') {
                    let value = fact.content[colon_pos + 1..].trim();
                    return Some(value.to_string());
                }
            }
        }

        None
    }

    /// Update an existing fact
    pub fn update_fact(
        &self,
        key: &str,
        new_value: String,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<bool> {
        let facts = storage.list_by_type(MemoryType::Factual);

        // Find the fact ID first (to avoid borrowing issues)
        let mut fact_id_to_update: Option<MemoryId> = None;
        for fact in facts {
            if fact.metadata.tags.contains(&key.to_string()) {
                // Check namespace if set
                if let Some(ref ns) = self.namespace {
                    let ns_tag = format!("namespace:{}", ns);
                    if !fact.metadata.tags.contains(&ns_tag) {
                        continue;
                    }
                }

                fact_id_to_update = Some(fact.id.clone());
                break;
            }
        }

        // Now update the fact
        if let Some(fact_id) = fact_id_to_update {
            if let Some(entry) = storage.get_mut(&fact_id) {
                entry.content = format!("{}: {}", key, new_value);
                entry.record_access();
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Delete a fact by key
    pub fn delete_fact(&self, key: &str, storage: &mut dyn MemoryStorage) -> GraphBitResult<bool> {
        let facts = storage.list_by_type(MemoryType::Factual);

        for fact in facts {
            if fact.metadata.tags.contains(&key.to_string()) {
                // Check namespace if set
                if let Some(ref ns) = self.namespace {
                    let ns_tag = format!("namespace:{}", ns);
                    if !fact.metadata.tags.contains(&ns_tag) {
                        continue;
                    }
                }

                let fact_id = fact.id.clone();
                return storage.delete(&fact_id);
            }
        }

        Ok(false)
    }

    /// List all facts in current namespace
    pub fn list_facts(&self, storage: &dyn MemoryStorage) -> Vec<(String, String)> {
        let facts = storage.list_by_type(MemoryType::Factual);
        let mut result = Vec::with_capacity(facts.len());

        for fact in facts {
            // Check namespace if set
            if let Some(ref ns) = self.namespace {
                let ns_tag = format!("namespace:{}", ns);
                if !fact.metadata.tags.contains(&ns_tag) {
                    continue;
                }
            }

            // Parse "key: value" format
            if let Some(colon_pos) = fact.content.find(':') {
                let key = fact.content[..colon_pos].trim().to_string();
                let value = fact.content[colon_pos + 1..].trim().to_string();
                result.push((key, value));
            }
        }

        result
    }

    /// Get all facts as a HashMap
    pub fn get_all_facts(
        &self,
        storage: &dyn MemoryStorage,
    ) -> std::collections::HashMap<String, String> {
        let facts = self.list_facts(storage);
        facts.into_iter().collect()
    }

    /// Check if a fact exists
    pub fn has_fact(&self, key: &str, storage: &dyn MemoryStorage) -> bool {
        self.get_fact(key, storage).is_some()
    }

    /// Count facts in current namespace
    pub fn count_facts(&self, storage: &dyn MemoryStorage) -> usize {
        self.list_facts(storage).len()
    }

    /// Store a user preference
    pub fn store_preference(
        &self,
        key: String,
        value: String,
        storage: &mut dyn MemoryStorage,
    ) -> GraphBitResult<MemoryId> {
        let mut metadata = MemoryMetadata::new();
        metadata.set_source("user_preference".to_string());
        metadata.add_tag("preference".to_string());
        metadata.add_tag(key.clone());

        if let Some(ref ns) = self.namespace {
            metadata.add_tag(format!("namespace:{}", ns));
        }

        let content = format!("{}: {}", key, value);
        let mut entry = MemoryEntry::with_importance(
            content,
            MemoryType::Factual,
            0.9, // Preferences are very important
            None,
        );
        entry.metadata = metadata;

        let id = entry.id.clone();
        storage.store(entry)?;
        Ok(id)
    }

    /// Get a user preference
    pub fn get_preference(&self, key: &str, storage: &dyn MemoryStorage) -> Option<String> {
        let facts = storage.list_by_type(MemoryType::Factual);

        for fact in facts {
            if fact.metadata.tags.contains(&"preference".to_string())
                && fact.metadata.tags.contains(&key.to_string())
            {
                // Check namespace if set
                if let Some(ref ns) = self.namespace {
                    let ns_tag = format!("namespace:{}", ns);
                    if !fact.metadata.tags.contains(&ns_tag) {
                        continue;
                    }
                }

                if let Some(colon_pos) = fact.content.find(':') {
                    let value = fact.content[colon_pos + 1..].trim();
                    return Some(value.to_string());
                }
            }
        }

        None
    }

    /// Search facts by pattern with optional importance filtering
    pub fn search_facts(
        &self,
        pattern: &str,
        min_importance: Option<f32>,
        max_results: Option<usize>,
        storage: &dyn MemoryStorage,
    ) -> Vec<(String, String)> {
        let facts = storage.list_by_type(MemoryType::Factual);
        let pattern_lower = pattern.to_lowercase();
        let mut results: Vec<(String, String, f32)> = Vec::new();

        for fact in facts {
            // Check namespace if set
            if let Some(ref ns) = self.namespace {
                let ns_tag = format!("namespace:{}", ns);
                if !fact.metadata.tags.contains(&ns_tag) {
                    continue;
                }
            }

            // Check importance filter
            if let Some(min_imp) = min_importance {
                if fact.importance_score < min_imp {
                    continue;
                }
            }

            // Parse "key: value" format
            if let Some(colon_pos) = fact.content.find(':') {
                let key = fact.content[..colon_pos].trim();
                let value = fact.content[colon_pos + 1..].trim();

                // Case-insensitive pattern matching on key or value
                if key.to_lowercase().contains(&pattern_lower)
                    || value.to_lowercase().contains(&pattern_lower)
                {
                    results.push((key.to_string(), value.to_string(), fact.importance_score));
                }
            }
        }

        // Sort by importance (highest first)
        results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        // Apply max_results limit
        if let Some(limit) = max_results {
            results.truncate(limit);
        }

        // Return only key-value pairs (drop importance)
        results.into_iter().map(|(k, v, _)| (k, v)).collect()
    }

    /// Get facts by importance range
    pub fn get_facts_by_importance(
        &self,
        min_importance: f32,
        max_importance: Option<f32>,
        storage: &dyn MemoryStorage,
    ) -> Vec<(String, String)> {
        let facts = storage.list_by_type(MemoryType::Factual);
        let max_imp = max_importance.unwrap_or(1.0);
        let mut results: Vec<(String, String, f32)> = Vec::new();

        for fact in facts {
            // Check namespace if set
            if let Some(ref ns) = self.namespace {
                let ns_tag = format!("namespace:{}", ns);
                if !fact.metadata.tags.contains(&ns_tag) {
                    continue;
                }
            }

            // Check importance range
            if fact.importance_score >= min_importance && fact.importance_score <= max_imp {
                // Parse "key: value" format
                if let Some(colon_pos) = fact.content.find(':') {
                    let key = fact.content[..colon_pos].trim().to_string();
                    let value = fact.content[colon_pos + 1..].trim().to_string();
                    results.push((key, value, fact.importance_score));
                }
            }
        }

        // Sort by importance (highest first)
        results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        // Return only key-value pairs (drop importance)
        results.into_iter().map(|(k, v, _)| (k, v)).collect()
    }
}

impl Default for FactualMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::storage::InMemoryStorage;

    #[test]
    fn test_factual_memory_basic_operations() {
        let factual = FactualMemory::new();
        let mut storage = InMemoryStorage::new();

        // Store a fact
        factual
            .store_fact("user_name".to_string(), "Alice".to_string(), &mut storage)
            .unwrap();

        // Retrieve the fact
        let value = factual.get_fact("user_name", &storage);
        assert_eq!(value, Some("Alice".to_string()));

        // Update the fact
        let updated = factual
            .update_fact("user_name", "Bob".to_string(), &mut storage)
            .unwrap();
        assert!(updated);

        let value = factual.get_fact("user_name", &storage);
        assert_eq!(value, Some("Bob".to_string()));

        // Delete the fact
        let deleted = factual.delete_fact("user_name", &mut storage).unwrap();
        assert!(deleted);

        let value = factual.get_fact("user_name", &storage);
        assert_eq!(value, None);
    }

    #[test]
    fn test_factual_memory_namespace() {
        let mut factual = FactualMemory::new();
        let mut storage = InMemoryStorage::new();

        // Store facts in different namespaces
        factual.set_namespace("app1".to_string());
        factual
            .store_fact("setting".to_string(), "value1".to_string(), &mut storage)
            .unwrap();

        factual.set_namespace("app2".to_string());
        factual
            .store_fact("setting".to_string(), "value2".to_string(), &mut storage)
            .unwrap();

        // Retrieve from specific namespace
        factual.set_namespace("app1".to_string());
        assert_eq!(
            factual.get_fact("setting", &storage),
            Some("value1".to_string())
        );

        factual.set_namespace("app2".to_string());
        assert_eq!(
            factual.get_fact("setting", &storage),
            Some("value2".to_string())
        );
    }

    #[test]
    fn test_preferences() {
        let factual = FactualMemory::new();
        let mut storage = InMemoryStorage::new();

        factual
            .store_preference("theme".to_string(), "dark".to_string(), &mut storage)
            .unwrap();

        assert_eq!(
            factual.get_preference("theme", &storage),
            Some("dark".to_string())
        );
    }
}
