"""
Shared test helpers for iterative agent loop provider tests.

Provides common tools, test runner, and standardized test cases
that each provider test file can use.
"""

import sys
import time
import traceback
from graphbit import Node, Workflow, Executor, tool


# ── Tool definitions ──────────────────────────────────────────────────────────

@tool(_description="Calculate the sum of two numbers")
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b


@tool(_description="Calculate the product of two numbers")
def multiply(a: int, b: int) -> int:
    """Multiply two numbers together."""
    return a * b


@tool(_description="Get the weather in a city")
def get_weather(city: str) -> str:
    """Return mock weather for a city."""
    weather = {
        "dhaka": "Sunny, 32°C",
        "tokyo": "Cloudy, 18°C",
        "london": "Rainy, 12°C",
        "new york": "Clear, 25°C",
    }
    return weather.get(city.lower(), f"Weather unavailable for {city}")


@tool(_description="Look up the capital city of a country")
def get_capital(country: str) -> str:
    """Return the capital city of a country."""
    capitals = {
        "bangladesh": "Dhaka",
        "japan": "Tokyo",
        "usa": "Washington D.C.",
        "india": "New Delhi",
        "uk": "London",
    }
    return capitals.get(country.lower(), f"Capital not found for {country}")


@tool(_description="Look up the population of a country")
def get_population(country: str) -> str:
    """Return mock population data for a country."""
    populations = {
        "bangladesh": "170 million",
        "japan": "125 million",
        "usa": "335 million",
        "india": "1.4 billion",
    }
    return populations.get(country.lower(), f"Population data not available for {country}")


# ── Test runner ───────────────────────────────────────────────────────────────

