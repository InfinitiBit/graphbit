# Document Loading

GraphBit extracts content from multiple document formats for AI workflow processing.

## Components

- **DocumentLoader** - Main loading class
- **DocumentLoaderConfig** - Configuration options  
- **DocumentContent** - Extracted content and metadata

## Supported Formats

PDF, DOCX, TXT, JSON, CSV, XML, HTML

## Quick Start

```python
from graphbit import init, DocumentLoader

init()
loader = DocumentLoader()

# Load document
content = loader.load_document("document.pdf", "pdf")
print(f"Extracted {content.content_length()} characters")
```

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

## Workflow Integration

```python
from graphbit import Workflow, Node, Executor

# Document processing workflow
workflow = Workflow("Document Analysis")

# Add document loader node
doc_loader = Node.document_loader(
    name="PDF Loader",
    document_type="pdf",
    source_path="report.pdf"
)

# Add analysis agent
analyzer = Node.agent(
    name="Analyzer",
    prompt="Summarize: {input}"
)

# Connect and execute
loader_id = workflow.add_node(doc_loader)
analyzer_id = workflow.add_node(analyzer)
workflow.connect(loader_id, analyzer_id)

executor = Executor(llm_config)
result = executor.execute(workflow)
```

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

## Performance Tips

### Memory Optimization
```python
from graphbit import DocumentLoaderConfig, DocumentLoader

def memory_efficient_processing(files):
    config = DocumentLoaderConfig(max_file_size=10_000_000)
    loader = DocumentLoader(config)
    
    for file_path in files:
        content = loader.load_document(file_path, doc_type)
        # Process immediately, don't store
        process_content(content)
        del content  # Free memory
```

### Batch Configuration
```python
from graphbit import DocumentLoaderConfig, DocumentLoader

# Shared loader for multiple files
config = DocumentLoaderConfig(preserve_formatting=False)  # Faster
loader = DocumentLoader(config)

# Process in batches
for batch in chunked(file_list, 10):
    for file_path in batch:
        content = loader.load_document(file_path, doc_type)
        # Process batch...
```

## Common Issues

| Issue | Solution |
|-------|----------|
| File too large | Increase `max_file_size` in config |
| Encoding errors | Set `default_encoding="utf-8"` |
| Empty PDF content | Enable `ocr_enabled=True` for scanned PDFs |
| Unsupported format | Check `DocumentLoader.supported_types()` |

## API Reference

For complete API documentation, see [Python API Reference](../api-reference/python-api.md#document-processing).