# Embeddings

This document covers embedding generation functionality in GraphBit JavaScript bindings for creating vector representations of text.

## Overview

The embedding system allows you to convert text into numerical vectors (embeddings) for semantic search, similarity comparison, clustering, and other machine learning tasks. GraphBit supports multiple embedding providers including OpenAI and HuggingFace.

## Class: `EmbeddingConfig`

Configuration for embedding providers.

### Static Factory Methods

#### `EmbeddingConfig.openai(apiKey, model?)`

Create OpenAI embeddings configuration.

**Signature:**

```typescript
static openai(apiKey: string, model?: string): EmbeddingConfig
```

**Parameters:**

- `apiKey` (string, required): Your OpenAI API key
- `model` (string, optional): Model name (default: `"text-embedding-3-small"`)

**Available Models:**

- `text-embedding-3-small` - Fast, cost-effective (default)
- `text-embedding-3-large` - Higher quality, larger dimensions
- `text-embedding-ada-002` - Previous generation model

**Returns:** `EmbeddingConfig` instance

### 🟢 Verified Example

```javascript
const { EmbeddingConfig } = require('@infinitibit_gmbh/graphbit');

// With specific model
const config = EmbeddingConfig.openai(
  process.env.OPENAI_API_KEY,
  'text-embedding-3-small'
);

// With default model
const defaultConfig = EmbeddingConfig.openai(
  process.env.OPENAI_API_KEY
);
```

---

#### `EmbeddingConfig.huggingface(apiKey, model)`

Create HuggingFace embeddings configuration.

**Signature:**

```typescript
static huggingface(apiKey: string, model: string): EmbeddingConfig
```

**Parameters:**

- `apiKey` (string, required): Your HuggingFace API key
- `model` (string, required): Model identifier

**Popular Models:**

- `sentence-transformers/all-MiniLM-L6-v2` - Fast, general purpose
- `sentence-transformers/all-mpnet-base-v2` - Higher quality
- `intfloat/e5-large-v2` - Latest E5 model

**Returns:** `EmbeddingConfig` instance

### 🟢 Verified Example

```javascript
const config = EmbeddingConfig.huggingface(
  process.env.HUGGINGFACE_API_KEY,
  'sentence-transformers/all-MiniLM-L6-v2'
);
```

---

## Class: `EmbeddingClient`

Client for generating text embeddings.

### Constructor

#### `new EmbeddingClient(config)`

Create an embedding client.

**Signature:**

```typescript
constructor(config: EmbeddingConfig)
```

**Parameters:**

- `config` (EmbeddingConfig): Embedding configuration

### 🟢 Verified Example

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
const client = new EmbeddingClient(config);
```

---

### Methods

#### `embed(texts)`

Generate embeddings for an array of texts.

**Signature:**

```typescript
async embed(texts: string[]): Promise<EmbeddingResponse>
```

**Parameters:**

- `texts` (string[], required): Array of text strings to embed

**Returns:** Promise resolving to `EmbeddingResponse`

**EmbeddingResponse Structure:**

```typescript
interface EmbeddingResponse {
  embeddings: number[][];     // Array of embedding vectors
  model: string;              // Model used
  usage: EmbeddingUsage;      // Token usage statistics
}

interface EmbeddingUsage {
  promptTokens: number;       // Input tokens consumed
  totalTokens: number;        // Total tokens consumed
}
```

### 🟢 Verified Example

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

async function generateEmbeddings() {
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  const texts = [
    'The quick brown fox',
    'Machine learning is fascinating',
    'GraphBit makes development easy'
  ];
  
  const response = await client.embed(texts);
  
  console.log('Model:', response.model);
  console.log('Embeddings:', response.embeddings.length);
  console.log('Dimension:', response.embeddings[0].length);
  console.log('Usage:', response.usage);
  
  return response.embeddings;
}
```

#### `EmbeddingClient.similarity(embedding1, embedding2)`

Calculate cosine similarity between two embeddings.

