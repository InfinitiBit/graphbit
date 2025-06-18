# Core Concepts

Understanding GraphBit's fundamental concepts will help you build powerful AI agent workflows. This guide covers the key components and how they work together.

## Overview

GraphBit is built around three core concepts:

1. **Workflows** - Directed graphs that define the execution flow
2. **Nodes** - Individual processing units (agents, conditions, transforms)
3. **Agents** - AI-powered components that interact with LLM providers

## Workflows

A **Workflow** is a directed acyclic graph (DAG) that defines how data flows through your AI pipeline.

### Workflow Structure

```python
builder = graphbit.PyWorkflowBuilder("My Workflow")
builder.description("A sample workflow for processing data")
workflow = builder.build()
```

### Key Properties

- **Name**: Human-readable identifier
- **Description**: Purpose and functionality description
- **Nodes**: Processing units within the workflow
- **Edges**: Connections that define data/control flow
- **Context**: Shared state and variables during execution

### Workflow States

| State | Description |
|-------|-------------|
| `Pending` | Workflow created but not started |
| `Running` | Currently executing |
| `Completed` | Successfully finished |
| `Failed` | Encountered an error |
| `Cancelled` | Manually stopped |

```python
# Check workflow state
result = executor.execute(workflow)
print(f"Workflow state: {result.state()}")
print(f"Is completed: {result.is_completed()}")
print(f"Is failed: {result.is_failed()}")
```

## Nodes

**Nodes** are the building blocks of workflows. Each node performs a specific function and can pass data to connected nodes.

### Node Types

#### 1. Agent Nodes
Execute AI tasks using LLM providers:

```python
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Data Analyzer",
    description="Analyzes input data for patterns",
    agent_id="analyzer",
    prompt="Analyze the following data and identify key patterns: {input}"
)
```

#### 2. Condition Nodes
Make decisions based on data:

```python
quality_check = graphbit.PyWorkflowNode.condition_node(
    name="Quality Gate",
    description="Checks if quality meets threshold",
    expression="quality_score > 0.8"
)
```

#### 3. Transform Nodes
Process and transform data:

```python
# Available transformations: uppercase, lowercase, json_extract, split, join
formatter = graphbit.PyWorkflowNode.transform_node(
    name="Format Output",
    description="Converts to uppercase",
    transformation="uppercase"
)
```

#### 4. Delay Nodes
Add timing controls:

```python
rate_limit = graphbit.PyWorkflowNode.delay_node(
    name="Rate Limiter",
    description="Prevents API rate limiting",
    duration_seconds=5
)
```

#### 5. Document Loader Nodes
Load and process documents:

```python
loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Loader",
    description="Loads PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)
```

### Node Properties

Every node has these properties:

```python
# Access node properties
print(f"Node ID: {node.id()}")
print(f"Name: {node.name()}")
print(f"Description: {node.description()}")
```

## Agents

**Agents** are AI-powered components that execute tasks using Large Language Models.

### Agent Configuration

```python
# Basic agent
agent = graphbit.PyWorkflowNode.agent_node(
    name="Content Creator",
    description="Creates marketing content",
    agent_id="creator",
    prompt="Create engaging content about: {topic}"
)

# Agent with advanced configuration
agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Technical Writer",
    description="Writes technical documentation", 
    agent_id="tech_writer",
    prompt="Write technical docs for: {feature}",
    max_tokens=2000,
    temperature=0.3
)
```

### Agent Capabilities

GraphBit supports different agent capabilities:

```python
# Text processing
text_processor = graphbit.PyAgentCapability.text_processing()

# Data analysis
data_analyzer = graphbit.PyAgentCapability.data_analysis()

# Tool execution
tool_executor = graphbit.PyAgentCapability.tool_execution()

# Decision making
decision_maker = graphbit.PyAgentCapability.decision_making()

# Custom capability
custom_agent = graphbit.PyAgentCapability.custom("domain_expert")
```

### Prompt Templates

Agents use prompt templates with variable substitution:

```python
prompt = """
Task: Analyze the following {data_type}
Context: {context}
Requirements: 
- Provide {num_insights} key insights
- Focus on {focus_area}
- Format as {output_format}

Data: {input}
"""

agent = graphbit.PyWorkflowNode.agent_node(
    name="Flexible Analyzer",
    description="Analyzes various data types",
    agent_id="flex_analyzer",
    prompt=prompt
)
```

## Workflow Edges

**Edges** define how data flows between nodes in your workflow.

### Edge Types

#### 1. Data Flow
Passes data from one node to another:

```python
data_edge = graphbit.PyWorkflowEdge.data_flow()
builder.connect(source_id, target_id, data_edge)
```

#### 2. Control Flow
Controls execution order without data transfer:

```python
control_edge = graphbit.PyWorkflowEdge.control_flow()
builder.connect(first_id, second_id, control_edge)
```

