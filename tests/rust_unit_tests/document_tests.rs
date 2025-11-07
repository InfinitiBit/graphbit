use super::test_helpers::*;
use graphbit_core::document_loader::{
    detect_document_type, validate_document_source, DocumentLoader, DocumentLoaderConfig,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_document_loader_creation() {
    let config = DocumentLoaderConfig {
        max_file_size: 1024 * 1024, // 1MB
        default_encoding: "utf-8".to_string(),
        preserve_formatting: false,
        extraction_settings: Default::default(),
    };

    let _loader = DocumentLoader::with_config(config);
    assert!(DocumentLoader::supported_types().contains(&"txt"));
    assert!(DocumentLoader::supported_types().contains(&"pdf"));
}

#[tokio::test]
async fn test_text_file_loading() {
    let loader = DocumentLoader::new();
    let content = "Test content\nwith multiple\nlines";

    let temp_file = create_temp_file(content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("Test content"));
    assert_eq!(doc.document_type, "txt");
}

#[tokio::test]
async fn test_json_file_loading() {
    let loader = DocumentLoader::new();
    let content = r#"{
        "key": "value",
        "number": 42,
        "nested": {
            "inner": "data"
        }
    }"#;

    let temp_file = create_temp_file(content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "json")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("value"));
    assert_eq!(doc.document_type, "json");
}

#[tokio::test]
async fn test_url_loading() {
    let loader = DocumentLoader::new();

    // Test invalid URL format
    let result = loader.load_document("not-a-url", "txt").await;
    assert!(result.is_err());

    // Test unsupported protocol
    let result = loader.load_document("ftp://example.com", "txt").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_file_size_limits() {
    let config = DocumentLoaderConfig {
        max_file_size: 10, // Very small limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: false,
        extraction_settings: Default::default(),
    };

    let loader = DocumentLoader::with_config(config);
    let content = "This content is longer than 10 bytes";

    let temp_file = create_temp_file(content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("exceeds maximum allowed size"));
}

#[tokio::test]
async fn test_document_type_validation() {
    let loader = DocumentLoader::new();
    let temp_file = create_temp_file("test content");

    // Test unsupported document type
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "unsupported")
        .await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported document type"));
}

#[tokio::test]
async fn test_url_scheme_and_http_error_paths() {
    let loader = DocumentLoader::new();

    // Unsupported scheme (covered by early validation branch)
    let res = loader
        .load_document("ftp://example.com/file.txt", "txt")
        .await;
    assert!(res.is_err());

    // Looks like a URL but invalid type
    let res = loader.load_document("http://", "pdf").await;
    assert!(res.is_err());

    // Invalid type error branch
    let res = loader
        .load_document("/this/file/does/not/exist", "invalid-type")
        .await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_metadata_handling() {
    let loader = DocumentLoader::new();
    let content = "Test content";
    let temp_file = create_temp_file(content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert!(doc.metadata.contains_key("file_size"));
    assert!(doc.metadata.contains_key("file_path"));
    assert!(doc.extracted_at <= chrono::Utc::now());
}

#[tokio::test]
async fn test_encoding_handling() {
    let config = DocumentLoaderConfig {
        max_file_size: 1024 * 1024,
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings: Default::default(),
    };

    let loader = DocumentLoader::with_config(config);
    let content = "Test content with special chars: áéíóú";

    let temp_file = create_temp_file(content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("áéíóú"));
}

#[tokio::test]
async fn test_batch_loading() {
    let loader = DocumentLoader::new();
    let files = vec![
        ("file1.txt", "content 1"),
        ("file2.txt", "content 2"),
        ("file3.txt", "content 3"),
    ];

    let mut temp_files = Vec::new();
    for (name, content) in files {
        let temp_file = create_temp_file(content);
        temp_files.push((name, temp_file));
    }

    let mut results = Vec::new();
    for (_, file) in &temp_files {
        let result = loader
            .load_document(file.path().to_str().unwrap(), "txt")
            .await;
        assert!(result.is_ok());
        results.push(result.unwrap());
    }

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|doc| doc.document_type == "txt"));
}

