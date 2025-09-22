#!/usr/bin/env python3
"""
Simple OpenRouter Example for GraphBit.

This example demonstrates how to use OpenRouter with GraphBit to access
multiple AI models through a unified API. OpenRouter provides access to
400+ models from various providers including OpenAI, Anthropic, Google,
Meta, Mistral, and many others.

Requirements:
- OPENROUTER_API_KEY environment variable
- GraphBit Python package

Usage:
    python simple_task_openrouter.py
"""

import os
import sys
from pathlib import Path

# Add the parent directory to the path to import graphbit
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

try:
    from graphbit import Executor, LlmConfig, Node, Workflow
except ImportError as e:
    print(f"Error importing graphbit: {e}")
    print("Make sure GraphBit is installed: pip install graphbit")
    sys.exit(1)


def check_openrouter_setup():
    """Check if OpenRouter is properly configured."""
    api_key = os.getenv("OPENROUTER_API_KEY")
    if not api_key:
        print("❌ OPENROUTER_API_KEY environment variable not found!")
        print("\nTo get started with OpenRouter:")
        print("1. Sign up at https://openrouter.ai/")
        print("2. Get your API key from https://openrouter.ai/keys")
        print("3. Set the environment variable:")
        print("   export OPENROUTER_API_KEY='your-api-key-here'")
        print("\nOpenRouter provides access to 400+ models including:")
        print("- OpenAI GPT models (gpt-4o, gpt-4o-mini)")
        print("- Anthropic Claude models (claude-3-5-sonnet, claude-3-5-haiku)")
        print("- Google Gemini models (gemini-pro-1.5)")
        print("- Meta Llama models (llama-3.1-405b-instruct)")
        print("- Mistral models (mistral-large)")
        print("- And many more!")
        return False

    print(f"✅ OpenRouter API key found: {api_key[:8]}...")
    return True


def demonstrate_basic_openrouter():
    """Demonstrate basic OpenRouter usage with different models."""
    print("\n🔄 Testing Basic OpenRouter Integration...")

    # Test with different models available through OpenRouter
    models_to_test = [
        ("openai/gpt-4o-mini", "OpenAI GPT-4o Mini"),
        ("anthropic/claude-3-5-haiku", "Anthropic Claude 3.5 Haiku"),
        ("meta-llama/llama-3.1-8b-instruct", "Meta Llama 3.1 8B"),
    ]

    for model_id, model_name in models_to_test:
        try:
            print(f"\n📝 Testing {model_name} ({model_id})...")

            # Configure OpenRouter with specific model
            config = LlmConfig.openrouter(api_key=os.getenv("OPENROUTER_API_KEY"), model=model_id)

            print(f"   Provider: {config.provider()}")
            print(f"   Model: {config.model()}")

            # Create a simple workflow
            workflow = Workflow(f"OpenRouter {model_name} Test")

            # Create a simple task
            task_node = Node.agent(
                name=f"{model_name} Assistant", prompt="Explain what makes artificial intelligence useful in 2-3 sentences.", agent_id=f"openrouter_{model_id.replace('/', '_').replace('-', '_')}"
            )

            workflow.add_node(task_node)
            workflow.validate()

            # Execute the workflow
            executor = Executor(config, debug=True)
            context = executor.execute(workflow)

            if context.is_completed():
                result = context.get_variable(f"node_result_{task_node.node_id}")
                print(f"   ✅ Response: {result}")
            else:
                print(f"   ❌ Failed: {context.error}")

        except Exception as e:
            print(f"   ❌ Error with {model_name}: {e}")
            continue


