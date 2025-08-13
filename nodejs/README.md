# GraphBit Node.js/TypeScript Bindings

Production-grade TypeScript/JavaScript bindings for GraphBit agentic workflow automation framework.

## Features

- **Full TypeScript Support**: Complete type definitions and IntelliSense support
- **High Performance**: Built with Rust and NAPI-RS for maximum performance
- **Production Ready**: Comprehensive error handling, logging, and monitoring
- **Async/Await Support**: Native async/await compatibility
- **Cross-Platform**: Support for Windows, macOS, and Linux

## Installation

```bash
npm install graphbit
```

## Quick Start

```typescript
import { init, version, LlmConfig, LlmClient, Workflow, Node, Executor } from 'graphbit';

// Initialize the library
await init();

console.log(`GraphBit version: ${version()}`);

// Create an LLM configuration
const llmConfig = LlmConfig.openai("your-api-key", "gpt-3.5-turbo");
const llmClient = new LlmClient(llmConfig);

// Generate text
const response = await llmClient.generate("What is artificial intelligence?");
console.log(response.content);

// Create a workflow
const workflow = new Workflow("My Workflow");

// Add nodes
const inputNode = Node.input("user_input");
const agentNode = Node.agent("analyzer", "Analyze the input: {user_input}");
const outputNode = Node.output("result");

// Add nodes to workflow
workflow.addNode(inputNode);
workflow.addNode(agentNode);
workflow.addNode(outputNode);

// Connect nodes
workflow.connect("user_input", "analyzer");
workflow.connect("analyzer", "result");

// Execute workflow
const executor = new Executor();
const result = await executor.executeWithInput(workflow, {
  user_input: "Hello, GraphBit!"
});

console.log(result.data);
```

## API Reference

### Core Functions

#### `init(logLevel?, enableTracing?, debug?): Promise<void>`
Initialize the GraphBit library. Should be called once before using other functionality.

#### `version(): string`
Get the current version of GraphBit.

#### `getSystemInfo(): Promise<SystemInfo>`
Get comprehensive system information and health status.

#### `healthCheck(): Promise<HealthCheck>`
Perform a health check of the system.

### Workflow Management

#### `class Workflow`
Represents a workflow with nodes and connections.

```typescript
const workflow = new Workflow("My Workflow");
workflow.addNode(node);
workflow.connect("source", "target");
workflow.validate();
```

#### `class Node`
Represents a node in a workflow.

```typescript
// Create different types of nodes
const inputNode = Node.input("input_name");
const agentNode = Node.agent("agent_name", "prompt template");
const transformNode = Node.transform("transform_name", "transformation logic");
const outputNode = Node.output("output_name");
```

#### `class Executor`
Executes workflows with context.

```typescript
const executor = new Executor(30000, 3); // 30s timeout, 3 retries
const result = await executor.execute(workflow, context);
```

### LLM Integration

#### `class LlmConfig`
Configuration for LLM providers.

```typescript
// OpenAI
const openaiConfig = LlmConfig.openai("api-key", "gpt-4");

// Anthropic
const anthropicConfig = LlmConfig.anthropic("api-key", "claude-3-sonnet");

// Local
const localConfig = LlmConfig.local("http://localhost:8080", "llama2");
```

#### `class LlmClient`
Client for interacting with LLM providers.

```typescript
const client = new LlmClient(config);
const response = await client.generate("Your prompt here", {
  temperature: 0.7,
  maxTokens: 1000,
  topP: 1.0
});
```

### Embeddings

#### `class EmbeddingConfig`
Configuration for embedding providers.

```typescript
const config = EmbeddingConfig.openai("api-key", "text-embedding-ada-002");
```

#### `class EmbeddingClient`
Client for generating embeddings.

```typescript
const client = new EmbeddingClient(config);
const response = await client.embed("Text to embed");
const batchResponse = await client.embedBatch(["Text 1", "Text 2"]);
```

### Document Loading

#### `class DocumentLoaderConfig`
Configuration for document loading.

