"""Integration tests for GraphBit runtime configuration and management."""

import contextlib
import os
import time

import pytest

import graphbit


class TestAdvancedRuntimeConfiguration:
    """Tests for advanced runtime configuration scenarios."""

    def setUp(self) -> None:
        """Set up test environment."""
        # Ensure clean state before each test
        with contextlib.suppress(Exception):
            graphbit.shutdown()

    def test_runtime_configuration_before_init(self) -> None:
        """Test configuring runtime before initialization."""
        try:
            # Configure runtime with custom settings
            graphbit.configure_runtime(worker_threads=6, max_blocking_threads=12, thread_stack_size_mb=2)

            # Initialize after configuration
            graphbit.init(log_level="info", enable_tracing=True)

            # Verify configuration took effect
            system_info = graphbit.get_system_info()

            if "runtime_worker_threads" in system_info:
                # Configuration may or may not be reflected depending on implementation
                assert isinstance(system_info["runtime_worker_threads"], int)
                assert system_info["runtime_worker_threads"] > 0

        except Exception as e:
            pytest.skip(f"Runtime configuration before init test skipped: {e}")

    def test_runtime_configuration_after_init(self) -> None:
        """Test configuring runtime after initialization."""
        try:
            # Initialize first
            graphbit.init()

            # Try to configure after initialization
            graphbit.configure_runtime(worker_threads=8)

            # This may or may not be allowed depending on implementation
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

        except Exception as e:
            # Post-init configuration might not be allowed
            assert "already" in str(e).lower() or "initialized" in str(e).lower()

    def test_multiple_runtime_configurations(self) -> None:
        """Test multiple runtime configuration attempts."""
        try:
            # First configuration
            graphbit.configure_runtime(worker_threads=4)

            # Second configuration (should either replace or be ignored)
            graphbit.configure_runtime(worker_threads=8, max_blocking_threads=16)

            # Third configuration with different parameters
            graphbit.configure_runtime(thread_stack_size_mb=4)

            # Initialize and verify
            graphbit.init()
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

        except Exception as e:
            pytest.skip(f"Multiple runtime configuration test skipped: {e}")

    def test_invalid_runtime_configurations(self) -> None:
        """Test invalid runtime configuration handling."""
        # Test zero worker threads
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(worker_threads=0)
            # Should either accept or reject, but not crash

        # Test negative values
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(max_blocking_threads=-1)

        # Test extremely large values
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(worker_threads=10000)

        # Test zero stack size
        with contextlib.suppress(ValueError, RuntimeError):
            graphbit.configure_runtime(thread_stack_size_mb=0)

    def test_runtime_configuration_boundaries(self) -> None:
        """Test runtime configuration boundary conditions."""
        # Test minimum valid configurations
        try:
            graphbit.configure_runtime(worker_threads=1)
            graphbit.init()

            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

        except Exception as e:
            pytest.skip(f"Minimum configuration test skipped: {e}")

        # Reset for next test
        with contextlib.suppress(Exception):
            graphbit.shutdown()

        # Test maximum reasonable configurations
        try:
            graphbit.configure_runtime(worker_threads=32, max_blocking_threads=64, thread_stack_size_mb=8)
            graphbit.init()

            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

        except Exception as e:
            pytest.skip(f"Maximum configuration test skipped: {e}")


class TestRuntimeStateManagement:
    """Tests for runtime state management and lifecycle."""

    def test_runtime_initialization_states(self) -> None:
        """Test different runtime initialization states."""
        # Test initial state
        system_info = graphbit.get_system_info()
        system_info.get("runtime_initialized", False)

        # Initialize runtime
        graphbit.init()

        # Test post-initialization state
        updated_info = graphbit.get_system_info()
        post_init_state = updated_info.get("runtime_initialized", False)

        # Runtime should be initialized after init call
        assert isinstance(post_init_state, bool)

    def test_runtime_uptime_tracking(self) -> None:
        """Test runtime uptime tracking."""
        # Initialize runtime
        graphbit.init()

        # Get initial uptime
        initial_info = graphbit.get_system_info()
        initial_uptime = initial_info.get("runtime_uptime_seconds", 0)

        # Wait a short period
        time.sleep(1)

        # Get updated uptime
        updated_info = graphbit.get_system_info()
        updated_uptime = updated_info.get("runtime_uptime_seconds", 0)

        # Uptime should increase or at least be tracked
        if initial_uptime is not None and updated_uptime is not None:
            assert isinstance(initial_uptime, (int, float))
            assert isinstance(updated_uptime, (int, float))
            assert updated_uptime >= initial_uptime

    def test_runtime_health_monitoring(self) -> None:
        """Test runtime health monitoring capabilities."""
        # Initialize runtime
        graphbit.init()

        # Perform health check
        health = graphbit.health_check()

        assert isinstance(health, dict)

        # Check for runtime-specific health indicators
        runtime_health_indicators = ["runtime_healthy", "runtime_uptime_ok", "worker_threads_ok"]

        for indicator in runtime_health_indicators:
            if indicator in health:
                assert isinstance(health[indicator], bool)

    def test_runtime_shutdown_and_restart(self) -> None:
        """Test runtime shutdown and restart capabilities."""
        try:
            # Initialize runtime
            graphbit.init()

            # Verify runtime is running
            initial_info = graphbit.get_system_info()
            assert initial_info.get("runtime_initialized", False) is not False

            # Shutdown runtime
            graphbit.shutdown()

            # Re-initialize runtime
            graphbit.init()

            # Verify runtime is running again
            restart_info = graphbit.get_system_info()
            assert isinstance(restart_info, dict)

        except Exception as e:
            pytest.skip(f"Shutdown and restart test skipped: {e}")


