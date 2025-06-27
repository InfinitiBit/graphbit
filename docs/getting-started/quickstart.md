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
# GraphBit supports multiple providers: openai, anthropic, huggingface, ollama
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise ValueError("Please set OPENAI_API_KEY environment variable")

config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")

# Alternative configurations:
# config = graphbit.LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY"), "claude-3-5-sonnet-20241022")
# config = graphbit.LlmConfig.huggingface(os.getenv("HUGGINGFACE_API_KEY"), "microsoft/DialoGPT-medium")
# config = graphbit.LlmConfig.ollama("llama3.2")  # Local model, no API key needed
```

### Step 2: Create Your First Agent Node

```python
# Create a workflow
workflow = graphbit.Workflow("Content Analysis Pipeline")

# Create an analyzer agent
analyzer = graphbit.Node.agent(
    name="Content Analyzer",
    prompt="Analyze the following content and provide key insights: {input}",
    agent_id="analyzer"
)

# Add the node to the workflow
analyzer_id = workflow.add_node(analyzer)
```

### Step 3: Build and Execute the Workflow

```python
# Create executor with basic configuration
executor = graphbit.Executor(config)

# Execute the workflow
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
        print("Please set OPENAI_API_KEY environment variable")
        return
    
    config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
    
    # Build workflow
    workflow = graphbit.Workflow("Content Analysis Pipeline")
    
    analyzer = graphbit.Node.agent(
        name="Content Analyzer",
        prompt="Analyze this content and provide 3 key insights: {input}",
        agent_id="analyzer"
    )
    
    analyzer_id = workflow.add_node(analyzer)
    
    # Execute workflow
    executor = graphbit.Executor(config)
    
    print("Analyzing content...")
    result = executor.execute(workflow)
    
    print(f"Analysis completed in {result.execution_time_ms()}ms")
    print(f"Insights:\n{result.get_variable('output')}")

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
Analyzing content...
Analysis completed in 2347ms
Insights:
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
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Build multi-step workflow
    workflow = graphbit.Workflow("Content Creation Pipeline")
    
    # Step 1: Research Agent
    researcher = graphbit.Node.agent(
        name="Researcher",
        prompt="Research key points about: {topic}. Provide 5 important facts.",
        agent_id="researcher"
    )
    
    # Step 2: Writer Agent  
    writer = graphbit.Node.agent(
        name="Content Writer", 
        prompt="Write a 200-word article about {topic} using this research: {research_data}",
        agent_id="writer"
    )
    
    # Step 3: Editor Agent
    editor = graphbit.Node.agent(
        name="Editor",
        prompt="Edit and improve this article for clarity and engagement: {draft_content}",
        agent_id="editor"
    )
    
    # Add nodes and create connections
    research_id = workflow.add_node(researcher)
    writer_id = workflow.add_node(writer)
    editor_id = workflow.add_node(editor)
    
    # Connect the workflow: Research → Write → Edit
    workflow.connect(research_id, writer_id)
    workflow.connect(writer_id, editor_id)
    
    # Build and execute
    executor = graphbit.Executor(config)
    
    print("Executing content creation pipeline...")
    result = executor.execute(workflow)
    
    print(f"Pipeline completed in {result.execution_time_ms()}ms")
    print(f"Final article:\n{result.get_variable('output')}")

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
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create executor with reliability features
    executor = graphbit.Executor(config, timeout_seconds=60)
    
    # Build workflow
    workflow = graphbit.Workflow("Reliable Analysis")
    
    analyzer = graphbit.Node.agent(
        name="Robust Analyzer",
        prompt="Provide detailed analysis of: {input}",
        agent_id="robust_analyzer"
    )
    
    analyzer_id = workflow.add_node(analyzer)
    
    # Execute with error handling
    try:
        print("🛡️ Executing reliable workflow...")
        result = executor.execute(workflow)
        print(f"Success! Completed in {result.execution_time_ms()}ms")
        print(f"Result: {result.get_variable('output')}")
        
    except Exception as e:
        print(f"Workflow failed: {e}")

if __name__ == "__main__":
    reliable_workflow()
```

## Working with Different LLM Providers

GraphBit supports multiple LLM providers:

```python
import graphbit
import os

