//! Text-based document extractors (TXT, JSON, CSV).

use std::io::Cursor;

use csv::ReaderBuilder;

use crate::errors::{GraphBitError, GraphBitResult};
use std::fmt::Write;

/// Extract content from plain text files
pub async fn extract_text_content(file_path: &str) -> GraphBitResult<String> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read text file: {e}"))
    })?;
    Ok(content)
}

/// Extract content from JSON files
pub async fn extract_json_content(file_path: &str) -> GraphBitResult<String> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read JSON file: {e}"))
    })?;

    let json_value: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Invalid JSON content: {e}"))
    })?;

    serde_json::to_string_pretty(&json_value).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to format JSON: {e}"))
    })
}

/// Extract content from CSV files
pub async fn extract_csv_content(file_path: &str) -> GraphBitResult<String> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read CSV file: {e}"))
    })?;

    match parse_csv_to_structured_text(&content) {
        Ok(structured_content) => Ok(structured_content),
        Err(_) => Ok(content),
    }
}

/// Parse CSV content into structured, readable text format
pub fn parse_csv_to_structured_text(
    csv_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(Cursor::new(csv_content));

    let mut result = String::new();

    let headers = reader.headers()?.clone();
    let header_count = headers.len();

    result.push_str("CSV Document Content:\n");
    write!(
        result,
        "Columns ({}): {}\n\n",
        header_count,
        headers.iter().collect::<Vec<_>>().join(", ")
    )
    .unwrap();

    let mut row_count = 0;
    for (index, record) in reader.records().enumerate() {
        let record = record?;
        row_count += 1;

        writeln!(result, "Row {}:", index + 1).unwrap();

        for (i, field) in record.iter().enumerate() {
            if i < header_count {
                let header = headers.get(i).unwrap_or("Unknown");
                writeln!(result, "  {header}: {}", field.trim()).unwrap();
            }
        }
        result.push('\n');

        if row_count >= 100 {
            writeln!(
                result,
                "... and {} more rows (truncated for readability)",
                reader.records().count()
            )
            .unwrap();
            break;
        }
    }

    writeln!(result, "Total rows processed: {row_count}").unwrap();
    Ok(result)
}