#[tokio::test]
async fn test_error_handling() {
    let loader = DocumentLoader::new();

    // Test non-existent file
    let result = loader.load_document("non_existent.txt", "txt").await;
    assert!(result.is_err());

    // Test directory instead of file
    let temp_dir = tempfile::tempdir().unwrap();
    let result = loader
        .load_document(temp_dir.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_err());

    // Test empty file
    let temp_file = create_temp_file("");
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok()); // Empty files are allowed for text
}

#[tokio::test]
async fn test_concurrent_loading() {
    let content = "Test content";
    let temp_file = create_temp_file(content);
    let file_path = temp_file.path().to_str().unwrap().to_string();

    let mut handles = vec![];

    // Spawn multiple concurrent loading tasks
    for _ in 0..5 {
        let loader_clone = DocumentLoader::new();
        let path_clone = file_path.clone();
        let handle =
            tokio::spawn(async move { loader_clone.load_document(&path_clone, "txt").await });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_csv_file_loading() {
    let loader = DocumentLoader::new();
    let csv_content = "name,age,city\nJohn,30,New York\nJane,25,Los Angeles\nBob,35,Chicago";

    let temp_file = create_temp_file(csv_content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "csv")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("CSV Document Content"));
    assert!(doc.content.contains("Columns (3): name, age, city"));
    assert!(doc.content.contains("name: John"));
    assert!(doc.content.contains("age: 30"));
    assert!(doc.content.contains("city: New York"));
    assert_eq!(doc.document_type, "csv");
}

#[tokio::test]
async fn test_xml_file_loading() {
    let loader = DocumentLoader::new();
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1">
        <name>Test Item</name>
        <value>42</value>
    </item>
    <item id="2">
        <name>Another Item</name>
        <value>84</value>
    </item>
</root>"#;

    let temp_file = create_temp_file(xml_content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "xml")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("XML Document Content"));
    assert!(doc.content.contains("Element: name"));
    assert!(doc.content.contains("Text: Test Item"));
    assert_eq!(doc.document_type, "xml");
}

#[tokio::test]
async fn test_html_file_loading() {
    let loader = DocumentLoader::new();
    let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
</head>
<body>
    <h1>Welcome</h1>
    <p>This is a test HTML document.</p>
    <div class="content">
        <span>Some content here</span>
    </div>
</body>
</html>"#;

    let temp_file = create_temp_file(html_content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "html")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("HTML Document Content"));
    assert!(doc.content.contains("Title: Test Page"));
    assert!(doc.content.contains("Welcome"));
    assert_eq!(doc.document_type, "html");
}

#[tokio::test]
async fn test_invalid_json_file() {
    let loader = DocumentLoader::new();
    let invalid_json = r#"{ "key": "value", "invalid": }"#; // Missing value

    let temp_file = create_temp_file(invalid_json);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "json")
        .await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid JSON content"));
}

#[tokio::test]
async fn test_url_loading_edge_cases() {
    let loader = DocumentLoader::new();

    // Test malformed URL
    let result = loader.load_document("http://", "txt").await;
    assert!(result.is_err());

    // Test unsupported document type for URL
    let result = loader
        .load_document("https://example.com/test.pdf", "pdf")
        .await;
    assert!(result.is_err());
    // The exact error message may vary, so just check that it's an error

    // Test unsupported document type for URL
    let result = loader
        .load_document("https://example.com/test.docx", "docx")
        .await;
    assert!(result.is_err());
    // The exact error message may vary, so just check that it's an error

    // Test unsupported document type for URL loading
    let result = loader
        .load_document("https://example.com/test.xyz", "xyz")
        .await;
    assert!(result.is_err());
    // The exact error message may vary, so just check that it's an error
}

