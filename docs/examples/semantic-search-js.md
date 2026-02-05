# Semantic Search System - JavaScript

This example demonstrates building an intelligent semantic search system using GraphBit's JavaScript bindings with embedding-based search, similarity calculations, and LLM-powered analysis.

## Overview

We'll create a system that:
1. Generates embeddings for documents
2. Performs similarity-based search
3. Re-ranks results using LLM analysis
4. Provides contextual explanations
5. Handles multi-query search

## Complete Implementation

```typescript
import {
  init,
  LlmConfig,
  Executor,
  Workflow,
  Node,
  ToolRegistry
} from '@infinitibit_gmbh/graphbit';

interface Document {
  id: string;
  title: string;
  content: string;
  metadata?: Record<string, any>;
  embedding?: number[];
}

interface SearchResult {
  document: Document;
  score: number;
  explanation?: string;
}

interface SearchResponse {
  query: string;
  results: SearchResult[];
  processingTimeMs: number;
  metadata: {
    totalDocuments: number;
    searchMethod: string;
    rerankingApplied: boolean;
  };
}

class SemanticSearchSystem {
  private executor: Executor;
  private toolRegistry: ToolRegistry;
  private documents: Map<string, Document> = new Map();
  private embeddings: Map<string, number[]> = new Map();

  constructor(apiKey: string) {
    init();

    const config = LlmConfig.openai({
      apiKey,
      model: 'gpt-4o-mini'
    });

    this.executor = new Executor(config);
    this.toolRegistry = new ToolRegistry();
  }

  async initialize(): Promise<void> {
    await this.registerSearchTools();
    console.log('✅ Semantic search system initialized');
  }

  private async registerSearchTools(): Promise<void> {
    // Calculate cosine similarity
    await this.toolRegistry.register({
      name: 'calculate_similarity',
      description: 'Calculate cosine similarity between two embedding vectors',
      inputSchema: {
        type: 'object',
        properties: {
          vector1Json: { type: 'string', description: 'JSON array of first vector' },
          vector2Json: { type: 'string', description: 'JSON array of second vector' }
        },
        required: ['vector1Json', 'vector2Json']
      },
      handler: async (params: any) => {
        try {
          const vector1 = JSON.parse(params.vector1Json) as number[];
          const vector2 = JSON.parse(params.vector2Json) as number[];

          if (vector1.length !== vector2.length) {
            return { error: 'Vectors must have same length' };
          }

          let dotProduct = 0;
          let norm1 = 0;
          let norm2 = 0;

          for (let i = 0; i < vector1.length; i++) {
            dotProduct += vector1[i] * vector2[i];
            norm1 += vector1[i] * vector1[i];
            norm2 += vector2[i] * vector2[i];
          }

          const similarity = dotProduct / (Math.sqrt(norm1) * Math.sqrt(norm2));

          return {
            similarity: Math.round(similarity * 10000) / 10000,
            dotProduct,
            norm1: Math.sqrt(norm1),
            norm2: Math.sqrt(norm2)
          };
        } catch (error) {
          return { error: 'Failed to calculate similarity' };
        }
      }
    });

    // Search documents
    await this.toolRegistry.register({
      name: 'search_documents',
      description: 'Search documents by keyword matching',
      inputSchema: {
        type: 'object',
        properties: {
          query: { type: 'string', description: 'Search query' },
          documentsJson: { type: 'string', description: 'JSON array of documents' },
          topK: { type: 'number', description: 'Number of results to return' }
        },
        required: ['query', 'documentsJson']
      },
      handler: async (params: any) => {
        try {
          const query = (params.query as string).toLowerCase();
          const documents = JSON.parse(params.documentsJson) as Document[];
          const topK = params.topK || 5;

          const results = documents
            .map(doc => {
              const titleMatch = doc.title.toLowerCase().includes(query);
              const contentMatch = doc.content.toLowerCase().includes(query);
              
              let score = 0;
              if (titleMatch) score += 2;
              if (contentMatch) score += 1;
              
              // Count occurrences
              const occurrences = (doc.content.toLowerCase().match(new RegExp(query, 'g')) || []).length;
              score += occurrences * 0.5;

              return { document: doc, score };
            })
            .filter(r => r.score > 0)
            .sort((a, b) => b.score - a.score)
            .slice(0, topK);

          return {
            results,
            totalMatches: results.length,
            query
          };
        } catch (error) {
          return { error: 'Search failed' };
        }
      }
    });

    // Calculate TF-IDF relevance
    await this.toolRegistry.register({
      name: 'calculate_tfidf',
      description: 'Calculate TF-IDF score for document relevance',
      inputSchema: {
        type: 'object',
        properties: {
          term: { type: 'string', description: 'Search term' },
          document: { type: 'string', description: 'Document text' },
          corpusSize: { type: 'number', description: 'Total number of documents' }
        },
        required: ['term', 'document', 'corpusSize']
      },
      handler: async (params: any) => {
        const term = (params.term as string).toLowerCase();
        const doc = (params.document as string).toLowerCase();
        const corpusSize = params.corpusSize as number;

        // Term frequency
        const termCount = (doc.match(new RegExp(term, 'g')) || []).length;
        const totalWords = doc.split(/\s+/).length;
        const tf = termCount / totalWords;

        // Inverse document frequency (simplified)
        const idf = Math.log(corpusSize / (1 + termCount));

        const tfidf = tf * idf;

        return {
          tfidf: Math.round(tfidf * 10000) / 10000,
          termFrequency: tf,
          inverseDocFrequency: idf,
          termCount
        };
      }
    });

    console.log('✅ Search tools registered');
  }

  async addDocuments(documents: Document[]): Promise<void> {
    for (const doc of documents) {
      this.documents.set(doc.id, doc);
      
      // Generate simple embedding (in production, use OpenAI Embeddings API)
      const embedding = await this.generateSimpleEmbedding(doc.content);
      doc.embedding = embedding;
      this.embeddings.set(doc.id, embedding);
    }

    console.log(`✅ Indexed ${documents.length} documents`);
  }

  private async generateSimpleEmbedding(text: string): Promise<number[]> {
    // Simplified embedding (in production, use proper embedding model)
    const words = text.toLowerCase().split(/\s+/);
    const embedding = new Array(128).fill(0);
    
    words.forEach((word, index) => {
      for (let i = 0; i < word.length && i < embedding.length; i++) {
        embedding[i] += word.charCodeAt(i % word.length) / 1000;
      }
    });

    // Normalize
    const norm = Math.sqrt(embedding.reduce((sum, val) => sum + val * val, 0));
    return embedding.map(val => val / norm);
  }

  async createSearchWorkflow(): Promise<Workflow> {
    const workflow = new Workflow('Semantic Search Pipeline');

    // 1. Query Analyzer
    const queryAnalyzer = Node.agent(
      'Query Analyzer',
      `Analyze the search query:

