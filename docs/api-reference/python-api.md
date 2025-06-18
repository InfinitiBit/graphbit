# Python API Reference

Complete reference for GraphBit's Python API. This document covers all classes, methods, and their usage.

## Module: `graphbit`

### Core Functions

#### `init()`
Initialize the GraphBit library with default configuration.

```python
import graphbit
graphbit.init()
```

**Returns**: `None`  
**Raises**: `GraphBitError` if initialization fails

#### `version()`
Get the current GraphBit version.

```python
version = graphbit.version()
print(f"GraphBit version: {version}")
```

**Returns**: `str` - Version string (e.g., "0.1.0")

---

## LLM Configuration

### `PyLlmConfig`

Configuration class for Large Language Model providers.

#### Static Methods

##### `PyLlmConfig.openai(api_key: str, model: str) -> PyLlmConfig`
Create OpenAI provider configuration.

```python
config = graphbit.PyLlmConfig.openai("sk-...", "gpt-4o-mini")
```

**Parameters**:
- `api_key` (str): OpenAI API key
- `model` (str): Model name (e.g., "gpt-4", "gpt-3.5-turbo")

**Returns**: `PyLlmConfig` instance

##### `PyLlmConfig.anthropic(api_key: str, model: str) -> PyLlmConfig`
Create Anthropic provider configuration.

```python
config = graphbit.PyLlmConfig.anthropic("sk-ant-...", "claude-3-sonnet-20240229")
```

**Parameters**:
- `api_key` (str): Anthropic API key
- `model` (str): Model name (e.g., "claude-3-sonnet-20240229")

**Returns**: `PyLlmConfig` instance

##### `PyLlmConfig.huggingface(api_key: str, model: str) -> PyLlmConfig`
Create HuggingFace provider configuration.

```python
config = graphbit.PyLlmConfig.huggingface("hf_...", "microsoft/DialoGPT-medium")
```

**Parameters**:
- `api_key` (str): HuggingFace API token
- `model` (str): Model name from HuggingFace hub

**Returns**: `PyLlmConfig` instance

#### Instance Methods

##### `provider_name() -> str`
Get the provider name.

```python
provider = config.provider_name()  # "openai", "anthropic", "huggingface"
```

##### `model_name() -> str`
Get the model name.

```python
model = config.model_name()  # "gpt-4", "claude-3-sonnet", etc.
```

---

## Workflow Building

### `PyWorkflowBuilder`

Builder class for creating workflows using the builder pattern.

#### Constructor

##### `PyWorkflowBuilder(name: str)`
Create a new workflow builder.

```python
builder = graphbit.PyWorkflowBuilder("My Workflow")
```

**Parameters**:
- `name` (str): Workflow name

#### Methods

##### `description(description: str) -> None`
Set workflow description.

```python
builder.description("A workflow for processing customer feedback")
```

**Parameters**:
- `description` (str): Workflow description

##### `add_node(node: PyWorkflowNode) -> str`
Add a node to the workflow.

```python
node_id = builder.add_node(my_node)
```

**Parameters**:
- `node` (PyWorkflowNode): Node to add

**Returns**: `str` - Unique node ID

##### `connect(from_id: str, to_id: str, edge: PyWorkflowEdge) -> None`
Connect two nodes with an edge.

```python
builder.connect(node1_id, node2_id, graphbit.PyWorkflowEdge.data_flow())
```

**Parameters**:
- `from_id` (str): Source node ID
- `to_id` (str): Target node ID  
- `edge` (PyWorkflowEdge): Edge defining the connection

##### `build() -> PyWorkflow`
Build the final workflow.

```python
workflow = builder.build()
```

**Returns**: `PyWorkflow` instance  
**Raises**: `ValueError` if workflow is invalid

### `PyWorkflow`

Represents a complete workflow ready for execution.

#### Constructor

##### `PyWorkflow(name: str, description: str)`
Create a new workflow directly (rarely used - prefer builder).

```python
workflow = graphbit.PyWorkflow("Test", "A test workflow")
```

#### Methods

##### `validate() -> None`
Validate the workflow structure.

```python
try:
    workflow.validate()
    print("Workflow is valid")
except Exception as e:
    print(f"Invalid workflow: {e}")
```

**Raises**: `ValueError` if workflow is invalid

##### `id() -> str`
Get the workflow ID.

##### `name() -> str`
Get the workflow name.

##### `description() -> str`
Get the workflow description.

##### `node_count() -> int`
Get the number of nodes.

##### `edge_count() -> int`
Get the number of edges.

##### `to_json() -> str`
Serialize workflow to JSON.

```python
json_str = workflow.to_json()
```

##### `from_json(json_str: str) -> PyWorkflow`
Deserialize workflow from JSON.

