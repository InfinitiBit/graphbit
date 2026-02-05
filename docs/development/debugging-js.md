# Debugging Guide - JavaScript

This guide covers debugging techniques for GraphBit JavaScript development, with focus on Node.js integration and common development issues specific to the napi-rs bindings.

## Quick Debugging Checklist

When experiencing issues, work through this checklist:

1. **Environment Setup**
   - [ ] API keys configured (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.)
   - [ ] Node.js version ≥16.0.0
   - [ ] GraphBit package installed (`npm install @infinitibit_gmbh/graphbit`)
   - [ ] Dependencies up to date (`npm update`)

2. **Basic Health Check**
   ```typescript
   import { init, healthCheck, getSystemInfo } from '@infinitibit_gmbh/graphbit';

   init();
   const health = healthCheck();
   console.log(`System healthy: ${health.overallHealthy}`);
   ```

3. **Runtime Verification**
   ```typescript
   const info = getSystemInfo();
   console.log(`Node.js version: ${info.nodeVersion}`);
   console.log(`CPU count: ${info.cpuCount}`);
   ```

## JavaScript Bindings Debugging

### 1. Import and Initialization Issues

#### Problem: Module Import Fails

```bash
# Error: Cannot find module '@infinitibit_gmbh/graphbit'
node -e "require('@infinitibit_gmbh/graphbit')"
```

**Solution:**
```bash
# Verify installation
npm list @infinitibit_gmbh/graphbit

# Reinstall if missing
npm install @infinitibit_gmbh/graphbit

# Check for native module issues
npm rebuild @infinitibit_gmbh/graphbit
```

#### Problem: Native Module Load Error

```bash
# Error: Error loading native module
# Error: cannot open shared object file
```

**Solution:**
```bash
# Ensure compatible Node.js version
node --version  # Must be >= 16.0.0

# Rebuild native module
npm rebuild @infinitibit_gmbh/graphbit

# Check available binaries
ls node_modules/@infinitibit_gmbh/graphbit/
```

#### Problem: Initialization Errors

```typescript
import { init, getSystemInfo } from '@infinitibit_gmbh/graphbit';

try {
  init();
  console.log('Initialization successful');
} catch (error) {
  console.error('Initialization failed:', error instanceof Error ? error.message : error);
  
  // Try to get system info
  try {
    const info = getSystemInfo();
    console.log('System info:', info);
  } catch (sysError) {
    console.error('Cannot get system info - core issue');
  }
}
```

**Common Causes:**
- Missing environment variables (API keys)
- Incompatible Node.js version
- Missing dependencies
- Insufficient system resources

### 2. LLM Client Issues

#### Problem: Configuration Errors

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function debugConfig() {
  init();

  try {
    // Test OpenAI configuration
    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY || 'test-key',
      model: 'gpt-4o-mini'
    });
    
    const client = new LlmClient(config);
    console.log('Client created successfully');
  } catch (error) {
    console.error('Configuration error:', error instanceof Error ? error.message : error);
  }
}

debugConfig().catch(console.error);
```

#### Problem: API Request Failures

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function debugRequests() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  try {
    console.log('Testing API request...');
    const result = await client.complete('Hello');
    console.log('Request successful:', result);
  } catch (error) {
    if (error instanceof Error) {
      console.error('Error type:', error.constructor.name);
      console.error('Message:', error.message);
      console.error('Stack:', error.stack);
    } else {
      console.error('Unknown error:', error);
    }
  }
}

debugRequests().catch(console.error);
```

**Common Issues:**
- Invalid API key: Check `process.env.OPENAI_API_KEY`
- Network timeout: Check internet connection
- Rate limiting: Implement backoff strategy
- Model not found: Verify model name

#### Problem: Batch Processing Failures

```typescript
async function debugBatch() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  const prompts = ['Prompt 1', 'Prompt 2', 'Prompt 3'];

  try {
    console.log(`Processing ${prompts.length} prompts...`);
    
    const results = await client.completeBatch(
      prompts,
      100,    // maxTokens
      0.7,    // temperature
      1       // concurrency - use 1 for debugging
    );
    
    console.log('Batch results:', results);
  } catch (error) {
    console.error('Batch processing failed:', error);
  }
}
```

