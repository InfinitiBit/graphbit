# Core Concepts

Understanding GraphBit's fundamental concepts will help you build powerful AI agent workflows. This guide covers the key components and how they work together.

## Overview

GraphBit is built around these core concepts:

1. **Library Initialization** - Setting up the GraphBit environment
2. **LLM Providers** - Configuring language model clients
3. **Workflows** - Directed graphs that define the execution flow
4. **Nodes** - Individual processing units 
5. **Executors** - Engines that run workflows with different performance characteristics
6. **Embeddings** - Vector embeddings for semantic operations

## Library Initialization

Before using GraphBit, you must initialize the library:

```python
from graphbit import version, get_system_info, health_check

# Check system status
print(f"GraphBit version: {version()}")
print("System info:", get_system_info())
print("Health check:", health_check())
```

### Runtime Configuration

For advanced use cases, configure the async runtime:

```python
from graphbit import configure_runtime, init

# Configure before init() if needed
configure_runtime(
    worker_threads=8,           # Number of worker threads
    max_blocking_threads=16,    # Max blocking thread pool size
    thread_stack_size_mb=2      # Stack size per thread in MB
)

init()
```

## LLM Providers

GraphBit supports multiple LLM providers with consistent APIs.

### Provider Configuration

```python
from graphbit import LlmConfig

# OpenAI
openai_config = LlmConfig.openai(
    api_key="your-openai-key",
    model="gpt-4o-mini"  # Optional, defaults to gpt-4o-mini
)

# Anthropic
anthropic_config = LlmConfig.anthropic(
    api_key="your-anthropic-key", 
    model="claude-sonnet-4-20250514"  # Optional, defaults to claude-sonnet-4-20250514
)

# DeepSeek
deepseek_config = LlmConfig.deepseek(
    api_key="your-deepseek-key",
    model="deepseek-chat"  # Optional, defaults to deepseek-chat
)

# Ollama (local models)
ollama_config = LlmConfig.ollama(
    model="llama3.2"  # Optional, defaults to llama3.2
)
```

### LLM Client Usage

```python
from graphbit import LlmClient

# Create client
client = LlmClient(openai_config, debug=False)

# Basic completion
response = client.complete(
    prompt="Explain quantum computing",
    max_tokens=500,
    temperature=0.7
)

# Async completion
import asyncio
async_response = await client.complete_async(
    prompt="Write a poem about AI",
    max_tokens=200,
    temperature=0.9
)

# Batch processing
responses = await client.complete_batch(
    prompts=["Question 1", "Question 2", "Question 3"],
    max_tokens=100,
    temperature=0.5,
    max_concurrency=3
)

# Chat-style interaction
chat_response = await client.chat_optimized(
    messages=[
        ("user", "Hello, how are you?"),
        ("assistant", "I'm doing well, thank you!"),
        ("user", "Can you help me with Python?")
    ],
    max_tokens=300
)

# Streaming responses
async for chunk in client.complete_stream(
    prompt="Tell me a story",
    max_tokens=1000
):
    print(chunk, end="", flush=True)
```

### Client Management

```python
# Get client statistics
stats = client.get_stats()
print(f"Total requests: {stats['total_requests']}")
print(f"Success rate: {stats['successful_requests'] / stats['total_requests']}")

# Warmup client (pre-initialize connections)
await client.warmup()

# Reset statistics
client.reset_stats()
```

## Workflows

A **Workflow** defines the structure and flow of your AI pipeline.

### Creating Workflows

```python
from graphbit import Workflow, Node, Executor

# Create a workflow
workflow = Workflow("My AI Pipeline")

# Add nodes to workflow
agent_node = Node.agent(
    name="Analyzer",
    prompt=f"Analyze this data: {input}",
    agent_id="analyzer_001"  # Optional
)

# Add nodes and get their IDs
agent_id = workflow.add_node(agent_node)

# Validate workflow structure
workflow.validate()
```

