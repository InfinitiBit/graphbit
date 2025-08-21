"""Integration tests for GraphBit system functions and utilities."""

import contextlib
import os
import time

import pytest

import graphbit


class TestSystemInitialization:
    """Integration tests for system initialization and configuration."""

    def test_basic_initialization(self) -> None:
        """Test basic library initialization."""
        try:
            # Test default initialization
            graphbit.init()

            # Should be able to call multiple times without error
            graphbit.init()

        except Exception as e:
            pytest.fail(f"Basic initialization failed: {e}")

    def test_initialization_with_logging(self) -> None:
        """Test initialization with different logging configurations."""
        try:
            # Test with debug logging
            graphbit.init(log_level="debug", enable_tracing=True, debug=True)

            # Test with different log levels
            for log_level in ["trace", "debug", "info", "warn", "error"]:
                graphbit.init(log_level=log_level, enable_tracing=True)

            # Test with tracing disabled
            graphbit.init(enable_tracing=False)

        except Exception as e:
            pytest.fail(f"Initialization with logging failed: {e}")

    def test_version_information(self) -> None:
        """Test version information retrieval."""
        try:
            version = graphbit.version()
            assert isinstance(version, str)
            assert len(version) > 0
            assert "." in version  # Should be in semantic version format

            # Version should be consistent across calls
            version2 = graphbit.version()
            assert version == version2

        except Exception as e:
            pytest.fail(f"Version information test failed: {e}")


class TestSystemInformation:
    """Integration tests for system information and health status."""

    def test_system_info_basic(self) -> None:
        """Test basic system information retrieval."""
        try:
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)

            # Check for expected keys
            expected_keys = ["version", "python_binding_version", "cpu_count", "runtime_initialized", "memory_allocator", "build_target", "build_profile"]

            for key in expected_keys:
                assert key in system_info, f"Missing system info key: {key}"

            # Validate data types
            assert isinstance(system_info["version"], str)
            assert isinstance(system_info["python_binding_version"], str)
            assert isinstance(system_info["cpu_count"], int)
            assert isinstance(system_info["runtime_initialized"], bool)
            assert isinstance(system_info["memory_allocator"], str)
            assert isinstance(system_info["build_target"], str)
            assert isinstance(system_info["build_profile"], str)

            # Validate reasonable values
            assert system_info["cpu_count"] > 0
            assert system_info["build_profile"] in ["debug", "release"]

        except Exception as e:
            pytest.fail(f"System info test failed: {e}")

    def test_system_info_runtime_stats(self) -> None:
        """Test system information with runtime statistics."""
        try:
            # Initialize to ensure runtime is started
            graphbit.init()

            system_info = graphbit.get_system_info()

            # Check for runtime-specific keys
            runtime_keys = ["runtime_uptime_seconds", "runtime_worker_threads", "runtime_max_blocking_threads"]

            for key in runtime_keys:
                if key in system_info:  # These may be optional
                    if key == "runtime_uptime_seconds":
                        assert isinstance(system_info[key], (int, float))
                        assert system_info[key] >= 0
                    else:
                        assert isinstance(system_info[key], int)
                        assert system_info[key] > 0

        except Exception as e:
            pytest.fail(f"System info runtime stats test failed: {e}")


class TestHealthCheck:
    """Integration tests for health check functionality."""

    def test_basic_health_check(self) -> None:
        """Test basic health check functionality."""
        try:
            health_status = graphbit.health_check()
            assert isinstance(health_status, dict)

            # Health check should include status information
            expected_keys = ["status", "checks"]
            for key in expected_keys:
                if key in health_status:  # May be implementation-dependent
                    if key == "status":
                        assert isinstance(health_status[key], str)
                    elif key == "checks":
                        assert isinstance(health_status[key], (dict, list))

        except Exception as e:
            pytest.fail(f"Basic health check failed: {e}")

    def test_health_check_components(self) -> None:
        """Test health check for individual components."""
        try:
            # Initialize system first
            graphbit.init()

            health_status = graphbit.health_check()

            # Check if specific components are reported
            if "checks" in health_status and isinstance(health_status["checks"], dict):
                # At least some components should be checked
                checks = health_status["checks"]
                assert len(checks) > 0, "No health checks performed"

                # Each check should have a status
                for component, _status in checks.items():
                    assert isinstance(component, str)
                    assert len(component) > 0

        except Exception as e:
            pytest.fail(f"Health check components test failed: {e}")


