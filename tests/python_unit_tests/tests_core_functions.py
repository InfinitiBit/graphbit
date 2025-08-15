"""Unit tests for core GraphBit functions."""

import pytest

from graphbit import configure_runtime, get_system_info, health_check, shutdown, version


class TestCoreInitialization:
    """Test core initialization and system functions."""

    def test_version(self):
        """Test version retrieval."""
        v = version()
        assert isinstance(v, str)
        assert len(v) > 0

    def test_get_system_info(self):
        """Test system info retrieval."""
        info = get_system_info()
        assert isinstance(info, dict)
        assert "version" in info
        assert "cpu_count" in info
        assert "runtime_initialized" in info

    def test_health_check(self):
        """Test health check."""
        h = health_check()
        assert isinstance(h, dict)
        assert "overall_healthy" in h
        assert "runtime_healthy" in h
        assert "timestamp" in h

    def test_configure_runtime_invalid_params(self):
        """Test runtime configuration with invalid parameters."""
        with pytest.raises(ValueError):
            configure_runtime(worker_threads=-1)

        with pytest.raises(ValueError):
            configure_runtime(max_blocking_threads=0)

        with pytest.raises(ValueError):
            configure_runtime(thread_stack_size_mb=-5)

    def test_configure_runtime_valid_params(self):
        """Test runtime configuration with valid parameters."""
        # This should not raise an exception
        configure_runtime(worker_threads=2, max_blocking_threads=4, thread_stack_size_mb=2)


class TestModuleAttributes:
    """Test module-level attributes."""

    def test_module_version(self):
        """Test module version attribute."""
        import graphbit

        assert hasattr(graphbit, "__version__")
        assert isinstance(graphbit.__version__, str)

    def test_module_author(self):
        """Test module author attribute."""
        import graphbit

        assert hasattr(graphbit, "__author__")
        assert isinstance(graphbit.__author__, str)

    def test_module_description(self):
        """Test module description attribute."""
        import graphbit

        assert hasattr(graphbit, "__description__")
        assert isinstance(graphbit.__description__, str)

    def test_module_exports(self):
        """Test that required classes are exported."""
        import graphbit

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
            "DocumentLoader",
            "Node",
            "Workflow",
            "WorkflowContext",
            "WorkflowResult",
            "Executor",
            "TextSplitter",
        ]

        for class_name in required_classes:
            assert hasattr(graphbit, class_name), f"Missing class: {class_name}"


class TestErrorHandling:
    """Test error handling and edge cases."""

    def test_shutdown_cleanup(self):
        """Test shutdown function for cleanup."""
        # Should not raise an exception
        shutdown()
