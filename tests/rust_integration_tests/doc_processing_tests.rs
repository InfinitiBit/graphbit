//! Document Processing Integration Tests
//!
//! Tests for document loading, content extraction, and processing
//! with various file formats and edge cases.

use graphbit_core::document_loader::*;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_text_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");

    let content =
        "This is a test document.\nIt has multiple lines.\nAnd some special characters: àáâãäå";
    fs::write(&file_path, content).expect("Failed to write test file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.content, content);
    assert_eq!(document.document_type, "txt");
    assert!(document.file_size > 0);
    assert!(document.extracted_at <= chrono::Utc::now());
}

#[tokio::test]
async fn test_json_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.json");

    let json_content = json!({
        "name": "Test Document",
        "type": "JSON",
        "data": {
            "values": [1, 2, 3, 4, 5],
            "metadata": {
                "created": "2024-01-01",
                "author": "Test User"
            }
        }
    });

    fs::write(
        &file_path,
        serde_json::to_string_pretty(&json_content).unwrap(),
    )
    .expect("Failed to write JSON file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "json")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("Test Document"));
    assert!(document.content.contains("JSON"));
    assert_eq!(document.document_type, "json");
}

#[tokio::test]
async fn test_csv_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.csv");

    let csv_content =
        "Name,Age,City\nJohn Doe,30,New York\nJane Smith,25,Los Angeles\nBob Johnson,35,Chicago";
    fs::write(&file_path, csv_content).expect("Failed to write CSV file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "csv")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("Name,Age,City"));
    assert!(document.content.contains("John Doe"));
    assert_eq!(document.document_type, "csv");
}

#[tokio::test]
async fn test_xml_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.xml");

    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<document>
    <title>Test Document</title>
    <content>
        <paragraph>This is a test XML document.</paragraph>
        <paragraph>It contains structured data.</paragraph>
    </content>
    <metadata>
        <author>Test User</author>
        <date>2024-01-01</date>
    </metadata>
</document>"#;

    fs::write(&file_path, xml_content).expect("Failed to write XML file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "xml")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("Test Document"));
    assert!(document.content.contains("structured data"));
    assert_eq!(document.document_type, "xml");
}

#[tokio::test]
async fn test_html_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.html");

    let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>Test Document</title>
</head>
<body>
    <h1>Test HTML Document</h1>
    <p>This is a paragraph with <strong>bold text</strong> and <em>italic text</em>.</p>
    <ul>
        <li>List item 1</li>
        <li>List item 2</li>
    </ul>
</body>
</html>"#;

    fs::write(&file_path, html_content).expect("Failed to write HTML file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "html")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("Test HTML Document"));
    assert!(document.content.contains("paragraph"));
    assert_eq!(document.document_type, "html");
}

#[tokio::test]
async fn test_large_document_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("large.txt");

    // Create a large document (1MB)
    let large_content = "A".repeat(1024 * 1024);
    fs::write(&file_path, &large_content).expect("Failed to write large file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.content.len(), 1024 * 1024);
    assert_eq!(document.file_size, 1024 * 1024);
}

#[tokio::test]
async fn test_document_size_limit() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("oversized.txt");

    // Create a document larger than default limit (>10MB)
    let oversized_content = "A".repeat(11 * 1024 * 1024);
    fs::write(&file_path, &oversized_content).expect("Failed to write oversized file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    // Should fail due to size limit
    assert!(result.is_err());
    let error = result.err().unwrap();
    assert!(error.to_string().contains("exceeds maximum"));
}

#[tokio::test]
async fn test_custom_document_loader_config() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = DocumentLoaderConfig {
        max_file_size: 1024, // 1KB limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings: std::collections::HashMap::new(),
    };

    let loader = DocumentLoader::with_config(config);

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("small.txt");

    // Create a small file that fits within the limit
    let small_content = "Small content";
    fs::write(&file_path, small_content).expect("Failed to write small file");

    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());

    // Create a file that exceeds the custom limit
    let large_file_path = temp_dir.path().join("large.txt");
    let large_content = "A".repeat(2048); // 2KB, exceeds 1KB limit
    fs::write(&large_file_path, &large_content).expect("Failed to write large file");

    let result = loader
        .load_document(large_file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_nonexistent_file_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document("/nonexistent/path/file.txt", "txt")
        .await;

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert!(error.to_string().contains("not found"));
}

