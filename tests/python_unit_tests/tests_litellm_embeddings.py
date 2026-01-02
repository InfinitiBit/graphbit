"""Unit tests for LiteLLM embedding configuration."""

import pytest
from graphbit import EmbeddingConfig

class TestLiteLLMConfig:
    """Test LiteLLM embedding configuration."""

    def test_embedding_config_creation_litellm(self):
        """Test creating LiteLLM embedding configuration."""
        # Use dummy key as LiteLLM mostly uses environment variables or doesn't validate in init
        api_key = "sk-dummy-key-for-testing-litellm" 
        config = EmbeddingConfig.litellm(api_key=api_key)
        assert config is not None
        
    def test_embedding_config_creation_litellm_with_model(self):
        """Test creating LiteLLM embedding configuration with specific model."""
        api_key = "sk-dummy-key-for-testing-litellm"
        model = "text-embedding-3-large"
        config = EmbeddingConfig.litellm(api_key=api_key, model=model)
        assert config is not None

    def test_embedding_config_validation_litellm(self):
        """Test LiteLLM embedding configuration validation."""
        # Test with empty API key - should fail due to generic validation
        with pytest.raises((ValueError, TypeError)):
            EmbeddingConfig.litellm(api_key="")
