# Workflow Management

This document covers workflow creation, node management, and graph operations in GraphBit JavaScript bindings.

## Overview

Workflows in GraphBit allow you to orchestrate complex multi-step processes involving AI agents, data transformations, and conditional logic. The JavaScript API uses a builder pattern combined with plain objects for flexibility.

## Core Classes

### `WorkflowBuilder`

Builder class for creating workflows with metadata.

**Constructor:**

```typescript
new WorkflowBuilder(name: string)
```

**Methods:**

- `description(desc: string): this` - Set workflow description
- `addMetadata(key: string, value: string): this` - Add metadata key-value pair
- `build(): Workflow` - Build and return the workflow instance

### 🟢 Verified Example

```javascript
const { WorkflowBuilder } = require('@infinitibit_gmbh/graphbit');

const builder = new WorkflowBuilder('Data Processing Pipeline');
const workflow = builder
  .description('Processes and analyzes data')
  .addMetadata('version', '1.0')
  .build();
```

---

## Class: `Workflow`

Represents a complete workflow graph with nodes and edges.

### Methods

#### `id()`

Get the workflow ID.

**Signature:**

```typescript
async id(): Promise<string>
```

**Returns:** Promise resolving to the workflow ID

---

#### `name()`


Get the workflow name.

**Signature:**

```typescript
async name(): Promise<string>
```

**Returns:** Promise resolving to the workflow name

### 🟢 Verified Example

```javascript
const name = await workflow.name();
console.log(`Workflow: ${name}`); // Workflow: Data Processing Pipeline
```

---

#### `description()`

Get the workflow description.

**Signature:**

```typescript
async description(): Promise<string>
```

**Returns:** Promise resolving to the workflow description

---


#### `addNode(node)`

Add a node to the workflow.

**Signature:**

```typescript
async addNode(node: WorkflowNode): Promise<string>
```

**Parameters:**

- `node` (WorkflowNode): Node definition object with:
  - `id` (string): Unique node identifier
  - `name` (string): Human-readable name
  - `description` (string): Node description
  - `nodeType` (string): Type of node (e.g., `"Agent"`, `"Transform"`)
  - `retryConfig` (RetryConfig, optional): Retry configuration

**Returns:** Promise resolving to the node ID

### 🟢 Verified Example

```javascript
const { WorkflowBuilder, RetryableErrorType } = require('@infinitibit_gmbh/graphbit');

const workflow = new WorkflowBuilder('My Workflow').build();

const agentNode = {
  id: 'agent1',
  name: 'Research Agent',
  description: 'Researches topics',
  nodeType: 'Agent',
  retryConfig: {
    maxAttempts: 3,
    initialDelayMs: 100,
    backoffMultiplier: 2.0,
    maxDelayMs: 1000,
    jitterFactor: 0.1,
    // ⚠️ MUST use RetryableErrorType enum, not strings
    retryableErrors: [RetryableErrorType.NetworkError]
  }
};

const nodeId = await workflow.addNode(agentNode);
console.log(`Node added: ${nodeId}`);
```

**Critical Notes:**