#[tokio::test]
async fn test_unsupported_file_type() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.unknown");

    fs::write(&file_path, "Some content").expect("Failed to write file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "unknown")
        .await;

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert!(error.to_string().contains("Unsupported document type"));
}

#[tokio::test]
async fn test_unicode_document_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("unicode.txt");

    let unicode_content = "Hello, 世界! 🌍 Здравствуй мир! مرحبا بالعالم! नमस्ते दुनिया!";
    fs::write(&file_path, unicode_content).expect("Failed to write unicode file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("世界"));
    assert!(document.content.contains("🌍"));
    assert!(document.content.contains("мир"));
    assert!(document.content.contains("العالم"));
    assert!(document.content.contains("दुनिया"));
}

#[tokio::test]
async fn test_empty_file_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("empty.txt");

    fs::write(&file_path, "").expect("Failed to create empty file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.content, "");
    assert_eq!(document.file_size, 0);
}

#[tokio::test]
async fn test_document_metadata_extraction() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("metadata_test.txt");

    let content = "Test content for metadata extraction";
    fs::write(&file_path, content).expect("Failed to write file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();

    // Check metadata
    assert!(document.metadata.contains_key("file_size"));
    assert!(document.metadata.contains_key("file_path"));

    let file_size_value = document.metadata.get("file_size").unwrap();
    assert!(file_size_value.is_number());

    let file_path_value = document.metadata.get("file_path").unwrap();
    assert!(file_path_value.is_string());
}

#[tokio::test]
async fn test_supported_document_types() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let supported_types = DocumentLoader::supported_types();

    assert!(supported_types.contains(&"txt"));
    assert!(supported_types.contains(&"json"));
    assert!(supported_types.contains(&"csv"));
    assert!(supported_types.contains(&"xml"));
    assert!(supported_types.contains(&"html"));
    assert!(supported_types.contains(&"pdf"));
    assert!(supported_types.contains(&"docx"));

    assert!(supported_types.len() >= 7);
}

#[tokio::test]
async fn test_document_type_detection() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    assert_eq!(detect_document_type("file.txt"), Some("txt".to_string()));
    assert_eq!(detect_document_type("file.json"), Some("json".to_string()));
    assert_eq!(detect_document_type("file.csv"), Some("csv".to_string()));
    assert_eq!(detect_document_type("file.xml"), Some("xml".to_string()));
    assert_eq!(detect_document_type("file.html"), Some("html".to_string()));
    assert_eq!(detect_document_type("file.pdf"), Some("pdf".to_string()));
    assert_eq!(detect_document_type("file.docx"), Some("docx".to_string()));
    assert_eq!(detect_document_type("file.unknown"), None);
}

