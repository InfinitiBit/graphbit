# Core Concepts - JavaScript

This guide covers the fundamental concepts of GraphBit for JavaScript/Node.js developers, explaining key components and how they work together.

## Overview

GraphBit is built around these core concepts:

1. **Library Initialization** - Setting up the GraphBit environment
2. **LLM Providers** - Configuring language model clients
3. **Workflows** - Directed graphs that define execution flow
4. **Nodes** - Individual processing units
5. **Executors** - Engines that run workflows
6. **Tools** - External functions accessible to LLMs
7. **Results** - Accessing execution outcomes

## Library Initialization

Before using GraphBit, initialize the library:

```typescript
import { init, version, getSystemInfo, healthCheck } from '@infinitibit_gmbh/graphbit';

// Initialize the library
init();

// Check version and system status
console.log(`GraphBit version: ${version()}`);
console.log('System info:', getSystemInfo());
console.log('Health check:', healthCheck());
```

### System Information

```typescript
import { init, getSystemInfo } from '@infinitibit_gmbh/graphbit';

init();

const info = getSystemInfo();
console.log('System Information:', {
  nodeVersion: info.nodeVersion,
  cpuCount: info.cpuCount,
  platform: process.platform,
  arch: process.arch
});
```

### Health Checks

```typescript
import { init, healthCheck } from '@infinitibit_gmbh/graphbit';

init();

const health = healthCheck();

if (health.overallHealthy) {
  console.log('✅ System is healthy');
} else {
  console.warn('⚠️ System health issues detected');
  console.log('Memory healthy:', health.memoryHealthy);
  console.log('CPU healthy:', health.cpuHealthy);
}
```

## LLM Providers

GraphBit supports multiple LLM providers with a unified API.

### OpenAI Configuration

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Basic configuration
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || 'sk-...'
});

// With custom model
const customConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini' // Optional, defaults to gpt-4o-mini
});
```

### Anthropic Configuration

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || 'sk-ant-...'
});

// With custom model
const customConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY,
  model: 'claude-3-5-sonnet-20241022'
});
```

### OpenRouter Configuration

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// OpenRouter provides access to 400+ models
const config = LlmConfig.openrouter({
  apiKey: process.env.OPENROUTER_API_KEY
});

// With specific model
const customConfig = LlmConfig.openrouter({
  apiKey: process.env.OPENROUTER_API_KEY,
  model: 'anthropic/claude-3.5-sonnet'
});
```

### Ollama Configuration (Local Models)

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Local Ollama instance
const config = LlmConfig.ollama({
  model: 'llama3.2' // Optional, defaults to llama3.2
});

// Custom Ollama URL
const customConfig = LlmConfig.ollama({
  baseUrl: 'http://localhost:11434',
  model: 'llama3.2:70b'
});
```

### Additional Providers

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// DeepSeek
const deepseekConfig = LlmConfig.deepseek({
  apiKey: process.env.DEEPSEEK_API_KEY
});

// Fireworks AI
const fireworksConfig = LlmConfig.fireworks({
  apiKey: process.env.FIREWORKS_API_KEY
});

// Groq
const groqConfig = LlmConfig.groq({
  apiKey: process.env.GROQ_API_KEY
});

// Together AI
const togetherConfig = LlmConfig.together({
  apiKey: process.env.TOGETHER_API_KEY
});
```

## LLM Client

Execute LLM requests directly without workflows:

### Basic Completion

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function basicCompletion() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  const response = await client.complete('What is the capital of France?');
  console.log('Response:', response);
}

basicCompletion().catch(console.error);
```

### Batch Completions

```typescript
async function batchCompletions() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  const prompts = [
    'Capital of France?',
    'Capital of Germany?',
    'Capital of Italy?'
  ];

  const results = await client.completeBatch(
    prompts,
    100,  // maxTokens
    0.7,  // temperature
    3     // concurrency
  );

  results.forEach((result, i) => {
    console.log(`${prompts[i]} → ${result}`);
  });
}
```

