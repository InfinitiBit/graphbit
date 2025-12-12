/**
 * Embedding Similarity Demo
 * 
 * Demonstrates the new similarity() method for comparing embeddings.
 * Shows practical use cases like semantic search and clustering.
 *
 * Run with: npx ts-node examples/embedding-similarity-demo.ts
 */

import { EmbeddingClient, EmbeddingConfig } from '../index';

// Check for API key
if (!process.env.OPENAI_API_KEY) {
  console.error('Error: OPENAI_API_KEY environment variable is required');
  console.error('Set it with: export OPENAI_API_KEY=your-key-here');
  process.exit(1);
}

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Embedding Similarity Demo');
  console.log('='.repeat(70));
  console.log();

  // Create embedding client
  console.log('📝 Creating embedding client...');
  const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!);
  const client = new EmbeddingClient(config);
  console.log('✅ Client created!\n');

  // Example 1: Basic Similarity
  console.log('Example 1: Basic Similarity Calculation');
  console.log('-'.repeat(70));
  const texts1 = ['cat', 'dog', 'car'];
  console.log('Generating embeddings for:', texts1);
  
  const response1 = await client.embed(texts1);
  const catEmb = response1.embeddings[0];
  const dogEmb = response1.embeddings[1];
  const carEmb = response1.embeddings[2];
  
  const catDogSim = EmbeddingClient.similarity(catEmb, dogEmb);
  const catCarSim = EmbeddingClient.similarity(catEmb, carEmb);
  const dogCarSim = EmbeddingClient.similarity(dogEmb, carEmb);
  
  console.log(`\n📊 Similarity Scores:`);
  console.log(`  Cat ↔ Dog: ${catDogSim.toFixed(4)} (animals - should be high)`);
  console.log(`  Cat ↔ Car: ${catCarSim.toFixed(4)} (unrelated - should be low)`);
  console.log(`  Dog ↔ Car: ${dogCarSim.toFixed(4)} (unrelated - should be low)`);
  console.log();

  // Example 2: Semantic Search
  console.log('Example 2: Semantic Search');
  console.log('-'.repeat(70));
  const query = 'What is artificial intelligence?';
  const documents = [
    'AI is the simulation of human intelligence by machines',
    'The weather is sunny today',
    'Machine learning is a subset of AI',
    'I like to eat pizza',
    'Neural networks are inspired by the human brain'
  ];
  
  console.log(`Query: "${query}"`);
  console.log(`\nSearching through ${documents.length} documents...\n`);
  
  // Generate embeddings
  const allTexts = [query, ...documents];
  const response2 = await client.embed(allTexts);
  const queryEmb = response2.embeddings[0];
  const docEmbs = response2.embeddings.slice(1);
  
  // Calculate similarities and rank
  const results = documents.map((doc, idx) => ({
    document: doc,
    score: EmbeddingClient.similarity(queryEmb, docEmbs[idx])
  }));
  
  results.sort((a, b) => b.score - a.score);
  
  console.log('📊 Search Results (ranked by relevance):');
  results.forEach((result, idx) => {
    const bar = '█'.repeat(Math.round(result.score * 20));
    console.log(`\n${idx + 1}. Score: ${result.score.toFixed(4)} ${bar}`);
    console.log(`   "${result.document}"`);
  });
  console.log();

  // Example 3: Clustering
  console.log('Example 3: Clustering Similar Items');
  console.log('-'.repeat(70));
  const items = [
    'apple', 'banana', 'orange',      // Fruits
    'car', 'truck', 'bus',             // Vehicles
    'dog', 'cat', 'hamster'            // Animals
  ];
  
  console.log('Items to cluster:', items.join(', '));
  console.log('\nGenerating embeddings...');
  
  const response3 = await client.embed(items);
  
  console.log('\n📊 Similarity Matrix:\n');
  console.log('         ', items.map(i => i.slice(0, 6).padEnd(6)).join(' '));
  
  for (let i = 0; i < items.length; i++) {
    const row = items[i].slice(0, 8).padEnd(8);
    const sims = items.map((_, j) => {
      if (i === j) return '1.0000';
      const sim = EmbeddingClient.similarity(
        response3.embeddings[i],
        response3.embeddings[j]
      );
      return sim.toFixed(4);
    });
    console.log(row, sims.join(' '));
  }
  console.log();

  // Example 4: Duplicate Detection
  console.log('Example 4: Duplicate Detection');
  console.log('-'.repeat(70));
  const texts4 = [
    'The quick brown fox jumps over the lazy dog',
    'A fast brown fox leaps over a sleepy dog',  // Similar
    'The weather is nice today',                  // Different
    'The quick brown fox jumps over the lazy dog' // Exact duplicate
  ];
  
  console.log('Detecting duplicates and near-duplicates...\n');
  const response4 = await client.embed(texts4);
  
  const threshold = 0.95;
  console.log(`Using similarity threshold: ${threshold}\n`);
  
  for (let i = 0; i < texts4.length; i++) {
    for (let j = i + 1; j < texts4.length; j++) {
      const sim = EmbeddingClient.similarity(
        response4.embeddings[i],
        response4.embeddings[j]
      );
      
      if (sim > threshold) {
        console.log(`🔍 Similar pair found (${sim.toFixed(4)}):`);
        console.log(`   [${i}] "${texts4[i]}"`);
        console.log(`   [${j}] "${texts4[j]}"`);
        console.log();
      }
    }
  }

  // Example 5: Recommendation System
  console.log('Example 5: Simple Recommendation System');
  console.log('-'.repeat(70));
  const userLikes = ['science fiction movies', 'space exploration'];
  const recommendations = [
    'Star Wars: A New Hope',
    'The Martian',
    'Romantic comedy film',
    'Interstellar',
    'Cooking recipes',
    'Apollo 13'
  ];
  
  console.log('User likes:', userLikes.join(', '));
  console.log(`\nFinding recommendations from ${recommendations.length} items...\n`);
  
  const response5 = await client.embed([...userLikes, ...recommendations]);
  const userEmbs = response5.embeddings.slice(0, userLikes.length);
  const recEmbs = response5.embeddings.slice(userLikes.length);
  
  // Calculate average similarity to user preferences
  const scores = recommendations.map((rec, idx) => {
    const sims = userEmbs.map(userEmb => 
      EmbeddingClient.similarity(userEmb, recEmbs[idx])
    );
    const avgSim = sims.reduce((a, b) => a + b, 0) / sims.length;
    return { recommendation: rec, score: avgSim };
  });
  
  scores.sort((a, b) => b.score - a.score);
  
  console.log('📊 Recommendations (ranked):');
  scores.forEach((item, idx) => {
    const stars = '⭐'.repeat(Math.round(item.score * 5));
    console.log(`${idx + 1}. ${stars} (${item.score.toFixed(4)})`);
    console.log(`   "${item.recommendation}"`);
  });
  console.log();

  // Example 6: Edge Cases
  console.log('Example 6: Edge Cases & Validation');
  console.log('-'.repeat(70));
  
  console.log('Testing error handling...\n');
  
  try {
    console.log('1. Empty embeddings:');
    EmbeddingClient.similarity([], [1, 2, 3]);
  } catch (error) {
    console.log(`   ✅ Caught: ${error}`);
  }
  
  try {
    console.log('2. Mismatched lengths:');
    EmbeddingClient.similarity([1, 2], [1, 2, 3]);
  } catch (error) {
    console.log(`   ✅ Caught: ${error}`);
  }
  
  console.log('3. High-dimensional vectors (1536d):');
  const highDim1 = Array(1536).fill(0).map(() => Math.random());
  const highDim2 = Array(1536).fill(0).map(() => Math.random());
  const highDimSim = EmbeddingClient.similarity(highDim1, highDim2);
  console.log(`   ✅ Calculated: ${highDimSim.toFixed(4)}`);
  console.log();

  console.log('='.repeat(70));
  console.log(' Summary');
  console.log('='.repeat(70));
  console.log();
  console.log('✅ Similarity Method Features:');
  console.log('   - Cosine similarity calculation');
  console.log('   - Returns score between -1.0 and 1.0');
  console.log('   - Input validation');
  console.log('   - Handles high-dimensional vectors');
  console.log('   - Static method (no instance needed)');
  console.log();
  console.log('💡 Use Cases:');
  console.log('   - Semantic search');
  console.log('   - Document clustering');
  console.log('   - Duplicate detection');
  console.log('   - Recommendation systems');
  console.log('   - Content similarity');
  console.log();
  console.log('📚 For more information:');
  console.log('   - See tests/unit/embedding-client-enhanced.test.ts');
  console.log('   - Check REMEDIATION_PLAN.md Phase 2, Task 2.1');
  console.log();
}

main().catch(console.error);

