#!/usr/bin/env python3
"""Test Anthropic streaming implementation"""

import asyncio
import os
from dotenv import load_dotenv
from graphbit import LlmConfig, LlmClient, init

# Load environment variables from .env
load_dotenv()

# Initialize GraphBit
init(debug=True)

async def test_anthropic_stream_async_iteration():
    """Test Anthropic streaming with async iteration"""
    print("=" * 60)
    print("TEST 1: Anthropic Streaming - Async Iteration")
    print("=" * 60)

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("⚠️  ANTHROPIC_API_KEY not set. Skipping test.")
        return False

    config = LlmConfig.anthropic(api_key, "claude-3-haiku-20240307")
    client = LlmClient(config, debug=True)

    prompt = "Write a haiku about programming. Just the haiku, no explanation."
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


async def test_anthropic_stream_streaming_response():
    """Test Anthropic streaming with streaming_response() method"""
    print("\n" + "=" * 60)
    print("TEST 2: Anthropic Streaming - streaming_response()")
    print("=" * 60)

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("⚠️  ANTHROPIC_API_KEY not set. Skipping test.")
        return False

    config = LlmConfig.anthropic(api_key, "claude-3-haiku-20240307")
    client = LlmClient(config, debug=True)

    prompt = "What is 2 + 2? Answer in one word."
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


async def test_openai_stream_for_comparison():
    """Test OpenAI streaming for comparison"""
    print("\n" + "=" * 60)
    print("TEST 3: OpenAI Streaming (for comparison)")
    print("=" * 60)

    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        print("⚠️  OPENAI_API_KEY not set. Skipping test.")
        return False

    config = LlmConfig.openai(api_key, "gpt-4o-mini")
    client = LlmClient(config, debug=True)

    prompt = "Write a haiku about programming. Just the haiku, no explanation."
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


async def main():
    """Run all tests"""
    print("\n🚀 Testing Anthropic Streaming Implementation")
    print("=" * 60)

    results = []

    # Test 1: Async iteration
    results.append(("Anthropic async iteration", await test_anthropic_stream_async_iteration()))

    # Test 2: streaming_response()
    results.append(("Anthropic streaming_response()", await test_anthropic_stream_streaming_response()))

    # Test 3: OpenAI for comparison (if API key available)
    results.append(("OpenAI comparison", await test_openai_stream_for_comparison()))

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
