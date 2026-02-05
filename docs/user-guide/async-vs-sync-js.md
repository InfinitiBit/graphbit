# Async vs Sync Usage in GraphBit JavaScript API

GraphBit's JavaScript bindings provide a fully asynchronous interface based on Promises and async/await, allowing you to write modern, non-blocking code for your applications—whether it's a quick script, a web server, or a data pipeline.

---

## Overview

- **Asynchronous (Async) functions** return Promises immediately and execute in the background via the Tokio runtime, allowing your program to handle concurrent operations efficiently.
- **All GraphBit operations are async** - There are no blocking synchronous operations in the JavaScript API.
- **Async/Await Pattern** - Use `async`/`await` syntax for clean, readable code that handles asynchronous operations.

GraphBit integrates seamlessly with Node.js event loop, making it ideal for web servers, pipelines, and high-throughput applications.

---

## Supported Async Functions

### Core Module Functions

| Function | Type | Returns |
|----------|------|---------|
| `init(options?)` | Sync | `void` |
| `version()` | Sync | `string` |
| `getSystemInfo()` | Sync | `object` |
| `healthCheck()` | Sync | `object` |

### LLM Client

| Function | Type | Returns |
|----------|------|---------|
| `complete(prompt, maxTokens?, temperature?)` | Async | `Promise<string>` |
| `completeFull(prompt, maxTokens?, temperature?)` | Async | `Promise<object>` |
| `completeBatch(prompts, maxTokens?, temperature?, concurrency?)` | Async | `Promise<string[]>` |
| `completeStream(prompt, maxTokens?, temperature?)` | Async | `Promise<AsyncIterator>` |
| `getStats()` | Sync | `object` |
| `resetStats()` | Sync | `void` |
| `warmup()` | Async | `Promise<void>` |

### Embeddings Client

| Function | Type | Returns |
|----------|------|---------|
| `embed(text)` | Async | `Promise<number[]>` |
| `embedBatch(texts)` | Async | `Promise<number[][]>` |
| `findSimilar(query, topK)` | Async | `Promise<object[]>` |

### Document Loader

| Function | Type | Returns |
|----------|------|---------|
| `loadPdf(path)` | Async | `Promise<Document[]>` |
| `loadText(path)` | Async | `Promise<Document[]>` |
| `loadCsv(path)` | Async | `Promise<Document[]>` |
| `loadJson(path)` | Async | `Promise<Document[]>` |
| `loadHtml(path)` | Async | `Promise<Document[]>` |
| `loadMarkdown(path)` | Async | `Promise<Document[]>` |
| `loadDocx(path)` | Async | `Promise<Document[]>` |

### Workflow

| Function | Type | Returns |
|----------|------|---------|
| `addNode(node)` | Async | `Promise<string>` |
| `addEdge(fromId, toId)` | Async | `Promise<void>` |
| `validate()` | Async | `Promise<string[]>` |
| `setVariable(name, value)` | Async | `Promise<void>` |
| `getVariable(name)` | Async | `Promise<any>` |
| `getAllVariables()` | Async | `Promise<object>` |

### Workflow Executor

| Function | Type | Returns |
|----------|------|---------|
| `execute(workflow, options?)` | Async | `Promise<WorkflowResult>` |

### Workflow Result

| Function | Type | Returns |
|----------|------|---------|
| `isSuccess()` | Sync | `boolean` |
| `isFailed()` | Sync | `boolean` |
| `allOutputs()` | Sync | `object` |
| `getNodeOutput(nodeId)` | Sync | `any` |
| `getVariable(name)` | Sync | `any` |
| `getAllVariables()` | Sync | `object` |
| `error()` | Sync | `string` |
| `stats()` | Sync | `object` |
| `getExecutionDuration()` | Sync | `number` |

### Tool Registry

| Function | Type | Returns |
|----------|------|---------|
| `register(tool)` | Async | `Promise<void>` |
| `unregister(name)` | Async | `Promise<void>` |
| `execute(name, params)` | Async | `Promise<any>` |
| `listTools()` | Async | `Promise<string[]>` |
| `getTool(name)` | Async | `Promise<object>` |
| `getMetrics(name)` | Async | `Promise<object>` |
| `getExecutionHistory(name)` | Async | `Promise<object[]>` |
| `clearHistory()` | Async | `Promise<void>` |

---

## Usage Examples

### Asynchronous Usage - Async/Await Pattern

The modern, recommended approach using async/await:

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function main() {
  init();
  
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);
  
  // Await the async operation
  const result = await client.complete('Hello, world!');
  console.log(result);
}

main().catch(console.error);
```

### Promise Chain Pattern

Alternative approach using Promise chains:

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

init();

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
});

const client = new LlmClient(config);

client.complete('Hello, world!')
  .then(result => console.log(result))
  .catch(error => console.error(error));
```

### Concurrent Operations

Execute multiple operations concurrently:

```typescript
import { init, EmbeddingConfig, EmbeddingClient } from '@infinitibit_gmbh/graphbit';

async function embedMultiple() {
  init();
  
  const config = EmbeddingConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const embeddings = new EmbeddingClient(config);

  // Run multiple embeddings in parallel
  const results = await Promise.all([
    embeddings.embed(['Text 1']),
    embeddings.embed(['Text 2']),
    embeddings.embed(['Text 3'])
  ]);

  console.log('All embeddings:', results);
}

embedMultiple().catch(console.error);
```

### Batch Operations

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function processBatch() {
  init();
  
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  const prompts = [
    'What is AI?',
    'Explain machine learning',
    'What is deep learning?'
  ];

  // Batch processing with concurrency control
  const results = await client.completeBatch(
    prompts,
    100,      // maxTokens
    0.7,      // temperature
    2         // concurrency (2 concurrent requests)
  );

  console.log('Batch results:', results);
}

