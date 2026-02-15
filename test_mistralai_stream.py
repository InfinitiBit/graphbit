#!/usr/bin/env python3
"""Test MistralAI streaming implementation"""

import asyncio
import os
import time
from dotenv import load_dotenv
from graphbit import LlmConfig, LlmClient, init

# Load environment variables from .env
load_dotenv()

# Initialize GraphBit
init(debug=True)


async def test_mistralai_stream_async_iteration():
    """Test MistralAI streaming with async iteration"""
    print("=" * 60)
    print("TEST 1: MistralAI Streaming - Async Iteration")
    print("=" * 60)

    api_key = os.environ.get("MISTRALAI_API_KEY")
    if not api_key:
        print("⚠️  MISTRALAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.mistralai(api_key, "mistral-small-latest")
    client = LlmClient(config, debug=True)

    prompt = "What is the speed of light? Answer in one sentence."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response:")
    print("-" * 40)

    try:
        full_response = ""
        chunk_count = 0
        chunk_times = []
        start_time = time.time()

        iterator = client.stream(prompt, max_tokens=100)
        async for chunk in iterator:
            chunk_time = time.time() - start_time
            chunk_times.append(chunk_time)
            print(chunk, end='', flush=True)
            full_response += chunk
            chunk_count += 1

        elapsed = time.time() - start_time
        print("\n" + "-" * 40)
        print(f"✅ PASS: Received {len(full_response)} characters in {chunk_count} chunks")
        print(f"   Total time: {elapsed:.2f}s")

        if len(chunk_times) >= 2:
            deltas = [chunk_times[i] - chunk_times[i-1] for i in range(1, len(chunk_times))]
            avg_delta = sum(deltas) / len(deltas)
            print(f"   Avg inter-chunk delay: {avg_delta*1000:.1f}ms")
            print(f"   First chunk at: {chunk_times[0]*1000:.0f}ms")

        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        import traceback
        traceback.print_exc()
        return False


async def test_mistralai_stream_streaming_response():
    """Test MistralAI streaming with streaming_response() method"""
    print("\n" + "=" * 60)
    print("TEST 2: MistralAI Streaming - streaming_response()")
    print("=" * 60)

    api_key = os.environ.get("MISTRALAI_API_KEY")
    if not api_key:
        print("⚠️  MISTRALAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.mistralai(api_key, "mistral-small-latest")
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
        import traceback
        traceback.print_exc()
        return False


async def test_mistralai_stream_longer_response():
    """Test MistralAI streaming with a longer response and timing analysis"""
    print("\n" + "=" * 60)
    print("TEST 3: MistralAI Streaming - Longer Response + Timing")
    print("=" * 60)

    api_key = os.environ.get("MISTRALAI_API_KEY")
    if not api_key:
        print("⚠️  MISTRALAI_API_KEY not set. Skipping test.")
        return None

    config = LlmConfig.mistralai(api_key, "mistral-small-latest")
    client = LlmClient(config, debug=True)

    prompt = "Explain what Python is in 3 sentences."
    print(f"\nPrompt: {prompt}")
    print("\nStreaming response:")
    print("-" * 40)

    try:
        full_response = ""
        chunk_count = 0
        chunk_times = []
        start_time = time.time()

        iterator = client.stream(prompt, max_tokens=200)
        async for chunk in iterator:
            chunk_time = time.time() - start_time
            chunk_times.append(chunk_time)
            print(chunk, end='', flush=True)
            full_response += chunk
            chunk_count += 1

        elapsed = time.time() - start_time
        print("\n" + "-" * 40)
        print(f"Total chunks received: {chunk_count}")
        print(f"Total characters: {len(full_response)}")
        print(f"Total time: {elapsed:.2f}s")

        if len(chunk_times) >= 2:
            deltas = [chunk_times[i] - chunk_times[i-1] for i in range(1, len(chunk_times))]
            avg_delta = sum(deltas) / len(deltas)
            min_delta = min(deltas)
            max_delta = max(deltas)
            print(f"\n📊 Streaming Timing Analysis:")
            print(f"   First chunk latency: {chunk_times[0]*1000:.0f}ms")
            print(f"   Avg inter-chunk delay: {avg_delta*1000:.1f}ms")
            print(f"   Min inter-chunk delay: {min_delta*1000:.1f}ms")
            print(f"   Max inter-chunk delay: {max_delta*1000:.1f}ms")

            # Verify true streaming (not buffered)
            if chunk_count > 5 and avg_delta < 2000:
                print(f"   ✅ TRUE STREAMING confirmed: {chunk_count} chunks with avg {avg_delta*1000:.1f}ms gap")
            elif chunk_count <= 5:
                print(f"   ⚠️  Low chunk count ({chunk_count}), may indicate buffering")
            else:
                print(f"   ⚠️  High avg delay ({avg_delta*1000:.1f}ms), may indicate issues")

        print(f"✅ PASS: Streaming worked correctly")
        return True
    except Exception as e:
        print(f"\n❌ FAIL: {e}")
        import traceback
        traceback.print_exc()
        return False


async def main():
    """Run all tests"""
    print("\n🚀 Testing MistralAI Streaming Implementation")
    print("=" * 60)
    print("NOTE: Requires MISTRALAI_API_KEY in .env file")
    print("      Using model: mistral-small-latest")
    print("=" * 60)

    results = []

    # Test 1: Async iteration
    results.append(("Async iteration", await test_mistralai_stream_async_iteration()))

    # Test 2: streaming_response()
    results.append(("streaming_response()", await test_mistralai_stream_streaming_response()))

    # Test 3: Longer response with timing
    results.append(("Longer response + timing", await test_mistralai_stream_longer_response()))

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
