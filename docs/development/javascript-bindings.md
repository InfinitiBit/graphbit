# JavaScript Bindings Architecture

This document provides comprehensive documentation for GraphBit's JavaScript bindings, built using napi-rs for seamless Rust-JavaScript interoperability.

## Overview

GraphBit's JavaScript bindings provide a production-grade, high-performance Node.js API that exposes the full power of the Rust core library. The bindings are designed with:

- **Type Safety**: Full TypeScript support with auto-generated type definitions
- **Performance**: Zero-copy operations where possible with native module compilation
- **Reliability**: Comprehensive error handling with circuit breakers
- **Async Support**: Full async/await compatibility with Promise-based API
- **Resource Management**: Proper cleanup and memory management via napi-rs

## Architecture

### Module Structure

```
javascript/src/
├── lib.rs                    # Main napi module initialization
├── llm_client.rs             # LLM client with resilience patterns
├── llm_config.rs             # LLM provider configuration
├── workflow.rs               # Workflow definition and execution
├── executor.rs               # Production-grade executor
├── workflow_result.rs         # Execution results and metadata
├── tools.rs                  # Tool registry and function calling
├── embeddings.rs             # Embedding provider bindings
├── document_loader.rs        # Multi-format document loading
├── text_splitter.rs          # Intelligent chunking strategies
├── node.rs                   # Workflow node definitions
├── utils.rs                  # Utility functions
└── error.rs                  # Error handling and conversion
```

### Key Design Principles

1. **Production-Ready**: Built for high-throughput, low-latency environments
2. **Resilient**: Circuit breakers, retries, and timeout handling
3. **Observable**: Comprehensive metrics and tracing
4. **Configurable**: Flexible configuration for different use cases
5. **JavaScript Native**: Fully integrated with Node.js event loop via tokio

## Core Components

### 1. Library Initialization

The main module initialization sets up the napi environment and registers all classes and functions.

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

// Initialize the library
init();

// With options
init({
  logLevel: 'info',
  enableTracing: true
});
```

**Features**:
- Runtime initialization
- Tracing configuration
- Debug mode support
- Health monitoring

### 2. LLM Client

Direct language model access without workflow overhead.

**File**: `llm_client.rs`

**Capabilities**:
- Single prompt completions
- Batch processing with concurrency control
- Streaming responses
- Token usage tracking
- Statistics and metrics
- Circuit breaker for resilience

**Methods** (11 total):
- `complete()` - Simple text completion
- `completeFull()` - Full response with metadata
- `completeBatch()` - Batch processing
- `completeStream()` - Streaming responses
- `getStats()` - Client statistics
- `resetStats()` - Clear statistics
- `warmup()` - Pre-establish connections
- Additional async variants

### 3. Workflow Orchestration

Build and execute complex multi-step workflows with DAG structure.

**File**: `workflow.rs`

**Capabilities**:
- Multi-node workflow composition
- Node type support (Agent, Task, Condition, Parallel, Loop)
- Variable management
- Output tracking
- Execution state monitoring

**Methods** (24 total):
- Workflow metadata access
- Node management (add, connect, validate)
- Variable operations
- Output retrieval
- State inspection

### 4. Executor

Production-grade workflow execution engine with configurable profiles.

**File**: `executor.rs`

**Capabilities**:
- Standard execution profile
- Low-latency optimization
- High-throughput optimization
- Custom configuration
- Concurrency management
- Timeout handling
- Retry policies

**Profiles**:
1. **Low-Latency**: Optimized for fast response times
2. **High-Throughput**: Optimized for batch processing
3. **Custom**: User-defined configuration

### 5. Workflow Results

Structured execution output with comprehensive metadata and access patterns.

**File**: `workflow_result.rs`

**Capabilities**:
- Success/failure status
- Node output access
- Variable retrieval
- Execution metrics
- Error information

**Methods** (13 total):
- Status checking
- Output access
- Variable management
- Metrics retrieval
- Error details

### 6. Tool Registry

Manage and execute tools for LLM function calling with monitoring.

**File**: `tools.rs`

**Capabilities**:
- Tool registration and unregistration
- Tool execution
- Execution history tracking
- Performance metrics
- Tool metadata management
- Lifecycle management

**Methods** (16 total):
- Tool management
- Execution control
- History and metrics
- Performance monitoring

### 7. Embeddings

Generate embeddings for semantic search and similarity operations.

**File**: `embeddings.rs`

**Capabilities**:
- Single text embedding
- Batch embedding
- Similarity search
- Multiple embedding models

**Methods** (3 total):
- `embed()` - Single embedding
- `embedBatch()` - Multiple embeddings
- `findSimilar()` - Similarity search

**Supported Models**:
- OpenAI (text-embedding-3-small, text-embedding-3-large)
- Anthropic embeddings
- Ollama local embeddings

### 8. Document Loader

Load and process multi-format documents.

**File**: `document_loader.rs`

**Capabilities**:
- PDF file parsing
- Text file loading
- CSV data parsing
- JSON document loading
- HTML page extraction
- Markdown parsing
- DOCX document loading

**Methods** (7 total):
- `loadPdf()`
- `loadText()`
- `loadCsv()`
- `loadJson()`
- `loadHtml()`
- `loadMarkdown()`
- `loadDocx()`

### 9. Text Splitting

Intelligent chunking strategies for document processing.

**File**: `text_splitter.rs`

**Capabilities**:
- Recursive character-based splitting
- Token-aware splitting for LLMs
- Custom delimiter splitting
- Paragraph boundary splitting
- Sentence boundary splitting

**Methods** (5+):
- `recursiveCharacterSplit()` - Smart chunking
- `tokenSplit()` - LLM-aware chunking
- `customSplit()` - Delimiter-based
- `paragraphSplit()` - Paragraph boundaries
- `sentenceSplit()` - Sentence boundaries

### 10. Node Definitions

Define workflow nodes with different execution types.

**File**: `node.rs`

**Node Types**:
- **Agent**: LLM-based execution with prompt and tools
- **Task**: Deterministic task execution
- **Condition**: Branching based on conditions
- **Parallel**: Concurrent execution of nodes
- **Loop**: Iterative execution

**Static Methods**:
- `Node.agent()` - Create agent node
- `Node.task()` - Create task node
- `Node.condition()` - Create condition node
- `Node.parallel()` - Create parallel node

### 11. Configuration Management

Configure LLM providers and executor settings.

**File**: `llm_config.rs`, `executor.rs`

**LLM Providers** (8 total):
- OpenAI
- Anthropic
- Ollama
- OpenRouter
- Azure OpenAI
- DeepSeek
- Replicate
- TogetherAI

**Configuration Options**:
- API keys
- Model selection
- Provider-specific settings
- Timeout configuration
- Retry policies

### 12. Error Handling

Comprehensive error handling with conversion from Rust to JavaScript.

**File**: `error.rs`

**Features**:
- Custom error types
- Error context preservation
- Stack trace maintenance
- JavaScript Error integration

**Error Categories**:
- Validation errors
- Runtime errors
- Timeout errors
- Provider errors
- Configuration errors

## Build System

### napi-rs Integration

The bindings use **napi-rs v2.16** for stable Node.js native module generation.

**Key Features**:
- Cross-platform compilation
- Automatic TypeScript definition generation
- N-API compatibility (Node.js 16+)
- Performance optimization via LTO

### Platform Support

**Supported Platforms**:
- Windows (x64, ARM64)
- macOS (Intel, Apple Silicon)
- Linux (x64, ARM64)
- Alpine Linux (musl)

**Binary Distribution**:
- Platform-specific `.node` files
- Automatic platform detection
- Fallback to build from source

### Compilation Process

```bash
# Development build
npm run build

