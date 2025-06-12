---
title: Python Library Documentation
description: Complete API reference for the GraphBit Python library
---

# GraphBit Python Library Documentation

**Version**: Latest  
**Repository**: [GraphBit](https://github.com/InfinitiBit/graphbit)

!!! info "Prerequisites"
    Before using the GraphBit Python library, make sure you have:
    
    - Python 3.8 or later
    - An API key for your chosen LLM provider
    - GraphBit installed: `pip install graphbit`

## Table of Contents

1. [Installation](#installation)
2. [Basic Concepts](#basic-concepts)
3. [Workflow](#workflow)
4. [Core Classes](#core-classes)
   - [PyLlmConfig](#pyllmconfig)
   - [PyAgentCapability](#pyagentcapability)
   - [PyWorkflowNode](#pyworkflownode)
   - [PyWorkflowEdge](#pyworkflowedge)
   - [PyWorkflow](#pyworkflow)
   - [PyWorkflowBuilder](#pyworkflowbuilder)
   - [PyWorkflowExecutor](#pyworkflowexecutor)
   - [PyWorkflowContext](#pyworkflowcontext)
   - [PyValidationResult](#pyvalidationresult)
5. [Embeddings Classes](#embeddings-classes)
   - [PyEmbeddingConfig](#pyembeddingconfig)
   - [PyEmbeddingService](#pyembeddingservice)
   - [PyEmbeddingRequest](#pyembeddingrequest)
   - [PyEmbeddingResponse](#pyembeddingresponse)
6. [Configuration Classes](#configuration-classes)
   - [PyRetryConfig](#pyretryconfig)
   - [PyCircuitBreakerConfig](#pycircuitbreakerconfig)
   - [PyPoolConfig](#pypoolconfig)
7. [Performance](#performance)
   - [PyPerformanceStats](#pyperformancestats)
   - [PyObjectPoolStats](#pyobjectpoolstats)
   - [PyPerformanceMonitor](#pyperformancemonitor)
8. [Advanced Features](#advanced-features)
   - [Dynamic Workflow Generation](#dynamic-workflow-generation)
   - [Document Loader Nodes](#document-loader-nodes)
9. [Error Handling](#error-handling)
10. [Examples](#examples)
11. [API Reference](#api-reference)

## Overview

GraphBit is a declarative agentic workflow automation framework that provides powerful Python bindings for building, executing, and managing AI-powered workflows. The library enables you to create complex multi-agent workflows with various node types, execution patterns, and optimization configurations.

### Key Features

- **Declarative Workflow Definition**: Build workflows using a builder pattern
- **Multiple Node Types**: Agent, condition, transform, and delay nodes
- **LLM Provider Support**: OpenAI, Anthropic, and custom providers
- **Embedding Support**: OpenAI and HuggingFace embeddings for semantic analysis
- **Async Execution**: Support for both synchronous and asynchronous execution
- **Performance Optimization**: Memory pools, circuit breakers, and retry mechanisms
- **Concurrent Execution**: Execute multiple workflows in parallel
- **Rich Monitoring**: Performance statistics and pool monitoring

## Installation

### From PyPI (Recommended)

```bash
pip install graphbit
```

### From Source

```bash
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/python
pip install -e .
```

### Development Installation

```bash
pip install maturin
maturin develop --release
```

## Quick Start

```python
import graphbit
import os

# Initialize the library
graphbit.init()

# Set up LLM configuration
api_key = os.getenv("OPENAI_API_KEY")
config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")

# Create a simple workflow
builder = graphbit.PyWorkflowBuilder("Hello World")
node = graphbit.PyWorkflowNode.agent_node(
    "Greeter",
    "Generates greetings",
    "greeter",
    "Say hello to {name}"
)

node_id = builder.add_node(node)
workflow = builder.build()

# Execute the workflow
executor = graphbit.PyWorkflowExecutor(config)
context = executor.execute(workflow)
print(f"Workflow completed: {context.is_completed()}")
print(f"Execution time: {context.execution_time_ms()}ms")
```

## Workflow

## Core Classes

### PyLlmConfig

Configuration class for LLM providers.

#### Static Methods

##### `openai(api_key: str, model: str) -> PyLlmConfig`
Creates an OpenAI configuration.

**Parameters:**
- `api_key` (str): OpenAI API key
- `model` (str): Model name (e.g., "gpt-4", "gpt-3.5-turbo")

**Returns:** PyLlmConfig instance

**Example:**
```python
config = graphbit.PyLlmConfig.openai("your-api-key", "gpt-4")
```

##### `anthropic(api_key: str, model: str) -> PyLlmConfig`
Creates an Anthropic configuration.

**Parameters:**
- `api_key` (str): Anthropic API key
- `model` (str): Model name (e.g., "claude-3-sonnet-20240229", "claude-3-haiku-20240307")

**Returns:** PyLlmConfig instance

**Example:**
```python
config = graphbit.PyLlmConfig.anthropic("your-api-key", "claude-3-sonnet-20240229")
```

##### `huggingface(api_key: str, model: str) -> PyLlmConfig`
Creates a HuggingFace configuration.

**Parameters:**
- `api_key` (str): HuggingFace API token
- `model` (str): Model name from HuggingFace Hub (e.g., "microsoft/DialoGPT-medium", "gpt2")

**Returns:** PyLlmConfig instance

**Example:**
```python
config = graphbit.PyLlmConfig.huggingface("your-hf-token", "microsoft/DialoGPT-medium")
```

**Popular HuggingFace Models:**
- `microsoft/DialoGPT-medium` - Conversational AI
- `gpt2` - Text generation
- `facebook/blenderbot-400M-distill` - Chatbot
- `Salesforce/codegen-350M-mono` - Code generation
- `facebook/bart-large-cnn` - Summarization

**Get your token:** [HuggingFace Settings](https://huggingface.co/settings/tokens)

#### Instance Methods

##### `provider_name() -> str`
Returns the provider name.

##### `model_name() -> str`
Returns the model name.

### PyAgentCapability

Represents different agent capabilities.

#### Static Methods

##### `text_processing() -> PyAgentCapability`
Creates a text processing capability.

##### `data_analysis() -> PyAgentCapability`
Creates a data analysis capability.

##### `tool_execution() -> PyAgentCapability`
Creates a tool execution capability.

##### `decision_making() -> PyAgentCapability`
Creates a decision making capability.

##### `custom(name: str) -> PyAgentCapability`
Creates a custom capability.

**Parameters:**
- `name` (str): Custom capability name

#### Instance Methods

##### `__str__() -> str`
Returns string representation of the capability.

### PyWorkflowNode

Factory class for creating different types of workflow nodes.

#### Static Methods

##### `agent_node(name: str, description: str, agent_id: str, prompt: str) -> PyWorkflowNode`
Creates an agent node.

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `agent_id` (str): Unique agent identifier
- `prompt` (str): Prompt template (supports variable substitution with {variable})

**Returns:** PyWorkflowNode instance

**Example:**
```python
node = graphbit.PyWorkflowNode.agent_node(
    "Content Analyzer",
    "Analyzes content quality",
    "analyzer",
    "Analyze this content: {content}"
)
```

##### `condition_node(name: str, description: str, expression: str) -> PyWorkflowNode`
Creates a condition node for branching logic.

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `expression` (str): Boolean expression to evaluate

**Example:**
```python
condition = graphbit.PyWorkflowNode.condition_node(
    "Quality Gate",
    "Checks if quality score is acceptable",
    "quality_score > 0.8"
)
```

##### `transform_node(name: str, description: str, transformation: str) -> PyWorkflowNode`
Creates a transformation node.

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `transformation` (str): Transformation type

**Example:**
```python
transform = graphbit.PyWorkflowNode.transform_node(
    "Uppercase",
    "Converts text to uppercase",
    "uppercase"
)
```

##### `delay_node(name: str, description: str, duration_seconds: int) -> PyWorkflowNode`
Creates a delay node.

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `duration_seconds` (int): Delay duration in seconds

**Example:**
```python
delay = graphbit.PyWorkflowNode.delay_node(
    "Cool Down",
    "Waits before next step",
    30
)
```

#### Instance Methods

##### `id() -> str`
Returns the node ID.

##### `name() -> str`
Returns the node name.

##### `description() -> str`
Returns the node description.

### PyWorkflowEdge

Represents connections between workflow nodes.

#### Static Methods

##### `data_flow() -> PyWorkflowEdge`
Creates a data flow edge (passes data between nodes).

##### `control_flow() -> PyWorkflowEdge`
Creates a control flow edge (execution flow without data).

##### `conditional(condition: str) -> PyWorkflowEdge`
Creates a conditional edge.

**Parameters:**
- `condition` (str): Condition expression

### PyWorkflow

Represents a complete workflow definition.

#### Instance Methods

##### `id() -> str`
Returns the workflow ID.

##### `name() -> str`
Returns the workflow name.

##### `description() -> str`
Returns the workflow description.

##### `node_count() -> int`
Returns the number of nodes in the workflow.

##### `edge_count() -> int`
Returns the number of edges in the workflow.

##### `validate() -> None`
Validates the workflow structure. Raises exception if invalid.

**Raises:** `PyValueError` if workflow is invalid

##### `to_json() -> str`
Serializes the workflow to JSON.

**Returns:** JSON string representation

##### `from_json(json_str: str) -> PyWorkflow` (static)
Deserializes a workflow from JSON.

**Parameters:**
- `json_str` (str): JSON string

**Returns:** PyWorkflow instance

### PyWorkflowBuilder

Builder pattern for constructing workflows.

#### Constructor

##### `PyWorkflowBuilder(name: str)`
Creates a new workflow builder.

**Parameters:**
- `name` (str): Workflow name

#### Instance Methods

##### `description(description: str) -> None`
Sets the workflow description.

**Parameters:**
- `description` (str): Workflow description

##### `add_node(node: PyWorkflowNode) -> str`
Adds a node to the workflow.

**Parameters:**
- `node` (PyWorkflowNode): Node to add

**Returns:** Node ID (str)

##### `connect(from_id: str, to_id: str, edge: PyWorkflowEdge) -> None`
Connects two nodes with an edge.

**Parameters:**
- `from_id` (str): Source node ID
- `to_id` (str): Target node ID
- `edge` (PyWorkflowEdge): Edge type

##### `build() -> PyWorkflow`
Builds and returns the final workflow.

**Returns:** PyWorkflow instance

**Example:**
```python
builder = graphbit.PyWorkflowBuilder("My Workflow")
builder.description("A sample workflow")

# Add nodes
node1_id = builder.add_node(agent_node)
node2_id = builder.add_node(transform_node)

# Connect nodes
builder.connect(node1_id, node2_id, graphbit.PyWorkflowEdge.data_flow())

# Build workflow
workflow = builder.build()
```

### PyWorkflowExecutor

Executes workflows with configured LLM and settings.

#### Constructor

##### `PyWorkflowExecutor(llm_config: PyLlmConfig)`
Creates a new executor with the given LLM configuration.

#### Static Methods

##### `new_high_throughput(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Creates an executor optimized for high throughput.

##### `new_low_latency(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Creates an executor optimized for low latency.

##### `new_memory_optimized(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Creates an executor optimized for memory usage.

#### Configuration Methods

##### `with_pool_config(pool_config: PyPoolConfig) -> PyWorkflowExecutor`
Sets pool configuration.

##### `enable_memory_pools() -> PyWorkflowExecutor`
Enables memory pools for performance optimization.

##### `disable_memory_pools() -> PyWorkflowExecutor`
Disables memory pools.

##### `get_pool_config() -> Optional[PyPoolConfig]`
Returns current pool configuration.

##### `with_max_node_execution_time(timeout_ms: int) -> PyWorkflowExecutor`
Sets maximum execution time per node.

##### `with_fail_fast(fail_fast: bool) -> PyWorkflowExecutor`
Enables or disables fail-fast behavior.

##### `with_retry_config(retry_config: PyRetryConfig) -> PyWorkflowExecutor`
Sets retry configuration.

##### `with_circuit_breaker_config(config: PyCircuitBreakerConfig) -> PyWorkflowExecutor`
Sets circuit breaker configuration.

##### `without_retries() -> PyWorkflowExecutor`
Disables retries.

#### Execution Methods

##### `execute(workflow: PyWorkflow) -> PyWorkflowContext`
Executes a workflow synchronously.

**Parameters:**
- `workflow` (PyWorkflow): Workflow to execute

**Returns:** PyWorkflowContext

##### `execute_async(workflow: PyWorkflow) -> Awaitable[PyWorkflowContext]`
Executes a workflow asynchronously.

**Parameters:**
- `workflow` (PyWorkflow): Workflow to execute

**Returns:** Awaitable PyWorkflowContext

##### `execute_concurrent(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]`
Executes multiple workflows concurrently.

**Parameters:**
- `workflows` (List[PyWorkflow]): List of workflows to execute

**Returns:** List of PyWorkflowContext

##### `execute_batch(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]`
Executes multiple workflows in batch.

#### Monitoring Methods

##### `get_performance_stats() -> PyPerformanceStats`
Returns performance statistics.

##### `get_pool_stats() -> PyObjectPoolStats`
Returns object pool statistics.

##### `get_max_node_execution_time() -> Optional[int]`
Returns maximum node execution time.

##### `get_fail_fast() -> Optional[bool]`
Returns fail-fast setting.

### PyWorkflowContext

Contains workflow execution context and results.

#### Instance Methods

##### `workflow_id() -> str`
Returns the workflow ID.

##### `state() -> str`
Returns the current workflow state.

##### `is_completed() -> bool`
Returns True if workflow completed successfully.

##### `is_failed() -> bool`
Returns True if workflow failed.

##### `get_variable(key: str) -> Optional[str]`
Gets a context variable.

**Parameters:**
- `key` (str): Variable name

**Returns:** Variable value or None

##### `variables() -> List[Tuple[str, str]]`
Returns all context variables as key-value pairs.

##### `execution_time_ms() -> int`
Returns workflow execution time in milliseconds.

### PyValidationResult

Contains workflow validation results.

#### Instance Methods

##### `is_valid() -> bool`
Returns True if validation passed.

##### `errors() -> List[str]`
Returns list of validation errors.

## Embeddings Classes

### PyEmbeddingConfig

Configuration class for embedding providers with fluent API design.

#### Static Methods

##### `openai(api_key: str, model: str) -> PyEmbeddingConfig`
Creates OpenAI embedding configuration.

**Parameters:**
- `api_key` (str): Your OpenAI API key
- `model` (str): OpenAI embedding model name

**Returns:** PyEmbeddingConfig instance

**Example:**
```python
config = graphbit.PyEmbeddingConfig.openai(
    "sk-...",
    "text-embedding-3-small"
)
```

**Supported Models:**
- `text-embedding-3-small` (1536 dimensions)
- `text-embedding-3-large` (3072 dimensions)
- `text-embedding-ada-002` (1536 dimensions)

##### `huggingface(api_key: str, model: str) -> PyEmbeddingConfig`
Creates HuggingFace embedding configuration.

**Parameters:**
- `api_key` (str): Your HuggingFace API token
- `model` (str): HuggingFace model identifier

**Returns:** PyEmbeddingConfig instance

**Example:**
```python
config = graphbit.PyEmbeddingConfig.huggingface(
    "hf_...",
    "sentence-transformers/all-MiniLM-L6-v2"
)
```

#### Instance Methods

##### `with_timeout(timeout_seconds: int) -> PyEmbeddingConfig`
Sets request timeout in seconds.

**Parameters:**
- `timeout_seconds` (int): Timeout in seconds

**Returns:** Modified PyEmbeddingConfig instance

##### `with_max_batch_size(size: int) -> PyEmbeddingConfig`
Sets maximum batch size for processing multiple texts.

**Parameters:**
- `size` (int): Maximum batch size

**Returns:** Modified PyEmbeddingConfig instance

##### `with_base_url(url: str) -> PyEmbeddingConfig`
Sets custom API endpoint URL.

**Parameters:**
- `url` (str): Custom endpoint URL

**Returns:** Modified PyEmbeddingConfig instance

##### `provider_name() -> str`
Returns the provider name ("openai" or "huggingface").

##### `model_name() -> str`
Returns the configured model name.

### PyEmbeddingService

Main service class for generating embeddings.

#### Constructor

##### `PyEmbeddingService(config: PyEmbeddingConfig)`
Creates an embedding service with the given configuration.

**Parameters:**
- `config` (PyEmbeddingConfig): Embedding configuration

**Raises:**
- `PyRuntimeError`: If service initialization fails

**Example:**
```python
config = graphbit.PyEmbeddingConfig.openai("your-key", "text-embedding-3-small")
service = graphbit.PyEmbeddingService(config)
```

#### Async Methods

##### `embed_text(text: str) -> List[float]`
Generates embedding for a single text.

**Parameters:**
- `text` (str): Input text to embed

**Returns:** List of floats representing the embedding vector

**Raises:**
- `PyRuntimeError`: If embedding generation fails

**Example:**
```python
embedding = await service.embed_text("Hello world")
print(f"Embedding has {len(embedding)} dimensions")
```

##### `embed_texts(texts: List[str]) -> List[List[float]]`
Generates embeddings for multiple texts efficiently.

**Parameters:**
- `texts` (List[str]): List of input texts

**Returns:** List of embedding vectors

**Raises:**
- `PyRuntimeError`: If embedding generation fails

**Example:**
```python
embeddings = await service.embed_texts([
    "First document",
    "Second document", 
    "Third document"
])
```

##### `get_dimensions() -> int`
Returns the embedding dimension size for the configured model.

**Returns:** Number of dimensions in embeddings

**Raises:**
- `PyRuntimeError`: If dimension detection fails

#### Static Methods

##### `cosine_similarity(a: List[float], b: List[float]) -> float`
Calculates cosine similarity between two embedding vectors.

**Parameters:**
- `a` (List[float]): First embedding vector
- `b` (List[float]): Second embedding vector

**Returns:** Similarity score between -1 and 1 (1 = identical, 0 = orthogonal, -1 = opposite)

**Raises:**
- `PyValueError`: If vectors have different dimensions

**Example:**
```python
similarity = graphbit.PyEmbeddingService.cosine_similarity(
    embedding1, embedding2
)
print(f"Similarity: {similarity:.3f}")
```

#### Instance Methods

##### `get_provider_info() -> (str, str)`
Returns tuple of (provider_name, model_name).

**Returns:** Tuple containing provider and model names

### PyEmbeddingRequest

Represents a request for embedding generation.

#### Constructor

##### `PyEmbeddingRequest(text: str)`
Creates a request for single text embedding.

**Parameters:**
- `text` (str): Text to embed

#### Static Methods

##### `multiple(texts: List[str]) -> PyEmbeddingRequest`
Creates a request for multiple text embeddings.

**Parameters:**
- `texts` (List[str]): Texts to embed

**Returns:** PyEmbeddingRequest instance

#### Instance Methods

##### `with_user(user: str) -> PyEmbeddingRequest`
Associates a user ID with the request.

**Parameters:**
- `user` (str): User identifier

**Returns:** Modified PyEmbeddingRequest instance

##### `input_count() -> int`
Returns the number of input texts.

### PyEmbeddingResponse

Represents a response from embedding generation.

#### Instance Methods

##### `embeddings() -> List[List[float]]`
Returns the generated embeddings.

##### `model() -> str`
Returns the model name used for generation.

##### `prompt_tokens() -> int`
Returns the number of prompt tokens processed.

##### `total_tokens() -> int`
Returns the total number of tokens processed.

##### `embedding_count() -> int`
Returns the number of embeddings generated.

##### `dimensions() -> int`
Returns the dimension size of embeddings.

## Configuration Classes

### PyRetryConfig

Configuration for retry behavior.

#### Static Methods

##### `new(max_attempts: int) -> PyRetryConfig`
Creates retry configuration.

**Parameters:**
- `max_attempts` (int): Maximum retry attempts

##### `default() -> PyRetryConfig`
Creates default retry configuration.

#### Instance Methods

##### `with_exponential_backoff(initial_delay_ms: int, multiplier: float, max_delay_ms: int) -> PyRetryConfig`
Configures exponential backoff.

**Parameters:**
- `initial_delay_ms` (int): Initial delay in milliseconds
- `multiplier` (float): Backoff multiplier
- `max_delay_ms` (int): Maximum delay in milliseconds

##### `with_jitter(jitter_factor: float) -> PyRetryConfig`
Adds jitter to retry delays.

**Parameters:**
- `jitter_factor` (float): Jitter factor (0.0-1.0)

##### `max_attempts() -> int`
Returns maximum attempts.

##### `initial_delay_ms() -> int`
Returns initial delay.

### PyCircuitBreakerConfig

Configuration for circuit breaker pattern.

#### Static Methods

##### `new(failure_threshold: int, recovery_timeout_ms: int) -> PyCircuitBreakerConfig`
Creates circuit breaker configuration.

**Parameters:**
- `failure_threshold` (int): Number of failures before opening circuit
- `recovery_timeout_ms` (int): Recovery timeout in milliseconds

##### `default() -> PyCircuitBreakerConfig`
Creates default circuit breaker configuration.

#### Instance Methods

##### `failure_threshold() -> int`
Returns failure threshold.

##### `recovery_timeout_ms() -> int`
Returns recovery timeout.

### PyPoolConfig

Configuration for object pools.

#### Static Methods

##### `new() -> PyPoolConfig`
Creates new pool configuration.

##### `high_throughput() -> PyPoolConfig`
Creates configuration optimized for high throughput.

##### `low_latency() -> PyPoolConfig`
Creates configuration optimized for low latency.

##### `memory_optimized() -> PyPoolConfig`
Creates configuration optimized for memory usage.

#### Instance Methods

##### `with_object_pools(enabled: bool) -> PyPoolConfig`
Enables or disables object pools.

##### `with_uuid_pool(enabled: bool) -> PyPoolConfig`
Enables or disables UUID pool.

##### `with_pool_sizes(agent_messages: int, node_results: int, contexts: int, hash_maps: int) -> PyPoolConfig`
Sets pool sizes.

**Parameters:**
- `agent_messages` (int): Agent message pool size
- `node_results` (int): Node result pool size
- `contexts` (int): Context pool size
- `hash_maps` (int): HashMap pool size

##### `object_pools_enabled() -> bool`
Returns True if object pools are enabled.

##### `uuid_pool_enabled() -> bool`
Returns True if UUID pool is enabled.

##### `__str__() -> str`
Returns string representation.

## Performance

### PyPerformanceStats

Performance statistics for workflow execution.

#### Attributes

- `max_concurrency` (int): Maximum concurrency level
- `timeout_ms` (int): Timeout in milliseconds
- `fail_fast_enabled` (bool): Whether fail-fast is enabled
- `retry_enabled` (bool): Whether retries are enabled
- `circuit_breaker_enabled` (bool): Whether circuit breaker is enabled

#### Instance Methods

##### `__str__() -> str`
Returns formatted string representation.

### PyObjectPoolStats

Object pool statistics.

#### Attributes

- `agent_messages_hit_rate` (float): Hit rate for agent message pool
- `node_results_hit_rate` (float): Hit rate for node results pool
- `workflow_contexts_hit_rate` (float): Hit rate for workflow contexts pool
- `hash_maps_hit_rate` (float): Hit rate for hash maps pool
- `total_hit_rate` (float): Overall hit rate

#### Instance Methods

##### `__str__() -> str`
Returns formatted string representation.

##### `__repr__() -> str`
Returns detailed representation.

### PyPerformanceMonitor

Static class for monitoring performance.

#### Static Methods

##### `get_object_pool_stats() -> PyObjectPoolStats`
Returns current object pool statistics.

##### `get_pool_sizes() -> Dict[str, int]`
Returns current pool sizes.

##### `get_detailed_pool_stats() -> Dict[str, Tuple[int, int, float]]`
Returns detailed pool statistics (capacity, size, hit_rate).

##### `prefill_uuid_pool() -> None`
Pre-fills the UUID pool for better performance.

##### `reset_pool_stats() -> None`
Resets pool statistics.

## Advanced Features

### Dynamic Workflow Generation

GraphBit provides powerful dynamic workflow generation capabilities that allow workflows to automatically generate new nodes and connections based on objectives and context. This enables adaptive, self-modifying workflows that can evolve during execution.

#### PyDynamicGraphConfig

Configuration class for dynamic graph generation behavior.

##### Constructor Methods

**`new() -> PyDynamicGraphConfig`**
Creates a new dynamic graph configuration with default settings.

**`with_max_auto_nodes(max_nodes: int) -> PyDynamicGraphConfig`**
Sets the maximum number of nodes that can be automatically generated.

**`with_confidence_threshold(threshold: float) -> PyDynamicGraphConfig`**
Sets the minimum confidence threshold for accepting generated nodes (0.0 to 1.0).

**`with_generation_temperature(temperature: float) -> PyDynamicGraphConfig`**
Sets the creativity level for node generation (0.0 = conservative, 1.0 = creative).

**`with_max_generation_depth(depth: int) -> PyDynamicGraphConfig`**
Sets the maximum depth for recursive node generation.

**`with_completion_objectives(objectives: List[str]) -> PyDynamicGraphConfig`**
Sets the objectives for workflow auto-completion.

**`enable_validation() -> PyDynamicGraphConfig`**
Enables validation of generated nodes before insertion.

**`disable_validation() -> PyDynamicGraphConfig`**
Disables validation for faster generation.

##### Example

```python
import graphbit

# Configure dynamic graph behavior
config = (graphbit.PyDynamicGraphConfig.new()
    .with_max_auto_nodes(10)
    .with_confidence_threshold(0.7)
    .with_generation_temperature(0.5)
    .with_max_generation_depth(3)
    .enable_validation())

print(f"Max nodes: {config.max_auto_nodes()}")
print(f"Confidence threshold: {config.confidence_threshold()}")
```

#### PyDynamicGraphManager

Core engine for dynamic workflow generation.

##### Constructor

**`new(llm_config: PyLlmConfig, config: PyDynamicGraphConfig) -> PyDynamicGraphManager`**
Creates a new dynamic graph manager.

**`initialize(llm_config: PyLlmConfig, config: PyDynamicGraphConfig) -> None`**
Initializes the manager with configuration.

##### Node Generation

**`generate_node(workflow_id: str, context: PyWorkflowContext, objective: str, allowed_node_types: Optional[List[str]]) -> PyDynamicNodeResponse`**
Generates a single node based on workflow context and objective.

**Parameters:**
- `workflow_id`: Unique identifier for the workflow
- `context`: Current workflow execution context
- `objective`: Description of what the node should accomplish
- `allowed_node_types`: Optional list of allowed node types (`["agent", "condition", "transform", "delay", "document_loader"]`)

**`auto_complete_workflow(workflow: PyWorkflow, context: PyWorkflowContext, objectives: List[str]) -> PyAutoCompletionResult`**
Automatically completes a workflow by generating necessary nodes and connections.

##### Analytics

**`get_analytics() -> PyDynamicGraphAnalytics`**
Returns analytics about the dynamic generation process.

##### Example

```python
import graphbit
import os

# Set up dynamic workflow generation
llm_config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
dynamic_config = (graphbit.PyDynamicGraphConfig.new()
    .with_max_auto_nodes(5)
    .with_confidence_threshold(0.8))

manager = graphbit.PyDynamicGraphManager.new(llm_config, dynamic_config)
manager.initialize(llm_config, dynamic_config)

# Create initial workflow
builder = graphbit.PyWorkflowBuilder("Dynamic Research")
initial_node = graphbit.PyWorkflowNode.agent_node(
    "Researcher", 
    "Initial research", 
    "researcher", 
    "Research the topic: {topic}"
)

node_id = builder.add_node(initial_node)
workflow = builder.build()

# Execute and generate additional nodes
executor = graphbit.PyWorkflowExecutor(llm_config)
context = executor.execute(workflow)

# Generate a follow-up node
response = manager.generate_node(
    workflow.id(),
    context,
    "Analyze the research findings and identify key insights",
    ["agent", "transform"]
)

print(f"Generated node: {response.get_node().name()}")
print(f"Confidence: {response.confidence():.2f}")
print(f"Reasoning: {response.reasoning()}")

# Auto-complete workflow
objectives = [
    "Summarize findings in executive format",
    "Generate actionable recommendations",
    "Create presentation slides"
]

completion_result = manager.auto_complete_workflow(workflow, context, objectives)
print(f"Generated {completion_result.node_count()} nodes")
print(f"Success: {completion_result.success()}")
```

#### PyDynamicNodeResponse

Response containing a generated node and metadata.

##### Methods

**`get_node() -> PyWorkflowNode`**
Returns the generated workflow node.

**`confidence() -> float`**
Returns the confidence score for the generated node (0.0 to 1.0).

**`reasoning() -> str`**
Returns the reasoning behind the node generation.

**`completes_workflow() -> bool`**
Returns true if this node completes the workflow objectives.

**`get_suggested_connections() -> List[PySuggestedConnection]`**
Returns suggested connections to other nodes.

#### PySuggestedConnection

Represents a suggested connection between nodes.

##### Methods

**`from_node() -> str`**
Returns the source node ID.

**`to_node() -> str`**
Returns the target node ID.

**`edge_type() -> str`**
Returns the suggested edge type ("data_flow", "control_flow", or "conditional").

**`confidence() -> float`**
Returns confidence in this connection suggestion.

**`reasoning() -> str`**
Returns reasoning for the suggested connection.

#### PyDynamicGraphAnalytics

Analytics and metrics for dynamic graph generation.

##### Methods

**`total_nodes_generated() -> int`**
Total number of nodes generated.

**`successful_generations() -> int`**
Number of successful node generations.

**`failed_generations() -> int`**
Number of failed node generations.

**`avg_confidence() -> float`**
Average confidence score of generated nodes.

**`cache_hit_rate() -> float`**
Cache hit rate for generation requests.

**`success_rate() -> float`**
Success rate of node generation (successful / total).

**`avg_generation_time() -> float`**
Average time in milliseconds to generate a node.

**`get_generation_times() -> List[int]`**
List of generation times for all attempts.

### Document Loader Nodes

Document loader nodes enable workflows to automatically load and process various document types as part of the workflow execution.

#### Creating Document Loader Nodes

**`PyWorkflowNode.document_loader_node(name: str, description: str, document_type: str, source_path: str) -> PyWorkflowNode`**

Creates a document loader node that can process various document formats.

**Parameters:**
- `name`: Node name
- `description`: Node description  
- `document_type`: Type of document to load (see supported types below)
- `source_path`: Path to the document file

**Supported Document Types:**
- `"pdf"` - PDF documents
- `"txt"` - Plain text files
- `"docx"` - Microsoft Word documents
- `"json"` - JSON data files
- `"csv"` - Comma-separated value files
- `"xml"` - XML documents
- `"html"` - HTML web pages

#### Example Usage

```python
import graphbit

# Create a document processing workflow
builder = graphbit.PyWorkflowBuilder("Document Analysis")

# Load a PDF document
pdf_loader = graphbit.PyWorkflowNode.document_loader_node(
    "PDF Loader",
    "Loads and extracts text from PDF document",
    "pdf",
    "/path/to/document.pdf"
)

# Load CSV data
csv_loader = graphbit.PyWorkflowNode.document_loader_node(
    "Data Loader", 
    "Loads CSV data for analysis",
    "csv",
    "/path/to/data.csv"
)

# Process the loaded content
analyzer = graphbit.PyWorkflowNode.agent_node(
    "Document Analyzer",
    "Analyzes loaded document content",
    "analyzer",
    "Analyze this document content and extract key insights: {document_content}"
)

# Build workflow
pdf_id = builder.add_node(pdf_loader)
csv_id = builder.add_node(csv_loader) 
analyzer_id = builder.add_node(analyzer)

# Connect document loaders to analyzer
builder.connect(pdf_id, analyzer_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(csv_id, analyzer_id, graphbit.PyWorkflowEdge.data_flow())

workflow = builder.build()

# Execute workflow
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
executor = graphbit.PyWorkflowExecutor(config)
context = executor.execute(workflow)

print(f"Document processing completed: {context.is_completed()}")
```

#### Multi-Document Processing Pipeline

```python
def create_document_pipeline():
    """Creates a comprehensive document processing pipeline."""
    builder = graphbit.PyWorkflowBuilder("Multi-Document Analysis")
    
    # Load multiple document types
    pdf_loader = graphbit.PyWorkflowNode.document_loader_node(
        "PDF Loader", "Load PDF reports", "pdf", "/docs/report.pdf"
    )
    
    excel_loader = graphbit.PyWorkflowNode.document_loader_node(
        "Excel Loader", "Load Excel data", "csv", "/docs/data.csv"  
    )
    
    json_loader = graphbit.PyWorkflowNode.document_loader_node(
        "Config Loader", "Load JSON configuration", "json", "/docs/config.json"
    )
    
    # Document preprocessing
    preprocessor = graphbit.PyWorkflowNode.transform_node(
        "Preprocessor",
        "Clean and normalize document content",
        "clean_text({document_content})"
    )
    
    # Content analysis
    content_analyzer = graphbit.PyWorkflowNode.agent_node(
        "Content Analyzer",
        "Performs content analysis",
        "content_analyzer",
        "Analyze the content from these documents and identify key themes: {content}"
    )
    
    # Data analysis  
    data_analyzer = graphbit.PyWorkflowNode.agent_node(
        "Data Analyzer",
        "Performs statistical analysis", 
        "data_analyzer",
        "Analyze this data and provide statistical insights: {data}"
    )
    
    # Report generator
    report_generator = graphbit.PyWorkflowNode.agent_node(
        "Report Generator",
        "Generates comprehensive report",
        "reporter", 
        "Generate a comprehensive report combining content analysis: {content_analysis} and data insights: {data_analysis}"
    )
    
    # Build the workflow
    pdf_id = builder.add_node(pdf_loader)
    excel_id = builder.add_node(excel_loader)
    json_id = builder.add_node(json_loader)
    prep_id = builder.add_node(preprocessor)
    content_id = builder.add_node(content_analyzer)
    data_id = builder.add_node(data_analyzer)
    report_id = builder.add_node(report_generator)
    
    # Connect the pipeline
    builder.connect(pdf_id, prep_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(prep_id, content_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(excel_id, data_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(content_id, report_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(data_id, report_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

# Execute the document pipeline
workflow = create_document_pipeline()
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
executor = graphbit.PyWorkflowExecutor(config).with_max_node_execution_time(60000)

context = executor.execute(workflow)
if context.is_completed():
    print("Document analysis pipeline completed successfully!")
    print(f"Processing time: {context.execution_time_ms()}ms")
else:
    print("Pipeline failed to complete")
```

#### Error Handling for Document Loaders

```python
try:
    # Create document loader with validation
    doc_loader = graphbit.PyWorkflowNode.document_loader_node(
        "Document Loader",
        "Load important document", 
        "pdf",  # Make sure this is a supported type
        "/path/to/document.pdf"
    )
    
    builder.add_node(doc_loader)
    
except ValueError as e:
    if "Unsupported document type" in str(e):
        print(f"Document type not supported: {e}")
        # Fall back to supported type or handle error
    else:
        print(f"Document loader error: {e}")
except Exception as e:
    print(f"Unexpected error creating document loader: {e}")
```

### Concurrent Workflow Execution

```python
import asyncio
import graphbit

async def run_multiple_workflows():
    config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")
    executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)
    
    workflows = [workflow1, workflow2, workflow3]
    results = executor.execute_concurrent(workflows)
    
    for i, result in enumerate(results):
        print(f"Workflow {i}: {'Success' if result.is_completed() else 'Failed'}")

asyncio.run(run_multiple_workflows())
```

### Performance Optimization

```python
# Create high-performance executor
config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")
pool_config = graphbit.PyPoolConfig.high_throughput()

executor = (graphbit.PyWorkflowExecutor.new_high_throughput(config)
    .with_pool_config(pool_config)
    .enable_memory_pools()
    .with_max_node_execution_time(30000))  # 30 seconds

# Monitor performance
stats = executor.get_performance_stats()
print(f"Max concurrency: {stats.max_concurrency}")

pool_stats = executor.get_pool_stats()
print(f"Total hit rate: {pool_stats.total_hit_rate:.2%}")
```

### Retry and Circuit Breaker

```python
# Configure retry behavior
retry_config = (graphbit.PyRetryConfig.new(3)
    .with_exponential_backoff(1000, 2.0, 10000)
    .with_jitter(0.1))

# Configure circuit breaker
circuit_config = graphbit.PyCircuitBreakerConfig.new(5, 60000)

# Create resilient executor
executor = (graphbit.PyWorkflowExecutor(config)
    .with_retry_config(retry_config)
    .with_circuit_breaker_config(circuit_config)
    .with_fail_fast(False))
```

## Error Handling

The library provides specific exception types for different error conditions:

```python
try:
    result = executor.execute(workflow)
except Exception as e:
    if "validation" in str(e).lower():
        print(f"Validation error: {e}")
    elif "timeout" in str(e).lower():
        print(f"Timeout error: {e}")
    else:
        print(f"Execution error: {e}")
```

## Examples

### Multi-Agent Research Pipeline

```python
import graphbit
import asyncio

async def create_research_pipeline():
    # Create workflow
    builder = graphbit.PyWorkflowBuilder("Research Pipeline")
    builder.description("Multi-agent research and analysis workflow")
    
    # Research agent
    researcher = graphbit.PyWorkflowNode.agent_node(
        "Researcher",
        "Conducts research on given topic",
        "researcher",
        "Research the topic '{topic}' and provide key findings with sources."
    )
    
    # Quality gate
    quality_check = graphbit.PyWorkflowNode.condition_node(
        "Quality Gate",
        "Validates research quality",
        "word_count > 200 && source_count >= 3"
    )
    
    # Analyst agent
    analyst = graphbit.PyWorkflowNode.agent_node(
        "Analyst",
        "Analyzes research data",
        "analyst", 
        "Analyze this research: {research_data}. Identify key patterns and insights."
    )
    
    # Summary agent
    summarizer = graphbit.PyWorkflowNode.agent_node(
        "Summarizer",
        "Creates executive summary",
        "summarizer",
        "Create a concise executive summary: {analysis}"
    )
    
    # Build workflow
    r_id = builder.add_node(researcher)
    q_id = builder.add_node(quality_check)
    a_id = builder.add_node(analyst)
    s_id = builder.add_node(summarizer)
    
    # Connect nodes
    builder.connect(r_id, q_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(q_id, a_id, graphbit.PyWorkflowEdge.conditional("passed"))
    builder.connect(a_id, s_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

# Execute
async def main():
    workflow = await create_research_pipeline()
    
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
    executor = graphbit.PyWorkflowExecutor(config)
    
    context = await executor.execute_async(workflow)
    
    if context.is_completed():
        print("Research completed successfully!")
        print(f"Execution time: {context.execution_time_ms()}ms")
    else:
        print("Research failed")

asyncio.run(main())
```

### Content Generation Pipeline

```python
def create_content_pipeline():
    builder = graphbit.PyWorkflowBuilder("Content Pipeline")
    
    # Ideation
    ideator = graphbit.PyWorkflowNode.agent_node(
        "Ideator",
        "Generates content ideas",
        "ideator",
        "Generate 5 creative ideas for content about: {topic}"
    )
    
    # Writer
    writer = graphbit.PyWorkflowNode.agent_node(
        "Writer", 
        "Writes content",
        "writer",
        "Write a {content_type} about {topic} using these ideas: {ideas}"
    )
    
    # Editor
    editor = graphbit.PyWorkflowNode.agent_node(
        "Editor",
        "Edits and improves content",
        "editor",
        "Edit and improve this content for clarity and engagement: {content}"
    )
    
    # Build workflow
    i_id = builder.add_node(ideator)
    w_id = builder.add_node(writer)
    e_id = builder.add_node(editor)
    
    builder.connect(i_id, w_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(w_id, e_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## API Reference

### Module Functions

#### `init() -> None`
Initializes the GraphBit library. Must be called before using other functions.

#### `version() -> str`
Returns the library version.

### Exception Handling

The library raises standard Python exceptions with descriptive messages:

- `PyValueError`: For invalid parameter values
- `RuntimeError`: For execution errors
- `TimeoutError`: For timeout conditions

## Best Practices

1. **Always call `graphbit.init()`** before using the library
2. **Use appropriate executor configurations** for your use case
3. **Implement proper error handling** around workflow execution
4. **Monitor performance** using the built-in statistics
5. **Validate workflows** before execution in production
6. **Use memory pools** for high-throughput scenarios
7. **Configure retries and circuit breakers** for resilient execution

## Performance Tips

1. Use `new_high_throughput()` for batch processing
2. Use `new_low_latency()` for real-time applications
3. Enable memory pools for repeated executions
4. Use concurrent execution for independent workflows
5. Monitor pool hit rates and adjust sizes accordingly
6. Pre-fill UUID pools for better performance

---

*This documentation covers GraphBit Python library version latest. For the most up-to-date information, refer to the official repository.* 