#### 3. Conditional Flow
Executes based on conditions:

```python
conditional_edge = graphbit.PyWorkflowEdge.conditional("score > 0.5")
builder.connect(condition_id, success_id, conditional_edge)
```

## Workflow Context

The **Workflow Context** maintains state and variables throughout execution.

### Working with Context

```python
# Access workflow information
context = executor.execute(workflow)

print(f"Workflow ID: {context.workflow_id()}")
print(f"Execution time: {context.execution_time_ms()}ms")

# Get variables
output = context.get_variable("output")
all_vars = context.variables()  # Returns list of (key, value) tuples

# Check status
if context.is_completed():
    print("Workflow completed successfully!")
elif context.is_failed():
    print("Workflow failed!")
```

### Variable Flow

Variables automatically flow between connected nodes:

```python
# Node 1 output becomes Node 2 input
builder = graphbit.PyWorkflowBuilder("Variable Flow Example")

# First node creates 'analysis' variable
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Analyzer",
    description="Analyzes input",
    agent_id="analyzer", 
    prompt="Analyze: {input}"
)

# Second node uses 'analysis' from first node
summarizer = graphbit.PyWorkflowNode.agent_node(
    name="Summarizer", 
    description="Summarizes analysis",
    agent_id="summarizer",
    prompt="Summarize this analysis: {analysis}"
)

# Connect nodes - analysis flows automatically
id1 = builder.add_node(analyzer)
id2 = builder.add_node(summarizer)
builder.connect(id1, id2, graphbit.PyWorkflowEdge.data_flow())
```

## Execution Patterns

GraphBit supports different execution patterns for various use cases.

### Sequential Execution

Default execution pattern - nodes execute in dependency order:

```python
executor = graphbit.PyWorkflowExecutor(config)
result = executor.execute(workflow)
```

### Concurrent Execution

Execute multiple workflows simultaneously:

```python
workflows = [workflow1, workflow2, workflow3]
results = executor.execute_concurrent(workflows)

for i, result in enumerate(results):
    print(f"Workflow {i+1}: {result.execution_time_ms()}ms")
```

### Batch Execution

Process multiple workflows efficiently:

```python
workflows = [workflow1, workflow2, workflow3, workflow4]
results = executor.execute_batch(workflows)
```

### Asynchronous Execution

Non-blocking execution with Python async/await:

```python
import asyncio

async def async_workflow():
    result = await executor.execute_async(workflow)
    return result

# Run async workflow
result = asyncio.run(async_workflow())
```

## Error Handling

GraphBit provides comprehensive error handling mechanisms.

### Workflow Validation

Validate workflows before execution:

```python
try:
    workflow.validate()
    print("✅ Workflow is valid")
except Exception as e:
    print(f"❌ Validation failed: {e}")
```

### Execution Errors

Handle runtime errors gracefully:

```python
try:
    result = executor.execute(workflow)
    if result.is_failed():
        print(f"Workflow failed: {result.get_variable('error')}")
except Exception as e:
    print(f"Execution error: {e}")
```

## Best Practices

### 1. Node Naming
Use descriptive names for nodes:

```python
# Good
content_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="SEO Content Analyzer",
    description="Analyzes content for SEO optimization",
    agent_id="seo_analyzer",
    prompt="Analyze content for SEO: {content}"
)

# Avoid
node1 = graphbit.PyWorkflowNode.agent_node(
    name="Node1",
    description="Does stuff",
    agent_id="n1", 
    prompt="Do something: {input}"
)
```

### 2. Prompt Design
Create clear, specific prompts:

```python
# Good - specific and structured
prompt = """
Analyze the following customer feedback for sentiment and key themes.

Customer Feedback: {feedback}

Please provide:
1. Overall sentiment (positive/negative/neutral)
2. Key themes mentioned
3. Specific issues or compliments
4. Recommended actions

Format your response as JSON with these fields:
- sentiment
- themes
- details  
- recommendations
"""

# Avoid - vague and unstructured
prompt = "Look at this feedback: {feedback}"
```

### 3. Error Recovery
Build resilient workflows:

```python
# Use retry configuration
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(
        graphbit.PyRetryConfig.default()
        .with_exponential_backoff(1000, 2.0, 30000)
    ) \
    .with_circuit_breaker_config(
        graphbit.PyCircuitBreakerConfig.default()
    )
```

### 4. Resource Management
Optimize for your use case:

```python
# High throughput
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)

# Low latency  
executor = graphbit.PyWorkflowExecutor.new_low_latency(config)

# Memory optimized
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)
```

## Next Steps

Now that you understand GraphBit's core concepts, explore:

- [Workflow Builder](workflow-builder.md) - Build complex workflows
- [Agent Configuration](agents.md) - Configure AI agents
- [LLM Providers](llm-providers.md) - Work with different AI providers
- [API Reference](../api-reference/python-api.md) - Complete API documentation 