# Release build (with LTO optimization)
npm run build:release

# Watch mode
npm run build:watch

# Test compilation
npm test
```

## Type System

### TypeScript Definitions

Automatic generation from Rust source via napi-rs.

**Definition Features**:
- Complete type coverage
- JSDoc comments
- Async method signatures
- Parameter descriptions
- Return type information

**Location**: `index.d.ts` (auto-generated)

### Type Safety

**Key Type Patterns**:
- All async operations return `Promise<T>`
- Error handling via Promise rejection
- Generic types for flexible APIs
- Optional parameters with defaults

## Async Model

### Event Loop Integration

The JavaScript bindings integrate seamlessly with Node.js event loop:

**Tokio Runtime**:
- Single global tokio runtime per process
- Async operations scheduled on tokio threads
- Proper promise/callback integration

**Async Patterns**:
```typescript
// All operations are async
const result = await executor.execute(workflow);
const output = await client.complete(prompt);
const tools = await registry.listTools();
```

## Performance Considerations

### Zero-Copy Operations

Where possible, napi-rs enables direct memory sharing:

- Large data transfers minimize copying
- Streaming operations for efficiency
- Buffer reuse for batch operations

### Concurrency Management

**Built-in Features**:
- Configurable concurrent operations
- Thread pool management
- Connection pooling for API calls
- Circuit breaker pattern

### Memory Management

**Optimizations**:
- Automatic garbage collection integration
- Resource cleanup on error
- Proper reference counting
- Memory leak prevention

## Testing

### Test Framework

**Framework**: Vitest with 30-second timeouts for LLM operations

**Test Files**: 27 comprehensive test suites

**Test Categories**:
- Unit tests for individual components
- Integration tests for workflows
- Type tests for TypeScript definitions

### Running Tests

```bash
# Run all tests
npm test

# Watch mode
npm run test:watch

# Specific test file
npm test llm_client.test.ts

# Coverage
npm run test:coverage
```

## Examples and Patterns

### Basic Workflow Execution

```typescript
import { init, LlmConfig, Executor, Workflow, Node } from '@infinitibit_gmbh/graphbit';

init();

const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
const executor = new Executor(config);

const workflow = new Workflow('MyWorkflow');
const node = Node.agent('Agent', 'Analyze input', 'agent_1');
await workflow.addNode(node);
await workflow.validate();

const result = await executor.execute(workflow);
if (result.isSuccess()) {
  console.log(result.allOutputs());
}
```

### Tool-Enabled Agent

```typescript
const registry = new ToolRegistry();

// Use registerAsync for async tools
registerAsync(registry, 'search', 'Search the web', {}, async (args) => {
  // Async implementation
});

const agent = Node.agent('Smart Agent', 'Find information', 'agent_1');
// Tools can be passed to agent during workflow setup
```

## Debugging

### Enable Tracing

```typescript
init({
  logLevel: 'debug',
  enableTracing: true,
  debug: true
});
```

### Inspect System Info

```typescript
import { getSystemInfo, healthCheck } from '@infinitibit_gmbh/graphbit';

const info = getSystemInfo();
const health = healthCheck();
console.log(JSON.stringify({ info, health }, null, 2));
```

## Contributing

See [Contributing Guide](../development/contributing.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- PR process

## References

- [JavaScript API Reference](./javascript-api.md)
- [Development Guide](../development/contributing.md)
- [Examples](../examples/)
