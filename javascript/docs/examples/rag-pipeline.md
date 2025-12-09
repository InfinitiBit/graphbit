# RAG Pipeline Example

**Level:** Intermediate  
**Estimated Time:** 30 minutes  
**Prerequisites:** OpenAI API key

## Overview

This example demonstrates a complete **Retrieval-Augmented Generation (RAG)** system using GraphBit. You'll learn how to:

1. Load documents from files
2. Split documents into chunks
3. Generate embeddings for semantic search
4. Build a semantic search index
5. Query the index using an AI agent

---

## Prerequisites

### 1. Install GraphBit

```bash
npm install graphbit
```

### 2. Set Environment Variables

```bash
# .env file
OPENAI_API_KEY=your_openai_api_key_here
```

### 3. Install Dependencies

```bash
npm install dotenv
```

---

## Complete Code

```javascript
const { 
  init,
  DocumentLoader,
  TextSplitter,
  EmbeddingConfig,
  EmbeddingClient,
  AgentBuilder,
  LlmConfig
} = require('graphbit');
require('dotenv').config();

/**
 * RAG System Class
 * Handles document ingestion, indexing, and retrieval
 */
class RAGSystem {
  constructor(apiKey) {
    this.apiKey = apiKey;
    this.documents = [];
    this.embeddings = [];
    this.chunks = [];
  }

  /**
   * Step 1: Ingest documents from files
   */
  async ingestDocuments(filePaths) {
    console.log('\n📂 Step 1: Loading documents...');
    
    const loader = new DocumentLoader();
    
    for (const filePath of filePaths) {
      try {
        const doc = await loader.loadFile(filePath, 'txt');
        this.documents.push({
          path: filePath,
          content: doc.content,
          source: doc.source
        });
        console.log(`  ✅ Loaded: ${filePath} (${doc.content.length} chars)`);
      } catch (error) {
        console.error(`  ❌ Failed to load ${filePath}:`, error.message);
      }
    }
    
    console.log(`\n  Total documents loaded: ${this.documents.length}`);
    return this.documents.length;
  }

  /**
   * Step 2: Split documents into chunks
   */
  async splitDocuments() {
    console.log('\n✂️  Step 2: Splitting documents into chunks...');
    
    // Use recursive splitter for natural boundaries
    const splitter = TextSplitter.recursive(800, 100);
    
    for (const doc of this.documents) {
      const docChunks = splitter.split(doc.content);
      
      // Add metadata to each chunk
      docChunks.forEach((chunk, idx) => {
        this.chunks.push({
          text: chunk.content,
          source: doc.path,
          chunkIndex: chunk.chunkIndex,
          position: {
            start: chunk.startIndex,
            end: chunk.endIndex
          }
        });
      });
      
      console.log(`  ✅ ${doc.path}: ${docChunks.length} chunks`);
    }
    
    console.log(`\n  Total chunks created: ${this.chunks.length}`);
    return this.chunks.length;
  }

  /**
   * Step 3: Generate embeddings for all chunks
   */
  async generateEmbeddings() {
    console.log('\n🔢 Step 3: Generating embeddings...');
    
    const config = EmbeddingConfig.openai(this.apiKey, 'text-embedding-3-small');
    const client = new EmbeddingClient(config);
    
    const texts = this.chunks.map(c => c.text);
    
    // Process in batches to avoid rate limits
    const batchSize = 100;
    const allEmbeddings = [];
    
    for (let i = 0; i < texts.length; i += batchSize) {
      const batch = texts.slice(i, i + batchSize);
      const batchNum = Math.floor(i / batchSize) + 1;
      const totalBatches = Math.ceil(texts.length / batchSize);
      
      console.log(`  Processing batch ${batchNum}/${totalBatches}...`);
      
      const response = await client.embed(batch);
      allEmbeddings.push(...response.embeddings);
      
      // Rate limiting delay
      if (i + batchSize < texts.length) {
        await new Promise(resolve => setTimeout(resolve, 500));
      }
    }
    
    this.embeddings = allEmbeddings;
    console.log(`\n  ✅ Generated ${this.embeddings.length} embeddings`);
    console.log(`  Embedding dimension: ${this.embeddings[0].length}`);
    
    return this.embeddings.length;
  }

  /**
   * Step 4: Search for relevant chunks using semantic similarity
   */
  async search(query, topK = 3) {
    console.log(`\n🔍 Searching for: "${query}"`);
    
    // Generate embedding for query
    const config = EmbeddingConfig.openai(this.apiKey);
    const client = new EmbeddingClient(config);
    const queryResponse = await client.embed([query]);
    const queryEmbedding = queryResponse.embeddings[0];
    
    // Calculate cosine similarity for all chunks
    const results = this.embeddings.map((embedding, idx) => ({
      chunk: this.chunks[idx],
      score: this.cosineSimilarity(queryEmbedding, embedding)
    }));
    
    // Sort by similarity and return top K
    results.sort((a, b) => b.score - a.score);
    const topResults = results.slice(0, topK);
    
    console.log('\n📊 Top results:');
    topResults.forEach((result, idx) => {
      console.log(`\n  ${idx + 1}. Score: ${result.score.toFixed(4)}`);
      console.log(`     Source: ${result.chunk.source}`);
      console.log(`     Text: ${result.chunk.text.substring(0, 100)}...`);
    });
    
    return topResults;
  }

  /**
   * Step 5: Generate answer using retrieved context
   */
  async generateAnswer(query, context) {
    console.log('\n🤖 Generating answer with AI...');
    
    const llmConfig = LlmConfig.openai({
      apiKey: this.apiKey,
      model: 'gpt-4o-mini'
    });
    
    const agent = await new AgentBuilder('RAG Assistant', llmConfig)
      .description('Answers questions based on provided context')
      .systemPrompt(`You are a helpful assistant. Answer questions based ONLY on the provided context. 
        If the context doesn't contain the answer, say "I don't have enough information to answer that."`)
      .temperature(0.3)
      .maxTokens(500)
      .build();
    
    // Construct prompt with context
    const contextText = context
      .map((c, idx) => `[${idx + 1}] ${c.chunk.text}`)
      .join('\n\n');
    
    const prompt = `Context:\n${contextText}\n\nQuestion: ${query}\n\nAnswer:`;
    
    const answer = await agent.execute(prompt);
    
    console.log('\n💬 Answer:');
    console.log(`  ${answer}\n`);
    
    return answer;
  }

  /**
   * Helper: Calculate cosine similarity
   */
  cosineSimilarity(a, b) {
    const dotProduct = a.reduce((sum, val, i) => sum + val * b[i], 0);
    const magnitudeA = Math.sqrt(a.reduce((sum, val) => sum + val * val, 0));
    const magnitudeB = Math.sqrt(b.reduce((sum, val) => sum + val * val, 0));
    return dotProduct / (magnitudeA * magnitudeB);
  }

  /**
   * Complete RAG Query Pipeline
   */
  async query(question, topK = 3) {
    const relevantChunks = await this.search(question, topK);
    const answer = await this.generateAnswer(question, relevantChunks);
    
    return {
      question,
      answer,
      sources: relevantChunks.map(r => ({
        source: r.chunk.source,
        text: r.chunk.text,
        score: r.score
      }))
    };
  }
}

