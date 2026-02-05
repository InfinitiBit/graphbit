# GraphBit JavaScript API Reference

Complete API reference for the GraphBit JavaScript bindings.

## Table of Contents

- [Initialization](#initialization)
- [LLM Configuration](#llm-configuration)
- [Workflows](#workflows)
- [Agents](#agents)
- [Document Processing](#document-processing)
- [Text Splitting](#text-splitting)
- [Embeddings](#embeddings)
- [Validation](#validation)
- [Error Handling](#error-handling)
- [Types](#types)

## Initialization

### `init()`

Initialize the GraphBit library. Must be called before using any other functions.

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

init();
```

**Returns:** `void`

### `version()`

Get the version of the GraphBit bindings.

```typescript
import { version } from '@infinitibit_gmbh/graphbit';

const v = version();
console.log(v); // "0.5.5"
```

**Returns:** `string`

### `versionInfo()`

Get detailed version information.

```typescript
import { versionInfo } from '@infinitibit_gmbh/graphbit';

const info = versionInfo();
console.log(info);
// {
//   version: "0.5.5",
//   rustVersion: "1.70.0",
//   napiVersion: "2.16.0"
// }
```

**Returns:** `{ version: string; rustVersion: string; napiVersion: string }`

### `getSystemInfo()`

Get detailed system information.

```typescript
import { getSystemInfo } from '@infinitibit_gmbh/graphbit';

const info = getSystemInfo();
console.log(info);
```

**Returns:** `SystemInfo`

**SystemInfo:**

```typescript
interface SystemInfo {
  os: string;
  osVersion: string;
  arch: string;
  cpuCount: number;
  totalMemoryMb: number;
  nodeVersion: string;
  graphbitVersion: string;
}
```

### `healthCheck()`

Perform a health check on the library.

```typescript
import { healthCheck } from '@infinitibit_gmbh/graphbit';

const status = healthCheck();
if (status.healthy) {
  console.log('Healthy!');
}
```

**Returns:** `HealthStatus`

**HealthStatus:**

```typescript
interface HealthStatus {
  healthy: boolean;
  timestamp: number;
  version: string;
  uptimeSeconds: number;
}
```

### `configureRuntime(config)`

Configure runtime settings.

```typescript
import { configureRuntime } from '@infinitibit_gmbh/graphbit';

configureRuntime({
  maxThreads: 4,
  enableMonitoring: true,
  memoryLimitMb: 1024,
});
```

**Parameters:**

- `config.maxThreads` (number, optional): Maximum thread pool size
- `config.enableMonitoring` (boolean, optional): Enable performance monitoring
- `config.memoryLimitMb` (number, optional): Memory limit in MB


## LLM Configuration

### `LlmConfig`

Configuration for LLM providers.

#### `LlmConfig.openai(options)`

Create an OpenAI configuration.

```typescript
const config = LlmConfig.openai({
  apiKey: 'your-api-key',
  model: 'gpt-4o-mini', // optional, default: 'gpt-4o-mini'
  temperature: 0.7, // optional, default: 0.7
  maxTokens: 1000, // optional
  baseUrl: 'https://...', // optional
});
```

**Parameters:**

- `options.apiKey` (string, required): OpenAI API key
- `options.model` (string, optional): Model name
- `options.temperature` (number, optional): Temperature (0-2)
- `options.maxTokens` (number, optional): Maximum tokens
- `options.baseUrl` (string, optional): Custom API base URL

#### `LlmConfig.anthropic(options)`

Create an Anthropic configuration.

```typescript
const config = LlmConfig.anthropic({
  apiKey: 'your-api-key',
  model: 'claude-3-5-sonnet-20241022', // optional
  temperature: 0.7, // optional
  maxTokens: 1000, // optional
});
```

**Parameters:**

- `options.apiKey` (string, required): Anthropic API key
- `options.model` (string, optional): Model name
- `options.temperature` (number, optional): Temperature (0-1)
- `options.maxTokens` (number, optional): Maximum tokens

#### `LlmConfig.ollama(options)`

Create an Ollama configuration.

```typescript
const config = LlmConfig.ollama({
  model: 'llama2',
  baseUrl: 'http://localhost:11434', // optional
  temperature: 0.8, // optional
});
```

**Parameters:**

- `options.model` (string, required): Model name
- `options.baseUrl` (string, optional): Ollama server URL
- `options.temperature` (number, optional): Temperature

#### `LlmConfig.azureOpenai(options)`

Create an Azure OpenAI configuration.

```typescript
const config = LlmConfig.azureOpenai({
  apiKey: 'your-api-key',
  deploymentName: 'gpt-4-deployment',
  endpoint: 'https://your-resource.openai.azure.com',
  temperature: 0.7, // optional
  maxTokens: 1000, // optional
});
```

**Parameters:**

- `options.apiKey` (string, required): Azure API key
- `options.deploymentName` (string, required): Deployment name
- `options.endpoint` (string, required): Azure endpoint URL
- `options.temperature` (number, optional): Temperature
- `options.maxTokens` (number, optional): Maximum tokens

## Workflows

### `WorkflowBuilder`

Builder for creating workflows.

#### Constructor

```typescript
const builder = new WorkflowBuilder(name: string);
```

#### Methods

##### `description(description: string): WorkflowBuilder`

Set the workflow description.

```typescript
builder.description('A workflow that processes documents');
```

##### `addMetadata(key: string, value: string): WorkflowBuilder`

Add metadata to the workflow.

```typescript
builder.addMetadata('version', JSON.stringify('1.0'));
builder.addMetadata('author', JSON.stringify('Team'));
```

##### `build(): Workflow`

Build and return the workflow.

```typescript
const workflow = builder.build();
```

### `Workflow`

Represents a workflow instance.

#### Methods

##### `id(): Promise<string>`

Get the workflow ID.

```typescript
const id = await workflow.id();
```

##### `name(): Promise<string>`

Get the workflow name.

```typescript
const name = await workflow.name();
```

##### `description(): Promise<string | undefined>`

Get the workflow description.

```typescript
const desc = await workflow.description();
```

##### `state(): Promise<WorkflowState>`

Get the current workflow state.

```typescript
const state = await workflow.state();
// 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Cancelled'
```

##### `validate(): Promise<boolean>`

Validate the workflow structure.

```typescript
const isValid = await workflow.validate();
```

```

### `Executor`

Executes workflows.

#### Constructor

```typescript
const executor = new Executor(
  llmConfig: LlmConfig,
  config?: ExecutorConfig
);
```

**ExecutorConfig:**

```typescript
interface ExecutorConfig {
  timeoutSeconds?: number; // Execution timeout
  debug?: boolean; // Enable debug logging
  maxParallel?: number; // Max parallel node execution
}
```

#### Methods

##### `execute(workflow: Workflow): Promise<WorkflowContext>`

Execute a workflow.

```typescript
const result = await executor.execute(workflow);
```

### `WorkflowContext`

Represents the result of workflow execution.

#### Methods

##### `isCompleted(): Promise<boolean>`

Check if the workflow completed successfully.

```typescript
const completed = await result.isCompleted();
```

##### `isFailed(): Promise<boolean>`

Check if the workflow failed.

```typescript
const failed = await result.isFailed();
```

##### `state(): Promise<WorkflowState>`

Get the workflow state.

```typescript
const state = await result.state();
```

##### `stats(): Promise<WorkflowExecutionStats | undefined>`

Get execution statistics.

```typescript
const stats = await result.stats();
if (stats) {
  console.log(stats.totalDurationMs);
  console.log(stats.nodesExecuted);
}
```

##### `error(): Promise<string | undefined>`

Get the error message if the workflow failed.

```typescript
const error = await result.error();
```

##### `getAllOutputs(): Promise<string>`

Get all node outputs as a JSON string.

```typescript
const outputs = await result.getAllOutputs();
```

## Agents

### `AgentBuilder`

Builder for creating agents.

#### Constructor

```typescript
const builder = new AgentBuilder(name: string, llmConfig: LlmConfig);

```

#### Methods

##### `description(description: string): AgentBuilder`

Set the agent description.

```typescript
builder.description('An agent that generates code');
```

##### `systemPrompt(prompt: string): AgentBuilder`

Set the system prompt.

```typescript
builder.systemPrompt('You are a helpful coding assistant.');
```

##### `llmConfig(config: LlmConfig): AgentBuilder`

Set the LLM configuration.

```typescript
builder.llmConfig(llmConfig);
```

##### `addCapability(capability: AgentCapability): AgentBuilder`

Add a capability to the agent.

```typescript
builder.addCapability('CodeGeneration');
builder.addCapability('TextGeneration');
```

**AgentCapability:**

- `'TextGeneration'`
- `'ToolCalling'`
- `'FunctionCalling'`
- `'Vision'`
- `'CodeGeneration'`
- `'Reasoning'`

##### `temperature(temp: number): AgentBuilder`

Set the temperature.

```typescript
builder.temperature(0.7);
```

##### `maxTokens(tokens: number): AgentBuilder`

Set the maximum tokens.

```typescript
builder.maxTokens(2000);
```

##### `build(): Agent`

Build and return the agent.

```typescript
const agent = builder.build();
```

### `Agent`

Represents an agent instance.

#### Methods

##### `id(): Promise<string>`

Get the agent ID.

```typescript
const id = await agent.id();
```

### `ToolRegistry`

Registry for managing generic tools.

#### `new ToolRegistry()`

Create a new tool registry.

```typescript
const registry = new ToolRegistry();
```

#### `register(name, description, parameters, callback)`

Register a generic tool.

```typescript
registry.register(
  'calculate',
  'Perform calculation',
  { type: 'object' },
  (args) => {
    return args.x + args.y;
  }
);
```

#### `execute(name, args)`

Execute a specific tool.

```typescript
const result = await registry.execute('calculate', { x: 10, y: 20 });
console.log(result.success, result.result);
```

#### `getStats()`

Get statistics about tool usage.

```typescript
const stats = registry.getStats();
console.log(stats.totalExecutions);
```

#### `clearAll()`

Clear all tools and history.

```typescript
registry.clearAll();
```

```

## Document Processing

### `DocumentLoader`

Loads and processes documents.

#### Constructor

```typescript
const loader = new DocumentLoader(config?: DocumentLoaderConfig);
```

**DocumentLoaderConfig:**

```typescript
interface DocumentLoaderConfig {
  extractImages?: boolean;
  extractTables?: boolean;
}
```

#### Methods

##### `loadFile(path: string): Promise<DocumentContent>`

Load a document from a file.

```typescript
const doc = await loader.loadFile('./document.pdf');
console.log(doc.content);
console.log(doc.source);
```

##### `loadText(text: string, source?: string): Promise<DocumentContent>`

Load a document from text.

```typescript
const doc = await loader.loadText('Hello world', 'example.txt');
```

**DocumentContent:**

```typescript
interface DocumentContent {
  content: string;
  metadata?: string;
  source: string;
  documentType: string;
}
```

#### `DocumentLoader.supportedTypes()`

Get a list of supported document types.

```typescript
const types = DocumentLoader.supportedTypes();
// ['txt', 'pdf', 'docx', 'json', 'csv', 'xml', 'html']
```

#### `DocumentLoader.detectDocumentType(path)`

Detect document type from file path.

```typescript
const type = DocumentLoader.detectDocumentType('doc.pdf'); // 'pdf'
```


## Text Splitting

### `TextSplitter`

Splits text into chunks.

#### Factory Methods

##### `TextSplitter.character(chunkSize: number, chunkOverlap?: number): TextSplitter`

Create a character-based splitter.

```typescript
const splitter = TextSplitter.character(1000, 200);
```

##### `TextSplitter.recursive(chunkSize: number, chunkOverlap?: number): TextSplitter`

Create a recursive splitter.

```typescript
const splitter = TextSplitter.recursive(1000, 200);
```

##### `TextSplitter.sentence(maxSentences?: number): TextSplitter`

Create a sentence-based splitter.

```typescript
const splitter = TextSplitter.sentence(5);
```

##### `TextSplitter.token(chunkSize: number, chunkOverlap?: number): TextSplitter`

Create a token-based splitter.

```typescript
const splitter = TextSplitter.token(500, 50);
```

#### Methods

##### `split(text: string): Promise<TextChunk[]>`

Split text into chunks.

```typescript
const chunks = await splitter.split(text);
chunks.forEach((chunk) => {
  console.log(chunk.content);
  console.log(chunk.startIndex, chunk.endIndex);
});
```

**TextChunk:**

```typescript
interface TextChunk {
  content: string;
  startIndex: number;
  endIndex: number;
  metadata?: string;
}
```

## Embeddings

### `EmbeddingConfig`

Configuration for embedding providers.

#### `EmbeddingConfig.openai(apiKey: string, model?: string): EmbeddingConfig`

Create an OpenAI embedding configuration.

```typescript
const config = EmbeddingConfig.openai('your-api-key', 'text-embedding-3-small');
```

#### `EmbeddingConfig.huggingface(apiKey: string, model?: string): EmbeddingConfig`

Create a HuggingFace embedding configuration.

```typescript
const config = EmbeddingConfig.huggingface(
  'your-api-key',
  'sentence-transformers/all-MiniLM-L6-v2'
);
```

### `EmbeddingClient`

Generates embeddings.

#### Constructor

```typescript
const client = new EmbeddingClient(config: EmbeddingConfig);
```

#### Methods

##### `embed(texts: string[]): Promise<EmbeddingResponse>`

Generate embeddings for texts.

```typescript
const response = await client.embed(['First text', 'Second text']);

console.log(response.embeddings); // number[][]
console.log(response.model);
console.log(response.usage);
```

**EmbeddingResponse:**

```typescript
interface EmbeddingResponse {
  embeddings: number[][];
  model: string;
  usage?: {
    promptTokens: number;
    totalTokens: number;
  };
}
```

#### `EmbeddingClient.similarity(embedding1, embedding2)`

Calculate cosine similarity between two embeddings.

```typescript
const score = EmbeddingClient.similarity(emb1, emb2);
// Returns 0.0 to 1.0 (or -1.0 to 1.0)
```


## Validation

### `validateJson(data: string, schema: string): ValidationResult`

Validate JSON data against a schema.

```typescript
import { validateJson } from '@infinitibit_gmbh/graphbit';

const result = validateJson(
  JSON.stringify({ name: 'John', age: 30 }),
  JSON.stringify({
    type: 'object',
    properties: {
      name: { type: 'string' },
      age: { type: 'number' },
    },
    required: ['name'],
  })
);

if (result.valid) {
  console.log('Valid!');
} else {
  console.log('Errors:', result.errors);
}
```

**ValidationResult:**

```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
}
```

## Error Handling

All errors thrown by the bindings include:

```typescript
interface GraphBitError {
  kind: ErrorKind;
  message: string;
  details?: string;
  code?: string;
}
```

**ErrorKind:**

- `'Configuration'`
- `'Validation'`
- `'Execution'`
- `'Network'`
- `'LlmProvider'`
- `'Agent'`
- `'Workflow'`
- `'Graph'`
- `'DocumentProcessing'`
- `'Serialization'`
- `'Unknown'`

## Types

### Core Types

```typescript
enum WorkflowState {
  Pending = 'Pending',
  Running = 'Running',
  Completed = 'Completed',
  Failed = 'Failed',
  Cancelled = 'Cancelled',
}

interface WorkflowExecutionStats {
  totalDurationMs: number;
  nodesExecuted: number;
  nodesFailed: number;
  nodesSkipped: number;
}

enum AgentCapability {
  TextGeneration = 'TextGeneration',
  ToolCalling = 'ToolCalling',
  FunctionCalling = 'FunctionCalling',
  Vision = 'Vision',
  CodeGeneration = 'CodeGeneration',
  Reasoning = 'Reasoning',
}
```

### LLM Types

```typescript
interface LlmResponse {
  content: string;
  finishReason?: FinishReason;
  usage?: LlmUsage;
  toolCalls?: LlmToolCall[];
  model: string;
}

interface LlmUsage {
  promptTokens: number;
  completionTokens: number;
  totalTokens: number;
}

enum FinishReason {
  Stop = 'Stop',
  Length = 'Length',
  ToolCalls = 'ToolCalls',
  ContentFilter = 'ContentFilter',
  Error = 'Error',
}
```
