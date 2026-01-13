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

    # Test TRUE streaming with async iteration
    prompt = "Explain what LangChain is in 3 sentences."
    print(f"Prompt: {prompt}\n")
    print("Streaming response (TRUE real-time):")
    print("-" * 60)

    # Use async for to get chunks as they arrive in REAL-TIME
    full_response = ""
    async for chunk in client.stream(prompt, max_tokens=200, temperature=0.7):
        print(chunk, end='', flush=True)
        full_response += chunk

    print("\n" + "-" * 60)
    print(f"\nFull streamed response:\n{full_response}")
    print("-" * 60)
    print("✅ TRUE streaming test completed!\n")

    # Also test the complete method for comparison
    print("=" * 60)
    print("Testing regular completion for comparison...")
    print("-" * 60)

    response = client.complete(prompt, max_tokens=200, temperature=0.7)
    print(f"Complete response:\n{response}")

    print("\n" + "=" * 60)
    print("All tests completed!")

if __name__ == "__main__":
    asyncio.run(test_streaming())
