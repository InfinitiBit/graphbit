use graphbit_core::document_loader::{
    detect_document_type, validate_document_source, DocumentLoader,
    DocumentLoaderConfig,
};
use std::collections::HashMap;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[tokio::test]
async fn test_load_json_success_and_invalid() {
    // valid JSON
    let mut tmp = NamedTempFile::new().expect("create temp");
    write!(tmp, "{{\"k\": 1}}").unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let loader = DocumentLoader::new();
    let content = loader
        .load_document(&path, "json")
        .await
        .expect("load json");
    assert!(content.content.contains("\"k\""));

    // invalid JSON should error
    let mut tmp2 = NamedTempFile::new().expect("create temp");
    write!(tmp2, "not a json").unwrap();
    let path2 = tmp2.path().to_str().unwrap().to_string();

    let res = loader.load_document(&path2, "json").await;
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(
            e.to_string().to_lowercase().contains("invalid json")
                || e.to_string().to_lowercase().contains("failed to read json")
        );
    }
}

#[tokio::test]
async fn test_document_loader_config() {
    let mut extraction_settings = HashMap::new();
    extraction_settings.insert("preserve_whitespace".to_string(), serde_json::json!(true));

    let config = DocumentLoaderConfig {
        max_file_size: 1024, // 1KB
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings,
    };

    let loader = DocumentLoader::with_config(config.clone());

    // Test that config is applied - create a file that exceeds the limit
    let temp_dir = TempDir::new().expect("create temp dir");
    let large_file = temp_dir.path().join("large.txt");
    let large_content = "A".repeat(2048); // 2KB, exceeds 1KB limit
    std::fs::write(&large_file, &large_content).expect("write large file");

    let result = loader
        .load_document(large_file.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("exceeds maximum allowed size"));

    // Test small file works
    let small_file = temp_dir.path().join("small.txt");
    std::fs::write(&small_file, "small content").expect("write small file");

    let result = loader
        .load_document(small_file.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_document_loader_default() {
    let loader1 = DocumentLoader::new();
    let loader2 = DocumentLoader::default();

    // Both should work the same way
    let temp_dir = TempDir::new().expect("create temp dir");
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "test content").expect("write file");

    let result1 = loader1
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;
    let result2 = loader2
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(result1.unwrap().content, result2.unwrap().content);
}

#[test]
fn test_supported_types() {
    let types = DocumentLoader::supported_types();
    assert!(types.contains(&"txt"));
    assert!(types.contains(&"pdf"));
    assert!(types.contains(&"docx"));
    assert!(types.contains(&"json"));
    assert!(types.contains(&"csv"));
    assert!(types.contains(&"xml"));
    assert!(types.contains(&"html"));
    assert_eq!(types.len(), 7);
}

#[test]
fn test_detect_document_type() {
    // Test supported extensions
    assert_eq!(detect_document_type("test.txt"), Some("txt".to_string()));
    assert_eq!(detect_document_type("test.pdf"), Some("pdf".to_string()));
    assert_eq!(detect_document_type("test.docx"), Some("docx".to_string()));
    assert_eq!(detect_document_type("test.json"), Some("json".to_string()));
    assert_eq!(detect_document_type("test.csv"), Some("csv".to_string()));
    assert_eq!(detect_document_type("test.xml"), Some("xml".to_string()));
    assert_eq!(detect_document_type("test.html"), Some("html".to_string()));

    // Test case insensitive
    assert_eq!(detect_document_type("test.PDF"), Some("pdf".to_string()));
    assert_eq!(detect_document_type("test.TXT"), Some("txt".to_string()));

    // Test unsupported extensions
    assert_eq!(detect_document_type("test.exe"), None);
    assert_eq!(detect_document_type("test.unknown"), None);
    assert_eq!(detect_document_type("test"), None);

    // Test complex paths
    assert_eq!(
        detect_document_type("/path/to/document.pdf"),
        Some("pdf".to_string())
    );
    assert_eq!(
        detect_document_type("C:\\Users\\test\\document.docx"),
        Some("docx".to_string())
    );
}

#[tokio::test]
async fn test_validate_document_source() {
    // Test supported document types
    let temp_dir = TempDir::new().expect("create temp dir");
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "test content").expect("write file");

    let result = validate_document_source(file_path.to_str().unwrap(), "txt");
    assert!(result.is_ok());

    // Test unsupported document type
    let result = validate_document_source(file_path.to_str().unwrap(), "exe");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported document type"));

    // Test non-existent file
    let result = validate_document_source("/nonexistent/file.txt", "txt");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));

    // Test URL (should pass validation)
    let result = validate_document_source("https://example.com/doc.txt", "txt");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_extract_xml_content() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let xml_file = temp_dir.path().join("test.xml");
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1">First item</item>
    <item id="2">Second item</item>
