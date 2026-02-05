# Tool System

This document covers the tool registration and execution system in GraphBit JavaScript bindings, allowing you to create custom tools that agents can use.

## Overview

The Tool System provides a registry for custom functions that can be executed by agents or workflows. Tools are defined with parameters, descriptions, and callback functions, making them discoverable and executable at runtime.

## Functions

### `createToolRegistry()`

Create a new tool registry instance.

**Signature:**

```typescript
function createToolRegistry(): ToolRegistry
```

**Returns:** New `ToolRegistry` instance

### 🟢 Verified Example

```javascript
const { createToolRegistry } = require('@infinitibit_gmbh/graphbit');

const registry = createToolRegistry();
```

---

### `tool(name, description, parameters, callback)`

Helper function to create a tool definition.

**Signature:**

```typescript
function tool(
  name: string,
  description: string,
  parameters: any,
  callback: (...args: any[]) => any
): RegisteredToolWrapper
```

**Parameters:**

- `name` (string): Tool name
- `description` (string): Tool description
- `parameters` (any): Parameter schema
- `callback` (function): Tool implementation

**Returns:** `RegisteredToolWrapper` object

### 🟢 Verified Example

```javascript
const { tool } = require('@infinitibit_gmbh/graphbit');

const calculator = tool(
  'calculator',
  'Performs basic math calculations',
  {
    operation: 'string',
    a: 'number',
    b: 'number'
  },
  (args) => {
    const { operation, a, b } = args;
    switch (operation) {
      case 'add': return a + b;
      case 'subtract': return a - b;
      case 'multiply': return a * b;
      case 'divide': return b !== 0 ? a / b : 'Error: Division by zero';
      default: return 'Unknown operation';
    }
  }
);

console.log('Tool:', calculator.name); // "calculator"
```

---

## Class: `ToolRegistry`

Registry for managing and executing tools.

### Methods

#### `register(name, description, parameters, callback)`

Register a tool in the registry.

**Signature:**

```typescript
register(
  name: string,
  description: string,
  parameters: any,
  callback: (...args: any[]) => any
): void
```

**Parameters:**

- `name` (string): Unique tool name
- `description` (string): What the tool does
- `parameters` (any): Parameter schema
- `callback` (function): Tool implementation function

### 🟢 Verified Example

```javascript
const registry = createToolRegistry();

registry.register(
  'greet',
  'Greets a person by name',
  { name: 'string' },
  (args) => `Hello, ${args.name}!`
);
```

---

#### `execute(name, args)`

Execute a registered tool.

**Signature:**

```typescript
async execute(name: string, args: any): Promise<ToolResult>
```

**Parameters:**

- `name` (string): Name of the tool to execute
- `args` (any): Arguments to pass to the tool

**Returns:** Promise resolving to `ToolResult`

**ToolResult Structure:**

```typescript
interface ToolResult {
  success: boolean;          // Whether execution succeeded
  result: any;               // Tool return value
  error?: string;            // Error message if failed
  executionTimeMs: number;   // Execution time in milliseconds
}
```

### 🟢 Verified Example

```javascript
const registry = createToolRegistry();

registry.register(
  'add',
  'Adds two numbers',
  { a: 'number', b: 'number' },
  (args) => args.a + args.b
);

const result = await registry.execute('add', { a: 5, b: 3 });

console.log('Success:', result.success);       // true
console.log('Result:', result.result);         // 8
console.log('Time:', result.executionTimeMs);  // 0.30ms
```

---

#### `getTool(name)`

Get a tool definition by name.

**Signature:**

```typescript
getTool(name: string): any | null
```

**Parameters:**

- `name` (string): Tool name

**Returns:** Tool definition or `null` if not found

### 🟢 Verified Example

```javascript
const registry = createToolRegistry();
registry.register('test', 'Test tool', {}, () => 'test');

const toolDef = registry.getTool('test');

if (toolDef) {
  console.log('Found tool:', toolDef);
}
```

---

#### `hasTool(name)`

Check if atools is registered.

**Signature:**

```typescript
hasTool(name: string): boolean
```

**Parameters:**

- `name` (string): Tool name to check

**Returns:** `true` if tool exists

### 🟢 Verified Example

```javascript
const registry = createToolRegistry();
registry.register('exists', 'Exists', {}, () => true);

console.log(registry.hasTool('exists'));     // true
console.log(registry.hasTool('nonexistent')); // false
```