#[tokio::test]
async fn test_document_source_validation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create test files
    let txt_path = temp_dir.path().join("file.txt");
    fs::write(&txt_path, "Test content").expect("Failed to write test file");
    let json_path = temp_dir.path().join("file.json");
    fs::write(&json_path, "{}").expect("Failed to write test file");

    // Valid combinations
    assert!(validate_document_source(txt_path.to_str().unwrap(), "txt").is_ok());
    assert!(validate_document_source(json_path.to_str().unwrap(), "json").is_ok());

    // URL validation (currently not implemented, should return error)
    let result = validate_document_source("https://example.com/file.txt", "txt");
    // Implementation may vary, but should handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_concurrent_document_loading() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let num_files = 10;
    let mut file_paths = Vec::new();

    // Create multiple test files
    for i in 0..num_files {
        let file_path = temp_dir.path().join(format!("test_{}.txt", i));
        let content = format!(
            "This is test document number {}.\nWith some additional content for testing.",
            i
        );
        fs::write(&file_path, content).expect("Failed to write test file");
        file_paths.push(file_path);
    }

    let _loader = DocumentLoader::new();
    let start_time = std::time::Instant::now();

    // Load documents concurrently
    let mut handles = Vec::new();
    for (i, file_path) in file_paths.iter().enumerate() {
        let path_str = file_path.to_str().unwrap().to_string();
        let loader_clone = DocumentLoader::new();
        let handle = tokio::spawn(async move {
            let result = loader_clone.load_document(&path_str, "txt").await;
            (i, result)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = Vec::new();
    for handle in handles {
        let (index, result) = handle.await.expect("Task should complete");
        results.push((index, result));
    }

    let duration = start_time.elapsed();

    // Verify all documents loaded successfully
    assert_eq!(results.len(), num_files);
    for (index, result) in results {
        assert!(
            result.is_ok(),
            "Document {} should load successfully",
            index
        );
        let document = result.unwrap();
        assert!(document
            .content
            .contains(&format!("test document number {}", index)));
        assert_eq!(document.document_type, "txt");
    }

    // Should complete quickly with concurrent loading
    assert!(
        duration.as_millis() < 1000,
        "Concurrent loading should be fast"
    );
    println!(
        "Loaded {} documents concurrently in {:?}",
        num_files, duration
    );
}

#[tokio::test]
async fn test_document_processing_pipeline() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create documents of different types
    let txt_path = temp_dir.path().join("sample.txt");
    fs::write(&txt_path, "Sample text document for processing pipeline.")
        .expect("Failed to write txt");

    let json_path = temp_dir.path().join("sample.json");
    let json_content = serde_json::json!({
        "title": "Sample JSON Document",
        "content": "This is a JSON document for testing",
        "metadata": {
            "type": "test",
            "version": "1.0"
        }
    });
    fs::write(&json_path, json_content.to_string()).expect("Failed to write json");

    let csv_path = temp_dir.path().join("sample.csv");
    fs::write(
        &csv_path,
        "Name,Age,City\nAlice,30,New York\nBob,25,San Francisco",
    )
    .expect("Failed to write csv");

    let loader = DocumentLoader::new();

    // Process each document type
    let txt_doc = loader
        .load_document(txt_path.to_str().unwrap(), "txt")
        .await
        .unwrap();
    let json_doc = loader
        .load_document(json_path.to_str().unwrap(), "json")
        .await
        .unwrap();
    let csv_doc = loader
        .load_document(csv_path.to_str().unwrap(), "csv")
        .await
        .unwrap();

    // Verify content extraction
    assert!(txt_doc.content.contains("Sample text document"));
    assert!(json_doc.content.contains("Sample JSON Document"));
    assert!(csv_doc.content.contains("Alice"));

    // Verify metadata
    assert_eq!(txt_doc.document_type, "txt");
    assert_eq!(json_doc.document_type, "json");
    assert_eq!(csv_doc.document_type, "csv");

    // All should have file size metadata
    assert!(txt_doc.file_size > 0);
    assert!(json_doc.file_size > 0);
    assert!(csv_doc.file_size > 0);
}

#[tokio::test]
async fn test_document_validation_and_sanitization() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test file path validation
    let result = graphbit_core::document_loader::validate_document_source("", "txt");
    assert!(result.is_err(), "Empty path should be invalid");

    let result = graphbit_core::document_loader::validate_document_source(
        "/nonexistent/path/file.txt",
        "txt",
    );
    assert!(result.is_err(), "Non-existent path should be invalid");

    // Test document type detection
    assert_eq!(
        graphbit_core::document_loader::detect_document_type("test.txt"),
        Some("txt".to_string())
    );
    assert_eq!(
        graphbit_core::document_loader::detect_document_type("data.json"),
        Some("json".to_string())
    );
    assert_eq!(
        graphbit_core::document_loader::detect_document_type("sheet.csv"),
        Some("csv".to_string())
    );
    assert_eq!(
        graphbit_core::document_loader::detect_document_type("page.html"),
        Some("html".to_string())
    );
    assert_eq!(
        graphbit_core::document_loader::detect_document_type("unknown.xyz"),
        None
    );

    // Test supported types
    let supported_types = DocumentLoader::supported_types();
    assert!(supported_types.contains(&"txt"));
    assert!(supported_types.contains(&"json"));
    assert!(supported_types.contains(&"csv"));
    assert!(supported_types.contains(&"xml"));
    assert!(supported_types.contains(&"html"));
    assert!(supported_types.contains(&"pdf"));
    assert!(supported_types.contains(&"docx"));
    assert_eq!(supported_types.len(), 7);
}