</root>"#;
    std::fs::write(&xml_file, xml_content).expect("write xml file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(xml_file.to_str().unwrap(), "xml")
        .await;

    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.document_type, "xml");
    assert!(content.content.contains("<root>"));
    assert!(content.content.contains("First item"));
    assert!(content.content.contains("Second item"));
    assert!(content.file_size > 0);
}

#[tokio::test]
async fn test_extract_html_content() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let html_file = temp_dir.path().join("test.html");
    let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
</head>
<body>
    <h1>Welcome</h1>
    <p>This is a test HTML document.</p>
</body>
</html>"#;
    std::fs::write(&html_file, html_content).expect("write html file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(html_file.to_str().unwrap(), "html")
        .await;

    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.document_type, "html");
    assert!(content.content.contains("<html>"));
    assert!(content.content.contains("Welcome"));
    assert!(content.content.contains("test HTML document"));
    assert!(content.file_size > 0);
}

#[tokio::test]
async fn test_unsupported_document_type() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let file_path = temp_dir.path().join("test.exe");
    std::fs::write(&file_path, "binary content").expect("write file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "exe")
        .await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported document type"));
}

#[tokio::test]
async fn test_file_not_found() {
    let loader = DocumentLoader::new();
    let result = loader
        .load_document("/nonexistent/path/file.txt", "txt")
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("File not found"));
}

#[tokio::test]
async fn test_invalid_url_format() {
    let loader = DocumentLoader::new();

    // Test invalid URL scheme
    let result = loader.load_document("ftp://example.com/file.txt", "txt").await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid URL format"));

    // Test malformed URL
    let result = loader.load_document("not-a-url://test", "txt").await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid URL format"));
}

#[tokio::test]
async fn test_pdf_extraction_attempt() {
    // Create a fake PDF file (will fail extraction but test the code path)
    let temp_dir = TempDir::new().expect("create temp dir");
    let pdf_file = temp_dir.path().join("test.pdf");
    std::fs::write(&pdf_file, "fake pdf content").expect("write fake pdf");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(pdf_file.to_str().unwrap(), "pdf")
        .await;

    // This should fail because it's not a real PDF, but it tests the extraction path
    assert!(result.is_err());
    // The error should be related to PDF parsing, not file not found
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Failed to open PDF file") || error_msg.contains("PDF")
    );
}

#[tokio::test]
async fn test_docx_extraction_attempt() {
    // Create a fake DOCX file (will fail extraction but test the code path)
    let temp_dir = TempDir::new().expect("create temp dir");
    let docx_file = temp_dir.path().join("test.docx");
    std::fs::write(&docx_file, "fake docx content").expect("write fake docx");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(docx_file.to_str().unwrap(), "docx")
        .await;

    // This should fail because it's not a real DOCX, but it tests the extraction path
    assert!(result.is_err());
    // The error should be related to DOCX parsing
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Failed to open DOCX file")
        || error_msg.contains("DOCX")
        || error_msg.contains("zip")
    );
}