def demonstrate_multi_model_workflow():
    """Demonstrate using multiple models in a single workflow."""
    print("\n🔄 Testing Multi-Model Workflow...")

    try:
        # Create workflow with multiple models
        workflow = Workflow("Multi-Model Content Analysis")

        # Step 1: Use Claude for detailed analysis
        analyzer = Node.agent(
            name="Claude Analyzer",
            prompt="""Analyze this topic and provide:
            1. Key concepts and themes
            2. Current trends and developments
            3. Potential applications

            Topic: The future of renewable energy technology""",
            agent_id="claude_analyzer",
            llm_config=LlmConfig.openrouter(api_key=os.getenv("OPENROUTER_API_KEY"), model="anthropic/claude-3-5-haiku"),
        )

        # Step 2: Use GPT for summarization
        summarizer = Node.agent(
            name="GPT Summarizer",
            prompt="Create a concise 2-sentence summary of this analysis: {input}",
            agent_id="gpt_summarizer",
            llm_config=LlmConfig.openrouter(api_key=os.getenv("OPENROUTER_API_KEY"), model="openai/gpt-4o-mini"),
        )

        # Step 3: Use Llama for creative expansion
        creative_writer = Node.agent(
            name="Llama Creative Writer",
            prompt="Based on this summary, write a creative opening paragraph for a blog post: {input}",
            agent_id="llama_writer",
            llm_config=LlmConfig.openrouter(api_key=os.getenv("OPENROUTER_API_KEY"), model="meta-llama/llama-3.1-8b-instruct"),
        )

        # Connect the nodes
        workflow.add_node(analyzer)
        workflow.add_node(summarizer)
        workflow.add_node(creative_writer)
        workflow.add_edge(analyzer, summarizer)
        workflow.add_edge(summarizer, creative_writer)
        workflow.validate()

        # Execute with default OpenRouter config
        config = LlmConfig.openrouter(api_key=os.getenv("OPENROUTER_API_KEY"), model="anthropic/claude-3-5-haiku")  # Default model

        executor = Executor(config, debug=True)
        context = executor.execute(workflow)

        if context.is_completed():
            print("✅ Multi-model workflow completed successfully!")

            # Get results from each step
            analysis = context.get_variable(f"node_result_{analyzer.node_id}")
            summary = context.get_variable(f"node_result_{summarizer.node_id}")
            creative = context.get_variable(f"node_result_{creative_writer.node_id}")

            print(f"\n📊 Claude Analysis:\n{analysis}\n")
            print(f"📝 GPT Summary:\n{summary}\n")
            print(f"✨ Llama Creative Opening:\n{creative}\n")
        else:
            print(f"❌ Workflow failed: {context.error}")

    except Exception as e:
        print(f"❌ Multi-model workflow error: {e}")


def demonstrate_openrouter_with_site_info():
    """Demonstrate OpenRouter with site information for rankings."""
    print("\n🔄 Testing OpenRouter with Site Information...")

    try:
        # Configure OpenRouter with site information
        config = LlmConfig.openrouter_with_site(
            api_key=os.getenv("OPENROUTER_API_KEY"), model="openai/gpt-4o-mini", site_url="https://github.com/graphbit-ai/graphbit", site_name="GraphBit AI Framework"
        )

        print(f"   Provider: {config.provider()}")
        print(f"   Model: {config.model()}")

        # Create a simple workflow
        workflow = Workflow("OpenRouter with Site Info")

        task_node = Node.agent(name="Site-Aware Assistant", prompt="Explain the benefits of using a unified API for accessing multiple AI models.", agent_id="site_aware_assistant")

        workflow.add_node(task_node)
        workflow.validate()

        executor = Executor(config, debug=True)
        context = executor.execute(workflow)

        if context.is_completed():
            result = context.get_variable(f"node_result_{task_node.node_id}")
            print(f"   ✅ Response: {result}")
        else:
            print(f"   ❌ Failed: {context.error}")

    except Exception as e:
        print(f"❌ Site info demo error: {e}")


def main():
    """Run OpenRouter examples."""
    print("🚀 GraphBit OpenRouter Integration Examples")
    print("=" * 50)

    # Check setup
    if not check_openrouter_setup():
        return

    # Run examples
    demonstrate_basic_openrouter()
    demonstrate_multi_model_workflow()
    demonstrate_openrouter_with_site_info()

    print("\n✅ OpenRouter examples completed!")
    print("\nNext steps:")
    print("- Explore more models available on OpenRouter: https://openrouter.ai/models")
    print("- Check pricing and rate limits: https://openrouter.ai/docs/limits")
    print("- Build more complex workflows with GraphBit")


if __name__ == "__main__":
    main()