/**
 * Main execution
 */
async function main() {
  console.log('🚀 GraphBit RAG Pipeline Example\n');
  console.log('================================\n');
  
  // Initialize GraphBit
  init();
  
  // Create RAG system
  const rag = new RAGSystem(process.env.OPENAI_API_KEY);
  
  // Sample documents (you would use real file paths)
  const documents = [
    './docs/guide.txt',
    './docs/faq.txt',
    './docs/api.txt'
  ];
  
  // Build the index
  await rag.ingestDocuments(documents);
  await rag.splitDocuments();
  await rag.generateEmbeddings();
  
  console.log('\n✅ RAG system ready!\n');
  console.log('================================\n');
  
  // Example queries
  const queries = [
    'How do I install GraphBit?',
    'What are the main features?',
    'How do I create an agent?'
  ];
  
  for (const query of queries) {
    const result = await rag.query(query, 3);
    
    console.log('\n' + '='.repeat(50));
    console.log(`Question: ${result.question}`);
    console.log(`Answer: ${result.answer}`);
    console.log('\nSources:');
    result.sources.forEach((source, idx) => {
      console.log(`  ${idx + 1}. ${source.source} (score: ${source.score.toFixed(4)})`);
    });
    console.log('='.repeat(50) + '\n');
    
    // Rate limiting between queries
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
}

// Run the example
main().catch(console.error);
```

---

## Walkthrough

### Step 1: Document Loading

The `DocumentLoader` reads files from disk and extracts their text content.

```javascript
const loader = new DocumentLoader();
const doc = await loader.loadFile(filePath, 'txt');
```

### Step 2: Text Splitting

Documents are split into manageable chunks using a recursive splitter that preserves paragraph boundaries.

```javascript
const splitter = TextSplitter.recursive(800, 100);
const chunks = splitter.split(doc.content);
```

**Why 800/100?**

- 800 chars: Optimal chunk size for embeddings
- 100 chars: Overlap to preserve context across chunks

### Step 3: Embedding Generation

Each chunk is converted into a vector representation using OpenAI's embedding model.

```javascript
const config = EmbeddingConfig.openai(apiKey, 'text-embedding-3-small');
const client = new EmbeddingClient(config);
const response = await client.embed(texts);
```

**Batching:** Process 100 chunks at a time to respect API rate limits.

### Step 4: Semantic Search

When a query comes in, we:

1. Generate an embedding for the query
2. Calculate cosine similarity with all chunk embeddings
3. Return the top K most similar chunks

```javascript
const queryEmbedding = await client.embed([query]);
const similarities = embeddings.map((emb, idx) => ({
  chunk: chunks[idx],
  score: cosineSimilarity(queryEmbedding[0], emb)
}));
```

### Step 5: Answer Generation

The retrieved chunks are used as context for an AI agent to generate an accurate answer.

```javascript
const agent = await new AgentBuilder('RAG Assistant', llmConfig)
  .systemPrompt('Answer based on provided context only')
  .temperature(0.3)  // Low temperature for factual responses
  .build();

const answer = await agent.execute(promptWithContext);
```

---

## Running the Example

### 1. Prepare Documents

Create sample text files in a `docs/` directory:

```bash
mkdir docs
echo "GraphBit is installed via npm install graphbit" > docs/guide.txt
echo "Main features include agents, workflows, and embeddings" > docs/faq.txt
echo "Create agents with AgentBuilder class" > docs/api.txt
```

### 2. Run the Script

```bash
node rag-pipeline-example.js
```

### 3. Expected Output

```
🚀 GraphBit RAG Pipeline Example

================================

📂 Step 1: Loading documents...
  ✅ Loaded: ./docs/guide.txt (47 chars)
  ✅ Loaded: ./docs/faq.txt (58 chars)
  ✅ Loaded: ./docs/api.txt (37 chars)

  Total documents loaded: 3

✂️  Step 2: Splitting documents into chunks...
  ✅ ./docs/guide.txt: 1 chunks
  ✅ ./docs/faq.txt: 1 chunks
  ✅ ./docs/api.txt: 1 chunks

  Total chunks created: 3

🔢 Step 3: Generating embeddings...
  Processing batch 1/1...

  ✅ Generated 3 embeddings
  Embedding dimension: 1536

✅ RAG system ready!

================================

🔍 Searching for: "How do I install GraphBit?"

📊 Top results:

  1. Score: 0.9234
     Source: ./docs/guide.txt
     Text: GraphBit is installed via npm install graphbit

🤖 Generating answer with AI...

💬 Answer:
  GraphBit is installed using npm with the command: npm install graphbit

==================================================
Question: How do I install GraphBit?
Answer: GraphBit is installed using npm with the command: npm install graphbit

Sources:
  1. ./docs/guide.txt (score: 0.9234)
==================================================
```

---

## Customization

### Use Different Embedding Models

```javascript
// Use larger model for better quality
const config = EmbeddingConfig.openai(apiKey, 'text-embedding-3-large');

// Or use HuggingFace
const config = EmbeddingConfig.huggingface(
  hfApiKey,
  'sentence-transformers/all-MiniLM-L6-v2'
);
```

### Adjust Chunk Size

```javascript
// Smaller chunks for more precise retrieval
const splitter = TextSplitter.recursive(400, 50);

// Larger chunks for more context
const splitter = TextSplitter.recursive(1500, 200);
```

### Change Number of Retrieved Chunks

```javascript
// Retrieve more context
const result = await rag.query(question, 5);  // Top 5 instead of 3
```

### Use Different LLM

```javascript
const llmConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
  model: 'claude-3-5-sonnet-20241022'
});
```

---

## Troubleshooting

### Issue: Rate Limit Errors

**Solution:** Increase delay between batches

```javascript
// Increase from 500ms to 2000ms
await new Promise(resolve => setTimeout(resolve, 2000));
```

### Issue: Low Similarity Scores

**Possible Causes:**

- Query and documents use different terminology
- Chunk size too small/large
- Wrong embedding model

**Solution:** Try different chunk sizes or use `text-embedding-3-large`

### Issue: Agent Hallucinating

**Solution:** Lower temperature and strengthen system prompt

```javascript
.systemPrompt(`CRITICAL: Answer ONLY using the provided context. 
  Never make up information. If unsure, say "I don't know".`)
