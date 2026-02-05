# Tool Calling Guide - JavaScript

This comprehensive guide covers GraphBit's powerful tool calling system for JavaScript, which allows you to create functions that can be executed by LLM agents within workflows.

## Overview

Tools are functions that extend LLM capabilities. GraphBit allows you to:
- Register custom tools with the ToolRegistry
- Define input schemas for type safety
- Execute tools automatically via LLM agents
- Execute tools manually for testing
- Monitor tool performance

## Quick Start

Here's a simple example to get you started with tool calling:

```typescript
import 'dotenv/config';
import { init, LlmConfig, Executor, WorkflowBuilder, AgentBuilder, ToolRegistry } from '@infinitibit_gmbh/graphbit';

// Initialize
init();

// Configure LLM
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});
const executor = new Executor(config);

// Create tool registry
const registry = new ToolRegistry();

// Register tools
registry.register('get_weather', 'Get current weather information for any city', {
  location: { type: 'string' }
}, (args) => {
  return {
    location: args.location,
    temperature: 72,
    condition: 'sunny'
  };
});

registry.register('add', 'Add two numbers together', {
  a: { type: 'number' },
  b: { type: 'number' }
}, (args) => {
  return args.a + args.b;
});

// Create workflow with tool-enabled agent
const agent = await new AgentBuilder('Smart Agent', config)
  .systemPrompt('Answer: What is the weather in Paris? And what is 15 + 27?')
  .build();

// Get agent metadata
const agentId = await agent.id();
const agentName = await agent.name();
const agentDesc = await agent.description();

// Build workflow
const workflow = await new WorkflowBuilder('Tool Calling Example')
  .description('Example workflow demonstrating tool calling')
  .build();

// Add agent node
await workflow.addNode({
  id: agentId.uuid,
  name: agentName,
  description: agentDesc,
  nodeType: 'Agent'
});

// Execute workflow
const result = await executor.execute(workflow);

if (result.isSuccess()) {
  console.log('Result:', result.variables());
}
```

## Tool Registration

### Basic Tool Registration

```typescript
const registry = new ToolRegistry();

registry.register('add', 'Add two numbers together', {
  a: { type: 'number' },
  b: { type: 'number' }
}, (args) => {
  return args.a + args.b;
});
```

### Advanced Tool with Complex Parameters

```typescript
registry.register('search_data', 'Search and filter data with advanced options', {
  query: { type: 'string' },
  filters: { type: 'object' },
  sortBy: { type: 'string' },
  limit: { type: 'integer' }
}, (args) => {
  const { query, filters = {}, sortBy = 'relevance', limit = 10 } = args;
  
  // Simulate search logic
  const results = [
    { id: 1, title: 'Item 1', category: 'A', price: 10 },
    { id: 2, title: 'Item 2', category: 'B', price: 20 }
  ];
  
  return {
    query,
    results: results.slice(0, limit),
    total: results.length,
    sortBy
  };
});
```

### Tool with External API Integration (Async)

For proper async support with correct timing tracking, use `registerAsync` from `async-helpers`:

```typescript
import fetch from 'node-fetch';
import { ToolRegistry, registerAsync } from './async-helpers';

const registry = new ToolRegistry();

// ✅ CORRECT: Use registerAsync for async operations
registerAsync(registry, 'fetch_github_user', 'Fetch GitHub user information by username', {
  username: { type: 'string' }
}, async (args) => {
  const response = await fetch(`https://api.github.com/users/${args.username}`);
  
  if (!response.ok) {
    throw new Error(`User ${args.username} not found`);
  }
  
  const data = await response.json();
  return {
    username: data.login,
    name: data.name,
    bio: data.bio,
    publicRepos: data.public_repos,
    followers: data.followers
  };
});

