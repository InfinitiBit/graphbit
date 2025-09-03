# Document Loader

GraphBit extracts content from multiple document formats for AI workflow processing.

## Components

- **DocumentLoader** - Main loading class
- **DocumentLoaderConfig** - Configuration options  
- **DocumentContent** - Extracted content and metadata

---

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

---

## Quick Start

```python
from graphbit import DocumentLoader

loader = DocumentLoader()

# Load document
content = loader.load_document("document.pdf", "pdf")
print(f"Extracted {content.content_length()} characters")
```

---

## Auto-Detection

```python
from graphbit import DocumentLoader

def load_document(file_path):
    doc_type = DocumentLoader.detect_document_type(file_path)
    if not doc_type:
        return None
    
    loader = DocumentLoader()
    return loader.load_document(file_path, doc_type)
```

---

## Configuration

### Basic Setup
```python
from graphbit import DocumentLoaderConfig, DocumentLoader

config = DocumentLoaderConfig(
    max_file_size=50_000_000,    # 50MB limit
    default_encoding="utf-8",    # Text encoding
    preserve_formatting=True     # Keep formatting
)

loader = DocumentLoader(config)
```

### Advanced Settings
```python
from graphbit import DocumentLoaderConfig

config = DocumentLoaderConfig()
config.extraction_settings = {
    "pdf_parser": "advanced",
    "ocr_enabled": True,
    "table_detection": True
}
```

### Properties

#### `max_file_size`
Get or set the maximum file size limit.

```python
size = config.max_file_size

config.set_max_file_size = 100_000_000  # 100MB
```

#### `default_encoding`
Get or set the default text encoding.

```python
encoding = config.default_encoding

config.set_default_encoding("utf-8")
```

#### `preserve_formatting`
Get or set the formatting preservation flag.

```python
preserve = config.preserve_formatting

config.set_preserve_formatting(True)
```

#### `extraction_settings`
Get or set extraction settings as a dictionary.

```python
settings = config.extraction_settings

settings = {"pdf_parser": "advanced", "ocr_enabled": True}
config.set_extraction_settings(settings)
```

---

## Document Types

### PDF Processing
```python
from graphbit import DocumentLoaderConfig, DocumentLoader

config = DocumentLoaderConfig(preserve_formatting=True)
config.extraction_settings = {
    "ocr_enabled": True,
    "table_detection": True
}
loader = DocumentLoader(config)
content = loader.load_document("report.pdf", "pdf")

# Access metadata
metadata = content.metadata
print(f"Pages: {metadata.get('pages')}")
```

### Text Files
```python
from graphbit import DocumentLoaderConfig, DocumentLoader

config = DocumentLoaderConfig(default_encoding="utf-8")
loader = DocumentLoader(config)
content = loader.load_document("notes.txt", "txt")
```

### Structured Data
```python
from graphbit import DocumentLoader

# JSON, CSV, XML automatically parsed as text
loader = DocumentLoader()
json_content = loader.load_document("data.json", "json")
csv_content = loader.load_document("data.csv", "csv")
```

---

## Static Methods

### `DocumentLoader.supported_types()`
Get list of supported document types.

```python
types = DocumentLoader.supported_types()
print(f"Supported formats: {types}")
# Output: ['txt', 'pdf', 'docx', 'json', 'csv', 'xml', 'html']
```

### `DocumentLoader.detect_document_type(file_path)`
Detect document type from file extension.

```python
doc_type = DocumentLoader.detect_document_type("report.pdf")
print(f"Detected type: {doc_type}")  # "pdf"

# Returns None if type cannot be detected
unknown_type = DocumentLoader.detect_document_type("file.unknown")
print(unknown_type)  # None
```

### `DocumentLoader.validate_document_source(source_path, document_type)`
Validate document source and type combination.

```python
try:
    DocumentLoader.validate_document_source("report.pdf", "pdf")
    print("Valid document source")
except Exception as e:
    print(f"Invalid: {e}")
```

---

## Batch Processing

```python
import os
from graphbit import DocumentLoader

def process_directory(directory):
    loader = DocumentLoader()
    results = []
    
    for filename in os.listdir(directory):
        file_path = os.path.join(directory, filename)
        doc_type = DocumentLoader.detect_document_type(file_path)
        
        if doc_type:
            try:
                content = loader.load_document(file_path, doc_type)
                results.append({'file': filename, 'content': content})
            except Exception as e:
                print(f"Failed {filename}: {e}")
    
    return results
```

---

## Error Handling

```python
import os
from graphbit import DocumentLoader, DocumentLoaderConfig

def safe_load_document(file_path, max_size=50_000_000):
    try:
        # Validate file
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"File not found: {file_path}")
        
        # Check size
        if os.path.getsize(file_path) > max_size:
            raise ValueError("File too large")
        
        # Detect and validate type
        doc_type = DocumentLoader.detect_document_type(file_path)
        if not doc_type:
            raise ValueError("Unsupported file type")
        
        # Load with size limit
        config = DocumentLoaderConfig(max_file_size=max_size)
        loader = DocumentLoader(config)
        content = loader.load_document(file_path, doc_type)
        
        if content.is_empty():
            raise RuntimeError("No content extracted")
        
        return content
        
    except (FileNotFoundError, ValueError, RuntimeError) as e:
        print(f"Error: {e}")
        return None
```

---

## Common Issues

| Issue | Solution |
|-------|----------|
| File too large | Increase `max_file_size` in config |
| Encoding errors | Set `default_encoding="utf-8"` |
| Empty PDF content | Enable `ocr_enabled=True` for scanned PDFs |
| Unsupported format | Check `DocumentLoader.supported_types()` |

---

## API Reference

For complete API documentation, see [Python API Reference](../api-reference/python-api.md#document-processing).