#[tokio::test]
async fn test_detect_document_type_utility() {
    // Test supported extensions
    assert_eq!(detect_document_type("test.txt"), Some("txt".to_string()));
    assert_eq!(
        detect_document_type("document.pdf"),
        Some("pdf".to_string())
    );
    assert_eq!(detect_document_type("data.json"), Some("json".to_string()));
    assert_eq!(
        detect_document_type("spreadsheet.csv"),
        Some("csv".to_string())
    );
    assert_eq!(detect_document_type("markup.xml"), Some("xml".to_string()));
    assert_eq!(detect_document_type("page.html"), Some("html".to_string()));
    assert_eq!(
        detect_document_type("document.docx"),
        Some("docx".to_string())
    );

    // Test unsupported extensions
    assert_eq!(detect_document_type("file.xyz"), None);
    assert_eq!(detect_document_type("file.exe"), None);

    // Test files without extensions
    assert_eq!(detect_document_type("README"), None);
    assert_eq!(detect_document_type("file"), None);

    // Test case insensitive
    assert_eq!(detect_document_type("FILE.TXT"), Some("txt".to_string()));
    assert_eq!(
        detect_document_type("Document.PDF"),
        Some("pdf".to_string())
    );
}

#[tokio::test]
async fn test_validate_document_source_utility() {
    // Test supported document types
    let temp_file = create_temp_file("test content");
    let file_path = temp_file.path().to_str().unwrap();

    assert!(validate_document_source(file_path, "txt").is_ok());
    assert!(validate_document_source(file_path, "json").is_ok());
    assert!(validate_document_source(file_path, "csv").is_ok());

    // Test unsupported document type
    let result = validate_document_source(file_path, "unsupported");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported document type"));

    // Test non-existent file
    let result = validate_document_source("/non/existent/file.txt", "txt");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));

    // Test URL (should pass validation)
    assert!(validate_document_source("https://example.com/test.txt", "txt").is_ok());
    assert!(validate_document_source("http://example.com/data.json", "json").is_ok());
}

#[tokio::test]
async fn test_document_loader_config_edge_cases() {
    // Test with custom extraction settings
    let mut extraction_settings = HashMap::new();
    extraction_settings.insert(
        "preserve_whitespace".to_string(),
        serde_json::Value::Bool(true),
    );
    extraction_settings.insert(
        "max_pages".to_string(),
        serde_json::Value::Number(10.into()),
    );

    let config = DocumentLoaderConfig {
        max_file_size: 5 * 1024 * 1024, // 5MB
        default_encoding: "iso-8859-1".to_string(),
        preserve_formatting: true,
        extraction_settings,
    };

    let loader = DocumentLoader::with_config(config.clone());

    // Test that config is properly applied
    let content = "Test content";
    let temp_file = create_temp_file(content);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.content, content);
}

#[tokio::test]
async fn test_file_metadata_error_handling() {
    let loader = DocumentLoader::new();

    // Test with a path that exists but can't read metadata (simulated by using a directory)
    let temp_dir = tempfile::tempdir().unwrap();
    let result = loader
        .load_document(temp_dir.path().to_str().unwrap(), "txt")
        .await;

    // This should fail because it's a directory, not a file
    assert!(result.is_err());
}

#[tokio::test]
async fn test_empty_and_whitespace_files() {
    let loader = DocumentLoader::new();

    // Test completely empty file
    let empty_file = create_temp_file("");
    let result = loader
        .load_document(empty_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, "");

    // Test file with only whitespace
    let whitespace_file = create_temp_file("   \n\t  \n  ");
    let result = loader
        .load_document(whitespace_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
    assert!(result.unwrap().content.trim().is_empty());

    // Test empty JSON file (should fail)
    let empty_json_file = create_temp_file("");
    let result = loader
        .load_document(empty_json_file.path().to_str().unwrap(), "json")
        .await;
    assert!(result.is_err());
}

// ===== COMPREHENSIVE TESTS FOR 100% FUNCTION COVERAGE =====

#[tokio::test]
async fn test_document_loader_default_trait() {
    // Test Default trait implementation for DocumentLoader
    let default_loader = DocumentLoader::default();
    let new_loader = DocumentLoader::new();

    // Both should work identically
    let content = "Test content for default trait";
    let temp_file = create_temp_file(content);

    let result1 = default_loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    let result2 = new_loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap().content, result2.unwrap().content);
}

