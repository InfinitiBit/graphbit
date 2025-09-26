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
        assert config.model() == "lucataco/glaive-function-calling-v1"  # Default model

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
            "lucataco/glaive-function-calling-v1:cd9c955362e3fb2278764e130497f4013a0aaf7217f1cf7540bebab40f01fa8a",
            "homanp/llama-2-13b-function-calling:2288c783ba83e28b9ac4906e2dfa8004e3eda67f11ffc7a6a80bd927e46bc6c9",
            "lucataco/termes-2-pro-llama-3-8b:51ca4143b8464e9dfeb0c88339962a6bdf2ffd833f047ed293b38537a92c1515",
            "lucataco/dolphin-2.9-llama3-8b:ee173688d3b8d9e05a5b910f10fb9bab1e9348963ab224579bb90d9fce3fb00b",
            "ibm-granite/granite-3.3-8b-instruct:8afd11cc386bd05622227e71b208b9ecc000b946d84d373be96090f38ec91bdf",
        ]

        for model in function_calling_models:
            config = LlmConfig.replicate("test-api-key", model=model)
            assert config.provider() == "replicate"
            assert config.model() == model

    def test_replicate_config_with_none_values(self):
        """Test Replicate configuration with None values."""
        config = LlmConfig.replicate("test-api-key", model=None, version=None)
        assert config.provider() == "replicate"
        assert config.model() == "lucataco/glaive-function-calling-v1"  # Should use default

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

    @pytest.mark.skipif(not os.getenv("REPLICATE_API_KEY"), reason="REPLICATE_API_KEY environment variable not set")
    def test_replicate_real_api_connection(self):
        """Test actual connection to Replicate API (requires real API key)."""
        api_key = os.getenv("REPLICATE_API_KEY")
        config = LlmConfig.replicate(api_key, model="llucataco/dolphin-2.9-llama3-8b:ee173688d3b8d9e05a5b910f10fb9bab1e9348963ab224579bb90d9fce3fb00b")
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

    def test_glaive_function_calling_model(self):
        """Test Glaive function calling model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/glaive-function-calling-v1:cd9c955362e3fb2278764e130497f4013a0aaf7217f1cf7540bebab40f01fa8a")
        assert config.model() == "lucataco/glaive-function-calling-v1:cd9c955362e3fb2278764e130497f4013a0aaf7217f1cf7540bebab40f01fa8a"

    def test_llama2_function_calling_model(self):
        """Test Llama-2 function calling model configuration."""
        config = LlmConfig.replicate("test-key", model="homanp/llama-2-13b-function-calling", version="2288c783ba83e28b9ac4906e2dfa8004e3eda67f11ffc7a6a80bd927e46bc6c9")
        assert config.model() == "homanp/llama-2-13b-function-calling"

    def test_hermes_model(self):
        """Test Hermes model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/hermes-2-pro-llama-3-8b:51ca4143b8464e9dfeb0c88339962a6bdf2ffd833f047ed293b38537a92c1515")
        assert config.model() == "lucataco/hermes-2-pro-llama-3-8b:51ca4143b8464e9dfeb0c88339962a6bdf2ffd833f047ed293b38537a92c1515"

    def test_dolphin_model(self):
        """Test Dolphin model configuration."""
        config = LlmConfig.replicate("test-key", model="lucataco/dolphin-2.9-llama3-8b:ee173688d3b8d9e05a5b910f10fb9bab1e9348963ab224579bb90d9fce3fb00b")
        assert config.model() == "lucataco/dolphin-2.9-llama3-8b:ee173688d3b8d9e05a5b910f10fb9bab1e9348963ab224579bb90d9fce3fb00b"

    def test_granite_model(self):
        """Test Granite model configuration."""
        config = LlmConfig.replicate("test-key", model="ibm-granite/granite-3.3-8b-instruct:8afd11cc386bd05622227e71b208b9ecc000b946d84d373be96090f38ec91bdf")
        assert config.model() == "ibm-granite/granite-3.3-8b-instruct:8afd11cc386bd05622227e71b208b9ecc000b946d84d373be96090f38ec91bdf"


if __name__ == "__main__":
    pytest.main([__file__])