```typescript
const score = EmbeddingClient.similarity(emb1, emb2);
```
```

---

## Complete Examples

### Example 1: Semantic SearchSystem

### 🟢 Verified End-to-End Example

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

class SemanticSearch {
  constructor(apiKey) {
    const config = EmbeddingConfig.openai(apiKey);
    this.client = new EmbeddingClient(config);
    this.documents = [];
    this.embeddings = [];
  }
  
  async indexDocuments(documents) {
    console.log(`Indexing ${documents.length} documents...`);
    
    // Generate embeddings for all documents
    const response = await this.client.embed(documents);
    
    this.documents = documents;
    this.embeddings = response.embeddings;
    
    console.log(`✅ Indexed ${this.embeddings.length} documents`);
    console.log(`   Embedding dimension: ${this.embeddings[0].length}`);
    console.log(`   Tokens used: ${response.usage.totalTokens}`);
  }
  
  async search(query, topK = 3) {
    // Embed the query
    const queryResponse = await this.client.embed([query]);
    const queryEmbedding = queryResponse.embeddings[0];
    
    // Calculate similarity scores
    const scores = this.embeddings.map((docEmbed, idx) => ({
      index: idx,
      document: this.documents[idx],
      score: this.cosineSimilarity(queryEmbedding, docEmbed)
    }));
    
    // Sort by similarity and return top K
    scores.sort((a, b) => b.score - a.score);
    return scores.slice(0, topK);
  }
  
  cosineSimilarity(a, b) {
    const dotProduct = a.reduce((sum, val, i) => sum + val * b[i], 0);
    const magnitudeA = Math.sqrt(a.reduce((sum, val) => sum + val * val, 0));
    const magnitudeB = Math.sqrt(b.reduce((sum, val) => sum + val * val, 0));
    return dotProduct / (magnitudeA * magnitudeB);
  }
}

// Usage
async function main() {
  const search = new SemanticSearch(process.env.OPENAI_API_KEY);
  
  // Index documents
  const docs = [
    'Python is a programming language',
    'Machine learning uses algorithms',
    'GraphBit simplifies workflow automation',
    'JavaScript is used for web development',
    'Embeddings represent text as vectors'
  ];
  
  await search.indexDocuments(docs);
  
  // Search
  const results = await search.search('coding languages', 2);
  
  console.log('\nSearch Results:');
  results.forEach((result, idx) => {
    console.log(`${idx + 1}. ${result.document} (score: ${result.score.toFixed(3)})`);
  });
}

main().catch(console.error);
```

---

### Example 2: Document Clustering

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

async function clusterDocuments(documents, numClusters = 3) {
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  // 1. Generate embeddings
  console.log('Generating embeddings...');
  const response = await client.embed(documents);
  const embeddings = response.embeddings;
  
  console.log(`Embedded ${embeddings.length} documents`);
  console.log(`Dimension: ${embeddings[0].length}`);
  
  // 2. Simple k-means clustering (simplified for example)
  // In production, use a proper clustering library
  const clusters = simpleKMeans(embeddings, numClusters);
  
  // 3. Group documents by cluster
  const clusteredDocs = {};
  embeddings.forEach((_, idx) => {
    const cluster = clusters[idx];
    if (!clusteredDocs[cluster]) clusteredDocs[cluster] = [];
    clusteredDocs[cluster].push(documents[idx]);
  });
  
  // 4. Display results
  Object.entries(clusteredDocs).forEach(([cluster, docs]) => {
    console.log(`\nCluster ${cluster}:`);
    docs.forEach(doc => console.log(`  - ${doc}`));
  });
  
  return clusteredDocs;
}

// Simplified k-means (for illustration)
function simpleKMeans(embeddings, k) {
  // Implementation details omitted for brevity
  // Use a proper library like ml-kmeans in production
  return embeddings.map((_, idx) => idx % k);
}
```

---

### Example 3: Integration with Text Splitter

```javascript
const { 
  EmbeddingConfig, 
  EmbeddingClient, 
  DocumentLoader, 
  TextSplitter 
} = require('@infinitibit_gmbh/graphbit');