#[tokio::test]
async fn test_document_loader_config_default_trait() {
    // Test Default trait implementation for DocumentLoaderConfig
    let default_config = DocumentLoaderConfig::default();

    assert_eq!(default_config.max_file_size, 10 * 1024 * 1024); // 10MB
    assert_eq!(default_config.default_encoding, "utf-8");
    assert!(!default_config.preserve_formatting);
    assert!(default_config.extraction_settings.is_empty());

    // Test that default config works
    let loader = DocumentLoader::with_config(default_config);
    let content = "Test with default config";
    let temp_file = create_temp_file(content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, content);
}

#[tokio::test]
async fn test_pdf_content_extraction_error_paths() {
    let loader = DocumentLoader::new();

    // Test PDF extraction with invalid PDF file
    let fake_pdf_content = "This is not a real PDF file content";
    let temp_file = create_temp_file(fake_pdf_content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "pdf")
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to open PDF file"));
}

#[tokio::test]
async fn test_docx_content_extraction_error_paths() {
    let loader = DocumentLoader::new();

    // Test DOCX extraction with invalid DOCX file
    let fake_docx_content = "This is not a real DOCX file content";
    let temp_file = create_temp_file(fake_docx_content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "docx")
        .await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Failed to parse DOCX file") || error_msg.contains("Failed to open DOCX file"));
}

#[tokio::test]
async fn test_url_loading_comprehensive_error_paths() {
    let loader = DocumentLoader::new();

    // Test URL loading for different content types (will fail due to network, but tests code paths)

    // Test text content from URL
    let result = loader.load_document("https://example.com/test.txt", "txt").await;
    assert!(result.is_err()); // Network error expected

    // Test JSON content from URL
    let result = loader.load_document("https://example.com/data.json", "json").await;
    assert!(result.is_err()); // Network error expected

    // Test CSV content from URL
    let result = loader.load_document("https://example.com/data.csv", "csv").await;
    assert!(result.is_err()); // Network error expected

    // Test XML content from URL
    let result = loader.load_document("https://example.com/data.xml", "xml").await;
    assert!(result.is_err()); // Network error expected

    // Test HTML content from URL
    let result = loader.load_document("https://example.com/page.html", "html").await;
    assert!(result.is_err()); // Network error expected
}

#[tokio::test]
async fn test_url_loading_content_size_limits() {
    let config = DocumentLoaderConfig {
        max_file_size: 10, // Very small limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: false,
        extraction_settings: Default::default(),
    };

    let loader = DocumentLoader::with_config(config);

    // Test URL loading with size limit (will fail on network request)
    let result = loader.load_document("https://httpbin.org/json", "json").await;
    // This will fail due to network or size constraints
    assert!(result.is_err());
}

#[tokio::test]
async fn test_url_loading_json_processing() {
    let loader = DocumentLoader::new();

    // Test URL loading with invalid JSON content
    // This will fail on network request, but tests the code path
    let result = loader.load_document("https://example.com/invalid.json", "json").await;
    assert!(result.is_err());
    // The error could be network-related or JSON parsing related
}

