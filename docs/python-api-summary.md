---
title: GraphBit Python API Summary
description: Complete overview of all Python classes and methods available in GraphBit
---

# GraphBit Python API Reference

Comprehensive reference for the GraphBit Python bindings - a high-performance agentic workflow automation framework.

## 🚀 Quick Start

```python
import graphbit
import os

# Initialize GraphBit
graphbit.init()

# Configure LLM provider
config = graphbit.PyLlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)

# Build and execute workflow
builder = graphbit.PyWorkflowBuilder("Hello GraphBit")
node = graphbit.PyWorkflowNode.agent_node(
    "Greeter", "Generates greetings", "greeter-001", 
    "Say hello to {name}"
)

node_id = builder.add_node(node)
workflow = builder.build()

executor = graphbit.PyWorkflowExecutor(config)
result = executor.execute(workflow)
print(f"Completed in {result.execution_time_ms()}ms")
```

---

## 🏗️ Core Classes

### PyLlmConfig

Configuration for LLM providers.

#### Static Methods

```python
# Create OpenAI configuration
config = graphbit.PyLlmConfig.openai(api_key: str, model: str)

# Create Anthropic configuration  
config = graphbit.PyLlmConfig.anthropic(api_key: str, model: str)

# Create HuggingFace configuration
config = graphbit.PyLlmConfig.huggingface(api_key: str, model: str)

# Create Ollama configuration (local)
config = graphbit.PyLlmConfig.ollama(model: str, base_url: str)
```

#### Instance Methods

```python
# Get provider information
provider_name = config.provider_name()  # Returns: str
model_name = config.model_name()        # Returns: str
```

#### Example

```python
# Multiple provider configurations
openai_config = graphbit.PyLlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o"
)

anthropic_config = graphbit.PyLlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"), 
    model="claude-3-5-sonnet-20241022"
)

ollama_config = graphbit.PyLlmConfig.ollama(
    model="llama3.1:8b",
    base_url="http://localhost:11434"
)
```

---

### PyWorkflowBuilder

Builder pattern for creating workflows.

#### Constructor

```python
builder = graphbit.PyWorkflowBuilder(name: str)
```

#### Methods

```python
# Set workflow description
builder.description(description: str) -> None

# Add nodes to workflow
node_id = builder.add_node(node: PyWorkflowNode) -> str

# Connect nodes with edges
builder.connect(from_id: str, to_id: str, edge: PyWorkflowEdge) -> None

# Build final workflow
workflow = builder.build() -> PyWorkflow
```

#### Example

```python
builder = graphbit.PyWorkflowBuilder("Content Pipeline")
builder.description("Automated content creation workflow")

# Add nodes
researcher_id = builder.add_node(research_node)
writer_id = builder.add_node(writer_node)
editor_id = builder.add_node(editor_node)

# Connect with data flow
builder.connect(researcher_id, writer_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(writer_id, editor_id, graphbit.PyWorkflowEdge.data_flow())

workflow = builder.build()
```

---

### PyWorkflowNode

Factory for creating different types of workflow nodes.

#### Agent Nodes

```python
# Basic agent node
node = graphbit.PyWorkflowNode.agent_node(
    name: str,
    description: str, 
    agent_id: str,
    prompt: str
)

# Agent node with custom configuration
node = graphbit.PyWorkflowNode.agent_node_with_config(
    name: str,
    description: str,
    agent_id: str,
    prompt: str,
    max_tokens: int,
    temperature: float
)
```

#### Condition Nodes

```python
# Conditional logic node
condition = graphbit.PyWorkflowNode.condition_node(
    name: str,
    description: str,
    expression: str  # Boolean expression (e.g., "quality_score > 0.8")
)
```

#### Transform Nodes

```python
# Data transformation node
transform = graphbit.PyWorkflowNode.transform_node(
    name: str,
    description: str,
    transformation: str  # "uppercase", "lowercase", "json_extract", "split", "join"
)

# JSON extraction with field specification
json_extractor = graphbit.PyWorkflowNode.transform_node(
    name: str,
    description: str,
    transformation: "json_extract",
    field: str  # Optional field to extract
)
```

#### Delay Nodes

```python
# Rate limiting / timing control
delay = graphbit.PyWorkflowNode.delay_node(
    name: str,
    description: str,
    duration_seconds: int
)
```

#### Document Loader Nodes