```python
workflow = graphbit.PyWorkflow.from_json(json_string)
```

---

## Workflow Nodes

### `PyWorkflowNode`

Factory class for creating different types of workflow nodes.

#### Static Methods

##### `agent_node(name: str, description: str, agent_id: str, prompt: str) -> PyWorkflowNode`
Create an AI agent node.

```python
agent = graphbit.PyWorkflowNode.agent_node(
    name="Content Analyzer",
    description="Analyzes content for sentiment",
    agent_id="analyzer",
    prompt="Analyze the sentiment of: {input}"
)
```

**Parameters**:
- `name` (str): Human-readable node name
- `description` (str): Node description
- `agent_id` (str): Unique agent identifier
- `prompt` (str): LLM prompt template with variables

##### `agent_node_with_config(name: str, description: str, agent_id: str, prompt: str, max_tokens: int, temperature: float) -> PyWorkflowNode`
Create an agent node with advanced configuration.

```python
agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Creative Writer",
    description="Writes creative content",
    agent_id="writer",
    prompt="Write a story about: {topic}",
    max_tokens=1000,
    temperature=0.8
)
```

**Parameters**:
- `name` (str): Node name
- `description` (str): Node description
- `agent_id` (str): Agent identifier
- `prompt` (str): Prompt template
- `max_tokens` (int): Maximum tokens to generate
- `temperature` (float): Creativity level (0.0-1.0)

##### `condition_node(name: str, description: str, expression: str) -> PyWorkflowNode`
Create a condition node for branching logic.

```python
condition = graphbit.PyWorkflowNode.condition_node(
    name="Quality Check",
    description="Checks if quality score is acceptable",
    expression="quality_score > 0.8"
)
```

**Parameters**:
- `name` (str): Node name
- `description` (str): Node description
- `expression` (str): Boolean expression to evaluate

##### `transform_node(name: str, description: str, transformation: str) -> PyWorkflowNode`
Create a data transformation node.

```python
transformer = graphbit.PyWorkflowNode.transform_node(
    name="Uppercase",
    description="Converts text to uppercase",
    transformation="uppercase"
)
```

**Parameters**:
- `name` (str): Node name
- `description` (str): Node description
- `transformation` (str): Transformation type

**Available transformations**:
- `"uppercase"` - Convert to uppercase
- `"lowercase"` - Convert to lowercase
- `"json_extract"` - Extract JSON from text
- `"split"` - Split text by delimiter
- `"join"` - Join text with delimiter

##### `delay_node(name: str, description: str, duration_seconds: int) -> PyWorkflowNode`
Create a delay node for timing control.

```python
delay = graphbit.PyWorkflowNode.delay_node(
    name="Rate Limit",
    description="Prevents API rate limiting",
    duration_seconds=5
)
```

**Parameters**:
- `name` (str): Node name
- `description` (str): Node description
- `duration_seconds` (int): Delay duration in seconds

##### `document_loader_node(name: str, description: str, document_type: str, source_path: str) -> PyWorkflowNode`
Create a document loader node.

```python
loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Loader",
    description="Loads PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)
```

**Parameters**:
- `name` (str): Node name
- `description` (str): Node description
- `document_type` (str): Document type ("pdf", "txt", "docx")
- `source_path` (str): Path to document

#### Instance Methods

##### `id() -> str`
Get the node ID.

##### `name() -> str`
Get the node name.

##### `description() -> str`
Get the node description.

---

## Workflow Edges

### `PyWorkflowEdge`

Defines connections between workflow nodes.

#### Static Methods

##### `data_flow() -> PyWorkflowEdge`
Create a data flow edge that passes data between nodes.

```python
edge = graphbit.PyWorkflowEdge.data_flow()
```

##### `control_flow() -> PyWorkflowEdge`
Create a control flow edge that controls execution order.

```python
edge = graphbit.PyWorkflowEdge.control_flow()
```

##### `conditional(condition: str) -> PyWorkflowEdge`
Create a conditional edge that executes based on a condition.

```python
edge = graphbit.PyWorkflowEdge.conditional("score > 0.5")
```

**Parameters**:
- `condition` (str): Boolean condition expression

---

## Workflow Execution

### `PyWorkflowExecutor`

Executes workflows with configurable performance and reliability settings.

#### Constructors

##### `PyWorkflowExecutor(llm_config: PyLlmConfig)`
Create a basic executor.

```python
executor = graphbit.PyWorkflowExecutor(config)
```

##### `PyWorkflowExecutor.new_high_throughput(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Create executor optimized for high throughput.

```python
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)
```

##### `PyWorkflowExecutor.new_low_latency(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Create executor optimized for low latency.