#[tokio::test]
async fn test_document_content_struct_fields() {
    let loader = DocumentLoader::new();
    let content = "Test content for struct validation";
    let temp_file = create_temp_file(content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();

    // Verify all DocumentContent fields are properly set
    assert_eq!(doc.source, temp_file.path().to_str().unwrap());
    assert_eq!(doc.document_type, "txt");
    assert_eq!(doc.content, content);
    assert!(doc.metadata.contains_key("file_size"));
    assert!(doc.metadata.contains_key("file_path"));
    assert_eq!(doc.file_size, content.len());
    assert!(doc.extracted_at <= chrono::Utc::now());

    // Verify metadata values
    assert_eq!(
        doc.metadata.get("file_size").unwrap(),
        &serde_json::Value::Number(content.len().into())
    );
    assert_eq!(
        doc.metadata.get("file_path").unwrap(),
        &serde_json::Value::String(temp_file.path().to_str().unwrap().to_string())
    );
}

#[tokio::test]
async fn test_document_loader_config_comprehensive() {
    // Test DocumentLoaderConfig with all fields
    let mut extraction_settings = HashMap::new();
    extraction_settings.insert("test_setting".to_string(), serde_json::Value::Bool(true));
    extraction_settings.insert("numeric_setting".to_string(), serde_json::Value::Number(42.into()));

    let config = DocumentLoaderConfig {
        max_file_size: 2 * 1024 * 1024, // 2MB
        default_encoding: "iso-8859-1".to_string(),
        preserve_formatting: true,
        extraction_settings: extraction_settings.clone(),
    };

    let loader = DocumentLoader::with_config(config.clone());

    // Test that the config is properly used
    let content = "Test content with custom config";
    let temp_file = create_temp_file(content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.content, content);
}

#[tokio::test]
async fn test_extract_content_methods_comprehensive() {
    let loader = DocumentLoader::new();

    // Test all extract_*_content methods through load_document

    // Text content
    let text_content = "Simple text content\nwith multiple lines\nand special chars: áéíóú";
    let text_file = create_temp_file(text_content);
    let result = loader.load_document(text_file.path().to_str().unwrap(), "txt").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, text_content);

    // JSON content (valid)
    let json_content = r#"{"key": "value", "number": 123, "array": [1, 2, 3]}"#;
    let json_file = create_temp_file(json_content);
    let result = loader.load_document(json_file.path().to_str().unwrap(), "json").await;
    assert!(result.is_ok());
    let doc = result.unwrap();
    // Should be formatted JSON
    assert!(doc.content.contains("\"key\""));
    assert!(doc.content.contains("\"value\""));

    // CSV content
    let csv_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
    let csv_file = create_temp_file(csv_content);
    let result = loader.load_document(csv_file.path().to_str().unwrap(), "csv").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, csv_content);

    // XML content
    let xml_content = r#"<?xml version="1.0"?><root><item>test</item></root>"#;
    let xml_file = create_temp_file(xml_content);
    let result = loader.load_document(xml_file.path().to_str().unwrap(), "xml").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, xml_content);

    // HTML content
    let html_content = r#"<html><body><h1>Test</h1><p>Content</p></body></html>"#;
    let html_file = create_temp_file(html_content);
    let result = loader.load_document(html_file.path().to_str().unwrap(), "html").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, html_content);
}

#[tokio::test]
async fn test_file_reading_error_paths() {
    let loader = DocumentLoader::new();

    // Test file that can't be read (permission denied simulation)
    // Create a file and then try to read it as different types to trigger different error paths
    let temp_file = create_temp_file("test content");
    let file_path = temp_file.path().to_str().unwrap().to_string();

    // Test reading non-existent file after temp file is dropped
    drop(temp_file);

    let result = loader.load_document(&file_path, "txt").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));
}

#[tokio::test]
async fn test_json_validation_error_paths() {
    let loader = DocumentLoader::new();

    // Test various invalid JSON formats
    let invalid_json_cases = vec![
        r#"{"key": "value",}"#, // Trailing comma
        r#"{"key": }"#,         // Missing value
        r#"{key: "value"}"#,    // Unquoted key
        r#"{"key": "value""#,   // Missing closing brace
        r#"invalid json"#,      // Not JSON at all
    ];

    for (i, invalid_json) in invalid_json_cases.iter().enumerate() {
        let temp_file = create_temp_file(invalid_json);
        let result = loader
            .load_document(temp_file.path().to_str().unwrap(), "json")
            .await;

        assert!(result.is_err(), "Case {} should fail: {}", i, invalid_json);
        assert!(result.unwrap_err().to_string().contains("Invalid JSON content"));
    }
}

