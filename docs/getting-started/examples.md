# Basic Examples

This guide provides simple, practical examples to help you get started with GraphBit quickly.

## Example 1: Simple Text Analysis

Analyze text content and extract key insights:

```python
import graphbit
import os

# Initialize
graphbit.init()
config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create workflow
workflow = graphbit.Workflow("Text Analysis")
analyzer = graphbit.Node.agent(
    name="Text Analyzer",
    prompt="Analyze this text and provide 3 key insights: {input}",
    agent_id="analyzer"
)

workflow.add_node(analyzer)

# Execute
executor = graphbit.Executor(config)
result = executor.execute(workflow)

print(f"Analysis: {result.get_variable('output')}")
```

## Example 2: Sequential Pipeline  

Create a multi-step content processing pipeline:

```python
import graphbit
import os

def create_content_pipeline():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    workflow = graphbit.Workflow("Content Pipeline")
    
    # Step 1: Research
    researcher = graphbit.Node.agent(
        name="Researcher",
        prompt="Research 3 key facts about: {topic}",
        agent_id="researcher"
    )
    
    # Step 2: Writer
    writer = graphbit.Node.agent(
        name="Writer",
        prompt="Write a paragraph about {topic} using these facts: {research_output}",
        agent_id="writer"
    )
    
    # Connect nodes
    research_id = workflow.add_node(researcher)
    writer_id = workflow.add_node(writer)
    workflow.connect(research_id, writer_id)
    
    executor = graphbit.Executor(config)
    
    return executor.execute(workflow)

# Usage
result = create_content_pipeline()
print(f"Final content: {result.get_variable('output')}")
```

## Example 3: Conditional Workflow

Use condition nodes for branching logic:

```python
import graphbit
import os

def create_quality_check_workflow():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    workflow = graphbit.Workflow("Quality Check")
    
    # Analyzer
    analyzer = graphbit.Node.agent(
        name="Quality Checker",
        prompt="Rate this content quality from 1-10: {input}",
        agent_id="checker"
    )
    
    # Condition
    quality_gate = graphbit.Node.condition(
        name="Quality Gate",
        expression="quality_score > 7"
    )
    
    # Approver
    approver = graphbit.Node.agent(
        name="Content Approver",
        prompt="Approve this high-quality content: {input}",
        agent_id="approver"
    )
    
    # Connect with conditional flow
    check_id = workflow.add_node(analyzer)
    gate_id = workflow.add_node(quality_gate)
    approve_id = workflow.add_node(approver)
    
    workflow.connect(check_id, gate_id)
    workflow.connect(gate_id, approve_id)
    
    executor = graphbit.Executor(config)
    
    return executor.execute(workflow)
```

## Example 4: Data Transformation

Transform data between workflow steps:

```python
import graphbit
import os

def create_transform_workflow():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    workflow = graphbit.Workflow("Data Transform")
    
    # Generator
    generator = graphbit.Node.agent(
        name="Data Generator",
        prompt="Generate a JSON object with name, age, and city for a person",
        agent_id="generator"
    )
    
    # Transformer
    transformer = graphbit.Node.transform(
        name="JSON Extractor",
        transformation="json_extract"
    )
    
    # Processor
    processor = graphbit.Node.agent(
        name="Data Processor",
        prompt="Summarize this person's data: {extracted_data}",
        agent_id="processor"
    )
    
    # Connect
    gen_id = workflow.add_node(generator)
    trans_id = workflow.add_node(transformer)
    proc_id = workflow.add_node(processor)
    
    workflow.connect(gen_id, trans_id)
    workflow.connect(trans_id, proc_id)
    
    executor = graphbit.Executor(config)
    
    return executor.execute(workflow)
```

## Example 5: Multiple LLM Providers

Use different LLM providers in the same workflow:

```python
import graphbit
import os

def multi_provider_example():
    graphbit.init()
    
    # OpenAI for creative tasks
    openai_config = graphbit.LlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "gpt-4o-mini"
    )
    
    # Anthropic for analytical tasks
    anthropic_config = graphbit.LlmConfig.anthropic(
        os.getenv("ANTHROPIC_API_KEY"),
        "claude-3-5-sonnet-20241022"
    )
    
    # Ollama for local execution
    ollama_config = graphbit.LlmConfig.ollama("llama3.2")
    
    # Create separate executors for different providers
    creative_executor = graphbit.Executor(openai_config)
    analytical_executor = graphbit.Executor(anthropic_config)
    local_executor = graphbit.Executor(ollama_config)
    
    # Creative workflow
    creative_workflow = graphbit.Workflow("Creative Writing")
    writer = graphbit.Node.agent(
        name="Creative Writer",
        prompt="Write a creative story about: {topic}",
        agent_id="creative_writer"
    )
    
    creative_workflow.add_node(writer)
    
    # Analytical workflow
    analytical_workflow = graphbit.Workflow("Analysis")
    analyzer = graphbit.Node.agent(
        name="Data Analyzer",
        prompt="Analyze this data and provide insights: {data}",
        agent_id="analyzer"
    )
    
    analytical_workflow.add_node(analyzer)
    
    # Execute with different providers
    creative_result = creative_executor.execute(creative_workflow)
    analytical_result = analytical_executor.execute(analytical_workflow)
    
    print(f"Creative (OpenAI): {creative_result.get_variable('output')}")
    print(f"Analytical (Anthropic): {analytical_result.get_variable('output')}")
    
    return creative_result, analytical_result
```

