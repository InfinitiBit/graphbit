# Quick Start Tutorial

Welcome to GraphBit! This tutorial will guide you through creating your first AI agent workflow in just 5 minutes.

## Prerequisites

Before starting, ensure you have:
- GraphBit installed ([Installation Guide](installation.md))
- An OpenAI API key set in your environment
- Python 3.10+ available

---

## Your First Workflow

Let's create a simple content analysis workflow that analyzes text and provides insights.

### Step 1: Basic Setup

Create a new Python file `my_first_workflow.py`:

```python
import os

from graphbit import LlmConfig

# Configure LLM (using OpenAI GPT-4)
# GraphBit supports multiple providers: openai, anthropic, ollama
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise ValueError("Please set OPENAI_API_KEY environment variable")

config = LlmConfig.openai(api_key, "gpt-4o-mini")

# Alternative configurations:
# config = LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY"), "claude-sonnet-4-20250514")
# config = LlmConfig.ollama("llama3.2")  # Local model, no API key needed
```

### Step 2: Create Your First Agent Node

```python
from graphbit import Node, Workflow

# Create a workflow
workflow = Workflow("Content Analysis Pipeline")

# Create an analyzer agent
analyzer = Node.agent(
    name="Content Analyzer",
    prompt=f"Analyze the following content and provide key insights: {input}",
    agent_id="analyzer"
)

# Add the node to the workflow
analyzer_id = workflow.add_node(analyzer)
```

### Step 3: Build and Execute the Workflow

```python
from graphbit import Executor

# Create executor with basic configuration
executor = Executor(config)

# Execute the workflow
print("🚀 Executing workflow...")
result = executor.execute(workflow)

# Display results
print(f"✅ Workflow completed in {result.execution_time_ms()}ms")
print(f"📊 Result: {result.get_node_output('output')}")
```

### Complete Example

Here's the complete working example:

```python
import os

from graphbit import init, LlmConfig, Node, Workflow, Executor

def main():    
    # Configure LLM
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("Please set OPENAI_API_KEY environment variable")
        return
    
    config = LlmConfig.openai(api_key, "gpt-4o-mini")
    
    # Build workflow
    workflow = Workflow("Content Analysis Pipeline")
    
    analyzer = Node.agent(
        name="Content Analyzer",
        prompt=f"Analyze this content and provide 3 key insights: {input}",
        agent_id="analyzer"
    )
    
    analyzer_id = workflow.add_node(analyzer)
    
    # Execute workflow
    executor = Executor(config)
    
    print("Analyzing content...")
    result = executor.execute(workflow)
    
    print(f"Analysis completed in {result.execution_time_ms()}ms")
    print(f"Insights:\n{result.get_node_output('Content Analyzer')}")

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

---

## Multi-Step Workflows

Let's create a more complex workflow with multiple connected nodes:

```python
import os

from graphbit import init, LlmConfig, Node, Workflow, Executor

