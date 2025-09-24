//! Document loading and processing functionality for `GraphBit` workflows
//!
//! This module provides utilities for loading and extracting content from various
//! document formats including PDF, TXT, Word, JSON, CSV, XML, and HTML.

use crate::errors::{GraphBitError, GraphBitResult};
use csv::ReaderBuilder;
use quick_xml::events::Event;
use quick_xml::Reader;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: DocumentLoaderConfig::default(),
        }
    }

    /// Create a new document loader with custom configuration
    #[must_use]
    pub const fn with_config(config: DocumentLoaderConfig) -> Self {
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

        // Enhanced CSV parsing: convert to structured, readable format
        match self.parse_csv_to_structured_text(&content) {
            Ok(structured_content) => Ok(structured_content),
            Err(_) => {
                // Fallback to raw content if parsing fails
                Ok(content)
            }
        }
    }

    /// Parse CSV content into structured, readable text format
    fn parse_csv_to_structured_text(
        &self,
        csv_content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(Cursor::new(csv_content));

        let mut result = String::new();

        // Get headers
        let headers = reader.headers()?.clone();
        let header_count = headers.len();

        result.push_str("CSV Document Content:\n");
        result.push_str(&format!(
            "Columns ({}): {}\n\n",
            header_count,
            headers.iter().collect::<Vec<_>>().join(", ")
        ));

        // Process records
        let mut row_count = 0;
        for (index, record) in reader.records().enumerate() {
            let record = record?;
            row_count += 1;

            result.push_str(&format!("Row {}:\n", index + 1));

            for (i, field) in record.iter().enumerate() {
                if i < header_count {
                    let header = headers.get(i).unwrap_or("Unknown");
                    result.push_str(&format!("  {}: {}\n", header, field.trim()));
                }
            }
            result.push('\n');

            // Limit output for very large CSV files
            if row_count >= 100 {
                result.push_str(&format!(
                    "... and {} more rows (truncated for readability)\n",
                    reader.records().count()
                ));
                break;
            }
        }

        result.push_str(&format!("Total rows processed: {}\n", row_count));
        Ok(result)
    }

    /// Extract content from XML files
    async fn extract_xml_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read XML file: {e}"))
        })?;

        // Enhanced XML parsing: extract structured text content
        match self.parse_xml_to_structured_text(&content) {
            Ok(structured_content) => Ok(structured_content),
            Err(_) => {
                // Fallback to raw content if parsing fails
                Ok(content)
            }
        }
    }

    /// Parse XML content into structured, readable text format
    fn parse_xml_to_structured_text(
        &self,
        xml_content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut result = String::new();
        let mut buf = Vec::new();
        let mut current_path = Vec::new();
        let mut text_content = Vec::new();

        result.push_str("XML Document Content:\n\n");

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())?.to_string();
                    current_path.push(name.clone());

                    // Add element structure info
                    let indent = "  ".repeat(current_path.len() - 1);
                    result.push_str(&format!("{}Element: {}\n", indent, name));

                    // Handle attributes
                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = std::str::from_utf8(&attr.value)?;
                        result.push_str(&format!("{}  @{}: {}\n", indent, key, value));
                    }
                }
                Ok(Event::End(_)) => {
                    if !text_content.is_empty() {
                        let content = text_content.join(" ").trim().to_string();
                        if !content.is_empty() {
                            let indent = "  ".repeat(current_path.len());
                            result.push_str(&format!("{}Text: {}\n", indent, content));
                        }
                        text_content.clear();
                    }
                    current_path.pop();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.trim().to_string();
                    if !text.is_empty() {
                        text_content.push(text);
                    }
                }
                Ok(Event::CData(e)) => {
                    let text = std::str::from_utf8(&e)?;
                    if !text.trim().is_empty() {
                        text_content.push(text.to_string());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("Error parsing XML: {}", e).into()),
                _ => {} // Ignore other events
            }
            buf.clear();
        }

        result.push_str("\nXML parsing completed.\n");
        Ok(result)
    }

    /// Extract content from HTML files
    async fn extract_html_content(&self, file_path: &str) -> GraphBitResult<String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            GraphBitError::validation("document_loader", format!("Failed to read HTML file: {e}"))
        })?;

        // Enhanced HTML parsing: extract structured text content
        match self.parse_html_to_structured_text(&content) {
            Ok(structured_content) => Ok(structured_content),
            Err(_) => {
                // Fallback to raw content if parsing fails
                Ok(content)
            }
        }
    }

    /// Parse HTML content into structured, readable text format
    fn parse_html_to_structured_text(
        &self,
        html_content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let document = Html::parse_document(html_content);
        let mut result = String::new();

        result.push_str("HTML Document Content:\n\n");

        // Extract title
        if let Ok(title_selector) = Selector::parse("title") {
            if let Some(title) = document.select(&title_selector).next() {
                result.push_str(&format!(
                    "Title: {}\n\n",
                    title.text().collect::<String>().trim()
                ));
            }
        }

        // Extract meta description
        if let Ok(meta_selector) = Selector::parse("meta[name='description']") {
            if let Some(meta) = document.select(&meta_selector).next() {
                if let Some(content) = meta.value().attr("content") {
                    result.push_str(&format!("Description: {}\n\n", content.trim()));
                }
            }
        }

        // Extract headings with hierarchy
        for level in 1..=6 {
            let selector_str = format!("h{}", level);
            if let Ok(heading_selector) = Selector::parse(&selector_str) {
                for heading in document.select(&heading_selector) {
                    let text = heading.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        let indent = "  ".repeat(level - 1);
                        result.push_str(&format!("{}H{}: {}\n", indent, level, text));
                    }
                }
            };
        }

        // Extract paragraphs
        if let Ok(p_selector) = Selector::parse("p") {
            result.push_str("\nParagraphs:\n");
            for paragraph in document.select(&p_selector) {
                let text = paragraph.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    result.push_str(&format!("  {}\n\n", text));
                }
            }
        }

        // Extract lists
        if let Ok(ul_selector) = Selector::parse("ul, ol") {
            result.push_str("Lists:\n");
            for list in document.select(&ul_selector) {
                let list_type = list.value().name();
                result.push_str(&format!(
                    "  {} List:\n",
                    if list_type == "ul" {
                        "Unordered"
                    } else {
                        "Ordered"
                    }
                ));

                if let Ok(li_selector) = Selector::parse("li") {
                    for (index, item) in list.select(&li_selector).enumerate() {
                        let text = item.text().collect::<String>().trim().to_string();
                        if !text.is_empty() {
                            let prefix = if list_type == "ul" {
                                "•"
                            } else {
                                &format!("{}.", index + 1)
                            };
                            result.push_str(&format!("    {} {}\n", prefix, text));
                        }
                    }
                }
                result.push('\n');
            }
        }

        // Extract links
        if let Ok(a_selector) = Selector::parse("a[href]") {
            result.push_str("Links:\n");
            for link in document.select(&a_selector) {
                let text = link.text().collect::<String>().trim().to_string();
                if let Some(href) = link.value().attr("href") {
                    if !text.is_empty() && !href.is_empty() {
                        result.push_str(&format!("  {} -> {}\n", text, href));
                    }
                }
            }
            result.push('\n');
        }

        // Extract table data
        if let Ok(table_selector) = Selector::parse("table") {
            result.push_str("Tables:\n");
            for (table_index, table) in document.select(&table_selector).enumerate() {
                result.push_str(&format!("  Table {}:\n", table_index + 1));

                // Extract headers
                if let Ok(th_selector) = Selector::parse("th") {
                    let headers: Vec<String> = table
                        .select(&th_selector)
                        .map(|th| th.text().collect::<String>().trim().to_string())
                        .filter(|h| !h.is_empty())
                        .collect();

                    if !headers.is_empty() {
                        result.push_str(&format!("    Headers: {}\n", headers.join(" | ")));
                    }
                }

                // Extract rows
                if let Ok(tr_selector) = Selector::parse("tr") {
                    for (row_index, row) in table.select(&tr_selector).enumerate() {
                        if let Ok(td_selector) = Selector::parse("td") {
                            let cells: Vec<String> = row
                                .select(&td_selector)
                                .map(|td| td.text().collect::<String>().trim().to_string())
                                .filter(|c| !c.is_empty())
                                .collect();

                            if !cells.is_empty() {
                                result.push_str(&format!(
                                    "    Row {}: {}\n",
                                    row_index + 1,
                                    cells.join(" | ")
                                ));
                            }
                        }
                    }
                }
                result.push('\n');
            }
        }

        result.push_str("HTML parsing completed.\n");
        Ok(result)
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
    #[must_use]
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
#[must_use]
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
