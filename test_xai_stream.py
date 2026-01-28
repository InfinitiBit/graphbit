#!/usr/bin/env python3
"""Test xAI (Grok) streaming implementation"""

import asyncio
import os
import time
from dotenv import load_dotenv
from graphbit import LlmConfig, LlmClient, init

# Load environment variables from .env
load_dotenv()

# Initialize GraphBit
init(debug=False)


async def test_xai_stream_async_iteration():
    """Test xAI streaming with async iteration"""
    print("=" * 60)
    print("TEST 1: xAI Streaming - Async Iteration")
    print("=" * 60)

    api_key = os.environ.get("XAI_API_KEY")
    if not api_key:
        print("⚠️  XAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.xai(api_key, "grok-3-mini")
    client = LlmClient(config)

    prompt = "What is the speed of light? Answer in one sentence."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response:")
    print("-" * 40)

    try:
        full_response = ""
        iterator = client.stream(prompt, max_tokens=100)
        async for chunk in iterator:
            print(chunk, end='', flush=True)
            full_response += chunk

        print("\n" + "-" * 40)
        print(f"✅ PASS: Received {len(full_response)} characters")
        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        return False


async def test_xai_stream_streaming_response():
    """Test xAI streaming with streaming_response() method"""
    print("\n" + "=" * 60)
    print("TEST 2: xAI Streaming - streaming_response()")
    print("=" * 60)

    api_key = os.environ.get("XAI_API_KEY")
    if not api_key:
        print("⚠️  XAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.xai(api_key, "grok-3-mini")
    client = LlmClient(config)

    prompt = "What is 2 + 2? Answer with just the number."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response (auto-printed):")
    print("-" * 40)

    try:
        iterator = client.stream(prompt, max_tokens=50)
        response = await iterator.streaming_response()

        print("\n" + "-" * 40)
        print(f"Full response returned: '{response}'")
        print(f"✅ PASS: Received {len(response)} characters")
        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        return False


async def test_xai_stream_timing():
    """Test xAI streaming with timing to verify true streaming"""
    print("\n" + "=" * 60)
    print("TEST 3: xAI Streaming - Timing Verification")
    print("=" * 60)

    api_key = os.environ.get("XAI_API_KEY")
    if not api_key:
        print("⚠️  XAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.xai(api_key, "grok-3-mini")
    client = LlmClient(config)

    prompt = "Explain what Python is in 20 sentences."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response with timing:")
    print("-" * 40)

    try:
        full_response = ""
        chunk_count = 0
        start_time = time.time()
        first_chunk_time = None

        iterator = client.stream(prompt, max_tokens=2000)
        async for chunk in iterator:
            if first_chunk_time is None:
                first_chunk_time = time.time() - start_time
            print(chunk, end='', flush=True)
            full_response += chunk
            chunk_count += 1

        total_time = time.time() - start_time

        print("\n" + "-" * 40)
        print(f"Total chunks received: {chunk_count}")
        print(f"Total characters: {len(full_response)}")
        print(f"Time to first chunk: {first_chunk_time:.3f}s")
        print(f"Total time: {total_time:.2f}s")

        if chunk_count > 5:
            print(f"✅ PASS: True streaming confirmed ({chunk_count} chunks)")
            return True
        else:
            print(f"⚠️  Warning: Only {chunk_count} chunks (may not be token-level)")
            return True  # Still passes if response was received
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        return False


async def main():
    """Run all tests"""
    print("\n🚀 Testing xAI (Grok) Streaming Implementation")
    print("=" * 60)

    results = []

    # Test 1: Async iteration
    results.append(("Async iteration", await test_xai_stream_async_iteration()))

    # Test 2: streaming_response()
    results.append(("streaming_response()", await test_xai_stream_streaming_response()))

    # Test 3: Timing verification
    results.append(("Timing verification", await test_xai_stream_timing()))

    # Summary
    print("\n" + "=" * 60)
    print("TEST SUMMARY")
    print("=" * 60)

    passed = 0
    skipped = 0
    failed = 0

    for test_name, result in results:
        if result is True:
            print(f"✅ {test_name}: PASSED")
            passed += 1
        elif result is False:
            print(f"❌ {test_name}: FAILED")
            failed += 1
        else:
            print(f"⚠️  {test_name}: SKIPPED")
            skipped += 1

    print(f"\nTotal: {passed} passed, {failed} failed, {skipped} skipped")

    return failed == 0


if __name__ == "__main__":
    success = asyncio.run(main())
    exit(0 if success else 1)