```python
# Document processing node
loader = graphbit.PyWorkflowNode.document_loader_node(
    name: str,
    description: str,
    document_type: str,  # "pdf", "text", "markdown", "html"
    source_path: str
)
```

#### Node Properties

```python
# Get node information
node_id = node.id()                    # Returns: str
node_name = node.name()                # Returns: str
node_description = node.description()  # Returns: str
```

#### Examples

```python
# Content analysis agent
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Content Analyzer",
    description="Analyzes content quality and readability",
    agent_id="analyzer-v2",
    prompt="Analyze the quality, readability, and tone of: {content}"
)

# Quality gate condition
quality_gate = graphbit.PyWorkflowNode.condition_node(
    name="Quality Gate",
    description="Ensures content meets minimum standards",
    expression="quality_score >= 0.7 && readability_score >= 0.6"
)

# Text transformer
formatter = graphbit.PyWorkflowNode.transform_node(
    name="Text Formatter",
    description="Formats text to uppercase",
    transformation="uppercase"
)

# Rate limiter
rate_limit = graphbit.PyWorkflowNode.delay_node(
    name="API Rate Limiter",
    description="Prevents exceeding API rate limits",
    duration_seconds=2
)

# PDF document processor
pdf_processor = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Processor",
    description="Extracts text from PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)
```

---

### PyWorkflowEdge

Defines how nodes connect and data flows.

#### Static Methods

```python
# Data flow edge (passes output to next node)
edge = graphbit.PyWorkflowEdge.data_flow()

# Control flow edge (execution order only)
edge = graphbit.PyWorkflowEdge.control_flow()

# Conditional edge (executes based on condition)
edge = graphbit.PyWorkflowEdge.conditional(condition: str)
```

#### Examples

```python
# Simple data flow
data_edge = graphbit.PyWorkflowEdge.data_flow()

# Execution control
control_edge = graphbit.PyWorkflowEdge.control_flow()

# Conditional execution
approval_edge = graphbit.PyWorkflowEdge.conditional("status == 'approved'")
quality_edge = graphbit.PyWorkflowEdge.conditional("quality_score >= 0.8")
```

---

### PyWorkflowExecutor

Executes workflows with reliability features.

#### Constructors

```python
# Basic executor
executor = graphbit.PyWorkflowExecutor(config: PyLlmConfig)

# Performance-optimized presets
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config: PyLlmConfig)
executor = graphbit.PyWorkflowExecutor.new_low_latency(config: PyLlmConfig)
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config: PyLlmConfig)
```

#### Configuration Methods

```python
# Reliability configuration
executor = executor.with_retry_config(retry_config: PyRetryConfig)
executor = executor.with_circuit_breaker_config(circuit_breaker: PyCircuitBreakerConfig)
executor = executor.without_retries()

# Performance configuration
executor = executor.with_max_node_execution_time(timeout_ms: int)
executor = executor.with_fail_fast(fail_fast: bool)
```

#### Execution Methods

```python
# Synchronous execution
result = executor.execute(workflow: PyWorkflow) -> PyWorkflowContext

# Asynchronous execution
result = await executor.execute_async(workflow: PyWorkflow) -> PyWorkflowContext

# Concurrent workflow execution
results = executor.execute_concurrent(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]

# Batch execution
results = executor.execute_batch(workflows: List[PyWorkflow]) -> List[PyWorkflowContext]

# Concurrent agent task execution
results = executor.execute_concurrent_agent_tasks(
    prompts: List[str], 
    agent_id: str
) -> List[str]
```

#### Monitoring

```python
# Get concurrency statistics
stats = await executor.get_concurrency_stats() -> PyConcurrencyStats
```

#### Examples

```python
# Production executor with reliability
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(graphbit.PyRetryConfig.default()) \
    .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default()) \
    .with_max_node_execution_time(300000)  # 5 minutes

# Low-latency executor for real-time applications
fast_executor = graphbit.PyWorkflowExecutor.new_low_latency(config) \
    .without_retries() \
    .with_fail_fast(True)

# Execute multiple workflows concurrently
workflows = [workflow1, workflow2, workflow3]
results = executor.execute_concurrent(workflows)

# Monitor performance
stats = await executor.get_concurrency_stats()
print(f"Active tasks: {stats.current_active_tasks}")
```

---

### PyWorkflowContext

Execution context and results.

#### Methods

