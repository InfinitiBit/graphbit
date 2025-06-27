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
import graphbit

# Initialize GraphBit
graphbit.init()

# Create a new workflow
workflow = graphbit.Workflow("My AI Pipeline")
```

### Creating Nodes

GraphBit supports several node types:

#### Agent Nodes
Execute AI tasks using LLM providers:

```python
# Basic agent node
analyzer = graphbit.Node.agent(
    name="Data Analyzer",
    prompt="Analyze this data for patterns: {input}",
    agent_id="analyzer_001"  # Optional - auto-generated if not provided
)

# Agent with explicit ID
summarizer = graphbit.Node.agent(
    name="Content Summarizer", 
    prompt="Summarize the following content: {analysis}",
    agent_id="summarizer"
)
```

#### Transform Nodes
Process and modify data:

```python
# Text transformation
formatter = graphbit.Node.transform(
    name="Text Formatter",
    transformation="uppercase"
)

# Other available transformations
lowercase_node = graphbit.Node.transform("Lowercase", "lowercase")
```

#### Condition Nodes
Make decisions based on data evaluation:

```python
# Quality check
quality_gate = graphbit.Node.condition(
    name="Quality Gate",
    expression="quality_score > 0.8 and confidence > 0.7"
)

# Simple condition
threshold_check = graphbit.Node.condition(
    name="Threshold Check", 
    expression="value > 100"
)
```

### Building the Workflow

```python
# Add nodes to workflow and get their IDs
analyzer_id = workflow.add_node(analyzer)
formatter_id = workflow.add_node(formatter)
quality_id = workflow.add_node(quality_gate)
summarizer_id = workflow.add_node(summarizer)

# Connect nodes to define data flow
workflow.connect(analyzer_id, formatter_id)
workflow.connect(formatter_id, quality_id)
workflow.connect(quality_id, summarizer_id)

# Validate workflow structure
workflow.validate()
```

## Workflow Patterns

### Sequential Processing

Create linear processing pipelines:

```python
# Create workflow
workflow = graphbit.Workflow("Sequential Pipeline")

# Create processing steps
step1 = graphbit.Node.agent(
    name="Input Processor",
    prompt="Process the initial input: {input}",
    agent_id="step1"
)

step2 = graphbit.Node.agent(
    name="Data Enricher", 
    prompt="Enrich the processed data: {step1_output}",
    agent_id="step2"
)

