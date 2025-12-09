# Text Splitter

This document covers text chunking functionality in GraphBit JavaScript bindings for splitting large documents into manageable pieces.

## Overview

The `TextSplitter` class provides multiple strategies for splitting text into chunks, useful for processing large documents, preparing data for embeddings, or managing context windows for LLMs.

## Class: `TextSplitter`

TextSplitter uses static factory methods to create splitters with different strategies. Unlike Python which has separate classes (`CharacterSplitter`, `RecursiveSplitter`, etc.), JavaScript provides a unified class with strategy-based factories.

### Static Factory Methods

#### `TextSplitter.character(chunkSize, chunkOverlap?)`

Create a character-based text splitter.

**Signature:**

```typescript
static character(chunkSize: number, chunkOverlap?: number): TextSplitter
```

**Parameters:**

- `chunkSize` (number, required): Maximum characters per chunk
- `chunkOverlap` (number, optional): Character overlap between chunks (default: 0)

**Returns:** `TextSplitter` instance

### 🟢 Verified Example

```javascript
const { TextSplitter } = require('graphbit');

const splitter = TextSplitter.character(100, 20);

const text = 'Your long document text here...';
const chunks = splitter.split(text);

console.log(`Split into ${chunks.length} chunks`);
chunks.forEach((chunk, idx) => {
  console.log(`Chunk ${idx}: ${chunk.content.substring(0, 50)}...`);
});
```

---

#### `TextSplitter.recursive(chunkSize, chunkOverlap?)`

Create a recursive text splitter that tries larger separators first, then backs off to smaller ones.

**Signature:**

```typescript
static recursive(chunkSize: number, chunkOverlap?: number): TextSplitter
```

**Parameters:**

- `chunkSize` (number, required): Target maximum characters per chunk
- `chunkOverlap` (number, optional): Character overlap (default: 0)

**Description:** Attempts to split on paragraph boundaries (`\n\n`), then line boundaries (`\n`), then sentences, then spaces, in that order of priority.

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.recursive(500, 50);

const document = `
# Section 1

This is a paragraph with multiple sentences.

## Subsection

Another paragraph here.
`;

const chunks = splitter.split(document);
console.log(`Created ${chunks.length} chunks`); // 3 chunks
```

**Best for:** Structured documents with natural boundaries (markdown, code, articles)

---

#### `TextSplitter.sentence(chunkSize?, chunkOverlap?)`

Create a sentence-based text splitter.

**Signature:**

```typescript
static sentence(chunkSize?: number, chunkOverlap?: number): TextSplitter
```

**Parameters:**

- `chunkSize` (number, optional): Approximate sentences per chunk (default: varies)
- `chunkOverlap` (number, optional): Sentence overlap (default: 0)

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.sentence(2, 0);

const text = `
First sentence here. Second sentence here! Third sentence here?
Fourth sentence. Fifth sentence! Sixth sentence?
`;

const chunks = splitter.split(text);
// Each chunk contains ~2 sentences

chunks.forEach((chunk, idx) => {
  console.log(`Chunk ${idx}: ${chunk.content}`);
});
```

**Best for:** Natural language documents, articles, stories

---

#### `TextSplitter.token(chunkSize, chunkOverlap?)`

Create a token-based text splitter.

**Signature:**

```typescript
static token(chunkSize: number, chunkOverlap?: number): TextSplitter
```

**Parameters:**

- `chunkSize` (number, required): Maximum tokens per chunk
- `chunkOverlap` (number, optional): Token overlap (default: 0)

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.token(100, 10);

const text = 'Your document with many tokens...';
const chunks = splitter.split(text);

console.log(`Split into ${chunks.length} chunks`);
```

**Best for:** LLM context window management, API token limits

---

### Instance Methods

#### `split(text)`

Split text into chunks using the configured strategy.

**Signature:**

```typescript
split(text: string): TextChunk[]
```

**Parameters:**

- `text` (string): The text to split

**Returns:** Array of `TextChunk` objects

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.character(50, 10);

const chunks = splitter.split('This is a test document...');

// Access chunk properties
chunks.forEach((chunk) => {
  console.log('Content:', chunk.content);
  console.log('Position:', chunk.startIndex, '-', chunk.endIndex);
  console.log('Chunk index:', chunk.chunkIndex);
  console.log('Metadata:', chunk.metadata);
});
```