```python
# Workflow information
workflow_id = context.workflow_id()     # Returns: str
state = context.state()                 # Returns: str
execution_time = context.execution_time_ms()  # Returns: int

# State checks
is_completed = context.is_completed()   # Returns: bool
is_failed = context.is_failed()         # Returns: bool

# Variable access
value = context.get_variable(key: str)  # Returns: Optional[str]
variables = context.variables()         # Returns: List[Tuple[str, str]]
```

#### Examples

```python
result = executor.execute(workflow)

print(f"Workflow: {result.workflow_id()}")
print(f"Status: {result.state()}")
print(f"Duration: {result.execution_time_ms()}ms")

if result.is_completed():
    output = result.get_variable("output")
    print(f"Result: {output}")
elif result.is_failed():
    error = result.get_variable("error")
    print(f"Error: {error}")

# Get all variables
for key, value in result.variables():
    print(f"{key}: {value}")
```

---

## 🧠 Embeddings API

### PyEmbeddingConfig

Configuration for embedding providers.

#### Static Methods

```python
# OpenAI embeddings
config = graphbit.PyEmbeddingConfig.openai(api_key: str, model: str)

# HuggingFace embeddings
config = graphbit.PyEmbeddingConfig.huggingface(api_key: str, model: str)
```

#### Configuration Methods

```python
# Chain configuration
config = config.with_base_url(base_url: str)
config = config.with_timeout(timeout_seconds: int)
config = config.with_max_batch_size(max_batch_size: int)
```

#### Properties

```python
provider = config.provider_name()  # Returns: str
model = config.model_name()        # Returns: str
```

### PyEmbeddingService

Service for generating embeddings.

#### Constructor

```python
service = graphbit.PyEmbeddingService(config: PyEmbeddingConfig)
```

#### Async Methods

```python
# Single text embedding
embedding = await service.embed_text(text: str) -> List[float]

# Multiple text embeddings
embeddings = await service.embed_texts(texts: List[str]) -> List[List[float]]

# Get embedding dimensions
dimensions = await service.get_dimensions() -> int
```

#### Static Methods

```python
# Calculate cosine similarity
similarity = graphbit.PyEmbeddingService.cosine_similarity(
    a: List[float], 
    b: List[float]
) -> float
```

#### Properties

```python
provider, model = service.get_provider_info()  # Returns: Tuple[str, str]
```

#### Examples

```python
import asyncio

async def embedding_example():
    # Configure embedding service
    config = graphbit.PyEmbeddingConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="text-embedding-3-small"
    ).with_timeout(30).with_max_batch_size(100)
    
    service = graphbit.PyEmbeddingService(config)
    
    # Generate embeddings
    texts = ["Hello world", "Good morning", "How are you?"]
    embeddings = await service.embed_texts(texts)
    
    # Calculate similarities
    sim_01 = graphbit.PyEmbeddingService.cosine_similarity(embeddings[0], embeddings[1])
    sim_02 = graphbit.PyEmbeddingService.cosine_similarity(embeddings[0], embeddings[2])
    
    print(f"Similarity 0-1: {sim_01:.3f}")
    print(f"Similarity 0-2: {sim_02:.3f}")
    
    # Model information
    dimensions = await service.get_dimensions()
    provider, model = service.get_provider_info()
    print(f"Model: {provider}/{model} ({dimensions} dimensions)")

asyncio.run(embedding_example())
```

---

## 🔄 Dynamic Workflow Generation

### PyDynamicGraphConfig

Configuration for dynamic workflow generation.

#### Constructor and Methods

```python
# Create configuration
config = graphbit.PyDynamicGraphConfig()

# Chain configuration
config = config.with_max_auto_nodes(max_nodes: int)
config = config.with_confidence_threshold(threshold: float)
config = config.with_generation_temperature(temperature: float)
config = config.with_max_generation_depth(depth: int)
config = config.with_completion_objectives(objectives: List[str])
config = config.enable_validation()
config = config.disable_validation()
```

#### Properties

```python
max_nodes = config.max_auto_nodes()              # Returns: int
threshold = config.confidence_threshold()        # Returns: float
temperature = config.generation_temperature()    # Returns: float
```

### PyDynamicGraphManager

Manages dynamic workflow generation.

#### Constructor

```python
manager = graphbit.PyDynamicGraphManager(
    llm_config: PyLlmConfig,
    config: PyDynamicGraphConfig
)

# Initialize after creation
manager.initialize(llm_config: PyLlmConfig, config: PyDynamicGraphConfig)
```

#### Methods