class TestRuntimeConfiguration:
    """Integration tests for runtime configuration."""

    def test_runtime_configuration_basic(self) -> None:
        """Test basic runtime configuration."""
        try:
            # Test configuring worker threads
            graphbit.configure_runtime(worker_threads=4)

            # Test configuring blocking threads
            graphbit.configure_runtime(max_blocking_threads=8)

            # Test configuring thread stack size
            graphbit.configure_runtime(thread_stack_size_mb=2)

            # Test configuring multiple parameters
            graphbit.configure_runtime(worker_threads=6, max_blocking_threads=12, thread_stack_size_mb=1)

        except Exception as e:
            pytest.fail(f"Runtime configuration test failed: {e}")

    def test_runtime_configuration_validation(self) -> None:
        """Test runtime configuration parameter validation."""
        try:
            # Test invalid worker threads (should handle gracefully)
            with contextlib.suppress(ValueError, RuntimeError):
                graphbit.configure_runtime(worker_threads=0)

            # Test invalid stack size
            with contextlib.suppress(ValueError, RuntimeError):
                graphbit.configure_runtime(thread_stack_size_mb=0)

            # Test very large values (should handle gracefully)
            try:
                graphbit.configure_runtime(worker_threads=1000)
                graphbit.configure_runtime(max_blocking_threads=10000)
            except Exception:
                pass  # nosec B110: acceptable in test context

        except Exception as e:
            pytest.fail(f"Runtime configuration validation test failed: {e}")


class TestSystemShutdown:
    """Integration tests for graceful system shutdown."""

    def test_basic_shutdown(self) -> None:
        """Test basic system shutdown functionality."""
        try:
            # Initialize first
            graphbit.init()

            # Test shutdown
            graphbit.shutdown()

            # System should still be functional after shutdown
            # (shutdown may just clean up resources)
            version = graphbit.version()
            assert isinstance(version, str)

        except Exception as e:
            pytest.fail(f"Basic shutdown test failed: {e}")

    def test_shutdown_and_reinit(self) -> None:
        """Test shutdown followed by reinitialization."""
        try:
            # Initialize
            graphbit.init()

            # Get initial system info
            info1 = graphbit.get_system_info()

            # Shutdown
            graphbit.shutdown()

            # Reinitialize
            graphbit.init()

            # Get system info again
            info2 = graphbit.get_system_info()

            # Basic properties should be consistent
            assert info1["version"] == info2["version"]
            assert info1["cpu_count"] == info2["cpu_count"]

        except Exception as e:
            pytest.fail(f"Shutdown and reinit test failed: {e}")


class TestSystemUtilities:
    """Integration tests for system utility functions."""

    def test_multiple_initializations(self) -> None:
        """Test multiple initializations don't cause issues."""
        try:
            # Multiple init calls should be safe
            for _i in range(5):
                graphbit.init()
                version = graphbit.version()
                assert isinstance(version, str)

        except Exception as e:
            pytest.fail(f"Multiple initializations test failed: {e}")

    def test_system_consistency(self) -> None:
        """Test system information consistency over time."""
        try:
            graphbit.init()

            # Get system info multiple times
            info_snapshots = []
            for _i in range(3):
                info = graphbit.get_system_info()
                info_snapshots.append(info)
                time.sleep(0.1)  # Small delay

            # Core properties should remain consistent
            consistent_keys = ["version", "cpu_count", "build_profile"]
            for key in consistent_keys:
                if key in info_snapshots[0]:
                    values = [info[key] for info in info_snapshots if key in info]
                    assert all(v == values[0] for v in values), f"Inconsistent {key}: {values}"

        except Exception as e:
            pytest.fail(f"System consistency test failed: {e}")

    def test_error_state_recovery(self) -> None:
        """Test system recovery from error states."""
        try:
            # Test invalid configuration followed by valid one
            with contextlib.suppress(Exception):
                graphbit.configure_runtime(worker_threads=-1)

            # System should still work after invalid config
            graphbit.init()
            version = graphbit.version()
            assert isinstance(version, str)

            # Valid configuration should work
            graphbit.configure_runtime(worker_threads=2)

        except Exception as e:
            pytest.fail(f"Error state recovery test failed: {e}")


@pytest.mark.integration
class TestSystemIntegration:
    """Integration tests for system-wide functionality."""

    def test_full_system_lifecycle(self) -> None:
        """Test complete system lifecycle from init to shutdown."""
        try:
            # 1. Initialize with configuration
            graphbit.init(log_level="info", enable_tracing=True)

            # 2. Configure runtime
            graphbit.configure_runtime(worker_threads=4, max_blocking_threads=8)

            # 3. Check system health
            health = graphbit.health_check()
            assert isinstance(health, dict)

            # 4. Get system information
            system_info = graphbit.get_system_info()
            assert isinstance(system_info, dict)
            assert system_info["runtime_initialized"] is True

            # 5. Perform some operations (create a simple component)
            version = graphbit.version()
            assert isinstance(version, str)

            # 6. Graceful shutdown
            graphbit.shutdown()

        except Exception as e:
            pytest.fail(f"Full system lifecycle test failed: {e}")

    def test_system_with_llm_integration(self) -> None:
        """Test system functions with LLM components."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        try:
            # Initialize system
            graphbit.init(enable_tracing=True)

            # Create LLM component
            config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
            client = graphbit.LlmClient(config)

            # System should report LLM integration in health check
            health = graphbit.health_check()
            system_info = graphbit.get_system_info()

            # System should remain stable
            assert isinstance(health, dict)
            assert isinstance(system_info, dict)

            # LLM should function properly
            response = client.complete("Test", max_tokens=5)
            assert isinstance(response, str)

        except Exception as e:
            pytest.fail(f"System with LLM integration test failed: {e}")
