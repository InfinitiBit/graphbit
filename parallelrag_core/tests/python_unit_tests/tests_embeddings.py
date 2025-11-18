"""Unit tests for embedding clients."""

import os

import pytest

from graphbit import EmbeddingClient, EmbeddingConfig


def get_api_key(provider: str) -> str:
    """Get API key from environment variables."""
    key = os.getenv(f"{provider.upper()}_API_KEY")
    if not key:
        pytest.skip(f"No {provider.upper()}_API_KEY found in environment")
    assert key is not None
    return key


class TestEmbeddingConfig:
    """Test embedding configuration classes."""

    def test_embedding_config_creation_openai(self):
        """Test creating OpenAI embedding configuration."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        assert config is not None

    def test_embedding_config_creation_huggingface(self):
        """Test creating HuggingFace embedding configuration."""
        api_key = get_api_key("huggingface")
        config = EmbeddingConfig.huggingface(api_key=api_key, model="sentence-transformers/all-MiniLM-L6-v2")
        assert config is not None

    def test_embedding_config_validation(self):
        """Test embedding configuration validation."""
        # Test with empty API key
        with pytest.raises((ValueError, TypeError)):
            EmbeddingConfig.openai(api_key="")

        # Test with empty model for HuggingFace
        with pytest.raises((ValueError, TypeError)):
            EmbeddingConfig.huggingface(api_key="test", model="")


class TestEmbeddingClient:
    """Test embedding client functionality."""

    def test_embedding_client_creation_openai(self):
        """Test creating OpenAI embedding client."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)
        assert client is not None

    def test_embedding_client_creation_huggingface(self):
        """Test creating HuggingFace embedding client."""
        api_key = get_api_key("huggingface")
        config = EmbeddingConfig.huggingface(api_key=api_key, model="sentence-transformers/all-MiniLM-L6-v2")
        client = EmbeddingClient(config)
        assert client is not None

    def test_embedding_client_creation_invalid_config(self):
        """Test creating embedding client with invalid configuration."""
        with pytest.raises((ValueError, TypeError)):
            EmbeddingClient("invalid_config")

    def test_embedding_client_embed_text_openai(self):
        """Test OpenAI embedding generation for single text."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)

        sample_text = "This is a test text for embedding."
        embedding = client.embed(sample_text)
        assert embedding is not None
        assert isinstance(embedding, list)
        assert len(embedding) > 0
        assert all(isinstance(x, float) for x in embedding)

    def test_embedding_client_embed_texts_openai(self):
        """Test OpenAI embedding generation for multiple texts."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)

        sample_texts = ["This is the first test text.", "This is the second test text.", "This is the third test text."]
        embeddings = client.embed_many(sample_texts)
        assert embeddings is not None
        assert isinstance(embeddings, list)
        assert len(embeddings) == len(sample_texts)
        for embedding in embeddings:
            assert isinstance(embedding, list)
            assert len(embedding) > 0
            assert all(isinstance(x, float) for x in embedding)


class TestEmbeddingClientErrorHandling:
    """Test embedding client error handling."""

    def test_empty_api_key(self):
        """Test creating client with empty API key."""
        with pytest.raises((ValueError, TypeError)):
            EmbeddingConfig.openai(api_key="")

    def test_empty_model_huggingface(self):
        """Test creating client with empty model."""
        with pytest.raises((ValueError, TypeError)):
            EmbeddingConfig.huggingface(api_key="test_key", model="")

    def test_empty_text_embedding(self):
        """Test embedding empty text."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)

        with pytest.raises((ValueError, TypeError)):
            client.embed("")

    def test_none_text_embedding(self):
        """Test embedding None text."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)

        with pytest.raises((ValueError, TypeError)):
            client.embed(None)

    def test_empty_list_embeddings(self):
        """Test embedding empty list of texts."""
        api_key = get_api_key("openai")
        config = EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = EmbeddingClient(config)

        with pytest.raises((ValueError, TypeError)):
            client.embed_many([])
