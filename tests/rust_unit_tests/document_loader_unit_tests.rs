use graphbit_core::document_loader::{DocumentLoader, DocumentLoaderConfig};
use std::io::Write;
use tempfile::NamedTempFile;

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