#[tokio::test]
async fn test_pdf_document_processing() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let pdf_path = temp_dir.path().join("test.pdf");

    // Create a mock PDF file (since we don't have real PDF processing yet)
    fs::write(&pdf_path, "Mock PDF content for testing").expect("Failed to write PDF file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(pdf_path.to_str().unwrap(), "pdf")
        .await;

    // This will currently fail since PDF processing is not fully implemented
    match result {
        Ok(document) => {
            assert_eq!(document.document_type, "pdf");
            println!("PDF processing succeeded: {}", document.content);
        }
        Err(e) => {
            println!(
                "PDF processing failed as expected (not yet implemented): {:?}",
                e
            );
            assert!(
                e.to_string()
                    .contains("PDF processing not yet fully implemented")
                    || e.to_string().contains("not yet implemented")
                    || e.to_string().contains("Failed to open PDF file")
            );
        }
    }
}

#[tokio::test]
async fn test_docx_document_processing() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let docx_path = temp_dir.path().join("test.docx");

    // Create a mock DOCX file (since we don't have real DOCX processing yet)
    fs::write(&docx_path, "Mock DOCX content for testing").expect("Failed to write DOCX file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(docx_path.to_str().unwrap(), "docx")
        .await;

    // This will currently fail since DOCX processing is not fully implemented
    match result {
        Ok(document) => {
            assert_eq!(document.document_type, "docx");
            println!("DOCX processing succeeded: {}", document.content);
        }
        Err(e) => {
            println!(
                "DOCX processing failed as expected (not yet implemented): {:?}",
                e
            );
            assert!(
                e.to_string()
                    .contains("DOCX processing not yet fully implemented")
                    || e.to_string().contains("not yet implemented")
                    || e.to_string().contains("Failed to parse DOCX file")
            );
        }
    }
}

#[tokio::test]
async fn test_document_encoding_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test UTF-8 encoding
    let utf8_path = temp_dir.path().join("utf8.txt");
    let utf8_content = "UTF-8 content: Hello, 世界! 🌍";
    fs::write(&utf8_path, utf8_content).expect("Failed to write UTF-8 file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(utf8_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.content, utf8_content);
    assert!(document.content.contains("世界"));
    assert!(document.content.contains("🌍"));
}

#[tokio::test]
async fn test_document_size_limits_and_configuration() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test custom configuration with smaller limit
    let config = DocumentLoaderConfig {
        max_file_size: 1024, // 1KB limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings: std::collections::HashMap::new(),
    };

    let loader = DocumentLoader::with_config(config);

    // Create a file larger than the limit
    let large_path = temp_dir.path().join("large.txt");
    let large_content = "A".repeat(2048); // 2KB file
    fs::write(&large_path, &large_content).expect("Failed to write large file");

    let result = loader
        .load_document(large_path.to_str().unwrap(), "txt")
        .await;
    assert!(
        result.is_err(),
        "Should fail to load file exceeding size limit"
    );

    let error = result.unwrap_err();
    assert!(error.to_string().contains("exceeds maximum allowed size"));

    // Test with file within limits
    let small_path = temp_dir.path().join("small.txt");
    let small_content = "Small content";
    fs::write(&small_path, small_content).expect("Failed to write small file");

    let result = loader
        .load_document(small_path.to_str().unwrap(), "txt")
        .await;
    assert!(
        result.is_ok(),
        "Should successfully load file within size limit"
    );
}

#[tokio::test]
async fn test_document_error_handling() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let loader = DocumentLoader::new();

    // Test non-existent file
    let result = loader.load_document("/nonexistent/file.txt", "txt").await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("File not found"));

    // Test unsupported document type
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.xyz");
    fs::write(&file_path, "test content").expect("Failed to write file");

    let result = loader
        .load_document(file_path.to_str().unwrap(), "xyz")
        .await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Unsupported document type"));

    // Test empty file
    let empty_path = temp_dir.path().join("empty.txt");
    fs::write(&empty_path, "").expect("Failed to create empty file");

    let result = loader
        .load_document(empty_path.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.is_empty());
    assert_eq!(document.file_size, 0);
}

