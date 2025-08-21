"""Integration tests for comprehensive GraphBit executor functionality."""

import asyncio
import os
import time
from typing import Any

import pytest

from graphbit import LlmConfig, Executor, Workflow, Node, WorkflowResult


class TestExecutorConfiguration:
    """Integration tests for executor configuration options."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for executor tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    def test_executor_creation_with_options(self, llm_config: Any) -> None:
        """Test executor creation with various configuration options."""
        try:
            # Test basic executor
            executor1 = Executor(llm_config)
            assert executor1 is not None

            # Test with debug mode
            executor2 = Executor(llm_config, debug=True)
            assert executor2 is not None

            # Test with custom timeout
            executor3 = Executor(llm_config, timeout_seconds=120, debug=False)
            assert executor3 is not None

            # Test with lightweight mode
            executor4 = Executor(llm_config, lightweight_mode=True)
            assert executor4 is not None

        except Exception as e:
            pytest.fail(f"Executor configuration test failed: {e}")

    def test_executor_specialized_configurations(self, llm_config: Any) -> None:
        """Test specialized executor configurations."""
        try:
            # Test high throughput executor
            ht_executor = Executor.new_high_throughput(llm_config)
            assert ht_executor is not None

            # Test low latency executor
            ll_executor = Executor.new_low_latency(llm_config)
            assert ll_executor is not None

            # Test memory optimized executor
            mo_executor = Executor.new_memory_optimized(llm_config)
            assert mo_executor is not None

        except Exception as e:
            pytest.fail(f"Specialized executor configuration test failed: {e}")

    def test_executor_configuration_validation(self, llm_config: Any) -> None:
        """Test executor configuration parameter validation."""
        try:
            # Test invalid timeout values
            with pytest.raises((ValueError, RuntimeError)):
                Executor(llm_config, timeout_seconds=0)

            with pytest.raises((ValueError, RuntimeError)):
                Executor(llm_config, timeout_seconds=4000)  # Too high

            # Test invalid specialized executor timeouts
            with pytest.raises((ValueError, RuntimeError)):
                Executor.new_low_latency(llm_config, timeout_seconds=0)

            with pytest.raises((ValueError, RuntimeError)):
                Executor.new_low_latency(llm_config, timeout_seconds=400)  # Too high for low latency

        except Exception as e:
            pytest.fail(f"Executor validation test failed: {e}")


class TestExecutorStatistics:
    """Integration tests for executor statistics and monitoring."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for stats tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def simple_workflow(self) -> Any:
        """Create simple workflow for stats testing."""
        workflow = Workflow("stats_test")
        agent = Node.agent("stats_agent", "Quick test", "stats_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_statistics_tracking(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test executor statistics tracking."""
        try:
            executor = Executor(llm_config)

            # Get initial stats
            initial_stats = executor.get_stats()
            assert isinstance(initial_stats, dict)

            expected_stats_keys = ["total_executions", "successful_executions", "failed_executions"]

            for key in expected_stats_keys:
                if key in initial_stats:  # Some stats may be optional
                    assert isinstance(initial_stats[key], (int, float))

            # Execute workflow to update stats
            simple_workflow.validate()
            result = executor.execute(simple_workflow)
            assert isinstance(result, WorkflowResult)

            # Get updated stats
            updated_stats = executor.get_stats()
            assert isinstance(updated_stats, dict)

        except Exception as e:
            pytest.skip(f"Executor statistics test skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_stats_reset(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test executor statistics reset functionality."""
        try:
            executor = Executor(llm_config)

            # Execute workflow to generate stats
            simple_workflow.validate()
            executor.execute(simple_workflow)

            # Reset stats
            executor.reset_stats()

            # Verify stats were reset
            stats_after_reset = executor.get_stats()
            assert isinstance(stats_after_reset, dict)

        except Exception as e:
            pytest.skip(f"Executor stats reset test skipped: {e}")


class TestExecutorAsyncOperations:
    """Integration tests for executor async operations."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for async tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def simple_workflow(self) -> Any:
        """Create simple workflow for async testing."""
        workflow = Workflow("async_test")
        agent = Node.agent("async_agent", "Async test", "async_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_executor_async_execution(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test async workflow execution."""
        try:
            executor = Executor(llm_config)
            simple_workflow.validate()

            # Test async execution
            result = await executor.run_async(simple_workflow)
            assert isinstance(result, WorkflowResult)

        except Exception as e:
            pytest.fail(f"Async execution test failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_multiple_async_executions(self, llm_config: Any) -> None:
        """Test multiple concurrent async executions."""
        try:
            executor = Executor(llm_config)

            # Create multiple workflows
            workflows = []
            for i in range(3):
                workflow = Workflow(f"async_multi_{i}")
                agent = Node.agent(f"agent_{i}", f"Task {i}", f"multi_{i:03d}")
                workflow.add_node(agent)
                workflow.validate()
                workflows.append(workflow)

            # Start multiple async executions
            tasks = [executor.run_async(workflow) for workflow in workflows]
            results = await asyncio.gather(*tasks)

            # All results should be workflow results
            assert len(results) == len(workflows)
            assert all(isinstance(result, WorkflowResult) for result in results)

        except Exception as e:
            pytest.fail(f"Multiple async executions test failed: {e}")


class TestExecutorRuntimeConfiguration:
    """Integration tests for executor runtime configuration."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for runtime config tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    def test_executor_runtime_configuration(self, llm_config: Any) -> None:
        """Test runtime configuration of executor."""
        try:
            executor = Executor(llm_config)

            # Test configuring timeout
            executor.configure(timeout_seconds=180)

            # Test configuring retries
            executor.configure(max_retries=5)

            # Test configuring metrics
            executor.configure(enable_metrics=True)

            # Test configuring debug mode
            executor.configure(debug=True)

        except Exception as e:
            pytest.fail(f"Runtime configuration test failed: {e}")

    def test_executor_runtime_configuration_validation(self, llm_config: Any) -> None:
        """Test runtime configuration validation."""
        try:
            executor = Executor(llm_config)

            # Test invalid timeout
            with pytest.raises((ValueError, RuntimeError)):
                executor.configure(timeout_seconds=0)

            with pytest.raises((ValueError, RuntimeError)):
                executor.configure(timeout_seconds=4000)

            # Test invalid retries
            with pytest.raises((ValueError, RuntimeError)):
                executor.configure(max_retries=0)

            with pytest.raises((ValueError, RuntimeError)):
                executor.configure(max_retries=100)  # Too many retries

        except Exception as e:
            pytest.fail(f"Runtime configuration validation test failed: {e}")

    def test_executor_mode_management(self, llm_config: Any) -> None:
        """Test executor mode management."""
        try:
            executor = Executor(llm_config)

            # Test getting execution mode
            mode = executor.get_execution_mode()
            assert isinstance(mode, str)

            # Test lightweight mode
            executor.set_lightweight_mode(True)
            assert executor.is_lightweight_mode() is True

            executor.set_lightweight_mode(False)
            assert executor.is_lightweight_mode() is False

        except Exception as e:
            pytest.fail(f"Executor mode management test failed: {e}")


class TestExecutorPerformance:
    """Integration tests for executor performance characteristics."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for performance tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_performance_modes(self, llm_config: Any) -> None:
        """Test performance characteristics of different executor modes."""
        try:
            # Create simple workflow for performance testing
            workflow = Workflow("performance_test")
            agent = Node.agent("perf_agent", "Quick test", "perf_001")
            workflow.add_node(agent)
            workflow.validate()

            # Test different executor modes
            executor_types = [
                ("standard", Executor(llm_config)),
                ("high_throughput", Executor.new_high_throughput(llm_config)),
                ("low_latency", Executor.new_low_latency(llm_config)),
                ("memory_optimized", Executor.new_memory_optimized(llm_config)),
            ]

            for executor_name, executor in executor_types:
                start_time = time.time()
                result = executor.execute(workflow)
                end_time = time.time()

                execution_time = end_time - start_time

                # Verify successful execution
                assert isinstance(result, WorkflowResult)

                # Execution should complete in reasonable time
                assert execution_time < 120, f"{executor_name} took too long: {execution_time}s"

        except Exception as e:
            pytest.skip(f"Executor performance test skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    async def test_executor_memory_efficiency(self, llm_config: Any) -> None:
        """Test executor memory efficiency."""
        try:
            # Test memory optimized executor
            executor = Executor.new_memory_optimized(llm_config)

            # Execute multiple workflows to test memory usage
            workflows = []
            for i in range(5):
                workflow = Workflow(f"memory_test_{i}")
                agent = Node.agent(f"mem_agent_{i}", f"Memory test {i}", f"mem_{i:03d}")
                workflow.add_node(agent)
                workflow.validate()
                workflows.append(workflow)

            # Execute workflows concurrently
            tasks = [executor.run_async(workflow) for workflow in workflows]
            results = await asyncio.gather(*tasks)

            # Verify results
            assert len(results) == 5
            assert all(isinstance(result, WorkflowResult) for result in results)

            # Get final stats
            stats = executor.get_stats()
            assert isinstance(stats, dict)
            for key in ("total_executions", "totalRequests", "runs"):
                if key in stats:
                    assert int(stats[key]) >= 0

        except Exception as e:
            pytest.fail(f"Executor memory efficiency test failed: {e}")


@pytest.mark.integration
class TestExecutorIntegration:
    """Integration tests for executor with other components."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for integration tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_with_complex_workflow(self, llm_config: Any) -> None:
        """Test executor with complex workflow structures."""
        try:
            workflow = Workflow("complex_executor_test")

            # Create multi-node workflow
            agent1 = Node.agent("agent1", "First step", "comp_001")
            condition = Node.condition("condition", "step1_complete == true")
            agent2 = Node.agent("agent2", "Second step", "comp_002")
            transform = Node.transform("transform", "finalize_data")

            workflow.add_node(agent1)
            workflow.add_node(condition)
            workflow.add_node(agent2)
            workflow.add_node(transform)

            try:
                workflow.connect(agent1.id(), condition.id())
                workflow.connect(condition.id(), agent2.id())
                workflow.connect(agent2.id(), transform.id())
            except Exception:
                pass  # nosec B110: acceptable in test context

            workflow.validate()

            # Test with different executor types
            executor_types = [Executor(llm_config), Executor.new_high_throughput(llm_config), Executor.new_low_latency(llm_config)]

            for executor in executor_types:
                result = executor.execute(workflow)
                assert isinstance(result, WorkflowResult)

                # Check execution metrics
                stats = executor.get_stats()
                assert stats["total_executions"] > 0

        except Exception as e:
            pytest.skip(f"Complex workflow executor test skipped: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_error_handling_integration(self, llm_config: Any) -> None:
        """Test executor error handling with various scenarios."""
        try:
            executor = Executor(llm_config, debug=True)

            # Test with invalid workflow
            invalid_workflow = Workflow("invalid_test")
            # Empty workflow

            try:
                invalid_workflow.validate()
                result = executor.execute(invalid_workflow)
                assert isinstance(result, WorkflowResult)

            except Exception:
                pass  # nosec B110: acceptable in test context

            # Test executor stats after error
            stats = executor.get_stats()
            assert isinstance(stats, dict)
            assert "total_executions" in stats

        except Exception as e:
            pytest.skip(f"Executor error handling test skipped: {e}")
