# Quick Start Tutorial

Welcome to GraphBit! This tutorial will guide you through creating your first AI agent workflow in just 5 minutes.

## Prerequisites

Before starting, ensure you have:
- GraphBit installed ([Installation Guide](installation.md))
- An OpenAI API key set in your environment
- Python 3.10+ available

## Your First Workflow

Let's create a simple content analysis workflow that analyzes text and provides insights.

### Step 1: Basic Setup

Create a new Python file `my_first_workflow.py`:

```python
import graphbit
import os

# Initialize GraphBit
graphbit.init()

# Configure LLM (using OpenAI GPT-4)
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise ValueError("Please set OPENAI_API_KEY environment variable")

config = graphbit.PyLlmConfig.openai(api_key, "gpt-4o-mini")
```

### Step 2: Create Your First Agent Node

```python
# Create a workflow builder
builder = graphbit.PyWorkflowBuilder("Content Analysis Pipeline")
builder.description("Analyzes content and provides insights")

# Create an analyzer agent
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Content Analyzer",
    description="Analyzes text content for insights",
    agent_id="analyzer",
    prompt="Analyze the following content and provide key insights: {input}"
)

# Add the node to the workflow
analyzer_id = builder.add_node(analyzer)
```

### Step 3: Build and Execute the Workflow

```python
# Build the workflow
workflow = builder.build()

# Create executor with basic configuration
executor = graphbit.PyWorkflowExecutor(config)

# Execute the workflow with sample input
print("🚀 Executing workflow...")
result = executor.execute(workflow)

# Display results
print(f"✅ Workflow completed in {result.execution_time_ms()}ms")
print(f"📊 Result: {result.get_variable('output')}")
```

### Complete Example

Here's the complete working example:

```python
import graphbit
import os

def main():
    # Initialize GraphBit
    graphbit.init()
    
    # Configure LLM
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("❌ Please set OPENAI_API_KEY environment variable")
        return
    
    config = graphbit.PyLlmConfig.openai(api_key, "gpt-4o-mini")
    
    # Build workflow
    builder = graphbit.PyWorkflowBuilder("Content Analysis Pipeline")
    
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Content Analyzer",
        description="Analyzes text content",
        agent_id="analyzer",
        prompt="Analyze this content and provide 3 key insights: {input}"
    )
    
    analyzer_id = builder.add_node(analyzer)
    workflow = builder.build()
    
    # Execute workflow
    executor = graphbit.PyWorkflowExecutor(config)
    
    print("🚀 Analyzing content...")
    result = executor.execute(workflow)
    
    print(f"✅ Analysis completed in {result.execution_time_ms()}ms")
    print(f"📊 Insights:\n{result.get_variable('output')}")

if __name__ == "__main__":
    main()
```

### Run Your Workflow

```bash
export OPENAI_API_KEY="your-api-key-here"
python my_first_workflow.py
```

Expected output:
```
🚀 Analyzing content...
✅ Analysis completed in 2347ms
📊 Insights:
Based on the analysis, here are 3 key insights:
1. [AI-generated insight about the content]
2. [Another relevant insight]
3. [Additional analysis point]
```

## Multi-Step Workflows

Let's create a more complex workflow with multiple connected nodes:

```python
import graphbit
import os

def create_content_pipeline():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Build multi-step workflow
    builder = graphbit.PyWorkflowBuilder("Content Creation Pipeline")
    builder.description("Research → Write → Edit workflow")
    
    # Step 1: Research Agent
    researcher = graphbit.PyWorkflowNode.agent_node(
        name="Researcher",
        description="Researches the topic",
        agent_id="researcher",
        prompt="Research key points about: {topic}. Provide 5 important facts."
    )
    
    # Step 2: Writer Agent  
    writer = graphbit.PyWorkflowNode.agent_node(
        name="Content Writer",
        description="Writes content based on research",
        agent_id="writer",
        prompt="Write a 200-word article about {topic} using this research: {research_data}"
    )
    
    # Step 3: Editor Agent
    editor = graphbit.PyWorkflowNode.agent_node(
        name="Editor",
        description="Edits and improves content",
        agent_id="editor", 
        prompt="Edit and improve this article for clarity and engagement: {draft_content}"
    )
    
    # Add nodes and create connections
    research_id = builder.add_node(researcher)
    writer_id = builder.add_node(writer)
    editor_id = builder.add_node(editor)
    
    # Connect the workflow: Research → Write → Edit
    builder.connect(research_id, writer_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(writer_id, editor_id, graphbit.PyWorkflowEdge.data_flow())
    
    # Build and execute
    workflow = builder.build()
    executor = graphbit.PyWorkflowExecutor(config)
    
    print("🚀 Executing content creation pipeline...")
    result = executor.execute(workflow)
    
    print(f"✅ Pipeline completed in {result.execution_time_ms()}ms")
    print(f"📄 Final article:\n{result.get_variable('output')}")

if __name__ == "__main__":
    create_content_pipeline()
```