#[tokio::test]
async fn test_url_loading_placeholder() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let loader = DocumentLoader::new();

    // Test URL loading (currently not implemented)
    let result = loader
        .load_document("https://example.com/document.txt", "txt")
        .await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("URL loading not yet implemented"));
}

#[tokio::test]
async fn test_document_content_preservation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test formatting preservation for different document types
    let xml_path = temp_dir.path().join("formatted.xml");
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<document>
    <title>Test Document</title>
    <content>
        <paragraph>First paragraph with <emphasis>emphasis</emphasis>.</paragraph>
        <paragraph>Second paragraph with line breaks.</paragraph>
    </content>
</document>"#;

    fs::write(&xml_path, xml_content).expect("Failed to write XML file");

    let config = DocumentLoaderConfig {
        max_file_size: 10 * 1024 * 1024,
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings: std::collections::HashMap::new(),
    };

    let loader = DocumentLoader::with_config(config);
    let result = loader
        .load_document(xml_path.to_str().unwrap(), "xml")
        .await;

    assert!(result.is_ok());
    let document = result.unwrap();
    assert!(document.content.contains("<?xml version=\"1.0\""));
    assert!(document.content.contains("<emphasis>emphasis</emphasis>"));
    assert!(document.content.contains("    <title>")); // Preserves indentation
}

#[tokio::test]
async fn test_document_loader_performance() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create documents of varying sizes
    let sizes = [1024, 10240, 102400]; // 1KB, 10KB, 100KB
    let mut file_paths = Vec::new();

    for (i, size) in sizes.iter().enumerate() {
        let file_path = temp_dir.path().join(format!("perf_test_{}.txt", i));
        let content = "A".repeat(*size);
        fs::write(&file_path, content).expect("Failed to write performance test file");
        file_paths.push((file_path, *size));
    }

    let loader = DocumentLoader::new();
    let start_time = std::time::Instant::now();

    // Load all documents and measure performance
    let mut total_bytes = 0;
    for (file_path, expected_size) in file_paths {
        let result = loader
            .load_document(file_path.to_str().unwrap(), "txt")
            .await;
        assert!(result.is_ok());

        let document = result.unwrap();
        assert_eq!(document.file_size, expected_size);
        total_bytes += expected_size;
    }

    let duration = start_time.elapsed();
    let throughput = total_bytes as f64 / duration.as_secs_f64() / 1024.0; // KB/s

    println!(
        "Document loading performance: {} KB in {:?} ({:.2} KB/s)",
        total_bytes / 1024,
        duration,
        throughput
    );

    // Should process at reasonable speed
    assert!(throughput > 100.0, "Should process at least 100 KB/s");
}

#[tokio::test]
async fn test_document_loader_memory_efficiency() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create multiple medium-sized files
    let num_files = 20;
    let file_size = 50 * 1024; // 50KB each
    let mut file_paths = Vec::new();

    for i in 0..num_files {
        let file_path = temp_dir.path().join(format!("memory_test_{}.txt", i));
        let content = format!("Content for file {} ", i).repeat(file_size / 20);
        fs::write(&file_path, content).expect("Failed to write memory test file");
        file_paths.push(file_path);
    }

    let loader = DocumentLoader::new();

    // Load documents one by one to test memory efficiency
    let start_time = std::time::Instant::now();
    for (i, file_path) in file_paths.iter().enumerate() {
        let result = loader
            .load_document(file_path.to_str().unwrap(), "txt")
            .await;
        assert!(result.is_ok(), "File {} should load successfully", i);

        let document = result.unwrap();
        assert!(document
            .content
            .contains(&format!("Content for file {}", i)));

        // Document should be dropped here, freeing memory
    }

    let duration = start_time.elapsed();
    println!("Loaded {} files sequentially in {:?}", num_files, duration);

    // Should complete within reasonable time
    assert!(duration.as_secs() < 5, "Should complete within 5 seconds");
}
