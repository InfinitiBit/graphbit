//! Binary document extractors (PDF, DOCX, Excel).

use calamine::{open_workbook_auto, Data, Reader};

use crate::errors::{GraphBitError, GraphBitResult};
use std::fmt::Write;

/// Extract content from PDF files
pub async fn extract_pdf_content(file_path: &str) -> GraphBitResult<String> {
    let bytes = std::fs::read(file_path).map_err(|e| {
        GraphBitError::validation("document_loader", format!("Failed to read PDF file: {e}"))
    })?;

    let text_content = pdf_extract::extract_text_from_mem(&bytes).map_err(|e| {
        GraphBitError::validation(
            "document_loader",
            format!("Failed to extract text from PDF: {e}"),
        )
    })?;

    if text_content.trim().is_empty() {
        return Err(GraphBitError::validation(
            "document_loader",
            "No text content could be extracted from the PDF",
        ));
    }

    Ok(text_content.trim().to_string())
}

/// Extract content from DOCX files
pub async fn extract_docx_content(file_path: &str) -> GraphBitResult<String> {
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

/// Extract content from Excel (XLSB, XLSX, XLS, etc.) files
pub async fn extract_excel_content(file_path: &str) -> GraphBitResult<String> {
    let mut workbook = open_workbook_auto(file_path).map_err(|e| {
        GraphBitError::validation(
            "document_loader",
            format!("Failed to open Excel file {}: {}", file_path, e),
        )
    })?;

    let mut result = String::new();
    result.push_str("Excel Document Content:\n\n");

    let sheet_names = workbook.sheet_names().to_vec();
    for sheet_name in sheet_names {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            writeln!(result, "Sheet: {}", sheet_name).unwrap();
            result.push_str("-".repeat(sheet_name.len() + 7).as_str());
            result.push('\n');

            for row in range.rows() {
                let row_str = row
                    .iter()
                    .map(|cell| match cell {
                        Data::Empty => "".to_string(),
                        Data::String(s) => s.clone(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(d) => d.to_string(),
                        Data::Error(e) => format!("Error({:?})", e),
                        _ => "".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" | ");

                if !row_str.trim().is_empty() {
                    writeln!(result, "{}", row_str).unwrap();
                }
            }
            result.push('\n');
        }
    }

    if result.trim().is_empty() {
        return Err(GraphBitError::validation(
            "document_loader",
            "No content could be extracted from the Excel file",
        ));
    }

    Ok(result.trim().to_string())
}