processBatch().catch(console.error);
```

### Workflow Execution

```typescript
import { 
  init, 
  LlmConfig, 
  Executor, 
  WorkflowBuilder,
  AgentBuilder
} from '@infinitibit_gmbh/graphbit';

async function executeWorkflow() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  // Create agent
  const agent = await new AgentBuilder('Agent', config)
    .systemPrompt('Process input')
    .description('Process input data')
    .build();

  const agentId = await agent.id();

  // Build workflow
  const workflow = await new WorkflowBuilder('MyWorkflow')
    .description('Example workflow')
    .build();
  
  // All workflow operations are async
  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });
  
  await workflow.validate();

  const executor = new Executor(config);
  const result = await executor.execute(workflow);

  if (await result.isCompleted()) {
    console.log('Success!');
  } else {
    console.error('Failed:', await result.error());
  }
}

executeWorkflow().catch(console.error);
```

### Tool Registry Operations

```typescript
import { init, ToolRegistry } from '@infinitibit_gmbh/graphbit';

async function manageTools() {
  init();

  const registry = new ToolRegistry();

  // Register tool (async)
  await registry.register({
    name: 'add',
    description: 'Add two numbers',
    inputSchema: {
      type: 'object',
      properties: {
        a: { type: 'number' },
        b: { type: 'number' }
      }
    },
    handler: async (params) => params.a + params.b
  });

  // Execute tool (async)
  const result = await registry.execute('add', { a: 5, b: 3 });
  console.log('Result:', result);

  // List tools (async)
  const tools = await registry.listTools();
  console.log('Tools:', tools);

  // Get metrics (async)
  const metrics = await registry.getMetrics('add');
  console.log('Metrics:', metrics);
}

manageTools().catch(console.error);
```

### Error Handling

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function withErrorHandling() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  try {
    const result = await client.complete('Hello');
    console.log(result);
  } catch (error) {
    console.error('Error:', error instanceof Error ? error.message : error);
  }
}

withErrorHandling().catch(console.error);
```

### Sequential Operations

Execute operations one after another:

```typescript
import { init, LlmConfig, DocumentLoader, TextSplitter } from '@infinitibit_gmbh/graphbit';

async function processDocument() {
  init();

  const loader = new DocumentLoader();
  const docs = await loader.loadText('./document.txt');

  // Process each document
  for (const doc of docs) {
    const chunks = TextSplitter.recursiveCharacterSplit(
      doc.content,
      { chunkSize: 1000, chunkOverlap: 100 }
    );
    
    console.log(`Document "${doc.title}" split into ${chunks.length} chunks`);
  }
}

processDocument().catch(console.error);
```

### Timeout Handling

```typescript
import { init, LlmConfig, Executor, WorkflowBuilder, AgentBuilder } from '@infinitibit_gmbh/graphbit';

async function withTimeout() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  // Create simple agent
  const agent = await new AgentBuilder('QuickTask', config)
    .systemPrompt('Complete task quickly')
    .build();

  const agentId = await agent.id();

  // Build workflow
  const workflow = await new WorkflowBuilder('QuickTask')
    .description('Quick task workflow')
    .build();

  await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: 'Agent'
  });

  const executor = new Executor(config);

  // Execute with timeout
  const result = await executor.execute(workflow, {
    timeout: 10000  // 10 seconds
  });

  if (await result.isCompleted()) {
    console.log('Completed within timeout');
  } else {
    console.error('Failed or timed out:', await result.error());
  }
}

withTimeout().catch(console.error);
```

## Best Practices

### 1. Always Use Async/Await
Modern, readable code:

```typescript
// ✅ Good
async function main() {
  const result = await client.complete(prompt);
  console.log(result);
}

// ❌ Avoid
client.complete(prompt).then(result => console.log(result));
```

### 2. Handle Errors Properly
Use try/catch for error handling:

```typescript
// ✅ Good
async function safeOperation() {
  try {
    const result = await operation();
    return result;
  } catch (error) {
    console.error('Operation failed:', error);
    throw error;
  }
}
```

### 3. Use Concurrency for Parallel Operations
Run operations concurrently when possible:

```typescript
// ✅ Good - Parallel execution
const [result1, result2, result3] = await Promise.all([
  operation1(),
  operation2(),
  operation3()
]);

// ❌ Avoid - Sequential execution
const result1 = await operation1();
const result2 = await operation2();
const result3 = await operation3();
```

### 4. Set Timeouts for Long Operations
Prevent hanging:

```typescript
const result = await executor.execute(workflow, {
  timeout: 30000  // 30 seconds
});
```

### 5. Resource Cleanup
Properly manage resources:

```typescript
async function cleanup() {
  try {
    // Operations
    const result = await executor.execute(workflow);
  } finally {
    // Cleanup if needed
    console.log('Operation complete');
  }
}
```

## Common Patterns

### Sequential Pipeline

```typescript
async function pipeline(input: string) {
  const step1 = await processor1(input);
  const step2 = await processor2(step1);
  const step3 = await processor3(step2);
  return step3;
}
```

### Parallel Processing

```typescript
async function parallel(inputs: string[]) {
  return Promise.all(
    inputs.map(input => processor(input))
  );
}
```

### Retry Logic

```typescript
async function retryOperation(
  operation: () => Promise<any>,
  maxRetries = 3
) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await operation();
    } catch (error) {
      if (i === maxRetries - 1) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }
}
```

## See Also

- [Workflow Builder](./workflow-builder-js.md)
- [LLM Providers](./llm-providers-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