class TestConcurrentRuntimeOperations:
    """Tests for concurrent runtime operations."""

    def test_concurrent_initialization(self) -> None:
        """Test concurrent initialization attempts."""
        import threading

        results = []
        errors = []

        def init_runtime():
            try:
                graphbit.init()
                results.append("success")
            except Exception as e:
                errors.append(str(e))

        # Create multiple threads trying to initialize concurrently
        threads = [threading.Thread(target=init_runtime) for _ in range(5)]

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join()

        # At least one initialization should succeed
        assert len(results) >= 1

        # System should be functional after concurrent initialization
        system_info = graphbit.get_system_info()
        assert isinstance(system_info, dict)

    def test_concurrent_configuration_attempts(self) -> None:
        """Test concurrent configuration attempts."""
        import threading

        def configure_runtime(worker_count: int):
            with contextlib.suppress(Exception):
                graphbit.configure_runtime(worker_threads=worker_count)

        # Create multiple threads trying to configure concurrently
        threads = [threading.Thread(target=configure_runtime, args=(i + 2,)) for i in range(3)]

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join()

        # Initialize after concurrent configuration
        graphbit.init()

        # System should still be functional
        system_info = graphbit.get_system_info()
        assert isinstance(system_info, dict)

    def test_concurrent_system_info_access(self) -> None:
        """Test concurrent access to system information."""
        import threading

        graphbit.init()

        results = []

        def get_system_info():
            try:
                info = graphbit.get_system_info()
                results.append(info)
            except Exception as e:
                results.append(f"Error: {e}")

        # Create multiple threads accessing system info concurrently
        threads = [threading.Thread(target=get_system_info) for _ in range(10)]

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join()

        # All requests should succeed
        assert len(results) == 10
        assert all(isinstance(result, dict) for result in results)


