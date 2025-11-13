#!/usr/bin/env python3
"""
Example: Using Python Bridge to integrate HuggingFace Python wrapper with Rust executor

This example demonstrates how to use the Python bridge system to call the Python
HuggingFace wrapper from the Rust-based workflow execution system.
"""

import os
from dotenv import load_dotenv
from graphbit.graphbit import init, LlmConfig, LlmClient, Workflow, Node, Executor

# Load environment variables
load_dotenv()

def main():
    print("=" * 80)
    print("Python Bridge Example: HuggingFace Integration")
    print("=" * 80)
    
    # Initialize GraphBit
    print("\n1. Initializing GraphBit...")
    init()
    print("   ✓ Initialized")
    
    # Get API key
    api_key = os.getenv("HUGGINGFACE_API_KEY")
    if not api_key:
        print("   ✗ HUGGINGFACE_API_KEY not set in environment")
        return
    print(f"   ✓ API key found (length: {len(api_key)})")
    
    # Create config using Python bridge
    print("\n2. Creating LlmConfig with Python bridge...")
    config = LlmConfig.huggingface_python(
        api_key=api_key,
        model="moonshotai/Kimi-K2-Thinking"  # or any HuggingFace model
    )
    print(f"   ✓ Config created: {config.provider()}/{config.model()}")
    
    # Create LLM client
    print("\n3. Creating LlmClient...")
    client = LlmClient(config, debug=False)
    print("   ✓ Client created")
    
    # Test completion
    print("\n4. Testing completion...")
    print("   Prompt: 'Explain what a Python bridge is in one sentence.'")
    
    response = client.complete_full(
        prompt="Explain what a Python bridge is in one sentence.",
        max_tokens=1000,
        temperature=0.7
    )

    workflow = Workflow("test_workflow")

    executor = Executor(config)

    agent = Node.agent(
        name="agent",
        prompt="Explain what a Python bridge is in one sentence."
    )

    workflow.add_node(agent)

    result = executor.execute(workflow)

    agent_response = result.get_node_output("agent")


    
    print("\n5. Response:")
    print(f"   Content: {response.content}")
    print(f"   Model: {response.model}")
    print(f"   Tokens: {response.usage.total_tokens}")
    print(f"   Finish reason: {response.finish_reason}")
    
    print("\n" + "=" * 80)
    print("✅ Python Bridge Example Completed Successfully")
    print("=" * 80)
    
    print("\n📝 Key Points:")
    print("   - The Python bridge allows Rust code to call Python LLM implementations")
    print("   - Use LlmConfig.huggingface_python() to create a bridge config")
    print("   - The bridge handles async/GIL coordination automatically")
    print("   - Response format is consistent with other LLM providers")

    print("\n" + "=" * 80)
    print("Agent Result:\n")
    print(agent_response)

if __name__ == "__main__":
    main()