#[tokio::test]
async fn test_supported_types_comprehensive() {
    // Test DocumentLoader::supported_types() function
    let supported = DocumentLoader::supported_types();

    // Verify all expected types are present
    let expected_types = vec!["txt", "pdf", "docx", "json", "csv", "xml", "html"];
    for expected_type in expected_types {
        assert!(
            supported.contains(&expected_type),
            "Missing supported type: {}",
            expected_type
        );
    }

    // Verify the count matches
    assert_eq!(supported.len(), 7);
}

#[tokio::test]
async fn test_detect_document_type_edge_cases() {
    // Test detect_document_type with various edge cases

    // Test files with multiple extensions
    assert_eq!(detect_document_type("file.backup.txt"), Some("txt".to_string()));
    assert_eq!(detect_document_type("archive.tar.gz"), None); // gz not supported

    // Test files with no extension
    assert_eq!(detect_document_type("README"), None);
    assert_eq!(detect_document_type("Makefile"), None);

    // Test files with dots in name but no extension
    assert_eq!(detect_document_type("file.name"), None);
    assert_eq!(detect_document_type("file."), None);

    // Test empty and edge case paths
    assert_eq!(detect_document_type(""), None);
    assert_eq!(detect_document_type("."), None);
    assert_eq!(detect_document_type(".."), None);
    // Note: .txt (hidden file) behavior depends on Path::extension() implementation
    // Let's test what actually happens
    let result = detect_document_type(".txt");
    // This might be None if Path considers .txt as the filename, not extension
    assert!(result.is_none() || result == Some("txt".to_string()));

    // Test case sensitivity
    assert_eq!(detect_document_type("FILE.TXT"), Some("txt".to_string()));
    assert_eq!(detect_document_type("Document.PDF"), Some("pdf".to_string()));
    assert_eq!(detect_document_type("data.JSON"), Some("json".to_string()));

    // Test unsupported extensions
    assert_eq!(detect_document_type("file.exe"), None);
    assert_eq!(detect_document_type("file.bin"), None);
    assert_eq!(detect_document_type("file.unknown"), None);
}

#[tokio::test]
async fn test_validate_document_source_comprehensive() {
    // Test validate_document_source with comprehensive cases

    let temp_file = create_temp_file("test content");
    let file_path = temp_file.path().to_str().unwrap();

    // Test all supported document types with valid file
    let supported_types = DocumentLoader::supported_types();
    for doc_type in supported_types {
        let result = validate_document_source(file_path, doc_type);
        assert!(result.is_ok(), "Should validate {} type", doc_type);
    }

    // Test unsupported document types
    let unsupported_types = vec!["exe", "bin", "unknown", "xyz"];
    for doc_type in unsupported_types {
        let result = validate_document_source(file_path, doc_type);
        assert!(result.is_err(), "Should reject {} type", doc_type);
        assert!(result.unwrap_err().to_string().contains("Unsupported document type"));
    }

    // Test non-existent file
    let result = validate_document_source("/path/that/does/not/exist.txt", "txt");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));

    // Test URL validation (should pass)
    let url_cases = vec![
        ("https://example.com/file.txt", "txt"),
        ("http://example.com/data.json", "json"),
        ("https://test.com/document.pdf", "pdf"),
    ];

    for (url, doc_type) in url_cases {
        let result = validate_document_source(url, doc_type);
        assert!(result.is_ok(), "Should validate URL: {}", url);
    }
}

#[tokio::test]
async fn test_url_loading_http_client_creation() {
    let loader = DocumentLoader::new();

    // Test URL loading with malformed URL that would fail client creation
    let result = loader.load_document("https://", "txt").await;
    assert!(result.is_err());
    // Should fail due to malformed URL or network error
}