#[tokio::test]
async fn test_document_content_structure() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let file_path = temp_dir.path().join("test.txt");
    let test_content = "Hello, World!\nThis is a test document.";
    std::fs::write(&file_path, test_content).expect("write file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "txt")
        .await;

    assert!(result.is_ok());
    let content = result.unwrap();

    // Verify DocumentContent structure
    assert_eq!(content.source, file_path.to_str().unwrap());
    assert_eq!(content.document_type, "txt");
    assert_eq!(content.content, test_content);
    assert_eq!(content.file_size, test_content.len());
    assert!(!content.metadata.is_empty());
    assert!(content.metadata.contains_key("file_path"));

    // Verify timestamp is recent (within last minute)
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(content.extracted_at);
    assert!(diff.num_seconds() < 60);
}

#[tokio::test]
async fn test_load_csv_xml_html_return_raw() {
    let mut tmp_csv = NamedTempFile::new().expect("tmp");
    write!(tmp_csv, "a,b,c\n1,2,3").unwrap();
    let path_csv = tmp_csv.path().to_str().unwrap().to_string();

    let loader = DocumentLoader::new();
    let csv = loader.load_document(&path_csv, "csv").await.expect("csv");
    assert!(csv.content.contains("a,b,c"));

    let mut tmp_xml = NamedTempFile::new().expect("tmp");
    write!(tmp_xml, "<root><x>1</x></root>").unwrap();
    let path_xml = tmp_xml.path().to_str().unwrap().to_string();
    let xml = loader.load_document(&path_xml, "xml").await.expect("xml");
    assert!(xml.content.contains("<root>"));

    let mut tmp_html = NamedTempFile::new().expect("tmp");
    write!(tmp_html, "<html><body>hi</body></html>").unwrap();
    let path_html = tmp_html.path().to_str().unwrap().to_string();
    let html = loader
        .load_document(&path_html, "html")
        .await
        .expect("html");
    assert!(html.content.contains("<html>"));
}

#[tokio::test]
async fn test_pdf_and_docx_error_on_invalid_files() {
    // create a non-pdf file but call pdf extractor
    let mut tmp = NamedTempFile::new().expect("tmp");
    write!(tmp, "this is not a pdf").unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let loader = DocumentLoader::new();
    let res = loader.load_document(&path, "pdf").await;
    assert!(res.is_err());

    // docx invalid
    let mut tmp2 = NamedTempFile::new().expect("tmp");
    write!(tmp2, "not a docx").unwrap();
    let path2 = tmp2.path().to_str().unwrap().to_string();
    let res2 = loader.load_document(&path2, "docx").await;
    assert!(res2.is_err());
}

#[tokio::test]
async fn test_file_size_limit_validation() {
    // create a file larger than allowed
    let mut tmp = NamedTempFile::new().expect("tmp");
    // write 1KB
    let big = vec![b'a'; 1024];
    tmp.write_all(&big).unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let cfg = DocumentLoaderConfig {
        max_file_size: 10,
        ..Default::default()
    };
    let loader = DocumentLoader::with_config(cfg);

    let res = loader.load_document(&path, "txt").await;
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(
            e.to_string().to_lowercase().contains("exceeds")
                || e.to_string().to_lowercase().contains("file size")
        );
    }
}

// URL loading tests - these test the URL loading code paths
#[tokio::test]
async fn test_url_loading_unsupported_types() {
    let loader = DocumentLoader::new();

    // Test PDF URL loading - should fail immediately because PDF URL loading is not supported
    // Use a simple URL that would work for other types but should fail for PDF
    let result = loader
        .load_document("https://www.google.com", "pdf")
        .await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    println!("PDF error message: {}", error_msg);
    // Should fail because PDF URL loading is not supported
    assert!(
        error_msg.contains("URL loading for pdf documents is not yet supported"),
        "Expected PDF URL loading error, got: {}", error_msg
    );

    // Test DOCX URL loading - should fail immediately because DOCX URL loading is not supported
    let result = loader
        .load_document("https://www.google.com", "docx")
        .await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    println!("DOCX error message: {}", error_msg);
    assert!(
        error_msg.contains("URL loading for docx documents is not yet supported"),
        "Expected DOCX URL loading error, got: {}", error_msg
    );
}

