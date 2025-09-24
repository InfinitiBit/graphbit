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
async fn test_csv_extraction_comprehensive() {
    let loader = DocumentLoader::new();

    // Test CSV with headers and multiple rows
    let mut tmp = NamedTempFile::new().expect("tmp");
    write!(tmp, "product,price,category\nLaptop,999.99,Electronics\nBook,19.99,Education\nChair,149.50,Furniture").unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let result = loader.load_document(&path, "csv").await.expect("csv");
    assert!(result.content.contains("CSV Document Content"));
    assert!(result
        .content
        .contains("Columns (3): product, price, category"));
    assert!(result.content.contains("product: Laptop"));
    assert!(result.content.contains("price: 999.99"));
    assert!(result.content.contains("category: Electronics"));
    assert!(result.content.contains("Total rows processed: 3"));

    // Test CSV with empty fields
    let mut tmp2 = NamedTempFile::new().expect("tmp");
    write!(
        tmp2,
        "name,email,phone\nJohn,john@email.com,\nJane,,555-1234"
    )
    .unwrap();
    let path2 = tmp2.path().to_str().unwrap().to_string();

    let result2 = loader.load_document(&path2, "csv").await.expect("csv");
    assert!(result2.content.contains("name: John"));
    assert!(result2.content.contains("email: john@email.com"));
    assert!(result2.content.contains("name: Jane"));
    assert!(result2.content.contains("phone: 555-1234"));

    // Test malformed CSV (should fallback to raw content)
    let mut tmp3 = NamedTempFile::new().expect("tmp");
    write!(tmp3, "invalid\"csv\"content\nwith\"unmatched\"quotes").unwrap();
    let path3 = tmp3.path().to_str().unwrap().to_string();

    let result3 = loader.load_document(&path3, "csv").await.expect("csv");
    // Should fallback to raw content or handle gracefully
    assert!(!result3.content.is_empty());
}

#[tokio::test]
async fn test_xml_extraction_comprehensive() {
    let loader = DocumentLoader::new();

    // Test XML with nested elements and attributes
    let mut tmp = NamedTempFile::new().expect("tmp");
    write!(
        tmp,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<catalog>
    <book id="1" category="fiction">
        <title>The Great Gatsby</title>
        <author>F. Scott Fitzgerald</author>
        <price currency="USD">12.99</price>
    </book>
    <book id="2" category="science">
        <title>A Brief History of Time</title>
        <author>Stephen Hawking</author>
        <price currency="USD">15.99</price>
    </book>
</catalog>"#
    )
    .unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let result = loader.load_document(&path, "xml").await.expect("xml");
    assert!(result.content.contains("XML Document Content"));
    assert!(result.content.contains("Element: catalog"));
    assert!(result.content.contains("Element: book"));
    assert!(result.content.contains("Text: The Great Gatsby"));
    assert!(result.content.contains("Text: F. Scott Fitzgerald"));

    // Test XML with CDATA
    let mut tmp2 = NamedTempFile::new().expect("tmp");
    write!(
        tmp2,
        r#"<root><description><![CDATA[This is <b>bold</b> text]]></description></root>"#
    )
    .unwrap();
    let path2 = tmp2.path().to_str().unwrap().to_string();

    let result2 = loader.load_document(&path2, "xml").await.expect("xml");
    assert!(result2.content.contains("This is <b>bold</b> text"));

    // Test malformed XML (should fallback to raw content)
    let mut tmp3 = NamedTempFile::new().expect("tmp");
    write!(tmp3, "<root><unclosed>content</root>").unwrap();
    let path3 = tmp3.path().to_str().unwrap().to_string();

    let result3 = loader.load_document(&path3, "xml").await.expect("xml");
    // Should fallback to raw content
    assert!(!result3.content.is_empty());
}

