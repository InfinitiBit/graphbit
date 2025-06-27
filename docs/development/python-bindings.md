# Python Bindings Architecture

This document provides comprehensive documentation for GraphBit's Python bindings, built using PyO3 for seamless Rust-Python interoperability.

## Overview

GraphBit's Python bindings provide a production-grade, high-performance Python API that exposes the full power of the Rust core library. The bindings are designed with:

- **Type Safety**: Full type checking and validation
- **Performance**: Zero-copy operations where possible
- **Reliability**: Comprehensive error handling and circuit breakers
- **Async Support**: Full async/await compatibility
- **Resource Management**: Proper cleanup and memory management

## Architecture

### Module Structure

```
python/src/
├── lib.rs              # Main Python module and initialization
├── runtime.rs          # Tokio runtime management
├── errors.rs           # Error handling and conversion
├── validation.rs       # Input validation utilities
├── llm/               # LLM provider bindings
│   ├── mod.rs
│   ├── client.rs      # LLM client with resilience patterns
│   └── config.rs      # Provider configuration
├── embeddings/        # Embedding provider bindings
│   ├── mod.rs
│   ├── client.rs      # Embedding client
│   └── config.rs      # Embedding configuration
└── workflow/          # Workflow execution bindings
    ├── mod.rs
    ├── executor.rs    # Production-grade executor
    ├── workflow.rs    # Workflow definition
    ├── node.rs        # Node implementation
    └── result.rs      # Execution results
```

### Key Design Principles

1. **Production-Ready**: Built for high-throughput, low-latency environments
2. **Resilient**: Circuit breakers, retries, and timeout handling
3. **Observable**: Comprehensive metrics and tracing
4. **Configurable**: Flexible configuration for different use cases

## Core Components

### 1. Library Initialization

The main library provides global initialization and system management:

```python
import graphbit

# Initialize with default settings
graphbit.init()

# Initialize with custom configuration
graphbit.init(
    log_level="info",
    enable_tracing=True,
    debug=False
)

# System information
info = graphbit.get_system_info()
health = graphbit.health_check()
version = graphbit.version()
```

#### Key Functions

- `init()`: Global library initialization with logging/tracing setup
- `version()`: Get library version information
- `get_system_info()`: Comprehensive system and runtime information
- `health_check()`: System health validation
- `configure_runtime()`: Advanced runtime configuration
- `shutdown()`: Graceful shutdown for cleanup

### 2. Runtime Management

The runtime module provides optimized Tokio runtime management:

```rust
// Runtime configuration
pub struct RuntimeConfig {
    pub worker_threads: Option<usize>,
    pub thread_stack_size: Option<usize>,
    pub enable_blocking_pool: bool,
    pub max_blocking_threads: Option<usize>,
    pub thread_keep_alive: Option<Duration>,
    pub thread_name_prefix: String,
}
```

**Features:**
- Auto-detected optimal thread configuration
- Memory-efficient stack sizes
- Production-grade thread management
- Runtime statistics and monitoring

### 3. Error Handling

Comprehensive error handling with structured error types:

```rust
pub enum PythonBindingError {
    Core(String),
    Configuration { message: String, field: Option<String> },
    Runtime { message: String, operation: String },
    Network { message: String, retry_count: u32 },
    Authentication { message: String, provider: Option<String> },
    Validation { message: String, field: String, value: Option<String> },
    RateLimit { message: String, retry_after: Option<u64> },
    Timeout { message: String, operation: String, duration_ms: u64 },
    ResourceExhausted { message: String, resource_type: String },
}
```

**Error Mapping:**
- Network errors → `PyConnectionError`
- Authentication errors → `PyPermissionError`
- Validation errors → `PyValueError`
- Timeout errors → `PyTimeoutError`
- Resource errors → `PyMemoryError`

## LLM Integration

### Configuration

```python
# OpenAI configuration
config = graphbit.LlmConfig.openai(
    api_key="your-key",
    model="gpt-4o-mini"  # default
)

# Anthropic configuration
config = graphbit.LlmConfig.anthropic(
    api_key="your-key", 
    model="claude-3-5-sonnet-20241022"  # default
)

# Ollama configuration (local)
config = graphbit.LlmConfig.ollama(
    model="llama3.2"  # default
)
```

### Client Usage

```python
# Create client with resilience features
client = graphbit.LlmClient(config, debug=False)

# Synchronous completion
response = client.complete(
    prompt="Hello, world!",
    max_tokens=100,
    temperature=0.7
)

# Asynchronous completion
import asyncio
response = await client.complete_async(
    prompt="Hello, world!",
    max_tokens=100,
    temperature=0.7
)

# Batch processing
responses = await client.complete_batch(
    prompts=["Hello", "World"],
    max_tokens=100,
    temperature=0.7,
    max_concurrency=5
)

# Streaming responses
async for chunk in client.complete_stream(
    prompt="Tell me a story",
    max_tokens=500
):
    print(chunk, end="")
```

### Client Features

- **Circuit Breaker**: Automatic failure detection and recovery
- **Retry Logic**: Exponential backoff with configurable limits
- **Timeout Handling**: Per-request and global timeouts
- **Connection Pooling**: Efficient connection reuse
- **Metrics**: Request/response statistics and monitoring
- **Warmup**: Preload models for faster first requests

## Workflow Execution

### Executor Configuration

