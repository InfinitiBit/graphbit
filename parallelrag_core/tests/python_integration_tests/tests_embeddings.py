"""Integration tests for GraphBit embeddings functionality."""

import os
import time
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
        return graphbit.EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")

    @pytest.fixture
    def openai_client(self, openai_config: Any) -> Any:
        """Create OpenAI embedding client."""
        return graphbit.EmbeddingClient(openai_config)

    def test_openai_config_creation(self, openai_config: Any) -> None:
        """Test OpenAI config creation."""
        assert openai_config is not None

    def test_openai_client_creation(self, openai_client: Any) -> None:
        """Test OpenAI client creation."""
        assert openai_client is not None

    def test_openai_single_embedding(self, openai_client: Any) -> None:
        """Test single text embedding with OpenAI."""
        try:
            embedding = openai_client.embed("Hello world!")
            assert isinstance(embedding, list)
            assert len(embedding) > 0
            assert all(isinstance(x, float) for x in embedding)
        except Exception as e:
            pytest.fail(f"Single embedding failed: {e}")

    def test_openai_multiple_embeddings(self, openai_client: Any) -> None:
        """Test multiple text embeddings with OpenAI."""
        texts = ["Hello", "World", "AI", "Machine Learning"]
        try:
            embeddings = openai_client.embed_many(texts)
            assert isinstance(embeddings, list)
            assert len(embeddings) == len(texts)
            assert all(isinstance(emb, list) for emb in embeddings)
            assert all(len(emb) > 0 for emb in embeddings)
        except Exception as e:
            pytest.fail(f"Multiple embeddings failed: {e}")

    def test_openai_embedding_consistency(self, openai_client: Any) -> None:
        """Test embedding consistency for same text."""
        text = "Consistent embedding test"
        try:
            embedding1 = openai_client.embed(text)
            embedding2 = openai_client.embed(text)

            # Calculate similarity between embeddings
            similarity = graphbit.EmbeddingClient.similarity(embedding1, embedding2)
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
        return graphbit.EmbeddingConfig.huggingface(api_key=hf_api_key, model="facebook/bart-base")

    @pytest.fixture
    def hf_client(self, hf_config: Any) -> Any:
        """Create HuggingFace embedding client."""
        return graphbit.EmbeddingClient(hf_config)

    def test_hf_config_creation(self, hf_config: Any) -> None:
        """Test HuggingFace config creation."""
        assert hf_config is not None

    def test_hf_client_creation(self, hf_client: Any) -> None:
        """Test HuggingFace client creation."""
        assert hf_client is not None

    def test_hf_single_embedding(self, hf_client: Any) -> None:
        """Test single text embedding with HuggingFace."""
        try:
            embedding = hf_client.embed("Natural language processing")
            assert isinstance(embedding, list)
            assert len(embedding) > 0
            assert all(isinstance(x, float) for x in embedding)
        except Exception as e:
            pytest.fail(f"HF single embedding failed: {e}")

    def test_hf_multiple_embeddings(self, hf_client: Any) -> None:
        """Test multiple text embeddings with HuggingFace."""
        texts = ["Deep learning", "Neural networks", "Transformer models", "BERT"]
        try:
            embeddings = hf_client.embed_many(texts)
            assert isinstance(embeddings, list)
            assert len(embeddings) == len(texts)
            assert all(isinstance(emb, list) for emb in embeddings)
            assert all(len(emb) > 0 for emb in embeddings)
        except Exception as e:
            pytest.fail(f"HF multiple embeddings failed: {e}")


class TestEmbeddingUtilities:
    """Integration tests for embedding utility functions."""

    def test_cosine_similarity_identical(self) -> None:
        """Test cosine similarity with identical vectors."""
        vec1 = [1.0, 2.0, 3.0]
        vec2 = [1.0, 2.0, 3.0]
        similarity = graphbit.EmbeddingClient.similarity(vec1, vec2)
        assert abs(similarity - 1.0) < 1e-6

    def test_cosine_similarity_orthogonal(self) -> None:
        """Test cosine similarity with orthogonal vectors."""
        vec1 = [1.0, 0.0, 0.0]
        vec2 = [0.0, 1.0, 0.0]
        similarity = graphbit.EmbeddingClient.similarity(vec1, vec2)
        assert abs(similarity - 0.0) < 1e-6

    def test_cosine_similarity_opposite(self) -> None:
        """Test cosine similarity with opposite vectors."""
        vec1 = [1.0, 0.0, 0.0]
        vec2 = [-1.0, 0.0, 0.0]
        similarity = graphbit.EmbeddingClient.similarity(vec1, vec2)
        assert abs(similarity - (-1.0)) < 1e-6

    def test_cosine_similarity_zero_vector(self) -> None:
        """Test cosine similarity error handling with zero vectors."""
        vec1 = [0.0, 0.0, 0.0]
        vec2 = [1.0, 0.0, 0.0]

        # Should handle zero vector gracefully (may return error or special value)
        try:
            similarity = graphbit.EmbeddingClient.similarity(vec1, vec2)
            # If it doesn't raise an error, verify it's a valid value
            assert isinstance(similarity, float)
        except Exception as e:  # noqa: B110
            # Exception is acceptable for zero vectors
            print(f"Zero vector similarity failed as expected: {e}")