### Streaming Responses

```typescript
async function streamingResponse() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  console.log('Streaming response:');
  
  const stream = await client.stream('Write a short poem');

  for await (const chunk of stream) {
    process.stdout.write(chunk);
  }

  console.log('\nDone');
}
```

## Workflows

Workflows define the execution structure as a directed acyclic graph (DAG).

### Creating Workflows

```typescript
import { Workflow } from '@infinitibit_gmbh/graphbit';

const workflow = new Workflow('MyWorkflow');
console.log('Workflow created:', workflow);
```

### Adding Nodes

```typescript
import { Workflow, Node } from '@infinitibit_gmbh/graphbit';

async function createWorkflow() {
  const workflow = new Workflow('SimpleWorkflow');

  // Create an agent node
  const agent = Node.agent(
    'Analyzer',           // name
    'Analyze this input', // prompt
    'analyzer'           // agent_id (unique identifier)
  );

  // Add node to workflow
  await workflow.addNode(agent);

  return workflow;
}
```

### Connecting Nodes

```typescript
async function createConnectedWorkflow() {
  const workflow = new Workflow('ConnectedWorkflow');

  const node1 = Node.agent('Agent 1', 'First task', 'agent_1');
  const node2 = Node.agent('Agent 2', 'Second task', 'agent_2');
  const node3 = Node.agent('Agent 3', 'Third task', 'agent_3');

  await workflow.addNode(node1);
  await workflow.addNode(node2);
  await workflow.addNode(node3);

  // Connect nodes: agent_1 → agent_2 → agent_3
  await workflow.connect('agent_1', 'agent_2');
  await workflow.connect('agent_2', 'agent_3');

  return workflow;
}
```

### Validating Workflows

```typescript
async function validateWorkflow() {
  const workflow = new Workflow('ValidatedWorkflow');

  const node1 = Node.agent('Agent 1', 'Task', 'agent_1');
  await workflow.addNode(node1);

  // Always validate before execution
  try {
    await workflow.validate();
    console.log('✅ Workflow is valid');
  } catch (error) {
    console.error('❌ Workflow validation failed:', error);
  }
}
```

## Nodes

Nodes are the building blocks of workflows. Different node types serve different purposes.

### Agent Nodes

Agent nodes execute LLM-based tasks:

```typescript
import { Node } from '@infinitibit_gmbh/graphbit';

// Simple agent
const agent = Node.agent(
  'Summarizer',
  'Summarize the following text',
  'summarizer'
);

// Agent with context variables
const contextAgent = Node.agent(
  'Analyzer',
  'Analyze the data: {{data}}',  // Template with variable
  'analyzer'
);
```

### Task Nodes

Task nodes execute computational tasks (not yet available in JavaScript bindings, Python-only feature):

```typescript
// Note: Task nodes are available in Python but not yet in JavaScript
// Use agent nodes with specific prompts for computational tasks
```

### Condition Nodes

Condition nodes enable branching logic:

```typescript
const condition = Node.condition(
  'Checker',
  'Evaluate if condition is met',
  'checker'
);

// Connect with conditions
await workflow.addNode(condition);
await workflow.addNode(yesPath);
await workflow.addNode(noPath);

await workflow.connectCondition('checker', 'yes_path', true);
await workflow.connectCondition('checker', 'no_path', false);
```

### Parallel Nodes

Parallel nodes execute multiple sub-nodes concurrently:

```typescript
const parallel = Node.parallel('ParallelTasks', [
  Node.agent('Task A', 'Do A', 'task_a'),
  Node.agent('Task B', 'Do B', 'task_b'),
  Node.agent('Task C', 'Do C', 'task_c')
]);

await workflow.addNode(parallel);
```

## Executors

Executors run workflows with different performance characteristics.

### Default Executor