```python
# Basic executor
executor = graphbit.Executor(llm_config)

# High-throughput executor
executor = graphbit.Executor.new_high_throughput(
    llm_config,
    timeout_seconds=300,
    debug=False
)

# Low-latency executor  
executor = graphbit.Executor.new_low_latency(
    llm_config,
    timeout_seconds=30,
    debug=False
)

# Memory-optimized executor
executor = graphbit.Executor.new_memory_optimized(
    llm_config,
    timeout_seconds=180,
    debug=False
)
```

### Execution Modes

1. **HighThroughput**: Optimized for batch processing
   - Higher concurrency (4x CPU cores)
   - Longer timeouts
   - Resource-intensive operations

2. **LowLatency**: Optimized for real-time applications
   - Shorter timeouts (30s default)
   - Fewer retries
   - Quick response prioritization

3. **MemoryOptimized**: Resource-constrained environments
   - Lower concurrency
   - Smaller memory footprint
   - Efficient resource usage

4. **Balanced**: General-purpose configuration
   - Default settings
   - Good balance of performance and resources

### Workflow Execution

```python
# Synchronous execution
result = executor.execute(workflow)

# Asynchronous execution
result = await executor.run_async(workflow)

# Get execution statistics
stats = executor.get_stats()
print(f"Total executions: {stats['total_executions']}")
print(f"Success rate: {stats['successful_executions'] / stats['total_executions']}")
print(f"Average duration: {stats['average_duration_ms']}ms")
```

## Embedding Integration

### Configuration

```python
# OpenAI embeddings
config = graphbit.EmbeddingConfig.openai(
    api_key="your-key",
    model="text-embedding-3-small"  # default
)

# HuggingFace embeddings
config = graphbit.EmbeddingConfig.huggingface(
    api_key="your-key",
    model="sentence-transformers/all-MiniLM-L6-v2"
)
```

### Client Usage

```python
client = graphbit.EmbeddingClient(config)

# Single text embedding
embedding = await client.embed_text("Hello, world!")

# Batch text embeddings
embeddings = await client.embed_batch([
    "First text",
    "Second text",
    "Third text"
])

# Document embedding with metadata
embedding = await client.embed_document(
    content="Document content",
    metadata={"source": "file.txt", "type": "document"}
)
```

## Performance Optimizations

### Memory Management

- **Stack Size**: Optimized 1MB stack per thread
- **Allocator**: jemalloc on Linux for better memory efficiency
- **Connection Pooling**: Reuse HTTP connections
- **Zero-Copy**: Minimize data copying between Rust and Python

### Concurrency

- **Worker Threads**: Auto-detected optimal count (2x CPU cores, capped at 32)
- **Blocking Pool**: Separate thread pool for I/O operations
- **Circuit Breakers**: Prevent cascade failures
- **Rate Limiting**: Respect provider limits

### Monitoring

```python
# System information
info = graphbit.get_system_info()
print(f"Worker threads: {info['runtime_worker_threads']}")
print(f"Memory allocator: {info['memory_allocator']}")

# Health check
health = graphbit.health_check()
print(f"Overall healthy: {health['overall_healthy']}")
print(f"Available memory: {health['available_memory_mb']}MB")

# Client statistics
stats = client.get_stats()
print(f"Total requests: {stats['total_requests']}")
print(f"Average response time: {stats['average_response_time_ms']}ms")
```

## Development Guidelines

### Error Handling

Always handle errors appropriately:

```python
try:
    result = client.complete("Hello, world!")
except ConnectionError as e:
    # Network issues
    print(f"Connection failed: {e}")
except TimeoutError as e:
    # Request timeout
    print(f"Request timed out: {e}")
except ValueError as e:
    # Invalid input
    print(f"Invalid input: {e}")
```

### Resource Management

```python
# Initialize once per application
graphbit.init()

# Reuse clients
client = graphbit.LlmClient(config)

# Graceful shutdown
graphbit.shutdown()
```

### Debugging

```python
# Enable debug mode
graphbit.init(debug=True, log_level="debug")

# Create client with debug output
client = graphbit.LlmClient(config, debug=True)

# Check system health
health = graphbit.health_check()
if not health['overall_healthy']:
    print("System issues detected!")
```

## Best Practices

### Initialization

1. Call `graphbit.init()` once at application startup
2. Configure logging level appropriately for environment
3. Use debug mode only during development

### Client Management

1. Create LLM/Embedding clients once and reuse
2. Use appropriate execution modes for your use case
3. Monitor client statistics for performance insights

### Error Handling

1. Handle specific exception types appropriately
2. Implement retry logic for transient failures
3. Use circuit breaker patterns for resilience

### Performance

1. Use async methods for I/O-bound operations
2. Batch requests when possible
3. Monitor memory usage and adjust concurrency
4. Use streaming for large responses

## Migration Guide

### From v0.0.x to v0.1.x

Key changes in the Python bindings:

1. **Initialization**: Now requires explicit `graphbit.init()`
2. **Error Types**: More specific exception types
3. **Async Support**: Full async/await compatibility
4. **Configuration**: Simplified configuration objects
5. **Metrics**: Built-in statistics and monitoring

### Upgrading Code

```python
# Old way (v0.0.x)
import graphbit
client = graphbit.LlmClient("openai", api_key="key")

# New way (v0.1.x)  
import graphbit
graphbit.init()
config = graphbit.LlmConfig.openai(api_key="key")
client = graphbit.LlmClient(config)
```

This comprehensive Python binding provides a robust, production-ready interface to GraphBit's core functionality while maintaining excellent performance and reliability characteristics. 