#!/usr/bin/env python3
"""
Test script for DeepSeek streaming implementation.
This script tests the streaming functionality of DeepSeek chat models.
"""

import asyncio
import os
import time

from dotenv import load_dotenv

from graphbit import LlmClient, LlmConfig

# Load environment variables
load_dotenv()


async def test_deepseek_streaming():
    """Test DeepSeek streaming with timing analysis"""

    # Get API key from environment
    api_key = os.getenv("DEEPSEEK_API_KEY")
    if not api_key:
        print("❌ ERROR: DEEPSEEK_API_KEY not found in .env file")
        return

    print("=" * 80)
    print("DeepSeek Streaming Test")
    print("=" * 80)

    # Initialize DeepSeek LLM client
    config = LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
    client = LlmClient(config)

    # Test prompt
    prompt = "Write a short poem about artificial intelligence in 4 lines."

    print(f"\n📝 Prompt: {prompt}")
    print("\n" + "=" * 80)
    print("🔄 Streaming Response:")
    print("-" * 80)

    # Track timing for each chunk
    start_time = time.time()
    first_chunk_time = None
    chunk_count = 0
    total_content = ""
    chunk_times = []

    try:
        # Stream the response - use async for iteration
        stream = client.stream(prompt, max_tokens=500)

        async for chunk in stream:
            current_time = time.time()

            # Record first chunk time (important for true streaming verification)
            if first_chunk_time is None:
                first_chunk_time = current_time - start_time
                print(f"\n⏱️  First chunk received after: {first_chunk_time:.3f}s")
                print("-" * 80)

            # Track chunk timing
            chunk_count += 1
            chunk_times.append(current_time)

            # Print the chunk content
            total_content += chunk
            print(chunk, end="", flush=True)

        # Calculate statistics
        end_time = time.time()
        total_time = end_time - start_time

        print("\n" + "=" * 80)
        print("\n📊 Streaming Statistics:")
        print(f"  • Total chunks received: {chunk_count}")
        print(f"  • Total characters: {len(total_content)}")
        print(f"  • First chunk latency: {first_chunk_time:.3f}s" if first_chunk_time else "  • No chunks received")
        print(f"  • Total time: {total_time:.3f}s")

        if chunk_count > 1:
            # Calculate inter-chunk timings
            inter_chunk_times = []
            for i in range(1, len(chunk_times)):
                inter_chunk_times.append(chunk_times[i] - chunk_times[i - 1])

            avg_inter_chunk = sum(inter_chunk_times) / len(inter_chunk_times)
            print(f"  • Average inter-chunk time: {avg_inter_chunk:.3f}s")
            print(f"  • Min inter-chunk time: {min(inter_chunk_times):.3f}s")
            print(f"  • Max inter-chunk time: {max(inter_chunk_times):.3f}s")

        # Verify true streaming (first chunk should arrive much faster than total time)
        if first_chunk_time and total_time > 0:
            streaming_ratio = first_chunk_time / total_time
            print(f"\n✅ Streaming verification:")
            print(f"  • First chunk ratio: {streaming_ratio:.2%} of total time")
            if streaming_ratio < 0.5 and chunk_count > 3:
                print(f"  • ✅ TRUE STREAMING: First chunk arrived at {streaming_ratio:.2%} of total time")
                print(f"  • This confirms real-time, token-by-token streaming!")
            else:
                print(f"  • ⚠️  Possible buffering detected (ratio: {streaming_ratio:.2%})")

    except Exception as e:
        print(f"\n❌ ERROR during streaming: {e}")
        import traceback

        traceback.print_exc()
        return

    print("\n" + "=" * 80)


async def test_deepseek_stream_streaming_response():
    """Test xAI streaming with streaming_response() method"""
    print("\n" + "=" * 60)
    print("TEST 2: xAI Streaming - streaming_response()")
    print("=" * 60)

    api_key = os.environ.get("DEEPSEEK_API_KEY")
    if not api_key:
        print("⚠️  DEEPSEEK_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.deepseek(api_key, "deepseek-chat")
    client = LlmClient(config)

    prompt = "what is langchain? Be comprehensive."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response (auto-printed):")
    print("-" * 40)

    try:
        iterator = client.stream(prompt, max_tokens=500)
        response = await iterator.streaming_response()

        print("\n" + "-" * 40)
        print(f"Full response returned: '{response}'")
        print(f"✅ PASS: Received {len(response)} characters")
        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        return False


def test_deepseek_non_streaming():
    """Test DeepSeek non-streaming for comparison"""

    api_key = os.getenv("DEEPSEEK_API_KEY")
    if not api_key:
        print("❌ ERROR: DEEPSEEK_API_KEY not found in .env file")
        return

    print("\n" + "=" * 80)
    print("DeepSeek Non-Streaming Test (for comparison)")
    print("=" * 80)

    config = LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
    client = LlmClient(config)

    prompt = "Write a short poem about artificial intelligence in 4 lines."

    print(f"\n📝 Prompt: {prompt}")
    print("\n🔄 Non-Streaming Response:")
    print("-" * 80)

    start_time = time.time()

    try:
        response = client.complete(prompt, max_tokens=500)
        end_time = time.time()

        # complete() returns a string directly
        print(response)
        print("-" * 80)
        print(f"\n⏱️  Total time: {end_time - start_time:.3f}s")
        print(f"📊 Response length: {len(response)} characters")

    except Exception as e:
        print(f"\n❌ ERROR: {e}")
        import traceback

        traceback.print_exc()

    print("\n" + "=" * 80)


async def main():
    """Run both streaming and non-streaming tests"""

    # Test streaming
    await test_deepseek_streaming()

    # Wait a bit between tests
    await asyncio.sleep(2)

    await test_deepseek_stream_streaming_response()

    await asyncio.sleep(2)

    # Test non-streaming for comparison
    test_deepseek_non_streaming()

    print("\n✅ All tests completed!")


if __name__ == "__main__":
    asyncio.run(main())