```python
# Generate individual nodes
response = manager.generate_node(
    workflow_id: str,
    context: PyWorkflowContext,
    objective: str,
    allowed_node_types: Optional[List[str]] = None
) -> PyDynamicNodeResponse

# Auto-complete workflows
result = manager.auto_complete_workflow(
    workflow: PyWorkflow,
    context: PyWorkflowContext, 
    objectives: List[str]
) -> PyAutoCompletionResult

# Get analytics
analytics = manager.get_analytics() -> PyDynamicGraphAnalytics
```

### PyWorkflowAutoCompletion

Engine for automatic workflow completion.

#### Constructor

```python
auto_completion = graphbit.PyWorkflowAutoCompletion(manager: PyDynamicGraphManager)

# Initialize after creation
auto_completion.initialize(manager: PyDynamicGraphManager)
```

#### Methods

```python
# Complete workflow
result = auto_completion.complete_workflow(
    workflow: PyWorkflow,
    context: PyWorkflowContext,
    objectives: List[str]
) -> PyAutoCompletionResult

# Configuration
auto_completion = auto_completion.with_max_iterations(max_iterations: int)
auto_completion = auto_completion.with_timeout_ms(timeout_ms: int)
```

#### Examples

```python
async def dynamic_workflow_example():
    # Configure dynamic generation
    dynamic_config = graphbit.PyDynamicGraphConfig() \
        .with_max_auto_nodes(15) \
        .with_confidence_threshold(0.8) \
        .with_generation_temperature(0.2) \
        .enable_validation()
    
    # Create manager
    manager = graphbit.PyDynamicGraphManager(llm_config, dynamic_config)
    manager.initialize(llm_config, dynamic_config)
    
    # Start with base workflow
    builder = graphbit.PyWorkflowBuilder("Dynamic Content Creation")
    base_workflow = builder.build()
    
    # Create context
    context = graphbit.PyWorkflowContext()
    context.set_variable("topic", "Machine Learning Basics")
    context.set_variable("audience", "Beginners")
    
    # Define objectives
    objectives = [
        "Create comprehensive educational content",
        "Ensure accuracy and clarity",
        "Include practical examples"
    ]
    
    # Auto-complete workflow
    auto_completion = graphbit.PyWorkflowAutoCompletion(manager)
    result = auto_completion.complete_workflow(base_workflow, context, objectives)
    
    # Analyze results
    analytics = result.get_analytics()
    print(f"Generated {analytics.total_nodes_generated()} nodes")
    print(f"Success rate: {analytics.success_rate():.2%}")
    print(f"Completion time: {result.completion_time_ms()}ms")
    
    return base_workflow
```

---

## 🛡️ Reliability Features

### PyRetryConfig

Configuration for retry policies.

#### Constructor

```python
retry_config = graphbit.PyRetryConfig(max_attempts: int)
```

#### Static Methods

```python
# Default retry configuration
retry_config = graphbit.PyRetryConfig.default()
```

#### Configuration Methods

```python
# Exponential backoff
retry_config = retry_config.with_exponential_backoff(
    initial_delay_ms: int,
    multiplier: float,
    max_delay_ms: int
)

# Add jitter to prevent thundering herd
retry_config = retry_config.with_jitter(jitter_factor: float)
```

#### Properties

```python
max_attempts = retry_config.max_attempts()           # Returns: int
initial_delay = retry_config.initial_delay_ms()     # Returns: int
```

### PyCircuitBreakerConfig

Configuration for circuit breakers.

#### Constructor

```python
circuit_breaker = graphbit.PyCircuitBreakerConfig(
    failure_threshold: int,
    recovery_timeout_ms: int
)
```

#### Static Methods

```python
# Default circuit breaker configuration
circuit_breaker = graphbit.PyCircuitBreakerConfig.default()
```

#### Properties

```python
threshold = circuit_breaker.failure_threshold()      # Returns: int
timeout = circuit_breaker.recovery_timeout_ms()     # Returns: int
```

#### Examples

```python
# Configure retry with exponential backoff
retry_config = graphbit.PyRetryConfig(max_attempts=5) \
    .with_exponential_backoff(
        initial_delay_ms=1000,   # Start with 1 second
        multiplier=2.0,          # Double each time
        max_delay_ms=30000       # Cap at 30 seconds
    ) \
    .with_jitter(0.1)           # Add 10% randomness

# Configure circuit breaker
circuit_breaker = graphbit.PyCircuitBreakerConfig(
    failure_threshold=10,        # Open after 10 failures
    recovery_timeout_ms=60000    # Wait 1 minute before retry
)

# Apply to executor
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(retry_config) \
    .with_circuit_breaker_config(circuit_breaker)
```

