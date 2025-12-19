# Document Loader

This document covers document loading functionality in GraphBit JavaScript bindings for extracting content from various file formats.

## Overview

The `DocumentLoader` class provides utilities for loading and extracting text content from different document types (text files, JSON, and more). It handles encoding, file size limits, and formatting preservation.

## Class: `DocumentLoader`

### Constructors

#### `new DocumentLoader()`

Create a document loader with default configuration.

**Signature:**

```typescript
constructor()
```

**Default Configuration:**

- `maxFileSize`: 10485760 bytes (10 MB)
- `defaultEncoding`: "utf-8"
- `preserveFormatting`: false

### 🟢 Verified Example

```javascript
const { DocumentLoader } = require('graphbit');

const loader = new DocumentLoader();
```

---

#### `DocumentLoader.withConfig(config)`

Create a document loader with custom configuration.

**Signature:**

```typescript
static withConfig(config: DocumentLoaderConfig): DocumentLoader
```

**Parameters:**

- `config` (DocumentLoaderConfig): Configuration object

**DocumentLoaderConfig Interface:**

```typescript
interface DocumentLoaderConfig {
  maxFileSize?: number;         // Max file size in bytes
  defaultEncoding?: string;     // Text encoding (e.g., 'utf-8')
  preserveFormatting?: boolean; // Preserve document formatting
}
```

### 🟢 Verified Example

```javascript
const loader = DocumentLoader.withConfig({
  maxFileSize: 50000000,      // 50 MB
  defaultEncoding: 'utf-8',
  preserveFormatting: true    // Keep formatting
});

const config = loader.config();
console.log('Max file size:', config.maxFileSize);
```

---

### Methods

#### `loadFile(path, documentType)`

Load a document from a file path.

**Signature:**

```typescript
async loadFile(path: string, documentType: string): Promise<DocumentContent>
```

**Parameters:**

- `path` (string): Absolute or relative file path
- `documentType` (string): Document type (e.g., "txt", "json", "pdf")

**Returns:** Promise resolving to `DocumentContent`

**Supported Document Types:**

- `"txt"` - Plain text files
- `"json"` - JSON files
- Additional types may vary by platform

### 🟢 Verified Example

```javascript
const { DocumentLoader } = require('graphbit');

const loader = new DocumentLoader();

// Load text file
const textDoc = await loader.loadFile('./documents/report.txt', 'txt');
console.log('Content:', textDoc.content);
console.log('Source:', textDoc.source);
console.log('Type:', textDoc.documentType);

// Load JSON file
const jsonDoc = await loader.loadFile('./data/config.json', 'json');
const parsed = JSON.parse(jsonDoc.content);
```

---

#### `loadText(text, source?)`

Load content from a text string (useful for in-memory processing).

**Signature:**

```typescript
async loadText(text: string, source?: string): Promise<DocumentContent>
```

**Parameters:**

- `text` (string): The text content to load
- `source` (string, optional): Source identifier (default: auto-generated)

**Returns:** Promise resolving to `DocumentContent`

### 🟢 Verified Example

```javascript
const loader = new DocumentLoader();

// With source
const doc1 = await loader.loadText(
  'This is my document content',
  'user-input'
);
console.log('Source:', doc1.source); // "user-input"

// Without source
const doc2 = await loader.loadText('More content');
console.log('Source:', doc2.source); // Auto-generated
```

---

#### `config()`

Get the current loader configuration.

**Signature:**

```typescript
config(): DocumentLoaderConfig
```

**Returns:** Current configuration object

### 🟢 Verified Example

```javascript
const loader = DocumentLoader.withConfig({
  maxFileSize: 20000000,
  preserveFormatting: true
});

const config = loader.config();
console.log('Config:', config);
// {
//   maxFileSize: 20000000,
//   defaultEncoding: "utf-8",
//   preserveFormatting: true
// }
```

---

## Interface: `DocumentContent`

Represents loaded document content with metadata.

**Properties:**

```typescript
interface DocumentContent {
  content: string;          // Extracted text content
  metadata?: string;        // Optional metadata (JSON string)
  source: string;           // Source path or identifier
  documentType: string;     // Document type (e.g., "txt", "json")
}
```

### 🟢 Verified Example

```javascript
const loader = new DocumentLoader();
const doc = await loader.loadFile('./test.txt', 'txt');

console.log('Content:', doc.content);           // "File contents..."
console.log('Source:', doc.source);             // "./test.txt"
console.log('Type:', doc.documentType);         // "txt"
console.log('Metadata:', doc.metadata || 'N/A'); // Optional metadata
```

---

## Complete Examples

