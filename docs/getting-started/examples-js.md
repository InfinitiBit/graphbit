# Basic Examples - JavaScript

This guide provides simple, practical examples to help you get started with GraphBit's JavaScript bindings quickly.

## Example 1: Simple Text Analysis

Analyze text content and extract key insights:

```typescript
import { init, LlmConfig, Node, Workflow, Executor } from '@infinitibit_gmbh/graphbit';

async function simpleTextAnalysis() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Create workflow
  const workflow = new Workflow('Text Analysis');
  
  const analyzer = Node.agent(
    'Text Analyzer',
    'Analyze this text and provide 3 key insights: The rise of artificial intelligence is transforming industries across the globe.',
    'analyzer'
  );

  await workflow.addNode(analyzer);
  await workflow.validate();

  // Execute
  const executor = new Executor(config);
  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    const vars = result.variables();
    console.log('Analysis:', vars.analyzer);
  } else {
    console.error('Error:', result.error());
  }
}

// Run the example
simpleTextAnalysis().catch(console.error);
```

## Example 2: Sequential Pipeline

Create a multi-step content processing pipeline:

```typescript
import { init, LlmConfig, Workflow, Node, Executor } from '@infinitibit_gmbh/graphbit';

async function contentPipeline(topic: string) {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  const workflow = new Workflow('Content Pipeline');

  // Step 1: Research
  const researcher = Node.agent(
    'Researcher',
    `Research 3 key facts about: ${topic}`,
    'researcher'
  );

  // Step 2: Writer
  const writer = Node.agent(
    'Writer',
    `Write a paragraph about ${topic} using the facts from Researcher.`,
    'writer'
  );

  // Add nodes and connect them
  await workflow.addNode(researcher);
  await workflow.addNode(writer);
  await workflow.connect('researcher', 'writer');
  await workflow.validate();

  // Execute workflow
  const executor = new Executor(config);
  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    const vars = result.variables();
    console.log('Research:', vars.researcher);
    console.log('\nFinal Content:', vars.writer);
    return vars.writer;
  } else {
    console.error('Error:', result.error());
    return null;
  }
}

// Usage
contentPipeline('quantum computing')
  .then(content => console.log('Pipeline completed:', content))
  .catch(console.error);
```

## Example 3: Multiple LLM Providers

Use different LLM providers in the same application:

```typescript
import { init, LlmConfig, Node, Workflow, Executor } from '@infinitibit_gmbh/graphbit';

async function multiProviderExample() {
  init();

  // OpenAI for creative tasks
  const openaiConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Anthropic for analytical tasks
  const anthropicConfig = LlmConfig.anthropic({
    apiKey: process.env.ANTHROPIC_API_KEY || '',
    model: 'claude-sonnet-4-20250514'
  });

  // Ollama for local execution
  const ollamaConfig = LlmConfig.ollama({
    model: 'llama3.2'
  });

  // Create separate executors for different providers
  const creativeExecutor = new Executor(openaiConfig);
  const analyticalExecutor = new Executor(anthropicConfig);
  const localExecutor = new Executor(ollamaConfig);

  // Creative workflow with OpenAI
  const creativeWorkflow = new Workflow('Creative Writing');
  const writer = Node.agent(
    'Creative Writer',
    'Write a creative story about: A robot learning to paint',
    'creative_writer'
  );
  
  await creativeWorkflow.addNode(writer);
  await creativeWorkflow.validate();

  // Analytical workflow with Anthropic
  const analyticalWorkflow = new Workflow('Analysis');
  const analyzer = Node.agent(
    'Data Analyzer',
    'Analyze this data and provide insights: Sales increased 25% in Q4 2024',
    'analyzer'
  );
  
  await analyticalWorkflow.addNode(analyzer);
  await analyticalWorkflow.validate();

  // Execute with different providers
  console.log('🎨 Running creative workflow with OpenAI...');
  const creativeResult = await creativeExecutor.execute(creativeWorkflow);
  
  console.log('📊 Running analytical workflow with Anthropic...');
  const analyticalResult = await analyticalExecutor.execute(analyticalWorkflow);

  if (creativeResult.isSuccess()) {
    console.log('\nCreative (OpenAI):', creativeResult.variables().creative_writer);
  }

  if (analyticalResult.isSuccess()) {
    console.log('\nAnalytical (Anthropic):', analyticalResult.variables().analyzer);
  }

  return { creativeResult, analyticalResult };
}

// Run the example
multiProviderExample().catch(console.error);
```

## Example 4: Error Handling and Performance Optimization

Build robust workflows with error handling and optimized performance:

