# Performance Optimization - JavaScript

This guide covers performance optimization strategies for GraphBit JavaScript applications, from execution patterns to resource management and configuration tuning.

## Overview

Performance optimization in GraphBit focuses on:
- **Execution Optimization**: Parallel processing and efficient node execution
- **Resource Management**: Memory, CPU, and network optimization
- **Caching Strategies**: Reducing redundant computations
- **Configuration Tuning**: Optimal settings for different scenarios
- **Monitoring & Profiling**: Identifying and resolving bottlenecks

## Execution Optimization

### Parallel Processing

```typescript
import { init, WorkflowBuilder, AgentBuilder, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

async function createParallelWorkflow() {
  const workflow = await new WorkflowBuilder('ParallelProcessingWorkflow')
    .description('Parallel processing demonstration')
    .build();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // Input processor
  const inputProcessor = await new AgentBuilder('Input Processor', config)
    .systemPrompt('Prepare data for parallel processing')
    .build();

  const inputId = await inputProcessor.id();
  const inputNodeId = await workflow.addNode({
    id: inputId.uuid,
    name: await inputProcessor.name(),
    description: await inputProcessor.description(),
    nodeType: 'Agent'
  });

  // Parallel processing branches
  const branchNodeIds: string[] = [];
  for (let i = 0; i < 4; i++) {
    const branch = await new AgentBuilder(`Parallel Branch ${i + 1}`, config)
      .systemPrompt(`Process branch ${i + 1} data`)
      .build();

    const branchId = await branch.id();
    const nodeId = await workflow.addNode({
      id: branchId.uuid,
      name: await branch.name(),
      description: await branch.description(),
      nodeType: 'Agent'
    });
    branchNodeIds.push(nodeId);
  }

  // Results aggregator
  const aggregator = await new AgentBuilder('Results Aggregator', config)
    .systemPrompt('Combine results from parallel branches')
    .build();

  const aggId = await aggregator.id();
  const aggNodeId = await workflow.addNode({
    id: aggId.uuid,
    name: await aggregator.name(),
    description: await aggregator.description(),
    nodeType: 'Agent'
  });

  // Connect input to all branches (fan-out)
  for (const branchNodeId of branchNodeIds) {
    await workflow.addEdge(inputNodeId, branchNodeId);
  }

  // Connect all branches to aggregator (fan-in)
  for (const branchNodeId of branchNodeIds) {
    await workflow.addEdge(branchNodeId, aggNodeId);
  }

  await workflow.validate();

  return workflow;
}

// Usage
async function parallelExample() {
  init();
  const workflow = await createParallelWorkflow();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  const executor = new Executor(config);

  const start = Date.now();
  const result = await executor.execute(workflow);
  const duration = Date.now() - start;

  console.log(`Parallel execution completed in ${duration}ms`);
  console.log(`Success: ${await result.isCompleted()}`);
}

parallelExample().catch(console.error);
```

## Executor Configuration

### Different Executor Types

```typescript
async function createOptimizedExecutors() {
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // Low-latency executor (single thread, optimized for response time)
  const lowLatency = Executor.newLowLatency(config);

  // High-throughput executor (multi-thread, optimized for batch processing)
  const highThroughput = Executor.newHighThroughput(config);

  // Default executor (balanced configuration)
  const balanced = new Executor(config);

  return { lowLatency, highThroughput, balanced };
}

// Benchmark different configurations
async function benchmarkExecutors() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // Create simple workflow
  const agent = await new AgentBuilder('Agent', config)
    .systemPrompt('Process task')
    .build();

  const agentId = await agent.id();

  const workflow = await new WorkflowBuilder('BenchmarkWorkflow')
    .description('Benchmark test workflow')
    .build();

  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });

  await workflow.validate();

  const { lowLatency, highThroughput, balanced } = await createOptimizedExecutors();

  // Test low-latency
  const start1 = Date.now();
  await lowLatency.execute(workflow);
  const duration1 = Date.now() - start1;
  console.log(`Low-latency: ${duration1}ms`);

  // Test high-throughput
  const start2 = Date.now();
  await highThroughput.execute(workflow);
  const duration2 = Date.now() - start2;
  console.log(`High-throughput: ${duration2}ms`);

  // Test balanced
  const start3 = Date.now();
  await balanced.execute(workflow);
  const duration3 = Date.now() - start3;
  console.log(`Balanced: ${duration3}ms`);
}
```