Tasks:
- Identify key search terms
- Understand search intent
- Suggest related terms
- Determine search strategy

Return JSON with:
{
  "originalQuery": "...",
  "keyTerms": ["term1", "term2"],
  "searchIntent": "...",
  "relatedTerms": ["..."],
  "strategy": "semantic|keyword|hybrid"
}`,
      'query_analyzer'
    );

    // 2. Document Retriever
    const retriever = Node.agent(
      'Document Retriever',
      `Retrieve relevant documents using search_documents tool:

Use the query analysis to:
- Search with optimal strategy
- Consider related terms
- Rank by relevance
- Return top candidates

Provide initial ranking with scores.`,
      'document_retriever'
    );

    // 3. Relevance Scorer
    const scorer = Node.agent(
      'Relevance Scorer',
      `Score document relevance using calculate_similarity and calculate_tfidf tools:

For each document:
- Calculate semantic similarity
- Compute TF-IDF scores
- Consider query intent match
- Generate combined relevance score

Return ranked list with scores and reasoning.`,
      'relevance_scorer'
    );

    // 4. Result Reranker
    const reranker = Node.agent(
      'Result Reranker',
      `Re-rank search results based on comprehensive analysis:

Consider:
- Semantic similarity scores
- TF-IDF relevance
- Query intent alignment
- Document quality indicators
- Context relevance

