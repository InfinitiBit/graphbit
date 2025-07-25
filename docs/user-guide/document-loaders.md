# Document Loaders

GraphBit provides robust document loading capabilities for extracting and processing content from a variety of file formats. This guide covers configuration and usage for all supported document loaders.

## Overview

GraphBit's document loader system supports:
- **Multiple Formats** – TXT, PDF, DOCX, JSON, CSV, XML, and HTML
- **Unified Interface** – Consistent API for all document types
- **Metadata Extraction** – File size, type, and extraction timestamp
- **Customizable Settings** – File size limits, encoding, and formatting options

## Supported Document Types

| Type   | Description                        |
|--------|------------------------------------|
| txt    | Plain text files                   |
| pdf    | PDF documents                      |
| docx   | Microsoft Word documents           |
| json   | JSON structured data files         |
| csv    | Comma-separated values (spreadsheets) |
| xml    | XML structured data files          |
| html   | HTML web pages                     |

## Configuration

### Basic Document Loader Configuration

```python
import graphbit

# Initialize GraphBit
graphbit.init()

# Create a document loader with default configuration
loader = graphbit.DocumentLoader()

# Custom configuration (optional)
from graphbit import DocumentLoaderConfig
config = DocumentLoaderConfig(
    max_file_size=5 * 1024 * 1024,  # 5MB
    default_encoding="utf-8",
    preserve_formatting=True,
    extraction_settings={}
)
loader = graphbit.DocumentLoader(config)
```

## Basic Usage

### Loading a Document

```python
# Load a TXT file
content = loader.load_document("/path/to/file.txt", document_type="txt")
print(f"Content: {content.content[:100]}...")
print(f"Type: {content.document_type}")
print(f"File size: {content.file_size} bytes")

# Load a PDF file
content = loader.load_document("/path/to/file.pdf", document_type="pdf")
print(f"Extracted text: {content.content[:200]}...")
```

### Example: Loading Different Document Types

```python
# Supported types: txt, pdf, docx, json, csv, xml, html
files = [
    ("/docs/sample.txt", "txt"),
    ("/docs/report.pdf", "pdf"),
    ("/docs/contract.docx", "docx"),
    ("/docs/data.json", "json"),
    ("/docs/table.csv", "csv"),
    ("/docs/feed.xml", "xml"),
    ("/docs/page.html", "html"),
]

for path, doc_type in files:
    try:
        doc = loader.load_document(path, document_type=doc_type)
        print(f"Loaded {doc_type.upper()} from {path}: {doc.content[:80]}...")
    except Exception as e:
        print(f"Failed to load {doc_type} from {path}: {e}")
```

## Document Loader Node in Workflows

You can use document loader nodes in GraphBit workflows:

```python
loader_node = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Loader",
    description="Loads PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)
```

## Notes
- Only local file paths are currently supported (URL loading is not yet implemented).
- File size and type validation is performed automatically.
- Extraction quality may vary by document type and content structure.

## What's Next

- Learn about [Validation](validation.md) for input and document validation
- Explore [Workflow Builder](workflow-builder.md) for building custom pipelines
- See [Embeddings](embeddings.md) for semantic search and vectorization
- Check [LLM Providers](llm-providers.md) for language model integration
- Review [Monitoring](monitoring.md) for workflow observability 