## LLM Provider Optimization

### Model Selection

```typescript
// Fast and cost-effective models for production
const fastModels = {
  openai: LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini' // Fastest OpenAI model
  }),
  
  anthropic: LlmConfig.anthropic({
    apiKey: process.env.ANTHROPIC_API_KEY,
    model: 'claude-3-5-haiku-20241022' // Fast Anthropic model
  }),
  
  ollama: LlmConfig.ollama({
    model: 'llama3.2:1b' // Small, fast local model
  })
};

// Use appropriate model for task complexity
async function selectModelByComplexity(taskComplexity: 'simple' | 'complex') {
  if (taskComplexity === 'simple') {
    // Use fast, inexpensive model
    return LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY,
      model: 'gpt-4o-mini'
    });
  } else {
    // Use more capable model
    return LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY,
      model: 'gpt-4o'
    });
  }
}
```

### Batch Processing

```typescript
import { LlmClient } from '@infinitibit_gmbh/graphbit';

async function efficientBatchProcessing() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  const client = new LlmClient(config);

  const prompts = Array.from({ length: 10 }, (_, i) => 
    `Process item ${i + 1}`
  );

  const start = Date.now();
  
  // Process batch with concurrency control
  const results = await client.completeBatch(
    prompts,
    100,    // maxTokens
    0.7,    // temperature
    5       // concurrency - balance between speed and rate limits
  );

  const duration = Date.now() - start;
  
  console.log(`Processed ${prompts.length} items in ${duration}ms`);
  console.log(`Average: ${Math.round(duration / prompts.length)}ms per item`);
  
  return results;
}
```

## Caching Strategies

### Response Caching

```typescript
interface CacheEntry<T> {
  value: T;
  timestamp: number;
  ttl: number;
}

class ResponseCache<T = any> {
  private cache: Map<string, CacheEntry<T>> = new Map();

  /**
   * Get cached value
   */
  get(key: string): T | null {
    const entry = this.cache.get(key);
    
    if (!entry) return null;

    // Check if expired
    if (Date.now() - entry.timestamp > entry.ttl) {
      this.cache.delete(key);
      return null;
    }

    return entry.value;
  }

  /**
   * Set cached value
   */
  set(key: string, value: T, ttlMs: number = 3600000): void {
    this.cache.set(key, {
      value,
      timestamp: Date.now(),
      ttl: ttlMs
    });
  }

  /**
   * Clear cache
   */
  clear(): void {
    this.cache.clear();
  }

  /**
   * Get cache statistics
   */
  getStats(): { size: number; keys: string[] } {
    return {
      size: this.cache.size,
      keys: Array.from(this.cache.keys())
    };
  }
}

// Usage with LLM client
class CachedLlmClient {
  private cache = new ResponseCache<string>();

  constructor(private client: LlmClient) {}

  /**
   * Complete with caching
   */
  async complete(prompt: string, cacheTtlMs: number = 3600000): Promise<string> {
    const cacheKey = this.getCacheKey(prompt);
    
    // Check cache first
    const cached = this.cache.get(cacheKey);
    if (cached) {
      console.log('Cache hit:', cacheKey);
      return cached;
    }

    // Execute and cache
    console.log('Cache miss:', cacheKey);
    const result = await this.client.complete(prompt);
    this.cache.set(cacheKey, result, cacheTtlMs);

    return result;
  }

  private getCacheKey(prompt: string): string {
    // Simple hash function
    let hash = 0;
    for (let i = 0; i < prompt.length; i++) {
      hash = ((hash << 5) - hash) + prompt.charCodeAt(i);
      hash = hash & hash;
    }
    return hash.toString(36);
  }

  getCacheStats(): any {
    return this.cache.getStats();
  }
}

// Example
async function cachedExample() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);
  const cachedClient = new CachedLlmClient(client);

  // First call - cache miss
  const start1 = Date.now();
  await cachedClient.complete('Hello, world!');
  console.log(`First call: ${Date.now() - start1}ms`);

  // Second call - cache hit
  const start2 = Date.now();
  await cachedClient.complete('Hello, world!');
  console.log(`Second call: ${Date.now() - start2}ms`);

  console.log('Cache stats:', cachedClient.getCacheStats());
}
```