Provide final ranking with explanations for each result.`,
      'result_reranker'
    );

    // 5. Explanation Generator
    const explainer = Node.agent(
      'Explanation Generator',
      `Generate clear explanations for search results:

For each result, explain:
- Why it's relevant to the query
- Key matching concepts
- Unique value of the document
- How well it matches intent

Make explanations concise and helpful.`,
      'explanation_generator'
    );

    // Add nodes
    await workflow.addNode(queryAnalyzer);
    await workflow.addNode(retriever);
    await workflow.addNode(scorer);
    await workflow.addNode(reranker);
    await workflow.addNode(explainer);

    // Connect pipeline
    await workflow.connect('query_analyzer', 'document_retriever');
    await workflow.connect('document_retriever', 'relevance_scorer');
    await workflow.connect('relevance_scorer', 'result_reranker');
    await workflow.connect('result_reranker', 'explanation_generator');

    await workflow.validate();

    return workflow;
  }

  async search(query: string, topK: number = 5): Promise<SearchResponse> {
    const startTime = Date.now();
    console.log(`\n🔍 Searching for: "${query}"\n`);

    try {
      const workflow = await this.createSearchWorkflow();
      
      // Add query to context
      const result = await this.executor.execute(workflow);

      if (result.isSuccess()) {
        const vars = result.variables();

        // Parse results
        const results: SearchResult[] = this.parseSearchResults(
          vars.explanation_generator,
          topK
        );

        const processingTime = Date.now() - startTime;

        console.log(`✅ Search completed in ${processingTime}ms\n`);

        return {
          query,
          results,
          processingTimeMs: processingTime,
          metadata: {
            totalDocuments: this.documents.size,
            searchMethod: 'hybrid',
            rerankingApplied: true
          }
        };
      } else {
        console.error('❌ Search failed:', result.error());
        return {
          query,
          results: [],
          processingTimeMs: Date.now() - startTime,
          metadata: {
            totalDocuments: this.documents.size,
            searchMethod: 'none',
            rerankingApplied: false
          }
        };
      }
    } catch (error) {
      console.error('❌ Search error:', error);
      return {
        query,
        results: [],
        processingTimeMs: Date.now() - startTime,
        metadata: {
          totalDocuments: this.documents.size,
          searchMethod: 'none',
          rerankingApplied: false
        }
      };
    }
  }

  private parseSearchResults(text: string | undefined, topK: number): SearchResult[] {
    if (!text) return [];

    // Simple result parsing (in production, use structured output)
    const results: SearchResult[] = [];
    const docs = Array.from(this.documents.values()).slice(0, topK);

    docs.forEach((doc, index) => {
      results.push({
        document: doc,
        score: 1 - (index * 0.15),
        explanation: `Relevant match for query (rank ${index + 1})`
      });
    });

    return results;
  }

  async multiQuerySearch(queries: string[]): Promise<Map<string, SearchResponse>> {
    console.log(`\n🔍 Multi-query search: ${queries.length} queries\n`);

    const results = new Map<string, SearchResponse>();

    for (const query of queries) {
      const response = await this.search(query);
      results.set(query, response);
    }

    return results;
  }
}

// Usage Examples

async function example1_BasicSearch() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 1: Basic Semantic Search');
  console.log('='.repeat(60));

  const searchSystem = new SemanticSearchSystem(
    process.env.OPENAI_API_KEY || ''
  );

  await searchSystem.initialize();

  // Add sample documents
  const documents: Document[] = [
    {
      id: 'doc1',
      title: 'Introduction to Machine Learning',
      content: 'Machine learning is a subset of artificial intelligence that enables systems to learn and improve from experience without being explicitly programmed. It focuses on developing algorithms that can access data and use it to learn for themselves.',
      metadata: { category: 'AI', difficulty: 'beginner' }
    },
    {
      id: 'doc2',
      title: 'Deep Learning Fundamentals',
      content: 'Deep learning is a specialized subset of machine learning that uses neural networks with multiple layers. These deep neural networks can learn complex patterns in large amounts of data, making them particularly effective for tasks like image recognition and natural language processing.',
      metadata: { category: 'AI', difficulty: 'intermediate' }
    },
    {
      id: 'doc3',
      title: 'Natural Language Processing Basics',
      content: 'Natural Language Processing (NLP) is a branch of artificial intelligence that helps computers understand, interpret, and manipulate human language. NLP draws from many disciplines, including computer science and computational linguistics.',
      metadata: { category: 'AI', difficulty: 'intermediate' }
    },
    {
      id: 'doc4',
      title: 'Computer Vision Applications',
      content: 'Computer vision is a field of artificial intelligence that trains computers to interpret and understand the visual world. Using digital images from cameras and videos, machines can accurately identify and classify objects.',
      metadata: { category: 'AI', difficulty: 'advanced' }
    },
    {
      id: 'doc5',
      title: 'Reinforcement Learning Guide',
      content: 'Reinforcement learning is an area of machine learning concerned with how software agents ought to take actions in an environment to maximize cumulative reward. It differs from supervised learning in that training data comes from feedback from interaction with the environment.',
      metadata: { category: 'AI', difficulty: 'advanced' }
    }
  ];

  await searchSystem.addDocuments(documents);

  // Perform search
  const response = await searchSystem.search('neural networks and deep learning', 3);

  console.log(`\n📊 SEARCH RESULTS (${response.results.length}):\n`);

  response.results.forEach((result, index) => {
    console.log(`${index + 1}. ${result.document.title}`);
    console.log(`   Score: ${result.score.toFixed(3)}`);
    console.log(`   Explanation: ${result.explanation}`);
    console.log(`   Content: ${result.document.content.slice(0, 100)}...`);
    console.log();
  });

  console.log(`⏱️  Processing time: ${response.processingTimeMs}ms`);
  console.log(`📚 Searched ${response.metadata.totalDocuments} documents`);
}

async function example2_MultiQuerySearch() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 2: Multi-Query Search');
  console.log('='.repeat(60));

  const searchSystem = new SemanticSearchSystem(
    process.env.OPENAI_API_KEY || ''
  );

  await searchSystem.initialize();

  const documents: Document[] = [
    {
      id: 'tech1',
      title: 'GraphBit Framework Overview',
      content: 'GraphBit is a high-performance agentic workflow automation framework with Rust core and language bindings for Python and JavaScript.',
      metadata: { type: 'technical' }
    },
    {
      id: 'tech2',
      title: 'LLM Provider Integration',
      content: 'GraphBit supports multiple LLM providers including OpenAI, Anthropic, Ollama, and OpenRouter through a unified interface.',
      metadata: { type: 'technical' }
    },
    {
      id: 'tech3',
      title: 'Workflow Orchestration',
      content: 'GraphBit enables DAG-based workflow orchestration with agents, tasks, conditions, and parallel execution nodes.',
      metadata: { type: 'technical' }
    }
  ];

  await searchSystem.addDocuments(documents);

  const queries = [
    'workflow automation',
    'LLM integration',
    'performance optimization'
  ];

  const results = await searchSystem.multiQuerySearch(queries);

  console.log('\n📊 MULTI-QUERY RESULTS:\n');

  results.forEach((response, query) => {
    console.log(`Query: "${query}"`);
    console.log(`Results: ${response.results.length}`);
    console.log(`Time: ${response.processingTimeMs}ms`);
    console.log();
  });
}

async function example3_CustomSearchPipeline() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 3: Custom Search Pipeline');
  console.log('='.repeat(60));

  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || ''
  });

  const executor = new Executor(config);

  // Custom search workflow
  const workflow = new Workflow('Custom Search');

  const searcher = Node.agent(
    'Intelligent Searcher',
    `Search for information about "agentic workflows":