```python
executor = graphbit.PyWorkflowExecutor.new_low_latency(config)
```

##### `PyWorkflowExecutor.new_memory_optimized(llm_config: PyLlmConfig) -> PyWorkflowExecutor`
Create executor optimized for memory usage.

```python
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)
```

#### Configuration Methods

##### `with_max_node_execution_time(timeout_ms: int) -> PyWorkflowExecutor`
Set maximum execution time per node.

```python
executor = executor.with_max_node_execution_time(30000)  # 30 seconds
```

##### `with_fail_fast(fail_fast: bool) -> PyWorkflowExecutor`
Enable or disable fail-fast behavior.

```python
executor = executor.with_fail_fast(True)
```

##### `with_retry_config(retry_config: PyRetryConfig) -> PyWorkflowExecutor`
Add retry configuration.

```python
retry_config = graphbit.PyRetryConfig.default()
executor = executor.with_retry_config(retry_config)
```

##### `with_circuit_breaker_config(config: PyCircuitBreakerConfig) -> PyWorkflowExecutor`
Add circuit breaker configuration.

```python
cb_config = graphbit.PyCircuitBreakerConfig.default()
executor = executor.with_circuit_breaker_config(cb_config)
```

##### `without_retries() -> PyWorkflowExecutor`
Disable retry logic.

```python
executor = executor.without_retries()
```

#### Execution Methods

##### `execute(workflow: PyWorkflow) -> PyWorkflowContext`
Execute a workflow synchronously.

```python
result = executor.execute(workflow)
```

**Returns**: `PyWorkflowContext` - Execution result

##### `execute_async(workflow: PyWorkflow) -> Awaitable[PyWorkflowContext]`
Execute a workflow asynchronously.

```python
import asyncio

async def run_workflow():
    result = await executor.execute_async(workflow)
    return result

result = asyncio.run(run_workflow())
```

##### `execute_concurrent(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]`
Execute multiple workflows concurrently.

```python
workflows = [workflow1, workflow2, workflow3]
results = executor.execute_concurrent(workflows)
```

##### `execute_batch(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]`
Execute workflows in batches for efficiency.

```python
results = executor.execute_batch(workflows)
```

##### `execute_concurrent_agent_tasks(prompts: List[str], agent_id: str) -> List[str]`
Execute multiple prompts with the same agent concurrently.

```python
prompts = ["Analyze A", "Analyze B", "Analyze C"]
results = executor.execute_concurrent_agent_tasks(prompts, "analyzer")
```

---

## Workflow Context

### `PyWorkflowContext`

Contains workflow execution results and state.

#### Methods

##### `workflow_id() -> str`
Get the workflow ID.

##### `state() -> str`
Get the workflow state ("completed", "failed", "running", etc.).

##### `is_completed() -> bool`
Check if workflow completed successfully.

##### `is_failed() -> bool`
Check if workflow failed.

##### `get_variable(key: str) -> Optional[str]`
Get a variable value.

```python
output = result.get_variable("output")
if output:
    print(f"Result: {output}")
```

##### `variables() -> List[Tuple[str, str]]`
Get all variables as key-value pairs.

```python
all_vars = result.variables()
for key, value in all_vars:
    print(f"{key}: {value}")
```

##### `execution_time_ms() -> int`
Get execution time in milliseconds.

```python
time_ms = result.execution_time_ms()
print(f"Executed in {time_ms}ms")
```

---

## Reliability Configuration

### `PyRetryConfig`

Configuration for retry behavior.

#### Constructors

##### `PyRetryConfig(max_attempts: int)`
Create retry configuration.

```python
retry_config = graphbit.PyRetryConfig(3)  # Max 3 attempts
```

##### `PyRetryConfig.default() -> PyRetryConfig`
Create default retry configuration.

```python
retry_config = graphbit.PyRetryConfig.default()
```

#### Methods

##### `with_exponential_backoff(initial_delay_ms: int, multiplier: float, max_delay_ms: int) -> PyRetryConfig`
Configure exponential backoff.

```python
retry_config = retry_config.with_exponential_backoff(
    initial_delay_ms=1000,
    multiplier=2.0,
    max_delay_ms=30000
)
```

##### `with_jitter(jitter_factor: float) -> PyRetryConfig`
Add jitter to retry delays.

```python
retry_config = retry_config.with_jitter(0.1)  # 10% jitter
```

##### `max_attempts() -> int`
Get maximum retry attempts.

##### `initial_delay_ms() -> int`
Get initial delay in milliseconds.

### `PyCircuitBreakerConfig`

Configuration for circuit breaker pattern.

#### Constructors