```typescript
import { Executor } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
});

const executor = new Executor(config);
```

### Low-Latency Executor

Optimized for fast response times (single thread):

```typescript
const executor = Executor.newLowLatency(config);
```

### High-Throughput Executor

Optimized for batch processing (multi-thread):

```typescript
const executor = Executor.newHighThroughput(config);
```

### Executing Workflows

```typescript
async function executeWorkflow() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const executor = new Executor(config);

  const workflow = new Workflow('ExecutableWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  // Execute workflow
  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    console.log('✅ Success:', result.variables());
  } else {
    console.error('❌ Failed:', result.error());
  }
}
```

## Tools

Tools extend LLM capabilities with custom functions.

### Creating Tool Registry

```typescript
import { ToolRegistry } from '@infinitibit_gmbh/graphbit';

const registry = new ToolRegistry();
```

### Registering Tools

```typescript
async function registerCalculator() {
  init();

  const registry = new ToolRegistry();

  registry.register('add', 'Add two numbers', {
    a: { type: 'number' },
    b: { type: 'number' }
  }, (args: any) => {
    return { result: args.a + args.b };
  });

  console.log('Tool registered');
}
```

### Executing Tools

```typescript
async function executeTool() {
  init();

  const registry = new ToolRegistry();

  registry.register('multiply', 'Multiply two numbers', {
    x: { type: 'number' },
    y: { type: 'number' }
  }, (args: any) => {
    return args.x * args.y;
  });

  const result = await registry.execute('multiply', { x: 5, y: 3 });
  console.log('Result:', result); // 15
}
```

### Listing Tools

```typescript
async function listTools() {
  init();

  const registry = new ToolRegistry();

  // For sync tools, use register() directly
  registry.register('tool1', 'First tool (sync)', {}, () => 'result1');

  // For async tools, use registerAsync() for proper timing
  // This ensures the Promise is handled correctly by the Rust core
  registerAsync(registry, 'asyncTool', 'Async tool', {}, async () => {
    await someAsyncOperation();
    return 'result';
  });

  registry.register('tool2', 'Second tool (sync)', {}, () => 'result2');

  const tools = registry.getRegisteredTools();
  console.log('Available tools:', tools);
}
```