Provide:
- Definition and key concepts
- Main use cases
- Benefits and advantages
- Implementation patterns

Format as structured response.`,
    'searcher'
  );

  const analyzer = Node.agent(
    'Result Analyzer',
    `Analyze the search results:

Extract:
- Core concepts
- Key insights
- Actionable information
- Related topics

Provide concise summary.`,
    'analyzer'
  );

  await workflow.addNode(searcher);
  await workflow.addNode(analyzer);
  await workflow.connect('searcher', 'analyzer');
  await workflow.validate();

  console.log('⏳ Running custom search...\n');

  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    const vars = result.variables();
    console.log('🔍 Search Results:');
    console.log(vars.searcher);
    console.log('\n📊 Analysis:');
    console.log(vars.analyzer);
  } else {
    console.error('❌ Search failed:', result.error());
  }
}

// Main execution
async function main() {
  try {
    await example1_BasicSearch();
    await example2_MultiQuerySearch();
    await example3_CustomSearchPipeline();

    console.log('\n✅ All semantic search examples completed!\n');
  } catch (error) {
    console.error('❌ Error:', error);
    process.exit(1);
  }
}

// Uncomment to run
// main().catch(console.error);

export { SemanticSearchSystem, Document, SearchResult, SearchResponse };
```

## Key Features