##### `PyCircuitBreakerConfig(failure_threshold: int, recovery_timeout_ms: int)`
Create circuit breaker configuration.

```python
cb_config = graphbit.PyCircuitBreakerConfig(5, 60000)  # 5 failures, 60s recovery
```

##### `PyCircuitBreakerConfig.default() -> PyCircuitBreakerConfig`
Create default circuit breaker configuration.

```python
cb_config = graphbit.PyCircuitBreakerConfig.default()
```

#### Methods

##### `failure_threshold() -> int`
Get failure threshold.

##### `recovery_timeout_ms() -> int`
Get recovery timeout in milliseconds.

---

## Embeddings

### `PyEmbeddingConfig`

Configuration for embedding providers.

#### Static Methods

##### `PyEmbeddingConfig.openai(api_key: str, model: str) -> PyEmbeddingConfig`
Create OpenAI embeddings configuration.

```python
config = graphbit.PyEmbeddingConfig.openai("sk-...", "text-embedding-ada-002")
```

##### `PyEmbeddingConfig.huggingface(api_key: str, model: str) -> PyEmbeddingConfig`
Create HuggingFace embeddings configuration.

```python
config = graphbit.PyEmbeddingConfig.huggingface("hf_...", "sentence-transformers/all-MiniLM-L6-v2")
```

#### Methods

##### `with_base_url(base_url: str) -> PyEmbeddingConfig`
Set custom base URL.

##### `with_timeout(timeout_seconds: int) -> PyEmbeddingConfig`
Set request timeout.

##### `with_max_batch_size(max_batch_size: int) -> PyEmbeddingConfig`
Set maximum batch size.

##### `provider_name() -> str`
Get provider name.

##### `model_name() -> str`
Get model name.

### `PyEmbeddingService`

Service for generating text embeddings.

#### Constructor

##### `PyEmbeddingService(config: PyEmbeddingConfig)`
Create embedding service.

```python
service = graphbit.PyEmbeddingService(config)
```

#### Methods

##### `embed_text(text: str) -> Awaitable[List[float]]`
Generate embedding for single text.

```python
import asyncio

async def get_embedding():
    embedding = await service.embed_text("Hello world")
    return embedding

embedding = asyncio.run(get_embedding())
```

##### `embed_texts(texts: List[str]) -> Awaitable[List[List[float]]]`
Generate embeddings for multiple texts.

```python
async def get_embeddings():
    texts = ["Text 1", "Text 2", "Text 3"]
    embeddings = await service.embed_texts(texts)
    return embeddings
```

##### `cosine_similarity(a: List[float], b: List[float]) -> float`
Calculate cosine similarity between two embeddings.

```python
similarity = graphbit.PyEmbeddingService.cosine_similarity(embed1, embed2)
print(f"Similarity: {similarity}")
```

---

## Agent Capabilities

### `PyAgentCapability`

Represents different agent capabilities.

#### Static Methods

##### `text_processing() -> PyAgentCapability`
Text processing capability.

##### `data_analysis() -> PyAgentCapability`
Data analysis capability.

##### `tool_execution() -> PyAgentCapability`
Tool execution capability.

##### `decision_making() -> PyAgentCapability`
Decision making capability.

##### `custom(name: str) -> PyAgentCapability`
Custom capability.

```python
custom_cap = graphbit.PyAgentCapability.custom("domain_expert")
```

---

## Error Handling

GraphBit uses standard Python exceptions:

- `ValueError` - Invalid parameters or workflow structure
- `RuntimeError` - Execution errors
- `TimeoutError` - Operation timeouts
- `ConnectionError` - Network/API errors

```python
try:
    result = executor.execute(workflow)
except ValueError as e:
    print(f"Invalid workflow: {e}")
except RuntimeError as e:
    print(f"Execution failed: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

## Usage Examples

### Basic Workflow
```python
import graphbit
import os

# Initialize and configure
graphbit.init()
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Build workflow
builder = graphbit.PyWorkflowBuilder("Analysis")
agent = graphbit.PyWorkflowNode.agent_node(
    "Analyzer", "Analyzes data", "analyzer", "Analyze: {input}"
)
builder.add_node(agent)
workflow = builder.build()

# Execute
executor = graphbit.PyWorkflowExecutor(config)
result = executor.execute(workflow)
print(result.get_variable("output"))
```

### Advanced Configuration
```python
# Create executor with full configuration
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(
        graphbit.PyRetryConfig.default()
        .with_exponential_backoff(1000, 2.0, 30000)
        .with_jitter(0.1)
    ) \
    .with_circuit_breaker_config(
        graphbit.PyCircuitBreakerConfig.default()
    ) \
    .with_max_node_execution_time(30000) \
    .with_fail_fast(False)
``` 