```typescript
import { init, LlmConfig, Node, Workflow, Executor } from '@infinitibit_gmbh/graphbit';

async function robustWorkflowExample() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Create high-throughput executor
  const executor = Executor.newHighThroughput(config);
  
  // Alternative: Create low-latency executor for real-time
  // const executor = Executor.newLowLatency(config);

  const workflow = new Workflow('Robust Workflow');

  const agent = Node.agent(
    'Reliable Agent',
    'Process this data reliably: Analyze market trends for 2025',
    'reliable'
  );

  await workflow.addNode(agent);
  await workflow.validate();

  try {
    console.log('⏳ Executing workflow...\n');
    
    const result = await executor.execute(workflow);

    if (result.isSuccess()) {
      const vars = result.variables();
      console.log('✅ Success:', vars.reliable);
      console.log(`\n⏱️  Execution time: ${result.executionTimeMs()}ms`);
    } else {
      console.error('❌ Workflow failed:', result.state());
      console.error('Error:', result.error());
    }
  } catch (error) {
    console.error('💥 Exception:', error);
    
    // Implement retry logic
    console.log('🔄 Retrying with low-latency executor...');
    
    const retryExecutor = Executor.newLowLatency(config);
    const retryResult = await retryExecutor.execute(workflow);
    
    if (retryResult.isSuccess()) {
      console.log('✅ Retry succeeded');
    }
  }
}

// Run the example
robustWorkflowExample().catch(console.error);
```

## Example 5: Embeddings and Similarity

Use embeddings for semantic search and similarity:

```typescript
import { init, EmbeddingConfig, EmbeddingClient } from '@infinitibit_gmbh/graphbit';

async function embeddingsExample() {
  init();

  // Create embedding service
  const embeddingConfig = EmbeddingConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'text-embedding-3-small'
  });

  const service = new EmbeddingClient(embeddingConfig);

  // Process multiple texts
  const texts = [
    'Machine learning is revolutionizing technology',
    'AI is transforming how we work and live',
    'The weather is nice today'
  ];

  console.log('📝 Generating embeddings...\n');

  try {
    const embeddings = await service.embedMany(texts);

    console.log('✅ Embeddings generated successfully\n');

    embeddings.forEach((embedding, i) => {
      console.log(`Text ${i + 1}: ${texts[i]}`);
      console.log(`Embedding dimensions: ${embedding.length}`);
      console.log(`First 5 values: [${embedding.slice(0, 5).join(', ')}...]\n`);
    });

    // Calculate cosine similarity between first two texts
    const similarity = cosineSimilarity(embeddings[0], embeddings[1]);
    console.log(`📊 Similarity between text 1 and 2: ${similarity.toFixed(4)}`);

    const similarity2 = cosineSimilarity(embeddings[0], embeddings[2]);
    console.log(`📊 Similarity between text 1 and 3: ${similarity2.toFixed(4)}`);

  } catch (error) {
    console.error('❌ Error generating embeddings:', error);
  }
}

// Utility function for cosine similarity
function cosineSimilarity(vecA: number[], vecB: number[]): number {
  let dotProduct = 0;
  let normA = 0;
  let normB = 0;

  for (let i = 0; i < vecA.length; i++) {
    dotProduct += vecA[i] * vecB[i];
    normA += vecA[i] * vecA[i];
    normB += vecB[i] * vecB[i];
  }

  return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
}

// Run the example
embeddingsExample().catch(console.error);
```

## Example 6: System Monitoring and Diagnostics

Monitor GraphBit performance and health:

```typescript
import { 
  init, 
  LlmConfig, 
  Node, 
  Workflow, 
  Executor,
  getSystemInfo,
  healthCheck 
} from '@infinitibit_gmbh/graphbit';

async function systemMonitoringExample() {
  init();

  // Get system information
  const systemInfo = getSystemInfo();
  console.log('🖥️  System Information:');
  console.log(`   GraphBit version: ${systemInfo.version}`);
  console.log(`   JavaScript binding version: ${systemInfo.jsBindingVersion}`);
  console.log(`   CPU count: ${systemInfo.cpuCount}`);
  console.log(`   Memory allocator: ${systemInfo.memoryAllocator}`);
  console.log(`   Runtime initialized: ${systemInfo.runtimeInitialized}`);

  // Perform health check
  const health = healthCheck();
  console.log('\n🏥 Health Check:');
  console.log(`   Overall healthy: ${health.overallHealthy}`);
  console.log(`   Runtime healthy: ${health.runtimeHealthy}`);
  console.log(`   Memory healthy: ${health.memoryHealthy}`);
  if (health.availableMemoryMb) {
    console.log(`   Available memory: ${health.availableMemoryMb} MB`);
  }

  // Create and monitor an executor
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  const executor = new Executor(config);

  // Execute a simple workflow to generate stats
  const workflow = new Workflow('Monitoring Test');
  
  const agent = Node.agent(
    'Monitor Agent',
    'Say hello and describe your capabilities',
    'monitor'
  );
  
  await workflow.addNode(agent);
  await workflow.validate();

  console.log('\n⏳ Executing monitoring test workflow...\n');

  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    console.log('✅ Workflow executed successfully');
    console.log(`⏱️  Execution time: ${result.executionTimeMs()}ms`);
    console.log(`📊 Workflow state: ${result.state()}`);
    console.log('\n💬 Agent Response:');
    console.log(result.variables().monitor);
  } else {
    console.error('❌ Workflow failed:', result.error());
  }
}

// Run monitoring example
systemMonitoringExample().catch(console.error);
```

