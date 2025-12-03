//! Document loader bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::document_loader::{
    DocumentLoader as CoreDocumentLoader,
    DocumentContent as CoreDocumentContent,
    DocumentLoaderConfig as CoreDocumentLoaderConfig,
};
use std::collections::HashMap;

/// Document content
#[napi(object)]
pub struct DocumentContent {
    pub content: String,
    pub metadata: Option<String>,
    pub source: String,
    pub document_type: String,
}

impl From<CoreDocumentContent> for DocumentContent {
    fn from(doc: CoreDocumentContent) -> Self {
        Self {
            content: doc.content,
            metadata: serde_json::to_string(&doc.metadata).ok(),
            source: doc.source,
            document_type: doc.document_type,
        }
    }
}

/// Document loader configuration
#[napi(object)]
pub struct DocumentLoaderConfig {
    pub max_file_size: Option<i64>,
    pub default_encoding: Option<String>,
    pub preserve_formatting: Option<bool>,
}

impl From<DocumentLoaderConfig> for CoreDocumentLoaderConfig {
    fn from(config: DocumentLoaderConfig) -> Self {
        Self {
            max_file_size: config.max_file_size.map(|s| s as usize).unwrap_or(10 * 1024 * 1024),
            default_encoding: config.default_encoding.unwrap_or_else(|| "utf-8".to_string()),
            preserve_formatting: config.preserve_formatting.unwrap_or(false),
            extraction_settings: HashMap::new(),
        }
    }
}

impl From<CoreDocumentLoaderConfig> for DocumentLoaderConfig {
    fn from(config: CoreDocumentLoaderConfig) -> Self {
        Self {
            max_file_size: Some(config.max_file_size as i64),
            default_encoding: Some(config.default_encoding),
            preserve_formatting: Some(config.preserve_formatting),
        }
    }
}

/// Document loader
#[napi]
pub struct DocumentLoader {
    inner: CoreDocumentLoader,
    config: CoreDocumentLoaderConfig,
}

#[napi]
impl DocumentLoader {
    /// Create a new document loader with default configuration
    #[napi(constructor)]
    pub fn new() -> Self {
        let config = CoreDocumentLoaderConfig::default();
        Self {
            inner: CoreDocumentLoader::new(),
            config,
        }
    }

    /// Create a document loader with custom configuration
    #[napi(factory)]
    pub fn with_config(config: DocumentLoaderConfig) -> Self {
        let core_config: CoreDocumentLoaderConfig = config.into();

        Self {
            inner: CoreDocumentLoader::with_config(core_config.clone()),
            config: core_config,
        }
    }

    /// Load a document from a file path
    #[napi]
    pub async fn load_file(&self, path: String, document_type: String) -> Result<DocumentContent> {
        let content = self.inner.load_document(&path, &document_type)
            .await
            .map_err(crate::errors::to_napi_error)?;

        Ok(DocumentContent::from(content))
    }

    /// Load a document from text content
    #[napi]
    pub async fn load_text(&self, text: String, source: Option<String>) -> Result<DocumentContent> {
        let content = CoreDocumentContent {
            source: source.unwrap_or_else(|| "text".to_string()),
            document_type: "txt".to_string(),
            content: text,
            metadata: HashMap::new(),
            file_size: 0,
            extracted_at: chrono::Utc::now(),
        };

        Ok(DocumentContent::from(content))
    }

    /// Get document loader configuration
    #[napi]
    pub fn config(&self) -> DocumentLoaderConfig {
        self.config.clone().into()
    }
}

