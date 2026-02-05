# Embeddings (JavaScript/TypeScript)

GraphBit provides vector embedding capabilities for semantic search, similarity analysis, and AI-powered text operations.

## Overview

GraphBit's embedding system supports:
- **OpenAI Provider** - OpenAI embedding models
- **HuggingFace Provider** - HuggingFace embedding models
- **Unified Interface** - Consistent API across providers
- **Batch Processing** - Efficient processing of multiple texts

## Configuration

### OpenAI Configuration

```typescript
import { EmbeddingConfig } from '@infinitibit_gmbh/graphbit';


const config = EmbeddingConfig.openai(
  process.env.OPENAI_API_KEY || '',
  'text-embedding-3-small' // Optional - defaults to text-embedding-3-small
);
```

### HuggingFace Configuration

```typescript
import { EmbeddingConfig } from '@infinitibit_gmbh/graphbit';


const config = EmbeddingConfig.huggingface(
  process.env.HUGGINGFACE_API_KEY || '',
  'sentence-transformers/all-MiniLM-L6-v2'
);
```

## Basic Usage

### Creating Embedding Client

```typescript
import { EmbeddingClient, EmbeddingConfig } from '@infinitibit_gmbh/graphbit';


const config = EmbeddingConfig.openai(
  process.env.OPENAI_API_KEY || ''
);

const client = new EmbeddingClient(config);
```

### Batch Text Embeddings

```typescript
import { EmbeddingClient, EmbeddingConfig } from '@infinitibit_gmbh/graphbit';


const client = new EmbeddingClient(
  EmbeddingConfig.openai(process.env.OPENAI_API_KEY || '')
);

const texts = [
  'Machine learning is transforming industries',
  'Natural language processing enables computers to understand text',
  'Deep learning models require large datasets'
];

const response = await client.embed(texts);

console.log(`Generated ${response.embeddings.length} embeddings`);
console.log(`Model: ${response.model}`);
console.log(`Usage: ${response.usage.totalTokens} tokens`);

for (let i = 0; i < response.embeddings.length; i++) {
  const embedding = response.embeddings[i];
  console.log(`Text ${i}: ${texts[i].substring(0, 50)}...`);
  console.log(`Vector dimension: ${embedding.length}`);
}
```

### Calculating Similarity

GraphBit provides a helper method to calculate cosine similarity between two embeddings.

```typescript
import { EmbeddingClient } from '@infinitibit_gmbh/graphbit';

// ... obtain embeddings as emb1, emb2

const similarity = EmbeddingClient.similarity(emb1, emb2);
console.log(`Similarity score: ${similarity}`); // 0.0 to 1.0 (or -1.0 to 1.0)
```


## Response Format

```typescript
interface EmbeddingResponse {
  embeddings: number[][];      // Array of embedding vectors
  model: string;               // Model used
  usage: EmbeddingUsage;       // Token usage stats
}

interface EmbeddingUsage {
  promptTokens: number;        // Prompt tokens used
  totalTokens: number;         // Total tokens used
}
```

##  Complete Example

```typescript
import { EmbeddingClient, EmbeddingConfig } from '@infinitibit_gmbh/graphbit';


async function main() {
  // Create client
  const client = new EmbeddingClient(
    EmbeddingConfig.openai(process.env.OPENAI_API_KEY || '')
  );

  // Texts to embed
  const texts = [
    'GraphBit is a high-performance AI agent framework',
    'It provides workflow orchestration and agent management',
    'The framework supports multiple LLM providers'
  ];

  // Generate embeddings
  const response = await client.embed(texts);

  console.log('Embedding Results:');
  console.log(`  Model: ${response.model}`);
  console.log(`  Tokens used: ${response.usage.totalTokens}`);
  console.log(`  Generated ${response.embeddings.length} vectors`);
  console.log(`  Vector dimension: ${response.embeddings[0]?.length}`);
}

main().catch(console.error);
```

## Integration with Text Splitters

```typescript
import { 
  TextSplitter, 
  EmbeddingClient, 
  EmbeddingConfig 
} from '@infinitibit_gmbh/graphbit';


async function embedDocument(text: string) {
  // Split large document
  const splitter = TextSplitter.recursive(1000, 100);
  const chunks = splitter.split(text);

  // Generate embeddings
  const client = new EmbeddingClient(
    EmbeddingConfig.openai(process.env.OPENAI_API_KEY || '')
  );

  const chunkTexts = chunks.map(chunk => chunk.content);
  const response = await client.embed(chunkTexts);

  // Combine chunks with embeddings
  return chunks.map((chunk, i) => ({
    content: chunk.content,
    embedding: response.embeddings[i],
    startIndex: chunk.startIndex,
    endIndex: chunk.endIndex
  }));
}
```