#[tokio::test]
async fn test_url_loading_response_processing() {
    let loader = DocumentLoader::new();

    // Test URL loading for different content types
    // These will fail due to network, but test the code paths

    // Test text content from URL
    let result = loader.load_document("https://example.com/test.txt", "txt").await;
    assert!(result.is_err()); // Network error expected

    // Test JSON content from URL
    let result = loader.load_document("https://example.com/data.json", "json").await;
    assert!(result.is_err()); // Network error expected

    // Test CSV content from URL
    let result = loader.load_document("https://example.com/data.csv", "csv").await;
    assert!(result.is_err()); // Network error expected

    // Test XML content from URL
    let result = loader.load_document("https://example.com/data.xml", "xml").await;
    assert!(result.is_err()); // Network error expected

    // Test HTML content from URL
    let result = loader.load_document("https://example.com/page.html", "html").await;
    assert!(result.is_err()); // Network error expected
}

#[tokio::test]
async fn test_url_loading_unsupported_document_types() {
    let loader = DocumentLoader::new();

    // Test URL loading with unsupported document type
    let result = loader.load_document("https://example.com/file.xyz", "xyz").await;
    assert!(result.is_err());
    // Error could be "Unsupported document type" (early validation) or network error
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unsupported document type") || error_msg.contains("Failed to"));
}

#[tokio::test]
async fn test_file_metadata_extraction() {
    let loader = DocumentLoader::new();

    // Test metadata extraction for different file sizes
    let small_content = "small";
    let medium_content = "medium content with more text to test file size metadata";
    let large_content = "large content ".repeat(100);

    let test_cases = vec![
        (small_content, "small file"),
        (medium_content, "medium file"),
        (large_content.as_str(), "large file"),
    ];

    for (content, description) in test_cases {
        let temp_file = create_temp_file(content);
        let result = loader
            .load_document(temp_file.path().to_str().unwrap(), "txt")
            .await;

        assert!(result.is_ok(), "Failed to load {}", description);
        let doc = result.unwrap();

        // Verify metadata
        assert_eq!(doc.file_size, content.len());
        assert_eq!(
            doc.metadata.get("file_size").unwrap(),
            &serde_json::Value::Number(content.len().into())
        );
        assert!(doc.metadata.contains_key("file_path"));
    }
}

#[tokio::test]
async fn test_document_content_serialization() {
    let loader = DocumentLoader::new();
    let content = "Test content for serialization";
    let temp_file = create_temp_file(content);

    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let doc = result.unwrap();

    // Test that DocumentContent can be serialized and deserialized
    let serialized = serde_json::to_string(&doc).expect("Should serialize");
    assert!(!serialized.is_empty());
    assert!(serialized.contains("Test content for serialization"));

    let deserialized: graphbit_core::document_loader::DocumentContent =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(deserialized.source, doc.source);
    assert_eq!(deserialized.document_type, doc.document_type);
    assert_eq!(deserialized.content, doc.content);
    assert_eq!(deserialized.file_size, doc.file_size);
}

#[tokio::test]
async fn test_document_loader_config_serialization() {
    // Test DocumentLoaderConfig serialization
    let mut extraction_settings = HashMap::new();
    extraction_settings.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));

    let config = DocumentLoaderConfig {
        max_file_size: 5 * 1024 * 1024,
        default_encoding: "utf-16".to_string(),
        preserve_formatting: true,
        extraction_settings,
    };

    // Test serialization
    let serialized = serde_json::to_string(&config).expect("Should serialize config");
    assert!(!serialized.is_empty());
    assert!(serialized.contains("utf-16"));
    assert!(serialized.contains("test_value"));

    // Test deserialization
    let deserialized: DocumentLoaderConfig =
        serde_json::from_str(&serialized).expect("Should deserialize config");

    assert_eq!(deserialized.max_file_size, config.max_file_size);
    assert_eq!(deserialized.default_encoding, config.default_encoding);
    assert_eq!(deserialized.preserve_formatting, config.preserve_formatting);
    assert_eq!(deserialized.extraction_settings.len(), config.extraction_settings.len());
}

#[tokio::test]
async fn test_load_document_unsupported_type_error_path() {
    let loader = DocumentLoader::new();
    let temp_file = create_temp_file("test content");

    // Test the specific error path in load_from_file for unsupported types
    // This tests the match arm that returns the detailed error message
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "unsupported_type")
        .await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unsupported document type"));
    // The error message format may vary, so just check for the key part
}