## Adding Reliability Features

Enhance your workflow with error handling and retries:

```python
import graphbit
import os

def reliable_workflow():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create executor with reliability features
    executor = graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(graphbit.PyRetryConfig.default()) \
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default())
    
    # Build workflow
    builder = graphbit.PyWorkflowBuilder("Reliable Analysis")
    
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Robust Analyzer",
        description="Analyzes content with error handling",
        agent_id="robust_analyzer",
        prompt="Provide detailed analysis of: {input}"
    )
    
    analyzer_id = builder.add_node(analyzer)
    workflow = builder.build()
    
    # Execute with error handling
    try:
        print("🛡️ Executing reliable workflow...")
        result = executor.execute(workflow)
        print(f"✅ Success! Completed in {result.execution_time_ms()}ms")
        print(f"📊 Result: {result.get_variable('output')}")
        
    except Exception as e:
        print(f"❌ Workflow failed: {e}")

if __name__ == "__main__":
    reliable_workflow()
```

## Working with Different LLM Providers

GraphBit supports multiple LLM providers:

```python
import graphbit
import os

# OpenAI Configuration
openai_config = graphbit.PyLlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), 
    "gpt-4o-mini"
)

# Anthropic Configuration
anthropic_config = graphbit.PyLlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-3-haiku-20240307"
)

# HuggingFace Configuration
hf_config = graphbit.PyLlmConfig.huggingface(
    os.getenv("HUGGINGFACE_API_KEY"),
    "microsoft/DialoGPT-medium"
)

# Use any configuration with the same workflow
def run_with_provider(config):
    builder = graphbit.PyWorkflowBuilder("Multi-Provider Test")
    
    agent = graphbit.PyWorkflowNode.agent_node(
        name="Test Agent",
        description="Tests different providers",
        agent_id="test_agent",
        prompt="Say hello and identify yourself: {input}"
    )
    
    builder.add_node(agent)
    workflow = builder.build()
    
    executor = graphbit.PyWorkflowExecutor(config)
    result = executor.execute(workflow)
    
    print(f"Provider: {config.provider_name()}")
    print(f"Response: {result.get_variable('output')}")
    print("---")

# Test different providers
if __name__ == "__main__":
    graphbit.init()
    
    if os.getenv("OPENAI_API_KEY"):
        run_with_provider(openai_config)
    
    if os.getenv("ANTHROPIC_API_KEY"):
        run_with_provider(anthropic_config)
```

## Next Steps

Congratulations! You've created your first GraphBit workflows. Here's what to explore next:

### 📚 Learn More
- [Core Concepts](../user-guide/concepts.md) - Understand workflows, agents, and nodes
- [Node Types](../api-reference/node-types.md) - Explore all available node types
- [LLM Providers](../user-guide/llm-providers.md) - Configure different AI providers

### 🎯 Advanced Features
- [Dynamic Graph Generation](../advanced/dynamic-graphs.md) - Auto-generate workflow nodes
- [Embeddings & Vector Search](../user-guide/embeddings.md) - Add semantic search
- [Performance Optimization](../advanced/performance.md) - Tune for production

### 📋 Examples
- [Content Generation Pipeline](../examples/content-generation.md) - Complete content creation workflow
- [Data Processing Workflow](../examples/data-processing.md) - ETL with AI agents
- [Code Review Automation](../examples/code-review.md) - Automated code analysis

### 🛠️ Production Ready
- [Error Handling & Reliability](../user-guide/reliability.md) - Production-grade error handling
- [Monitoring & Observability](../advanced/monitoring.md) - Track workflow performance
- [Configuration Options](../api-reference/configuration.md) - Fine-tune your setup

## Common Patterns

### Conditional Workflows
```python
# Add condition nodes for branching logic
condition = graphbit.PyWorkflowNode.condition_node(
    name="Quality Check",
    description="Checks content quality",
    expression="quality_score > 0.8"
)
```

### Transform Data
```python
# Transform node for data processing
transformer = graphbit.PyWorkflowNode.transform_node(
    name="JSON Extractor",
    description="Extracts JSON from response",
    transformation="json_extract"
)
```

### Add Delays
```python
# Delay node for rate limiting
delay = graphbit.PyWorkflowNode.delay_node(
    name="Rate Limit",
    description="Waits between API calls",
    duration_seconds=5
)
```

Ready to build more complex workflows? Continue with our [User Guide](../user-guide/concepts.md)! 
