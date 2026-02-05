<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="https://raw.githubusercontent.com/InfinitiBit/graphbit/refs/heads/main/assets/logo(circle).png" width="180" alt="GraphBit Logo" />
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> | 
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/FMhgB3paMD">Discord</a>
    <br /><br />
</p>

<p align="center">
    <a href="https://trendshift.io/repositories/14884" target="_blank"><img src="https://trendshift.io/api/badge/repositories/14884" alt="InfinitiBit%2Fgraphbit | Trendshift" width="250" height="55"/></a>
    <br>
    <a href="https://www.npmjs.com/package/@infinitibit_gmbh/graphbit"><img src="https://img.shields.io/npm/dm/@infinitibit_gmbh/graphbit?period=total&units=INTERNATIONAL_SYSTEM&left_color=GREY&right_color=red&left_text=Downloads" alt="NPM Downloads"/></a>
</p>

<p align="center">
    <a href="https://www.npmjs.com/package/@infinitibit_gmbh/graphbit"><img src="https://img.shields.io/npm/v/@infinitibit_gmbh/graphbit?color=red&label=NPM" alt="NPM"></a>
    <!-- <a href="https://www.npmjs.com/package/@infinitibit_gmbh/graphbit"><img src="https://img.shields.io/npm/dm/@infinitibit_gmbh/graphbit?color=red&label=Downloads" alt="NPM Downloads"></a> -->
    <a href="https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml"><img src="https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main&label=Build" alt="Build Status"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
    <br>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-orange.svg?logo=rust" alt="Rust Version"></a>
    <a href="https://nodejs.org"><img src="https://img.shields.io/badge/node->=16.0.0-green.svg?logo=node.js&logoColor=white" alt="Node Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/FMhgB3paMD"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

---

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
const doc = await loader.loadFile('./document.pdf', 'pdf');

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