---

#### `getRegisteredTools()`

Get list of all registered tool names.

**Signature:**

```typescript
getRegisteredTools(): string[]
```

**Returns:** Array of tool names

### 🟢 Verified Example

```javascript
const registry = createToolRegistry();

registry.register('tool1', 'Tool 1', {}, () => 1);
registry.register('tool2', 'Tool 2', {}, () => 2);
registry.register('tool3', 'Tool 3', {}, () => 3);

const tools = registry.getRegisteredTools();
console.log('Registered tools:', tools); // ['tool1', 'tool2', 'tool3']
```

---

#### `unregisterTool(name)`

Unregister a tool by name.

**Signature:**

```typescript
unregisterTool(name: string): boolean
```

**Parameters:**

- `name` (string): Name of the tool to remove

**Returns:** `true` if tool was found and removed

---

#### `getToolMetadata(name)`

Get metadata for a specific tool.

**Signature:**

```typescript
getToolMetadata(name: string): ToolMetadata | null
```

**ToolMetadata:**

```typescript
interface ToolMetadata {
  name: string;
  description: string;
  parametersSchema: any;
  createdAt: number;
  callCount: number;
  totalDurationMs: number;
  avgDurationMs: number;
  lastCalledAt?: number;
}
```

---

#### `getAllMetadata()`

Get metadata for all registered tools.

**Signature:**

```typescript
getAllMetadata(): ToolMetadata[]
```

---

#### `getExecutionHistory()`

Get list of all tool executions.

**Signature:**

```typescript
getExecutionHistory(): ToolExecution[]
```

**ToolExecution:**

```typescript
interface ToolExecution {
  toolName: string;
  success: boolean;
  durationMs: number;
  timestamp: number;
  error?: string;
}
```

---

#### `clearHistory()`

Clear execution history.

**Signature:**

```typescript
clearHistory(): void
```

---

#### `getStats()`

Get comprehensive statistics.

**Signature:**

```typescript
getStats(): ToolStats
```

**ToolStats:**

```typescript
interface ToolStats {
  totalTools: number;
  totalExecutions: number;
  successfulExecutions: number;
  failedExecutions: number;
  avgExecutionTimeMs: number;
  totalExecutionTimeMs: number;
}
```

---

#### `clearAll()`

Clear all tools and history.

**Signature:**

```typescript
clearAll(): void
```

---

#### `getLlmTools()`

Get tools in LLM-compatible format.

**Signature:**

```typescript
getLlmTools(): any[]
```

---

#### `getToolCount()`

Get count of registered tools.

**Signature:**

```typescript
getToolCount(): number
```


---

## Complete Examples

### Example 1: Basic Tool Registry

### 🟢 Verified End-to-End Example

```javascript
const { createToolRegistry } = require('@infinitibit_gmbh/graphbit');

// Create registry
const registry = createToolRegistry();

// Register multiple tools
registry.register(
  'getCurrentTime',
  'Returns the current time',
  {},
  () => new Date().toISOString()
);

registry.register(
  'calculateAge',
  'Calculates age from birth year',
  { birthYear: 'number' },
  (args) => new Date().getFullYear() - args.birthYear
);

registry.register(
  'generateId',
  'Generates a random ID',
  { prefix: 'string' },
  (args) => `${args.prefix}-${Math.random().toString(36).substr(2, 9)}`
);

// Execute tools
async function runTools() {
  const time = await registry.execute('getCurrentTime', {});
  console.log('Current time:', time.result);
  
  const age = await registry.execute('calculateAge', { birthYear: 1990 });
  console.log('Age:', age.result);
  
  const id = await registry.execute('generateId', { prefix: 'user' });
  console.log('ID:', id.result);
}

runTools();
```

---

### Example 2: Tool Registry with Validation

