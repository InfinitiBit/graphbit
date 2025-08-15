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
    assert!(doc.content.contains("name,age,city"));
    assert!(doc.content.contains("John,30,New York"));
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
    assert!(doc.content.contains("<root>"));
    assert!(doc.content.contains("Test Item"));
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
    assert!(doc.content.contains("<html>"));
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
