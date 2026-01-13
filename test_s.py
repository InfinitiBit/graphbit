"""
Test script for streaming LLM responses
"""
import asyncio
import os
from graphbit import LlmConfig, LlmClient
from dotenv import load_dotenv

load_dotenv()

async def test_streaming():
    """Test the streaming functionality"""
    # Get API key from environment
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("Error: OPENAI_API_KEY environment variable not set")
        return

    # Create configuration
    config = LlmConfig.openai(api_key, "gpt-4o-mini")

    # Create client
    client = LlmClient(config, debug=True)

    print("Testing streaming completion...")
    print("-" * 60)

    # Test streaming with the new streaming_response() method
    prompt = "What is LangChain?"
    prompt2 = "What is Google?"
    print(f"Prompt: {prompt}\n")
    print("Streaming response (TRUE real-time):")
    print("-" * 60)

    # Get the stream iterator
    chunks = client.stream(prompt, temperature=0.7)
    chunks2  = client.stream(prompt2, temperature=0.7)

    # Use streaming_response() to print in real-time and get full response
    full_response = await chunks.streaming_response()

    print("\n" + "-" * 60)
    print(f"\nFull response returned:\n{full_response}")
    print("-" * 60)
    print("✅ Streaming test completed!\n")

    print(f"Prompt: {prompt2}\n")
    print("Streaming response (TRUE real-time):")
    print("-" * 60)

    # Use streaming_response() to print in real-time and get full response
    full_response2 = await chunks2.streaming_response()

    print("\n" + "-" * 60)
    print(f"\nFull response returned:\n{full_response2}")
    print("-" * 60)
    print("✅ Streaming test completed!\n")

if __name__ == "__main__":
    asyncio.run(test_streaming())
