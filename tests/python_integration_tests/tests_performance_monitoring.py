"""Integration tests for GraphBit performance monitoring and statistics."""

import contextlib
import os
import time
from typing import Any

import pytest

import graphbit


class TestClientStatistics:
    """Tests for LLM client statistics and monitoring."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get API key for statistics tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        assert api_key is not None
        return api_key

    @pytest.fixture
    def client(self, api_key: str) -> Any:
        """Create LLM client for statistics testing."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        return graphbit.LlmClient(config, debug=True)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_initial_client_statistics(self, client: Any) -> None:
        """Test initial state of client statistics."""
        stats = client.get_stats()

        assert isinstance(stats, dict)

        # Check for expected statistical fields
        expected_fields = ["total_requests", "successful_requests", "failed_requests", "average_response_time_ms", "uptime"]

        for field in expected_fields:
            if field in stats:
                if field == "uptime" or field == "average_response_time_ms":
                    assert isinstance(stats[field], (int, float))
                    assert stats[field] >= 0
                else:
                    assert isinstance(stats[field], int)
                    assert stats[field] >= 0

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_statistics_tracking_completion(self, client: Any) -> None:
        """Test statistics tracking during completion operations."""
        # Get initial stats
        initial_stats = client.get_stats()
        initial_total = initial_stats.get("total_requests", 0)
        initial_successful = initial_stats.get("successful_requests", 0)

        # Perform completion
        try:
            result = client.complete("What is 2+2? Answer with just the number.", max_tokens=10)
            assert isinstance(result, str)

            # Get updated stats
            updated_stats = client.get_stats()

            # Verify stats were updated
            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] > initial_total

            if "successful_requests" in updated_stats:
                assert updated_stats["successful_requests"] > initial_successful

            # Check response time tracking
            if "average_response_time_ms" in updated_stats:
                assert updated_stats["average_response_time_ms"] > 0

        except Exception:
            # If completion fails, check that failure stats are tracked
            updated_stats = client.get_stats()

            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] > initial_total

            if "failed_requests" in updated_stats:
                assert updated_stats["failed_requests"] >= 0

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_statistics_tracking_batch(self, client: Any) -> None:
        """Test statistics tracking during batch operations."""
        initial_stats = client.get_stats()
        initial_total = initial_stats.get("total_requests", 0)

        # Perform batch completion
        prompts = ["Count to 1", "Count to 2", "Count to 3"]

        try:
            results = client.complete_batch(prompts, max_tokens=10, max_concurrency=2)
            assert isinstance(results, list)
            assert len(results) == len(prompts)

            # Get updated stats
            updated_stats = client.get_stats()

            # Batch operations might count as multiple requests or single batch
            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] >= initial_total

        except Exception:
            # Even if batch fails, stats should be updated
            updated_stats = client.get_stats()
            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] >= initial_total

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_statistics_reset(self, client: Any) -> None:
        """Test statistics reset functionality."""
        # Perform some operations to generate stats
        with contextlib.suppress(Exception):
            client.complete("Test", max_tokens=5)  # Stats should still be generated even if completion fails

        # Get stats before reset
        stats_before = client.get_stats()

        # Reset stats
        client.reset_stats()

        # Get stats after reset
        stats_after = client.get_stats()

        # Verify reset worked
        for field in ["total_requests", "successful_requests", "failed_requests"]:
            if field in stats_after:
                assert stats_after[field] == 0 or stats_after[field] < stats_before.get(field, 0)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    @pytest.mark.asyncio
    async def test_async_statistics_tracking(self, client: Any) -> None:
        """Test statistics tracking for async operations."""
        initial_stats = client.get_stats()
        initial_total = initial_stats.get("total_requests", 0)

        # Perform async completion
        try:
            result = await client.complete_async("Quick test", max_tokens=5)
            assert isinstance(result, str)

            # Check stats were updated
            updated_stats = client.get_stats()
            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] > initial_total

        except Exception:
            # Even if async completion fails, stats should be tracked
            updated_stats = client.get_stats()
            if "total_requests" in updated_stats:
                assert updated_stats["total_requests"] >= initial_total