def create_content_pipeline():
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Build multi-step workflow
    workflow = Workflow("Content Creation Pipeline")
    
    # Step 1: Research Agent
    researcher = Node.agent(
        name="Researcher",
        prompt=f"Research key points about: {topic}. Provide 5 important facts.",
        agent_id="researcher"
    )
    
    # Step 2: Writer Agent  
    writer = Node.agent(
        name="Content Writer", 
        prompt=f"Write a 200-word article about {topic} using this research data.",
        agent_id="writer"
    )
    
    # Step 3: Editor Agent
    editor = Node.agent(
        name="Editor",
        prompt=f"Edit and improve this article for clarity and engagement.",
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
    executor = Executor(config)
    
    print("Executing content creation pipeline...")
    result = executor.execute(workflow)
    
    print(f"Pipeline completed in {result.execution_time_ms()}ms")
    print(f"Final article:\n{result.get_node_output('Editor')}")

if __name__ == "__main__":
    create_content_pipeline()
```

---

## Adding Reliability Features

Enhance your workflow with error handling and retries:

```python
import os

from graphbit import init, LlmConfig, Node, Workflow, Executor

def reliable_workflow():
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create executor with reliability features
    executor = Executor(config, timeout_seconds=60)
    
    # Build workflow
    workflow = Workflow("Reliable Analysis")
    
    analyzer = Node.agent(
        name="Robust Analyzer",
        prompt=f"Provide detailed analysis of: {input}",
        agent_id="robust_analyzer"
    )
    
    analyzer_id = workflow.add_node(analyzer)
    
    # Execute with error handling
    try:
        print("🛡️ Executing reliable workflow...")
        result = executor.execute(workflow)
        print(f"Success! Completed in {result.execution_time_ms()}ms")
        print(f"Result: {result.get_node_output('Robust Analyzer')}")
        
    except Exception as e:
        print(f"Workflow failed: {e}")

if __name__ == "__main__":
    reliable_workflow()
```

---

## Working with Different LLM Providers

GraphBit supports multiple LLM providers:

```python
import os

from graphbit import init, LlmConfig, Node, Workflow, Executor

# OpenAI Configuration
openai_config = LlmConfig.openai(
    os.getenv("OPENAI_API_KEY"), 
    "gpt-4o-mini"
)

# Anthropic Configuration
anthropic_config = LlmConfig.anthropic(
    os.getenv("ANTHROPIC_API_KEY"),
    "claude-sonnet-4-20250514"
)

# OpenRouter Configuration (access to 400+ models)
openrouter_config = LlmConfig.openrouter(
    os.getenv("OPENROUTER_API_KEY"),
    "anthropic/claude-3-5-sonnet"  # Use Claude through OpenRouter
)

# Ollama Configuration (local)
ollama_config = LlmConfig.ollama("llama3.2")

# Use any configuration with the same workflow
def run_with_provider(config):
    workflow = Workflow("Multi-Provider Test")
    
    agent = Node.agent(
        name="Test Agent",
        prompt=f"Say hello and identify yourself: {input}",
        agent_id="test_agent"
    )
    
    workflow.add_node(agent)
    
    executor = Executor(config)
    result = executor.execute(workflow)
    
    print(f"Provider: {config.provider()}")
    print(f"Model: {config.model()}")
    print(f"Response: {result.get_node_output('output')}")
    print("---")

# Test different providers
if __name__ == "__main__":
    init()
    
    if os.getenv("OPENAI_API_KEY"):
        run_with_provider(openai_config)
    
    if os.getenv("ANTHROPIC_API_KEY"):
        run_with_provider(anthropic_config)
    
    # Ollama works without API key
    run_with_provider(ollama_config)
```

---

## Performance Optimization

Create optimized executors for different use cases:

```python
import os

from graphbit import init, LlmConfig, Node, Workflow, Executor

def performance_examples():
    init()
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # High-throughput executor for batch processing
    high_throughput = Executor(config, timeout_seconds=120)
    
    # Low-latency executor for real-time applications
    low_latency = Executor(config, lightweing_mode=True, timeout_seconds=30)    
    
    # Use appropriate executor based on your needs
    workflow = Workflow("Performance Test")
    
    agent = Node.agent(
        name="Performance Agent",
        prompt=f"Process this efficiently: {input}",
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

---

## Next Steps

Congratulations! You've created your first GraphBit workflows. Here's what to explore next:

### Learn More
- [Core Concepts](../user-guide/concepts.md) - Understand workflows, agents, and nodes
- [Node Types](../api-reference/node-types.md) - Explore all available node types
- [LLM Providers](../user-guide/llm-providers.md) - Configure different AI providers

### Advanced Features
- [Dynamic Graph Generation](../user-guide/dynamics-graph.md) - Auto-generate workflow nodes
- [Embeddings & Vector Search](../user-guide/embeddings.md) - Add semantic search
- [Performance Optimization](../user-guide/performance.md) - Tune for production

### Examples
- [Content Generation Pipeline](../examples/content-generation.md) - Complete content creation workflow
- [Data Processing Workflow](../examples/data-processing.md) - ETL with AI agents
- [Code Review Automation](../examples/semantic-search.md) - Semantic search workflow

### Production Ready
- [Error Handling & Reliability](../user-guide/reliability.md) - Production-grade error handling
- [Monitoring & Observability](../user-guide/monitoring.md) - Track workflow performance
- [Configuration Options](../api-reference/configuration.md) - Fine-tune your setup

Ready to build more complex workflows? Continue with our [User Guide](../user-guide/concepts.md)! 

---