---

## 🎯 Agent Capabilities

### PyAgentCapability

Defines agent capabilities and specializations.

#### Static Methods

```python
# Predefined capabilities
text_processing = graphbit.PyAgentCapability.text_processing()
data_analysis = graphbit.PyAgentCapability.data_analysis()
tool_execution = graphbit.PyAgentCapability.tool_execution()
decision_making = graphbit.PyAgentCapability.decision_making()

# Custom capability
custom_capability = graphbit.PyAgentCapability.custom(name: str)
```

#### Examples

```python
# Define specialized agents with capabilities
research_capability = graphbit.PyAgentCapability.data_analysis()
writing_capability = graphbit.PyAgentCapability.text_processing()
decision_capability = graphbit.PyAgentCapability.decision_making()
custom_seo = graphbit.PyAgentCapability.custom("seo_optimization")

print(f"Research capability: {research_capability}")
print(f"Writing capability: {writing_capability}")
print(f"Decision capability: {decision_capability}")
print(f"Custom SEO capability: {custom_seo}")
```

---

## 📊 Monitoring & Analytics

### PyConcurrencyStats

Concurrency and performance statistics.

#### Properties

```python
# Current state
current_active = stats.current_active_tasks          # Returns: int
peak_active = stats.peak_active_tasks               # Returns: int

# Performance metrics
total_acquisitions = stats.total_permit_acquisitions # Returns: int
avg_wait_time = stats.avg_wait_time_ms              # Returns: float
permit_failures = stats.permit_failures            # Returns: int
```

### PyDynamicGraphAnalytics

Analytics for dynamic workflow generation.

#### Methods

```python
# Generation statistics
total_generated = analytics.total_nodes_generated()     # Returns: int
successful = analytics.successful_generations()          # Returns: int
failed = analytics.failed_generations()                 # Returns: int

# Performance metrics
avg_confidence = analytics.avg_confidence()             # Returns: float
cache_hit_rate = analytics.cache_hit_rate()            # Returns: float
success_rate = analytics.success_rate()                # Returns: float
avg_generation_time = analytics.avg_generation_time()   # Returns: float

# Detailed metrics
generation_times = analytics.get_generation_times()     # Returns: List[int]
```

#### Examples

```python
# Monitor workflow execution
result = executor.execute(workflow)
print(f"Execution time: {result.execution_time_ms()}ms")

# Monitor concurrency
stats = await executor.get_concurrency_stats()
print(f"Active tasks: {stats.current_active_tasks}")
print(f"Peak concurrency: {stats.peak_active_tasks}")
print(f"Average wait time: {stats.avg_wait_time_ms:.2f}ms")

# Monitor dynamic generation
analytics = manager.get_analytics()
print(f"Generated nodes: {analytics.total_nodes_generated()}")
print(f"Success rate: {analytics.success_rate():.2%}")
print(f"Average confidence: {analytics.avg_confidence():.2f}")
print(f"Cache hit rate: {analytics.cache_hit_rate():.2%}")
```

---

## 📁 Validation & Results

### PyValidationResult

Results from workflow validation.

#### Methods

```python
# Validation status
is_valid = result.is_valid()           # Returns: bool
errors = result.errors()               # Returns: List[str]
```

### PyWorkflow

Workflow definition and management.

#### Constructor

```python
workflow = graphbit.PyWorkflow(name: str, description: str)
```

#### Methods

```python
# Add nodes
node_id = workflow.add_node(node: PyWorkflowNode) -> str

# Connect nodes
workflow.connect_nodes(from_id: str, to_id: str, edge: PyWorkflowEdge)

# Validation
workflow.validate()  # Raises exception if invalid

# Properties
workflow_id = workflow.id()              # Returns: str
name = workflow.name()                   # Returns: str
description = workflow.description()     # Returns: str
node_count = workflow.node_count()       # Returns: int
edge_count = workflow.edge_count()       # Returns: int

# Serialization
json_str = workflow.to_json()            # Returns: str
workflow = graphbit.PyWorkflow.from_json(json_str: str)  # Returns: PyWorkflow
```

---

## 🚀 Module Functions

### Initialization

```python
# Initialize GraphBit (required before use)
graphbit.init()

# Get version information
version = graphbit.version()  # Returns: str
```

---

## 💡 Common Patterns

### Basic Workflow Pattern

