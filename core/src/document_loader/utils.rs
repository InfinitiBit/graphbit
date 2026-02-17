//! Document loader utility functions.

use std::path::Path;

use crate::errors::{GraphBitError, GraphBitResult};

/// Get supported document types
pub fn supported_types() -> Vec<&'static str> {
    vec![
        "txt", "pdf", "docx", "json", "csv", "xml", "html", "xlsb", "xlsx", "xls",
    ]
}

/// Helper function to determine document type from file extension
pub fn detect_document_type(file_path: &str) -> Option<String> {
    let supported_types = supported_types();
    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_lowercase)
        .filter(|ext| supported_types.contains(&ext.as_str()))
}

/// Utility function to validate document path and type
pub fn validate_document_source(source_path: &str, document_type: &str) -> GraphBitResult<()> {
    let supported_types = supported_types();
    if !supported_types.contains(&document_type) {
        return Err(GraphBitError::validation(
            "document_loader",
            format!(
                "Unsupported document type: {document_type}. Supported types: {supported_types:?}",
            ),
        ));
    }

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
