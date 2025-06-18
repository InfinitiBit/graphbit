#!/usr/bin/env python3
"""Comprehensive test runner for GraphBit Python integration tests."""
import os
import subprocess  # nosec B404
import sys
import time
from typing import Any, Dict, List, Tuple


class IntegrationTestRunner:
    """Main test runner for GraphBit integration tests."""

    def __init__(self) -> None:
        """Initialize the test runner with default test modules."""
        self.test_modules = [
            ("tests_embeddings.py", "Embedding Integration Tests"),
            ("tests_llm.py", "LLM Integration Tests"),
            ("tests_static_workflow.py", "Static Workflow Integration Tests"),
            ("tests_dynamic_workflow.py", "Dynamic Workflow Integration Tests"),
            ("tests_workflow_builder.py", "Builder Integration Tests"),
            ("tests_executor_async.py", "Async Executor Tests"),
            ("tests_workflow_context.py", "Workflow Context Accessor Tests"),
            ("tests_executor_batch.py", "Executor Batch + Agent Task Tests"),
            ("tests_validation_result.py", "Validation Error Handling Tests"),
        ]

        self.results: Dict[str, Dict[str, Any]] = {}

    def setup_environment(self) -> bool:
        """Set up test environment and check prerequisites."""
        print("=" * 60)
        print("GraphBit Python Integration Test Suite")
        print("=" * 60)

        # Initialize GraphBit
        try:
            import graphbit

            graphbit.init()
            print(f"✓ GraphBit initialized (version: {graphbit.version()})")
        except Exception as e:
            print(f"✗ Failed to initialize GraphBit: {e}")
            return False

        # Check API keys
        api_keys = {
            "OPENAI_API_KEY": os.getenv("OPENAI_API_KEY"),
            "ANTHROPIC_API_KEY": os.getenv("ANTHROPIC_API_KEY"),
            "HUGGINGFACE_API_KEY": os.getenv("HUGGINGFACE_API_KEY"),
        }

        print("\nAPI Key Status:")
        for key, value in api_keys.items():
            status = "✓ Available" if value else "✗ Not set"
            print(f"  {key}: {status}")

        if not any(api_keys.values()):
            print("\n⚠️  Warning: No API keys detected. Many tests will be skipped.")

        print(f"\nPython version: {sys.version}")
        print(f"Working directory: {os.getcwd()}")

        return True

    def run_test_module(self, module_name: str, description: str) -> Tuple[bool, str, float]:
        """Run a specific test module and return results."""
        print(f"\n{'=' * 60}")
        print(f"Running: {description}")
        print(f"Module: {module_name}")
        print("=" * 60)

        start_time = time.time()

        try:
            # Run pytest on the specific module
            result = subprocess.run(  # nosec B603
                [
                    sys.executable,
                    "-m",
                    "pytest",
                    module_name,
                    "-v",
                    "--tb=short",
                    "--color=yes",
                    "--durations=10",
                ],
                capture_output=True,
                text=True,
                cwd=os.path.dirname(os.path.abspath(__file__)),
                check=False,
            )

            duration = time.time() - start_time

            if result.returncode == 0:
                print("✓ All tests passed")
                return True, result.stdout, duration
            else:
                print("✗ Some tests failed or were skipped")
                return False, result.stdout + "\n" + result.stderr, duration

        except Exception as e:
            duration = time.time() - start_time
            error_msg = f"Failed to run test module: {e}"
            print(f"✗ {error_msg}")
            return False, error_msg, duration

    def run_all_tests(self) -> bool:
        """Run all integration test modules."""
        if not self.setup_environment():
            print("Environment setup failed. Exiting.")
            return False

        overall_success = True
        total_duration = 0.0

        print(f"\n{'=' * 60}")
        print("Starting Integration Test Execution")
        print("=" * 60)

        for module_name, description in self.test_modules:
            success, output, duration = self.run_test_module(module_name, description)
            total_duration += duration

            self.results[module_name] = {
                "success": success,
                "description": description,
                "output": output,
                "duration": duration,
            }

            if not success:
                overall_success = False

        self.print_summary(total_duration)
        return overall_success

    def print_summary(self, total_duration: float) -> None:
        """Print test execution summary."""
        print(f"\n{'=' * 60}")
        print("INTEGRATION TEST SUMMARY")
        print("=" * 60)

        passed_count = sum(1 for r in self.results.values() if r["success"])
        total_count = len(self.results)

        for _module_name, result in self.results.items():
            status = "✓ PASSED" if result["success"] else "✗ FAILED"
            duration_str = f"{result['duration']:.2f}s"
            print(f"{status:>10} | {duration_str:>8} | {result['description']}")

        print("-" * 60)
        print(f"Total: {passed_count}/{total_count} modules passed")
        print(f"Total execution time: {total_duration:.2f} seconds")

        if passed_count == total_count:
            print("\n🎉 All integration tests completed successfully!")
        else:
            print(f"\n⚠️  {total_count - passed_count} module(s) had failures or skipped tests")
            print("\nDetailed output for failed modules:")
            print("-" * 60)

            for module_name, result in self.results.items():
                if not result["success"]:
                    print(f"\n{module_name} ({result['description']}):")
                    print(result["output"][:1000])  # Limit output length
                    if len(result["output"]) > 1000:
                        print("... (output truncated)")

    def run_specific_tests(self, test_patterns: List[str]) -> bool:
        """Run specific tests matching the given patterns."""
        if not self.setup_environment():
            print("Environment setup failed. Exiting.")
            return False

        print(f"\nRunning specific tests matching: {', '.join(test_patterns)}")

        # Build pytest command with specific patterns
        cmd = [sys.executable, "-m", "pytest", "-v", "--tb=short"]
        for pattern in test_patterns:
            cmd.extend(["-k", pattern])

        # Add all test modules
        for module_name, _ in self.test_modules:
            cmd.append(module_name)

        try:
            result = subprocess.run(cmd, cwd=os.path.dirname(os.path.abspath(__file__)), check=False)  # nosec B603
            return result.returncode == 0
        except Exception as e:
            print(f"Failed to run specific tests: {e}")
            return False


def main() -> None:
    """Run the test runner."""
    runner = IntegrationTestRunner()

    # Check command line arguments for specific test patterns
    if len(sys.argv) > 1:
        # Run specific tests
        test_patterns = sys.argv[1:]
        success = runner.run_specific_tests(test_patterns)
    else:
        # Run all tests
        success = runner.run_all_tests()

    # Exit with appropriate code
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
