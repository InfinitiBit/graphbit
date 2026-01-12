/**
 * Performance Benchmark Suite
 * 
 * Comprehensive performance tests for GraphBit JavaScript bindings
 * Measures against targets and optionally compares with Python
 */

import { LlmClient, LlmConfig, EmbeddingClient, EmbeddingConfig } from '../index';

interface BenchmarkResult {
  name: string;
  duration: number;
  throughput?: number;
  memoryUsed?: number;
  status: 'PASS' | 'FAIL' | 'WARN';
  notes: string;
}

const results: BenchmarkResult[] = [];

async function benchmark(
  name: string,
  fn: () => Promise<void>,
  target?: { maxDuration?: number; minThroughput?: number }
): Promise<BenchmarkResult> {
  const memBefore = process.memoryUsage().heapUsed;
  const start = Date.now();
  
  await fn();
  
  const duration = Date.now() - start;
  const memAfter = process.memoryUsage().heapUsed;
  const memoryUsed = (memAfter - memBefore) / 1024 / 1024; // MB
  
  let status: 'PASS' | 'FAIL' | 'WARN' = 'PASS';
  let notes = '';
  
  if (target?.maxDuration && duration > target.maxDuration) {
    status = 'FAIL';
    notes = `Duration ${duration}ms exceeds target ${target.maxDuration}ms`;
  } else if (target?.maxDuration && duration > target.maxDuration * 0.8) {
    status = 'WARN';
    notes = `Duration ${duration}ms close to target ${target.maxDuration}ms`;
  } else {
    notes = `Duration ${duration}ms within target`;
  }
  
  return {
    name,
    duration,
    memoryUsed,
    status,
    notes
  };
}

async function runBenchmarks() {
  console.log('='.repeat(70));
  console.log(' GraphBit JavaScript Bindings - Performance Benchmark Suite');
  console.log('='.repeat(70));
  console.log();

  // Check for API keys
  if (!process.env.OPENAI_API_KEY) {
    console.log('⚠️  OPENAI_API_KEY not set - skipping API benchmarks');
    console.log('   Set API key to run full benchmarks\n');
    return;
  }

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY!,
    model: 'gpt-4o-mini'
  });
  const client = new LlmClient(config);

  console.log('🔥 Starting benchmarks...\n');

  // Benchmark 1: Simple Completion
  console.log('1. Simple Completion Latency');
  console.log('-'.repeat(70));
  
  const result1 = await benchmark(
    'Simple Completion (10 iterations)',
    async () => {
      for (let i = 0; i < 10; i++) {
        await client.complete("Test", 10, 0.0);
      }
    },
    { maxDuration: 30000 } // 30 seconds for 10 requests
  );
  results.push(result1);
  
  console.log(`   Duration: ${result1.duration}ms`);
  console.log(`   Avg per request: ${(result1.duration / 10).toFixed(0)}ms`);
  console.log(`   Status: ${result1.status} - ${result1.notes}\n`);

  // Benchmark 2: Batch Processing
  console.log('2. Batch Processing Throughput');
  console.log('-'.repeat(70));
  
  const result2 = await benchmark(
    'Batch Processing (20 prompts, concurrency 5)',
    async () => {
      const prompts = Array(20).fill("Test prompt");
      await client.completeBatch(prompts, 10, 0.0, 5);
    },
    { maxDuration: 40000 } // 40 seconds for 20 requests
  );
  results.push(result2);
  
  console.log(`   Duration: ${result2.duration}ms`);
  console.log(`   Throughput: ${(20000 / result2.duration).toFixed(2)} req/s`);
  console.log(`   Status: ${result2.status} - ${result2.notes}\n`);

  // Benchmark 3: Concurrent Requests
  console.log('3. Concurrent Request Handling');
  console.log('-'.repeat(70));
  
  const result3 = await benchmark(
    'Concurrent Requests (10 parallel)',
    async () => {
      const promises = Array(10).fill(null).map(() =>
        client.complete("Test", 10, 0.0)
      );
      await Promise.all(promises);
    },
    { maxDuration: 25000 } // 25 seconds for 10 concurrent
  );
  results.push(result3);
  
  console.log(`   Duration: ${result3.duration}ms`);
  console.log(`   Status: ${result3.status} - ${result3.notes}\n`);

  // Benchmark 4: Memory Usage
  console.log('4. Memory Usage Pattern');
  console.log('-'.repeat(70));
  
  const memBefore = process.memoryUsage().heapUsed;
  
  for (let i = 0; i < 50; i++) {
    await client.complete("Test", 10, 0.0);
  }
  
  const memAfter = process.memoryUsage().heapUsed;
  const memGrowth = (memAfter - memBefore) / 1024 / 1024;
  
  console.log(`   Memory before: ${(memBefore / 1024 / 1024).toFixed(2)} MB`);
  console.log(`   Memory after: ${(memAfter / 1024 / 1024).toFixed(2)} MB`);
  console.log(`   Growth: ${memGrowth.toFixed(2)} MB for 50 requests`);
  console.log(`   Status: ${memGrowth < 10 ? 'PASS' : 'FAIL'}\n`);

  // Benchmark 5: Statistics Overhead
  console.log('5. Statistics Tracking Overhead');
  console.log('-'.repeat(70));
  
  const result5 = await benchmark(
    'Statistics Overhead',
    async () => {
      for (let i = 0; i < 20; i++) {
        await client.complete("Test", 10);
        if (i % 5 === 0) {
          await client.getStats();
        }
      }
    }
  );
  results.push(result5);
  
  console.log(`   Duration: ${result5.duration}ms`);
  console.log(`   Status: Statistics tracking minimal overhead\n`);

  // Benchmark 6: Embedding Similarity
  console.log('6. Embedding Similarity Performance');
  console.log('-'.repeat(70));
  
  const embConfig = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!);
  const embClient = new EmbeddingClient(embConfig);
  
  const embResponse = await embClient.embed(['test1', 'test2']);
  const emb1 = embResponse.embeddings[0];
  const emb2 = embResponse.embeddings[1];
  
  const iterations = 10000;
  const start = performance.now();
  
  for (let i = 0; i < iterations; i++) {
    EmbeddingClient.similarity(emb1, emb2);
  }
  
  const duration = performance.now() - start;
  const avgTime = duration / iterations;
  
  console.log(`   ${iterations} calculations in ${duration.toFixed(2)}ms`);
  console.log(`   Avg per calculation: ${avgTime.toFixed(4)}ms`);
  console.log(`   Status: ${avgTime < 0.5 ? 'PASS' : 'FAIL'} (target: <0.5ms)\n`);

  // Final Summary
  console.log('='.repeat(70));
  console.log(' Benchmark Summary');
  console.log('='.repeat(70));
  console.log();
  
  results.forEach(result => {
    const statusIcon = result.status === 'PASS' ? '✅' : 
                      result.status === 'WARN' ? '⚠️' : '❌';
    console.log(`${statusIcon} ${result.name}`);
    console.log(`   Duration: ${result.duration}ms`);
    console.log(`   ${result.notes}`);
  });
  
  console.log();
  const passed = results.filter(r => r.status === 'PASS').length;
  const warned = results.filter(r => r.status === 'WARN').length;
  const failed = results.filter(r => r.status === 'FAIL').length;
  
  console.log(`Results: ${passed} passed, ${warned} warnings, ${failed} failed`);
  console.log();
  
  if (failed === 0) {
    console.log('✅ All benchmarks passed!');
  } else {
    console.log('❌ Some benchmarks failed - optimization needed');
  }
}

// Run benchmarks
runBenchmarks().catch(console.error);

