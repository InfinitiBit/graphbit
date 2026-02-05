# LLM Integration and Advanced Usage - JavaScript

This example demonstrates comprehensive LLM integration with GraphBit's JavaScript bindings, showcasing various providers, execution modes, and advanced features.

## Overview

We'll explore:
1. **Multiple LLM Providers**: OpenAI, Anthropic, Ollama, OpenRouter
2. **Execution Modes**: Sync, batch, streaming
3. **Performance Optimization**: Different executor configurations
4. **Error Handling**: Resilience patterns and fallbacks
5. **Monitoring**: Performance metrics and health checks

## Complete LLM System Implementation

```typescript
import {
  init,
  LlmConfig,
  LlmClient,
  Executor,
  Workflow,
  Node,
  healthCheck,
  getSystemInfo
} from '@infinitibit_gmbh/graphbit';

class AdvancedLLMSystem {
  private clients: Map<string, LlmClient> = new Map();
  private executors: Map<string, Executor> = new Map();
  private metrics: Array<{
    provider: string;
    operation: string;
    duration: number;
    success: boolean;
  }> = [];

  constructor() {
    init();
    this.initializeProviders();
  }

  private initializeProviders(): void {
    console.log('🚀 Initializing LLM providers...\n');

    // OpenAI
    if (process.env.OPENAI_API_KEY) {
      const config = LlmConfig.openai({
        apiKey: process.env.OPENAI_API_KEY,
        model: 'gpt-4o-mini'
      });
      this.clients.set('openai', new LlmClient(config));
      this.executors.set('openai', new Executor(config));
      console.log('✅ OpenAI initialized');
    }

    // Anthropic
    if (process.env.ANTHROPIC_API_KEY) {
      const config = LlmConfig.anthropic({
        apiKey: process.env.ANTHROPIC_API_KEY,
        model: 'claude-3-5-sonnet-20241022'
      });
      this.clients.set('anthropic', new LlmClient(config));
      this.executors.set('anthropic', new Executor(config));
      console.log('✅ Anthropic initialized');
    }

    // OpenRouter (access to 400+ models)
    if (process.env.OPENROUTER_API_KEY) {
      const config = LlmConfig.openrouter({
        apiKey: process.env.OPENROUTER_API_KEY,
        model: 'anthropic/claude-3.5-sonnet'
      });
      this.clients.set('openrouter', new LlmClient(config));
      this.executors.set('openrouter', new Executor(config));
      console.log('✅ OpenRouter initialized');
    }

    // Ollama (local models)
    try {
      const config = LlmConfig.ollama({ model: 'llama3.2' });
      this.clients.set('ollama', new LlmClient(config));
      this.executors.set('ollama', new Executor(config));
      console.log('✅ Ollama initialized');
    } catch (error) {
      console.log('⚠️  Ollama not available:', error instanceof Error ? error.message : error);
    }

    if (this.clients.size === 0) {
      throw new Error('No LLM providers available. Set API keys or install Ollama.');
    }

    console.log(`\n✅ Initialized ${this.clients.size} provider(s)\n`);
  }

  async testBasicCompletion(provider: string = 'openai'): Promise<string | null> {
    const client = this.clients.get(provider);
    if (!client) {
      console.error(`❌ Provider '${provider}' not available`);
      return null;
    }

    const prompt = 'Explain quantum computing in simple terms.';
    console.log(`\n📝 Testing basic completion with ${provider}...`);
    console.log(`Prompt: ${prompt}`);

    const start = Date.now();
    try {
      const response = await client.complete(prompt);
      const duration = Date.now() - start;

      this.recordMetric(provider, 'completion', duration, true);
      
      console.log(`✅ Completed in ${duration}ms`);
      console.log(`Response: ${response.substring(0, 200)}...\n`);

      return response;
    } catch (error) {
      const duration = Date.now() - start;
      this.recordMetric(provider, 'completion', duration, false);
      
      console.error(`❌ Completion failed:`, error);
      return null;
    }
  }

  async testBatchCompletion(provider: string = 'openai'): Promise<string[] | null> {
    const client = this.clients.get(provider);
    if (!client) {
      console.error(`❌ Provider '${provider}' not available`);
      return null;
    }

    const prompts = [
      'What is machine learning?',
      'Explain neural networks briefly.',
      'What are the benefits of cloud computing?',
      'How does blockchain work?',
      'What is the future of AI?'
    ];

    console.log(`\n📦 Testing batch completion with ${provider}...`);
    console.log(`Processing ${prompts.length} prompts...`);

    const start = Date.now();
    try {
      const results = await client.completeBatch(
        prompts,
        100,  // maxTokens
        0.7,  // temperature
        3     // concurrency
      );
      const duration = Date.now() - start;

      this.recordMetric(provider, 'batch', duration, true);

      console.log(`✅ Batch completed in ${duration}ms`);
      console.log(`Average: ${Math.round(duration / prompts.length)}ms per prompt`);
      
      results.forEach((result, i) => {
        console.log(`\n${i + 1}. ${prompts[i]}`);
        console.log(`   → ${result.substring(0, 100)}...`);
      });

      return results;
    } catch (error) {
      const duration = Date.now() - start;
      this.recordMetric(provider, 'batch', duration, false);
      
      console.error(`❌ Batch completion failed:`, error);
      return null;
    }
  }

  async testStreamingCompletion(provider: string = 'openai'): Promise<void> {
    const client = this.clients.get(provider);
    if (!client) {
      console.error(`❌ Provider '${provider}' not available`);
      return;
    }

    const prompt = 'Write a short poem about artificial intelligence.';
    console.log(`\n🌊 Testing streaming completion with ${provider}...`);
    console.log(`Prompt: ${prompt}\n`);

    const start = Date.now();
    try {
      const stream = await client.stream(prompt);

      let fullResponse = '';
      let chunkCount = 0;

      for await (const chunk of stream) {
        process.stdout.write(chunk);
        fullResponse += chunk;
        chunkCount++;
      }

      const duration = Date.now() - start;
      this.recordMetric(provider, 'streaming', duration, true);

      console.log(`\n\n✅ Streaming completed in ${duration}ms (${chunkCount} chunks)`);
    } catch (error) {
      const duration = Date.now() - start;
      this.recordMetric(provider, 'streaming', duration, false);
      
      console.error(`\n❌ Streaming failed:`, error);
    }
  }

  async testWorkflowExecution(provider: string = 'openai'): Promise<void> {
    const executor = this.executors.get(provider);
    if (!executor) {
      console.error(`❌ Provider '${provider}' not available`);
      return;
    }

    console.log(`\n🔄 Testing workflow execution with ${provider}...`);

    const workflow = new Workflow('Multi-Step Analysis');

    const researcher = Node.agent(
      'Researcher',
      'Research the topic: Quantum Computing. Provide key facts.',
      'researcher'
    );

    const analyzer = Node.agent(
      'Analyzer',
      'Analyze the research and identify main concepts.',
      'analyzer'
    );

    const summarizer = Node.agent(
      'Summarizer',
      'Create a concise summary of the analysis.',
      'summarizer'
    );

    await workflow.addNode(researcher);
    await workflow.addNode(analyzer);
    await workflow.addNode(summarizer);

    await workflow.connect('researcher', 'analyzer');
    await workflow.connect('analyzer', 'summarizer');

    await workflow.validate();

    const start = Date.now();
    try {
      const result = await executor.execute(workflow);
      const duration = Date.now() - start;

      this.recordMetric(provider, 'workflow', duration, result.isSuccess());

      if (result.isSuccess()) {
        console.log(`✅ Workflow completed in ${duration}ms`);
        console.log('Variables:', result.variables());
      } else {
        console.error(`❌ Workflow failed:`, result.error());
      }
    } catch (error) {
      const duration = Date.now() - start;
      this.recordMetric(provider, 'workflow', duration, false);
      
      console.error(`❌ Workflow execution failed:`, error);
    }
  }

  async testMultiProviderFallback(): Promise<string | null> {
    console.log('\n🔄 Testing multi-provider fallback...');

    const providers = ['openai', 'anthropic', 'ollama'];
    const prompt = 'What is the meaning of life?';

    for (const provider of providers) {
      const client = this.clients.get(provider);
      if (!client) continue;

      console.log(`\nTrying ${provider}...`);

      try {
        const response = await client.complete(prompt);
        console.log(`✅ Success with ${provider}`);
        return response;
      } catch (error) {
        console.log(`❌ ${provider} failed, trying next...`);
      }
    }

    console.error('❌ All providers failed');
    return null;
  }

  async benchmarkProviders(prompt: string): Promise<void> {
    console.log('\n📊 Benchmarking all providers...\n');

    const results: Array<{
      provider: string;
      duration: number;
      success: boolean;
      responseLength: number;
    }> = [];

    for (const [provider, client] of this.clients) {
      const start = Date.now();
      
      try {
        const response = await client.complete(prompt);
        const duration = Date.now() - start;

        results.push({
          provider,
          duration,
          success: true,
          responseLength: response.length
        });

        console.log(`${provider}: ${duration}ms ✅`);
      } catch (error) {
        const duration = Date.now() - start;

        results.push({
          provider,
          duration,
          success: false,
          responseLength: 0
        });

        console.log(`${provider}: Failed after ${duration}ms ❌`);
      }
    }

    console.log('\n📊 Benchmark Results:');
    const sorted = results.filter(r => r.success).sort((a, b) => a.duration - b.duration);
    
    if (sorted.length > 0) {
      console.log('Fastest:', sorted[0].provider, `(${sorted[0].duration}ms)`);
      console.log('Average:', Math.round(sorted.reduce((sum, r) => sum + r.duration, 0) / sorted.length), 'ms');
    } else {
      console.log('No successful completions');
    }
  }

  async testExecutorModes(): Promise<void> {
    console.log('\n⚙️  Testing different executor modes...\n');

    if (!process.env.OPENAI_API_KEY) {
      console.log('⚠️  OpenAI API key required for this test');
      return;
    }

    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY
    });

    const workflow = new Workflow('Simple Task');
    const node = Node.agent('Agent', 'Say hello', 'agent1');
    await workflow.addNode(node);
    await workflow.validate();

    // Test low-latency executor
    const lowLatency = Executor.newLowLatency(config);
    let start = Date.now();
    await lowLatency.execute(workflow);
    console.log(`Low-latency: ${Date.now() - start}ms`);

    // Test high-throughput executor
    const highThroughput = Executor.newHighThroughput(config);
    start = Date.now();
    await highThroughput.execute(workflow);
    console.log(`High-throughput: ${Date.now() - start}ms`);

    // Test default executor
    const defaultExecutor = new Executor(config);
    start = Date.now();
    await defaultExecutor.execute(workflow);
    console.log(`Default: ${Date.now() - start}ms`);
  }

  private recordMetric(
    provider: string,
    operation: string,
    duration: number,
    success: boolean
  ): void {
    this.metrics.push({ provider, operation, duration, success });
  }

  getMetrics(): any {
    const byProvider: Record<string, any> = {};

    this.metrics.forEach(m => {
      if (!byProvider[m.provider]) {
        byProvider[m.provider] = {
          total: 0,
          successful: 0,
          failed: 0,
          avgDuration: 0,
          operations: []
        };
      }

      byProvider[m.provider].total++;
      if (m.success) {
        byProvider[m.provider].successful++;
      } else {
        byProvider[m.provider].failed++;
      }
      byProvider[m.provider].operations.push({
        operation: m.operation,
        duration: m.duration,
        success: m.success
      });
    });

    // Calculate averages
    for (const provider of Object.keys(byProvider)) {
      const ops = byProvider[provider].operations;
      const successfulOps = ops.filter((o: any) => o.success);
      
      if (successfulOps.length > 0) {
        byProvider[provider].avgDuration = Math.round(
          successfulOps.reduce((sum: number, o: any) => sum + o.duration, 0) / successfulOps.length
        );
      }
    }

    return byProvider;
  }

  printSummary(): void {
    console.log('\n' + '='.repeat(50));
    console.log('📊 PERFORMANCE SUMMARY');
    console.log('='.repeat(50) + '\n');

    const metrics = this.getMetrics();

    for (const [provider, data] of Object.entries(metrics)) {
      console.log(`\n${provider.toUpperCase()}:`);
      console.log(`  Total operations: ${data.total}`);
      console.log(`  Successful: ${data.successful}`);
      console.log(`  Failed: ${data.failed}`);
      console.log(`  Success rate: ${Math.round((data.successful / data.total) * 100)}%`);
      console.log(`  Average duration: ${data.avgDuration}ms`);
    }

    // System health
    console.log('\n' + '-'.repeat(50));
    console.log('SYSTEM HEALTH:');
    const health = healthCheck();
    console.log(`  Overall: ${health.overallHealthy ? '✅ Healthy' : '⚠️  Degraded'}`);

    const info = getSystemInfo();
    console.log(`  Node version: ${info.nodeVersion}`);
    console.log(`  CPU count: ${info.cpuCount}`);
    
    console.log('\n' + '='.repeat(50) + '\n');
  }
}

// Main execution
async function main() {
  try {
    const system = new AdvancedLLMSystem();

    // Test basic completion
    await system.testBasicCompletion('openai');

    // Test batch completion
    await system.testBatchCompletion('openai');

    // Test streaming
    await system.testStreamingCompletion('openai');

    // Test workflow execution
    await system.testWorkflowExecution('openai');

    // Test multi-provider fallback
    await system.testMultiProviderFallback();

    // Benchmark providers
    await system.benchmarkProviders('What is artificial intelligence?');

    // Test executor modes
    await system.testExecutorModes();

    // Print summary
    system.printSummary();

  } catch (error) {
    console.error('❌ Fatal error:', error);
    process.exit(1);
  }
}

main().catch(console.error);
```

## Key Features Demonstrated

1. **Multiple Providers**: OpenAI, Anthropic, OpenRouter, Ollama
2. **Completion Modes**: Basic, batch, streaming
3. **Workflow Integration**: Multi-step LLM workflows
4. **Fallback Strategy**: Automatic provider fallback
5. **Performance Metrics**: Comprehensive tracking
6. **Executor Modes**: Low-latency, high-throughput, default
7. **Error Handling**: Graceful error handling with retries

## Best Practices

1. **Check provider availability** before use
2. **Handle streaming responses** incrementally
3. **Use batch processing** for multiple prompts
4. **Implement fallback strategies** for reliability
5. **Track performance metrics** for optimization
6. **Choose appropriate executor mode** for use case
7. **Monitor system health** regularly

## Performance Tips

- Use `gpt-4o-mini` for fast, cost-effective processing
- Batch operations for better throughput
- Stream large responses to reduce memory
- Use low-latency executor for interactive applications
- Implement caching for repeated queries

## See Also

- [LLM Providers Guide](../user-guide/llm-providers-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
- [Reliability Guide](../user-guide/reliability-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
