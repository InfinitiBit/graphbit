"""Integration tests for GIL release validation in GraphBit embeddings.

This test suite validates that the GIL fixes in python/src/embeddings/client.rs
enable true parallelism for embedding generation operations.
"""

import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import Any, List

import pytest

import graphbit


class TestGILReleaseValidation:
    """Tests to validate that GIL is released during embedding operations."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI embedding client."""
        config = graphbit.EmbeddingConfig.openai(
            api_key=api_key, model="text-embedding-3-small"
        )
        return graphbit.EmbeddingClient(config)

    def test_embed_releases_gil(self, openai_client: Any) -> None:
        """Verify that embed() releases GIL and enables parallel execution.
        
        This test validates the GIL fix in python/src/embeddings/client.rs:47
        by measuring speedup from parallel execution.
        """
        texts = [f"Test text number {i}" for i in range(10)]

        # Sequential execution (baseline)
        start_time = time.time()
        for text in texts:
            openai_client.embed(text)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=5) as executor:
            list(executor.map(openai_client.embed, texts))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - embed():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        # Speedup > 2x indicates true parallelism (GIL released)
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during embed() execution."
        )

    def test_embed_many_releases_gil(self, openai_client: Any) -> None:
        """Verify that embed_many() releases GIL and enables parallel execution.
        
        This test validates the GIL fix in python/src/embeddings/client.rs:75
        by measuring speedup from parallel execution.
        """
        # Create batches of texts
        batches = [
            [f"Batch {i} text {j}" for j in range(5)]
            for i in range(6)
        ]

        # Sequential execution (baseline)
        start_time = time.time()
        for batch in batches:
            openai_client.embed_many(batch)
        sequential_time = time.time() - start_time

        # Parallel execution (should be faster if GIL is released)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=3) as executor:
            list(executor.map(openai_client.embed_many, batches))
        parallel_time = time.time() - start_time

        # Calculate speedup
        speedup = sequential_time / parallel_time

        print(f"\nGIL Release Test - embed_many():")
        print(f"  Sequential: {sequential_time:.2f}s")
        print(f"  Parallel:   {parallel_time:.2f}s")
        print(f"  Speedup:    {speedup:.2f}x")

        # Validate that speedup indicates GIL release
        assert speedup > 2.0, (
            f"Expected speedup > 2x (indicating GIL release), got {speedup:.2f}x. "
            f"This suggests the GIL is still held during embed_many() execution."
        )