async function processDocumentForRAG(filePath) {
  // 1. Load document
  const loader = new DocumentLoader();
  const doc = await loader.loadFile(filePath, 'txt');
  
  console.log(`Loaded: ${doc.source}`);
  
  // 2. Split into chunks
  const splitter = TextSplitter.recursive(800, 100);
  const chunks = splitter.split(doc.content);
  
  console.log(`Split into ${chunks.length} chunks`);
  
  // 3. Generate embeddings for chunks
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  const chunkTexts = chunks.map(c => c.content);
  
  // Process in batches to avoid rate limits
  const batchSize = 20;
  const allEmbeddings = [];
  
  for (let i = 0; i < chunkTexts.length; i += batchSize) {
    const batch = chunkTexts.slice(i, i + batchSize);
    console.log(`Processing batch ${Math.floor(i/batchSize) + 1}...`);
    
    const response = await client.embed(batch);
    allEmbeddings.push(...response.embeddings);
    
    // Rate limiting
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  
  // 4. Combine chunks with embeddings
  const records = chunks.map((chunk, idx) => ({
    text: chunk.content,
    embedding: allEmbeddings[idx],
    metadata: {
      source: doc.source,
      chunkIndex: chunk.chunkIndex,
      startIndex: chunk.startIndex,
      endIndex: chunk.endIndex
    }
  }));
  
  console.log(`✅ Created ${records.length} embedded chunks`);
  
  return records;
}

// Usage
processDocumentForRAG('./document.txt')
  .then(records => {
    console.log('Ready to store in vector database');
    // Store records in your vector DB (Pinecone, Weaviate, etc.)
  })
  .catch(console.error);
```

---

### Example 4: Batch Processing with Rate Limiting

```javascript
const { EmbeddingConfig, EmbeddingClient } = require('@infinitibit_gmbh/graphbit');

async function batchEmbedWithRateLimit(texts, batchSize = 100, delayMs = 1000) {
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  const allEmbeddings = [];
  let totalTokens = 0;
  
  for (let i = 0; i < texts.length; i += batchSize) {
    const batch = texts.slice(i, i + batchSize);
    const batchNum = Math.floor(i / batchSize) + 1;
    const totalBatches = Math.ceil(texts.length / batchSize);
    
    console.log(`Processing batch ${batchNum}/${totalBatches}...`);
    
    const response = await client.embed(batch);
    allEmbeddings.push(...response.embeddings);
    totalTokens += response.usage.totalTokens;
    
    console.log(`  Tokens: ${response.usage.totalTokens}`);
    
    // Wait before next batch (except for last batch)
    if (i + batchSize < texts.length) {
      await new Promise(resolve => setTimeout(resolve, delayMs));
    }
  }
  
  console.log(`\n✅ Total: ${allEmbeddings.length} embeddings`);
  console.log(`   Total tokens: ${totalTokens}`);
  
  return allEmbeddings;
}

// Usage
const largeTextCollection = [...]; // Array of texts
const embeddings = await batchEmbedWithRateLimit(largeTextCollection, 50, 500);
```

---

## Best Practices

### 1. Batch Processing

```javascript
// ❌ Bad: One at a time (slow, expensive)
for (const text of texts) {
  await client.embed([text]);
}

// ✅ Good: Batch processing
const response = await client.embed(texts);
```

### 2. Rate Limiting

```javascript
async function embedWithRateLimit(client, textBatches) {
  const results = [];
  
  for (const batch of textBatches) {
    const response = await client.embed(batch);
    results.push(response);
    
    // Wait 1 second between batches
    await new Promise(r => setTimeout(r, 1000));
  }
  
  return results;
}
```

### 3. Error Handling

```javascript
async function safeEmbed(client, texts, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await client.embed(texts);
    } catch (error) {
      if (attempt === maxRetries) throw error;
      
      console.log(`Attempt ${attempt} failed, retrying...`);
      await new Promise(r => setTimeout(r, 1000 * attempt));
    }
  }
}
```

### 4. Model Selection

```javascript
// For high volume, lower cost
const smallConfig = EmbeddingConfig.openai(
  apiKey,
  'text-embedding-3-small'
);