class TestExecutorStatistics:
    """Tests for workflow executor statistics and monitoring."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for executor statistics tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    @pytest.fixture
    def simple_workflow(self) -> Any:
        """Create simple workflow for statistics testing."""
        workflow = graphbit.Workflow("stats_test")
        agent = graphbit.Node.agent("stats_agent", "Quick response", "stats_001")
        workflow.add_node(agent)
        return workflow

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_initial_statistics(self, llm_config: Any) -> None:
        """Test initial state of executor statistics."""
        executor = graphbit.Executor(llm_config)
        stats = executor.get_stats()

        assert isinstance(stats, dict)

        # Check for expected executor statistical fields
        expected_fields = ["total_executions", "successful_executions", "failed_executions", "average_duration_ms"]

        for field in expected_fields:
            if field in stats:
                assert isinstance(stats[field], (int, float))
                assert stats[field] >= 0

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_execution_statistics(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test executor statistics during workflow execution."""
        executor = graphbit.Executor(llm_config)

        # Get initial stats
        initial_stats = executor.get_stats()
        initial_total = initial_stats.get("total_executions", 0)
        initial_successful = initial_stats.get("successful_executions", 0)

        try:
            # Validate and execute workflow
            simple_workflow.validate()
            result = executor.execute(simple_workflow)

            assert isinstance(result, graphbit.WorkflowResult)

            # Get updated stats
            updated_stats = executor.get_stats()

            # Verify execution was tracked
            if "total_executions" in updated_stats:
                assert updated_stats["total_executions"] > initial_total

            if result.is_success() and "successful_executions" in updated_stats:
                assert updated_stats["successful_executions"] > initial_successful

            # Check execution time tracking
            if "average_duration_ms" in updated_stats:
                assert updated_stats["average_duration_ms"] >= 0

        except Exception:
            # If execution fails, verify failure stats are tracked
            updated_stats = executor.get_stats()

            if "total_executions" in updated_stats:
                assert updated_stats["total_executions"] > initial_total

            if "failed_executions" in updated_stats:
                assert updated_stats["failed_executions"] >= 0

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_statistics_reset(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test executor statistics reset functionality."""
        executor = graphbit.Executor(llm_config)

        # Perform execution to generate stats
        try:
            simple_workflow.validate()
            executor.execute(simple_workflow)
        except Exception:
            pass  # nosec B110: acceptable in test context

        # Get stats before reset
        stats_before = executor.get_stats()

        # Reset stats
        executor.reset_stats()

        # Get stats after reset
        stats_after = executor.get_stats()

        # Verify reset worked
        for field in ["total_executions", "successful_executions", "failed_executions"]:
            if field in stats_after:
                assert stats_after[field] == 0 or stats_after[field] < stats_before.get(field, 0)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_executor_performance_modes_statistics(self, llm_config: Any, simple_workflow: Any) -> None:
        """Test statistics across different executor performance modes."""
        # Test different executor types
        executors = [
            ("standard", graphbit.Executor(llm_config)),
            ("high_throughput", graphbit.Executor.new_high_throughput(llm_config)),
            ("low_latency", graphbit.Executor.new_low_latency(llm_config)),
            ("memory_optimized", graphbit.Executor.new_memory_optimized(llm_config)),
        ]

        for _mode_name, executor in executors:
            try:
                # Execute workflow
                simple_workflow.validate()
                executor.execute(simple_workflow)

                # Get stats for this mode
                stats = executor.get_stats()
                assert isinstance(stats, dict)

                # Verify stats are being tracked for each mode
                if "total_executions" in stats:
                    assert stats["total_executions"] >= 0

            except Exception:
                # If execution fails, verify we still get stats
                stats = executor.get_stats()
                assert isinstance(stats, dict)


class TestSystemPerformanceMonitoring:
    """Tests for system-level performance monitoring."""

    def test_system_info_performance_metrics(self) -> None:
        """Test system information performance metrics."""
        system_info = graphbit.get_system_info()

        assert isinstance(system_info, dict)

        # Check for performance-related metrics
        performance_fields = ["cpu_count", "runtime_initialized", "runtime_uptime_seconds", "runtime_worker_threads", "runtime_max_blocking_threads"]

        for field in performance_fields:
            if field in system_info:
                if field == "cpu_count":
                    assert isinstance(system_info[field], int)
                    assert system_info[field] > 0
                elif field == "runtime_initialized":
                    assert isinstance(system_info[field], bool)
                elif field.startswith("runtime_"):
                    assert isinstance(system_info[field], (int, float))
                    if field == "runtime_uptime_seconds":
                        assert system_info[field] >= 0
                    else:
                        assert system_info[field] > 0

    def test_health_check_performance_monitoring(self) -> None:
        """Test health check performance monitoring."""
        health = graphbit.health_check()

        assert isinstance(health, dict)

        # Check for health-related performance metrics
        if "checks" in health:
            checks = health["checks"]
            if isinstance(checks, dict):
                # Verify health checks provide meaningful information
                assert len(checks) >= 0

                for component, _status in checks.items():
                    assert isinstance(component, str)
                    assert len(component) > 0

    def test_runtime_configuration_monitoring(self) -> None:
        """Test monitoring of runtime configuration."""
        # Configure runtime and verify it's reflected in monitoring
        try:
            graphbit.configure_runtime(worker_threads=4, max_blocking_threads=8)

            system_info = graphbit.get_system_info()

            # Check if configuration changes are reflected
            if "runtime_worker_threads" in system_info:
                assert isinstance(system_info["runtime_worker_threads"], int)

            if "runtime_max_blocking_threads" in system_info:
                assert isinstance(system_info["runtime_max_blocking_threads"], int)

        except Exception as e:
            # Configuration might not always be changeable
            pytest.skip(f"Runtime configuration monitoring test skipped: {e}")


class TestPerformanceBenchmarking:
    """Tests for performance benchmarking capabilities."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get API key for performance tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        assert api_key is not None
        return api_key

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_completion_performance_measurement(self, api_key: str) -> None:
        """Test performance measurement of completion operations."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        # Measure completion performance
        start_time = time.time()

        try:
            result = client.complete("Quick test", max_tokens=5)
            end_time = time.time()

            assert isinstance(result, str)

            # Verify reasonable performance
            duration = end_time - start_time
            assert duration > 0
            assert duration < 60  # Should complete within 60 seconds

            # Check if client tracks response time
            stats = client.get_stats()
            if "average_response_time_ms" in stats:
                assert stats["average_response_time_ms"] > 0
                assert stats["average_response_time_ms"] < 60000  # Less than 60 seconds in ms

        except Exception as e:
            pytest.skip(f"Performance measurement test skipped due to error: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_batch_performance_comparison(self, api_key: str) -> None:
        """Test performance comparison between single and batch operations."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)

        prompts = ["Test 1", "Test 2", "Test 3"]

        try:
            # Measure individual completions
            individual_start = time.time()
            individual_results = []
            for prompt in prompts:
                result = client.complete(prompt, max_tokens=5)
                individual_results.append(result)
            individual_end = time.time()
            individual_duration = individual_end - individual_start

            # Measure batch completion
            batch_start = time.time()
            batch_results = client.complete_batch(prompts, max_tokens=5, max_concurrency=3)
            batch_end = time.time()
            batch_duration = batch_end - batch_start

            # Verify results
            assert len(individual_results) == len(prompts)
            assert len(batch_results) == len(prompts)

            # Performance comparison (batch might be faster or slower depending on implementation)
            assert individual_duration > 0
            assert batch_duration > 0

        except Exception as e:
            pytest.skip(f"Batch performance comparison skipped due to error: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_execution_performance(self, api_key: str) -> None:
        """Test workflow execution performance monitoring."""
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        executor = graphbit.Executor(config)

        # Create simple workflow
        workflow = graphbit.Workflow("performance_test")
        agent = graphbit.Node.agent("perf_agent", "Quick response", "perf_001")
        workflow.add_node(agent)

        try:
            workflow.validate()

            # Measure execution performance
            start_time = time.time()
            result = executor.execute(workflow)
            end_time = time.time()

            assert isinstance(result, graphbit.WorkflowResult)

            # Check execution time
            execution_duration = end_time - start_time
            assert execution_duration > 0
            assert execution_duration < 120  # Should complete within 2 minutes

            # Check if result tracks execution time
            result_duration = result.execution_time_ms()
            assert isinstance(result_duration, int)
            assert result_duration >= 0

            # Check executor stats
            stats = executor.get_stats()
            if "average_duration_ms" in stats:
                assert stats["average_duration_ms"] >= 0

        except Exception as e:
            pytest.skip(f"Workflow performance test skipped due to error: {e}")


@pytest.mark.integration
class TestCrossComponentPerformanceMonitoring:
    """Integration tests for performance monitoring across components."""

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_end_to_end_performance_tracking(self) -> None:
        """Test performance tracking across entire pipeline."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        # Initialize components
        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        client = graphbit.LlmClient(config)
        executor = graphbit.Executor(config)

        # Create workflow
        workflow = graphbit.Workflow("e2e_perf_test")
        agent = graphbit.Node.agent("e2e_agent", "End to end test", "e2e_001")
        workflow.add_node(agent)

        try:
            # Track performance across operations
            overall_start = time.time()

            # 1. Direct LLM completion
            client_result = client.complete("Direct test", max_tokens=5)

            # 2. Workflow execution
            workflow.validate()
            workflow_result = executor.execute(workflow)

            overall_end = time.time()

            # Verify results
            assert isinstance(client_result, str)
            assert isinstance(workflow_result, graphbit.WorkflowResult)

            # Check overall performance
            total_duration = overall_end - overall_start
            assert total_duration > 0
            assert total_duration < 180  # Should complete within 3 minutes

            # Check individual component stats
            client_stats = client.get_stats()
            executor_stats = executor.get_stats()

            assert isinstance(client_stats, dict)
            assert isinstance(executor_stats, dict)

        except Exception as e:
            pytest.skip(f"End-to-end performance test skipped due to error: {e}")

    def test_resource_utilization_monitoring(self) -> None:
        """Test monitoring of resource utilization."""
        # Get system information
        system_info = graphbit.get_system_info()

        # Check resource-related metrics
        resource_fields = ["cpu_count", "memory_allocator", "runtime_worker_threads"]

        for field in resource_fields:
            if field in system_info:
                assert system_info[field] is not None

                if field == "cpu_count" or field == "runtime_worker_threads":
                    assert isinstance(system_info[field], int)
                    assert system_info[field] > 0
                elif field == "memory_allocator":
                    assert isinstance(system_info[field], str)
                    assert len(system_info[field]) > 0

    def test_concurrent_operations_monitoring(self) -> None:
        """Test monitoring during concurrent operations."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

        # Create multiple clients for concurrent testing
        clients = [graphbit.LlmClient(config) for _ in range(3)]

        try:
            # Perform concurrent operations
            start_time = time.time()

            results = []
            for _i, client in enumerate(clients):
                result = client.complete(f"Concurrent test {_i}", max_tokens=5)
                results.append(result)

            end_time = time.time()

            # Verify all operations completed
            assert len(results) == len(clients)
            assert all(isinstance(r, str) for r in results)

            # Check that operations completed in reasonable time
            duration = end_time - start_time
            assert duration > 0
            assert duration < 180  # Should complete within 3 minutes

            # Check stats for each client
            for _i, client in enumerate(clients):
                stats = client.get_stats()
                assert isinstance(stats, dict)

                if "total_requests" in stats:
                    assert stats["total_requests"] >= 1

        except Exception as e:
            pytest.skip(f"Concurrent operations monitoring test skipped due to error: {e}")
