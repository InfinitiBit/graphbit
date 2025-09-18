//! Document loading and processing functionality for `GraphBit` workflows
//!
//! This module provides utilities for loading and extracting content from various
//! document formats including PDF, TXT, Word, JSON, CSV, XML, and HTML.

use crate::errors::{GraphBitError, GraphBitResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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
        // Validate document type
        let supported_types = ["pdf", "txt", "docx", "json", "csv", "xml", "html"];
        if !supported_types.contains(&document_type.to_lowercase().as_str()) {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("Unsupported document type: {document_type}"),
            ));
        }

        // Check if source is a URL or file path
        let content = if source_path.starts_with("http://") || source_path.starts_with("https://") {
            self.load_from_url(source_path, document_type).await?
        } else if source_path.contains("://") {
            // This looks like a URL but not HTTP/HTTPS
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

        // Check if file exists
        if !path.exists() {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("File not found: {file_path}"),
            ));
        }

        // Check file size
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

        // Extract content based on document type
        let content = match document_type.to_lowercase().as_str() {
            "txt" => self.extract_text_content(file_path).await?,
            "pdf" => self.extract_pdf_content(file_path).await?,
            "docx" => self.extract_docx_content(file_path).await?,
            "json" => self.extract_json_content(file_path).await?,
            "csv" => self.extract_csv_content(file_path).await?,
            "xml" => self.extract_xml_content(file_path).await?,
            "html" => self.extract_html_content(file_path).await?,
            _ => {
                return Err(GraphBitError::validation(
                    "document_loader",
                    format!(
                        "Unsupported document type: {document_type}. Supported types: {:?}",
                        Self::supported_types()
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
        // Validate URL format
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("Invalid URL format: {url}"),
            ));
        }

        // Create HTTP client with timeout and user agent
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

        // Fetch the document
        let response = client.get(url).send().await.map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to fetch URL {url}: {e}"))
        })?;

        // Check response status
        if !response.status().is_success() {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("HTTP error {}: {url}", response.status()),
            ));
        }

        // Check content length
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

        // Get content type from response headers
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        // Download the content
        let content_bytes = response.bytes().await.map_err(|e| {
            GraphBitError::validation(
                "document_loader",
                format!("Failed to read response body: {e}"),
            )
        })?;

        // Check actual size
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

        // Convert bytes to string based on document type
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

        // Process content based on type
        let processed_content = match document_type.to_lowercase().as_str() {
            "json" => {
                // Validate and format JSON
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

        // Create metadata
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

    /// Extract content from plain text files
    async fn extract_text_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read text file: {e}"))
        })?;
        Ok(content)
    }

    /// Extract content from JSON files
    async fn extract_json_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read JSON file: {e}"))
        })?;

        // Validate JSON and optionally format it
        let json_value: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Invalid JSON content: {e}"))
        })?;

        // Return formatted JSON for better readability
        serde_json::to_string_pretty(&json_value).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to format JSON: {e}"))
        })
    }

    /// Extract content from CSV files
    async fn extract_csv_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read CSV file: {e}"))
        })?;

        // For now, return the raw CSV content
        // TODO: Could be enhanced to parse CSV and convert to structured format
        Ok(content)
    }

    /// Extract content from XML files
    async fn extract_xml_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read XML file: {e}"))
        })?;

        // For now, return the raw XML content
        // TODO: Could be enhanced to parse XML and extract structured data
        Ok(content)
    }

    /// Extract content from HTML files
    async fn extract_html_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read HTML file: {e}"))
        })?;

        // For now, return the raw HTML content
        // TODO: Could be enhanced to extract text content from HTML tags
        Ok(content)
    }

    /// Extract content from PDF files
    async fn extract_pdf_content(&self, file_path: &str) -> GraphBitResult<String> {
        use lopdf::Document;

        let doc = Document::load(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to open PDF file: {e}"))
        })?;

        let mut text_content = String::new();

        // Extract text from each page
        for page_id in doc.get_pages().keys() {
            if let Ok(page_text) = doc.extract_text(&[*page_id]) {
                text_content.push_str(&page_text);
                text_content.push('\n');
            }
        }

        if text_content.trim().is_empty() {
            return Err(GraphBitError::validation(
                "document_loader",
                "No text content could be extracted from the PDF",
            ));
        }

        Ok(text_content.trim().to_string())
    }

    /// Extract content from DOCX files
    async fn extract_docx_content(&self, file_path: &str) -> GraphBitResult<String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to open DOCX file: {e}"))
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read DOCX file: {e}"))
        })?;

        let docx = docx_rs::read_docx(&buffer).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to parse DOCX file: {e}"))
        })?;

        let mut text_content = String::new();

        // Extract text from document children
        for child in &docx.document.children {
            if let docx_rs::DocumentChild::Paragraph(paragraph) = child {
                for para_child in &paragraph.children {
                    if let docx_rs::ParagraphChild::Run(run_element) = para_child {
                        for run_child in &run_element.children {
                            if let docx_rs::RunChild::Text(text) = run_child {
                                text_content.push_str(&text.text);
                            }
                        }
                    }
                }
                text_content.push('\n');
            }
        }

        if text_content.trim().is_empty() {
            return Err(GraphBitError::validation(
                "document_loader",
                "No text content could be extracted from the DOCX file",
            ));
        }

        Ok(text_content.trim().to_string())
    }

    /// Get supported document types
    pub fn supported_types() -> Vec<&'static str> {
        vec!["txt", "pdf", "docx", "json", "csv", "xml", "html"]
    }
}

impl Default for DocumentLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to determine document type from file extension
pub fn detect_document_type(file_path: &str) -> Option<String> {
    let supported_types = DocumentLoader::supported_types();
    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .filter(|ext| supported_types.contains(&ext.as_str()))
}

/// Utility function to validate document path and type
pub fn validate_document_source(source_path: &str, document_type: &str) -> GraphBitResult<()> {
    // Check if document type is supported
    let supported_types = DocumentLoader::supported_types();
    if !supported_types.contains(&document_type) {
        return Err(GraphBitError::validation(
            "document_loader",
            format!(
                "Unsupported document type: {document_type}. Supported types: {supported_types:?}",
            ),
        ));
    }

    // If it's a file path, check if it exists
    if !source_path.starts_with("http://") && !source_path.starts_with("https://") {
        let path = Path::new(source_path);
        if !path.exists() {
            return Err(GraphBitError::validation(
                "document_loader",
                format!("File not found: {source_path}"),
            ));
        }
    }

    Ok(())
}