// For higher quality results
const largeConfig = EmbeddingConfig.openai(
  apiKey,
  'text-embedding-3-large'
);
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Config creation** | `EmbeddingConfig.openai(api_key, model)` | `EmbeddingConfig.openai(apiKey, model)` |
| **Embed method** | `embed(text)` for single, `embed_many(texts)` for multiple | `embed(texts)` for array only |
| **Similarity** | `EmbeddingClient.similarity(a, b)` static method | Not provided - implement manually |
| **Response** | Separate fields | `EmbeddingResponse` object |

**Key Difference:** JavaScript `embed()` only accepts arrays, while Python has separate methods for single vs multiple texts.

---

## Common Use Cases

### Use Case 1: RAG (Retrieval Augmented Generation)

```javascript
async function ragPipeline(documents, query) {
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  // 1. Embed all documents
  const docResponse = await client.embed(documents);
  
  // 2. Embed query
  const queryResponse = await client.embed([query]);
  const queryEmbed = queryResponse.embeddings[0];
  
  // 3. Find most relevant documents
  const relevantDocs = findTopK(queryEmbed, docResponse.embeddings, documents, 3);
  
  // 4. Use relevant docs as context for LLM
  return relevantDocs;
}
```

### Use Case 2: Duplicate Detection

```javascript
async function findDuplicates(texts, threshold = 0.95) {
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
  const client = new EmbeddingClient(config);
  
  const response = await client.embed(texts);
  const embeddings = response.embeddings;
  
  const duplicates = [];
  
  for (let i = 0; i < embeddings.length; i++) {
    for (let j = i + 1; j < embeddings.length; j++) {
      const similarity = cosineSimilarity(embeddings[i], embeddings[j]);
      
      if (similarity > threshold) {
        duplicates.push({
          index1: i,
          index2: j,
          text1: texts[i],
          text2: texts[j],
          similarity
        });
      }
    }
  }
  
  return duplicates;
}
```

---

## Troubleshooting

### Issue: Rate Limiting

```javascript
// Problem: Too many requests
// Error: "Rate limit exceeded"

// Solution: Add delays between batches
async function embedWithDelay(client, textBatches) {
  const results = [];
  
  for (const batch of textBatches) {
    const response = await client.embed(batch);
    results.push(response);
    
    await new Promise(r => setTimeout(r, 1000)); // 1sec delay
  }
  
  return results;
}
```

### Issue: Invalid API Key

```javascript
// Problem: Authentication error

// Solution: Verify API key is set
if (!process.env.OPENAI_API_KEY) {
  throw new Error('OPENAI_API_KEY environment variable not set');
}

const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
```

### Issue: Empty/Invalid Text

```javascript
// Problem: Empty strings cause errors

// Solution: Filter before embedding
const validTexts = texts.filter(t => t && t.trim().length > 0);

if (validTexts.length === 0) {
  throw new Error('No valid texts to embed');
}

const response = await client.embed(validTexts);
```

---

## Performance Tips

### Tip 1: Optimal Batch Sizes

```javascript
// OpenAI recommends ~20-100 texts per batch
const OPTIMAL_BATCH_SIZE = 50;

function createBatches(texts, size = OPTIMAL_BATCH_SIZE) {
  const batches = [];
  for (let i = 0; i < texts.length; i += size) {
    batches.push(texts.slice(i, i + size));
  }
  return batches;
}
```

### Tip 2: Caching

```javascript
class CachedEmbeddingClient {
  constructor(config) {
    this.client = new EmbeddingClient(config);
    this.cache = new Map();
  }
  
  async embed(texts) {
    const uncached = texts.filter(t => !this.cache.has(t));
    
    if (uncached.length > 0) {
      const response = await this.client.embed(uncached);
      uncached.forEach((text, idx) => {
        this.cache.set(text, response.embeddings[idx]);
      });
    }
    
    return {
      embeddings: texts.map(t => this.cache.get(t)),
      model: 'cached',
      usage: { promptTokens: 0, totalTokens: 0 }
    };
  }
}
```

---

## Related Documentation

- [Document Loader](./document-loader.md) - Load documents before embedding
- [Text Splitter](./text-splitter.md) - Split documents into embeddable chunks
- [LLM Configuration](./llm-config.md) - Configure LLMs for RAG