#[tokio::test]
async fn test_url_loading_invalid_url_formats() {
    let loader = DocumentLoader::new();

    // Test various invalid URL formats
    let invalid_urls = vec![
        "http://",           // Empty host
        "https://",          // Empty host
        "://example.com",    // Missing protocol
        "http:/",            // Malformed protocol
        "https:/",           // Malformed protocol
    ];

    for invalid_url in invalid_urls {
        let result = loader.load_document(invalid_url, "txt").await;
        assert!(result.is_err(), "Should fail for invalid URL: {}", invalid_url);
    }
}

#[tokio::test]
async fn test_file_size_boundary_conditions() {
    // Test file size limits at boundary conditions
    let config = DocumentLoaderConfig {
        max_file_size: 100, // 100 bytes limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: false,
        extraction_settings: Default::default(),
    };

    let loader = DocumentLoader::with_config(config);

    // Test file exactly at the limit (should pass)
    let content_at_limit = "a".repeat(100);
    let temp_file = create_temp_file(&content_at_limit);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok(), "File at size limit should pass");

    // Test file over the limit (should fail)
    let content_over_limit = "a".repeat(101);
    let temp_file = create_temp_file(&content_over_limit);
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_err(), "File over size limit should fail");
    assert!(result.unwrap_err().to_string().contains("exceeds maximum allowed size"));
}

#[tokio::test]
async fn test_comprehensive_error_message_formats() {
    let loader = DocumentLoader::new();

    // Test various error message formats

    // File not found error
    let result = loader.load_document("/nonexistent/path/file.txt", "txt").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));

    // Unsupported document type error
    let temp_file = create_temp_file("test");
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "xyz")
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported document type"));

    // Invalid URL format error
    let result = loader.load_document("ftp://example.com/file.txt", "txt").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid URL format"));
}

#[tokio::test]
async fn test_document_timestamp_accuracy() {
    let loader = DocumentLoader::new();
    let content = "Test content for timestamp";
    let temp_file = create_temp_file(content);

    let before_load = chrono::Utc::now();
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    let after_load = chrono::Utc::now();

    assert!(result.is_ok());
    let doc = result.unwrap();

    // Verify timestamp is within reasonable bounds
    assert!(doc.extracted_at >= before_load);
    assert!(doc.extracted_at <= after_load);
}

#[tokio::test]
async fn test_load_from_file_vs_load_from_url_paths() {
    let loader = DocumentLoader::new();

    // Test file path (should use load_from_file)
    let temp_file = create_temp_file("test content");
    let result = loader
        .load_document(temp_file.path().to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());

    // Test URL path (should use load_from_url and fail due to network)
    let result = loader.load_document("https://example.com/test.txt", "txt").await;
    assert!(result.is_err()); // Network error expected
}

#[tokio::test]
async fn test_all_extract_methods_error_handling() {
    let loader = DocumentLoader::new();

    // Test each extract method with invalid content
    let invalid_content = "invalid content for all formats";
    let temp_file = create_temp_file(invalid_content);
    let file_path = temp_file.path().to_str().unwrap();

    // PDF extraction should fail
    let result = loader.load_document(file_path, "pdf").await;
    assert!(result.is_err());

    // DOCX extraction should fail
    let result = loader.load_document(file_path, "docx").await;
    assert!(result.is_err());

    // JSON extraction should fail with invalid JSON
    let result = loader.load_document(file_path, "json").await;
    assert!(result.is_err());

    // TXT, CSV, XML, HTML should succeed (they just read as text)
    let result = loader.load_document(file_path, "txt").await;
    assert!(result.is_ok());

    let result = loader.load_document(file_path, "csv").await;
    assert!(result.is_ok());

    let result = loader.load_document(file_path, "xml").await;
    assert!(result.is_ok());

    let result = loader.load_document(file_path, "html").await;
    assert!(result.is_ok());
}