---

#### `config()`

Get the splitter configuration.

**Signature:**

```typescript
config(): TextSplitterConfig
```

**Returns:** `TextSplitterConfig` object

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.character(100);
const config = splitter.config();

console.log('Strategy:', config.strategy);
console.log('Preserve word boundaries:', config.preserveWordBoundaries);
console.log('Trim whitespace:', config.trimWhitespace);
```

**Config Structure:**

```json
{
  "strategy": {
    "type": "Character",
    "chunk_size": 100,
    "chunk_overlap": 0
  },
  "preserveWordBoundaries": true,
  "trimWhitespace": true,
  "includeMetadata": true,
  "extraParams": {}
}
```

---

## Interface: `TextChunk`

Represents a single chunk of text.

**Properties:**

```typescript
interface TextChunk {
  content: string;           // The chunk text
  startIndex: number;        // Start position in original text
  endIndex: number;          // End position in original text
  chunkIndex: number;        // Chunk number in sequence (0-based)
  metadata: Record<string, any>;  // Additional metadata
}
```

### 🟢 Verified Example

```javascript
const splitter = TextSplitter.character(50, 10);
const chunks = splitter.split('Example text...');

const chunk = chunks[0];
console.log('Text:', chunk.content);               // "Example text..."
console.log('Starts at:', chunk.startIndex);       // 0
console.log('Ends at:', chunk.endIndex);           // 47
console.log('Chunk number:', chunk.chunkIndex);    // 0
console.log('Length:', chunk.metadata.length);     // 46
```

---

## Complete Examples

### Example 1: Processing a Large Document

### 🟢 Verified End-to-End Example

```javascript
const { TextSplitter } = require('graphbit');

function processLargeDocument(documentText) {
  // Choose strategy based on content type
  const splitter = TextSplitter.recursive(1000, 100);
  
  // Split the document
  const chunks = splitter.split(documentText);
  
  console.log(`Document split into ${chunks.length} chunks`);
  
  // Process each chunk
  chunks.forEach((chunk, idx) => {
    console.log(`\nChunk ${idx + 1}/${chunks.length}`);
    console.log(`  Position: ${chunk.startIndex}-${chunk.endIndex}`);
    console.log(`  Length: ${chunk.metadata.length} characters`);
    console.log(`  Preview: ${chunk.content.substring(0, 100)}...`);
    
    // Process chunk (e.g., create embeddings, send to LLM, etc.)
    // processChunk(chunk.content);
  });
  
  return chunks;
}

// Example usage
const largeDoc = `
# Document Title

This is a large document with multiple paragraphs.

## Section 1

Content for section 1...

## Section 2

Content for section 2...
`;

const chunks = processLargeDocument(largeDoc);
```

---

### Example 2: Handling Overlapping Chunks

```javascript
const { TextSplitter } = require('graphbit');

// Create splitter with overlap for context preservation
const splitter = TextSplitter.character(200, 50);

const text = `
This is the first paragraph with important context.
This paragraph continues the thought from before.
And this final paragraph concludes the idea.
`;

const chunks = splitter.split(text);

// Verify overlap
console.log('Chunk 0 end:', chunks[0].content.slice(-50));
console.log('Chunk 1 start:', chunks[1].content.slice(0, 50));
// These should overlap!
```

---

### Example 3: Choosing the Right Strategy

```javascript
const { TextSplitter } = require('graphbit');

function chooseStrategy(contentType, maxChunkSize) {
  switch (contentType) {
    case 'code':
      // Recursive for code (respects structure)
      return TextSplitter.recursive(maxChunkSize, 0);
      
    case 'article':
      // Sentence for natural language
      return TextSplitter.sentence(10, 1);
      
    case 'tokens':
      // Token for LLM APIs
      return TextSplitter.token(maxChunkSize, 20);
      
    default:
      // Character for generic text
      return TextSplitter.character(maxChunkSize, 50);
  }
}

