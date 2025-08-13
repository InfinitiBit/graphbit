"""Unit tests for core GraphBit functions."""

import pytest

import graphbit


class TestCoreInitialization:
    """Test core initialization and system functions."""

    def test_version(self):
        """Test version retrieval."""
        version = graphbit.version()
        assert isinstance(version, str)
        assert len(version) > 0

    def test_get_system_info(self):
        """Test system info retrieval."""
        info = graphbit.get_system_info()
        assert isinstance(info, dict)
        assert "version" in info
        assert "cpu_count" in info
        assert "runtime_initialized" in info

    def test_health_check(self):
        """Test health check."""
        health = graphbit.health_check()
        assert isinstance(health, dict)
        assert "overall_healthy" in health
        assert "runtime_healthy" in health
        assert "timestamp" in health

    def test_configure_runtime_invalid_params(self):
        """Test runtime configuration with invalid parameters."""
        with pytest.raises(ValueError):
            graphbit.configure_runtime(worker_threads=-1)

        with pytest.raises(ValueError):
            graphbit.configure_runtime(max_blocking_threads=0)

        with pytest.raises(ValueError):
            graphbit.configure_runtime(thread_stack_size_mb=-5)

    def test_configure_runtime_valid_params(self):
        """Test runtime configuration with valid parameters."""
        # This should not raise an exception
        graphbit.configure_runtime(worker_threads=2, max_blocking_threads=4, thread_stack_size_mb=2)

    def test_init_multiple_calls(self):
        """Test that multiple init calls don't cause issues."""
        # Should not raise exceptions
        graphbit.init()
        graphbit.init(enable_tracing=False)
        graphbit.init(log_level="warn")


class TestModuleAttributes:
    """Test module-level attributes."""

    def test_module_version(self):
        """Test module version attribute."""
        assert hasattr(graphbit, "__version__")
        assert isinstance(graphbit.__version__, str)

    def test_module_author(self):
        """Test module author attribute."""
        assert hasattr(graphbit, "__author__")
        assert isinstance(graphbit.__author__, str)

    def test_module_description(self):
        """Test module description attribute."""
        assert hasattr(graphbit, "__description__")
        assert isinstance(graphbit.__description__, str)

    def test_module_exports(self):
        """Test that required classes are exported."""
        required_classes = [
            "LlmConfig",
            "LlmClient",
            "EmbeddingConfig",
            "EmbeddingClient",
            "TextSplitterConfig",
            "TextChunk",
            "CharacterSplitter",
            "TokenSplitter",
            "SentenceSplitter",
            "RecursiveSplitter",
            "DocumentLoaderConfig",
            "DocumentContent",
            "DocumentLoader",  # Updated class names
            "Node",
            "Workflow",
            "WorkflowContext",
            "WorkflowResult",
            "Executor",
            "TextSplitter",  # Added missing class
        ]

        for class_name in required_classes:
            assert hasattr(graphbit, class_name), f"Missing class: {class_name}"


class TestErrorHandling:
    """Test error handling and edge cases."""

    def test_shutdown_cleanup(self):
        """Test shutdown function for cleanup."""
        # Should not raise an exception
        graphbit.shutdown()

        # Re-initialize for other tests
        graphbit.init(enable_tracing=False)
