# Basic Examples

This guide provides simple, practical examples to help you get started with GraphBit quickly.

## Example 1: Simple Text Analysis

Analyze text content and extract key insights:

```python
import graphbit
import os

# Initialize
graphbit.init()
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create workflow
builder = graphbit.PyWorkflowBuilder("Text Analysis")
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Text Analyzer",
    description="Analyzes text content",
    agent_id="analyzer",
    prompt="Analyze this text and provide 3 key insights: {input}"
)

builder.add_node(analyzer)
workflow = builder.build()

# Execute
executor = graphbit.PyWorkflowExecutor(config)
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
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    builder = graphbit.PyWorkflowBuilder("Content Pipeline")
    
    # Step 1: Research
    researcher = graphbit.PyWorkflowNode.agent_node(
        name="Researcher",
        description="Researches topics",
        agent_id="researcher",
        prompt="Research 3 key facts about: {topic}"
    )
    
    # Step 2: Writer
    writer = graphbit.PyWorkflowNode.agent_node(
        name="Writer",
        description="Writes content",
        agent_id="writer",
        prompt="Write a paragraph about {topic} using these facts: {research_output}"
    )
    
    # Connect nodes
    research_id = builder.add_node(researcher)
    writer_id = builder.add_node(writer)
    builder.connect(research_id, writer_id, graphbit.PyWorkflowEdge.data_flow())
    
    workflow = builder.build()
    executor = graphbit.PyWorkflowExecutor(config)
    
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
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    builder = graphbit.PyWorkflowBuilder("Quality Check")
    
    # Analyzer
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Quality Checker",
        description="Checks content quality",
        agent_id="checker",
        prompt="Rate this content quality from 1-10: {input}"
    )
    
    # Condition
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        name="Quality Gate",
        description="Checks if quality is acceptable",
        expression="quality_score > 7"
    )
    
    # Approver
    approver = graphbit.PyWorkflowNode.agent_node(
        name="Content Approver",
        description="Approves high-quality content",
        agent_id="approver",
        prompt="Approve this high-quality content: {input}"
    )
    
    # Connect with conditional flow
    check_id = builder.add_node(analyzer)
    gate_id = builder.add_node(quality_gate)
    approve_id = builder.add_node(approver)
    
    builder.connect(check_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(gate_id, approve_id, graphbit.PyWorkflowEdge.conditional("quality_score > 7"))
    
    workflow = builder.build()
    executor = graphbit.PyWorkflowExecutor(config)
    
    return executor.execute(workflow)
```

## Example 4: Data Transformation

Transform data between workflow steps:

```python
import graphbit
import os

def create_transform_workflow():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    builder = graphbit.PyWorkflowBuilder("Data Transform")
    
    # Generator
    generator = graphbit.PyWorkflowNode.agent_node(
        name="Data Generator",
        description="Generates sample data",
        agent_id="generator",
        prompt="Generate a JSON object with name, age, and city for a person"
    )
    
    # Transformer
    transformer = graphbit.PyWorkflowNode.transform_node(
        name="JSON Extractor",
        description="Extracts JSON data",
        transformation="json_extract"
    )
    
    # Processor
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Data Processor",
        description="Processes extracted data",
        agent_id="processor",
        prompt="Summarize this person's data: {extracted_data}"
    )
    
    # Connect
    gen_id = builder.add_node(generator)
    trans_id = builder.add_node(transformer)
    proc_id = builder.add_node(processor)
    
    builder.connect(gen_id, trans_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(trans_id, proc_id, graphbit.PyWorkflowEdge.data_flow())
    
    workflow = builder.build()
    executor = graphbit.PyWorkflowExecutor(config)
    
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
    openai_config = graphbit.PyLlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "gpt-4o-mini"
    )
    
    # Create separate executors for different providers
    creative_executor = graphbit.PyWorkflowExecutor(openai_config)
    
    # Creative workflow
    builder = graphbit.PyWorkflowBuilder("Creative Writing")
    writer = graphbit.PyWorkflowNode.agent_node(
        name="Creative Writer",
        description="Writes creative content",
        agent_id="creative_writer",
        prompt="Write a creative story about: {topic}"
    )
    
    builder.add_node(writer)
    workflow = builder.build()
    
    return creative_executor.execute(workflow)
```