.temperature(0.1)  // Very low for factual responses
```

---

## Performance Optimization

### 1. Cache Embeddings

```javascript
const fs = require('fs').promises;

// Save embeddings
await fs.writeFile(
  'embeddings_cache.json',
  JSON.stringify({ chunks: this.chunks, embeddings: this.embeddings })
);

// Load embeddings
const cache = JSON.parse(await fs.readFile('embeddings_cache.json'));
this.chunks = cache.chunks;
this.embeddings = cache.embeddings;
```

### 2. Use a Vector Database

For production, use Pinecone, Weaviate, or Qdrant instead of in-memory storage.

### 3. Parallel Processing

```javascript
// Process multiple queries in parallel
const results = await Promise.all(
  queries.map(q => rag.query(q))
);
```

---

## Next Steps

1. **Add More Documents:** Scale to hundreds/thousands of documents
2. **Implement Filtering:** Add metadata filtering (by date, category, etc.)
3. **Hybrid Search:** Combine semantic search with keyword search
4. **Reranking:** Add a reranking step for better results
5. **Streaming Responses:** Stream agent responses for better UX

---

## Related Examples

- [Multi-Agent System](./multi-agent-system.md) - Coordinate multiple agents
- [Error Handling](./error-handling.md) - Production-grade error handling
- [Production Deployment](./production-deployment.md) - Deploy to production

---

**Example Created:** 2025-12-05  
**GraphBit Version:** 0.5.1  
**Difficulty:** Intermediate
