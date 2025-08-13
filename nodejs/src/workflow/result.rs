//! Workflow result for GraphBit Node.js bindings

// use napi::bindgen_prelude::*; // Unused import
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub struct WorkflowResult {
    /// Whether the workflow execution was successful
    pub success: bool,
    /// Result data from the workflow
    pub data: HashMap<String, String>,
    /// Error message if execution failed
    pub error: Option<String>,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
    /// Execution duration in milliseconds
    pub duration_ms: Option<i64>,
}

#[napi]
impl WorkflowResult {
    /// Create a successful workflow result
    #[napi(factory)]
    pub fn success(data: HashMap<String, String>, duration_ms: Option<i64>) -> Self {
        Self {
            success: true,
            data,
            error: None,
            metadata: HashMap::new(),
            duration_ms,
        }
    }

    /// Create a failed workflow result
    #[napi(factory)]
    pub fn failure(error: String, duration_ms: Option<i64>) -> Self {
        Self {
            success: false,
            data: HashMap::new(),
            error: Some(error),
            metadata: HashMap::new(),
            duration_ms,
        }
    }

    /// Get a specific result value
    #[napi]
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Check if result has a specific key
    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }

    /// Get all result keys
    #[napi]
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
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
}