> **Note:** For async callbacks (that use `await` or return Promises), always use `registerAsync()`. See [Tool Calling Guide](./tool-calling-js.md#async-callbacks-callback-id-pattern) for details.

## Results

Access workflow execution results:

### Checking Success

```typescript
const result = await executor.execute(workflow);

if (result.isSuccess()) {
  console.log('✅ Workflow succeeded');
  const variables = result.variables();
  console.log('Variables:', variables);
} else {
  console.error('❌ Workflow failed');
  const error = result.error();
  console.error('Error:', error);
}
```

### Accessing Variables

```typescript
const result = await executor.execute(workflow);

if (result.isSuccess()) {
  const vars = result.variables();
  
  // Access specific variables from the workflow context
  console.log('Output:', vars.output);
  console.log('Status:', vars.status);
}
```

### Error Handling

```typescript
try {
  const result = await executor.execute(workflow);

  if (!result.isSuccess()) {
    const errorMsg = result.error();
    console.error('Workflow error:', errorMsg);
    
    // Implement recovery logic
    handleWorkflowError(errorMsg);
  }
} catch (error) {
  console.error('Execution exception:', error);
}
```

## Complete Example

Putting it all together:

```typescript
import { 
  init, 
  LlmConfig, 
  Executor, 
  Workflow, 
  Node,
  ToolRegistry,
  registerAsync 
} from '@infinitibit_gmbh/graphbit';

async function completeExample() {
  // 1. Initialize
  init();

  // 2. Configure LLM
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // 3. Create executor
  const executor = new Executor(config);

  // 4. Register tools
  const registry = new ToolRegistry();
  
  // Register async tool
  registerAsync(registry, 'get_weather', 'Get current weather for a location', {
    location: { type: 'string' }
  }, async (args: any) => {
    // Simulate async API call
    await new Promise(resolve => setTimeout(resolve, 100));
    return { 
      location: args.location,
      temperature: 72,
      condition: 'sunny'
    };
  });

  // 5. Build workflow
  const workflow = new Workflow('WeatherWorkflow');

  const locationAgent = Node.agent(
    'Location Parser',
    'Extract the location from user query',
    'location_parser'
  );

  const weatherAgent = Node.agent(
    'Weather Fetcher',
    'Get weather for location using get_weather tool',
    'weather_fetcher'
  );

  const responseAgent = Node.agent(
    'Response Generator',
    'Generate friendly response with weather info',
    'response_generator'
  );

  await workflow.addNode(locationAgent);
  await workflow.addNode(weatherAgent);
  await workflow.addNode(responseAgent);

  await workflow.connect('location_parser', 'weather_fetcher');
  await workflow.connect('weather_fetcher', 'response_generator');

  // 6. Validate
  await workflow.validate();

  // 7. Execute
  const result = await executor.execute(workflow);

  // 8. Handle result
  if (result.isSuccess()) {
    console.log('✅ Workflow completed successfully');
    console.log('Result:', result.variables());
  } else {
    console.error('❌ Workflow failed:', result.error());
  }
}

completeExample().catch(console.error);
```

## Async/Await Pattern

All GraphBit operations are async:

```typescript
// ❌ Wrong - missing await
const result = executor.execute(workflow);
console.log(result); // Prints Promise object

// ✅ Correct - with await
const result = await executor.execute(workflow);
console.log(result); // Prints actual result

// ✅ Correct - with .then()
executor.execute(workflow)
  .then(result => console.log(result))
  .catch(error => console.error(error));

// ✅ Correct - with try/catch
try {
  const result = await executor.execute(workflow);
  console.log(result);
} catch (error) {
  console.error(error);
}
```

## Common Patterns

### Sequential Workflow

```typescript
const workflow = new Workflow('Sequential');
await workflow.addNode(Node.agent('Step 1', 'First', 'step1'));
await workflow.addNode(Node.agent('Step 2', 'Second', 'step2'));
await workflow.addNode(Node.agent('Step 3', 'Third', 'step3'));

await workflow.connect('step1', 'step2');
await workflow.connect('step2', 'step3');
```

### Parallel Workflow

```typescript
const workflow = new Workflow('Parallel');
const parallel = Node.parallel('ParallelWork', [
  Node.agent('Task A', 'Do A', 'a'),
  Node.agent('Task B', 'Do B', 'b'),
  Node.agent('Task C', 'Do C', 'c')
]);

await workflow.addNode(parallel);
```

### Branching Workflow

```typescript
const workflow = new Workflow('Branching');
const decision = Node.condition('Decide', 'Check condition', 'check');
const yesPath = Node.agent('Yes', 'Handle yes', 'yes');
const noPath = Node.agent('No', 'Handle no', 'no');

await workflow.addNode(decision);
await workflow.addNode(yesPath);
await workflow.addNode(noPath);

await workflow.connectCondition('check', 'yes', true);
await workflow.connectCondition('check', 'no', false);
```

## Best Practices

1. **Always call `init()` before using GraphBit**
2. **Validate workflows before execution**
3. **Use meaningful node IDs** (unique and descriptive)
4. **Handle errors explicitly** with try/catch
5. **Choose appropriate executor type** for your use case
6. **Use environment variables** for API keys
7. **Check system health** in production
8. **Implement retry logic** for resilience

## See Also

- [JavaScript API Reference](../api-reference/javascript-api.md)
- [Workflow Builder Guide](./workflow-builder-js.md)
- [Tool Calling Guide](./tool-calling-js.md)
- [Async Patterns](./async-vs-sync-js.md)
- [LLM Providers Guide](./llm-providers-js.md)
