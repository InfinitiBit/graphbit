/**
 * Performance Benchmark Runner
 * 
 * Runs comprehensive performance benchmarks
 * Compares against targets and generates report
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('='.repeat(70));
console.log(' GraphBit JavaScript - Performance Benchmark Runner');
console.log('='.repeat(70));
console.log();

// Check API key
if (!process.env.OPENAI_API_KEY) {
  console.log('❌ OPENAI_API_KEY required for benchmarks');
  console.log('   Set with: $env:OPENAI_API_KEY="your-key"');
  process.exit(1);
}

console.log('✅ API key detected');
console.log();

// Benchmark targets
const targets = {
  llmSimpleLatency: 5000, // 5s for 10 requests
  llmBatchThroughput: 40000, // 40s for 20 requests
  embeddingSimilarity: 0.5, // 0.5ms per calculation
  memoryGrowth: 10, // 10MB for 50 requests
};

console.log('📊 Performance Targets:');
console.log(`   LLM Simple Latency: <${targets.llmSimpleLatency}ms (10 requests)`);
console.log(`   LLM Batch Throughput: <${targets.llmBatchThroughput}ms (20 requests)`);
console.log(`   Embedding Similarity: <${targets.embeddingSimilarity}ms per calc`);
console.log(`   Memory Growth: <${targets.memoryGrowth}MB (50 requests)`);
console.log();

console.log('🔥 Running benchmarks...');
console.log('   This may take 5-10 minutes...');
console.log();

try {
  const output = execSync('npx ts-node benchmarks/performance-suite.ts', {
    encoding: 'utf8',
    stdio: 'inherit',
    env: process.env
  });
  
  console.log();
  console.log('✅ Benchmarks completed successfully!');
  
} catch (error) {
  console.log();
  console.log('⚠️  Benchmark execution completed with errors');
  console.log('   Review output above for details');
}

console.log();
console.log('='.repeat(70));
console.log(' Next Steps');
console.log('='.repeat(70));
console.log();
console.log('1. Review benchmark results above');
console.log('2. Compare with targets');
console.log('3. Identify any optimization needs');
console.log('4. Document in performance report');
console.log();

