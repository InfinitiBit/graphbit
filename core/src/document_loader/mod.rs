//! Document loading and processing functionality for `GraphBit` workflows
//!
//! This module provides utilities for loading and extracting content from various
//! document formats including PDF, TXT, Word, JSON, CSV, XML, and HTML.

mod binary;
mod markup;
mod text;
mod types;
mod utils;

pub use types::{DocumentContent, DocumentLoaderConfig};
pub use utils::{detect_document_type, supported_types, validate_document_source};

use std::collections::HashMap;
use std::path::Path;

use crate::errors::{GraphBitError, GraphBitResult};

use binary::{extract_docx_content, extract_excel_content, extract_pdf_content};
use markup::{extract_html_content, extract_xml_content};
use text::{extract_csv_content, extract_json_content, extract_text_content};

/// Document loader for processing various file formats
pub struct DocumentLoader {
    config: DocumentLoaderConfig,
}

impl DocumentLoader {
    /// Create a new document loader with default configuration
    pub fn new() -> Self {
        Self {
            config: DocumentLoaderConfig::default(),
        }
    }

    /// Create a new document loader with custom configuration
    pub fn with_config(config: DocumentLoaderConfig) -> Self {
        Self { config }
    }

    /// Load and extract content from a document
    pub async fn load_document(
        &self,
        source_path: &str,
        document_type: &str,
    ) -> GraphBitResult<DocumentContent> {
        let supported_types = utils::supported_types();
        if !supported_types.contains(&document_type.to_lowercase().as_str()) {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("Unsupported document type: {document_type}"),
            ));
        }

        let content = if source_path.starts_with("http://") || source_path.starts_with("https://") {
            self.load_from_url(source_path, document_type).await?
        } else if source_path.contains("://") {
            return Err(GraphBitError::validation(
                "document_loader",
                format!(
                    "Invalid URL format: {source_path}. Only HTTP and HTTPS URLs are supported"
                ),
            ));
        } else {
            self.load_from_file(source_path, document_type).await?
        };

        Ok(content)
    }

    /// Load document from file path
    async fn load_from_file(
        &self,
        file_path: &str,
        document_type: &str,
    ) -> GraphBitResult<DocumentContent> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("File not found: {file_path}"),
            ));
        }

        let metadata = std::fs::metadata(path).map_err(|e| {
            GraphBitError::validation(
                "document_loader",
                format!("Failed to read file metadata: {e}"),
            )
        })?;

        let file_size = metadata.len() as usize;
        if file_size > self.config.max_file_size {
            return Err(GraphBitError::validation(
                "document_loader",
                format!(
                    "File size ({file_size} bytes) exceeds maximum allowed size ({} bytes)",
                    self.config.max_file_size
                ),
            ));
        }

        let content = match document_type.to_lowercase().as_str() {
            "txt" => extract_text_content(file_path).await?,
            "pdf" => extract_pdf_content(file_path).await?,
            "docx" => extract_docx_content(file_path).await?,
            "json" => extract_json_content(file_path).await?,
            "csv" => extract_csv_content(file_path).await?,
            "xml" => extract_xml_content(file_path).await?,
            "html" => extract_html_content(file_path).await?,
            "xlsb" | "xlsx" | "xls" => extract_excel_content(file_path).await?,
            _ => {
                return Err(GraphBitError::validation(
                    "document_loader",
                    format!(
                        "Unsupported document type: {document_type}. Supported types: {:?}",
                        utils::supported_types()
                    ),
                ))
            }
        };

        let mut doc_metadata = HashMap::new();
        doc_metadata.insert(
            "file_size".to_string(),
            serde_json::Value::Number(file_size.into()),
        );
        doc_metadata.insert(
            "file_path".to_string(),
            serde_json::Value::String(file_path.to_string()),
        );

        Ok(DocumentContent {
            source: file_path.to_string(),
            document_type: document_type.to_string(),
            content,
            metadata: doc_metadata,
            file_size,
            extracted_at: chrono::Utc::now(),
        })
    }

    /// Load document from URL
    async fn load_from_url(
        &self,
        url: &str,
        document_type: &str,
    ) -> GraphBitResult<DocumentContent> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("Invalid URL format: {url}"),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("GraphBit Document Loader/1.0")
            .build()
            .map_err(|e| {
                GraphBitError::validation(
                    "document_loader",
                    format!("Failed to create HTTP client: {e}"),
                )
            })?;

        let response = client.get(url).send().await.map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to fetch URL {url}: {e}"))
        })?;

        if !response.status().is_success() {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("HTTP error {}: {url}", response.status()),
            ));
        }

        if let Some(content_length) = response.content_length() {
            if content_length as usize > self.config.max_file_size {
                return Err(GraphBitError::validation(
                    "document_loader",
                    format!(
                        "Remote file size ({content_length} bytes) exceeds maximum allowed size ({} bytes)",
                        self.config.max_file_size
                    ),
                ));
            }
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let content_bytes = response.bytes().await.map_err(|e| {
            GraphBitError::validation(
                "document_loader",
                format!("Failed to read response body: {e}"),
            )
        })?;

        if content_bytes.len() > self.config.max_file_size {
            return Err(GraphBitError::validation(
                "document_loader",
                format!(
                    "Downloaded file size ({} bytes) exceeds maximum allowed size ({} bytes)",
                    content_bytes.len(),
                    self.config.max_file_size
                ),
            ));
        }

        let content = match document_type.to_lowercase().as_str() {
            "txt" | "json" | "csv" | "xml" | "html" => String::from_utf8(content_bytes.to_vec())
                .map_err(|e| {
                    GraphBitError::validation(
                        "document_loader",
                        format!("Failed to decode text content: {e}"),
                    )
                })?,
            "pdf" | "docx" => {
                return Err(GraphBitError::validation(
                    "document_loader",
                    format!("URL loading for {document_type} documents is not yet supported"),
                ));
            }
            _ => {
                return Err(GraphBitError::validation(
                    "document_loader",
                    format!("Unsupported document type for URL loading: {document_type}"),
                ));
            }
        };

        let processed_content = match document_type.to_lowercase().as_str() {
            "json" => {
                let json_value: serde_json::Value =
                    serde_json::from_str(&content).map_err(|e| {
                        GraphBitError::validation(
                            "document_loader",
                            format!("Invalid JSON content: {e}"),
                        )
                    })?;
                serde_json::to_string_pretty(&json_value).map_err(|e| {
                    GraphBitError::validation(
                        "document_loader",
                        format!("Failed to format JSON: {e}"),
                    )
                })?
            }
            _ => content,
        };

        let mut metadata = HashMap::new();
        metadata.insert(
            "file_size".to_string(),
            serde_json::Value::Number(content_bytes.len().into()),
        );
        metadata.insert(
            "url".to_string(),
            serde_json::Value::String(url.to_string()),
        );
        metadata.insert(
            "content_type".to_string(),
            serde_json::Value::String(content_type),
        );

        Ok(DocumentContent {
            source: url.to_string(),
            document_type: document_type.to_string(),
            content: processed_content,
            metadata,
            file_size: content_bytes.len(),
            extracted_at: chrono::Utc::now(),
        })
    }

    /// Get supported document types
    pub fn supported_types() -> Vec<&'static str> {
        utils::supported_types()
    }
}

impl Default for DocumentLoader {
    fn default() -> Self {
        Self::new()
    }
}