### Example 1: Loading Multiple Documents

### 🟢 Verified End-to-End Example

```javascript
const { DocumentLoader } = require('graphbit');
const fs = require('fs').promises;
const path = require('path');

async function loadDocuments(directory, fileType) {
  const loader = new DocumentLoader();
  
  // Get all files of specified type
  const files = await fs.readdir(directory);
  const targetFiles = files.filter(f => f.endsWith(`.${fileType}`));
  
  console.log(`Loading ${targetFiles.length} ${fileType} files...`);
  
  // Load all documents
  const documents = [];
  for (const file of targetFiles) {
    const filePath = path.join(directory, file);
    
    try {
      const doc = await loader.loadFile(filePath, fileType);
      documents.push({
        filename: file,
        content: doc.content,
        size: doc.content.length,
        source: doc.source
      });
      
      console.log(`✅ Loaded ${file} (${doc.content.length} chars)`);
    } catch (error) {
      console.error(`❌ Failed to load ${file}:`, error.message);
    }
  }
  
  return documents;
}

// Usage
const docs = await loadDocuments('./documents', 'txt');
console.log(`Loaded ${docs.length} documents`);
```

---

### Example 2: Processing with Size Limits

```javascript
const { DocumentLoader } = require('graphbit');

async function loadWithSizeLimit(filePath, maxSizeMB) {
  const maxBytes = maxSizeMB * 1024 * 1024;
  
  const loader = DocumentLoader.withConfig({
    maxFileSize: maxBytes,
    defaultEncoding: 'utf-8'
  });
  
  try {
    const doc = await loader.loadFile(filePath, 'txt');
    console.log(`✅ Loaded ${doc.content.length} characters`);
    return doc;
  } catch (error) {
    if (error.message.includes('size')) {
      console.error(`File exceeds ${maxSizeMB}MB limit`);
    }
    throw error;
  }
}

// Load with 5MB limit
await loadWithSizeLimit('./large-document.txt', 5);
```

---

### Example 3: Integration with Text Splitter

```javascript
const { DocumentLoader, TextSplitter } = require('graphbit');

async function loadAndSplitDocument(filePath) {
  // 1. Load document
  const loader = new DocumentLoader();
  const doc = await loader.loadFile(filePath, 'txt');
  
  console.log(`Loaded ${doc.content.length} characters from ${doc.source}`);
  
  // 2. Split into chunks
  const splitter = TextSplitter.recursive(1000, 100);
  const chunks = splitter.split(doc.content);
  
  console.log(`Split into ${chunks.length} chunks`);
  
  // 3. Return processed chunks with metadata
  return chunks.map(chunk => ({
    text: chunk.content,
    source: doc.source,
    documentType: doc.documentType,
    chunkIndex: chunk.chunkIndex,
    position: {
      start: chunk.startIndex,
      end: chunk.endIndex
    }
  }));
}

// Usage
const processedChunks = await loadAndSplitDocument('./article.txt');
processedChunks.forEach((chunk, idx) => {
  console.log(`Chunk ${idx}: ${chunk.text.substring(0, 50)}...`);
});
```

---

### Example 4: Loading User Input

```javascript
const { DocumentLoader } = require('graphbit');

async function processUserInput(userText, userId) {
  const loader = new DocumentLoader();
  
  // Load from string with source tracking
  const doc = await loader.loadText(userText, `user-${userId}`);
  
  console.log('Processing input from:', doc.source);
  console.log('Content length:', doc.content.length);
  
  // Process the content
  // ... your processing logic ...
  
  return {
    source: doc.source,
    processedAt: new Date().toISOString(),
    contentLength: doc.content.length
  };
}

// Usage
const result = await processUserInput(
  'User typed this text...',
  'user123'
);
```

---

## Configuration Best Practices

### 1. Set Appropriate Size Limits

```javascript
// ❌ Too restrictive for typical use
const tooSmall = DocumentLoader.withConfig({
  maxFileSize: 100000  // 100KB
});

// ✅ Reasonable for most documents
const goodSize = DocumentLoader.withConfig({
  maxFileSize: 10485760  // 10MB (default)
});

// ✅ For large documents
const largeFiles = DocumentLoader.withConfig({
  maxFileSize: 104857600  // 100MB
});
```

### 2. Handle Encoding Properly

```javascript
// Default UTF-8 is usually fine
const loader1 = new DocumentLoader();

// For special encodings
const loader2 = DocumentLoader.withConfig({
  defaultEncoding: 'latin1'  // If needed
});
```

### 3. Preserve Formatting When Needed

