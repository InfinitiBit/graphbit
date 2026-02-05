# Text Splitters (JavaScript/TypeScript)

Text splitters break large documents into manageable chunks while maintaining context and semantic coherence.

## Overview

Text splitters help you:
- Process large documents that exceed model context windows
- Create embeddings for semantic search
- Parallelize document processing
- Maintain context across chunk boundaries

## Available Splitters

### Character Splitter

Splits text based on character count.

```typescript
import { TextSplitter } from '@infinitibit_gmbh/graphbit';

const splitter = TextSplitter.character(
  1000,     // Chunk size
  200       // Chunk overlap (optional)
);

const chunks = splitter.split(text);

for (const chunk of chunks) {
  console.log(`Chunk: ${chunk.content.length} characters`);
  console.log(`Position: ${chunk.startIndex} to ${chunk.endIndex}`);
}
```

### Recursive Splitter

Hierarchically splits text using multiple separators, ideal for structured documents.

```typescript
import { TextSplitter } from '@infinitibit_gmbh/graphbit';

const splitter = TextSplitter.recursive(
  1000,     // Chunk size
  100       // Chunk overlap (optional)
);

const chunks = splitter.split(text);
```

### Sentence Splitter

Maintains sentence boundaries for semantic coherence.

```typescript
import { TextSplitter } from '@infinitibit_gmbh/graphbit';

const splitter = TextSplitter.sentence(
  500,      // Target size (optional)
  1         // Sentence overlap (optional)
);

const chunks = splitter.split(text);
```

### Token Splitter

Splits based on token count, useful for LLM context limits.

```typescript
import { TextSplitter } from '@infinitibit_gmbh/graphbit';

const splitter = TextSplitter.token(
  100,      // Max tokens per chunk
  20        // Token overlap (optional)
);

const chunks = splitter.split(text);
```

## Working with Chunks

Each chunk has the following properties:

```typescript
interface TextChunk {
  content: string;       // The chunk text
  startIndex: number;    // Start position in original text
  endIndex: number;      // End position in original text
  metadata?: string;     // Optional metadata (JSON string)
}
```

Example:

```typescript
const splitter = TextSplitter.character(1000);
const chunks = splitter.split(text);

for (const chunk of chunks) {
  console.log('Content:', chunk.content);
  console.log('Start:', chunk.startIndex);
  console.log('End:', chunk.endIndex);
  console.log('Length:', chunk.endIndex - chunk.startIndex);
}
```

## Best Practices

### Choose the Right Splitter

- **Character Splitter**: Simple documents, consistent chunk sizes
- **Token Splitter**: Working with LLMs, precise token control
- **Sentence Splitter**: Maintaining semantic boundaries
- **Recursive Splitter**: Structured documents, code files

### Optimize Chunk Size

Common sizes:
- Embeddings: 500-1000 characters
- LLM processing: 2000-4000 characters
- Summarization: 1000-2000 characters

### Use Appropriate Overlap

- Small overlap (10-20%): General documents
- Medium overlap (20-30%): Technical content
- Large overlap (30-50%): Dense information

## Complete Example

```typescript
import { TextSplitter } from '@infinitibit_gmbh/graphbit';

async function processDocument(text: string) {
  // Create a recursive splitter
  const splitter = TextSplitter.recursive(1000, 100);
  
  // Split the text
  const chunks = splitter.split(text);
  
  console.log(`Created ${chunks.length} chunks`);
  
  // Process each chunk
  for (let i = 0; i < chunks.length; i++) {
    const chunk = chunks[i];
    console.log(`Chunk ${i}:`);
    console.log(`  Length: ${chunk.content.length}`);
    console.log(`  Position: ${chunk.startIndex}-${chunk.endIndex}`);
    // ... process chunk (e.g., generate embeddings)
  }
}
```

## Integration with Embeddings

```typescript
import { TextSplitter, EmbeddingClient, EmbeddingConfig } from '@infinitibit_gmbh/graphbit';

async function createEmbeddings(text: string) {
  // Split text
  const splitter = TextSplitter.recursive(1000, 100);
  const chunks = splitter.split(text);
  
  // Create embedding client
  const embedder = new EmbeddingClient(
    EmbeddingConfig.openai(process.env.OPENAI_API_KEY || '', 'text-embedding-3-small')
  );
  
  // Generate embeddings for each chunk
  const texts = chunks.map(chunk => chunk.content);
  const response = await embedder.embed(texts);
  
  return chunks.map((chunk, i) => ({
    content: chunk.content,
    embedding: response.embeddings[i],
    metadata: chunk.metadata
  }));
}
```