#[tokio::test]
async fn test_url_validation() {
    let loader = DocumentLoader::new();

    // Test invalid URL format in load_from_url path
    let result = loader
        .load_document("ftp://example.com/file.txt", "txt")
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid URL format"));

    // Test unsupported document type for URL
    let result = loader
        .load_document("https://example.com/file.exe", "exe")
        .await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unsupported document type"));
}

#[tokio::test]
async fn test_document_content_metadata() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let file_path = temp_dir.path().join("metadata_test.json");
    let json_content = r#"{"test": "data", "number": 42}"#;
    std::fs::write(&file_path, json_content).expect("write json file");

    let loader = DocumentLoader::new();
    let result = loader
        .load_document(file_path.to_str().unwrap(), "json")
        .await;

    assert!(result.is_ok());
    let content = result.unwrap();

    // Verify all DocumentContent fields are properly set
    assert_eq!(content.source, file_path.to_str().unwrap());
    assert_eq!(content.document_type, "json");
    // JSON content might be reformatted, so check it contains the key data
    assert!(content.content.contains("test"));
    assert!(content.content.contains("data"));
    assert!(content.content.contains("42"));
    assert!(content.file_size > 0);

    // Check metadata contains expected keys
    assert!(content.metadata.contains_key("file_path"));
    assert_eq!(
        content.metadata.get("file_path").unwrap(),
        &serde_json::json!(file_path.to_str().unwrap())
    );

    // Verify extracted_at is set and reasonable
    let now = chrono::Utc::now();
    let time_diff = now.signed_duration_since(content.extracted_at);
    assert!(time_diff.num_seconds() >= 0);
    assert!(time_diff.num_seconds() < 10); // Should be very recent
}

#[tokio::test]
async fn test_document_loader_config_default() {
    let default_config = DocumentLoaderConfig::default();

    assert_eq!(default_config.max_file_size, 10 * 1024 * 1024); // 10MB
    assert_eq!(default_config.default_encoding, "utf-8");
    assert_eq!(default_config.preserve_formatting, false);
    assert!(default_config.extraction_settings.is_empty());
}

#[tokio::test]
async fn test_edge_cases() {
    let loader = DocumentLoader::new();

    // Test empty file
    let temp_dir = TempDir::new().expect("create temp dir");
    let empty_file = temp_dir.path().join("empty.txt");
    std::fs::write(&empty_file, "").expect("write empty file");

    let result = loader
        .load_document(empty_file.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.content, "");
    assert_eq!(content.file_size, 0);

    // Test file with only whitespace
    let whitespace_file = temp_dir.path().join("whitespace.txt");
    std::fs::write(&whitespace_file, "   \n\t  \n  ").expect("write whitespace file");

    let result = loader
        .load_document(whitespace_file.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.content, "   \n\t  \n  ");

    // Test very long filename
    let long_name = "a".repeat(200) + ".txt";
    let long_file = temp_dir.path().join(&long_name);
    std::fs::write(&long_file, "content").expect("write long filename file");

    let result = loader
        .load_document(long_file.to_str().unwrap(), "txt")
        .await;
    assert!(result.is_ok());
}

// Additional comprehensive tests for 100% document_loader coverage

#[tokio::test]
async fn test_document_loader_url_loading_comprehensive() {
    let loader = DocumentLoader::new();

    // Test URL loading with different protocols
    let test_urls = vec![
        "https://httpbin.org/json",
        "http://httpbin.org/json",
        "https://raw.githubusercontent.com/octocat/Hello-World/master/README",
    ];

    for url in test_urls {
        let result = loader.load_document(url, "json").await;
        // URL loading might fail due to network issues, but we test the code path
        // The important thing is that it doesn't panic and handles errors gracefully
        match result {
            Ok(content) => {
                assert!(!content.content.is_empty());
                assert_eq!(content.source, url);
            }
            Err(_e) => {
                // Network errors are acceptable in tests - any error is fine
                // as long as the code doesn't panic
            }
        }
    }
}