## Nodes

**Nodes** are the building blocks of workflows. Each node performs a specific function.

### Node Types

#### Agent Nodes
Execute AI tasks using LLM providers:

```python
from graphbit import Node

# Basic agent node
analyzer = Node.agent(
    name="Data Analyzer",
    prompt="Analyze the following data and identify key patterns: {input}",
    agent_id="analyzer"  # Optional - auto-generated if not provided
)

# Access node properties
print(f"Node ID: {analyzer.id()}")
print(f"Node Name: {analyzer.name()}")
```

---

## Workflow Execution

**Executors** run workflows with different performance characteristics.

### Executor Types

```python
from graphbit import Executor

# Standard executor
executor = Executor(
    config=llm_config,
    lightweight_mode=False,  # For backward compatibility
    timeout_seconds=300,     # 5 minutes
    debug=False
)

# Low-latency executor (for real-time applications)
executor = Executor(
    config=llm_config,
    lightweight_mode=True,  
    timeout_seconds=300,     # 5 minutes
    debug=False
)
```

### Execution Modes

```python
# Synchronous execution
result = executor.execute(workflow)

# Check execution results
if result.is_completed():
    print("Success:", result.get_all_node_outputs())
elif result.is_failed():
    print("Failed:", result.error())

# Asynchronous execution
async_result = await executor.run_async(workflow)
```

### Executor Configuration

```python
# Runtime configuration
executor.configure(
    timeout_seconds=600,
    max_retries=5,
    enable_metrics=True,
    debug=False
)

# Get execution statistics
stats = executor.get_stats()
print(f"Total executions: {stats['total_executions']}")
print(f"Success rate: {stats['successful_executions'] / stats['total_executions']}")
print(f"Average duration: {stats['average_duration_ms']}ms")

# Reset statistics
executor.reset_stats()

# Check execution mode
print(f"Current mode: {executor.get_execution_mode()}")

# Legacy mode support
executor.set_lightweight_mode(True)  # Enable lightweight mode
print(f"Lightweight mode: {executor.is_lightweight_mode()}")
```

## Embeddings

**Embeddings** provide semantic vector representations for text.

### Embedding Configuration

```python
from graphbit import EmbeddingConfig

# OpenAI embeddings
embedding_config = EmbeddingConfig.openai(
    api_key="your-openai-key",
    model="text-embedding-3-small"  # Optional
)
```

### Using Embeddings

```python
from graphbit import EmbeddingClient

# Create embedding client
embedding_client = EmbeddingClient(embedding_config)

# Single text embedding
vector = embedding_client.embed("This is a sample text")
print(f"Embedding dimension: {len(vector)}")

# Batch text embeddings
texts = ["Text 1", "Text 2", "Text 3"]
vectors = embedding_client.embed_many(texts)
print(f"Generated {len(vectors)} embeddings")

# Calculate similarity
similarity = EmbeddingClient.similarity(vector1, vector2)
print(f"Cosine similarity: {similarity}")
```

## Error Handling

GraphBit provides comprehensive error handling:

```python
from graphbit import init

try:
    init()
    
    # Your workflow code here

    result = executor.execute(workflow)
    
    if result.is_failed():
        print(f"Workflow failed: {result.error()}")
        
except Exception as e:
    print(f"GraphBit error: {e}")
```

## Best Practices

1. **Always initialize GraphBit** before using any functionality
2. **Use appropriate executor types** for your performance requirements
3. **Handle errors gracefully** and check execution results
4. **Monitor execution statistics** for performance optimization
5. **Configure timeouts** appropriate for your use case
6. **Use warmup** for production clients to reduce cold start latency

## What's Next

- Learn about [Workflow Builder](workflow-builder.md) for advanced workflow construction
- Explore [LLM Providers](llm-providers.md) for detailed provider configuration
- Check [Embeddings](embeddings.md) for semantic operations
- See [Performance](performance.md) for optimization techniques