## Memory Optimization

### Memory-Efficient Execution

```typescript
async function memoryEfficientExecution() {
  init();

  // Use low-latency executor to minimize memory overhead
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  const executor = Executor.newLowLatency(config);

  // Monitor memory
  const before = process.memoryUsage();
  console.log('Memory before:', {
    heapUsed: `${Math.round(before.heapUsed / 1024 / 1024)}MB`,
    external: `${Math.round(before.external / 1024 / 1024)}MB`
  });

  // Execute workflow
  const agent = await new AgentBuilder('Agent', config)
    .systemPrompt('Complete task')
    .build();

  const agentId = await agent.id();

  const workflow = await new WorkflowBuilder('MemoryEfficient')
    .description('Memory efficient workflow')
    .build();

  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });

  await workflow.validate();
  
  await executor.execute(workflow);

  // Check memory usage
  const after = process.memoryUsage();
  console.log('Memory after:', {
    heapUsed: `${Math.round(after.heapUsed / 1024 / 1024)}MB`,
    external: `${Math.round(after.external / 1024 / 1024)}MB`
  });

  const delta = {
    heapUsed: Math.round((after.heapUsed - before.heapUsed) / 1024 / 1024),
    external: Math.round((after.external - before.external) / 1024 / 1024)
  };

  console.log('Memory delta:', delta);
}
```

### Streaming for Large Responses

```typescript
async function streamLargeResponse() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  console.log('Starting streaming response...');
  
  // Stream response to avoid loading everything into memory
  const stream = await client.stream('Generate a long document');

  let chunkCount = 0;
  for await (const chunk of stream) {
    chunkCount++;
    process.stdout.write(chunk); // Process incrementally
  }

  console.log(`\n\nReceived ${chunkCount} chunks via streaming`);
}
```

## Profiling and Benchmarking

### Performance Profiler

```typescript
interface ProfileResult {
  label: string;
  durationMs: number;
  memoryDeltaMB: number;
  timestamp: Date;
}

class PerformanceProfiler {
  private results: ProfileResult[] = [];

  /**
   * Profile an async operation
   */
  async profile<T>(
    label: string,
    fn: () => Promise<T>
  ): Promise<{ result: T; profile: ProfileResult }> {
    const memBefore = process.memoryUsage();
    const start = Date.now();

    const result = await fn();

    const duration = Date.now() - start;
    const memAfter = process.memoryUsage();
    const memDelta = Math.round(
      (memAfter.heapUsed - memBefore.heapUsed) / 1024 / 1024
    );

    const profile: ProfileResult = {
      label,
      durationMs: duration,
      memoryDeltaMB: memDelta,
      timestamp: new Date()
    };

    this.results.push(profile);

    console.log(`[${label}] ${duration}ms, ${memDelta}MB`);

    return { result, profile };
  }

  /**
   * Get profiling results
   */
  getResults(): ProfileResult[] {
    return [...this.results];
  }

  /**
   * Generate report
   */
  generateReport(): string {
    if (this.results.length === 0) {
      return 'No profiling data';
    }

    const lines = [
      'Performance Profile Report',
      '='.repeat(50),
      ''
    ];

    this.results.forEach(r => {
      lines.push(`${r.label}:`);
      lines.push(`  Duration: ${r.durationMs}ms`);
      lines.push(`  Memory: ${r.memoryDeltaMB}MB`);
      lines.push(`  Time: ${r.timestamp.toISOString()}`);
      lines.push('');
    });

    const totalDuration = this.results.reduce((sum, r) => sum + r.durationMs, 0);
    const totalMemory = this.results.reduce((sum, r) => sum + r.memoryDeltaMB, 0);

    lines.push('Summary:');
    lines.push(`  Total operations: ${this.results.length}`);
    lines.push(`  Total duration: ${totalDuration}ms`);
    lines.push(`  Total memory: ${totalMemory}MB`);
    lines.push(`  Avg duration: ${Math.round(totalDuration / this.results.length)}ms`);

    return lines.join('\n');
  }
}

// Usage
async function profileExample() {
  init();

  const profiler = new PerformanceProfiler();
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  // Profile LLM client creation
  const { result: client } = await profiler.profile(
    'Create LLM Client',
    async () => new LlmClient(config)
  );

  // Profile API call
  await profiler.profile(
    'LLM Completion',
    () => client.complete('Hello')
  );

  // Profile workflow execution
  const executor = new Executor(config);

  const agent = await new AgentBuilder('Agent', config)
    .systemPrompt('Complete task')
    .build();

  const agentId = await agent.id();

  const workflow = await new WorkflowBuilder('ProfiledWorkflow')
    .description('Profiled workflow')
    .build();

  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });

  await workflow.validate();

  await profiler.profile(
    'Workflow Execution',
    () => executor.execute(workflow)
  );

  // Generate report
  console.log('\n' + profiler.generateReport());
}
```

