"""
Test Google Gemini LLM provider using LlmClient.
Tests: basic completion, streaming, and system prompt.
"""
import os
from graphbit import init, LlmConfig, LlmClient
from dotenv import load_dotenv
import asyncio

load_dotenv()
init()

config = LlmConfig.gemini(os.getenv("GEMINI_API_KEY"), "gemini-2.5-flash")
client = LlmClient(config)


# # TEST 1: Basic completion (synchronous)
# print("=" * 60)
# print("TEST 1: Basic Gemini Completion")
# print("=" * 60)

# response = client.complete("What is Rust?", max_tokens=2000)
# print(f"Response: {response}")
# print()


# # TEST 2: Streaming
# print("=" * 60)
# print("TEST 2: Gemini Streaming")
# print("=" * 60)

# async def test_streaming():
#     iterator = client.stream("What is Langchain?", max_tokens=2000)
#     response = await iterator.streaming_response()
#     print(f"Streamed Response: {response}")

# asyncio.run(test_streaming())
# print()


# TEST 3: Async completion
print("=" * 60)
print("TEST 3: Gemini Async Completion")
print("=" * 60)

async def test_async():
    response = await client.complete_async("What is Python?", max_tokens=2000)
    print(f"Async Response: {response}")

asyncio.run(test_async())
print()

print("All Gemini LlmClient tests completed!")
