"""Unit tests for Replicate AI provider in GraphBit Python bindings."""

import os
import sys

import pytest

from graphbit import LlmClient, LlmConfig

# Add the project root to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", ".."))


class TestReplicateProvider:
    """Test cases for Replicate AI provider configuration and functionality."""

    def test_replicate_config_creation(self):
        """Test creating Replicate configuration."""
        config = LlmConfig.replicate("test-api-key")
        assert config.provider() == "replicate"
        assert config.model() == "openai/gpt-5"  # Default model

    def test_replicate_config_with_version(self):
        """Test creating Replicate configuration with specific version."""
        config = LlmConfig.replicate("test-api-key", model="homanp/llama-2-13b-function-calling", version="2288c783ba83e28b9ac4906e2dfa8004e3eda67f11ffc7a6a80bd927e46bc6c9")
        assert config.provider() == "replicate"
        assert config.model() == "homanp/llama-2-13b-function-calling"

    def test_replicate_config_validation(self):
        """Test that Replicate configuration validates API key."""
        with pytest.raises(ValueError, match="API key"):
            LlmConfig.replicate("")

        with pytest.raises(ValueError, match="API key"):
            LlmConfig.replicate("   ")

    def test_replicate_client_creation(self):
        """Test creating LLM client with Replicate configuration."""
        config = LlmConfig.replicate("test-api-key")
        client = LlmClient(config)
        assert client is not None

    def test_replicate_function_calling_models(self):
        """Test that known function calling models are properly configured."""
        function_calling_models = [
            "openai/gpt-5",
            "openai/gpt-5-structured",
            "lucataco/glaive-function-calling-v1",
            "homanp/llama-2-13b-function-calling",
            "lucataco/hermes-2-pro-llama-3-8b",
            "lucataco/dolphin-2.9-llama3-8b",
            "ibm-granite/granite-3.3-8b-instruct",
        ]

        for model in function_calling_models:
            config = LlmConfig.replicate("test-api-key", model=model)
            assert config.provider() == "replicate"
            assert config.model() == model

    def test_replicate_config_with_none_values(self):
        """Test Replicate configuration with None values."""
        config = LlmConfig.replicate("test-api-key", model=None, version=None)
        assert config.provider() == "replicate"
        assert config.model() == "openai/gpt-5"  # Should use default

    def test_replicate_config_string_representation(self):
        """Test string representation of Replicate configuration."""
        config = LlmConfig.replicate("test-api-key", model="test-model")
        provider_str = config.provider()
        model_str = config.model()

        assert isinstance(provider_str, str)
        assert isinstance(model_str, str)
        assert provider_str == "replicate"
        assert model_str == "test-model"


class TestReplicateIntegration:
    """Integration tests for Replicate provider (require actual API key)."""

    @pytest.mark.skipif(not os.getenv("REPLICATE_API_KEY"), reason="Skipped: REPLICATE_API_KEY not set")
    def test_replicate_real_api_connection(self):
        """Test actual connection to Replicate API (requires real API key)."""
        api_key = os.getenv("REPLICATE_API_KEY")
        config = LlmConfig.replicate(api_key, model="lucataco/dolphin-2.9-llama3-8b")
        client = LlmClient(config)

        # This test just verifies the client can be created with a real API key
        # Actual API calls would require more complex setup and might be expensive
        assert client is not None
        assert config.provider() == "replicate"

    def test_replicate_error_handling(self):
        """Test error handling for invalid configurations."""
        # Test with invalid API key format
        with pytest.raises(ValueError, match="API key"):
            LlmConfig.replicate("")

        # Test client creation with invalid config should not fail at creation time
        # (errors typically occur during actual API calls)
        config = LlmConfig.replicate("invalid-key")
        client = LlmClient(config)
        assert client is not None


class TestReplicateModels:
    """Test specific Replicate model configurations."""

    def test_gpt5_model(self):
        """Test GPT-5 model configuration."""
        config = LlmConfig.replicate("test-key", model="openai/gpt-5")
        assert config.model() == "openai/gpt-5"

    def test_gpt5_structured_model(self):
        """Test GPT-5 structured model configuration."""
        config = LlmConfig.replicate("test-key", model="openai/gpt-5-structured")
        assert config.model() == "openai/gpt-5-structured"

    def test_glaive_function_calling_model(self):
        """Test Glaive function calling model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/glaive-function-calling-v1")
        assert config.model() == "lucataco/glaive-function-calling-v1"

    def test_llama2_function_calling_model(self):
        """Test Llama-2 function calling model configuration."""
        config = LlmConfig.replicate("test-key", model="homanp/llama-2-13b-function-calling", version="2288c783ba83e28b9ac4906e2dfa8004e3eda67f11ffc7a6a80bd927e46bc6c9")
        assert config.model() == "homanp/llama-2-13b-function-calling"

    def test_hermes_model(self):
        """Test Hermes model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/hermes-2-pro-llama-3-8b")
        assert config.model() == "lucataco/hermes-2-pro-llama-3-8b"

    def test_dolphin_model(self):
        """Test Dolphin model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/dolphin-2.9-llama3-8b")
        assert config.model() == "lucataco/dolphin-2.9-llama3-8b"

    def test_granite_model(self):
        """Test Granite model configuration."""
        config = LlmConfig.replicate("test-key", model="ibm-granite/granite-3.3-8b-instruct")
        assert config.model() == "ibm-granite/granite-3.3-8b-instruct"


if __name__ == "__main__":
    pytest.main([__file__])
