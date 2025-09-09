# Basic Examples

This guide provides simple, practical examples to help you get started with GraphBit quickly.

## Example 1: Simple Text Analysis

Analyze text content and extract key insights:

```python
import os

from graphbit import LlmConfig, Node, Workflow, Executor

# Initialize
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create workflow
workflow = Workflow("Text Analysis")
analyzer = Node.agent(
    name="Text Analyzer",
    prompt=f"Analyze this text and provide 3 key insights: {input}",
    agent_id="analyzer"
)

workflow.add_node(analyzer)

# Execute
executor = Executor(config)
result = executor.execute(workflow)

print(f"Analysis: {result.get_node_output('Text Analyzer')}")
```

## Example 2: Sequential Pipeline  

Create a multi-step content processing pipeline:

```python
import os

from graphbit import LlmConfig, Workflow, Node, Executor

def create_content_pipeline():
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    workflow = Workflow("Content Pipeline")
    
    # Step 1: Research
    researcher = Node.agent(
        name="Researcher",
        prompt=f"Research 3 key facts about: {topic}",
        agent_id="researcher"
    )
    
    # Step 2: Writer
    writer = Node.agent(
        name="Writer",
        prompt=f"Write a paragraph about {topic} using the facts from Researcher.",
        agent_id="writer"
    )
    
    # Connect nodes
    research_id = workflow.add_node(researcher)
    writer_id = workflow.add_node(writer)
    workflow.connect(research_id, writer_id)
    
    executor = Executor(config)
    
    return executor.execute(workflow)

# Usage
result = create_content_pipeline()
print(f"Final content: {result.get_node_output('Writer')}")
```

## Example 3: Multiple LLM Providers

Use different LLM providers in the same workflow:

```python
import os

from graphbit import LlmConfig, Node, Workflow, Executor

def multi_provider_example():
    # OpenAI for creative tasks
    openai_config = LlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "gpt-4o-mini"
    )
    
    # Anthropic for analytical tasks
    anthropic_config = LlmConfig.anthropic(
        os.getenv("ANTHROPIC_API_KEY"),
        "claude-sonnet-4-20250514"
    )
    
    # Ollama for local execution
    ollama_config = LlmConfig.ollama("llama3.2")
    
    # Create separate executors for different providers
    creative_executor = Executor(openai_config)
    analytical_executor = Executor(anthropic_config)
    local_executor = Executor(ollama_config)
    
    # Creative workflow
    creative_workflow = Workflow("Creative Writing")
    writer = Node.agent(
        name="Creative Writer",
        prompt=f"Write a creative story about: {topic}",
        agent_id="creative_writer"
    )
    
    creative_workflow.add_node(writer)
    
    # Analytical workflow
    analytical_workflow = Workflow("Analysis")
    analyzer = Node.agent(
        name="Data Analyzer",
        prompt=f"Analyze this data and provide insights: {data}",
        agent_id="analyzer"
    )
    
    analytical_workflow.add_node(analyzer)
    
    # Execute with different providers
    creative_result = creative_executor.execute(creative_workflow)
    analytical_result = analytical_executor.execute(analytical_workflow)
    
    print(f"Creative (OpenAI): {creative_result.get_node_output('Creative Writer')}")
    print(f"Analytical (Anthropic): {analytical_result.get_node_output('Data Analyzer')}")
    
    return creative_result, analytical_result
```

## Example 4: Error Handling and Performance Optimization

Build robust workflows with error handling and optimized performance:

```python
import os

from graphbit import LlmConfig, Node, Workflow, Executor

def robust_workflow_example():
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create high-throughput executor with timeout
    executor = Executor(config, timeout_seconds=120)
    
    # Alternative: Create low-latency executor for real-time
    # executor = Executor(config, lightweight_mode=True, timeout_seconds=60)
    
    workflow = Workflow("Robust Workflow")
    
    agent = Node.agent(
        name="Reliable Agent",
        prompt=f"Process this data reliably: {input}",
        agent_id="reliable"
    )
    
    workflow.add_node(agent)
    
    try:
        result = executor.execute(workflow)
        if result.is_success():
            print(f"Success: {result.get_node_output('output')}")
            print(f"Execution time: {result.execution_time_ms()}ms")
            print(f"Execution mode: {executor.get_execution_mode()}")
        else:
            print(f"Workflow failed: {result.state()}")
            
    except Exception as e:
        print(f"Exception: {e}")

# Run the example
robust_workflow_example()
```

## Example 5: Embeddings and Similarity