#[tokio::test]
async fn test_html_extraction_comprehensive() {
    let loader = DocumentLoader::new();

    // Test HTML with various elements
    let mut tmp = NamedTempFile::new().expect("tmp");
    write!(
        tmp,
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Sample Page</title>
    <meta name="description" content="This is a sample page for testing">
</head>
<body>
    <h1>Main Title</h1>
    <h2>Subtitle</h2>
    <p>This is a paragraph with some text.</p>
    <p>Another paragraph with <a href="https://example.com">a link</a>.</p>

    <ul>
        <li>First item</li>
        <li>Second item</li>
    </ul>

    <ol>
        <li>Numbered first</li>
        <li>Numbered second</li>
    </ol>

    <table>
        <tr>
            <th>Name</th>
            <th>Age</th>
        </tr>
        <tr>
            <td>John</td>
            <td>25</td>
        </tr>
        <tr>
            <td>Jane</td>
            <td>30</td>
        </tr>
    </table>
</body>
</html>"#
    )
    .unwrap();
    let path = tmp.path().to_str().unwrap().to_string();

    let result = loader.load_document(&path, "html").await.expect("html");
    assert!(result.content.contains("HTML Document Content"));
    assert!(result.content.contains("Title: Sample Page"));
    assert!(result
        .content
        .contains("Description: This is a sample page for testing"));
    assert!(result.content.contains("H1: Main Title"));
    assert!(result.content.contains("H2: Subtitle"));
    assert!(result
        .content
        .contains("This is a paragraph with some text"));
    assert!(result.content.contains("First item"));
    assert!(result.content.contains("https://example.com"));
    assert!(result.content.contains("Numbered first"));
    assert!(result.content.contains("a link"));
    assert!(result.content.contains("Name"));
    assert!(result.content.contains("John"));
    assert!(result.content.contains("25"));

    // Test minimal HTML
    let mut tmp2 = NamedTempFile::new().expect("tmp");
    write!(tmp2, "<html><body><p>Simple content</p></body></html>").unwrap();
    let path2 = tmp2.path().to_str().unwrap().to_string();

    let result2 = loader.load_document(&path2, "html").await.expect("html");
    assert!(result2.content.contains("Simple content"));

    // Test malformed HTML (should still parse gracefully)
    let mut tmp3 = NamedTempFile::new().expect("tmp");
    write!(
        tmp3,
        "<html><body><p>Unclosed paragraph<div>Content</body></html>"
    )
    .unwrap();
    let path3 = tmp3.path().to_str().unwrap().to_string();

    let result3 = loader.load_document(&path3, "html").await.expect("html");
    assert!(result3.content.contains("HTML Document Content"));
    assert!(result3.content.contains("Unclosed paragraph"));
}

#[tokio::test]
async fn test_extraction_fallback_behavior() {
    let loader = DocumentLoader::new();

    // Test empty CSV file
    let mut tmp_csv = NamedTempFile::new().expect("tmp");
    write!(tmp_csv, "").unwrap();
    let path_csv = tmp_csv.path().to_str().unwrap().to_string();

    let result_csv = loader.load_document(&path_csv, "csv").await.expect("csv");
    assert!(!result_csv.content.is_empty()); // Should handle empty files gracefully

    // Test empty XML file
    let mut tmp_xml = NamedTempFile::new().expect("tmp");
    write!(tmp_xml, "").unwrap();
    let path_xml = tmp_xml.path().to_str().unwrap().to_string();

    let result_xml = loader.load_document(&path_xml, "xml").await.expect("xml");
    assert!(!result_xml.content.is_empty()); // Should handle empty files gracefully

    // Test empty HTML file
    let mut tmp_html = NamedTempFile::new().expect("tmp");
    write!(tmp_html, "").unwrap();
    let path_html = tmp_html.path().to_str().unwrap().to_string();

    let result_html = loader
        .load_document(&path_html, "html")
        .await
        .expect("html");
    assert!(!result_html.content.is_empty()); // Should handle empty files gracefully
}

#[tokio::test]
async fn test_load_csv_xml_html_enhanced_parsing() {
    let mut tmp_csv = NamedTempFile::new().expect("tmp");
    write!(tmp_csv, "name,age,city\nJohn,25,NYC\nJane,30,LA").unwrap();
    let path_csv = tmp_csv.path().to_str().unwrap().to_string();

    let loader = DocumentLoader::new();
    let csv = loader.load_document(&path_csv, "csv").await.expect("csv");
    // Should contain structured CSV content, not raw CSV
    assert!(csv.content.contains("CSV Document Content"));
    assert!(csv.content.contains("Columns (3): name, age, city"));
    assert!(csv.content.contains("name: John"));
    assert!(csv.content.contains("age: 25"));
    assert!(csv.content.contains("city: NYC"));

    let mut tmp_xml = NamedTempFile::new().expect("tmp");
    write!(
        tmp_xml,
        "<root><person name=\"John\"><age>25</age><city>NYC</city></person></root>"
    )
    .unwrap();
    let path_xml = tmp_xml.path().to_str().unwrap().to_string();
    let xml = loader.load_document(&path_xml, "xml").await.expect("xml");
    // Should contain structured XML content, not raw XML
    assert!(xml.content.contains("XML Document Content"));
    assert!(xml.content.contains("Element: root"));
    assert!(xml.content.contains("Element: person"));

    let mut tmp_html = NamedTempFile::new().expect("tmp");
    write!(tmp_html, "<html><head><title>Test Page</title></head><body><h1>Welcome</h1><p>Hello World</p></body></html>").unwrap();
    let path_html = tmp_html.path().to_str().unwrap().to_string();
    let html = loader
        .load_document(&path_html, "html")
        .await
        .expect("html");
    // Should contain structured HTML content, not raw HTML
    assert!(html.content.contains("HTML Document Content"));
    assert!(html.content.contains("Title: Test Page"));
    assert!(html.content.contains("H1: Welcome"));
    assert!(html.content.contains("Hello World"));
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
