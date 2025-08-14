//! Workflow context for GraphBit Node.js bindings

// use napi::bindgen_prelude::*; // Unused import
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone)]
pub struct WorkflowContext {
    /// Context data as key-value pairs
    pub data: HashMap<String, String>,
    /// Metadata for the context
    pub metadata: HashMap<String, String>,
}

#[napi]
impl WorkflowContext {
    /// Create a new workflow context
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set a value in the context
    #[napi]
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get a value from the context
    #[napi]
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Set metadata
    #[napi]
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata
    #[napi]
    pub fn get_metadata(&self, key: String) -> Option<String> {
        self.metadata.get(&key).cloned()
    }

    /// Check if context contains a key
    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }

    /// Remove a key from context
    #[napi]
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.data.remove(&key)
    }

    /// Clear all context data
    #[napi]
    pub fn clear(&mut self) {
        self.data.clear();
        self.metadata.clear();
    }

    /// Get all keys in the context
    #[napi]
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}