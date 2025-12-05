# Executor

This document covers the `Executor` class for executing workflows in GraphBit JavaScript bindings.

## Overview

The `Executor` is responsible for running workflows, managing their lifecycle, and providing execution context. It handles the orchestration of nodes, manages timeouts, and provides debugging capabilities.

## Class: `Executor`

### Constructor

Create a new workflow executor.

**Signature:**

```typescript
constructor(llmConfig: LlmConfig, config?: ExecutorConfig)
```

**Parameters:**

- `llmConfig` (LlmConfig, required): LLM configuration for agent nodes
- `config` (ExecutorConfig, optional): Executor configuration object

**ExecutorConfig Interface:**

```typescript
interface ExecutorConfig {
  timeoutSeconds?: number;      // Execution timeout (default: 300)
  debug?: boolean;               // Enable debug mode (default: false)
  maxParallel?: number;          // Max parallel executions (default: CPU count)
  defaultRetryConfig?: RetryConfig;  // Default retry for all nodes
}
```

### 🟢 Verified Examples

#### Basic Executor

```javascript
const { Executor, LlmConfig } = require('graphbit');

// Create config
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

// Create basic executor
const executor = new Executor(config);
```

#### Executor with Configuration

```javascript
const executor = new Executor(config, {
  timeoutSeconds: 600,     // 10 minute timeout
  debug: true,             // Enable debug logging
  maxParallel: 4,          // Max 4 parallel nodes
  defaultRetryConfig: {    // Default retry for all nodes
    maxAttempts: 3,
    initialDelayMs: 1000,
    backoffMultiplier: 2.0,
    maxDelayMs: 10000,
    jitterFactor: 0.1,
    retryableErrors: [RetryableErrorType.NetworkError]
  }
});
```

---

### Methods

#### `execute(workflow)`

Execute a workflow and return the execution context.

**Signature:**

```typescript
async execute(workflow: Workflow): Promise<WorkflowContext>
```

**Parameters:**

- `workflow` (Workflow): The workflow to execute

**Returns:** Promise resolving to `WorkflowContext`

**Throws:**

- Execution errors
- Timeout errors
- Validation errors (if workflow is invalid)

### 🟢 Verified Example

```javascript
const { WorkflowBuilder, Executor, LlmConfig } = require('graphbit');

async function runWorkflow() {
  // Create workflow
  const workflow = new WorkflowBuilder('Data Processor').build();
  
  const node = {
    id: 'processor',
    name: 'Data Processor',
    description: 'Processes data',
    nodeType: 'Agent'
  };
  
  await workflow.addNode(node);
  await workflow.validate();

  // Execute
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });
  
  const executor = new Executor(config, {
    timeoutSeconds: 300,
    debug: true
  });
  
  try {
    const context = await executor.execute(workflow);
    
    // Check execution status
    if (await context.isCompleted()) {
      console.log('Workflow completed successfully');
      const outputs = await context.getAllOutputs();
      console.log('Outputs:', outputs);
    } else if (await context.isFailed()) {
      const error = await context.error();
      console.error('Workflow failed:', error);
    }
  } catch (error) {
    console.error('Execution error:', error);
  }
}
```

---

## Class: `WorkflowContext`

The execution context returned by `Executor.execute()`.

### Methods

#### `isCompleted()`

Check if the workflow completed successfully.

**Signature:**

```typescript
async isCompleted(): Promise<boolean>
```

**Returns:** Promise resolving to `true` if completed

### 🟢 Verified Example

```javascript
const context = await executor.execute(workflow);
const isCompleted = await context.isCompleted();

if (isCompleted) {
  console.log('Workflow completed!');
}
```

---

#### `isFailed()`

Check if the workflow execution failed.

**Signature:**

```typescript
async isFailed(): Promise<boolean>
```

**Returns:** Promise resolving to `true` if failed

### 🟢 Verified Example

```javascript
const isFailed = await context.isFailed();

if (isFailed) {
  const error = await context.error();
  console.error('Failure reason:', error);
}
```

---

#### `state()`

Get the current workflow state.

**Signature:**

```typescript
async state(): Promise<WorkflowState>
```

**Returns:** Promise resolving to workflow state enum

**WorkflowState Values:**

- `WorkflowState.Pending` - Not yet started
- `WorkflowState.Running` - Currently executing
- `WorkflowState.Completed` - Finished successfully
- `WorkflowState.Failed` - Execution failed
- `WorkflowState.Cancelled` - Was cancelled

```javascript
const { WorkflowState } = require('graphbit');

const state = await context.state();

if (state === WorkflowState.Running) {
  console.log('Workflow is still running...');
}
```

---

#### `stats()`

Get execution statistics.

**Signature:**

```typescript
async stats(): Promise<WorkflowExecutionStats | null>
```

**Returns:** Promise resolving to stats object or null

**WorkflowExecutionStats Interface:**

```typescript
interface WorkflowExecutionStats {
  totalNodes: number;
  successfulNodes: number;
  failedNodes: number;
  avgExecutionTimeMs: number;
  maxConcurrentNodes: number;
  totalExecutionTimeMs: number;
  peakMemoryUsageMb?: number;
}
```

```javascript
const stats = await context.stats();

if (stats) {
  console.log(`Total nodes: ${stats.totalNodes}`);
  console.log(`Successful: ${stats.successfulNodes}`);
  console.log(`Failed: ${stats.failedNodes}`);
  console.log(`Total time: ${stats.totalExecutionTimeMs}ms`);
}
```

---

#### `error()`

Get the error message if the workflow failed.

**Signature:**

```typescript
async error(): Promise<string | null>
```

