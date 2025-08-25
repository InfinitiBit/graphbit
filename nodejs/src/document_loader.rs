//! Document loader for GraphBit Node.js bindings

use crate::validation::validate_non_empty_string;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone)]
pub struct DocumentLoaderConfig {
    /// Type of document loader (e.g., "pdf", "docx", "txt", "url")
    pub loader_type: String,
    /// Additional configuration parameters
    pub config: HashMap<String, String>,
    /// Chunk size for processing large documents
    pub chunk_size: Option<i32>,
    /// Chunk overlap for text splitting
    pub chunk_overlap: Option<i32>,
}

#[napi]
impl DocumentLoaderConfig {
    /// Create a new document loader configuration
    #[napi(constructor)]
    pub fn new(
        loader_type: String,
        chunk_size: Option<i32>,
        chunk_overlap: Option<i32>,
    ) -> Result<Self> {
        validate_non_empty_string(&loader_type, "loader_type")?;

        Ok(Self {
            loader_type,
            config: HashMap::new(),
            chunk_size,
            chunk_overlap,
        })
    }

    /// Create a configuration for PDF documents
    #[napi(factory)]
    pub fn pdf(chunk_size: Option<i32>, chunk_overlap: Option<i32>) -> Self {
        Self {
            loader_type: "pdf".to_string(),
            config: HashMap::new(),
            chunk_size,
            chunk_overlap,
        }
    }

    /// Create a configuration for DOCX documents
    #[napi(factory)]
    pub fn docx(chunk_size: Option<i32>, chunk_overlap: Option<i32>) -> Self {
        Self {
            loader_type: "docx".to_string(),
            config: HashMap::new(),
            chunk_size,
            chunk_overlap,
        }
    }

    /// Create a configuration for text files
    #[napi(factory)]
    pub fn text(chunk_size: Option<i32>, chunk_overlap: Option<i32>) -> Self {
        Self {
            loader_type: "text".to_string(),
            config: HashMap::new(),
            chunk_size,
            chunk_overlap,
        }
    }

    /// Create a configuration for URLs
    #[napi(factory)]
    pub fn url(chunk_size: Option<i32>, chunk_overlap: Option<i32>) -> Self {
        Self {
            loader_type: "url".to_string(),
            config: HashMap::new(),
            chunk_size,
            chunk_overlap,
        }
    }

    /// Set a configuration parameter
    #[napi]
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Get a configuration parameter
    #[napi]
    pub fn get_config(&self, key: String) -> Option<String> {
        self.config.get(&key).cloned()
    }
}

#[napi(object)]
pub struct DocumentContent {
    /// The extracted text content
    pub content: String,
    /// Metadata about the document
    pub metadata: HashMap<String, String>,
    /// Number of pages (for multi-page documents)
    pub page_count: Option<i32>,
    /// Document type detected
    pub document_type: String,
    /// File size in bytes
    pub file_size: Option<i64>,
}

#[napi]
pub struct DocumentLoader {
    config: DocumentLoaderConfig,
}

#[napi]
impl DocumentLoader {
    /// Create a new document loader
    #[napi(constructor)]
    pub fn new(config: &DocumentLoaderConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Load a document from a file path
    #[napi]
    pub async fn load_from_path(&self, file_path: String) -> Result<DocumentContent> {
        validate_non_empty_string(&file_path, "file_path")?;

        let result = tokio::task::spawn(async move {
            // Use the actual core DocumentLoader API
            let loader = graphbit_core::document_loader::DocumentLoader::new();

            // Detect document type from file extension
            let document_type = graphbit_core::document_loader::detect_document_type(&file_path)
                .unwrap_or_else(|| "txt".to_string());

            loader
                .load_document(&file_path, &document_type)
                .await
                .map_err(|e| format!("Failed to load document: {}", e))
        })
        .await;

        match result {
            Ok(Ok(core_content)) => {
                // Convert HashMap<String, serde_json::Value> to HashMap<String, String>
                let mut metadata = HashMap::new();
                for (key, value) in core_content.metadata {
                    metadata.insert(key, value.to_string());
                }

                Ok(DocumentContent {
                    content: core_content.content,
                    metadata,
                    page_count: None, // Core doesn't provide page count directly
                    document_type: core_content.document_type,
                    file_size: Some(core_content.file_size as i64),
                })
            }
            Ok(Err(e)) => Err(Error::new(Status::GenericFailure, e)),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Task execution failed: {}", e),
            )),
        }
    }

    /// Load a document from text content
    #[napi]
    pub async fn load_from_text(
        &self,
        content: String,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<DocumentContent> {
        validate_non_empty_string(&content, "content")?;

        // For text content, we just return it directly with minimal processing
        let metadata = metadata.unwrap_or_default();
        let content_size = content.len();

        Ok(DocumentContent {
            content,
            metadata,
            page_count: Some(1), // Single page for text content
            document_type: "text".to_string(),
            file_size: Some(content_size as i64),
        })
    }

    /// Load a document from a URL
    #[napi]
    pub async fn load_from_url(&self, url: String) -> Result<DocumentContent> {
        validate_non_empty_string(&url, "url")?;

        let result = tokio::task::spawn(async move {
            // Use the actual core DocumentLoader API
            let loader = graphbit_core::document_loader::DocumentLoader::new();

            // Try to detect document type from URL
            let document_type = if url.ends_with(".pdf") {
                "pdf"
            } else if url.ends_with(".docx") {
                "docx"
            } else if url.ends_with(".txt") {
                "txt"
            } else {
                "html" // Default for web pages
            };

            loader
                .load_document(&url, document_type)
                .await
                .map_err(|e| format!("Failed to load from URL: {}", e))
        })
        .await;

        match result {
            Ok(Ok(core_content)) => {
                // Convert HashMap<String, serde_json::Value> to HashMap<String, String>
                let mut metadata = HashMap::new();
                for (key, value) in core_content.metadata {
                    metadata.insert(key, value.to_string());
                }

                Ok(DocumentContent {
                    content: core_content.content,
                    metadata,
                    page_count: None,
                    document_type: core_content.document_type,
                    file_size: Some(core_content.file_size as i64),
                })
            }
            Ok(Err(e)) => Err(Error::new(Status::GenericFailure, e)),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Task execution failed: {}", e),
            )),
        }
    }

    /// Get the current configuration
    #[napi(getter)]
    pub fn config(&self) -> DocumentLoaderConfig {
        self.config.clone()
    }

    /// Update the configuration
    #[napi]
    pub fn update_config(&mut self, config: &DocumentLoaderConfig) {
        self.config = config.clone();
    }
}
