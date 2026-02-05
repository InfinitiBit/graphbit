# JavaScript API Reference

Complete reference for GraphBit's JavaScript API. This document covers all classes, methods, and their usage based on the actual JavaScript binding implementation.

## Module: `@infinitibit_gmbh/graphbit`

### Core Functions

#### `init(options?)`
Initialize the GraphBit library with optional configuration.

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

// Basic initialization
init();

// With options
init({
  logLevel: 'info',
  enableTracing: true,
  debug: true
});
```

**Parameters**:
- `options` (object, optional): Configuration options
  - `logLevel` (string): Log level ("trace", "debug", "info", "warn", "error"). Default: "warn"
  - `enableTracing` (boolean): Enable tracing. Default: false
  - `debug` (boolean): Enable debug mode. Default: false

**Returns**: `void`  
**Throws**: `Error` if initialization fails

#### `version()`
Get the current GraphBit version.

```typescript
import { version } from '@infinitibit_gmbh/graphbit';

const v = version();
console.log(`GraphBit version: ${v}`);
```

**Returns**: `string` - Version string (e.g., "0.5.1")

#### `getSystemInfo()`
Get comprehensive system information and health status.

```typescript
import { getSystemInfo } from '@infinitibit_gmbh/graphbit';

const info = getSystemInfo();
console.log(`Node.js version: ${info.nodeVersion}`);
console.log(`CPU count: ${info.cpuCount}`);
```

**Returns**: `object` - Dictionary containing:
- `version`: GraphBit version
- `nodeVersion`: Node.js version
- `runtimeUptime`: Runtime uptime in milliseconds
- `cpuCount`: Number of CPU cores
- `buildTarget`: Build target platform
- `buildProfile`: Build profile (debug/release)

#### `healthCheck()`
Perform comprehensive health checks.

```typescript
import { healthCheck } from '@infinitibit_gmbh/graphbit';

const health = healthCheck();
if (health.overallHealthy) {
  console.log('System is healthy');
} else {
  console.log('System has issues');
}
```

**Returns**: `object` - Object containing health status information

#### `configureRuntime(config)`
Configure runtime parameters.

```typescript
import { configureRuntime } from '@infinitibit_gmbh/graphbit';

configureRuntime({
  maxThreads: 4,
  enableMonitoring: true
});
```

**Parameters**:
- `config`: Runtime configuration object

---

## LLM Configuration

### `LlmConfig`

Configuration class for Large Language Model providers.

#### Static Methods

##### `LlmConfig.openai(options)`
Create OpenAI provider configuration.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini'
});
```
**Parameters**: `OpenAiOptions` object

##### `LlmConfig.anthropic(options)`
Create Anthropic provider configuration.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || '',
  model: 'claude-3-5-sonnet-20241022'
});
```

##### `LlmConfig.ollama(options)`
Create Ollama (local) provider configuration.

```typescript
const config = LlmConfig.ollama({
  baseUrl: 'http://localhost:11434',
  model: 'llama2'
});
```

##### `LlmConfig.azureOpenai(options)`
Create Azure OpenAI provider configuration.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.azureOpenai({
  apiKey: process.env.AZURE_OPENAI_API_KEY || '',
  endpoint: process.env.AZURE_OPENAI_ENDPOINT || '',
  deploymentName: 'gpt-4'
});
```

##### `LlmConfig.deepseek(options)`
Create DeepSeek provider configuration.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

const config = LlmConfig.deepseek({
  apiKey: process.env.DEEPSEEK_API_KEY || '',
  model: 'deepseek-chat'
});
```

##### Other Providers
Supported methods: `bytedance`, `huggingface`, `perplexity`, `openrouter`, `fireworks`, `replicate`, `togetherai`, `xai`, `ai21`, `mistralai`.

---

## LLM Client

### `LlmClient`
Direct language model access without workflow overhead.

#### Constructor
```typescript
const client = new LlmClient(config);
```

#### Methods

##### `complete(prompt, maxTokens)`
Simple text completion.

```typescript
const text = await client.complete('Hello world');
```

**Returns**: `Promise<string>`

##### `completeFull(prompt, maxTokens)`
Full response with metadata.

```typescript
const response = await client.completeFull('Hello world');
console.log(response.content, response.usage);
```

**Returns**: `Promise<LlmResponse>`

---

## Workflow

### `WorkflowBuilder`
Builder for creating workflows.

```typescript
const builder = new WorkflowBuilder('My Workflow');
builder.description('Processes data');
const workflow = builder.build();
```

### `Workflow`
Workflow representation.

#### Methods

##### `addNode(node)`
Add execution node to workflow.

```typescript
await workflow.addNode({
  id: 'node1',
  nodeType: 'Agent',
  // ... configuration
});
```

##### `validate()`
Validate workflow structure.

```typescript
await workflow.validate();
```

---

## Executor

### `Executor`
Workflow execution engine.

#### Constructor
```typescript
const executor = new Executor(llmConfig, {
  timeoutSeconds: 300,
  maxParallel: 4
});
```

#### Methods

##### `execute(workflow)`
Execute a workflow.

```typescript
const context = await executor.execute(workflow);
```

**Returns**: `Promise<WorkflowContext>`

### `WorkflowContext`
Execution results and verification.

#### Methods
- `isCompleted(): Promise<boolean>`
- `isFailed(): Promise<boolean>`
- `error(): Promise<string | null>`
- `getAllOutputs(): Promise<string>`
- `stats(): Promise<WorkflowExecutionStats | null>`

---

## Agents

### `AgentBuilder`
Builder for creating agents.

```typescript
const agent = new AgentBuilder('My Agent', llmConfig)
  .description('Helper agent')
  .systemPrompt('You are helpful')
  .build();
```

---

## Document Processing

### `DocumentLoader`

#### Methods

##### `loadFile(path, type)`
Load a document from a file.

```typescript
const doc = await loader.loadFile('doc.pdf', 'pdf');
```

##### `loadText(text, source)`
Load from a string.

```typescript
const doc = await loader.loadText('content', 'source_id');
```

#### Static Methods

##### `DocumentLoader.supportedTypes()`
Get list of supported extensions.

##### `DocumentLoader.detectDocumentType(path)`
Detect type from file path.

---

## Embeddings

### `EmbeddingConfig`
Configuration for embeddings (e.g., `EmbeddingConfig.openai(...)`).

### `EmbeddingClient`

#### Methods

##### `embed(texts)`
Generate embeddings for an array of strings.

```typescript
const result = await client.embed(['text1', 'text2']);
```

##### `EmbeddingClient.similarity(embedding1, embedding2)`
Calculate cosine similarity (static method).

```typescript
const score = EmbeddingClient.similarity(emb1, emb2);
```

---

## Tool Registry

### `ToolRegistry`

#### Methods

- `register(name, description, schema, callback)`
- `execute(name, args)`
- `getToolCount()`
- `getLlmTools()`: Get tools formatted for LLM usage.

---