class TestRunner:
    """Run standardized tests for a provider."""

    def __init__(self, provider_name: str, config, delay_between_tests: float = 1.0):
        self.provider_name = provider_name
        self.executor = Executor(config)
        self.delay = delay_between_tests
        self.passed = 0
        self.failed = 0
        self.errors = []

    def run_test(self, test_name, test_func):
        """Run a single test with standard formatting."""
        print(f"\n{'=' * 60}")
        print(f"TEST: {test_name}")
        print(f"{'=' * 60}")
        try:
            test_func()
            self.passed += 1
            print(f"\n✅ PASSED: {test_name}")
        except Exception as e:
            self.failed += 1
            error_msg = str(e)
            self.errors.append((test_name, error_msg))
            print(f"\n❌ FAILED: {test_name}")
            print(f"   Error: {error_msg}")
            traceback.print_exc()
        if self.delay > 0:
            time.sleep(self.delay)

    def print_summary(self):
        """Print final test summary."""
        total = self.passed + self.failed
        print(f"\n{'=' * 60}")
        status = "ALL PASSED ✅" if self.failed == 0 else f"{self.failed} FAILED ❌"
        print(f"[{self.provider_name}] {self.passed}/{total} {status}")
        if self.errors:
            print(f"\nFailed tests:")
            for name, err in self.errors:
                print(f"  ❌ {name}: {err[:100]}")
        print(f"{'=' * 60}")
        return self.failed == 0

    # ── Standard test cases ───────────────────────────────────────────────

    def test_single_tool_call(self):
        """Test 1: Single tool call (add 5 + 3 = 8)."""
        def run():
            workflow = Workflow(f"{self.provider_name}Single")
            agent = Node.agent(
                name="Calc",
                prompt="Calculate 5 + 3 STRICTLY using the available add tool. Return ONLY the numeric result.",
                tools=[add],
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("Calc")
            metadata = result.get_node_response_metadata("Calc")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            print(f"  Tool calls: {len(metadata.get('tools_used', []))}")
            # print(metadata)
            assert "8" in str(output), f"Expected '8' in output, got: {output}"
            assert metadata.get('total_iterations', 0) >= 1, "Expected at least 1 iteration"
        self.run_test("Single Tool Call (add 5+3=8)", run)

    def test_multi_step_chain(self):
        """Test 2: Sequential tool chaining (add → multiply)."""
        def run():
            workflow = Workflow(f"{self.provider_name}Chain")
            agent = Node.agent(
                name="MathChain",
                prompt=(
                    "First, add 10 + 20 using the add tool. "
                    "Then, multiply the result by 3 using the multiply tool. "
                    "Report the final answer."
                ),
                tools=[add, multiply],
                max_iterations=5,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("MathChain")
            metadata = result.get_node_response_metadata("MathChain")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            tools_used = metadata.get('tools_used', [])
            print(f"  Tools used: {tools_used}")
            # for tc in tools_used:
            #     print(f"    iter {tc['iteration']}: {tc['tool_name']}({tc['parameters']}) → {tc['output']}")
            assert "90" in str(output), f"Expected '90' in output, got: {output}"
            assert metadata.get('total_iterations', 0) >= 2, "Expected at least 2 iterations"
        self.run_test("Multi-Step Chain (add→multiply = 90)", run)

    def test_parallel_tools(self):
        """Test 3: Multiple tools potentially called in one iteration."""
        def run():
            workflow = Workflow(f"{self.provider_name}Parallel")
            agent = Node.agent(
                name="Researcher",
                prompt=(
                    "Find two things about Japan: "
                    "1. Capital city (use get_capital) "
                    "2. Population (use get_population) "
                    "Summarize both."
                ),
                tools=[get_capital, get_population],
                max_iterations=5,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("Researcher")
            metadata = result.get_node_response_metadata("Researcher")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            tools_used = metadata.get('tools_used', [])
            print(f"  Tools used: {tools_used}")
            # for tc in tools_used:
            #     print(f"    iter {tc['iteration']}: {tc['tool_name']}({tc['parameters']}) → {tc['output']}")
            assert "Tokyo" in str(output), f"Expected 'Tokyo' in output, got: {output}"
            assert "125" in str(output) or "million" in str(output).lower(), \
                f"Expected population info, got: {output}"
        self.run_test("Parallel/Multi Tools (capital + population)", run)

    def test_three_step_chain(self):
        """Test 4: Three-step sequential chaining (add→add→multiply = 80)."""
        def run():
            workflow = Workflow(f"{self.provider_name}ThreeStep")
            agent = Node.agent(
                name="ThreeStep",
                prompt=(
                    "Use tools step by step:\n"
                    "Step 1: Add 5 + 10\n"
                    "Step 2: Add the result to 25\n"
                    "Step 3: Multiply the result by 2\n"
                    "Report the final answer."
                ),
                tools=[add, multiply],
                max_iterations=10,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("ThreeStep")
            metadata = result.get_node_response_metadata("ThreeStep")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            tools_used = metadata.get('tools_used', [])
            print(f"  Tools used: {tools_used}")
            # for tc in tools_used:
            #     print(f"    iter {tc['iteration']}: {tc['tool_name']}({tc['parameters']}) → {tc['output']}")
            assert "80" in str(output), f"Expected '80' in output, got: {output}"
            assert metadata.get('total_iterations', 0) >= 3, "Expected at least 3 iterations"
        self.run_test("Three-Step Chain (5+10→+25→×2 = 80)", run)

    def test_max_iterations_1(self):
        """Test 5: max_iterations=1 boundary — stops after 1 iteration."""
        def run():
            workflow = Workflow(f"{self.provider_name}MaxIter1")
            agent = Node.agent(
                name="Limited",
                prompt="Add 7 + 8 using the add tool.",
                tools=[add],
                max_iterations=1,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("Limited")
            metadata = result.get_node_response_metadata("Limited")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            print(f"  Max iterations: {metadata.get('max_iterations')}")
            assert metadata.get('total_iterations') == 1, \
                f"Expected total_iterations=1, got: {metadata.get('total_iterations')}"
            assert metadata.get('max_iterations') == 1, \
                f"Expected max_iterations=1, got: {metadata.get('max_iterations')}"
        self.run_test("max_iterations=1 Boundary", run)

    def test_no_tool_needed(self):
        """Test 6: Prompt that doesn't need tools — should respond directly."""
        def run():
            workflow = Workflow(f"{self.provider_name}NoTool")
            agent = Node.agent(
                name="Direct",
                prompt="What is 2 + 2? Answer directly without using tools.",
                tools=[add],
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("Direct")
            metadata = result.get_node_response_metadata("Direct")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            print(f"  Tool calls: {len(metadata.get('tools_used', []))}")
            assert "4" in str(output), f"Expected '4' in output, got: {output}"
        self.run_test("No Tool Needed (direct answer)", run)

    def test_conditional_tool_use(self):
        """Test 7: Conditional — add first, then weather if > 40."""
        def run():
            workflow = Workflow(f"{self.provider_name}Conditional")
            agent = Node.agent(
                name="ConditionalAgent",
                prompt=(
                    "Add 20 + 30 using the add tool. "
                    "If the sum is greater than 40, get the weather in Dhaka. "
                    "Report the final result."
                ),
                tools=[add, get_weather],
                max_iterations=5,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            output = result.get_node_output("ConditionalAgent")
            metadata = result.get_node_response_metadata("ConditionalAgent")
            print(f"  Output: {output}")
            print(f"  Iterations: {metadata.get('total_iterations')}")
            tools_used = metadata.get('tools_used', [])
            print(f"  Tools used: {tools_used}")
            # for tc in tools_used:
            #     print(f"    iter {tc['iteration']}: {tc['tool_name']}({tc['parameters']}) → {tc['output']}")
            assert len(tools_used) >= 2, f"Expected at least 2 tools used, got: {len(tools_used)}"
            # tool_names = [tc['tool_name'] for tc in tools_used]
            assert "add" in tools_used, "Expected 'add' tool to be used"
            assert "get_weather" in tools_used, "Expected 'get_weather' tool to be used"
        self.run_test("Conditional Tool Use (add→weather if >40)", run)

    def test_metadata_accuracy(self):
        """Test 8: Verify metadata fields are correctly populated."""
        def run():
            workflow = Workflow(f"{self.provider_name}Meta")
            agent = Node.agent(
                name="MetaAgent",
                prompt="Add 3 + 4 using the add tool.",
                tools=[add],
                max_iterations=5,
            )
            workflow.add_node(agent)
            result = self.executor.execute(workflow)
            metadata = result.get_node_response_metadata("MetaAgent")
            print(f"  Metadata keys: {list(metadata.keys()) if metadata else 'None'}")

            # Check required metadata fields
            assert metadata is not None, "Metadata should not be None"
            assert 'total_iterations' in metadata, "Missing 'total_iterations' in metadata"
            assert 'max_iterations' in metadata, "Missing 'max_iterations' in metadata"
            assert 'tools_used' in metadata, "Missing 'tools_used' in metadata"
            assert 'duration_ms' in metadata, "Missing 'duration_ms' in metadata"
            assert 'total_usage' in metadata, "Missing 'total_usage' in metadata"
            assert 'user_input' in metadata, "Missing 'user_input' in metadata"
            assert metadata['max_iterations'] == 5, f"Expected max_iterations=5, got {metadata['max_iterations']}"

            # Check tool call metadata fields
            # tc = metadata['tools_used']
            # assert len(tc) >= 1, "Expected at least 1 tool call"
            # tc0 = tc[0]
            # assert 'tool_name' in tc0, "Missing 'tool_name' in tool call"
            # assert 'parameters' in tc0, "Missing 'parameters' in tool call"
            # assert 'output' in tc0, "Missing 'output' in tool call"
            # assert 'iteration' in tc0, "Missing 'iteration' in tool call"
            # assert 'success' in tc0, "Missing 'success' in tool call"
            # assert tc0['success'] is True, "Tool call should be successful"
            print(f"  All metadata fields present ✓")
        self.run_test("Metadata Accuracy Check", run)

    def run_all_tests(self):
        """Run all standard tests."""
        print(f"\n{'#' * 60}")
        print(f"  ITERATIVE AGENT TESTS: {self.provider_name}")
        print(f"{'#' * 60}")

        self.test_single_tool_call()
        self.test_multi_step_chain()
        self.test_parallel_tools()
        self.test_three_step_chain()
        self.test_max_iterations_1()
        self.test_no_tool_needed()
        self.test_conditional_tool_use()
        self.test_metadata_accuracy()

        success = self.print_summary()
        return success
