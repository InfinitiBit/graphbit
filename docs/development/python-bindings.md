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

## Tool Calling Integration

GraphBit provides comprehensive tool calling capabilities, allowing LLMs to execute Python functions dynamically. The tool calling system consists of two layers:

1. **Low-level Rust bindings**: High-performance core functionality
2. **High-level Python decorators**: Easy-to-use Python interface

### Quick Start

```python
import graphbit
from graphbit.tools.decorators import tool

# Initialize GraphBit
graphbit.init()

# Register a tool using the decorator
@tool(
    description="Get current weather for a location",
    parameters={
        "type": "object", 
        "properties": {
            "location": {"type": "string", "description": "City and state, e.g. San Francisco, CA"},
            "unit": {"type": "string", "enum": ["celsius", "fahrenheit"], "description": "Temperature unit"}
        },
        "required": ["location"]
    },
    category="weather"
)
def get_weather(location: str, unit: str = "fahrenheit") -> dict:
    """Get weather information for a location."""
    # Your implementation here
    return {
        "location": location,
        "temperature": 72,
        "condition": "sunny", 
        "unit": unit
    }

# The tool is automatically registered and available for LLM use
```

### Core Components

#### 1. Low-Level Rust Bindings

The core tool calling functionality is implemented in Rust for maximum performance:

```python
# Create a tool manager
manager = graphbit.ToolManager()

# Register a tool manually 
def my_tool(params):
    return {"result": "success", "data": params}

manager.register_tool(
    name="my_tool",
    description="A sample tool",
    parameters={"type": "object", "properties": {}},
    function=my_tool,
    category="utilities",
    version="1.0.0",
    enabled=True
)

# Execute a tool
result = manager.execute_tool("my_tool", {"input": "test"})
print(f"Success: {result.success}")
print(f"Data: {result.data}")
print(f"Execution time: {result.execution_time_ms}ms")
```

#### 2. Python Decorator Interface

The high-level Python interface provides automatic registration and schema generation:

```python
from graphbit.tools.decorators import tool, get_registry
from graphbit.tools.decorators import list_tools

@tool(
    description="Calculate the area of a rectangle",
    auto_schema=True,  # Automatically generate schema from type hints
    category="math",
    version="1.0.0"
)
def calculate_area(length: float, width: float) -> dict:
    """
    Calculate the area of a rectangle.
    
    Args:
        length: Length of the rectangle in units
        width: Width of the rectangle in units
        
    Returns:
        Dictionary containing the calculated area
    """
    area = length * width
    return {
        "length": length,
        "width": width, 
        "area": area,
        "unit": "square units"
    }

# List all registered tools
tools = list_tools()
print(f"Registered tools: {tools}")

# Get tool registry for advanced operations
registry = get_registry()
tool_info = registry.get_tool("calculate_area")
```

### Automatic Schema Generation

GraphBit can automatically generate JSON schemas from Python function signatures and type hints:

```python
from graphbit.tools.decorators import tool
from graphbit.tools.schemas import generate_schema
from typing import List, Optional, Union

@tool(
    description="Process a list of items",
    auto_schema=True  # Enable automatic schema generation
)
def process_items(
    items: List[str], 
    operation: str,
    count: int = 10,
    verbose: Optional[bool] = False
) -> dict:
    """
    Process a list of items with specified operation.
    
    Args:
        items: List of items to process
        operation: Type of operation to perform
        count: Maximum number of items to process
        verbose: Whether to include detailed output
    """
    return {
        "processed": items[:count],
        "operation": operation,
        "total_processed": min(len(items), count),
        "verbose": verbose
    }

# The decorator automatically generates this schema:
# {
#     "type": "object",
#     "properties": {
#         "items": {"type": "array", "items": {"type": "string"}},
#         "operation": {"type": "string"},
#         "count": {"type": "integer", "default": 10},
#         "verbose": {"type": "boolean", "default": false, "nullable": true}
#     },
#     "required": ["items", "operation"]
# }
```

### Advanced Features

#### Tool Categories and Management

```python
from graphbit.tools.decorators import get_registry, enable_tool, disable_tool

# Organize tools by category
@tool(description="Database query tool", category="database")
def query_db(sql: str) -> dict:
    return {"result": "query_result"}

@tool(description="File system tool", category="filesystem") 
def read_file(path: str) -> str:
    return "file_content"

# List tools by category
registry = get_registry()
db_tools = registry.list_tools(category="database")
fs_tools = registry.list_tools(category="filesystem")

# Enable/disable tools dynamically
disable_tool("query_db")  # Temporarily disable
enable_tool("query_db")   # Re-enable
```

#### Tool Validation and Error Handling

```python
from graphbit.tools.decorators import tool
from graphbit.tools.utils import ToolValidationError, validate_tool_function

# Validation happens automatically when registering tools
@tool(description="Invalid tool example")
def invalid_tool(*args, **kwargs):  # This will raise ToolValidationError
    pass

# Manual validation
def my_function(param: str) -> str:
    return param.upper()

try:
    validate_tool_function(my_function)
    print("Function is valid for tool use")
except ToolValidationError as e:
    print(f"Validation failed: {e}")
```

#### Tool Metrics and Monitoring

