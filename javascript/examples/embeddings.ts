/**
 * Embeddings Example
 *
 * This example demonstrates how to generate embeddings using different providers.
 */

import { init, EmbeddingConfig, EmbeddingClient } from 'graphbit';

async function main() {
  // Initialize the GraphBit library
  init();
  console.log('GraphBit initialized');

  // Example texts to embed
  const texts = [
    'GraphBit is a workflow automation framework.',
    'Machine learning models can process natural language.',
    'The quick brown fox jumps over the lazy dog.',
    'Artificial intelligence is transforming software development.',
  ];

  console.log(`\nGenerating embeddings for ${texts.length} texts...\n`);

  // Example 1: OpenAI Embeddings
  if (process.env.OPENAI_API_KEY) {
    console.log('=== Example 1: OpenAI Embeddings ===');

    const openaiConfig = EmbeddingConfig.openai(
      process.env.OPENAI_API_KEY,
      'text-embedding-3-small'
    );

    const openaiClient = new EmbeddingClient(openaiConfig);

    try {
      const response = await openaiClient.embed(texts);

      console.log('✓ Embeddings generated successfully');
      console.log(`  Model: ${response.model}`);
      console.log(`  Number of embeddings: ${response.embeddings.length}`);
      console.log(`  Embedding dimensions: ${response.embeddings[0]!.length}`);

      if (response.usage) {
        console.log(`  Tokens used: ${response.usage.totalTokens}`);
      }

      // Show similarity between first two texts
      const similarity = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);
      console.log(`  Similarity between text 1 and 2: ${similarity.toFixed(4)}`);
    } catch (error) {
      console.error('Failed to generate OpenAI embeddings:', error);
    }
  } else {
    console.log('=== Example 1: OpenAI Embeddings ===');
    console.log('Skipped: OPENAI_API_KEY not set');
  }

  // Example 2: HuggingFace Embeddings
  if (process.env.HUGGINGFACE_API_KEY) {
    console.log('\n=== Example 2: HuggingFace Embeddings ===');

    const hfConfig = EmbeddingConfig.huggingface(
      process.env.HUGGINGFACE_API_KEY,
      'sentence-transformers/all-MiniLM-L6-v2'
    );

    const hfClient = new EmbeddingClient(hfConfig);

    try {
      const response = await hfClient.embed(texts);

      console.log('✓ Embeddings generated successfully');
      console.log(`  Model: ${response.model}`);
      console.log(`  Number of embeddings: ${response.embeddings.length}`);
      console.log(`  Embedding dimensions: ${response.embeddings[0]!.length}`);
    } catch (error) {
      console.error('Failed to generate HuggingFace embeddings:', error);
    }
  } else {
    console.log('\n=== Example 2: HuggingFace Embeddings ===');
    console.log('Skipped: HUGGINGFACE_API_KEY not set');
  }

  // Example 3: Batch processing
  console.log('\n=== Example 3: Batch Processing ===');

  if (process.env.OPENAI_API_KEY) {
    const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY);
    const client = new EmbeddingClient(config);

    // Process texts in batches
    const batchSize = 2;
    const allEmbeddings: number[][] = [];

    for (let i = 0; i < texts.length; i += batchSize) {
      const batch = texts.slice(i, i + batchSize);
      console.log(`Processing batch ${Math.floor(i / batchSize) + 1}...`);

      try {
        const response = await client.embed(batch);
        allEmbeddings.push(...response.embeddings);
      } catch (error) {
        console.error(`Failed to process batch:`, error);
      }
    }

    console.log(`✓ Processed ${allEmbeddings.length} embeddings in batches`);
  } else {
    console.log('Skipped: OPENAI_API_KEY not set');
  }

  console.log('\n✓ Embeddings examples completed');
}

/**
 * Calculate cosine similarity between two vectors
 */
function cosineSimilarity(a: number[], b: number[]): number {
  if (a.length !== b.length) {
    throw new Error('Vectors must have the same length');
  }

  let dotProduct = 0;
  let normA = 0;
  let normB = 0;

  for (let i = 0; i < a.length; i++) {
    dotProduct += a[i] * b[i];
    normA += a[i] * a[i];
    normB += b[i] * b[i];
  }

  return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
}

// Run the example
main().catch(console.error);