### 3. Workflow Execution Issues

#### Problem: Workflow Validation Fails

```typescript
import { init, Workflow, Node } from '@infinitibit_gmbh/graphbit';

async function debugWorkflow() {
  init();

  const workflow = new Workflow('DebugWorkflow');

  try {
    const node1 = Node.agent('Agent 1', 'Task 1', 'agent_1');
    const node2 = Node.agent('Agent 2', 'Task 2', 'agent_2');

    await workflow.addNode(node1);
    await workflow.addNode(node2);

    // This connection will fail if nodes aren't added
    await workflow.connect('agent_1', 'agent_2');

    console.log('About to validate workflow...');
    await workflow.validate();
    console.log('Workflow validation successful');
  } catch (error) {
    console.error('Workflow validation failed:', error instanceof Error ? error.message : error);
    console.log('Check node IDs and connections');
  }
}

debugWorkflow().catch(console.error);
```

#### Problem: Execution Timeout

```typescript
import { init, LlmConfig, Executor, Workflow, Node } from '@infinitibit_gmbh/graphbit';

async function debugTimeout() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const workflow = new Workflow('TimeoutTest');
  const agent = Node.agent('Agent', 'Long task', 'agent_1');

  await workflow.addNode(agent);
  await workflow.validate();

  const executor = new Executor(config);

  try {
    console.log('Starting execution with 30 second timeout...');
    
    const result = await executor.execute(workflow, {
      timeout: 30000  // 30 seconds
    });

    if (result.isSuccess()) {
      console.log('Execution completed');
    } else {
      console.error('Execution failed:', result.error());
    }
  } catch (error) {
    console.error('Timeout or execution error:', error);
  }
}

debugTimeout().catch(console.error);
```

### 4. Tool Registry Issues

#### Problem: Tool Registration Fails

```typescript
import { init, ToolRegistry } from '@infinitibit_gmbh/graphbit';

async function debugTools() {
  init();

  const registry = new ToolRegistry();

  try {
    console.log('Registering tool...');
    
    registry.register('test_tool', 'A test tool', {
      input: { type: 'string' }
    }, (args) => {
      console.log('Tool handler called with:', args);
      return { result: 'success' };
    });

    console.log('Tool registered successfully');

    // List tools
    const tools = registry.getRegisteredTools();
    console.log('Available tools:', tools);
  } catch (error) {
    console.error('Tool registration failed:', error);
  }
}

debugTools().catch(console.error);
```

#### Problem: Tool Execution Fails

```typescript
async function debugToolExecution() {
  init();

  const registry = new ToolRegistry();

  registry.register('divide', 'Divide two numbers', {
    numerator: { type: 'number' },
    denominator: { type: 'number' }
  }, (args) => {
    if (args.denominator === 0) {
      throw new Error('Division by zero');
    }
    return args.numerator / args.denominator;
  });

  try {
    console.log('Executing tool with invalid params...');
    const result = await registry.execute('divide', { 
      numerator: 10, 
      denominator: 0 
    });
    console.log('Result:', result);
  } catch (error) {
    console.error('Tool execution error:', error instanceof Error ? error.message : error);
  }

  try {
    console.log('Executing tool with valid params...');
    const result = await registry.execute('divide', { 
      numerator: 10, 
      denominator: 2 
    });
    console.log('Result:', result);
  } catch (error) {
    console.error('Tool execution error:', error);
  }
}
```

## Debugging Tools and Techniques

### Enable Detailed Logging

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

// Initialize with debug mode
init();

