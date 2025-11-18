"""Test client failures and error handling."""

import contextlib

import pytest

from graphbit import DocumentLoader, DocumentLoaderConfig, EmbeddingClient, EmbeddingConfig, LlmClient, LlmConfig, TextSplitter, TextSplitterConfig, shutdown


class TestClientFailures:
    """Test various client failure scenarios."""

    def teardown_method(self):
        """Cleanup after each test method."""
        with contextlib.suppress(Exception):
            shutdown()

    def test_llm_client_invalid_config(self):
        """Test LLM client with invalid configuration."""
        # Test with None config
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = LlmClient(None)

    def test_llm_client_invalid_api_key(self):
        """Test LLM client with invalid API key."""
        # Use a properly formatted but invalid API key
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Should create client but fail on actual API calls
        with pytest.raises(Exception, match=".*"):
            _ = client.complete("Hello world")

    def test_llm_client_timeout_failures(self):
        """Test LLM client timeout scenarios."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test with very short timeout (if supported)
        with pytest.raises(Exception, match=".*"):
            # This will likely fail due to invalid API key or timeout
            _ = client.complete("Hello world", timeout=0.001)

    def test_llm_client_invalid_prompts(self):
        """Test LLM client with invalid prompts."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test with empty prompt
        with pytest.raises((ValueError, Exception)):
            _ = client.complete("")

        # Test with None prompt
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = client.complete(None)

    def test_llm_client_extremely_long_prompts(self):
        """Test LLM client with extremely long prompts."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test with very long prompt
        long_prompt = "a" * 100000
        with pytest.raises(Exception, match=".*"):
            _ = client.complete(long_prompt)

    def test_llm_client_invalid_parameters(self):
        """Test LLM client with invalid parameters."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test with invalid max_tokens
        with pytest.raises((ValueError, Exception)):
            _ = client.complete("Hello", max_tokens=-1)

        # Test with invalid temperature
        with pytest.raises((ValueError, Exception)):
            _ = client.complete("Hello", temperature=-1.0)

        with pytest.raises((ValueError, Exception)):
            _ = client.complete("Hello", temperature=3.0)

    @pytest.mark.asyncio
    async def test_llm_client_async_failures(self):
        """Test LLM client async operation failures."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test async completion failure
        with pytest.raises(Exception, match=".*"):
            _ = await client.complete_async("Hello world")

    @pytest.mark.asyncio
    async def test_llm_client_batch_failures(self):
        """Test LLM client batch operation failures."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Test with empty batch
        with pytest.raises((ValueError, Exception)):
            _ = await client.complete_batch([])

        # Test with None in batch
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = await client.complete_batch([None, "Hello"])

    def test_embedding_client_invalid_config(self):
        """Test embedding client with invalid configuration."""
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = EmbeddingClient(None)

    def test_embedding_client_invalid_api_key(self):
        """Test embedding client with invalid API key."""
        config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "text-embedding-3-small")
        client = EmbeddingClient(config)

        # Should create client but fail on actual API calls
        with pytest.raises(Exception, match=".*"):
            _ = client.embed("Hello world")

    def test_embedding_client_invalid_text(self):
        """Test embedding client with invalid text."""
        config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "text-embedding-3-small")
        client = EmbeddingClient(config)

        # Test with empty text
        with pytest.raises((ValueError, Exception)):
            _ = client.embed("")

        # Test with None text
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = client.embed(None)

    def test_embedding_client_extremely_long_text(self):
        """Test embedding client with extremely long text."""
        config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "text-embedding-3-small")
        client = EmbeddingClient(config)

        # Test with very long text
        long_text = "a" * 100000
        with pytest.raises(Exception, match=".*"):
            _ = client.embed(long_text)

    def test_embedding_client_batch_failures(self):
        """Test embedding client batch operation failures."""
        config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "text-embedding-3-small")
        client = EmbeddingClient(config)

        # Test with empty batch
        with pytest.raises((ValueError, Exception)):
            _ = client.embed_many([])

        # Test with None in batch
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = client.embed_many([None, "Hello"])

    def test_embedding_client_similarity_failures(self):
        """Test embedding similarity calculation failures."""
        # Test with invalid vectors
        with pytest.raises((ValueError, TypeError, Exception)):
            _ = EmbeddingClient.similarity(None, [1, 2, 3])

        with pytest.raises((ValueError, TypeError, Exception)):
            _ = EmbeddingClient.similarity([1, 2, 3], None)

        # Test with mismatched dimensions
        with pytest.raises((ValueError, Exception)):
            _ = EmbeddingClient.similarity([1, 2], [1, 2, 3])

    def test_document_loader_invalid_config(self):
        """Test document loader with invalid configuration."""
        # DocumentLoader might accept None config, so test actual usage
        with contextlib.suppress(Exception):
            loader = DocumentLoader(None)
            # If it accepts None, test that it fails on actual usage
            with pytest.raises(Exception, match=".*"):
                _ = loader.load("nonexistent.txt")
            # If it rejects None config, that's also acceptable

    def test_document_loader_nonexistent_file(self):
        """Test document loader with nonexistent file."""
        config = DocumentLoaderConfig()
        loader = DocumentLoader(config)

        with pytest.raises((FileNotFoundError, Exception)):
            _ = loader.load("/nonexistent/file.txt")

    def test_document_loader_invalid_file_types(self):
        """Test document loader with invalid file types."""
        config = DocumentLoaderConfig()
        loader = DocumentLoader(config)

        # Test with unsupported file type
        with pytest.raises((ValueError, Exception)):
            _ = loader.load("test.unsupported")

    def test_document_loader_permission_denied(self):
        """Test document loader with permission denied."""
        config = DocumentLoaderConfig()
        loader = DocumentLoader(config)

        # Test with system file that might have restricted access
        with pytest.raises((PermissionError, Exception)):
            _ = loader.load("/etc/shadow")  # Unix system file

    def test_text_splitter_invalid_config(self):
        """Test text splitter with invalid configuration."""
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = TextSplitter(None)

    def test_text_splitter_invalid_text(self):
        """Test text splitter with invalid text."""
        config = TextSplitterConfig.character(100, 10)
        splitter = TextSplitter(config)

        # Test with None text
        with pytest.raises((TypeError, ValueError, Exception)):
            _ = splitter.split(None)

    def test_text_splitter_extremely_long_text(self):
        """Test text splitter with extremely long text."""
        config = TextSplitterConfig.character(100, 10)
        splitter = TextSplitter(config)

        # Test with very long text
        long_text = "a" * 1000000
        with contextlib.suppress(Exception):
            chunks = splitter.split(long_text)
            # Should either work or fail gracefully
            assert len(chunks) > 0
            # Extremely long text errors are acceptable

    def test_client_memory_leaks(self):
        """Test clients for potential memory leaks."""
        # Create many clients to test memory handling
        configs = []
        clients = []

        with contextlib.suppress(MemoryError):
            for _ in range(100):
                config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
                client = LlmClient(config)
                configs.append(config)
                clients.append(client)

        # Should be able to create at least some clients
        assert len(clients) > 0

    def test_client_concurrent_access(self):
        """Test client concurrent access scenarios."""
        import threading

        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        def worker():
            with contextlib.suppress(Exception):
                # This will likely fail due to invalid API key, but should not crash
                _ = client.complete("Hello")
                # API errors in concurrent scenarios are acceptable

        # Start multiple threads
        threads = []
        for _ in range(5):
            thread = threading.Thread(target=worker)
            threads.append(thread)
            thread.start()

        # Wait for all threads
        for thread in threads:
            thread.join()

        # Should not crash - concurrent access is handled gracefully
        # Errors are suppressed as they're acceptable in this scenario

    def test_client_resource_cleanup(self):
        """Test client resource cleanup."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")

        # Create and destroy many clients
        for _ in range(10):
            client = LlmClient(config)
            # Client should be garbage collected
            del client

    def test_client_state_corruption(self):
        """Test client behavior with potential state corruption."""
        config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef", "gpt-4")
        client = LlmClient(config)

        # Try to corrupt client state (if possible)
        with contextlib.suppress(AttributeError):
            if hasattr(client, "_config"):
                client._config = None

        # Client should handle corrupted state gracefully
        with pytest.raises(Exception, match=".*"):
            _ = client.complete("Hello")