**Returns:** Promise resolving to error message or null

```javascript
if (await context.isFailed()) {
  const errorMsg = await context.error();
  console.error('Error:', errorMsg);
}
```

---

#### `getAllOutputs()`

Get all node outputs as JSON string.

**Signature:**

```typescript
async getAllOutputs(): Promise<string>
```

**Returns:** Promise resolving to JSON string containing all outputs

```javascript
const outputs = await context.getAllOutputs();
const outputsObj = JSON.parse(outputs);

console.log('Node outputs:', outputsObj);
```

---

## Complete Example

### 🟢 Verified End-to-End Example

```javascript
const { 
  WorkflowBuilder, 
  Executor, 
  LlmConfig,
  RetryableErrorType 
} = require('graphbit');

async function completeWorkflowExecution() {
  // 1. Configure LLM
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // 2. Build workflow
  const workflow = new WorkflowBuilder('Data Analysis Pipeline')
    .description('Analyzes data and generates insights')
    .build();

  // 3. Add nodes with retry
  const analyzerNode = {
    id: 'analyzer',
    name: 'Data Analyzer',
    description: 'Analyzes incoming data',
    nodeType: 'Agent',
    retryConfig: {
      maxAttempts: 3,
      initialDelayMs: 1000,
      backoffMultiplier: 2.0,
      maxDelayMs: 10000,
      jitterFactor: 0.1,
      retryableErrors: [
        RetryableErrorType.NetworkError,
        RetryableErrorType.TimeoutError
      ]
    }
  };

  await workflow.addNode(analyzerNode);
  await workflow.validate();

  // 4. Create executor with config
  const executor = new Executor(llmConfig, {
    timeoutSeconds: 600,
    debug: true,
    maxParallel: 2
  });

  // 5. Execute and handle results
  try {
    const context = await executor.execute(workflow);

    // Check completion
    if (await context.isCompleted()) {
      console.log('✅ Workflow completed successfully');
      
      // Get statistics
      const stats = await context.stats();
      console.log(`Executed ${stats.totalNodes} nodes in ${stats.totalExecutionTimeMs}ms`);
      
      // Get outputs
      const outputs = await context.getAllOutputs();
      console.log('Results:', outputs);
      
    } else if (await context.isFailed()) {
      console.error('❌ Workflow failed');
      const error = await context.error();
      console.error('Error:', error);
    }
    
  } catch (error) {
    console.error('Execution error:', error.message);
    throw error;
  }
}

completeWorkflowExecution().catch(console.error);
```

---

## Error Handling

### Common Errors

```javascript
try {
  const context = await executor.execute(workflow);
} catch (error) {
  if (error.message.includes('timeout')) {
    console.error('Workflow timed out - increase timeoutSeconds');
  } else if (error.message.includes('validation')) {
    console.error('Workflow validation failed - check structure');
  } else if (error.message.includes('model not found')) {
    console.error('LLM model not available - check config');
  } else {
    console.error('Unexpected error:', error);
  }
}
```

---

## Best Practices

### 1. Always Set Timeouts

```javascript
// ❌ Bad: No timeout (uses default 300s)
const executor = new Executor(config);

// ✅ Good: Explicit timeout for your use case
const executor = new Executor(config, {
  timeoutSeconds: 900  // 15 minutes for long-running workflows
});
```

### 2. Enable Debug During Development

```javascript
const executor = new Executor(config, {
  debug: process.env.NODE_ENV === 'development',
  timeoutSeconds: 600
});
```

### 3. Handle Execution Context Properly

```javascript
const context = await executor.execute(workflow);

// ❌ Bad: Don't assume success
const outputs = await context.getAllOutputs();

// ✅ Good: Check status first
if (await context.isCompleted()) {
  const outputs = await context.getAllOutputs();
} else {
  const error = await context.error();
  // Handle failure
}
```

### 4. Use Appropriate Parallelism

```javascript
// For I/O-bound workflows (API calls)
const executor = new Executor(config, {
  maxParallel: 10  // Higher parallelism OK
});

// For CPU-bound workflows (local processing)
const executor = new Executor(config, {
  maxParallel: require('os').cpus().length  // Match CPU count
});
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Constructor** | `Executor(config, lightweight_mode, timeout_seconds, debug)` | `Executor(config, { timeoutSeconds, debug, ... })` |
| **Execute method** | `execute(workflow)` - sync | `execute(workflow)` - returns Promise |
| **Result type** | `WorkflowResult` | `WorkflowContext` |
| **Stats method** | `get_stats()` | `stats()` - async |
| **Configuration** | `configure(...)` method exists | All config via constructor |
| **Mode switching** | `set_lightweight_mode(bool)` | Not available (use config) |

---

## Related Documentation

- [Workflow Management](./workflow.md) - Create workflows to execute
- [LLM Configuration](./llm-config.md) - Configure LLM providers
- [Agent](./agent.md) - Build AI agents (coming soon)

---

## Troubleshooting

### "Model not found" Error

```javascript
// Problem: Ollama model not installed
const config = LlmConfig.ollama({ model: 'llama3.2' });

// Solution: Install model first or use cloud provider
// Terminal: ollama pull llama3.2

// Or use OpenAI:
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY
});
```

### Timeout Issues

```javascript
// If workflows timeout frequently:
const executor = new Executor(config, {
  timeoutSeconds: 1800,  // Increase to 30 minutes
  maxParallel: 1         // Reduce parallelism
});
```

### Memory Issues

```javascript
// For large workflows, process in batches:
const executor = new Executor(config, {
  maxParallel: 2,  // Limit concurrent nodes
  debug: false     // Disable debug to reduce memory
});
```
