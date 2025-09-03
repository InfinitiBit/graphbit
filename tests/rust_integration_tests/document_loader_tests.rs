use graphbit_core::document_loader::{
    detect_document_type, validate_document_source, DocumentLoader,
};
use serde_json::json;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_validate_document_source_file_not_found() {
    let res = validate_document_source("/this/path/does/not/exist.txt", "txt");
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(e.to_string().to_lowercase().contains("file not found"));
    }
}

#[tokio::test]
async fn test_load_text_file_success() {
    // create a temporary text file
    let mut tmp = NamedTempFile::new().expect("create temp file");
    writeln!(tmp, "Hello GraphBit\nThis is a test.").expect("write temp");
    let path = tmp.path().to_str().unwrap().to_string();

    let loader = DocumentLoader::new();
    let content = loader.load_document(&path, "txt").await.expect("load txt");

    assert_eq!(content.document_type, "txt");
    assert!(content.content.contains("Hello GraphBit"));
    assert_eq!(
        content.file_size,
        std::fs::metadata(&path).unwrap().len() as usize
    );
    assert_eq!(content.metadata.get("file_path").unwrap(), &json!(path));
}

#[tokio::test]
async fn test_detect_document_type_and_unsupported_type() {
    // file extension detection
    assert_eq!(
        detect_document_type("/tmp/example.pdf").as_deref(),
        Some("pdf")
    );
    assert_eq!(detect_document_type("/tmp/example.unknown"), None);

    // unsupported document type should error via load_document
    let loader = DocumentLoader::new();
    let res = loader.load_document("/tmp/nonexistent.file", "exe").await;
    assert!(res.is_err());
    if let Err(e) = res {
        assert!(
            e.to_string()
                .to_lowercase()
                .contains("unsupported document type")
                || e.to_string().to_lowercase().contains("file not found")
        );
    }
}