```javascript
const { createToolRegistry } = require('@infinitibit_gmbh/graphbit');

function createValidatedRegistry() {
  const registry = createToolRegistry();
  
  // Tool with parameter validation
  registry.register(
    'divide',
    'Divides two numbers safely',
    { numerator: 'number', denominator: 'number' },
    (args) => {
      // IMPORTANT: Return error strings instead of throwing
      // Throwing causes fatal NAPI errors
      if (args.denominator === 0) {
        return 'Error: Cannot divide by zero';
      }
      
      if (typeof args.numerator !== 'number' || typeof args.denominator !== 'number') {
        return 'Error: Both arguments must be numbers';
      }
      
      return args.numerator / args.denominator;
    }
  );
  
  return registry;
}

async function useSafeDivide() {
  const registry = createValidatedRegistry();
  
  // Valid operation
  const result1 = await registry.execute('divide', { numerator: 10, denominator: 2 });
  console.log('10 / 2 =', result1.result); // 5
  
  // Division by zero (handled gracefully)
  const result2 = await registry.execute('divide', { numerator: 10, denominator: 0 });
  console.log('10 / 0 =', result2.result); // "Error: Cannot divide by zero"
}

useSafeDivide();
```

---

### Example 3: Utility Tools Collection

```javascript
const { createToolRegistry } = require('@infinitibit_gmbh/graphbit');

function createUtilityTools() {
  const registry = createToolRegistry();
  
  // String manipulation
  registry.register(
    'toUpperCase',
    'Converts text to uppercase',
    { text: 'string' },
    (args) => args.text.toUpperCase()
  );
  
  registry.register(
    'slugify',
    'Creates URL-friendly slug from text',
    { text: 'string' },
    (args) => args.text
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-|-$/g, '')
  );
  
  // Array operations
  registry.register(
    sum',
    'Sums an array of numbers',
    { numbers: 'array' },
    (args) => args.numbers.reduce((sum, n) => sum + n, 0)
  );
  
  registry.register(
    'average',
    'Calculates average of numbers',
    { numbers: 'array' },
    (args) => {
      if (args.numbers.length === 0) return 0;
      const sum = args.numbers.reduce((s, n) => s + n, 0);
      return sum / args.numbers.length;
    }
  );
  
  // Object operations
  registry.register(
    'parseJSON',
    'Safely parses JSON string',
    { jsonString: 'string' },
    (args) => {
      try {
        return JSON.parse(args.jsonString);
      } catch (error) {
        return `Error: Invalid JSON - ${error.message}`;
      }
    }
  );
  
  return registry;
}

async function demonstrateUtils() {
  const registry = createUtilityTools();
  
  // Test tools
  const upper = await registry.execute('toUpperCase', { text: 'hello' });
  console.log('Uppercase:', upper.result); // "HELLO"
  
  const slug = await registry.execute('slugify', { text: 'My Blog Post!' });
  console.log('Slug:', slug.result); // "my-blog-post"
  
  const total = await registry.execute('sum', { numbers: [1, 2, 3, 4, 5] });
  console.log('Sum:', total.result); // 15
  
  const avg = await registry.execute('average', { numbers: [10, 20, 30] });
  console.log('Average:', avg.result); // 20
}

demonstrateUtils();
```

---

### Example 4: Integration with Agents

```javascript
const { createToolRegistry, AgentBuilder, LlmConfig } = require('@infinitibit_gmbh/graphbit');

async function createAgentWithTools() {
  // Create tool registry
  const tools = createToolRegistry();
  
  tools.register(
    'weatherLookup',
    'Gets current weather for a city',
    { city: 'string' },
    (args) => {
      // In real implementation, call weather API
      return `The weather in ${args.city} is sunny, 22°C`;
    }
  );
  
  tools.register(
    'calculateTip',
    'Calculates tip amount',
    { billAmount: 'number', tipPercent: 'number' },
    (args) => {
      const tip = (args.billAmount * args.tipPercent) / 100;
      return tip.toFixed(2);
    }
  );
  
  // Create agent
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });
  
  const agent = await new AgentBuilder('Assistant', llmConfig)
    .systemPrompt(`You are a helpful assistant with access to tools.
      When users ask about weather, use the weatherLookup tool.
      When users ask about tips, use the calculateTip tool.`)
    .temperature(0.6)
    .build();
  
  // In a real implementation, the agent would parse tool calls
  // from its responses and execute them via the registry
  
  return { agent, tools };
}
```

---

## Best Practices

### 1. Error Handling

```javascript
// ❌ DON'T: Throw errors in tool callbacks (causes fatal NAPI error)
registry.register('bad', 'Bad tool', {}, () => {
  throw new Error('This causes a crash!');
});

// ✅ DO: Return error strings or values
registry.register('good', 'Good tool', {}, () => {
  // Handle errors gracefully
  return 'Error: Something went wrong';
});
```