// Execute - timing includes full API call duration
const result = await registry.execute('fetch_github_user', { username: 'torvalds' });
console.log(result.executionTimeMs); // Actual API call time
```

> **Important:** Always use `registerAsync` for async operations. Using `registry.register` with a Promise directly will result in incorrect timing and the Promise serializing as `{}`.

## Tool Execution

### Automatic Execution in Workflows

Tools are automatically made available to agents when registered:

```typescript
// Create agent with prompt that will trigger tools
const agent = await new AgentBuilder('Data Analyst', config)
  .systemPrompt('Find information about user "torvalds" on GitHub and add 10 + 5')
  .build();

// Get agent metadata
const agentId = await agent.id();

// Build workflow
const workflow = await new WorkflowBuilder('Data Analysis')
  .description('Data analysis with tool calling')
  .build();

// Add agent node
await workflow.addNode({
  id: agentId.uuid,
  name: await agent.name(),
  description: await agent.description(),
  nodeType: 'Agent'
});

const result = await executor.execute(workflow);
```

### Manual Tool Execution

You can also execute tools directly for testing:

```typescript
const registry = new ToolRegistry();

// Register tool
registry.register('double_number', 'Double a number', {
  value: { type: 'number' }
}, (args) => args.value * 2);

// Execute manually
const result = await registry.execute('double_number', { value: 5 });
console.log('Result:', result); // 10
```

## Tool Management

### List All Registered Tools

```typescript
const toolNames = registry.getRegisteredTools();
console.log('Available tools:', toolNames);
```

### Check Tool Registration

```typescript
const isRegistered = registry.hasTool('get_weather');
console.log('Tool registered:', isRegistered); // true or false

const hasTools = registry.getToolCount() > 0;
console.log('Has registered tools:', hasTools); // true or false
```

### Unregister Tool

```typescript
const removed = registry.unregisterTool('double_number');
console.log('Tool removed:', removed);
```

### Clear All Tools

```typescript
registry.clearAll();
console.log('All tools cleared');
```

## Performance and Monitoring

### Get Tool Metrics

```typescript
const metadata = registry.getToolMetadata('get_weather');
console.log('Tool metrics:', {
  callCount: metadata.callCount,
  avgDurationMs: metadata.avgDurationMs,
  lastCalledAt: metadata.lastCalledAt
});
```

### Error Handling in Tool Handlers

Always handle errors gracefully:

```typescript
registry.register('risky_operation', 'An operation that might fail', {
  value: { type: 'string' }
}, (args) => {
  try {
    if (!args.value) {
      throw new Error('Value required');
    }
    // Process the value
    const result = args.value.toUpperCase();
    return { success: true, result };
  } catch (error) {
    return { success: false, error: error.message };
  }
});
```



### Tool Execution with Timeout (Async)

```typescript
import { ToolRegistry, registerAsync } from './async-helpers';

const registry = new ToolRegistry();

// ✅ Use registerAsync with async/await for timeout handling
registerAsync(registry, 'slow_operation', 'A slow operation with timeout', {
  data: { type: 'string' }
}, async (args) => {
  const timeout = new Promise((_, reject) =>
    setTimeout(() => reject(new Error('Operation timeout')), 5000)
  );

  const operation = new Promise(resolve => {
    setTimeout(() => resolve({ processed: args.data }), 1000);
  });

  const result = await Promise.race([operation, timeout]);
  return { success: true, result };
});

// Error handling is automatic - errors become result.error
const result = await registry.execute('slow_operation', { data: 'test' });
if (!result.success) {
  console.log('Timeout:', result.error);
}
```

## Async Callbacks (Callback ID Pattern)

### Why Use Async Helpers?

Due to NAPI-RS limitations, registering async callbacks directly doesn't work correctly:

```typescript
// ❌ DON'T: This won't track timing correctly
registry.register('bad_async', 'Bad async', {}, async (args) => {
  await new Promise(r => setTimeout(r, 1000));
  return { done: true };
});
// result.executionTimeMs will be ~0, not ~1000!
```

### Solution: Use registerAsync

```typescript
import { ToolRegistry, registerAsync, wrapAsync } from './async-helpers';