step3 = graphbit.Node.agent(
    name="Final Formatter",
    prompt="Format the final output: {step2_output}",
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
workflow = graphbit.Workflow("Parallel Analysis")

# Input node
input_processor = graphbit.Node.agent(
    name="Input Processor",
    prompt="Prepare data for analysis: {input}",
    agent_id="input_proc"
)

# Parallel analysis branches
sentiment_analyzer = graphbit.Node.agent(
    name="Sentiment Analyzer",
    prompt="Analyze sentiment of: {processed_data}",
    agent_id="sentiment"
)

topic_analyzer = graphbit.Node.agent(
    name="Topic Analyzer", 
    prompt="Extract key topics from: {processed_data}",
    agent_id="topics"
)

quality_analyzer = graphbit.Node.agent(
    name="Quality Analyzer",
    prompt="Assess content quality of: {processed_data}",
    agent_id="quality"
)

# Result aggregator
aggregator = graphbit.Node.agent(
    name="Result Aggregator",
    prompt="Combine analysis results:\nSentiment: {sentiment_output}\nTopics: {topics_output}\nQuality: {quality_output}",
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

### Conditional Workflows

Use condition nodes for dynamic routing:

```python
# Create workflow
workflow = graphbit.Workflow("Conditional Processing")

# Content analyzer
analyzer = graphbit.Node.agent(
    name="Content Analyzer",
    prompt="Analyze content quality (score 1-10): {input}",
    agent_id="analyzer"
)

# Quality gate condition
quality_gate = graphbit.Node.condition(
    name="Quality Gate",
    expression="quality_score >= 7"
)

# High quality path
approver = graphbit.Node.agent(
    name="Content Approver",
    prompt="Approve high-quality content: {analyzed_content}",
    agent_id="approver"
)

# Low quality path  
improver = graphbit.Node.agent(
    name="Content Improver",
    prompt="Suggest improvements for: {analyzed_content}",
    agent_id="improver"
)

# Build conditional flow
analyzer_id = workflow.add_node(analyzer)
gate_id = workflow.add_node(quality_gate)
approve_id = workflow.add_node(approver)
improve_id = workflow.add_node(improver)

# Connect analyzer to quality gate
workflow.connect(analyzer_id, gate_id)

# Connect based on conditions (simplified - actual conditional routing 
# is handled by the executor based on the condition node evaluation)
workflow.connect(gate_id, approve_id)
workflow.connect(gate_id, improve_id)

workflow.validate()
```

### Data Transformation Pipelines

Combine transform nodes with AI agents:

```python
# Create workflow
workflow = graphbit.Workflow("Data Transformation Pipeline")

# Text preprocessor
preprocessor = graphbit.Node.transform(
    name="Text Preprocessor",
    transformation="lowercase"
)

# Content processor
processor = graphbit.Node.agent(
    name="Content Processor", 
    prompt="Process the cleaned text: {preprocessed_text}",
    agent_id="processor"
)

# Output formatter
formatter = graphbit.Node.transform(
    name="Output Formatter",
    transformation="uppercase"
)

# Final quality check
validator = graphbit.Node.condition(
    name="Output Validator",
    expression="length > 10"
)

# Build transformation pipeline
prep_id = workflow.add_node(preprocessor)
proc_id = workflow.add_node(processor)
format_id = workflow.add_node(formatter)
valid_id = workflow.add_node(validator)

workflow.connect(prep_id, proc_id)
workflow.connect(proc_id, format_id)
workflow.connect(format_id, valid_id)

workflow.validate()
```

## Node Properties and Management

### Accessing Node Information

```python
# Create a node
analyzer = graphbit.Node.agent(
    name="Data Analyzer",
    prompt="Analyze: {input}",
    agent_id="analyzer"
)

# Access node properties
print(f"Node ID: {analyzer.id()}")
print(f"Node Name: {analyzer.name()}")

# Add to workflow
workflow = graphbit.Workflow("Test Workflow")
node_id = workflow.add_node(analyzer)
print(f"Workflow Node ID: {node_id}")
```

### Node Naming Best Practices

Use descriptive, clear names for nodes:

```python
# Good - descriptive and clear
email_analyzer = graphbit.Node.agent(
    name="Email Content Analyzer",
    prompt="Analyze email for spam indicators: {email_content}",
    agent_id="email_spam_detector"
)

# Good - indicates purpose
quality_gate = graphbit.Node.condition(
    name="Content Quality Gate",
    expression="spam_score < 0.3"
)

# Avoid - vague names
node1 = graphbit.Node.agent(
    name="Node1", 
    prompt="Do something: {input}",
    agent_id="n1"
)
```

## Workflow Execution

### Setting up Execution

```python
# Create LLM configuration
llm_config = graphbit.LlmConfig.openai(
    api_key="your-openai-key",
    model="gpt-4o-mini"
)

# Create executor
executor = graphbit.Executor(
    config=llm_config,
    timeout_seconds=300,
    debug=False
)

# Execute workflow
result = executor.execute(workflow)

# Check results
if result.is_completed():
    print("Success:", result.output())
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
def create_multi_stage_workflow():
    workflow = graphbit.Workflow("Multi-Stage Processing")
    
    # Stage 1: Data Preparation
    cleaner = graphbit.Node.transform("Data Cleaner", "lowercase")
    validator = graphbit.Node.condition("Data Validator", "length > 5")
    
    # Stage 2: Analysis
    analyzer = graphbit.Node.agent(
        name="Content Analyzer",
        prompt="Analyze cleaned content: {cleaned_data}",
        agent_id="analyzer"
    )
    
    # Stage 3: Output Processing
    formatter = graphbit.Node.transform("Output Formatter", "uppercase")
    finalizer = graphbit.Node.agent(
        name="Output Finalizer",
        prompt="Finalize the analysis: {formatted_output}",
        agent_id="finalizer"
    )
    
    # Build multi-stage pipeline
    clean_id = workflow.add_node(cleaner)
    valid_id = workflow.add_node(validator)
    analyze_id = workflow.add_node(analyzer)
    format_id = workflow.add_node(formatter)
    final_id = workflow.add_node(finalizer)
    
    # Connect stages
    workflow.connect(clean_id, valid_id)
    workflow.connect(valid_id, analyze_id)
    workflow.connect(analyze_id, format_id)
    workflow.connect(format_id, final_id)
    
    workflow.validate()
    return workflow
```

### Error Handling in Workflows

```python
def create_robust_workflow():
    workflow = graphbit.Workflow("Robust Processing")
    
    try:
        # Add nodes
        processor = graphbit.Node.agent(
            name="Data Processor",
            prompt="Process: {input}",
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
    workflow = graphbit.Workflow("Content Analysis")    
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
    return graphbit.Node.agent(
        name="Input Processor",
        prompt="Prepare input for analysis: {input}",
        agent_id="input_processor"
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
        return graphbit.Executor.new_high_throughput(
            llm_config=llm_config,
            timeout_seconds=600
        )
    elif workload_type == "realtime":
        return graphbit.Executor.new_low_latency(
            llm_config=llm_config,
            timeout_seconds=30
        )
    elif workload_type == "constrained":
        return graphbit.Executor.new_memory_optimized(
            llm_config=llm_config,
            timeout_seconds=300
        )
    else:
        return graphbit.Executor(
            config=llm_config,
            timeout_seconds=300
        )
```

## Common Patterns Summary

| Pattern | Use Case | Example |
|---------|----------|---------|
| Sequential | Linear processing | Data cleaning → Analysis → Output |
| Parallel | Independent analysis | Multiple analyzers running simultaneously |  
| Conditional | Dynamic routing | Quality gate directing to approval/rejection |
| Transform | Data preprocessing | Text cleaning and formatting |
| Multi-stage | Complex pipelines | Preparation → Analysis → Finalization |

## What's Next

- Learn about [LLM Providers](llm-providers.md) for configuring different AI models
- Explore [Agents](agents.md) for advanced agent configuration
- Check [Performance](performance.md) for execution optimization
- See [Validation](validation.md) for workflow validation strategies