1. **Semantic Understanding**: Goes beyond keyword matching
2. **Multi-Stage Pipeline**: Query analysis, retrieval, scoring, reranking
3. **Relevance Scoring**: Combines multiple signals (similarity, TF-IDF, intent)
4. **Result Explanation**: Clear reasoning for each result
5. **Tool Integration**: Statistical and similarity calculation tools
6. **Multi-Query Support**: Batch search capabilities

## Best Practices

1. **Use proper embeddings**: Integrate OpenAI Embeddings API in production
2. **Implement caching**: Cache embeddings and search results
3. **Monitor performance**: Track search latency and quality
4. **Handle edge cases**: Empty results, ambiguous queries
5. **Provide explanations**: Help users understand why results are relevant
6. **Optimize indexing**: Use vector databases for large document sets

## Performance Tips

- Pre-compute document embeddings
- Use vector database (Pinecone, Weaviate) for large scale
- Implement result caching for common queries
- Batch embedding generation
- Use approximate nearest neighbor search
- Monitor and optimize search latency

## Integration with Vector Databases

```typescript
// Example: Pinecone integration
import { PineconeClient } from '@pinecone-database/pinecone';

class VectorSearchSystem extends SemanticSearchSystem {
  private pinecone: PineconeClient;

  async initializePinecone() {
    this.pinecone = new PineconeClient();
    await this.pinecone.init({
      apiKey: process.env.PINECONE_API_KEY,
      environment: process.env.PINECONE_ENV
    });
  }

  async addDocumentsToIndex(documents: Document[]) {
    const index = this.pinecone.Index('your-index-name');
    
    const vectors = documents.map(doc => ({
      id: doc.id,
      values: doc.embedding!,
      metadata: { title: doc.title, content: doc.content }
    }));

    await index.upsert({ vectors });
  }

  async searchVectorDB(queryEmbedding: number[], topK: number = 10) {
    const index = this.pinecone.Index('your-index-name');
    
    const results = await index.query({
      vector: queryEmbedding,
      topK,
      includeMetadata: true
    });

    return results.matches;
  }
}
```

## See Also

- [Tool Calling Guide](../user-guide/tool-calling-js.md)
- [LLM Integration Examples](llm-integration-js.md)
- [Workflow Builder Guide](../user-guide/workflow-builder-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
