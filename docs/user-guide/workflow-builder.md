# Workflow Builder

The Workflow Builder in GraphBit provides a simple, direct approach to creating AI agent workflows. Unlike complex builder patterns, GraphBit uses straightforward workflow and node creation methods.

## Overview

GraphBit workflows are built using:
- **Workflow** - Container for nodes and connections
- **Node** - Individual processing units with different types
- **Connections** - Links between nodes that define data flow

## Basic Usage

### Creating a Workflow

```python
from graphbit import init, Workflow, Node, Executor, LlmConfig

# Initialize GraphBit
init()

# Create a new workflow
workflow = Workflow("My AI Pipeline")
```

### Creating Nodes

GraphBit supports several node types:

#### Agent Nodes
Execute AI tasks using LLM providers:

```python
from graphbit import Node
# Basic agent node
analyzer = Node.agent(
    name="Data Analyzer",
    prompt=f"Analyze this data for patterns: {input}",
    agent_id="analyzer_001"  # Optional - auto-generated if not provided
)

# Agent with explicit ID
summarizer = Node.agent(
    name="Content Summarizer", 
    prompt="Summarize the following analyzed content",
    agent_id="summarizer"
)

# Text transformation
formatter = Node.agent(
        name="Output Formatter",
        prompt="Transform the provided text to uppercase",
        agent_id="formatter"
    )

# Other types of transformation
lowercase_node = Node.agent(
        name="Output Formatter",
        prompt="Transform the provided text to lowercase",
        agent_id="formatter"
    )
```


### Building the Workflow

```python
# Add nodes to workflow and get their IDs
analyzer_id = workflow.add_node(analyzer)
formatter_id = workflow.add_node(formatter)
summarizer_id = workflow.add_node(summarizer)

# Connect nodes to define data flow
workflow.connect(analyzer_id, formatter_id)
workflow.connect(formatter_id, summarizer_id)

# Validate workflow structure
workflow.validate()
```

## Workflow Patterns

### Sequential Processing

Create linear processing pipelines:

```python
# Create workflow
workflow = Workflow("Sequential Pipeline")

# Create processing steps
step1 = Node.agent(
    name="Input Processor",
    prompt=f"Process the initial input: {input}",
    agent_id="step1"
)

step2 = Node.agent(
    name="Data Enricher", 
    prompt="Enrich the processed data",
    agent_id="step2"
)

step3 = Node.agent(
    name="Final Formatter",
    prompt="Format the final output",
    agent_id="step3"
)

# Add nodes and connect sequentially
id1 = workflow.add_node(step1)
id2 = workflow.add_node(step2)
id3 = workflow.add_node(step3)

workflow.connect(id1, id2)
workflow.connect(id2, id3)

workflow.validate()
```

### Parallel Processing Branches

Create workflows with multiple parallel processing paths:

```python
# Create workflow
workflow = Workflow("Parallel Analysis")

# Input node
input_processor = Node.agent(
    name="Input Processor",
    prompt=f"Prepare data for analysis: {input}",
    agent_id="input_proc"
)

# Parallel analysis branches
sentiment_analyzer = Node.agent(
    name="Sentiment Analyzer",
    prompt="Analyze sentiment of processed data",
    agent_id="sentiment"
)

topic_analyzer = Node.agent(
    name="Topic Analyzer", 
    prompt="Extract key topics from processed data",
    agent_id="topics"
)

quality_analyzer = Node.agent(
    name="Quality Analyzer",
    prompt="Assess content quality of processed data",
    agent_id="quality"
)

# Result aggregator
aggregator = Node.agent(
    name="Result Aggregator",
    prompt=("Combine analysis results from: Sentiment\nTopics\nQuality"),
    agent_id="aggregator"
)

# Build parallel structure
input_id = workflow.add_node(input_processor)
sentiment_id = workflow.add_node(sentiment_analyzer)
topic_id = workflow.add_node(topic_analyzer)
quality_id = workflow.add_node(quality_analyzer)
agg_id = workflow.add_node(aggregator)

# Connect input to all analyzers
workflow.connect(input_id, sentiment_id)
workflow.connect(input_id, topic_id)
workflow.connect(input_id, quality_id)

# Connect all analyzers to aggregator
workflow.connect(sentiment_id, agg_id)
workflow.connect(topic_id, agg_id)
workflow.connect(quality_id, agg_id)

workflow.validate()
```

## Node Properties and Management

### Accessing Node Information

```python
# Create a node
analyzer = Node.agent(
    name="Data Analyzer",
    prompt=f"Analyze: {input}",
    agent_id="analyzer"
)

# Access node properties
print(f"Node ID: {analyzer.id()}")
print(f"Node Name: {analyzer.name()}")

# Add to workflow
workflow = Workflow("Test Workflow")
node_id = workflow.add_node(analyzer)
print(f"Workflow Node ID: {node_id}")
```

### Node Naming Best Practices

Use descriptive, clear names for nodes:

```python
# Good - descriptive and clear
email_analyzer = Node.agent(
    name="Email Content Analyzer",
    prompt=f"Analyze email for spam indicators: {email_content}",
    agent_id="email_spam_detector"
)

# Avoid - vague names
node1 = Node.agent(
    name="Node1", 
    prompt=f"Do something: {input}",
    agent_id="n1"
)
```

## Workflow Execution

### Setting up Execution

