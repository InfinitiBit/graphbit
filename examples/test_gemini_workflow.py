"""
Test Google Gemini LLM provider using Node.agent Workflow.
Tests: agent workflow execution with Gemini as the LLM provider.
"""
import os
import asyncio
from graphbit import init, LlmConfig, Node, Workflow, Executor, tool
from dotenv import load_dotenv

load_dotenv()
init()


@tool(_description="Calculate the sum of two numbers")
def add(a: int, b: int) -> int:
    return a + b


@tool(_description="Calculate the product of two numbers")
def multiply(a: int, b: int) -> int:
    return a * b


async def test_simple_agent():
    """Test Gemini with a simple agent (no tools)."""
    print("=" * 60)
    print("TEST 1: Simple Gemini Agent (no tools)")
    print("=" * 60)

    config = LlmConfig.gemini(os.getenv("GEMINI_API_KEY"), "gemini-2.5-flash")

    agent = Node.agent(
        name="poet",
        prompt="Write a haiku about programming.",
        system_prompt="You are a poet. Write short, beautiful poems.",
    )

    workflow = Workflow("Test")
    workflow.add_node(agent)

    executor = Executor(config)
    result = executor.execute(workflow)

    print(f"Poem: {result.get_node_output("poet")}")
    print()


async def test_agent_with_tools():
    """Test Gemini with agent workflow and tool calling."""
    print("=" * 60)
    print("TEST 2: Gemini Agent Workflow with Tool Calling")
    print("=" * 60)

    config = LlmConfig.gemini(os.getenv("GEMINI_API_KEY"), "gemini-2.5-flash")

    agent = Node.agent(
        name="math_agent",
        prompt="What is 15 + 27? And what is 6 * 8? Give me both results.",
        system_prompt="You are a helpful math assistant. Use the provided tools to solve math problems. Always use the tools for calculations.",
        tools=[add, multiply],
    )

    workflow = Workflow("Test")
    workflow.add_node(agent)

    executor = Executor(config)
    result = executor.execute(workflow)

    print(f"Agent Response: {result.get_node_output("math_agent")}")
    print()


async def main():
    print("Testing Google Gemini with Node.agent Workflow")
    print("=" * 60)

    await test_simple_agent()
    await test_agent_with_tools()

    print("All Gemini Workflow tests completed!")


if __name__ == "__main__":
    asyncio.run(main())