```python
import graphbit
import os

# Standard workflow setup
graphbit.init()
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create workflow
builder = graphbit.PyWorkflowBuilder("My Workflow")
node = graphbit.PyWorkflowNode.agent_node("Agent", "Description", "agent-id", "Prompt: {input}")
node_id = builder.add_node(node)
workflow = builder.build()

# Execute
executor = graphbit.PyWorkflowExecutor(config)
result = executor.execute(workflow)
```

### RAG Pattern

```python
async def rag_pattern(query, documents):
    # Embedding setup
    embed_config = graphbit.PyEmbeddingConfig.openai(api_key, "text-embedding-3-small")
    embed_service = graphbit.PyEmbeddingService(embed_config)
    
    # Find relevant documents
    query_embedding = await embed_service.embed_text(query)
    doc_embeddings = await embed_service.embed_texts(documents)
    
    similarities = []
    for i, doc_embed in enumerate(doc_embeddings):
        sim = graphbit.PyEmbeddingService.cosine_similarity(query_embedding, doc_embed)
        similarities.append((documents[i], sim))
    
    # Get top documents
    top_docs = sorted(similarities, key=lambda x: x[1], reverse=True)[:3]
    context = "\n\n".join([doc for doc, _ in top_docs])
    
    # Create RAG workflow
    builder = graphbit.PyWorkflowBuilder("RAG")
    rag_node = graphbit.PyWorkflowNode.agent_node(
        "RAG Agent", "Answers using context", "rag",
        f"Context: {context}\n\nQuestion: {query}\n\nAnswer:"
    )
    
    workflow = builder.add_node(rag_node).build()
    executor = graphbit.PyWorkflowExecutor(llm_config)
    return executor.execute(workflow)
```

### Multi-Step Processing Pattern

```python
def multi_step_workflow():
    builder = graphbit.PyWorkflowBuilder("Multi-Step Processing")
    
    # Create processing chain
    nodes = []
    nodes.append(("loader", graphbit.PyWorkflowNode.document_loader_node(
        "Loader", "Loads documents", "pdf", "/path/to/doc.pdf")))
    nodes.append(("analyzer", graphbit.PyWorkflowNode.agent_node(
        "Analyzer", "Analyzes content", "analyzer", "Analyze: {document}")))
    nodes.append(("summarizer", graphbit.PyWorkflowNode.agent_node(
        "Summarizer", "Creates summary", "summarizer", "Summarize: {analysis}")))
    
    # Add nodes and connect sequentially
    node_ids = []
    for name, node in nodes:
        node_id = builder.add_node(node)
        node_ids.append(node_id)
        
        if len(node_ids) > 1:
            builder.connect(node_ids[-2], node_ids[-1], graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Conditional Processing Pattern

```python
def conditional_workflow():
    builder = graphbit.PyWorkflowBuilder("Conditional Processing")
    
    # Input processing
    processor = graphbit.PyWorkflowNode.agent_node(
        "Processor", "Processes input", "processor", "Process and rate quality: {input}"
    )
    
    # Quality gate
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        "Quality Gate", "Checks quality", "quality_score >= 0.7"
    )
    
    # High quality path
    approver = graphbit.PyWorkflowNode.agent_node(
        "Approver", "Approves content", "approver", "Approve: {content}"
    )
    
    # Low quality path
    improver = graphbit.PyWorkflowNode.agent_node(
        "Improver", "Improves content", "improver", "Improve: {content}"
    )
    
    # Build workflow
    proc_id = builder.add_node(processor)
    gate_id = builder.add_node(quality_gate)
    approve_id = builder.add_node(approver)
    improve_id = builder.add_node(improver)
    
    # Connect with conditions
    builder.connect(proc_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(gate_id, approve_id, graphbit.PyWorkflowEdge.conditional("quality_score >= 0.7"))
    builder.connect(gate_id, improve_id, graphbit.PyWorkflowEdge.conditional("quality_score < 0.7"))
    
    return builder.build()
```

---

## 🔗 Related Documentation

- **[Getting Started Guide](01-getting-started-workflows.md)** - Build your first workflow
- **[Complete API Reference](05-complete-api-reference.md)** - Detailed API documentation
- **[Use Case Examples](02-use-case-examples.md)** - Real-world workflow examples
- **[Embeddings Guide](07-embeddings-guide.md)** - Semantic search and RAG
- **[HuggingFace Integration](08-huggingface-integration.md)** - Open-source models