const registry = new ToolRegistry();

// ✅ DO: Use registerAsync for proper async support
registerAsync(registry, 'good_async', 'Good async', {}, async (args) => {
  await new Promise(r => setTimeout(r, 1000));
  return { done: true };
});
// result.executionTimeMs will be ~1000 ✓
```

### Async Error Handling

Errors in async callbacks are properly captured:

```typescript
registerAsync(registry, 'may_fail', 'May fail', {}, async (args) => {
  if (args.shouldFail) {
    throw new Error('Intentional failure');
  }
  return { success: true };
});

const result = await registry.execute('may_fail', { shouldFail: true });
console.log(result.success);  // false
console.log(result.error);    // "Intentional failure"
```

**Special error cases handled automatically:**
- `throw null` → error: "null was thrown"
- `throw undefined` → error: "undefined was thrown"
- `throw { code: 500 }` → error: serialized JSON

### Value Sanitization

Non-JSON-serializable values are automatically converted:

| Return Value | Becomes |
|--------------|--------|
| `Infinity` | `null` |
| `-Infinity` | `null` |
| `NaN` | `null` |
| `undefined` | `null` |

### Concurrent Async Calls

Multiple async calls work correctly in parallel:

```typescript
registerAsync(registry, 'slow_op', 'Slow', {}, async (args) => {
  await new Promise(r => setTimeout(r, 100));
  return { id: args.id };
});

// All execute concurrently with correct results
const [r1, r2, r3] = await Promise.all([
  registry.execute('slow_op', { id: 1 }),
  registry.execute('slow_op', { id: 2 }),
  registry.execute('slow_op', { id: 3 }),
]);
```

### Importing Async Helpers

```typescript
// TypeScript/ES Modules
import { ToolRegistry, registerAsync, wrapAsync } from './async-helpers';

// CommonJS
const { ToolRegistry, registerAsync, wrapAsync } = require('./async-helpers');
```

> **Note:** Import from `async-helpers`, not `index`. The `index.js` is auto-generated by NAPI-RS and doesn't include the async helpers.

### Get Tool Execution Result

```typescript
const output = await registry.execute('add', { a: 5, b: 3 });

if (typeof output === 'object' && output.success === false) {
  console.log('Tool error:', output.error);
} else {
  console.log('Success:', output);
}
```

## Complex Workflows with Tools

### Multi-Step Workflow with Multiple Tools

```typescript
const registry = new ToolRegistry();

// Register multiple tools
registry.register('fetch_data', 'Fetch data from API', {
  endpoint: { type: 'string' }
}, (args) => {
  return { data: 'sample data from ' + args.endpoint };
});

registry.register('process_data', 'Process fetched data', {
  data: { type: 'string' }
}, (args) => {
  return { processed: args.data.toUpperCase() };
});

registry.register('save_results', 'Save processed results', {
  result: { type: 'string' }
}, (args) => {
  console.log('Saving:', args.result);
  return { saved: true };
});

// Create multi-step workflow
const fetchAgent = await new AgentBuilder('Data Fetcher', config)
  .systemPrompt('Fetch user data from API')
  .build();

const processAgent = await new AgentBuilder('Data Processor', config)
  .systemPrompt('Process the fetched data')
  .build();

const saveAgent = await new AgentBuilder('Data Saver', config)
  .systemPrompt('Save processed data')
  .build();

// Get agent metadata
const fetchAgentId = await fetchAgent.id();
const processAgentId = await processAgent.id();
const saveAgentId = await saveAgent.id();

// Build workflow
const workflow = await new WorkflowBuilder('Data Pipeline')
  .description('Multi-step data processing pipeline')
  .build();

// Add nodes
const node1 = await workflow.addNode({
  id: fetchAgentId.uuid,
  name: await fetchAgent.name(),
  description: await fetchAgent.description(),
  nodeType: 'Agent'
});

const node2 = await workflow.addNode({
  id: processAgentId.uuid,
  name: await processAgent.name(),
  description: await processAgent.description(),
  nodeType: 'Agent'
});