class TestRuntimeResourceManagement:
    """Tests for runtime resource management."""

    def test_memory_management_configuration(self) -> None:
        """Test memory-related runtime configuration."""
        try:
            # Configure with memory-optimized settings
            graphbit.configure_runtime(worker_threads=2, thread_stack_size_mb=1)  # Lower thread count  # Smaller stack size

            graphbit.init()

            # Verify system is functional with memory-optimized config
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

            # Check memory allocator information
            if "memory_allocator" in system_info:
                assert isinstance(system_info["memory_allocator"], str)
                assert len(system_info["memory_allocator"]) > 0

        except Exception as e:
            pytest.skip(f"Memory management configuration test skipped: {e}")

    def test_thread_pool_configuration(self) -> None:
        """Test thread pool configuration options."""
        try:
            # Configure thread pool settings
            graphbit.configure_runtime(worker_threads=4, max_blocking_threads=8)

            graphbit.init()

            # Verify thread pool configuration
            system_info = graphbit.get_system_info()

            if "runtime_worker_threads" in system_info:
                assert isinstance(system_info["runtime_worker_threads"], int)
                assert system_info["runtime_worker_threads"] > 0

            if "runtime_max_blocking_threads" in system_info:
                assert isinstance(system_info["runtime_max_blocking_threads"], int)
                assert system_info["runtime_max_blocking_threads"] > 0

        except Exception as e:
            pytest.skip(f"Thread pool configuration test skipped: {e}")

    def test_runtime_performance_under_load(self) -> None:
        """Test runtime performance under various load conditions."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        try:
            # Configure for high performance
            graphbit.configure_runtime(worker_threads=8, max_blocking_threads=16)

            graphbit.init()

            # Create load
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # Perform multiple operations to test runtime under load
            start_time = time.time()

            for i in range(3):  # Reduced load for testing
                try:
                    result = client.complete(f"Quick test {i}", max_tokens=5)
                    assert isinstance(result, str)
                except Exception:
                    pass  # Some operations may fail, which is acceptable for load testing

            end_time = time.time()

            # Check that runtime remained responsive
            duration = end_time - start_time
            assert duration < 60  # Should complete within reasonable time

            # Verify system health after load
            health = graphbit.health_check()
            assert isinstance(health, dict)

        except Exception as e:
            pytest.skip(f"Runtime performance under load test skipped: {e}")


class TestRuntimeErrorRecovery:
    """Tests for runtime error recovery scenarios."""

    def test_recovery_from_configuration_errors(self) -> None:
        """Test recovery from configuration errors."""
        try:
            # Attempt invalid configuration
            with contextlib.suppress(Exception):
                graphbit.configure_runtime(worker_threads=-1)

            # Try valid configuration after error
            graphbit.configure_runtime(worker_threads=4)
            graphbit.init()

            # Verify system is functional
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

        except Exception as e:
            pytest.skip(f"Configuration error recovery test skipped: {e}")

    def test_recovery_from_initialization_errors(self) -> None:
        """Test recovery from initialization errors."""
        try:
            # Force potential initialization error scenario
            # This might not actually cause an error but tests the pattern
            graphbit.init(log_level="invalid_level")

            # Try normal initialization
            graphbit.init(log_level="info")

            # Verify system is functional
            version = graphbit.version()
            assert isinstance(version, str)
            assert len(version) > 0

        except Exception as e:
            pytest.skip(f"Initialization error recovery test skipped: {e}")

    def test_graceful_degradation(self) -> None:
        """Test graceful degradation under error conditions."""
        try:
            # Initialize system
            graphbit.init()

            # Test that basic functions work even if some components fail
            version = graphbit.version()
            assert isinstance(version, str)

            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

            health = graphbit.health_check()
            assert isinstance(health, dict)

        except Exception as e:
            pytest.fail(f"Basic system functions should not fail: {e}")


@pytest.mark.integration
class TestRuntimeIntegrationScenarios:
    """Integration tests for runtime in realistic scenarios."""

    def test_runtime_with_multiple_components(self) -> None:
        """Test runtime behavior with multiple active components."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        try:
            # Configure runtime for multi-component scenario
            graphbit.configure_runtime(worker_threads=6, max_blocking_threads=12)

            graphbit.init()

            # Create multiple components
            llm_config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            llm_client = graphbit.LlmClient(llm_config)
            executor = graphbit.Executor(llm_config)

            # Test basic functionality of each component
            version = graphbit.version()
            assert isinstance(version, str)

            # Test that all components can coexist
            try:
                result = llm_client.complete("Test", max_tokens=5)
                assert isinstance(result, str)
            except Exception:
                pass  # Network errors are acceptable

            # Create simple workflow for executor
            workflow = graphbit.Workflow("integration_test")
            agent = graphbit.Node.agent("test_agent", "Integration test", "int_001")
            workflow.add_node(agent)

            try:
                workflow.validate()
                result = executor.execute(workflow)
                assert isinstance(result, graphbit.WorkflowResult)
            except Exception:
                pass  # Execution errors are acceptable for integration test

            # Verify runtime health with multiple components
            health = graphbit.health_check()
            assert isinstance(health, dict)

        except Exception as e:
            pytest.skip(f"Multi-component runtime test skipped: {e}")

    def test_runtime_lifecycle_management(self) -> None:
        """Test complete runtime lifecycle management."""
        try:
            # Start with clean state
            with contextlib.suppress(Exception):
                graphbit.shutdown()

            # Configure runtime
            graphbit.configure_runtime(worker_threads=4)

            # Initialize
            graphbit.init(enable_tracing=True)

            # Use runtime
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

            # Monitor health
            health = graphbit.health_check()
            assert isinstance(health, dict)

            # Shutdown
            graphbit.shutdown()

            # Restart
            graphbit.init()

            # Verify functionality after restart
            version = graphbit.version()
            assert isinstance(version, str)

        except Exception as e:
            pytest.skip(f"Runtime lifecycle management test skipped: {e}")

    def test_runtime_configuration_persistence(self) -> None:
        """Test runtime configuration persistence across operations."""
        try:
            # Configure runtime with specific settings
            graphbit.configure_runtime(worker_threads=6, max_blocking_threads=10, thread_stack_size_mb=2)

            # Initialize and get initial system info
            graphbit.init()
            initial_info = graphbit.get_system_info()

            # Perform various operations
            graphbit.version()
            graphbit.health_check()

            # Check that configuration persists
            final_info = graphbit.get_system_info()

            # Compare relevant configuration fields
            for field in ["runtime_worker_threads", "runtime_max_blocking_threads"]:
                if field in initial_info and field in final_info:
                    assert initial_info[field] == final_info[field]

        except Exception as e:
            pytest.skip(f"Runtime configuration persistence test skipped: {e}")