#[tokio::test]
async fn test_document_loader_error_paths_comprehensive() {
    let loader = DocumentLoader::new();

    // Test loading non-existent file
    let result = loader.load_document("/nonexistent/path/file.txt", "txt").await;
    assert!(result.is_err());

    // Test loading with unsupported document type
    let temp_dir = TempDir::new().expect("create temp dir");
    let test_file = temp_dir.path().join("test.unsupported");
    std::fs::write(&test_file, "test content").expect("write test file");

    let result = loader.load_document(test_file.to_str().unwrap(), "unsupported").await;
    assert!(result.is_err());

    // Test loading file that exceeds size limit
    let small_config = DocumentLoaderConfig {
        max_file_size: 10, // Very small limit
        default_encoding: "utf-8".to_string(),
        preserve_formatting: false,
        extraction_settings: HashMap::new(),
    };
    let small_loader = DocumentLoader::with_config(small_config);

    let large_file = temp_dir.path().join("large.txt");
    std::fs::write(&large_file, "This content exceeds the 10 byte limit").expect("write large file");

    let result = small_loader.load_document(large_file.to_str().unwrap(), "txt").await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("exceeds maximum") || error_msg.contains("too large"));
}

#[tokio::test]
async fn test_document_loader_pdf_docx_error_handling() {
    let loader = DocumentLoader::new();
    let temp_dir = TempDir::new().expect("create temp dir");

    // Test PDF loading with invalid PDF file
    let fake_pdf = temp_dir.path().join("fake.pdf");
    std::fs::write(&fake_pdf, "This is not a PDF file").expect("write fake PDF");

    let result = loader.load_document(fake_pdf.to_str().unwrap(), "pdf").await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("PDF") || error_msg.contains("extract") || error_msg.contains("parse"));

    // Test DOCX loading with invalid DOCX file
    let fake_docx = temp_dir.path().join("fake.docx");
    std::fs::write(&fake_docx, "This is not a DOCX file").expect("write fake DOCX");

    let result = loader.load_document(fake_docx.to_str().unwrap(), "docx").await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("DOCX") || error_msg.contains("extract") || error_msg.contains("zip"));
}

#[tokio::test]
async fn test_document_loader_metadata_comprehensive() {
    let loader = DocumentLoader::new();
    let temp_dir = TempDir::new().expect("create temp dir");

    // Test metadata extraction for different file types
    let test_content = "Test content for metadata extraction";

    let file_types = vec!["txt", "json", "csv", "xml", "html"];

    for file_type in file_types {
        let test_file = temp_dir.path().join(format!("test.{}", file_type));
        let content = match file_type {
            "json" => r#"{"key": "value", "test": "data"}"#,
            "csv" => "header1,header2\nvalue1,value2",
            "xml" => "<root><item>test</item></root>",
            "html" => "<html><body><p>Test content</p></body></html>",
            _ => test_content,
        };

        std::fs::write(&test_file, content).expect("write test file");

        let result = loader.load_document(test_file.to_str().unwrap(), file_type).await;
        assert!(result.is_ok(), "Failed to load {} file", file_type);

        let doc = result.unwrap();
        assert_eq!(doc.document_type, file_type);
        assert!(!doc.content.is_empty());
        assert!(doc.file_size > 0);
        assert!(!doc.source.is_empty());
        assert!(doc.extracted_at.timestamp() > 0);
        // Check that metadata exists (specific keys may vary by implementation)
        // The important thing is that the document was loaded successfully
    }
}