```python
from graphbit import LlmConfig, Executor
import os
# Create LLM configuration
llm_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)

# Create executor
executor = Executor(
    config=llm_config,
    timeout_seconds=300,
    debug=False
)

# Execute workflow
result = executor.execute(workflow)

# Check results
if result.is_completed():
    print("Success:", result.get_all_nodes_outputs())
elif result.is_failed():
    print("Failed:", result.error())
```

### Asynchronous Execution

```python
import asyncio

async def run_workflow():
    # Create workflow and executor as above
    result = await executor.run_async(workflow)
    return result

# Run asynchronously
result = asyncio.run(run_workflow())
```

## Advanced Patterns

### Multi-Stage Processing

```python
from graphbit import Node

def create_multi_stage_workflow():
    workflow = Workflow("Multi-Stage Processing")
    
    # Stage 1: Data Preparation
    cleaner = Node.agent(
        name="Data Cleaner",
        prompt=f"Normalize and clean input text: {input}",
        agent_id="cleaner"
    )
    
    # Stage 2: Analysis
    analyzer = Node.agent(
        name="Content Analyzer",
        prompt="Analyze cleaned content",
        agent_id="analyzer"
    )
    
    # Stage 3: Output Processing
    
    formatter = Node.agent(
        name="Output Formatter",
        prompt="Transform the provided text to uppercase",
        agent_id="formatter"
    )
    finalizer = Node.agent(
        name="Output Finalizer",
        prompt="Finalize the analysis",
        agent_id="finalizer"
    )
    
    # Build multi-stage pipeline
    clean_id = workflow.add_node(cleaner)
    analyze_id = workflow.add_node(analyzer)
    format_id = workflow.add_node(formatter)
    final_id = workflow.add_node(finalizer)
    
    # Connect stages
    workflow.connect(clean_id, analyze_id)
    workflow.connect(analyze_id, format_id)
    workflow.connect(format_id, final_id)
    
    workflow.validate()
    return workflow
```

### Error Handling in Workflows

```python
def create_robust_workflow():
    workflow = Workflow("Robust Processing")
    
    try:
        # Add nodes
        processor = Node.agent(
            name="Data Processor",
            prompt=f"Process: {input}",
            agent_id="processor"
        )
        
        node_id = workflow.add_node(processor)
        # Validate before execution
        workflow.validate()        
        return workflow
        
    except Exception as e:
        print(f"Workflow creation failed: {e}")
        return None
```

## Best Practices

### 1. Workflow Organization

```python
# Organize complex workflows into functions
def create_analysis_workflow():
    workflow = Workflow("Content Analysis")    
    # Input processing
    input_node = create_input_processor()    
    # Analysis stages  
    sentiment_node = create_sentiment_analyzer()
    topic_node = create_topic_analyzer()    
    # Output processing
    output_node = create_output_formatter()
    
    # Build workflow
    input_id = workflow.add_node(input_node)
    sentiment_id = workflow.add_node(sentiment_node)
    topic_id = workflow.add_node(topic_node)
    output_id = workflow.add_node(output_node)    
    # Connect nodes
    workflow.connect(input_id, sentiment_id)
    workflow.connect(input_id, topic_id)
    workflow.connect(sentiment_id, output_id)
    workflow.connect(topic_id, output_id)    
    workflow.validate()
    return workflow

def create_input_processor():
    return Node.agent(
        name="Input Processor",
        prompt=f"Prepare input for analysis: {input}",
        agent_id="input_processor"
    )

def create_sentiment_analyzer():
    return Node.agent(
        name="Sentiment Analyzer",
        prompt="Analyze sentiment of processed data",
        agent_id="sentiment"
    )

def create_topic_analyzer():
    return Node.agent(
        name="Topic Analyzer", 
        prompt="Extract key topics from processed data",
        agent_id="topics"
    )

def create_output_formatter():
    return Node.agent(
        name="Output Formatter",
        prompt="Transform the provided text to uppercase",
        agent_id="formatter"
    )
```

### 2. Validation and Testing

```python
def test_workflow(workflow):
    """Test workflow structure before execution"""
    try:
        workflow.validate()
        print("✅ Workflow validation passed")
        return True
    except Exception as e:
        print(f"❌ Workflow validation failed: {e}")
        return False

# Always validate before execution
workflow = create_analysis_workflow()
if test_workflow(workflow):
    result = executor.execute(workflow)
```

### 3. Resource Management

```python
# Choose appropriate executor for your use case
def get_executor_for_workload(workload_type, llm_config):
    if workload_type == "batch":
        return Executor(
            config=llm_config,
            timeout_seconds=600
        )
    elif workload_type == "realtime":
        return Executor(
            config=llm_config,
            lightweight_mode=true
            timeout_seconds=30
        )
    elif workload_type == "constrained":
        return Executor(
            config=llm_config,
            timeout_seconds=300
        )
    else:
        return Executor(
            config=llm_config,
            timeout_seconds=300
        )
```

## Common Patterns Summary

| Pattern | Use Case | Example |
|---------|----------|---------|
| Sequential | Linear processing | Data cleaning → Analysis → Output |
| Parallel | Independent analysis | Multiple analyzers running simultaneously | 
| Multi-stage | Complex pipelines | Preparation → Analysis → Finalization |

## What's Next

- Learn about [LLM Providers](llm-providers.md) for configuring different AI models
- Explore [Agents](agents.md) for advanced agent configuration
- Check [Performance](performance.md) for execution optimization
- See [Validation](validation.md) for workflow validation strategies
