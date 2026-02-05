# GraphBit

JavaScript/TypeScript bindings for the GraphBit agentic workflow automation framework.

## Installation

```bash
npm install @infinitibit_gmbh/graphbit
```

## Quick Start

```javascript
import { init, LlmClient, LlmConfig } from '@infinitibit_gmbh/graphbit';

// Initialize GraphBit
init();

// Configure LLM provider
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

// Create client and generate completion
const client = new LlmClient(config);
const response = await client.complete('What is AI?', 100);
console.log(response);
```

## Features

- 🤖 **Multi-LLM Support** - OpenAI, Anthropic, Ollama
- 🔄 **Workflow Automation** - Build complex agentic workflows
- 🛠️ **Tool Integration** - Function calling and tool execution
- 📄 **Document Processing** - PDF, DOCX, text splitting
- 🎯 **Type-Safe** - Full TypeScript support
- ⚡ **High Performance** - Native Rust implementation via NAPI

## Core APIs

### LLM Client
```javascript
const client = new LlmClient(config);
await client.complete(prompt, maxTokens);
await client.completeBatch(prompts, maxTokens, temperature, concurrency);
```

### Workflows
```javascript
const workflow = new WorkflowBuilder('My Workflow')
  .description('Description')
  .build();

const executor = new Executor(llmConfig);
const result = await executor.execute(workflow);
```

### Embeddings
```javascript
const embClient = new EmbeddingClient(embConfig);
const response = await embClient.embed(['text 1', 'text 2']);
const similarity = EmbeddingClient.similarity(emb1, emb2);
```

### Document Processing
```javascript
const loader = new DocumentLoader();
const doc = await loader.loadFile('./document.pdf');

const splitter = TextSplitter.recursive(500, 50);
const chunks = await splitter.split(doc.content);
```

### Tool Calling (with Async Support)
```javascript
const { ToolRegistry, registerAsync } = require('@infinitibit_gmbh/graphbit');

const registry = new ToolRegistry();

// Register async tools with proper timing tracking
registerAsync(registry, 'fetchData', 'Fetches external data', {}, async (args) => {
  const response = await fetch(args.url);
  return await response.json();
});

const result = await registry.execute('fetchData', { url: 'https://api.example.com' });
console.log(result.executionTimeMs); // Includes full async wait time
```

## Documentation

- 📚 [Full Documentation](https://docs.graphbit.ai/javascript/)
- 🔧 [API Reference](https://docs.graphbit.ai/javascript/api-reference/javascript-api/)
- 🏠 [Homepage](https://graphbit.ai)

## Requirements

- Node.js ≥ 16.0.0
- Supported platforms: Windows, macOS, Linux (x64, ARM64)

## License

See [LICENSE.md](https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md) for details.

## Links

- [GitHub Repository](https://github.com/InfinitiBit/graphbit)
- [Issues](https://github.com/InfinitiBit/graphbit/issues)
- [Website](https://graphbit.ai)