## Example 6: Error Handling

Build robust workflows with error handling:

```python
import graphbit
import os

def robust_workflow_example():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create executor with retry and circuit breaker
    executor = graphbit.PyWorkflowExecutor(config) \
        .with_retry_config(graphbit.PyRetryConfig.default()) \
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default())
    
    builder = graphbit.PyWorkflowBuilder("Robust Workflow")
    
    agent = graphbit.PyWorkflowNode.agent_node(
        name="Reliable Agent",
        description="Performs reliable processing",
        agent_id="reliable",
        prompt="Process this data reliably: {input}"
    )
    
    builder.add_node(agent)
    workflow = builder.build()
    
    try:
        result = executor.execute(workflow)
        if result.is_completed():
            print(f"✅ Success: {result.get_variable('output')}")
        else:
            print("❌ Workflow failed")
            
    except Exception as e:
        print(f"❌ Exception: {e}")

# Run the example
robust_workflow_example()
```

## Example 7: Batch Processing

Process multiple items efficiently:

```python
import graphbit
import os

def batch_processing_example():
    graphbit.init()
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    
    # Create workflow template
    def create_analysis_workflow():
        builder = graphbit.PyWorkflowBuilder("Batch Analysis")
        
        analyzer = graphbit.PyWorkflowNode.agent_node(
            name="Batch Analyzer",
            description="Analyzes batch items",
            agent_id="batch_analyzer",
            prompt="Analyze and categorize: {item}"
        )
        
        builder.add_node(analyzer)
        return builder.build()
    
    # Create multiple workflows
    workflows = [create_analysis_workflow() for _ in range(3)]
    
    # Execute in batch
    executor = graphbit.PyWorkflowExecutor(config)
    results = executor.execute_batch(workflows)
    
    for i, result in enumerate(results):
        print(f"Batch {i+1}: {result.get_variable('output')}")

# Run batch processing
batch_processing_example()
```

## Example 8: Embeddings and Similarity

Use embeddings for semantic search:

```python
import graphbit
import os
import asyncio

async def embeddings_example():
    graphbit.init()
    
    # Create embedding service
    embedding_config = graphbit.PyEmbeddingConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "text-embedding-ada-002"
    )
    
    service = graphbit.PyEmbeddingService(embedding_config)
    
    # Generate embeddings
    texts = [
        "Machine learning is revolutionizing technology",
        "AI is transforming how we work and live",
        "The weather is nice today"
    ]
    
    embeddings = await service.embed_texts(texts)
    
    # Calculate similarities
    similarity_1_2 = graphbit.PyEmbeddingService.cosine_similarity(
        embeddings[0], embeddings[1]
    )
    similarity_1_3 = graphbit.PyEmbeddingService.cosine_similarity(
        embeddings[0], embeddings[2]
    )
    
    print(f"Similarity (ML vs AI): {similarity_1_2:.3f}")
    print(f"Similarity (ML vs Weather): {similarity_1_3:.3f}")

# Run async example
asyncio.run(embeddings_example())
```

## Tips for Getting Started

1. **Start Simple**: Begin with single-node workflows
2. **Test Incrementally**: Add complexity gradually
3. **Handle Errors**: Always include error handling
4. **Monitor Performance**: Track execution times
5. **Validate Workflows**: Use workflow validation
6. **Use Templates**: Create reusable workflow patterns

## Next Steps

Once you're comfortable with these examples:

- Explore [Core Concepts](../user-guide/concepts.md)
- Learn about [Advanced Features](../advanced/dynamic-graphs.md)
- Check out [Complete Examples](../examples/content-generation.md)
- Read the [API Reference](../api-reference/python-api.md) 