// Add console logging to track execution
console.log('='.repeat(50));
console.log('GraphBit Debug Session Started');
console.log('='.repeat(50));
```

### Create Debug Utilities

```typescript
const debug = {
  log: (label: string, data: any) => {
    console.log(`[DEBUG] ${label}:`, JSON.stringify(data, null, 2));
  },
  error: (label: string, error: unknown) => {
    console.error(`[ERROR] ${label}:`, error instanceof Error ? error.message : error);
  },
  time: async (label: string, fn: () => Promise<any>) => {
    console.log(`[START] ${label}`);
    const start = Date.now();
    try {
      const result = await fn();
      const duration = Date.now() - start;
      console.log(`[DONE] ${label} (${duration}ms)`);
      return result;
    } catch (error) {
      const duration = Date.now() - start;
      console.error(`[FAIL] ${label} (${duration}ms)`, error);
      throw error;
    }
  }
};

// Usage
await debug.time('API Request', () => 
  client.complete('Hello')
);
```

### Step-by-Step Debugging

```typescript
async function debugStep(step: string, fn: () => Promise<any>) {
  try {
    console.log(`→ ${step}`);
    const result = await fn();
    console.log(`✓ ${step}`);
    return result;
  } catch (error) {
    console.error(`✗ ${step}:`, error instanceof Error ? error.message : error);
    throw error;
  }
}

async function fullDebug() {
  await debugStep('Initialize', () => {
    init();
    return Promise.resolve();
  });

  const config = await debugStep('Create Config', () =>
    Promise.resolve(LlmConfig.openai({ 
      apiKey: process.env.OPENAI_API_KEY 
    }))
  );

  const client = await debugStep('Create Client', () =>
    Promise.resolve(new LlmClient(config))
  );

  const result = await debugStep('Execute Request', () =>
    client.complete('Hello')
  );

  console.log('Final result:', result);
}

fullDebug().catch(console.error);
```

## Common Error Patterns

### Type Errors

```typescript
// ❌ Wrong type
const result = await client.complete(123);

// ✅ Correct type
const result = await client.complete('prompt');
```

### Missing Async/Await

```typescript
// ❌ Missing await
const result = client.complete('Hello');
console.log(result);  // Prints Promise object

// ✅ Correct
const result = await client.complete('Hello');
console.log(result);  // Prints actual string
```

### Unhandled Promise Rejection

```typescript
// ❌ Unhandled rejection
executor.execute(workflow);  // Error ignored

// ✅ Proper error handling
executor.execute(workflow).catch(error => {
  console.error('Execution failed:', error);
});
```

## Performance Debugging

### Monitor Execution Time

```typescript
async function monitorPerformance() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  const times = [];

  for (let i = 0; i < 3; i++) {
    const start = Date.now();
    await client.complete('Test');
    const duration = Date.now() - start;
    times.push(duration);
    console.log(`Request ${i + 1}: ${duration}ms`);
  }

  const avg = times.reduce((a, b) => a + b) / times.length;
  console.log(`Average: ${avg}ms`);
}
```

### Monitor Memory Usage

```typescript
async function monitorMemory() {
  const before = process.memoryUsage();
  
  // Run operations
  await operation();
  
  const after = process.memoryUsage();
  
  console.log('Memory delta:', {
    heapUsed: `${Math.round((after.heapUsed - before.heapUsed) / 1024 / 1024)}MB`,
    external: `${Math.round((after.external - before.external) / 1024 / 1024)}MB`
  });
}
```

## Troubleshooting Workflow

1. **Identify the error location** - Which function failed?
2. **Check prerequisites** - Are inputs correct?
3. **Verify configuration** - Is setup correct?
4. **Test in isolation** - Can you reproduce with minimal code?
5. **Review logs** - What does the error message say?
6. **Try alternatives** - Is there a workaround?
7. **Update and retry** - Are dependencies up to date?

## Getting Help

If you can't resolve the issue:

1. Check [JavaScript API Reference](../api-reference/javascript-api.md)
2. Review [JavaScript Bindings Architecture](./javascript-bindings.md)
3. Check GitHub issues: https://github.com/InfinitiBit/graphbit/issues
4. Review examples: https://github.com/InfinitiBit/graphbit/tree/main/examples

## See Also

- [JavaScript Bindings Architecture](./javascript-bindings.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
- [Async vs Sync Usage](../user-guide/async-vs-sync-js.md)
