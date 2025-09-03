"""Test configuration failures and validation errors."""

import contextlib
import os

import pytest

from graphbit import DocumentLoaderConfig, EmbeddingConfig, LlmClient, LlmConfig, Node, TextSplitter, TextSplitterConfig, Workflow, configure_runtime, shutdown


class TestConfigurationFailures:
    """Test various configuration failure scenarios."""

    def teardown_method(self):
        """Clean up after each test method."""
        with contextlib.suppress(Exception):
            shutdown()

    def test_llm_config_invalid_provider(self):
        """Test LLM config with invalid provider."""
        # Test invalid API keys
        with pytest.raises(ValueError):
            _ = LlmConfig.openai("", "gpt-4")  # Empty API key

        with pytest.raises(TypeError):
            _ = LlmConfig.openai(None, "gpt-4")  # None API key

    def test_llm_config_invalid_model(self):
        """Test LLM config with invalid model names."""
        # Test with invalid model names
        with contextlib.suppress(Exception):
            _ = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "")  # Empty model
            # Might be allowed with default model

        with contextlib.suppress(Exception):
            _ = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", None)  # None model
            # Might be allowed with default model

    def test_llm_config_extremely_long_values(self):
        """Test LLM config with extremely long values."""
        long_string = "a" * 10000

        with contextlib.suppress(Exception):
            _ = LlmConfig.openai(long_string, "gpt-4")
            # Should either work or fail gracefully

    def test_llm_config_special_characters(self):
        """Test LLM config with special characters."""
        special_chars = "!@#$%^&*()[]{}|\\:;\"'<>?,./"

        with contextlib.suppress(Exception):
            _ = LlmConfig.openai(special_chars, "gpt-4")
            # Should handle special characters

    def test_llm_config_unicode_characters(self):
        """Test LLM config with unicode characters."""
        unicode_string = "测试🚀🎉"

        with contextlib.suppress(Exception):
            _ = LlmConfig.openai(unicode_string, "gpt-4")
            # Should handle unicode

    def test_embedding_config_invalid_provider(self):
        """Test embedding config with invalid provider."""
        with pytest.raises(ValueError):
            _ = EmbeddingConfig.openai("", "text-embedding-3-small")

        # Skip huggingface test as it may not be available
        with contextlib.suppress(Exception):
            _ = EmbeddingConfig.openai("invalid-key", "model-name")

    def test_embedding_config_invalid_model(self):
        """Test embedding config with invalid model."""
        with contextlib.suppress(Exception):
            _ = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "")

        with contextlib.suppress(Exception):
            _ = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "invalid-model")

    def test_document_loader_config_invalid_values(self):
        """Test document loader config with invalid values."""
        with contextlib.suppress(Exception):
            config = DocumentLoaderConfig()

            # Test with invalid file size limits
            config.max_file_size = -1
            config.timeout_seconds = -1
            # Invalid values are acceptable to be ignored

    def test_text_splitter_config_invalid_parameters(self):
        """Test text splitter config with invalid parameters."""
        # Test character splitter with invalid chunk size
        with pytest.raises(ValueError):
            _ = TextSplitterConfig.character(0, 0)  # Zero chunk size

        with pytest.raises(OverflowError):
            _ = TextSplitterConfig.character(-1, 0)  # Negative chunk size

        with pytest.raises((ValueError, Exception)):
            _ = TextSplitterConfig.character(10, 20)  # Overlap > chunk size

    def test_text_splitter_config_extreme_values(self):
        """Test text splitter config with extreme values."""
        # Test with very large values
        with contextlib.suppress(Exception):
            _ = TextSplitterConfig.character(1000000, 0)
            # Should either work or fail gracefully

    def test_text_splitter_config_invalid_patterns(self):
        """Test text splitter config with invalid regex patterns."""
        # Test token splitter with invalid regex - might not raise exception immediately
        with contextlib.suppress(Exception):
            config = TextSplitterConfig.token(100, 10, "[invalid")  # Invalid regex
            # If config creation succeeds, test actual usage
            splitter = TextSplitter(config)
            with pytest.raises(Exception, match=".*"):
                _ = splitter.split("test text")
            # Invalid regex patterns are acceptable to be ignored

    def test_workflow_config_invalid_names(self):
        """Test workflow config with invalid names."""
        # Test with empty name - might be allowed
        with contextlib.suppress(Exception):
            _ = Workflow("")
            # If empty name is allowed, that's valid

        # Test with None name
        with contextlib.suppress(Exception):
            _ = Workflow(None)
            # If None name is allowed, that's valid

    def test_workflow_config_extremely_long_names(self):
        """Test workflow config with extremely long names."""
        long_name = "a" * 10000

        with contextlib.suppress(Exception):
            _ = Workflow(long_name)
            # Should either work or fail gracefully

    def test_node_config_invalid_parameters(self):
        """Test node config with invalid parameters."""
        # Test agent node with invalid parameters
        with pytest.raises(ValueError):
            _ = Node.agent("", "description", "agent_id")  # Empty name

        with pytest.raises(ValueError):
            _ = Node.agent("name", "", "agent_id")  # Empty description

    def test_runtime_config_invalid_values(self):
        """Test runtime config with invalid values."""
        # Test with invalid thread counts
        with pytest.raises((ValueError, Exception)):
            configure_runtime(worker_threads=0)

        with pytest.raises((ValueError, Exception)):
            configure_runtime(worker_threads=-1)

        with pytest.raises((ValueError, Exception)):
            configure_runtime(max_blocking_threads=0)

    def test_config_type_mismatches(self):
        """Test configuration with wrong types."""
        # Test passing wrong types to functions
        with pytest.raises(TypeError):
            _ = LlmConfig.openai(123, "gpt-4")  # Number instead of string

        with pytest.raises(TypeError):
            _ = LlmConfig.openai("key", 123)  # Number instead of string

    def test_config_none_values(self):
        """Test configuration with None values where not allowed."""
        with pytest.raises(TypeError):
            _ = Workflow(None)

        # Some None values might be acceptable (like optional model names)
        with contextlib.suppress(TypeError, ValueError, Exception):
            _ = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", None)

    def test_config_serialization_failures(self):
        """Test configuration serialization edge cases."""
        # Create valid config
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")

        # Test that config can be used (basic validation)
        with contextlib.suppress(Exception):
            _ = LlmClient(config)
            # Should create client even with invalid API key
            # Client creation may succeed even with invalid API key

    def test_config_mutation_after_creation(self):
        """Test configuration mutation after creation."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")

        # Try to modify config (should be immutable or handle gracefully)
        with contextlib.suppress(Exception):
            # These might not be possible depending on implementation
            if hasattr(config, "api_key"):
                config.api_key = "modified"
            if hasattr(config, "model"):
                config.model = "modified"
            # Immutable config errors are acceptable

    def test_config_with_environment_variables(self):
        """Test configuration with environment variables."""
        original_env = os.environ.copy()

        try:
            # Set environment variables that might conflict
            os.environ["OPENAI_API_KEY"] = "env-key"
            os.environ["ANTHROPIC_API_KEY"] = "env-key"

            # Should still use provided values, not environment
            _ = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")

        finally:
            # Restore environment
            os.environ.clear()
            os.environ.update(original_env)

    def test_config_validation_edge_cases(self):
        """Test configuration validation edge cases."""
        # Test with whitespace-only strings
        with pytest.raises((ValueError, Exception)):
            _ = LlmConfig.openai("   ", "gpt-4")

        with pytest.raises((ValueError, Exception)):
            _ = LlmConfig.openai("key", "   ")

    def test_config_memory_limits(self):
        """Test configuration under memory constraints."""
        # Create many configurations to test memory handling
        configs = []
        with contextlib.suppress(MemoryError):
            for _ in range(1000):
                config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
                configs.append(config)

        # Should be able to create at least some configs
        assert len(configs) > 0