## Example 6: Error Handling and Performance Optimization

Build robust workflows with error handling and optimized performance:

```python
import graphbit
import os

def robust_workflow_example():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create high-throughput executor with timeout
    executor = graphbit.Executor.new_high_throughput(config, timeout_seconds=120)
    
    # Alternative: Create low-latency executor for real-time
    # executor = graphbit.Executor.new_low_latency(config, timeout_seconds=30)
    
    # Alternative: Create memory-optimized executor
    # executor = graphbit.Executor.new_memory_optimized(config)
    
    workflow = graphbit.Workflow("Robust Workflow")
    
    agent = graphbit.Node.agent(
        name="Reliable Agent",
        prompt="Process this data reliably: {input}",
        agent_id="reliable"
    )
    
    workflow.add_node(agent)
    
    try:
        result = executor.execute(workflow)
        if result.is_success():
            print(f"Success: {result.get_variable('output')}")
            print(f"Execution time: {result.execution_time_ms()}ms")
            print(f"Execution mode: {executor.get_execution_mode()}")
        else:
            print(f"Workflow failed: {result.state()}")
            
    except Exception as e:
        print(f"Exception: {e}")

# Run the example
robust_workflow_example()
```

## Example 7: Embeddings and Similarity

Use embeddings for semantic search and similarity:

```python
import graphbit
import os

def embeddings_example():
    graphbit.init()
    
    # Create embedding service
    embedding_config = graphbit.EmbeddingConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "text-embedding-3-small"
    )
    
    service = graphbit.EmbeddingClient(embedding_config)
    
    # Process multiple texts
    texts = [
        "Machine learning is revolutionizing technology",
        "AI is transforming how we work and live",
        "The weather is nice today"
    ]
    
    print("Generating embeddings...")
    
    # Note: Actual embedding implementation depends on the binding
    # This is a conceptual example
    for i, text in enumerate(texts):
        print(f"Text {i+1}: {text}")
    
    print("Embeddings generated successfully")

# Run async example
embeddings_example()
```

## Example 8: System Monitoring and Diagnostics

Monitor GraphBit performance and health:

```python
import graphbit
import os

def system_monitoring_example():
    graphbit.init()
    
    # Get system information
    system_info = graphbit.get_system_info()
    print("🖥️ System Information:")
    print(f"   GraphBit version: {system_info['version']}")
    print(f"   Python binding version: {system_info['python_binding_version']}")
    print(f"   CPU count: {system_info['cpu_count']}")
    print(f"   Memory allocator: {system_info['memory_allocator']}")
    print(f"   Runtime initialized: {system_info['runtime_initialized']}")
    
    # Perform health check
    health = graphbit.health_check()
    print("\nHealth Check:")
    print(f"   Overall healthy: {health['overall_healthy']}")
    print(f"   Runtime healthy: {health['runtime_healthy']}")
    print(f"   Memory healthy: {health['memory_healthy']}")
    print(f"   Available memory: {health.get('available_memory_mb', 'N/A')} MB")
    
    # Create and monitor an executor
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = graphbit.Executor(config, debug=True)  # Enable debugging
    
    # Get executor statistics
    stats = executor.get_stats()
    print(f"\nExecutor Statistics:")
    print(f"   Execution mode: {executor.get_execution_mode()}")
    print(f"   Lightweight mode: {executor.is_lightweight_mode()}")
    
    # Execute a simple workflow to generate stats
    workflow = graphbit.Workflow("Monitoring Test")
    agent = graphbit.Node.agent(
        name="Monitor Agent",
        prompt="Say hello: {input}",
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

## Example 9: Workflow Validation

Validate workflow structure before execution:

```python
import graphbit
import os

def workflow_validation_example():
    graphbit.init()
    config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create a workflow
    workflow = graphbit.Workflow("Validation Test")
    
    # Add nodes
    node1 = graphbit.Node.agent(
        name="First Agent",
        prompt="Process input: {input}",
        agent_id="agent1"
    )
    
    node2 = graphbit.Node.agent(
        name="Second Agent", 
        prompt="Continue processing: {previous_output}",
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
        executor = graphbit.Executor(config)
        result = executor.execute(workflow)
        print(f"Execution completed: {result.get_variable('output')}")
        
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
7. **Choose Appropriate Executors**: Use `new_high_throughput()`, `new_low_latency()`, or `new_memory_optimized()` based on your needs
8. **Check System Health**: Use `graphbit.health_check()` for diagnostics

## Next Steps

Once you're comfortable with these examples:

- Explore [Core Concepts](../user-guide/concepts.md)
- Learn about [Advanced Features](../advanced/dynamic-graphs.md)
- Check out [Complete Examples](../examples/content-generation.md)
- Read the [API Reference](../api-reference/python-api.md) 