const node3 = await workflow.addNode({
  id: saveAgentId.uuid,
  name: await saveAgent.name(),
  description: await saveAgent.description(),
  nodeType: 'Agent'
});

// Connect nodes
await workflow.addEdge(node1, node2);
await workflow.addEdge(node2, node3);

// Execute pipeline
const result = await executor.execute(workflow);
if (result.isSuccess()) {
  console.log('Pipeline completed successfully');
}
```

## Best Practices

### 1. Clear Descriptions

Provide clear, detailed descriptions for both tools and parameters:

```typescript
registry.register('convert_currency', 'Convert amount from one currency to another using current exchange rates', {
  amount: { type: 'number' },
  from: { type: 'string' },
  to: { type: 'string' }
}, (args) => { /* ... */ });
```

### 2. Input Validation

Validate inputs in your handler:

```typescript
handler: async (params) => {
  if (params.amount <= 0) {
    return { success: false, error: 'Amount must be positive' };
  }
  
  if (!['USD', 'EUR', 'GBP'].includes(params.from)) {
    return { success: false, error: 'Invalid source currency' };
  }
  // Continue processing
}
```

### 3. Schema Validation

Use JSON schema validation for parameters:

```typescript
handler: async (params) => {
  if (!params.email || !params.email.includes('@')) {
    return { success: false, error: 'Invalid email format' };
  }
  // Continue processing
}
```

### 4. Comprehensive Schema

Define complete JSON schemas for better LLM understanding:

```typescript
inputSchema: {
  type: 'object',
  properties: {
    query: {
      type: 'string',
      description: 'Search query',
      minLength: 1,
      maxLength: 200
    },
    limit: {
      type: 'integer',
      description: 'Maximum results',
      minimum: 1,
      maximum: 100,
      default: 10
    },
    includeMetadata: {
      type: 'boolean',
      description: 'Include metadata in results',
      default: false
    }
  },
  required: ['query']
}
```

### 5. Return Structured Data

Return consistent, structured responses:

```typescript
handler: async (params) => {
  try {
    const data = await fetchData(params);
    return {
      success: true,
      data,
      timestamp: new Date().toISOString()
    };
  } catch (error) {
    return {
      success: false,
      error: error.message,
      timestamp: new Date().toISOString()
    };
  }
}
```

## Troubleshooting

### Tools Not Found by Agent

Ensure tools are registered BEFORE creating the workflow:

```typescript
// ✅ CORRECT: Register tools first
registry.register('tool_name', 'Tool description', { /* params */ }, (args) => { /* handler */ });

// Then create workflow with WorkflowBuilder
const workflow = await new WorkflowBuilder('My Workflow')
  .description('Workflow using registered tools')
  .build();
// ...
```

### Tool Execution Timeout

Increase timeout for slow operations or optimize the tool:

```typescript
// For long-running operations, return early with status
handler: async (params) => {
  const startTime = Date.now();
  const maxDuration = 25000; // 25 seconds
  
  while (Date.now() - startTime < maxDuration) {
    // Process in chunks
  }
  
  return { success: true, result };
}
```

### Tools Not Available

Check if registry is properly configured:

```typescript
// ✅ Global registration (available to all agents)
const registry = new ToolRegistry();
registry.register('tool_name', 'Tool description', { /* params */ }, (args) => { /* handler */ });

// Then use in workflow with AgentBuilder
const agent = await new AgentBuilder('Agent', config)
  .systemPrompt('prompt')
  .build();
```

## Examples

See the examples directory for complete working examples:
- `tool-registration.ts` - Tool registration examples
- `tool-execution.ts` - Manual and automatic tool execution
- `tool-monitoring.ts` - Tool metrics and monitoring

## References

- [JavaScript API Reference](../api-reference/javascript-api.md)
- [Workflow Builder Guide](./workflow-builder.md)
- [Agents Guide](./agents-js.md)