## Example 7: Workflow Validation

Validate workflow structure before execution:

```typescript
import { init, LlmConfig, Node, Workflow, Executor } from '@infinitibit_gmbh/graphbit';

async function workflowValidationExample() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Create a workflow
  const workflow = new Workflow('Validation Test');

  // Add nodes
  const node1 = Node.agent(
    'First Agent',
    'Process this input: Start of the pipeline',
    'agent1'
  );

  const node2 = Node.agent(
    'Second Agent',
    'Continue processing from the first agent',
    'agent2'
  );

  await workflow.addNode(node1);
  await workflow.addNode(node2);
  await workflow.connect('agent1', 'agent2');

  // Validate workflow structure
  try {
    await workflow.validate();
    console.log('✅ Workflow validation passed');

    // Execute the validated workflow
    const executor = new Executor(config);
    const result = await executor.execute(workflow);

    if (result.isSuccess()) {
      const vars = result.variables();
      console.log('\n📝 First Agent:', vars.agent1);
      console.log('\n📝 Second Agent:', vars.agent2);
      console.log('\n✅ Execution completed successfully');
    }
  } catch (error) {
    console.error('❌ Workflow validation failed:', error);
  }
}

// Run validation example
workflowValidationExample().catch(console.error);
```

## Example 8: Tool Integration

Create workflows with custom tools:

```typescript
import { init, LlmConfig, Node, Workflow, Executor, ToolRegistry, registerAsync } from '@infinitibit_gmbh/graphbit';

async function toolIntegrationExample() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  const toolRegistry = new ToolRegistry();

  // Register a custom calculator tool
  registerAsync(
    toolRegistry,
    'calculate',
    'Perform mathematical calculations',
    {
      type: 'object',
      properties: {
        operation: { type: 'string', description: 'Operation: add, subtract, multiply, divide' },
        a: { type: 'number', description: 'First number' },
        b: { type: 'number', description: 'Second number' }
      },
      required: ['operation', 'a', 'b']
    },
    async (params: any) => {
      const { operation, a, b } = params;
      
      switch (operation) {
        case 'add':
          return { result: a + b };
        case 'subtract':
          return { result: a - b };
        case 'multiply':
          return { result: a * b };
        case 'divide':
          return { result: b !== 0 ? a / b : 'Error: Division by zero' };
        default:
          return { error: 'Unknown operation' };
      }
    }
  );

  const workflow = new Workflow('Calculator Workflow');

  const calculator = Node.agent(
    'Calculator Agent',
    'Use the calculate tool to add 15 and 27, then multiply the result by 3',
    'calculator'
  );

  await workflow.addNode(calculator);
  await workflow.validate();

  const executor = new Executor(config);
  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    console.log('🧮 Calculation Result:', result.variables().calculator);
  } else {
    console.error('❌ Error:', result.error());
  }
}

// Run the example
toolIntegrationExample().catch(console.error);
```

## Tips for Getting Started

1. **Start Simple**: Begin with single-node workflows to understand the basics
2. **Use TypeScript**: Leverage type safety for better development experience
3. **Handle Errors**: Always include error handling with try-catch blocks
4. **Test Incrementally**: Add complexity gradually
5. **Monitor Performance**: Use `result.executionTimeMs()` to track execution times
6. **Validate Workflows**: Always call `workflow.validate()` before execution
7. **Check System Health**: Use `healthCheck()` for diagnostics
8. **Use Appropriate Executors**: Choose between low-latency, high-throughput, or balanced modes

## Performance Optimization

- Use `Executor.newLowLatency()` for real-time applications
- Use `Executor.newHighThroughput()` for batch processing
- Use `Executor.newBalanced()` for general-purpose applications
- Cache embeddings when possible
- Implement retry logic for resilience
- Monitor memory usage in long-running applications

## Next Steps

Once you're comfortable with these examples:

- Explore [Core Concepts](../user-guide/concepts-js.md)
- Learn about [Dynamic Graph Generation](../user-guide/dynamics-graph-js.md)
- Check out [Complete Examples](../examples/content-generation-js.md)
- Read the [JavaScript API Reference](../api-reference/javascript-api.md)
- Learn about [Workflow Builder](../user-guide/workflow-builder-js.md)
- Study [Async Patterns](../user-guide/async-vs-sync-js.md)