```typescript
const pdfConfig = DocumentLoaderConfig.pdf(1000, 100);
const textConfig = DocumentLoaderConfig.text(500, 50);
```

#### `class DocumentLoader`
Loads and processes documents.

```typescript
const loader = new DocumentLoader(config);
const content = await loader.loadFromPath("document.pdf");
const urlContent = await loader.loadFromUrl("https://example.com/page");
```

### Text Splitting

#### Text Splitter Classes
Various strategies for splitting text into chunks.

```typescript
const config = new TextSplitterConfig(1000, 100);

// Character-based splitting
const charSplitter = new CharacterSplitter(config);
const chunks = charSplitter.split(text);

// Token-based splitting
const tokenSplitter = new TokenSplitter(config);

// Sentence-based splitting
const sentenceSplitter = new SentenceSplitter(config);

// Recursive splitting
const recursiveSplitter = new RecursiveSplitter(config);
```

## Error Handling

All async functions return Promises and can throw errors. Always use try-catch blocks:

```typescript
try {
  await init();
  const response = await llmClient.generate("Hello");
  console.log(response.content);
} catch (error) {
  console.error("Error:", error.message);
}
```

## Configuration

### Runtime Configuration

Configure the async runtime before initialization:

```typescript
await configureRuntime({
  workerThreads: 4,
  maxBlockingThreads: 512,
  threadStackSizeMb: 2
});
await init();
```

### Logging

Enable logging and tracing:

```typescript
await init("info", true, false); // log level, enable tracing, debug mode
```

## Examples

### Simple Text Generation

```typescript
import { init, LlmConfig, LlmClient } from 'graphbit';

await init();

const config = LlmConfig.openai(process.env.OPENAI_API_KEY!);
const client = new LlmClient(config);

const response = await client.generate("Explain quantum computing in simple terms");
console.log(response.content);
```

### Document Processing Pipeline

```typescript
import { 
  init, 
  DocumentLoaderConfig, 
  DocumentLoader, 
  TextSplitterConfig, 
  RecursiveSplitter,
  EmbeddingConfig,
  EmbeddingClient
} from 'graphbit';

await init();

// Load document
const loaderConfig = DocumentLoaderConfig.pdf(2000, 200);
const loader = new DocumentLoader(loaderConfig);
const document = await loader.loadFromPath("research_paper.pdf");

// Split into chunks
const splitterConfig = new TextSplitterConfig(1000, 100);
const splitter = new RecursiveSplitter(splitterConfig);
const chunks = splitter.split(document.content);

// Generate embeddings
const embeddingConfig = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!);
const embeddingClient = new EmbeddingClient(embeddingConfig);

for (const chunk of chunks) {
  const embedding = await embeddingClient.embed(chunk.content);
  console.log(`Chunk ${chunk.index}: ${embedding.embeddings[0].length} dimensions`);
}
```

### Complex Workflow

```typescript
import { 
  init, 
  Workflow, 
  Node, 
  Executor, 
  WorkflowContext 
} from 'graphbit';

await init();

const workflow = new Workflow("Content Analysis Pipeline");

// Create nodes
const input = Node.input("document");
const summarizer = Node.agent("summarizer", "Summarize this document: {document}");
const sentimentAnalyzer = Node.agent("sentiment", "Analyze sentiment of: {document}");
const combiner = Node.transform("combiner", "Combine summary and sentiment analysis");
const output = Node.output("analysis_result");

// Add nodes
[input, summarizer, sentimentAnalyzer, combiner, output].forEach(node => {
  workflow.addNode(node);
});

// Connect nodes
workflow.connect("document", "summarizer");
workflow.connect("document", "sentiment");
workflow.connect("summarizer", "combiner");
workflow.connect("sentiment", "combiner");
workflow.connect("combiner", "analysis_result");

// Execute
const executor = new Executor();
const context = new WorkflowContext();
context.set("document", "Your document text here...");

const result = await executor.execute(workflow, context);
console.log(result.data);
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please see CONTRIBUTING.md for guidelines.

## Support

For issues and questions, please use the GitHub issue tracker.
