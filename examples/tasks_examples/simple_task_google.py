#!/usr/bin/env python3
"""
Simple task example using Google Gemini models.

This example demonstrates how to configure and use Google Gemini models
with GraphBit for basic text generation tasks.

Requirements:
- Set GOOGLE_API_KEY environment variable
- Install GraphBit: pip install graphbit

Usage:
    python simple_task_google.py
"""


import os
import sys
from pathlib import Path

# Add the parent directory to the path to import graphbit
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "python"))

from graphbit import LlmConfig, LlmClient, Node, Workflow, Executor

# Try to load environment variables from .env file
try:
    from dotenv import load_dotenv
    env_path = Path(__file__).parent.parent / "content-creation" / ".env"
    if env_path.exists():
        load_dotenv(env_path)
        print(f"✅ Loaded environment variables from {env_path}")
except ImportError:
    print("💡 Install python-dotenv to automatically load .env files: pip install python-dotenv")


def main():
    """Main function demonstrating Google Gemini integration."""
    
    # Check for API key
    api_key = os.getenv("GOOGLE_API_KEY")
    if not api_key:
        print("❌ Error: GOOGLE_API_KEY environment variable not set")
        print("Please set your Google API key:")
        print("export GOOGLE_API_KEY='your-google-api-key-here'")
        print("\nGet your API key at: https://aistudio.google.com/")
        return

    print("🚀 GraphBit Google Gemini Integration Example")
    print("=" * 50)

    # Example 1: Basic configuration with default model (gemini-2.5-flash)
    print("\n📝 Example 1: Basic Google Gemini Configuration")
    config_basic = LlmConfig.google(
        api_key=api_key
    )
    print(f"Provider: {config_basic.provider()}")
    print(f"Model: {config_basic.model()}")

    # Example 2: Configuration with specific model
    print("\n📝 Example 2: Google Gemini with Specific Model")
    config_pro = LlmConfig.google(
        api_key=api_key,
        model="gemini-2.5-pro"
    )
    print(f"Provider: {config_pro.provider()}")
    print(f"Model: {config_pro.model()}")

    # Example 3: Create and run a simple task
    print("\n📝 Example 3: Running a Simple Task")

    # Create an LLM client with Google Gemini
    client = LlmClient(config_basic)

    try:
        print("🤖 Generating creative story with Google Gemini...")
        result = client.complete("Write a short creative story about a robot learning to paint")

        print("\n✅ Task completed successfully!")
        print(f"📖 Generated Story:\n{result}")

    except Exception as e:
        print(f"❌ Error executing task: {e}")
        return

    # Example 4: Different models comparison
    print("\n📝 Example 4: Comparing Different Gemini Models")

    models_to_test = [
        ("gemini-2.5-flash", "Fast and efficient for general tasks"),
        ("gemini-2.5-pro", "High-quality reasoning and analysis"),
        ("gemini-1.5-flash", "Balanced performance and speed"),
    ]

    prompt = "Explain quantum computing in one sentence."

    for model_name, description in models_to_test:
        print(f"\n🔬 Testing {model_name} ({description})")

        # Create configuration for this model
        model_config = LlmConfig.google(
            api_key=api_key,
            model=model_name
        )

        # Create client for this model
        model_client = LlmClient(model_config)

        try:
            result = model_client.complete(prompt)
            print(f"📝 Response: {result}")
        except Exception as e:
            print(f"❌ Error with {model_name}: {e}")

    print("\n🎉 Google Gemini integration examples completed!")
    print("\n💡 Tips for using Google Gemini:")
    print("  - Use gemini-2.5-flash for fast, general-purpose tasks")
    print("  - Use gemini-2.5-pro for complex reasoning and analysis")
    print("  - Use gemini-1.5-pro for very long documents (2M token context)")
    print("  - All models support multimodal inputs (text, images, etc.)")
    print("  - Context lengths range from 32K to 2M tokens depending on model")


if __name__ == "__main__":
    main()
