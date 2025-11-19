# GraphBit JavaScript/Node.js Bindings

High-performance JavaScript/Node.js bindings for the GraphBit agentic workflow automation framework, powered by Rust and napi-rs.

## Overview

This package provides native Node.js bindings to the GraphBit Rust core library, enabling you to build and execute AI-powered agentic workflows with near-native performance in JavaScript/TypeScript applications.

## Features

- 🚀 **High Performance**: Native Rust implementation with zero-copy data transfer
- 🔄 **Async/Await Support**: Full async support with Tokio runtime integration
- 🤖 **Multi-LLM Support**: OpenAI, Anthropic, Ollama, Azure, Mistral, and more
- 📊 **Workflow Engine**: Graph-based workflow execution with dependency management
- 📄 **Document Processing**: Load and process PDF, DOCX, TXT, JSON, CSV, XML, HTML
- ✂️ **Text Splitting**: Multiple chunking strategies for large documents
- 🔍 **Embeddings**: OpenAI and HuggingFace embedding support
- ✅ **Type Safety**: Full TypeScript type definitions
- 🧪 **Well Tested**: Comprehensive unit, integration, and benchmark tests

## Installation

```bash
npm install @graphbit/core
# or
yarn add @graphbit/core
# or
pnpm add @graphbit/core
```

## Quick Start

```typescript
import { init, Workflow, Executor, LlmConfig } from '@graphbit/core';

// Initialize GraphBit
init();

// Create a workflow
const workflow = new Workflow('My AI Workflow')
  .addNode({
    name: 'analyzer',
    type: 'agent',
    prompt: 'Analyze this text: {{input}}',
  })
  .addNode({
    name: 'summarizer',
    type: 'agent',
    prompt: 'Summarize: {{analyzer.output}}',
  })
  .addEdge('analyzer', 'summarizer');

// Configure LLM
const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini',
});

// Execute workflow
const executor = new Executor({ config: llmConfig });
const result = await executor.execute(workflow, {
  input: 'Your text here',
});

console.log(result.getOutput('summarizer'));
```

## Documentation

- [API Reference](./docs/api/README.md)
- [Examples](./docs/examples/README.md)
- [Migration Guide](./docs/MIGRATION.md)
- [Maintenance Plan](./docs/MAINTENANCE.md)

## Architecture

This package is designed as a standalone, decoupled module that can be easily migrated to a separate repository. The structure follows best practices for native Node.js addons:

```
javascript/
├── src/              # Rust source code for bindings
├── tests/            # Comprehensive test suite
│   ├── unit/         # Unit tests
│   ├── integration/  # Integration tests
│   ├── types/        # TypeScript type tests
│   ├── benchmarks/   # Performance benchmarks
│   └── fixtures/     # Test fixtures and mocks
├── docs/             # Documentation
├── config/           # Configuration files
└── build/            # Build artifacts
```

## Development

### Prerequisites

- Node.js >= 16.0.0
- Rust >= 1.70.0
- Cargo

### Building

```bash
# Install dependencies
npm install

# Build in debug mode
npm run build:debug

# Build in release mode
npm run build
```

### Testing

```bash
# Run all tests
npm test

# Run unit tests
npm run test:unit

# Run integration tests
npm run test:integration

# Run type tests
npm run test:types

# Run with coverage
npm run test:coverage

# Run benchmarks
npm run bench
```

### Code Quality

```bash
# Lint code
npm run lint

# Fix linting issues
npm run lint:fix

# Format code
npm run format

# Check formatting
npm run format:check

# Type check
npm run typecheck
```

## Platform Support

Pre-built binaries are available for:

- macOS (x64, ARM64)
- Linux (x64, ARM64, musl)
- Windows (x64, ARM64)

## Performance

GraphBit JavaScript bindings leverage native Rust performance:

- **Zero-copy data transfer** between Rust and JavaScript
- **Native async/await** with Tokio runtime
- **Optimized for throughput** with parallel execution
- **Memory efficient** with Rust's ownership model

## License

See [LICENSE.md](../LICENSE.md)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md)

## Support

- [Documentation](https://docs.graphbit.ai)
- [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)
- [Discord Community](https://discord.gg/graphbit)