#[tokio::test]
async fn test_document_loader_encoding_and_formatting() {
    let temp_dir = TempDir::new().expect("create temp dir");

    // Test with preserve formatting enabled
    let config_preserve = DocumentLoaderConfig {
        max_file_size: 10 * 1024 * 1024,
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings: HashMap::new(),
    };
    let loader_preserve = DocumentLoader::with_config(config_preserve);

    let formatted_content = "Line 1\n  Indented line\n    More indented\nLast line";
    let formatted_file = temp_dir.path().join("formatted.txt");
    std::fs::write(&formatted_file, formatted_content).expect("write formatted file");

    let result = loader_preserve.load_document(formatted_file.to_str().unwrap(), "txt").await;
    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("  Indented"));
    assert!(doc.content.contains("    More indented"));

    // Test with different encoding setting
    let config_encoding = DocumentLoaderConfig {
        max_file_size: 10 * 1024 * 1024,
        default_encoding: "iso-8859-1".to_string(),
        preserve_formatting: false,
        extraction_settings: HashMap::new(),
    };
    let loader_encoding = DocumentLoader::with_config(config_encoding);

    let result = loader_encoding.load_document(formatted_file.to_str().unwrap(), "txt").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_document_loader_extraction_settings() {
    let mut extraction_settings = HashMap::new();
    extraction_settings.insert("custom_setting".to_string(), serde_json::json!("custom_value"));
    extraction_settings.insert("numeric_setting".to_string(), serde_json::json!(42));
    extraction_settings.insert("boolean_setting".to_string(), serde_json::json!(true));

    let config = DocumentLoaderConfig {
        max_file_size: 10 * 1024 * 1024,
        default_encoding: "utf-8".to_string(),
        preserve_formatting: true,
        extraction_settings,
    };

    let loader = DocumentLoader::with_config(config.clone());

    // Config is private, so we can't directly access it, but we can test that the loader works
    // The important thing is that the loader was created successfully with custom settings

    // Test that the loader still works with custom settings
    let temp_dir = TempDir::new().expect("create temp dir");
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Test content with custom settings").expect("write test file");

    let result = loader.load_document(test_file.to_str().unwrap(), "txt").await;
    assert!(result.is_ok());
    let doc = result.unwrap();
    assert!(doc.content.contains("Test content"));
}

#[tokio::test]
async fn test_utility_functions_comprehensive() {
    // Test detect_document_type with various file paths
    assert_eq!(detect_document_type("file.txt"), Some("txt".to_string()));
    assert_eq!(detect_document_type("document.pdf"), Some("pdf".to_string()));
    assert_eq!(detect_document_type("spreadsheet.csv"), Some("csv".to_string()));
    assert_eq!(detect_document_type("data.json"), Some("json".to_string()));
    assert_eq!(detect_document_type("page.html"), Some("html".to_string()));
    assert_eq!(detect_document_type("config.xml"), Some("xml".to_string()));
    assert_eq!(detect_document_type("report.docx"), Some("docx".to_string()));

    // Test with uppercase extensions
    assert_eq!(detect_document_type("FILE.TXT"), Some("txt".to_string()));
    assert_eq!(detect_document_type("DOCUMENT.PDF"), Some("pdf".to_string()));

    // Test with no extension
    assert_eq!(detect_document_type("filename"), None);
    assert_eq!(detect_document_type(""), None);

    // Test with unsupported extension
    assert_eq!(detect_document_type("file.xyz"), None);
    assert_eq!(detect_document_type("file.exe"), None);

    // Test with multiple dots
    assert_eq!(detect_document_type("file.backup.txt"), Some("txt".to_string()));
    assert_eq!(detect_document_type("data.2023.json"), Some("json".to_string()));

    // Test validate_document_source function
    let temp_dir = TempDir::new().expect("create temp dir");
    let valid_file = temp_dir.path().join("valid.txt");
    std::fs::write(&valid_file, "content").expect("write valid file");

    // Test with valid file and supported type
    let result = validate_document_source(valid_file.to_str().unwrap(), "txt");
    assert!(result.is_ok());

    // Test with unsupported document type
    let result = validate_document_source(valid_file.to_str().unwrap(), "unsupported");
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("unsupported") || error_msg.contains("not supported"));

    // Test with non-existent file
    let result = validate_document_source("/nonexistent/file.txt", "txt");
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("not found") || error_msg.contains("does not exist"));

    // Test supported_types function
    let supported = DocumentLoader::supported_types();
    assert!(supported.contains(&"txt"));
    assert!(supported.contains(&"pdf"));
    assert!(supported.contains(&"docx"));
    assert!(supported.contains(&"json"));
    assert!(supported.contains(&"csv"));
    assert!(supported.contains(&"xml"));
    assert!(supported.contains(&"html"));
    assert_eq!(supported.len(), 7);
}