## Configuration Best Practices

### Production Configuration

```typescript
// Development configuration
const devConfig = {
  llm: LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini' // Fast for testing
  }),
  executorType: 'balanced'
};

// Production configuration
const prodConfig = {
  llm: LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini' // Cost-effective for production
  }),
  executorType: 'high-throughput'
};

// Apply configuration based on environment
async function getOptimalConfig() {
  const isProduction = process.env.NODE_ENV === 'production';
  const config = isProduction ? prodConfig : devConfig;

  const llmConfig = config.llm;
  
  const executor = config.executorType === 'high-throughput'
    ? Executor.newHighThroughput(llmConfig)
    : config.executorType === 'low-latency'
    ? Executor.newLowLatency(llmConfig)
    : new Executor(llmConfig);

  return { llmConfig, executor };
}
```

## Performance Monitoring

### Track Execution Metrics

```typescript
class PerformanceMonitor {
  private metrics: Array<{
    operation: string;
    duration: number;
    timestamp: Date;
  }> = [];

  /**
   * Record execution time
   */
  async measure<T>(
    operation: string,
    fn: () => Promise<T>
  ): Promise<T> {
    const start = Date.now();
    const result = await fn();
    const duration = Date.now() - start;

    this.metrics.push({
      operation,
      duration,
      timestamp: new Date()
    });

    return result;
  }

  /**
   * Get performance statistics
   */
  getStats(): Record<string, any> {
    const byOperation: Record<string, number[]> = {};

    this.metrics.forEach(m => {
      if (!byOperation[m.operation]) {
        byOperation[m.operation] = [];
      }
      byOperation[m.operation].push(m.duration);
    });

    const stats: Record<string, any> = {};

    Object.entries(byOperation).forEach(([op, durations]) => {
      const avg = durations.reduce((a, b) => a + b, 0) / durations.length;
      const min = Math.min(...durations);
      const max = Math.max(...durations);

      stats[op] = {
        count: durations.length,
        avgMs: Math.round(avg),
        minMs: min,
        maxMs: max
      };
    });

    return stats;
  }
}

// Usage
async function monitorPerformance() {
  init();

  const monitor = new PerformanceMonitor();
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const executor = new Executor(config);

  const agent = await new AgentBuilder('Agent', config)
    .systemPrompt('Complete task')
    .build();

  const agentId = await agent.id();

  const workflow = await new WorkflowBuilder('MonitoredWorkflow')
    .description('Monitored workflow')
    .build();

  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });

  await workflow.validate();

  // Execute multiple times
  for (let i = 0; i < 5; i++) {
    await monitor.measure('workflow-execution', () =>
      executor.execute(workflow)
    );
  }

  console.log('Performance statistics:', monitor.getStats());
}
```

## Best Practices

1. **Use parallel processing for independent tasks**
2. **Select appropriate executor type** (low-latency vs high-throughput)
3. **Choose fast, cost-effective models** for simple tasks
4. **Implement caching** for repeated requests
5. **Use batch processing** for multiple items
6. **Stream large responses** to reduce memory
7. **Profile regularly** to identify bottlenecks
8. **Monitor memory usage** in long-running processes
9. **Configure timeouts appropriately**
10. **Test with production-like loads**

## Performance Benchmarks

GraphBit achieves:
- **68× lower CPU usage** vs alternatives
- **140× lower memory usage** vs alternatives
- **Sub-second initialization** time
- **Concurrent execution** of parallel workflows
- **Zero-copy data transfer** between JavaScript and Rust

## See Also

- [Memory Management Guide](./memory-management-js.md)
- [Monitoring Guide](./monitoring-js.md)
- [Architecture Guide](../development/architecture-js.md)
- [Async Patterns](./async-vs-sync-js.md)