```python
from graphbit.tools.utils import get_tool_metrics

# Execute some tools...
# ...

# Get metrics
metrics = get_tool_metrics()
stats = metrics.get_stats()

print(f"Total tools: {stats['total_tools']}")
print(f"Total calls: {stats['total_calls']}") 
print(f"Success rate: {stats['total_successes'] / stats['total_calls'] * 100:.1f}%")

# Get stats for specific tool
tool_stats = metrics.get_stats("get_weather")
print(f"Weather tool called {tool_stats['total_calls']} times")
print(f"Average execution time: {tool_stats['avg_execution_time']:.2f}ms")
```

### Global vs Instance Management

#### Global Registration (Recommended)

```python
from graphbit.tools.decorators import tool

# Tools registered with the decorator are globally available
@tool(description="Global tool")
def global_tool(param: str) -> str:
    return f"Processed: {param}"

# Access global tools
import graphbit
definitions = graphbit.get_tool_definitions()
result = graphbit.execute_tool("global_tool", {"param": "test"})
```

#### Instance Management

```python
import graphbit

# Create isolated tool manager
manager = graphbit.ToolManager()

def instance_tool(param: str) -> str:
    return f"Instance: {param}"

manager.register_tool(
    name="instance_tool",
    description="Instance-specific tool",
    parameters={"type": "object", "properties": {"param": {"type": "string"}}},
    function=instance_tool
)

# Execute on specific instance
result = manager.execute_tool("instance_tool", {"param": "test"})
```

### Integration with LLM Workflows

```python
import graphbit
from graphbit.tools.decorators import tool

# Initialize system
graphbit.init()

# Register tools
@tool(description="Search for information")
def search_web(query: str) -> dict:
    return {"results": f"Search results for: {query}"}

@tool(description="Save data to file")
def save_data(filename: str, data: str) -> dict:
    # Implementation here
    return {"saved": True, "filename": filename}

# Create LLM client with tool access
config = graphbit.LlmConfig.openai(api_key="your-key")
client = graphbit.LlmClient(config)

# The LLM can now use registered tools automatically
response = client.complete(
    prompt="Search for 'GraphBit features' and save the results to 'features.txt'",
    max_tokens=200,
    tools=graphbit.get_tool_definitions()  # Provide available tools
)
```

### Type System Integration

GraphBit's schema generator supports Python's type system comprehensively:

```python
from typing import List, Dict, Optional, Union, Literal
from graphbit.tools.decorators import tool

@tool(description="Complex type example", auto_schema=True)
def complex_function(
    # Basic types
    name: str,
    age: int,
    height: float,
    active: bool,
    
    # Collections
    tags: List[str],
    metadata: Dict[str, str],
    
    # Optional and Union types
    nickname: Optional[str] = None,
    priority: Union[int, str] = "normal",
    
    # Literal types (enums)
    status: Literal["active", "inactive", "pending"] = "active"
) -> dict:
    """Function with comprehensive type annotations."""
    return {
        "name": name,
        "age": age,
        "status": status,
        "tags": tags
    }

# Schema is automatically generated with proper type constraints
```

### Error Handling and Debugging

```python
from graphbit.tools.decorators import tool
import graphbit

@tool(description="Tool that might fail")
def risky_tool(value: int) -> dict:
    if value < 0:
        raise ValueError("Value must be positive")
    return {"result": value * 2}

# Enable debug mode for detailed error information
graphbit.init(debug=True, log_level="debug")

try:
    result = graphbit.execute_tool("risky_tool", {"value": -1})
except Exception as e:
    print(f"Tool execution failed: {e}")

# Check tool execution results
result = graphbit.execute_tool("risky_tool", {"value": 5})
if result.success:
    print(f"Success: {result.data}")
else:
    print(f"Failed after {result.execution_time_ms}ms")
```

## Best Practices

### Tool Design

1. **Single Responsibility**: Each tool should have a clear, focused purpose
2. **Type Annotations**: Use comprehensive type hints for automatic schema generation
3. **Documentation**: Include docstrings with parameter descriptions
4. **Error Handling**: Implement proper error handling and validation
5. **JSON Serializable Returns**: Ensure return values are JSON serializable

### Registration

1. Use the `@tool` decorator for most use cases
2. Enable auto_schema for automatic schema generation
3. Organize tools with meaningful categories
4. Use semantic versioning for tool versions
5. Register tools at application startup

### Performance

1. Keep tool functions lightweight and fast
2. Use caching for expensive operations
3. Monitor tool execution metrics
4. Disable unused tools in production
5. Consider tool execution timeouts

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

### Tool Calling Migration

Tool calling is a new feature in v0.1.x. If you were previously using custom function calling approaches:

```python
# Before: Manual function management
def my_functions():
    return {"get_weather": get_weather_func}

# After: Use the @tool decorator
from graphbit.tools.decorators import tool

@tool(
    description="Get current weather",
    parameters={
        "type": "object",
        "properties": {
            "location": {"type": "string", "description": "City and state"}
        },
        "required": ["location"]
    }
)
def get_weather(location: str) -> dict:
    # Your implementation
    return {"temperature": 72, "condition": "sunny"}

# Tools are automatically registered and available to LLMs
```

This comprehensive Python binding provides a robust, production-ready interface to GraphBit's core functionality while maintaining excellent performance and reliability characteristics. 
