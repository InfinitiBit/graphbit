#!/usr/bin/env python3
"""Integration tests for GraphBit embeddings functionality."""
import os
from typing import Any

import pytest

import graphbit


class TestOpenAIEmbeddings:
    """Integration tests for OpenAI embeddings."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
            raise RuntimeError("Should not reach here due to pytest.skip")
        return api_key

    @pytest.fixture
    def openai_config(self, api_key: str) -> Any:
        """Create OpenAI embedding configuration."""
        return graphbit.PyEmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")

    @pytest.fixture
    def openai_service(self, openai_config: Any) -> Any:
        """Create OpenAI embedding service."""
        return graphbit.PyEmbeddingService(openai_config)

    def test_openai_config_creation(self, openai_config: Any) -> None:
        """Test OpenAI config creation and properties."""
        assert openai_config.provider_name() == "openai"
        assert openai_config.model_name() == "text-embedding-3-small"

    def test_openai_config_customization(self, api_key: str) -> None:
        """Test OpenAI config customization."""
        config = graphbit.PyEmbeddingConfig.openai(api_key, "text-embedding-3-large")
        config = config.with_timeout(120)
        config = config.with_max_batch_size(100)
        config = config.with_base_url("https://api.openai.com/v1")

        assert config.model_name() == "text-embedding-3-large"

    def test_openai_service_creation(self, openai_service: Any) -> None:
        """Test OpenAI service creation."""
        provider, model = openai_service.get_provider_info()
        assert provider == "openai"
        assert model == "text-embedding-3-small"

    @pytest.mark.asyncio
    async def test_openai_single_embedding(self, openai_service: Any) -> None:
        """Test single text embedding with OpenAI."""
        try:
            embedding = await openai_service.embed_text("Hello world!")
            assert isinstance(embedding, list)
            assert len(embedding) > 0
            assert all(isinstance(x, float) for x in embedding)
        except Exception as e:
            pytest.fail(f"Single embedding failed: {e}")

    @pytest.mark.asyncio
    async def test_openai_multiple_embeddings(self, openai_service: Any) -> None:
        """Test multiple text embeddings with OpenAI."""
        texts = ["Hello", "World", "AI", "Machine Learning"]
        try:
            embeddings = await openai_service.embed_texts(texts)
            assert isinstance(embeddings, list)
            assert len(embeddings) == len(texts)
            assert all(isinstance(emb, list) for emb in embeddings)
        except Exception as e:
            pytest.fail(f"Multiple embeddings failed: {e}")

    @pytest.mark.asyncio
    async def test_openai_get_dimensions(self, openai_service: Any) -> None:
        """Test getting embedding dimensions from OpenAI."""
        try:
            dimensions = await openai_service.get_dimensions()
            assert isinstance(dimensions, int)
            assert dimensions > 0
        except Exception as e:
            pytest.fail(f"Get dimensions failed: {e}")

    @pytest.mark.asyncio
    async def test_openai_embedding_consistency(self, openai_service: Any) -> None:
        """Test embedding consistency for same text."""
        text = "Consistent embedding test"
        try:
            embedding1 = await openai_service.embed_text(text)
            embedding2 = await openai_service.embed_text(text)

            # Calculate similarity between embeddings
            similarity = graphbit.PyEmbeddingService.cosine_similarity(embedding1, embedding2)
            assert similarity > 0.99  # Should be very similar for same text
        except Exception as e:
            pytest.fail(f"Embedding consistency test failed: {e}")


class TestHuggingFaceEmbeddings:
    """Integration tests for HuggingFace embeddings."""

    @pytest.fixture
    def hf_api_key(self) -> str:
        """Get HuggingFace API key from environment."""
        api_key = os.getenv("HUGGINGFACE_API_KEY")
        if not api_key:
            pytest.skip("HUGGINGFACE_API_KEY not set")
            raise RuntimeError("Should not reach here due to pytest.skip")
        return api_key

    @pytest.fixture
    def hf_config(self, hf_api_key: str) -> Any:
        """Create HuggingFace embedding configuration."""
        return graphbit.PyEmbeddingConfig.huggingface(api_key=hf_api_key, model="sentence-transformers/all-MiniLM-L6-v2")

    @pytest.fixture
    def hf_service(self, hf_config: Any) -> Any:
        """Create HuggingFace embedding service."""
        return graphbit.PyEmbeddingService(hf_config)

    def test_hf_config_creation(self, hf_config: Any) -> None:
        """Test HuggingFace config creation and properties."""
        assert hf_config.provider_name() == "huggingface"
        assert hf_config.model_name() == "sentence-transformers/all-MiniLM-L6-v2"

    def test_hf_config_customization(self, hf_api_key: str) -> None:
        """Test HuggingFace config customization."""
        config = graphbit.PyEmbeddingConfig.huggingface(hf_api_key, "sentence-transformers/all-mpnet-base-v2")
        config = config.with_timeout(180)
        config = config.with_max_batch_size(32)
        config = config.with_base_url("https://api-inference.huggingface.co")

        assert config.model_name() == "sentence-transformers/all-mpnet-base-v2"

    def test_hf_service_creation(self, hf_service: Any) -> None:
        """Test HuggingFace service creation."""
        provider, model = hf_service.get_provider_info()
        assert provider == "huggingface"
        assert model == "sentence-transformers/all-MiniLM-L6-v2"

    @pytest.mark.asyncio
    async def test_hf_single_embedding(self, hf_service: Any) -> None:
        """Test single text embedding with HuggingFace."""
        try:
            embedding = await hf_service.embed_text("Natural language processing")
            assert isinstance(embedding, list)
            assert len(embedding) > 0
            assert all(isinstance(x, float) for x in embedding)
        except Exception as e:
            pytest.fail(f"HF single embedding failed: {e}")

    @pytest.mark.asyncio
    async def test_hf_multiple_embeddings(self, hf_service: Any) -> None:
        """Test multiple text embeddings with HuggingFace."""
        texts = ["Deep learning", "Neural networks", "Transformer models", "BERT"]
        try:
            embeddings = await hf_service.embed_texts(texts)
            assert isinstance(embeddings, list)
            assert len(embeddings) == len(texts)
            assert all(isinstance(emb, list) for emb in embeddings)
        except Exception as e:
            pytest.fail(f"HF multiple embeddings failed: {e}")

    @pytest.mark.asyncio
    async def test_hf_get_dimensions(self, hf_service: Any) -> None:
        """Test getting embedding dimensions from HuggingFace."""
        try:
            dimensions = await hf_service.get_dimensions()
            assert isinstance(dimensions, int)
            assert dimensions > 0
        except Exception as e:
            pytest.fail(f"HF get dimensions failed: {e}")


class TestEmbeddingUtilities:
    """Integration tests for embedding utility functions."""

    def test_cosine_similarity_identical(self) -> None:
        """Test cosine similarity with identical vectors."""
        vec1 = [1.0, 2.0, 3.0]
        vec2 = [1.0, 2.0, 3.0]
        similarity = graphbit.PyEmbeddingService.cosine_similarity(vec1, vec2)
        assert abs(similarity - 1.0) < 1e-6

    def test_cosine_similarity_orthogonal(self) -> None:
        """Test cosine similarity with orthogonal vectors."""
        vec1 = [1.0, 0.0, 0.0]
        vec2 = [0.0, 1.0, 0.0]
        similarity = graphbit.PyEmbeddingService.cosine_similarity(vec1, vec2)
        assert abs(similarity - 0.0) < 1e-6

    def test_cosine_similarity_opposite(self) -> None:
        """Test cosine similarity with opposite vectors."""
        vec1 = [1.0, 0.0, 0.0]
        vec2 = [-1.0, 0.0, 0.0]
        similarity = graphbit.PyEmbeddingService.cosine_similarity(vec1, vec2)
        assert abs(similarity - (-1.0)) < 1e-6

    def test_embedding_request_single(self) -> None:
        """Test embedding request creation for single text."""
        request = graphbit.PyEmbeddingRequest("Test text")
        assert request is not None

    def test_embedding_request_multiple(self) -> None:
        """Test embedding request creation for multiple texts."""
        texts = ["Text 1", "Text 2", "Text 3"]
        request = graphbit.PyEmbeddingRequest.multiple(texts)
        assert request is not None

    def test_embedding_request_with_user(self) -> None:
        """Test embedding request creation with user identifier."""
        request = graphbit.PyEmbeddingRequest("Test text")
        request = request.with_user("test_user")
        assert request is not None

    def test_embedding_response_handling(self) -> None:
        """Test embedding response handling."""
        # This would typically test response parsing
        # Implementation depends on the actual response structure
        assert hasattr(graphbit, "PyEmbeddingResponse")


@pytest.mark.integration
class TestCrossProviderEmbeddings:
    """Integration tests comparing different embedding providers."""

    @pytest.fixture
    def providers(self) -> Any:
        """Set up multiple embedding providers for comparison."""
        providers = {}

        # OpenAI
        openai_key = os.getenv("OPENAI_API_KEY")
        if openai_key:
            openai_config = graphbit.PyEmbeddingConfig.openai(openai_key, "text-embedding-3-small")
            providers["openai"] = graphbit.PyEmbeddingService(openai_config)

        # HuggingFace
        hf_key = os.getenv("HUGGINGFACE_API_KEY")
        if hf_key:
            hf_config = graphbit.PyEmbeddingConfig.huggingface(hf_key, "sentence-transformers/all-MiniLM-L6-v2")
            providers["huggingface"] = graphbit.PyEmbeddingService(hf_config)

        if not providers:
            pytest.skip("No embedding providers available")

        return providers

    @pytest.mark.asyncio
    async def test_cross_provider_embedding_comparison(self, providers: Any) -> None:
        """Test embedding comparison across different providers."""
        text = "Machine learning is transforming technology."

        embeddings = {}
        for name, service in providers.items():
            try:
                embedding = await service.embed_text(text)
                embeddings[name] = embedding
            except Exception as e:
                pytest.fail(f"{name} embedding failed: {e}")

        # Compare embedding dimensions
        dimensions = {name: len(emb) for name, emb in embeddings.items()}
        assert all(dim > 0 for dim in dimensions.values())

        # If we have multiple providers, compare similarities
        if len(embeddings) > 1:
            provider_names = list(embeddings.keys())
            for i in range(len(provider_names)):
                for j in range(i + 1, len(provider_names)):
                    name1, name2 = provider_names[i], provider_names[j]
                    emb1, emb2 = embeddings[name1], embeddings[name2]

                    # Note: Embeddings from different providers might have different
                    # dimensions and scales, so direct comparison might not be meaningful
                    assert len(emb1) > 0
                    assert len(emb2) > 0

    @pytest.mark.asyncio
    async def test_cross_provider_batch_processing(self, providers: Any) -> None:
        """Test batch processing capabilities across providers."""
        texts = [
            "Artificial intelligence",
            "Machine learning algorithms",
            "Neural network architectures",
            "Deep learning models",
        ]

        for name, service in providers.items():
            try:
                embeddings = await service.embed_texts(texts)
                assert len(embeddings) == len(texts)
                assert all(isinstance(emb, list) for emb in embeddings)
                assert all(len(emb) > 0 for emb in embeddings)
            except Exception as e:
                pytest.fail(f"{name} batch processing failed: {e}")


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])