class TestEmbedBatchParallel:
    """Tests for the new embed_batch_parallel() method."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI embedding client."""
        config = graphbit.EmbeddingConfig.openai(
            api_key=api_key, model="text-embedding-3-small"
        )
        return graphbit.EmbeddingClient(config)

    def test_embed_batch_parallel_basic(self, openai_client: Any) -> None:
        """Test basic functionality of embed_batch_parallel().
        
        This test validates the new method added in python/src/embeddings/client.rs:86-190
        """
        # Prepare text batches
        texts_batch = [
            ["Batch 1 text 1", "Batch 1 text 2"],
            ["Batch 2 text 1", "Batch 2 text 2"],
            ["Batch 3 text 1", "Batch 3 text 2"],
        ]

        # Call embed_batch_parallel
        result = openai_client.embed_batch_parallel(
            texts_batch,
            max_concurrency=3,
            timeout_ms=60000,
        )

        # Validate result structure
        assert isinstance(result, dict), "Result should be a dictionary"
        assert "embeddings" in result, "Result should contain 'embeddings' key"
        assert "errors" in result, "Result should contain 'errors' key"
        assert "duration_ms" in result, "Result should contain 'duration_ms' key"
        assert "stats" in result, "Result should contain 'stats' key"

        # Validate embeddings
        embeddings = result["embeddings"]
        assert isinstance(embeddings, list), "Embeddings should be a list"
        assert len(embeddings) == len(texts_batch), (
            f"Expected {len(texts_batch)} embedding batches, got {len(embeddings)}"
        )

        # Validate each batch
        for i, batch_embeddings in enumerate(embeddings):
            assert isinstance(batch_embeddings, list), (
                f"Batch {i} embeddings should be a list"
            )
            assert len(batch_embeddings) == len(texts_batch[i]), (
                f"Batch {i}: Expected {len(texts_batch[i])} embeddings, "
                f"got {len(batch_embeddings)}"
            )
            for j, embedding in enumerate(batch_embeddings):
                assert isinstance(embedding, list), (
                    f"Batch {i}, embedding {j} should be a list"
                )
                assert len(embedding) > 0, (
                    f"Batch {i}, embedding {j} should not be empty"
                )
                assert all(isinstance(x, float) for x in embedding), (
                    f"Batch {i}, embedding {j} should contain only floats"
                )

        # Validate stats
        stats = result["stats"]
        assert isinstance(stats, dict), "Stats should be a dictionary"
        assert "successful_requests" in stats
        assert "failed_requests" in stats
        assert "total_embeddings" in stats
        assert stats["successful_requests"] == len(texts_batch)
        assert stats["failed_requests"] == 0

        print(f"\nBatch Parallel Test:")
        print(f"  Processed {stats['total_embeddings']} embeddings")
        print(f"  Duration: {result['duration_ms']}ms")
        print(f"  Avg response time: {stats['avg_response_time_ms']:.2f}ms")

    def test_embed_batch_parallel_concurrency(self, openai_client: Any) -> None:
        """Test that embed_batch_parallel() executes in parallel.
        
        This test validates lock-free parallelism by comparing execution times
        with different concurrency levels.
        """
        # Prepare text batches
        texts_batch = [
            [f"Batch {i} text {j}" for j in range(3)]
            for i in range(6)
        ]

        # Test with low concurrency
        start_time = time.time()
        result_low = openai_client.embed_batch_parallel(
            texts_batch,
            max_concurrency=1,  # Sequential
            timeout_ms=120000,
        )
        time_low_concurrency = time.time() - start_time

        # Test with high concurrency
        start_time = time.time()
        result_high = openai_client.embed_batch_parallel(
            texts_batch,
            max_concurrency=6,  # Parallel
            timeout_ms=120000,
        )
        time_high_concurrency = time.time() - start_time

        # Calculate speedup
        speedup = time_low_concurrency / time_high_concurrency

        print(f"\nConcurrency Test - embed_batch_parallel():")
        print(f"  Low concurrency (1):  {time_low_concurrency:.2f}s")
        print(f"  High concurrency (6): {time_high_concurrency:.2f}s")
        print(f"  Speedup:              {speedup:.2f}x")

        # Validate that high concurrency is faster
        assert speedup > 1.5, (
            f"Expected speedup > 1.5x with higher concurrency, got {speedup:.2f}x. "
            f"This suggests lock-free parallelism is not working correctly."
        )

        # Validate that both results are correct
        assert result_low["stats"]["successful_requests"] == len(texts_batch)
        assert result_high["stats"]["successful_requests"] == len(texts_batch)

    def test_embed_batch_parallel_error_handling(self, openai_client: Any) -> None:
        """Test error handling in embed_batch_parallel()."""
        # Test with empty batch
        with pytest.raises(Exception):
            openai_client.embed_batch_parallel(
                [],  # Empty batch
                max_concurrency=5,
            )

        # Test with valid batch (should succeed)
        texts_batch = [["Valid text 1", "Valid text 2"]]
        result = openai_client.embed_batch_parallel(
            texts_batch,
            max_concurrency=5,
            timeout_ms=60000,
        )
        assert result["stats"]["successful_requests"] == 1
        assert result["stats"]["failed_requests"] == 0


class TestBackwardCompatibility:
    """Tests to ensure backward compatibility after GIL fixes."""

    @pytest.fixture
    def api_key(self) -> str:
        """Get OpenAI API key from environment."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return api_key

    @pytest.fixture
    def openai_client(self, api_key: str) -> Any:
        """Create OpenAI embedding client."""
        config = graphbit.EmbeddingConfig.openai(
            api_key=api_key, model="text-embedding-3-small"
        )
        return graphbit.EmbeddingClient(config)

    def test_backward_compatibility_embed(self, openai_client: Any) -> None:
        """Verify existing code using embed() still works."""
        # This is the exact pattern from existing tests
        embedding = openai_client.embed("Hello world!")
        
        assert isinstance(embedding, list)
        assert len(embedding) > 0
        assert all(isinstance(x, float) for x in embedding)

    def test_backward_compatibility_embed_many(self, openai_client: Any) -> None:
        """Verify existing code using embed_many() still works."""
        # This is the exact pattern from existing tests
        texts = ["Hello", "World", "AI", "Machine Learning"]
        embeddings = openai_client.embed_many(texts)
        
        assert isinstance(embeddings, list)
        assert len(embeddings) == len(texts)
        assert all(isinstance(emb, list) for emb in embeddings)
        assert all(len(emb) > 0 for emb in embeddings)

    def test_empty_input_validation(self, openai_client: Any) -> None:
        """Test that empty inputs are now validated."""
        # Empty string should raise ValueError
        with pytest.raises(ValueError, match="Text input cannot be empty"):
            openai_client.embed("")

        # Empty list should raise ValueError
        with pytest.raises(ValueError, match="Text list cannot be empty"):
            openai_client.embed_many([])


if __name__ == "__main__":
    # Initialize GraphBit
    graphbit.init()
    print(f"GraphBit version: {graphbit.version()}")

    # Run tests
    pytest.main([__file__, "-v", "-s"])