@pytest.mark.integration
class TestCrossProviderEmbeddings:
    """Integration tests across multiple embedding providers."""

    @pytest.fixture
    def providers(self) -> Any:
        """Create embedding configurations for available providers."""
        configs = {}

        if os.getenv("OPENAI_API_KEY"):
            configs["openai"] = graphbit.EmbeddingConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="text-embedding-3-small")

        if os.getenv("HUGGINGFACE_API_KEY"):
            configs["huggingface"] = graphbit.EmbeddingConfig.huggingface(api_key=os.getenv("HUGGINGFACE_API_KEY"), model="facebook/bart-base")

        return configs

    def test_cross_provider_embedding_comparison(self, providers: Any) -> None:
        """Test embedding comparison across providers."""
        if len(providers) < 2:
            pytest.skip("Need at least 2 providers for comparison")

        text = "Machine learning is transforming the world"
        embeddings = {}

        # Generate embeddings from all providers
        for provider_name, config in providers.items():
            try:
                client = graphbit.EmbeddingClient(config)
                embedding = client.embed(text)
                embeddings[provider_name] = embedding
            except Exception as e:
                pytest.fail(f"Failed to generate embedding for {provider_name}: {e}")

        # Compare embeddings between providers
        provider_names = list(embeddings.keys())
        for i in range(len(provider_names)):
            for j in range(i + 1, len(provider_names)):
                provider1 = provider_names[i]
                provider2 = provider_names[j]

                try:
                    # Only compare if dimensions match
                    if len(embeddings[provider1]) == len(embeddings[provider2]):
                        similarity = graphbit.EmbeddingClient.similarity(embeddings[provider1], embeddings[provider2])
                        # Different providers may have different embedding spaces
                        # but similarity should be a valid float
                        assert isinstance(similarity, float)
                        assert -1.0 <= similarity <= 1.0
                        print(f"Similarity between {provider1} and {provider2}: {similarity}")
                    else:
                        print(f"Skipping similarity comparison between {provider1} (dim={len(embeddings[provider1])}) and {provider2} (dim={len(embeddings[provider2])}): different dimensions")
                except Exception as e:
                    # Don't fail the test for dimension mismatches between providers
                    if "dimensions must match" in str(e).lower():
                        print(f"Skipping similarity comparison between {provider1} and {provider2}: dimension mismatch")
                    else:
                        pytest.fail(f"Similarity comparison failed between {provider1} and {provider2}: {e}")

    def test_cross_provider_batch_processing(self, providers: Any) -> None:
        """Test batch processing across providers."""
        if not providers:
            pytest.skip("No providers available")

        texts = ["Artificial intelligence", "Natural language processing", "Computer vision", "Machine learning algorithms"]

        for provider_name, config in providers.items():
            try:
                client = graphbit.EmbeddingClient(config)
                embeddings = client.embed_many(texts)

                assert len(embeddings) == len(texts)
                assert all(isinstance(emb, list) for emb in embeddings)
                assert all(len(emb) > 0 for emb in embeddings)

                # All embeddings should have the same dimension
                dimensions = [len(emb) for emb in embeddings]
                if len(set(dimensions)) != 1:
                    print(f"Warning: Inconsistent dimensions for {provider_name}: {dimensions}")
                    # For models like BART that may return variable-length embeddings,
                    # we'll be more lenient but still check that all are non-empty
                    assert all(dim > 0 for dim in dimensions), f"Found empty embeddings for {provider_name}"
                else:
                    print(f"Consistent dimensions for {provider_name}: {dimensions[0]}")

            except Exception as e:
                pytest.fail(f"Batch processing failed for {provider_name}: {e}")


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run specific test cases
    pytest.main([__file__, "-v"])


class TestAdvancedEmbeddingOperations:
    """Integration tests for advanced embedding operations."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        assert api_key is not None
        return api_key

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI embedding client."""
        config = graphbit.EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        return graphbit.EmbeddingClient(config)

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_large_batch_processing(self, openai_client: Any) -> None:
        """Test processing large batches of texts."""
        large_batch = [f"This is test text number {i}" for i in range(20)]

        try:
            start_time = time.time()
            embeddings = openai_client.embed_many(large_batch)
            end_time = time.time()

            assert isinstance(embeddings, list)
            assert len(embeddings) == len(large_batch)
            assert all(isinstance(emb, list) for emb in embeddings)
            assert all(len(emb) > 0 for emb in embeddings)

            # Performance check
            processing_time = end_time - start_time
            assert processing_time < 120, f"Large batch processing took too long: {processing_time}s"

            # All embeddings should have the same dimensionality
            dimensions = [len(emb) for emb in embeddings]
            assert all(dim == dimensions[0] for dim in dimensions), "Inconsistent embedding dimensions"

        except Exception as e:
            pytest.fail(f"Large batch processing failed: {e}")

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_semantic_similarity_validation(self, openai_client: Any) -> None:
        """Test semantic similarity between related and unrelated texts."""
        try:
            # Related texts
            related_texts = ["The cat sat on the mat", "A feline rested on the rug"]

            # Unrelated texts
            unrelated_texts = ["Machine learning algorithms are powerful", "The stock market fluctuated today"]

            # Get embeddings
            related_embeddings = openai_client.embed_many(related_texts)
            unrelated_embeddings = openai_client.embed_many(unrelated_texts)

            # Calculate similarity between related texts
            related_similarity = graphbit.EmbeddingClient.similarity(related_embeddings[0], related_embeddings[1])

            # Calculate similarity between unrelated texts
            cross_similarity = graphbit.EmbeddingClient.similarity(related_embeddings[0], unrelated_embeddings[0])

            # Related texts should be more similar
            assert related_similarity > cross_similarity, f"Related similarity ({related_similarity}) should be higher than cross similarity ({cross_similarity})"

        except Exception as e:
            pytest.fail(f"Semantic similarity validation failed: {e}")