// Usage
const codeSplitter = chooseStrategy('code', 1000);
const articleSplitter = chooseStrategy('article', 10);
```

---

## Strategy Comparison

| Strategy | Best For | Preserves | Overhead |
|----------|----------|-----------|----------|
| **Character** | Generic text, fixed sizes | Nothing | Low |
| **Recursive** | Structured docs (markdown, code) | Paragraphs, lines | Medium |
| **Sentence** | Natural language | Sentence boundaries | Medium |
| **Token** | LLM context windows | Token boundaries | High |

---

## Best Practices

### 1. Choose Appropriate Chunk Size

```javascript
// ❌ Too small - excessive overhead
const tooSmall = TextSplitter.character(10, 5);

// ✅ Reasonable size for embeddings
const goodForEmbeddings = TextSplitter.recursive(1000, 100);

// ✅ Match LLM token limits
const goodForLLM = TextSplitter.token(4000, 200);
```

### 2. Use Overlap for Context

```javascript
// ❌ No overlap - loses context
const noOverlap = TextSplitter.character(500, 0);

// ✅ Overlap preserves context across chunks
const withOverlap = TextSplitter.character(500, 100);  // 20% overlap
```

### 3. Strategy Matching

```javascript
const text = getDocumentText();
const docType = detectDocumentType(text);

let splitter;
if (docType === 'prose') {
  splitter = TextSplitter.sentence(5, 1);
} else if (docType === 'structured') {
  splitter = TextSplitter.recursive(1000, 100);
} else {
  splitter = TextSplitter.character(500, 50);
}
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Classes** | Separate classes: `CharacterSplitter`, `RecursiveSplitter`, etc. | Unified `TextSplitter` class |
| **Creation** | `CharacterSplitter(chunk_size, chunk_overlap)` | `TextSplitter.character(chunkSize, chunkOverlap)` |
| **Methods** | `split_text(text)`, `split_texts(texts)` | `split(text)` only |
| **Config** | `TextSplitterConfig` factories | Config available via `config()` method |
| **Chunk structure** | `TextChunk` class | Plain object with properties |

**Key Difference:** JavaScript uses a factory pattern instead of separate classes, providing a cleaner API surface.

---

## Common Use Cases

### Use Case 1: Preparing Text for Embeddings

```javascript
const { TextSplitter, EmbeddingClient, EmbeddingConfig } = require('graphbit');

async function createDocumentEmbeddings(documentText) {
  // 1. Split document
  const splitter = TextSplitter.recursive(800, 100);
  const chunks = splitter.split(documentText);
  
  // 2. Create embeddings for each chunk
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  const chunkTexts = chunks.map(c => c.content);
  const response = await client.embed(chunkTexts);
  
  // 3. Store chunks with embeddings
  const records = chunks.map((chunk, idx) => ({
    text: chunk.content,
    embedding: response.embeddings[idx],
    metadata: {
      startIndex: chunk.startIndex,
      endIndex: chunk.endIndex,
      chunkIndex: chunk.chunkIndex
    }
  }));
  
  return records;
}
```

### Use Case 2: Chunking for LLM Context Window

```javascript
const { TextSplitter } = require('graphbit');

function fitToContextWindow(text, maxTokens = 4000) {
  // Use token splitter with safety margin
  const splitter = TextSplitter.token(maxTokens - 500, 100);
  
  const chunks = splitter.split(text);
  
  if (chunks.length > 1) {
    console.warn(`Text split into ${chunks.length} chunks to fit context`);
  }
  
  return chunks.map(c => c.content);
}

// Usage
const longText = '... very long document ...';
const safeChunks = fitToContextWindow(longText, 4096);
```

---

## Troubleshooting

### Issue: Chunks Too Small

```javascript
// Problem: Getting too many tiny chunks
const splitter = TextSplitter.character(10, 5);

// Solution: Increase chunk size
const better = TextSplitter.character(500, 50);
```

### Issue: Loss of Context

```javascript
// Problem: Important context split across chunks
const splitter = TextSplitter.character(100, 0);

// Solution: Add overlap
const better = TextSplitter.character(100, 20);
```

### Issue: Sentence Splitter Not Working Well

```javascript
// Problem: Unusual punctuation or formatting
const text = 'Mr. Smith went to Dr. Johnson...';

// Solution: Use character or recursive instead
const splitter = TextSplitter.recursive(200, 20);
```

---

## Related Documentation

- [Embeddings](./embeddings.md) - Generate embeddings for chunks
- [Document Loader](./document-loader.md) - Load documents before splitting
- [LLM Configuration](./llm-config.md) - Configure LLMs for processing chunks
