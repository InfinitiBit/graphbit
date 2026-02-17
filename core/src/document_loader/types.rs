//! Document loader types and configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document loader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLoaderConfig {
    /// Maximum file size to process (in bytes)
    pub max_file_size: usize,
    /// Character encoding for text files
    pub default_encoding: String,
    /// Whether to preserve formatting
    pub preserve_formatting: bool,
    /// Document-specific extraction settings
    pub extraction_settings: HashMap<String, serde_json::Value>,
}

impl Default for DocumentLoaderConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            default_encoding: "utf-8".to_string(),
            preserve_formatting: false,
            extraction_settings: HashMap::new(),
        }
    }
}

/// Loaded document content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    /// Original file path or URL
    pub source: String,
    /// Document type
    pub document_type: String,
    /// Extracted text content
    pub content: String,
    /// Document metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// File size in bytes
    pub file_size: usize,
    /// Extraction timestamp
    pub extracted_at: chrono::DateTime<chrono::Utc>,
}