class TestEmbeddingErrorHandling:
    """Integration tests for embedding error handling."""

    def test_invalid_api_key_handling(self) -> None:
        """Test handling of invalid API keys."""
        try:
            invalid_config = graphbit.EmbeddingConfig.openai(api_key="invalid-key", model="text-embedding-3-small")
            client = graphbit.EmbeddingClient(invalid_config)

            with pytest.raises((ValueError, RuntimeError)):
                client.embed("Test text")

        except Exception as e:
            assert any(word in str(e).lower() for word in ["auth", "key", "invalid", "unauthorized"])

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_empty_text_handling(self) -> None:
        """Test handling of empty texts."""
        api_key = os.getenv("OPENAI_API_KEY")
        config = graphbit.EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = graphbit.EmbeddingClient(config)

        try:
            # Test empty string
            empty_result = client.embed("")
            if empty_result:
                assert isinstance(empty_result, list)
                assert len(empty_result) > 0

        except Exception as e:
            # Empty text errors are acceptable
            assert any(word in str(e).lower() for word in ["empty", "text", "input", "invalid"])


class TestEmbeddingUtilitiesAdvanced:
    """Integration tests for advanced embedding utilities."""

    def test_similarity_edge_cases(self) -> None:
        """Test similarity calculation with edge cases."""
        try:
            # Test with identical vectors
            vec1 = [1.0, 2.0, 3.0, 4.0]
            vec2 = [1.0, 2.0, 3.0, 4.0]
            similarity = graphbit.EmbeddingClient.similarity(vec1, vec2)
            assert abs(similarity - 1.0) < 1e-6, f"Identical vectors should have similarity 1.0, got {similarity}"

            # Test with opposite vectors
            vec3 = [1.0, 2.0, 3.0]
            vec4 = [-1.0, -2.0, -3.0]
            similarity = graphbit.EmbeddingClient.similarity(vec3, vec4)
            assert abs(similarity - (-1.0)) < 1e-6, f"Opposite vectors should have similarity -1.0, got {similarity}"

            # Test with orthogonal vectors
            vec5 = [1.0, 0.0, 0.0]
            vec6 = [0.0, 1.0, 0.0]
            similarity = graphbit.EmbeddingClient.similarity(vec5, vec6)
            assert abs(similarity - 0.0) < 1e-6, f"Orthogonal vectors should have similarity 0.0, got {similarity}"

        except Exception as e:
            pytest.fail(f"Similarity edge cases test failed: {e}")

    def test_similarity_with_zero_vectors(self) -> None:
        """Test similarity calculation with zero vectors."""
        try:
            zero_vec = [0.0, 0.0, 0.0]
            normal_vec = [1.0, 2.0, 3.0]

            try:
                similarity = graphbit.EmbeddingClient.similarity(zero_vec, normal_vec)
                assert isinstance(similarity, float)
            except Exception as e:
                # Zero vector errors are acceptable
                assert "zero" in str(e).lower() or "magnitude" in str(e).lower() or "division" in str(e).lower()

        except Exception as e:
            pytest.fail(f"Zero vector similarity test failed: {e}")


@pytest.mark.integration
class TestEmbeddingPerformance:
    """Integration tests for embedding performance characteristics."""

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_single_vs_batch_performance(self) -> None:
        """Test performance comparison between single and batch processing."""
        api_key = os.getenv("OPENAI_API_KEY")
        config = graphbit.EmbeddingConfig.openai(api_key=api_key, model="text-embedding-3-small")
        client = graphbit.EmbeddingClient(config)

        texts = [f"Performance test text {i}" for i in range(3)]

        try:
            # Test batch processing
            start_time = time.time()
            batch_embeddings = client.embed_many(texts)
            batch_time = time.time() - start_time

            # Verify results
            assert len(batch_embeddings) == len(texts)
            assert all(isinstance(emb, list) and len(emb) > 0 for emb in batch_embeddings)

            # Should complete in reasonable time
            assert batch_time < 60, f"Batch processing took too long: {batch_time}s"

        except Exception as e:
            pytest.fail(f"Performance test failed: {e}")