# OpenAI Configuration
openai_config = graphbit.LlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), 
    "gpt-4o-mini"
)

# Anthropic Configuration
anthropic_config = graphbit.LlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-3-5-sonnet-20241022"
)

# Ollama Configuration (local)
ollama_config = graphbit.LlmConfig.ollama("llama3.2")

# Use any configuration with the same workflow
def run_with_provider(config):
    workflow = graphbit.Workflow("Multi-Provider Test")
    
    agent = graphbit.Node.agent(
        name="Test Agent",
        prompt="Say hello and identify yourself: {input}",
        agent_id="test_agent"
    )
    
    workflow.add_node(agent)
    
    executor = graphbit.Executor(config)
    result = executor.execute(workflow)
    
    print(f"Provider: {config.provider()}")
    print(f"Model: {config.model()}")
    print(f"Response: {result.get_variable('output')}")
    print("---")

# Test different providers
if __name__ == "__main__":
    graphbit.init()
    
    if os.getenv("OPENAI_API_KEY"):
        run_with_provider(openai_config)
    
    if os.getenv("ANTHROPIC_API_KEY"):
        run_with_provider(anthropic_config)
    
    # Ollama works without API key
    run_with_provider(ollama_config)
```

## Performance Optimization

Create optimized executors for different use cases:

```python
import graphbit
import os

def performance_examples():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # High-throughput executor for batch processing
    high_throughput = graphbit.Executor.new_high_throughput(config, timeout_seconds=120)
    
    # Low-latency executor for real-time applications
    low_latency = graphbit.Executor.new_low_latency(config, timeout_seconds=30)
    
    # Memory-optimized executor for resource-constrained environments
    memory_optimized = graphbit.Executor.new_memory_optimized(config)
    
    # Use appropriate executor based on your needs
    workflow = graphbit.Workflow("Performance Test")
    
    agent = graphbit.Node.agent(
        name="Performance Agent",
        prompt="Process this efficiently: {input}",
        agent_id="perf_agent"
    )
    
    workflow.add_node(agent)
    
    # Execute with chosen executor
    result = low_latency.execute(workflow)
    print(f"Execution mode: {low_latency.get_execution_mode()}")
    print(f"Completed in: {result.execution_time_ms()}ms")

if __name__ == "__main__":
    performance_examples()
```

## Next Steps

Congratulations! You've created your first GraphBit workflows. Here's what to explore next:

### Learn More
- [Core Concepts](../user-guide/concepts.md) - Understand workflows, agents, and nodes
- [Node Types](../api-reference/node-types.md) - Explore all available node types
- [LLM Providers](../user-guide/llm-providers.md) - Configure different AI providers

### Advanced Features
- [Dynamic Graph Generation](../advanced/dynamic-graphs.md) - Auto-generate workflow nodes
- [Embeddings & Vector Search](../user-guide/embeddings.md) - Add semantic search
- [Performance Optimization](../advanced/performance.md) - Tune for production

### Examples
- [Content Generation Pipeline](../examples/content-generation.md) - Complete content creation workflow
- [Data Processing Workflow](../examples/data-processing.md) - ETL with AI agents
- [Code Review Automation](../examples/code-review.md) - Automated code analysis

### Production Ready
- [Error Handling & Reliability](../user-guide/reliability.md) - Production-grade error handling
- [Monitoring & Observability](../advanced/monitoring.md) - Track workflow performance
- [Configuration Options](../api-reference/configuration.md) - Fine-tune your setup

## Common Patterns

### Conditional Workflows
```python
# Add condition nodes for branching logic
condition = graphbit.Node.condition(
    name="Quality Check",
    expression="quality_score > 0.8"
)
```

### Transform Data
```python
# Transform node for data processing
transformer = graphbit.Node.transform(
    name="JSON Extractor",
    transformation="json_extract"
)
```

### System Information and Health Check
```python
# Check GraphBit system status
import graphbit

graphbit.init()

# Get system information
system_info = graphbit.get_system_info()
print(f"GraphBit version: {system_info['version']}")
print(f"Runtime healthy: {system_info['runtime_initialized']}")

# Perform health check
health = graphbit.health_check()
print(f"Overall healthy: {health['overall_healthy']}")
```

Ready to build more complex workflows? Continue with our [User Guide](../user-guide/concepts.md)! 
