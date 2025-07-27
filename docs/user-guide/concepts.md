# Core Concepts

Understanding GraphBit's fundamental concepts will help you build powerful AI agent workflows. This guide covers the key components and how they work together.

## Overview

GraphBit is built around these core concepts:

1. **Library Initialization** - Setting up the GraphBit environment
2. **LLM Providers** - Configuring language model clients
3. **Workflows** - Directed graphs that define the execution flow
4. **Nodes** - Individual processing units (agents, conditions, transforms)
5. **Executors** - Engines that run workflows with different performance characteristics
6. **Embeddings** - Vector embeddings for semantic operations

## Library Initialization

Before using GraphBit, you must initialize the library:

```python
import graphbit

# Basic initialization
graphbit.init()

# With custom configuration
graphbit.init(
    log_level="info",           # trace, debug, info, warn, error
    enable_tracing=True,        # Enable detailed logging
    debug=False                 # Debug mode
)

# Check system status
print(f"GraphBit version: {graphbit.version()}")
print("System info:", graphbit.get_system_info())
print("Health check:", graphbit.health_check())
```

### Runtime Configuration

For advanced use cases, configure the async runtime:

```python
# Configure before init() if needed
graphbit.configure_runtime(
    worker_threads=8,           # Number of worker threads
    max_blocking_threads=16,    # Max blocking thread pool size
    thread_stack_size_mb=2      # Stack size per thread in MB
)

graphbit.init()
```

## LLM Providers

GraphBit supports multiple LLM providers with consistent APIs.

### Provider Configuration

```python
# OpenAI
openai_config = graphbit.LlmConfig.openai(
    api_key="your-openai-key",
    model="gpt-4o-mini"  # Optional, defaults to gpt-4o-mini
)

# Anthropic
anthropic_config = graphbit.LlmConfig.anthropic(
    api_key="your-anthropic-key", 
    model="claude-3-5-sonnet-20241022"  # Optional, defaults to claude-3-5-sonnet
)

# DeepSeek
deepseek_config = graphbit.LlmConfig.deepseek(
    api_key="your-deepseek-key",
    model="deepseek-chat"  # Optional, defaults to deepseek-chat
)

# Ollama (local models)
ollama_config = graphbit.LlmConfig.ollama(
    model="llama3.2"  # Optional, defaults to llama3.2
)
```

### LLM Client Usage

```python
# Create client
client = graphbit.LlmClient(openai_config, debug=False)

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
# Create a workflow
workflow = graphbit.Workflow("My AI Pipeline")

# Add nodes to workflow
agent_node = graphbit.Node.agent(
    name="Analyzer",
    prompt="Analyze this data: {input}",
    agent_id="analyzer_001"  # Optional
)

transform_node = graphbit.Node.transform(
    name="Formatter", 
    transformation="uppercase"
)

condition_node = graphbit.Node.condition(
    name="Quality Check",
    expression="quality_score > 0.8"
)

# Add nodes and get their IDs
agent_id = workflow.add_node(agent_node)
transform_id = workflow.add_node(transform_node)
condition_id = workflow.add_node(condition_node)

# Connect nodes
workflow.connect(agent_id, transform_id)
workflow.connect(transform_id, condition_id)

# Validate workflow structure
workflow.validate()
```

## Nodes

**Nodes** are the building blocks of workflows. Each node performs a specific function.

### Node Types

#### 1. Agent Nodes
Execute AI tasks using LLM providers:

```python
# Basic agent node
analyzer = graphbit.Node.agent(
    name="Data Analyzer",
    prompt="Analyze the following data and identify key patterns: {input}",
    agent_id="analyzer"  # Optional - auto-generated if not provided
)

# Access node properties
print(f"Node ID: {analyzer.id()}")
print(f"Node Name: {analyzer.name()}")
```

#### 2. Transform Nodes
Process and modify data:

```python
# Text transformation
formatter = graphbit.Node.transform(
    name="Text Formatter",
    transformation="uppercase"  # Available: uppercase, lowercase, etc.
)
```

#### 3. Condition Nodes
Make decisions based on data evaluation:

```python
# Conditional logic
gate = graphbit.Node.condition(
    name="Quality Gate",
    expression="score > 75 and confidence > 0.8"
)
```

## Workflow Execution

**Executors** run workflows with different performance characteristics.

### Executor Types

```python
# Standard executor
executor = graphbit.Executor(
    config=llm_config,
    lightweight_mode=False,  # For backward compatibility
    timeout_seconds=300,     # 5 minutes
    debug=False
)

# High-throughput executor (for batch processing)
high_throughput = graphbit.Executor.new_high_throughput(
    llm_config=llm_config,
    timeout_seconds=600,
    debug=False
)

# Low-latency executor (for real-time applications)
low_latency = graphbit.Executor.new_low_latency(
    llm_config=llm_config,
    timeout_seconds=30,
    debug=False
)

# Memory-optimized executor (for resource-constrained environments)
memory_optimized = graphbit.Executor.new_memory_optimized(
    llm_config=llm_config,
    timeout_seconds=300,
    debug=False
)
```

### Execution Modes

```python
# Synchronous execution
result = executor.execute(workflow)

# Check execution results
if result.is_completed():
    print("Success:", result.output())
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
# OpenAI embeddings
embedding_config = graphbit.EmbeddingConfig.openai(
    api_key="your-openai-key",
    model="text-embedding-3-small"  # Optional
)

# HuggingFace embeddings  
hf_embedding_config = graphbit.EmbeddingConfig.huggingface(
    api_key="your-hf-key",
    model="sentence-transformers/all-MiniLM-L6-v2"
)
```

### Using Embeddings

```python
# Create embedding client
embedding_client = graphbit.EmbeddingClient(embedding_config)

# Single text embedding
vector = embedding_client.embed("This is a sample text")
print(f"Embedding dimension: {len(vector)}")

# Batch text embeddings
texts = ["Text 1", "Text 2", "Text 3"]
vectors = embedding_client.embed_many(texts)
print(f"Generated {len(vectors)} embeddings")

# Calculate similarity
similarity = graphbit.EmbeddingClient.similarity(vector1, vector2)
print(f"Cosine similarity: {similarity}")
```

## Error Handling

GraphBit provides comprehensive error handling:

```python
try:
    graphbit.init()
    
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