```javascript
// For plain text processing
const plainLoader = DocumentLoader.withConfig({
  preserveFormatting: false  // Strip extra whitespace
});

// For code or formatted documents
const formattedLoader = DocumentLoader.withConfig({
  preserveFormatting: true  // Keep all formatting
});
```

---

## Error Handling

### Common Errors and Solutions

```javascript
const { DocumentLoader } = require('graphbit');

async function safeLoadDocument(filePath, docType) {
  const loader = new DocumentLoader();
  
  try {
    const doc = await loader.loadFile(filePath, docType);
    return doc;
  } catch (error) {
    if (error.message.includes('not found')) {
      console.error('File does not exist:', filePath);
    } else if (error.message.includes('size')) {
      console.error('File too large:', filePath);
    } else if (error.message.includes('encoding')) {
      console.error('Encoding error:', error.message);
    } else {
      console.error('Unknown error:', error);
    }
    
    return null;
  }
}
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Constructor** | `DocumentLoader(config=None)` | `new DocumentLoader()` or `DocumentLoader.withConfig(config)` |
| **Load method** | `load_document(source_path, document_type)` | `loadFile(path, documentType)` - async |
| **Text loading** | Not available | `loadText(text, source)` - async |
| **Config access** | Properties: `config.max_file_size` | Method: `loader.config()` |
| **Static methods** | `DocumentLoader.supported_types()`, `detect_document_type()` | Not available in JS |
| **Return type** | `DocumentContent` class | Plain object |

**Key Differences:**

- JS provides `loadText()` for in-memory loading (not in Python)
- Python has helper methods for type detection (not in JS)
- JS uses async/await throughout

---

## Use Cases

### Use Case 1: RAG System Document Ingestion

```javascript
const { DocumentLoader, TextSplitter, EmbeddingClient, EmbeddingConfig } = require('graphbit');

async function ingestDocument(filePath) {
  // 1. Load document
  const loader = DocumentLoader.withConfig({
    maxFileSize: 50000000,
    preserveFormatting: false
  });
  const doc = await loader.loadFile(filePath, 'txt');
  
  // 2. Split into chunks
  const splitter = TextSplitter.recursive(800, 100);
  const chunks = splitter.split(doc.content);
  
  // 3. Create embeddings
  const embedConfig = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const embedClient = new EmbeddingClient(embedConfig);
  
  const texts = chunks.map(c => c.content);
  const embeddings = await embedClient.embed(texts);
  
  // 4. Store in vector database
  const records = chunks.map((chunk, idx) => ({
    text: chunk.content,
    embedding: embeddings.embeddings[idx],
    source: doc.source,
    chunkIndex: idx
  }));
  
  return records;
}
```

### Use Case 2: Batch Document Processing

```javascript
async function processBatch(filePaths) {
  const loader = new DocumentLoader();
  const results = [];
  
  for (const filePath of filePaths) {
    try {
      const doc = await loader.loadFile(filePath, 'txt');
      results.push({
        success: true,
        path: filePath,
        contentLength: doc.content.length
      });
    } catch (error) {
      results.push({
        success: false,
        path: filePath,
        error: error.message
      });
    }
  }
  
  const successCount = results.filter(r => r.success).length;
  console.log(`Processed: ${successCount}/${filePaths.length} files`);
  
  return results;
}
```

---

## Troubleshooting

### Issue: "File too large" Error

```javascript
// Problem: Default 10MB limit exceeded
const loader = new DocumentLoader();
// await loader.loadFile('huge.txt', 'txt'); // Error!

// Solution: Increase limit
const bigLoader = DocumentLoader.withConfig({
  maxFileSize: 104857600  // 100MB
});
await bigLoader.loadFile('huge.txt', 'txt'); // Works
```

### Issue: Encoding Problems

```javascript
// Problem: Special characters appear corrupted
const loader = new DocumentLoader();

// Solution: Check file encoding and configure appropriately
const encodedLoader = DocumentLoader.withConfig({
  defaultEncoding: 'utf-8'  // Or 'latin1', 'ascii', etc.
});
```

### Issue: File Not Found

```javascript
const path = require('path');

// Problem: Relative paths may not resolve correctly
// await loader.loadFile('./doc.txt', 'txt'); // May fail

// Solution: Use absolute paths
const absolutePath = path.resolve(__dirname, './doc.txt');
await loader.loadFile(absolutePath, 'txt'); // Reliable
```

---

## Related Documentation

- [Text Splitter](./text-splitter.md) - Split loaded documents into chunks
- [Embeddings](./embeddings.md) - Create embeddings from document content
- [Workflow](./workflow.md) - Use documents in workflows