- `retryableErrors` MUST use the `RetryableErrorType` enum (numeric values)
- Passing string values like `['NetworkError']` will cause a `NumberExpected` error
- `retryConfig` is optional; omit it entirely if not needed (don't pass `null`)

---

#### `addEdge(from, to, edge)`

Connect two nodes with an edge.

**Signature:**

```typescript
async addEdge(from: string, to: string, edge: WorkflowEdge): Promise<void>
```

**Parameters:**

- `from` (string): Source node ID
- `to` (string): Target node ID
- `edge` (WorkflowEdge): Edge definition with:
  - `fromNode` (string): Must match `from` parameter
  - `toNode` (string): Must match `to` parameter
  - `condition` (string, optional): Conditional expression

**Returns:** Promise resolving to void

### 🟢 Verified Example

```javascript
// Assuming 'agent1' and 'agent2' nodes exist

const edge = {
  fromNode: 'agent1',  // ⚠️ Required even though redundant with args
  toNode: 'agent2',    // ⚠️ Required even though redundant with args
  // Omit 'condition' if not needed - DO NOT pass null
};

await workflow.addEdge('agent1', 'agent2', edge);
console.log('Edge added successfully');
```

**Critical Notes:**

- The `edge` object MUST contain `fromNode` and `toNode` fields matching the arguments
- This is a quirk of the NAPI binding structure validation
- For optional fields like `condition`, omit them entirely (use `undefined`, not `null`)

---

#### `validate()`

Validate the workflow structure.

**Signature:**

```typescript
async validate(): Promise<boolean>
```

**Returns:** Promise resolving to `true` if valid, throws an error otherwise

**Throws:** Validation error if workflow is invalid (cycles, missing nodes, etc.)

### 🟢 Verified Example

```javascript
try {
  const isValid = await workflow.validate();
  console.log('Workflow is valid:', isValid); // true
} catch (error) {
  console.error('Validation failed:', error);
}
```

---

## Complete Workflow Example

### 🟢 Verified End-to-End Example

```javascript
const { WorkflowBuilder, RetryableErrorType } = require('@infinitibit_gmbh/graphbit');

async function createWorkflow() {
  // 1. Create workflow
  const builder = new WorkflowBuilder('Analysis Pipeline');
  const workflow = builder
    .description('Analyzes and processes data')
    .build();

  // 2. Add first node
  const analyzerNode = {
    id: 'analyzer',
    name: 'Data Analyzer',
    description: 'Analyzes incoming data',
    nodeType: 'Agent',
    retryConfig: {
      maxAttempts: 3,
      initialDelayMs: 100,
      backoffMultiplier: 2.0,
      maxDelayMs: 1000,
      jitterFactor: 0.1,
      retryableErrors: [RetryableErrorType.NetworkError, RetryableErrorType.TimeoutError]
    }
  };
  await workflow.addNode(analyzerNode);

  // 3. Add second node
  const summarizerNode = {
    id: 'summarizer',
    name: 'Data Summarizer',
    description: 'Summarizes analysis results',
    nodeType: 'Agent'
    // No retryConfig - optional
  };
  await workflow.addNode(summarizerNode);

  // 4. Connect nodes
  const edge = {
    fromNode: 'analyzer',
    toNode: 'summarizer'
  };
  await workflow.addEdge('analyzer', 'summarizer', edge);

  // 5. Validate
  const isValid = await workflow.validate();
  console.log('Workflow valid:', isValid);

  return workflow;
}

createWorkflow().catch(console.error);
```

---

## Retry Configuration

### `RetryConfig` Interface

```typescript
interface RetryConfig {
  maxAttempts: number;              // 0 = no retries
  initialDelayMs: number;           // Starting delay in ms
  backoffMultiplier: number;        // Exponential backoff factor (e.g., 2.0)
  maxDelayMs: number;               // Maximum delay cap
  jitterFactor: number;             // Randomness factor (0.0-1.0)
  retryableErrors: RetryableErrorType[];  // Array of error types
}
```

### `RetryableErrorType` Enum

```javascript
const { RetryableErrorType } = require('@infinitibit_gmbh/graphbit');

// Available values:
RetryableErrorType.NetworkError          // Network connectivity issues
RetryableErrorType.TimeoutError          // Request timeouts
RetryableErrorType.RateLimitError        // Rate limiting
RetryableErrorType.TemporaryUnavailable  // Service temporarily unavailable
RetryableErrorType.InternalServerError   // 5xx errors
RetryableErrorType.AuthenticationError   // Temporary auth issues
RetryableErrorType.ResourceConflict      // Resource conflicts
RetryableErrorType.Other                 // All other errors (use cautiously)
```

### Example: Robust Retry Configuration

```javascript
const robustRetry = {
  maxAttempts: 5,
  initialDelayMs: 500,
  backoffMultiplier: 2.0,    // 500ms, 1000ms, 2000ms, 4000ms...
  maxDelayMs: 10000,         // Cap at 10 seconds
  jitterFactor: 0.2,         // ±20% randomness
  retryableErrors: [
    RetryableErrorType.NetworkError,
    RetryableErrorType.TimeoutError,
    RetryableErrorType.RateLimitError,
    RetryableErrorType.TemporaryUnavailable
  ]
};

const node = {
  id: 'robust-agent',
  name: 'Robust Agent',
  description: 'Agent with robust retry logic',
  nodeType: 'Agent',
  retryConfig: robustRetry
};
```

---

## Edge Cases & Gotchas

### ❌ Common Mistakes

```javascript
// ❌ WRONG: Using string instead of enum
retryableErrors: ['NetworkError']  // Will throw NumberExpected error

// ✅ CORRECT: Use enum
retryableErrors: [RetryableErrorType.NetworkError]

// ❌ WRONG: Passing null for optional fields
const edge = {
  fromNode: 'a',
  toNode: 'b',
  condition: null  // Will throw StringExpected error
};

// ✅ CORRECT: Omit optional fields
const edge = {
  fromNode: 'a',
  toNode: 'b'
  // condition omitted
};

// ❌ WRONG: Missing fromNode/toNode in edge
await workflow.addEdge('a', 'b', { condition: 'x > 5' });

// ✅ CORRECT: Include fromNode/toNode
await workflow.addEdge('a', 'b', { 
  fromNode: 'a', 
  toNode: 'b',
  condition: 'x > 5' 
});
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Node creation** | `Node.agent(name, prompt, ...)` factory | Plain object `{ id, name, nodeType, ... }` |
| **Builder** | No builder pattern | `WorkflowBuilder` for metadata |
| **Edge method** | `workflow.connect(from_id, to_id)` | `workflow.addEdge(from, to, edge)` with full edge object |
| **Retry errors** | Strings: `['NetworkError']` | Enum: `[RetryableErrorType.NetworkError]` |
| **Null handling** | `None` for optional fields | Omit field entirely (don't use `null`) |

---

## Best Practices

1. **Always use enums**: Import and use `RetryableErrorType` for retry configuration
2. **Validate early**: Call `validate()` before execution to catch structural errors
3. **Meaningful IDs**: Use descriptive node IDs for easier debugging
4. **Omit optionals**: For optional fields, omit them entirely rather than passing `null`
5. **Async/await**: All workflow methods are async - always use `await`

---

## Related Documentation

- [Executor](./executor.md) - Execute workflows
- [LLM Config](./llm-config.md) - Configure LLMs for agent nodes
- [Graph Operations](./graph-operations.md) - Advanced graph queries