### 2. Parameter Validation

```javascript
registry.register(
  'validateInput',
  'Validates input parameters',
  { value: 'number', min: 'number', max: 'number' },
  (args) => {
    if (typeof args.value !== 'number') {
      return 'Error: value must be a number';
    }
    
    if (args.value < args.min || args.value > args.max) {
      return `Error: value must be between ${args.min} and ${args.max}`;
    }
    
    return args.value;
  }
);
```

### 3. Descriptive Names

```javascript
// ❌ Vague
registry.register('process', 'Processes data', {}, ...);

// ✅ Clear and specific
registry.register('convertCelsiusToFahrenheit', 'Converts temperature from Celsius to Fahrenheit', ...);
```

### 4. Consistent Return Types

```javascript
registry.register(
  'performCalculation',
  'Performs calculation and returns result',
  { operation: 'string', a: 'number', b: 'number' },
  (args) => {
    // Always return same structure
    return {
      operation: args.operation,
      result: /* calculation */,
      timestamp: Date.now()
    };
  }
);
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Registry creation** | `ToolRegistry()` | `createToolRegistry()` |
| **Tool creation** | `@tool decorator` available | `tool()` helper function |
| **Register method** | `register_tool()` | `register()` |
| **Execute method** | `execute_tool()` | `execute()` - async |
| **Error handling** | Exceptions can be caught | MUST return error values - don't throw |

**Critical Difference:** JavaScript tool callbacks must NOT throw errors - this causes fatal NAPI errors. Always return error values or strings instead.

---

## Common Use Cases

### Use Case 1: API Integration Tools

```javascript
const registry = createToolRegistry();

registry.register(
  'fetchUserData',
  'Fetches user data from API',
  { userId: 'string' },
  async (args) => {
    try {
      const response = await fetch(`https://api.example.com/users/${args.userId}`);
      const data = await response.json();
      return data;
    } catch (error) {
      return `Error: ${error.message}`;
    }
  }
);
```

### Use Case 2: Data Processing Tools

```javascript
const registry = createToolRegistry();

registry.register(
  'filterData',
  'Filters array based on condition',
  { data: 'array', key: 'string', value: 'any' },
  (args) => {
    return args.data.filter(item => item[args.key] === args.value);
  }
);

registry.register(
  'transformData',
  'Maps data using transformer function',
  { data: 'array', field: 'string' },
  (args) => {
    return args.data.map(item => item[args.field]);
  }
);
```

---

## Troubleshooting

### Issue: Fatal NAPI Error

```javascript
// Problem: Tool throws error
registry.register('crasher', 'Crashes', {}, () => {
  throw new Error('Fatal!'); // DON'T DO THIS
});

// Solution: Return error value
registry.register('safe', 'Safe', {}, () => {
  return 'Error: Something went wrong'; // DO THIS
});
```

### Issue: Tool Not Found

```javascript
// Check if tool exists before executing
if (registry.hasTool('myTool')) {
  const result = await registry.execute('myTool', {});
} else {
  console.error('Tool not found');
}
```

### Issue: Async Tools

```javascript
// Tools can be async
registry.register(
  'asyncTool',
  'Async operation',
  {},
  async () => {
    await new Promise(r => setTimeout(r, 1000));
    return 'Done';
  }
);

// execute() already returns a Promise, so it handles both
const result = await registry.execute('asyncTool', {});
```

---

## Performance Tips

### Tip 1: Reuse Registry

```javascript
// ❌ Bad: Creating new registry repeatedly
function getTool() {
  const registry = createToolRegistry();
  registry.register(...);
  return registry;
}

// ✅ Good: Create once, reuse
const globalRegistry = createToolRegistry();
// Register tools once

function useTool() {
  return globalRegistry.execute(...);
}
```

### Tip 2: Light validation

```javascript
// Keep tool callbacks fast
registry.register('fast', 'Fast tool', {}, (args) => {
  // Minimal processing
  return args.value * 2;
});
```

---

## Related Documentation

- [Agent](./agent.md) - Use tools with agents
- [Workflow](./workflow.md) - Integrate tools in workflows
- [Executor](./executor.md) - Execute tool-enabled workflows
