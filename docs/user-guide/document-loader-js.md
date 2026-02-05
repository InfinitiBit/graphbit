# Document Loader (JavaScript/TypeScript)

GraphBit extracts content from multiple document formats for AI workflow processing.

## Components

- **DocumentLoader** - Main loading class
- **DocumentLoaderConfig** - Configuration options  
- **DocumentContent** - Extracted content and metadata

---

## Supported Document Types

| Type   | Description                        |
|--------|-----------------------------------|
| txt    | Plain text files                   |
| pdf    | PDF documents                      |
| docx   | Microsoft Word documents           |
| json   | JSON structured data files         |
| csv    | Comma-separated values            |
| xml    | XML structured data files          |
| html   | HTML web pages                     |

---

## Quick Start

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = new DocumentLoader();

// Load from text
const content = await loader.loadText(
  'This is sample text',
  'my-source'
);

console.log(`Loaded ${content.content.length} characters`);
```

---

## Loading Documents

### Load from Text

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = new DocumentLoader();

const content = await loader.loadText(
  'Your text content here',
  'inline-source' // Optional source identifier
);

console.log(content.content);
console.log(content.source);
console.log(content.documentType); // 'txt'
```

### Load from File

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = new DocumentLoader();

// Load a file (type auto-detected from extension)
const content = await loader.loadFile(
  './documents/report.pdf',
  'pdf'
);

console.log(`Content: ${content.content}`);
console.log(`Metadata: ${content.metadata}`);
```

---

## Configuration

### Custom Configuration

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = DocumentLoader.withConfig({
  maxFileSize: 50_000_000,      // 50MB limit
  defaultEncoding: 'utf-8',     // Text encoding
  preserveFormatting: true      // Keep formatting
});

const content = await loader.loadFile('document.pdf', 'pdf');
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `maxFileSize` | number | undefined | Maximum file size in bytes |
| `defaultEncoding` | string | undefined | Default text encoding |
| `preserveFormatting` | boolean | undefined | Preserve document formatting |

---

## Document Types

### PDF Documents

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = DocumentLoader.withConfig({
  preserveFormatting: true
});

const content = await loader.loadFile('report.pdf', 'pdf');

// Access content and metadata
console.log(content.content);
if (content.metadata) {
  console.log(`Metadata: ${content.metadata}`);
}
```

### Text Files

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = DocumentLoader.withConfig({
  defaultEncoding: 'utf-8'
});

const content = await loader.loadFile('notes.txt', 'txt');
console.log(content.content);
```

### Structured Data

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

const loader = new DocumentLoader();

// JSON
const jsonContent = await loader.loadFile('data.json', 'json');

// CSV
const csvContent = await loader.loadFile('data.csv', 'csv');

// XML
const xmlContent = await loader.loadFile('data.xml', 'xml');
```

---

## Batch Processing

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';
import * as fs from 'fs';
import * as path from 'path';

async function processDirectory(directory: string) {
  const loader = new DocumentLoader();
  const results = [];

  const files = fs.readdirSync(directory);
  
  for (const filename of files) {
    const filePath = path.join(directory, filename);
    const ext = path.extname(filename).slice(1); // Remove leading dot
    
    try {
      const content = await loader.loadFile(filePath, ext);
      results.push({ file: filename, content });
    } catch (error) {
      console.error(`Failed ${filename}:`, error);
    }
  }
  
  return results;
}
```

---

## Error Handling

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';
import * as fs from 'fs';

async function safeLoadDocument(filePath: string, maxSize = 50_000_000) {
  try {
    // Validate file exists
    if (!fs.existsSync(filePath)) {
      throw new Error(`File not found: ${filePath}`);
    }
    
    // Check file size
    const stats = fs.statSync(filePath);
    if (stats.size > maxSize) {
      throw new Error('File too large');
    }
    
    // Detect type from extension
    const ext = filePath.split('.').pop();
    if (!ext) {
      throw new Error('Cannot detect file type');
    }
    
    // Load with size limit
    const loader = DocumentLoader.withConfig({
      maxFileSize: maxSize
    });
    
    const content = await loader.loadFile(filePath, ext);
    
    if (!content.content || content.content.length === 0) {
      throw new Error('No content extracted');
    }
    
    return content;
    
  } catch (error) {
    console.error('Error:', error);
    return null;
  }
}
```

---

## Common Issues

| Issue | Solution |
|-------|----------|
| File too large | Increase `maxFileSize` in config |
| Encoding errors | Set `defaultEncoding: 'utf-8'` |
| Empty content | Check if file is valid and supported |
| Type errors | Verify file extension matches content |

---

## Complete Example

```typescript
import { DocumentLoader } from '@infinitibit_gmbh/graphbit';

async function main() {
  // Create loader with configuration
  const loader = DocumentLoader.withConfig({
    maxFileSize: 10_000_000, // 10MB
    defaultEncoding: 'utf-8',
    preserveFormatting: true
  });
  
  // Load from text
  const textContent = await loader.loadText(
    'Sample text content',
    'my-source'
  );
  console.log('Text loaded:', textContent.content);
  
  // Load from file
  const fileContent = await loader.loadFile('document.pdf', 'pdf');
  console.log('File loaded:', fileContent.content.length, 'chars');
}

main().catch(console.error);
```
