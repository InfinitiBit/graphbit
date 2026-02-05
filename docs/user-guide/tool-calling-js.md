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
await registry.register({
  name: 'get_weather',
  description: 'Get current weather information for any city',
  inputSchema: {
    type: 'object',
    properties: {
      location: { type: 'string', description: 'City name' }
    },
    required: ['location']
  },
  handler: async (params) => {
    return {
      location: params.location,
      temperature: 72,
      condition: 'sunny'
    };
  }
});

await registry.register({
  name: 'add',
  description: 'Add two numbers together',
  inputSchema: {
    type: 'object',
    properties: {
      a: { type: 'number' },
      b: { type: 'number' }
    },
    required: ['a', 'b']
  },
  handler: async (params) => {
    return params.a + params.b;
  }
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

await registry.register({
  name: 'add',
  description: 'Add two numbers together',
  inputSchema: {
    type: 'object',
    properties: {
      a: { type: 'number', description: 'First number' },
      b: { type: 'number', description: 'Second number' }
    },
    required: ['a', 'b']
  },
  handler: async (params) => {
    return params.a + params.b;
  }
});
```

### Advanced Tool with Complex Parameters

```typescript
await registry.register({
  name: 'search_data',
  description: 'Search and filter data with advanced options',
  inputSchema: {
    type: 'object',
    properties: {
      query: { 
        type: 'string', 
        description: 'Search query' 
      },
      filters: {
        type: 'object',
        properties: {
          category: { type: 'string' },
          minPrice: { type: 'number' },
          maxPrice: { type: 'number' }
        }
      },
      sortBy: {
        type: 'string',
        enum: ['price', 'date', 'relevance'],
        default: 'relevance'
      },
      limit: {
        type: 'integer',
        minimum: 1,
        maximum: 100,
        default: 10
      }
    },
    required: ['query']
  },
  handler: async (params) => {
    const { query, filters = {}, sortBy = 'relevance', limit = 10 } = params;
    
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
  }
});
```

### Tool with External API Integration

```typescript
import fetch from 'node-fetch';

await registry.register({
  name: 'fetch_github_user',
  description: 'Fetch GitHub user information by username',
  inputSchema: {
    type: 'object',
    properties: {
      username: { type: 'string', description: 'GitHub username' }
    },
    required: ['username']
  },
  handler: async (params) => {
    try {
      const response = await fetch(`https://api.github.com/users/${params.username}`);
      
      if (!response.ok) {
        return { error: `User ${params.username} not found` };
      }
      
      const data = await response.json();
      return {
        username: data.login,
        name: data.name,
        bio: data.bio,
        publicRepos: data.public_repos,
        followers: data.followers
      };
    } catch (error) {
      return { error: error.message };
    }
  }
});
```

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
await registry.register({
  name: 'double_number',
  description: 'Double a number',
  inputSchema: {
    type: 'object',
    properties: {
      value: { type: 'number' }
    }
  },
  handler: async (params) => params.value * 2
});

// Execute manually
const result = await registry.execute('double_number', { value: 5 });
console.log('Result:', result); // 10
```

## Tool Management

### List All Registered Tools

```typescript
const toolNames = await registry.getToolNames();
console.log('Available tools:', toolNames);
```

### Check Tool Registration

```typescript
const isRegistered = await registry.isRegistered('get_weather');
console.log('Tool registered:', isRegistered); // true or false

const hasTools = await registry.hasTools();
console.log('Has registered tools:', hasTools); // true or false
```

### Unregister Tool

```typescript
await registry.unregister('double_number');
console.log('Tool removed');
```

### Clear All Tools

```typescript
await registry.clearAllTools();
console.log('All tools cleared');
```

## Performance and Monitoring

### Get Tool Metrics

```typescript
const metrics = await registry.getMetrics('get_weather');
console.log('Tool metrics:', {
  executionCount: metrics.executionCount,
  averageExecutionTime: metrics.averageExecutionTime,
  lastExecuted: metrics.lastExecuted
});
```

### Error Handling in Tool Handlers

Always handle errors gracefully:

```typescript
await registry.register({
  name: 'risky_operation',
  description: 'An operation that might fail',
  inputSchema: {
    type: 'object',
    properties: {
      value: { type: 'string' }
    }
  },
  handler: async (params) => {
    try {
      // Risky operation
      const result = await performRiskyOperation(params.value);
      return { success: true, result };
    } catch (error) {
      return { success: false, error: error.message };
    }
  }
});
```

### Enable/Disable Tools

Control tool availability without unregistering:

```typescript
// Disable a tool temporarily
await registry.disableTool('fetch_github_user');
console.log('Tool disabled, will not be available to agents');

// Re-enable the tool
await registry.enableTool('fetch_github_user');
console.log('Tool re-enabled');
```

### Update Tool Description

```typescript
await registry.setDescription(
  'get_weather',
  'Get current weather information with details (updated)'
);
```

### Tool Execution with Timeout

```typescript
await registry.register({
  name: 'slow_operation',
  description: 'A slow operation with timeout',
  inputSchema: {
    type: 'object',
    properties: {
      data: { type: 'string' }
    }
  },
  handler: async (params) => {
    const timeout = new Promise((_, reject) =>
      setTimeout(() => reject(new Error('Operation timeout')), 5000)
    );

    const operation = performSlowOperation(params.data);

    try {
      const result = await Promise.race([operation, timeout]);
      return { success: true, result };
    } catch (error) {
      return { success: false, error: error.message };
    }
  }
});
```

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
await registry.register({
  name: 'fetch_data',
  description: 'Fetch data from API',
  inputSchema: { 
    type: 'object',
    properties: {
      endpoint: { type: 'string' }
    }
  },
  handler: async (params) => {
    return { data: 'sample data from ' + params.endpoint };
  }
});

await registry.register({
  name: 'process_data',
  description: 'Process fetched data',
  inputSchema: {
    type: 'object',
    properties: {
      data: { type: 'string' }
    }
  },
  handler: async (params) => {
    return { processed: params.data.toUpperCase() };
  }
});

await registry.register({
  name: 'save_results',
  description: 'Save processed results',
  inputSchema: {
    type: 'object',
    properties: {
      result: { type: 'string' }
    }
  },
  handler: async (params) => {
    console.log('Saving:', params.result);
    return { saved: true };
  }
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
await registry.register({
  name: 'convert_currency',
  description: 'Convert amount from one currency to another using current exchange rates',
  inputSchema: {
    type: 'object',
    properties: {
      amount: { 
        type: 'number', 
        description: 'Amount to convert (must be positive)' 
      },
      from: { 
        type: 'string', 
        description: 'Source currency code (e.g., USD, EUR, GBP)' 
      },
      to: { 
        type: 'string', 
        description: 'Target currency code (e.g., USD, EUR, GBP)' 
      }
    },
    required: ['amount', 'from', 'to']
  },
  handler: async (params) => { /* ... */ }
});
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
await registry.register({ /* tool definition */ });

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
await registry.register({ /* tool */ });

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