Use embeddings for semantic search and similarity:

```python
import os

from graphbit import EmbeddingConfig, EmbeddingClient

def embeddings_example():    
    # Create embedding service
    embedding_config = EmbeddingConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "text-embedding-3-small"
    )
    
    service = EmbeddingClient(embedding_config)
    
    # Process multiple texts
    texts = [
        "Machine learning is revolutionizing technology",
        "AI is transforming how we work and live",
        "The weather is nice today"
    ]
    
    print("Generating embeddings...")

    embeddings = service.embed_many(texts)
    
    for i, embedding in enumerate(embeddings):
        print(f"Text {i+1}: {embedding}")
    
    print("Embeddings generated successfully")

# Run async example
embeddings_example()
```

## Example 6: System Monitoring and Diagnostics

Monitor GraphBit performance and health:

```python
import os

from graphbit import LlmConfig, Node, Workflow, Executor, get_system_info, health_check

def system_monitoring_example():    
    # Get system information
    system_info = get_system_info()
    print("🖥️ System Information:")
    print(f"   GraphBit version: {system_info['version']}")
    print(f"   Python binding version: {system_info['python_binding_version']}")
    print(f"   CPU count: {system_info['cpu_count']}")
    print(f"   Memory allocator: {system_info['memory_allocator']}")
    print(f"   Runtime initialized: {system_info['runtime_initialized']}")
    
    # Perform health check
    health = health_check()
    print("\nHealth Check:")
    print(f"   Overall healthy: {health['overall_healthy']}")
    print(f"   Runtime healthy: {health['runtime_healthy']}")
    print(f"   Memory healthy: {health['memory_healthy']}")
    print(f"   Available memory: {health.get('available_memory_mb', 'N/A')} MB")
    
    # Create and monitor an executor
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = Executor(config, debug=True)  # Enable debugging
    
    # Get executor statistics
    stats = executor.get_stats()
    print(f"\nExecutor Statistics:")
    print(f"   Execution mode: {executor.get_execution_mode()}")
    print(f"   Lightweight mode: {executor.is_lightweight_mode()}")
    
    # Execute a simple workflow to generate stats
    workflow = Workflow("Monitoring Test")
    agent = Node.agent(
        name="Monitor Agent",
        prompt=f"Say hello: {input}",
        agent_id="monitor"
    )
    workflow.add_node(agent)
    
    result = executor.execute(workflow)
    
    # Get updated statistics
    updated_stats = executor.get_stats()
    print(f"\nUpdated Statistics:")
    for key, value in updated_stats.items():
        print(f"   {key}: {value}")

# Run monitoring example
if __name__ == "__main__":
    system_monitoring_example()
```

## Example 7: Workflow Validation

Validate workflow structure before execution:

```python
import os

from graphbit import LlmConfig, Node, Workflow, Executor

def workflow_validation_example():
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create a workflow
    workflow = Workflow("Validation Test")
    
    # Add nodes
    node1 = Node.agent(
        name="First Agent",
        prompt=f"Process input: {input}",
        agent_id="agent1"
    )
    
    node2 = Node.agent(
        name="Second Agent", 
        prompt="Continue processing the input.",
        agent_id="agent2"
    )
    
    id1 = workflow.add_node(node1)
    id2 = workflow.add_node(node2)
    workflow.connect(id1, id2)
    
    # Validate workflow structure
    try:
        workflow.validate()
        print("Workflow validation passed")
        
        # Execute the validated workflow
        executor = Executor(config)
        result = executor.execute(workflow)
        print(f"Execution completed: {result.get_node_output('Second Agent')}")
        
    except Exception as e:
        print(f"Workflow validation failed: {e}")

# Run validation example
workflow_validation_example()
```

## Tips for Getting Started

1. **Start Simple**: Begin with single-node workflows
2. **Test Incrementally**: Add complexity gradually
3. **Handle Errors**: Always include error handling with try-catch blocks
4. **Monitor Performance**: Use `result.execution_time_ms()` to track execution times
5. **Validate Workflows**: Call `workflow.validate()` before execution
6. **Use Templates**: Create reusable workflow patterns
7. **Check System Health**: Use `health_check()` for diagnostics

## Next Steps

Once you're comfortable with these examples:

- Explore [Core Concepts](../user-guide/concepts.md)
- Learn about [Dynamic Graph Generation](../user-guide/dynamics-graph.md)
- Check out [Complete Examples](../examples/content-generation.md)
- Read the [API Reference](../api-reference/python-api.md) 
