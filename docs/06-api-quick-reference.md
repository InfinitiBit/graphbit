# API Reference

This section provides detailed API documentation for all GraphBit components.

## Python Library

For the complete Python API reference, see: [Python Library Documentation](../python/README.md)

The Python Library Documentation contains:

- **Core Classes**: PyLlmConfig, PyWorkflowBuilder, PyWorkflowExecutor, PyWorkflowContext
- **Node Types**: Agent nodes, condition nodes, transform nodes, delay nodes, document loader nodes
- **Edge Types**: Data flow, control flow, conditional edges
- **Embedding Classes**: PyEmbeddingService, PyEmbeddingConfig, PyEmbeddingRequest, PyEmbeddingResponse
- **Dynamic Graph**: PyDynamicGraphManager, PyWorkflowAutoCompletion, dynamic node generation
- **Configuration**: Retry configs, circuit breaker configs, agent capabilities
- **Advanced Usage**: Examples and best practices

## Rust Core

The Rust core API documentation is available online:

- [Core Crate Documentation](https://docs.rs/graphbit-core) - Core workflow engine
- [LLM Crate Documentation](https://docs.rs/graphbit-llm) - LLM provider interfaces
- [Python Bindings](https://docs.rs/graphbit-python) - Python binding implementations

## Quick Reference

### Essential Classes

| Class | Purpose | Documentation |
|-------|---------|---------------|
| `PyLlmConfig` | LLM provider configuration | [Details](../python/README.md#pyllmconfig) |
| `PyWorkflowBuilder` | Workflow construction | [Details](../python/README.md#pyworkflowbuilder) |
| `PyWorkflowExecutor` | Workflow execution | [Details](../python/README.md#pyworkflowexecutor) |
| `PyWorkflowNode` | Node factory | [Details](../python/README.md#pyworkflownode) |
| `PyWorkflowEdge` | Edge factory | [Details](../python/README.md#pyworkflowedge) |
| `PyEmbeddingService` | Text embeddings | [Details](../python/README.md#pyembeddingservice) |
| `PyDynamicGraphManager` | Dynamic workflows | [Details](../python/README.md#pydynamicgraphmanager) |

### Node Types

| Node Type | Factory Method | Use Case |
|-----------|----------------|----------|
| Agent | `PyWorkflowNode.agent_node()` | LLM-powered processing |
| Condition | `PyWorkflowNode.condition_node()` | Conditional branching |
| Transform | `PyWorkflowNode.transform_node()` | Data transformation |
| Delay | `PyWorkflowNode.delay_node()` | Workflow timing |
| Document Loader | `PyWorkflowNode.document_loader_node()` | Load PDF, text, markdown, HTML files |

### Configuration Options

| Config Type | Class | Purpose |
|-------------|-------|---------|
| Retry | `PyRetryConfig` | Failure recovery with exponential backoff |
| Circuit Breaker | `PyCircuitBreakerConfig` | Fault tolerance and failure isolation |
| Embedding | `PyEmbeddingConfig` | Text embedding configuration |
| Dynamic Graph | `PyDynamicGraphConfig` | Dynamic workflow generation |
| Agent Capability | `PyAgentCapability` | Define agent skills and capabilities |

## Code Examples

For practical usage examples, see:

- [Getting Started Guide](01-getting-started-workflows.md) - Basic usage patterns
- [Python Library Documentation](../python/README.md) - Comprehensive examples
- [Use Case Examples](02-use-case-examples.md) - Real-world applications
- [GitHub Examples](https://github.com/InfinitiBit/graphbit/tree/main/examples) - Sample projects

## Type Definitions

### Enums and Constants

```python
# Node execution results
class ExecutionStatus:
    PENDING = "pending"
    RUNNING = "running" 
    COMPLETED = "completed"
    FAILED = "failed"

# Edge types
class EdgeType:
    DATA_FLOW = "data_flow"
    CONDITIONAL = "conditional"
    TRIGGER = "trigger"
```

### Error Types

GraphBit raises standard Python exceptions:

- `ValueError` - Invalid parameter values
- `RuntimeError` - Execution errors  
- `TimeoutError` - Timeout conditions
- `ConnectionError` - Network/provider issues

### Workflow Context

Access execution results and status:

```python
context = executor.execute(workflow)

# Check status
print(f"Completed: {context.is_completed()}")
print(f"Failed: {context.is_failed()}")
print(f"Execution time: {context.execution_time_ms()}ms")

# Access variables
output = context.get_variable("output")
all_vars = dict(context.variables())
```

## Contributing to Documentation

To improve this API reference:

1. See [Contributing Guide](CONTRIBUTING.md)
2. Submit documentation updates via GitHub
3. Add examples and clarifications
4. Report missing or unclear documentation

---

For the most up-to-date API documentation, always refer to the [Python Library Documentation](../python/README.md).

## Additional Resources

- [Getting Started Guide](01-getting-started-workflows.md) - Basic usage patterns
- [Python Library Documentation](../python/README.md) - Detailed class documentation

## Embeddings

### Basic Usage
```python
# Configure embeddings
config = graphbit.PyEmbeddingConfig.openai("api-key", "text-embedding-3-small")
# or
config = graphbit.PyEmbeddingConfig.huggingface("api-key", "sentence-transformers/all-MiniLM-L6-v2")

# Create service
service = graphbit.PyEmbeddingService(config)

# Generate embeddings
embedding = await service.embed_text("Hello world")
embeddings = await service.embed_texts(["text1", "text2", "text3"])

# Calculate similarity
similarity = graphbit.PyEmbeddingService.cosine_similarity(embedding1, embedding2)

# Get dimensions
dimensions = await service.get_dimensions()
```

### Configuration Methods
```python
config = (graphbit.PyEmbeddingConfig.openai("key", "model")
    .with_timeout(60)
    .with_max_batch_size(100)
    .with_base_url("custom-url"))
```

### Request/Response
```python
# Create requests
request = graphbit.PyEmbeddingRequest("single text")
request = graphbit.PyEmbeddingRequest.multiple(["text1", "text2"])
request = request.with_user("user-id")

# Response methods
response.embeddings()        # List[List[float]]
response.model()            # str
response.prompt_tokens()    # int
response.total_tokens()     # int
response.embedding_count()  # int
response.dimensions()       # int
```

## Dynamic Graph Management

### Basic Dynamic Node Generation
```python
# Configure dynamic behavior
dynamic_config = graphbit.PyDynamicGraphConfig() \
    .with_max_auto_nodes(10) \
    .with_confidence_threshold(0.8) \
    .with_generation_temperature(0.3) \
    .enable_validation()

# Create manager
manager = graphbit.PyDynamicGraphManager(llm_config, dynamic_config)
manager.initialize(llm_config, dynamic_config)

# Generate a node
node_response = manager.generate_node(
    workflow_id="workflow_123",
    context=context,
    objective="Create a validation step",
    allowed_node_types=["condition", "transform"]
)

print(f"Generated node with confidence: {node_response.confidence()}")
generated_node = node_response.get_node()
```

### Auto-Complete Workflows
```python
# Auto-complete workflow
auto_completion = manager.auto_complete_workflow(
    workflow=workflow,
    context=context,
    objectives=["error handling", "logging"]
)

if auto_completion.success():
    print(f"Generated {auto_completion.node_count()} new nodes")
    print(f"Completion time: {auto_completion.completion_time_ms()}ms")
```

### Analytics
```python
analytics = manager.get_analytics()
print(f"Total nodes generated: {analytics.total_nodes_generated()}")
print(f"Success rate: {analytics.success_rate():.2%}")
print(f"Average confidence: {analytics.avg_confidence():.2f}")
```

## Performance Configuration

### Executor Presets
```python
# Pre-configured executors for different use cases
high_throughput_executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)
low_latency_executor = graphbit.PyWorkflowExecutor.new_low_latency(config) 
memory_optimized_executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)
```

### Retry & Circuit Breaker
```python
# Retry configuration
retry_config = (graphbit.PyRetryConfig(max_attempts=3)
    .with_exponential_backoff(1000, 2.0, 10000)
    .with_jitter(0.1))

# Circuit breaker
circuit_config = graphbit.PyCircuitBreakerConfig(failure_threshold=5, recovery_timeout_ms=30000)

# Apply to executor
executor = (executor
    .with_retry_config(retry_config)
    .with_circuit_breaker_config(circuit_config))
```

## Async Execution

```python
# Async execution
context = await executor.execute_async(workflow)

# Concurrent execution
contexts = executor.execute_concurrent([workflow1, workflow2, workflow3])

# With asyncio
import asyncio
contexts = await asyncio.gather(*[
    executor.execute_async(workflow1),
    executor.execute_async(workflow2)
])
```

## Monitoring

```python
# Performance stats
stats = executor.get_performance_stats()
pool_stats = executor.get_pool_stats()

# Pool monitoring
monitor = graphbit.PyPerformanceMonitor()
pool_sizes = monitor.get_pool_sizes()
detailed_stats = monitor.get_detailed_pool_stats()
monitor.prefill_uuid_pool()
```

## Node Types

### Agent Node
```python
agent_node = graphbit.PyWorkflowNode.agent_node(
    name="Agent Name",
    description="What this agent does",
    agent_id="unique_agent_id",
    prompt="Agent prompt with {variables}"
)
```

### Condition Node
```python
condition_node = graphbit.PyWorkflowNode.condition_node(
    name="Condition Name",
    description="Condition description",
    expression="variable > 5 && status == 'ready'"
)
```

### Transform Node
```python
transform_node = graphbit.PyWorkflowNode.transform_node(
    name="Transform Name",
    description="Transform description",
    transformation="uppercase(input_text)"
)
```

### Document Loader Node
```python
doc_node = graphbit.PyWorkflowNode.document_loader_node(
    name="Doc Loader",
    description="Loads documents",
    document_type="pdf",  # pdf, txt, docx, json, csv, xml, html
    source_path="/path/to/document"
)
```

## Edge Types

```python
# Data flow edge
graphbit.PyWorkflowEdge.data_flow()

# Control flow edge
graphbit.PyWorkflowEdge.control_flow()

# Conditional edge
graphbit.PyWorkflowEdge.conditional("condition_expression")
```

## Context & Variables

```python
# Workflow context
context.workflow_id()      # str
context.state()           # str
context.is_completed()    # bool
context.is_failed()       # bool
context.execution_time_ms() # int

# Variables
context.get_variable("key")  # Optional[str]
context.variables()         # List[(str, str)]
```

## Error Handling

```python
try:
    context = executor.execute(workflow)
except Exception as e:
    print(f"Execution failed: {e}")

# Validation
try:
    workflow.validate()
except Exception as e:
    print(f"Validation failed: {e}")
```

## Common Patterns

### RAG Pipeline
```python
async def rag_pipeline(question, documents):
    # Setup embeddings
    embedding_service = graphbit.PyEmbeddingService(embedding_config)
    doc_embeddings = await embedding_service.embed_texts(documents)
    
    # Find relevant docs
    question_embedding = await embedding_service.embed_text(question)
    similarities = [
        graphbit.PyEmbeddingService.cosine_similarity(question_embedding, doc_emb)
        for doc_emb in doc_embeddings
    ]
    
    # Get top 3 documents
    top_docs = sorted(zip(documents, similarities), key=lambda x: x[1], reverse=True)[:3]
    context = "\n\n".join([doc for doc, _ in top_docs])
    
    # Create RAG workflow
    builder = graphbit.PyWorkflowBuilder("RAG")
    rag_node = graphbit.PyWorkflowNode.agent_node(
        "RAG Agent", "Answers using context", "rag",
        f"Context: {context}\n\nQuestion: {question}\n\nAnswer:"
    )
    builder.add_node(rag_node)
    
    # Execute
    executor = graphbit.PyWorkflowExecutor(llm_config)
    return executor.execute(builder.build())
```

### Semantic Search
```python
async def semantic_search(query, documents, top_k=5):
    service = graphbit.PyEmbeddingService(embedding_config)
    
    # Generate embeddings
    query_embedding = await service.embed_text(query)
    doc_embeddings = await service.embed_texts(documents)
    
    # Calculate similarities
    similarities = [
        (doc, graphbit.PyEmbeddingService.cosine_similarity(query_embedding, doc_emb))
        for doc, doc_emb in zip(documents, doc_embeddings)
    ]
    
    # Return top results
    return sorted(similarities, key=lambda x: x[1], reverse=True)[:top_k]
```

### Batch Processing
```python
async def process_large_dataset(texts):
    service = graphbit.PyEmbeddingService(embedding_config.with_max_batch_size(100))
    
    # Process in chunks
    chunk_size = 50
    all_embeddings = []
    
    for i in range(0, len(texts), chunk_size):
        chunk = texts[i:i + chunk_size]
        chunk_embeddings = await service.embed_texts(chunk)
        all_embeddings.extend(chunk_embeddings)
    
    return all_embeddings
```

## Links

- [Complete Documentation](README.md)
- [Getting Started](01-getting-started-workflows.md)
- [Embeddings Guide](07-embeddings-guide.md)
- [API Reference](05-complete-api-reference.md)

## LLM Providers

### Configuration
```python
# OpenAI
config = graphbit.PyLlmConfig.openai("api-key", "gpt-4")

# Anthropic
config = graphbit.PyLlmConfig.anthropic("api-key", "claude-3-sonnet")

# HuggingFace
config = graphbit.PyLlmConfig.huggingface("hf-token", "microsoft/DialoGPT-medium")

# Local Ollama
config = graphbit.PyLlmConfig.ollama("llama3.1")

# Ollama with custom URL
config = graphbit.PyLlmConfig.ollama_with_base_url("llama3.1", "http://localhost:11434")
```

### Popular HuggingFace Models
```python
# Conversational AI
config = graphbit.PyLlmConfig.huggingface(token, "microsoft/DialoGPT-medium")
config = graphbit.PyLlmConfig.huggingface(token, "facebook/blenderbot-400M-distill")

# Text Generation
config = graphbit.PyLlmConfig.huggingface(token, "gpt2")
config = graphbit.PyLlmConfig.huggingface(token, "gpt2-medium")

# Code Generation
config = graphbit.PyLlmConfig.huggingface(token, "Salesforce/codegen-350M-mono")
config = graphbit.PyLlmConfig.huggingface(token, "microsoft/codebert-base")
``` 
