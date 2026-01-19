#!/usr/bin/env python3
"""Test Perplexity streaming implementation"""

import asyncio
import os
from dotenv import load_dotenv
from graphbit import LlmConfig, LlmClient, init

# Load environment variables from .env
load_dotenv()

# Initialize GraphBit
init(debug=True)


async def test_perplexity_stream_async_iteration():
    """Test Perplexity streaming with async iteration"""
    print("=" * 60)
    print("TEST 1: Perplexity Streaming - Async Iteration")
    print("=" * 60)

    api_key = os.environ.get("PERPLEXITY_API_KEY")
    if not api_key:
        print("⚠️  PERPLEXITY_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.perplexity(api_key, "sonar")
    client = LlmClient(config, debug=True)

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


async def test_perplexity_stream_streaming_response():
    """Test Perplexity streaming with streaming_response() method"""
    print("\n" + "=" * 60)
    print("TEST 2: Perplexity Streaming - streaming_response()")
    print("=" * 60)

    api_key = os.environ.get("PERPLEXITY_API_KEY")
    if not api_key:
        print("⚠️  PERPLEXITY_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.perplexity(api_key, "sonar")
    client = LlmClient(config, debug=True)

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


async def test_perplexity_stream_longer_response():
    """Test Perplexity streaming with a longer response"""
    print("\n" + "=" * 60)
    print("TEST 3: Perplexity Streaming - Longer Response")
    print("=" * 60)

    api_key = os.environ.get("PERPLEXITY_API_KEY")
    if not api_key:
        print("⚠️  PERPLEXITY_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.perplexity(api_key, "sonar")
    client = LlmClient(config, debug=True)

    prompt = "Explain what Python is in 3 sentences."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response:")
    print("-" * 40)

    try:
        full_response = ""
        chunk_count = 0
        iterator = client.stream(prompt, max_tokens=200)
        async for chunk in iterator:
            print(chunk, end='', flush=True)
            full_response += chunk
            chunk_count += 1

        print("\n" + "-" * 40)
        print(f"Total chunks received: {chunk_count}")
        print(f"Total characters: {len(full_response)}")
        print(f"✅ PASS: Streaming worked correctly")
        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        return False


async def main():
    """Run all tests"""
    print("\n🚀 Testing Perplexity Streaming Implementation")
    print("=" * 60)

    results = []

    # Test 1: Async iteration
    results.append(("Async iteration", await test_perplexity_stream_async_iteration()))

    # Test 2: streaming_response()
    results.append(("streaming_response()", await test_perplexity_stream_streaming_response()))

    # Test 3: Longer response
    results.append(("Longer response", await test_perplexity_stream_longer_response()))

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
