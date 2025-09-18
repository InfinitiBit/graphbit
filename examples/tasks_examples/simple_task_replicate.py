#!/usr/bin/env python3
"""
Simple task example using Replicate AI provider.

This example demonstrates how to use GraphBit with Replicate's cloud AI models
for basic text processing tasks.

Requirements:
- REPLICATE_API_KEY environment variable set
- graphbit package installed

Usage:
    python simple_task_replicate.py
"""

import os
import asyncio
from graphbit import LlmConfig, LlmClient, Workflow, Node, Executor


def setup_replicate_config():
    """Set up Replicate configuration with appropriate model and timing."""
    api_key = os.getenv("REPLICATE_API_KEY")
    if not api_key:
        raise ValueError("REPLICATE_API_KEY environment variable is required")
    
    # Configure with Llama 2 70B model for high-quality responses
    config = LlmConfig.replicate(
        api_key=api_key,
        model="meta/llama-2-70b-chat:02e509c789964a7ea8736978a43525956ef40397be9033abf9fd2badfe68c9e3",
        max_wait_time=600,  # 10 minutes for large model
        poll_interval=3     # Poll every 3 seconds
    )
    
    print(f"✓ Configured Replicate provider")
    print(f"  Model: {config.model()}")
    print(f"  Provider: {config.provider()}")
    
    return config


def setup_fast_replicate_config():
    """Set up Replicate configuration with faster model for quick tasks."""
    api_key = os.getenv("REPLICATE_API_KEY")
    if not api_key:
        raise ValueError("REPLICATE_API_KEY environment variable is required")
    
    # Configure with Llama 2 13B model for faster responses
    config = LlmConfig.replicate(
        api_key=api_key,
        model="meta/llama-2-13b-chat:f4e2de70d66816a838a89eeeb621910adffb0dd0baba3976c96980970978018d",
        max_wait_time=300,  # 5 minutes for medium model
        poll_interval=2     # Poll every 2 seconds
    )
    
    print(f"✓ Configured fast Replicate provider")
    print(f"  Model: {config.model()}")
    print(f"  Provider: {config.provider()}")
    
    return config


async def basic_completion_example():
    """Demonstrate basic text completion with Replicate."""
    print("\n=== Basic Completion Example ===")
    
    config = setup_fast_replicate_config()  # Use faster model for demo
    client = LlmClient(config, debug=True)
    
    prompt = "Explain the concept of machine learning in simple terms."
    
    print(f"Prompt: {prompt}")
    print("Generating response... (this may take a moment)")
    
    response = await client.complete_async(
        prompt=prompt,
        max_tokens=300,
        temperature=0.7
    )
    
    print(f"Response: {response}")
    
    # Show client statistics
    stats = client.get_stats()
    print(f"\nClient Stats:")
    print(f"  Total requests: {stats['total_requests']}")
    print(f"  Successful requests: {stats['successful_requests']}")
    print(f"  Average response time: {stats['average_response_time_ms']}ms")


async def workflow_example():
    """Demonstrate workflow creation with Replicate."""
    print("\n=== Workflow Example ===")
    
    config = setup_replicate_config()
    
    # Create workflow
    workflow = Workflow("Replicate Content Analysis")
    
    # Create analyzer node
    analyzer = Node.agent(
        name="Content Analyzer",
        prompt="""
        Analyze the following content and provide:
        1. Main themes and topics
        2. Sentiment analysis
        3. Key insights
        4. Recommendations for improvement
        
        Content: {input}
        
        Provide a structured analysis.
        """,
        agent_id="replicate_analyzer"
    )
    
    # Create summarizer node with faster model
    fast_config = setup_fast_replicate_config()
    summarizer = Node.agent(
        name="Summary Generator",
        prompt="Create a concise executive summary of this analysis: {input}",
        agent_id="replicate_summarizer",
        llm_config=fast_config
    )
    
    # Build workflow
    analyzer_id = workflow.add_node(analyzer)
    summarizer_id = workflow.add_node(summarizer)
    workflow.add_edge(analyzer_id, summarizer_id)
    
    # Validate workflow
    workflow.validate()
    print("✓ Workflow created and validated")
    
    # Create executor with extended timeout for Replicate
    executor = Executor(config, timeout_seconds=900)  # 15 minutes
    
    # Sample content to analyze
    content = """
    Artificial Intelligence (AI) is rapidly transforming industries across the globe. 
    From healthcare to finance, AI technologies are enabling unprecedented automation 
    and decision-making capabilities. Machine learning algorithms can now process 
    vast amounts of data to identify patterns and make predictions with remarkable 
    accuracy. However, this technological advancement also raises important questions 
    about job displacement, privacy, and ethical considerations that society must address.
    """
    
    print(f"Analyzing content: {content[:100]}...")
    print("Processing workflow... (this may take several minutes)")
    
    # Execute workflow
    result = executor.execute(workflow, input_data=content)
    
    if result.is_completed():
        print("✓ Workflow completed successfully")
        print(f"Final output: {result.output()}")
    else:
        print(f"✗ Workflow failed: {result.error()}")


async def batch_processing_example():
    """Demonstrate batch processing with Replicate."""
    print("\n=== Batch Processing Example ===")
    
    config = setup_fast_replicate_config()  # Use faster model for batch
    client = LlmClient(config)
    
    prompts = [
        "What are the benefits of renewable energy?",
        "Explain quantum computing in one paragraph.",
        "Describe the impact of social media on society.",
        "What is the future of space exploration?"
    ]
    
    print(f"Processing {len(prompts)} prompts in batch...")
    print("This may take several minutes due to Replicate's async processing...")
    
    responses = await client.complete_batch(
        prompts=prompts,
        max_tokens=150,
        temperature=0.6,
        max_concurrency=2  # Process 2 at a time to be respectful
    )
    
    print("✓ Batch processing completed")
    for i, (prompt, response) in enumerate(zip(prompts, responses), 1):
        print(f"\n{i}. Prompt: {prompt}")
        print(f"   Response: {response[:100]}...")


async def main():
    """Run all examples."""
    print("GraphBit Replicate AI Examples")
    print("=" * 50)
    
    try:
        # Check API key
        if not os.getenv("REPLICATE_API_KEY"):
            print("❌ REPLICATE_API_KEY environment variable not set")
            print("Please set your Replicate API key:")
            print("export REPLICATE_API_KEY='your-api-key-here'")
            return
        
        # Run examples
        await basic_completion_example()
        await workflow_example()
        await batch_processing_example()
        
        print("\n" + "=" * 50)
        print("✓ All examples completed successfully!")
        print("\nNote: Replicate uses async processing, so responses may take")
        print("longer than other providers but offer access to powerful models.")
        
    except Exception as e:
        print(f"❌ Error running examples: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    asyncio.run(main())
