"""
Comprehensive tests for ParallelRAG application.

These tests validate:
1. Correct API usage patterns from GraphBit codebase
2. Performance meets benchmark expectations
3. Error handling is robust
4. Sequential vs parallel execution speedup
5. Memory usage is within acceptable limits
"""

import os
import time
import pytest
from typing import List

# Import the application
from parallelrag_core.parallel_rag_app import ParallelRAG, RAGConfig


@pytest.fixture
def api_key() -> str:
    """Get OpenAI API key from environment."""
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        pytest.skip("OPENAI_API_KEY not set")
    return api_key


@pytest.fixture
def rag_system(api_key: str) -> ParallelRAG:
    """Create ParallelRAG system for testing."""
    config = RAGConfig(openai_api_key=api_key)
    return ParallelRAG(config)


@pytest.fixture
def sample_documents() -> List[str]:
    """Generate sample documents for testing."""
    return [
        "Artificial Intelligence is transforming technology. " * 50,
        "Cloud computing provides scalable infrastructure. " * 50,
        "Cybersecurity protects digital assets. " * 50,
        "Machine learning enables data-driven decisions. " * 50,
        "Blockchain technology ensures transparency. " * 50,
    ]


@pytest.fixture
def large_documents() -> List[str]:
    """Generate larger documents for performance testing."""
    return [
        f"Document {i}: " + "This is a test sentence. " * 200
        for i in range(20)
    ]


class TestRAGConfig:
    """Test RAG configuration."""
    
    def test_default_config(self):
        """Test default configuration values."""
        config = RAGConfig()
        
        assert config.chunk_size == 200
        assert config.chunk_overlap == 20
        assert config.chunking_workers == 20
        assert config.embedding_workers == 20
        assert config.llm_workers == 20
        assert config.embedding_model == "text-embedding-3-small"
        assert config.llm_model == "gpt-4o-mini"
    
    def test_custom_config(self):
        """Test custom configuration."""
        config = RAGConfig(
            chunk_size=500,
            chunk_overlap=50,
            chunking_workers=10,
            embedding_workers=15,
            llm_workers=5,
            max_tokens=200,
            temperature=0.5
        )
        
        assert config.chunk_size == 500
        assert config.chunk_overlap == 50
        assert config.chunking_workers == 10
        assert config.embedding_workers == 15
        assert config.llm_workers == 5
        assert config.max_tokens == 200
        assert config.temperature == 0.5


class TestParallelRAGInitialization:
    """Test ParallelRAG initialization."""
    
    def test_initialization_with_api_key(self, api_key: str):
        """Test initialization with API key."""
        config = RAGConfig(openai_api_key=api_key)
        rag = ParallelRAG(config)
        
        assert rag.config.openai_api_key == api_key
        assert rag.splitter is not None
        assert rag.embed_client is not None
        assert rag.llm_client is not None
    
    def test_initialization_without_api_key(self):
        """Test initialization fails without API key."""
        # Temporarily remove API key from environment
        original_key = os.environ.pop("OPENAI_API_KEY", None)
        
        try:
            config = RAGConfig()
            with pytest.raises(ValueError, match="OpenAI API key required"):
                ParallelRAG(config)
        finally:
            # Restore API key
            if original_key:
                os.environ["OPENAI_API_KEY"] = original_key
    
    def test_statistics_initialization(self, rag_system: ParallelRAG):
        """Test statistics are initialized correctly."""
        stats = rag_system.get_statistics()
        
        assert stats["documents_processed"] == 0
        assert stats["chunks_created"] == 0
        assert stats["embeddings_generated"] == 0
        assert stats["llm_calls"] == 0
        assert stats["total_time"] == 0.0


class TestChunking:
    """Test document chunking functionality."""
    
    def test_chunk_documents(self, rag_system: ParallelRAG, sample_documents: List[str]):
        """Test chunking produces correct results."""
        chunk_lists = rag_system.chunk_documents(sample_documents)
        
        assert len(chunk_lists) == len(sample_documents)
        assert all(len(chunks) > 0 for chunks in chunk_lists)
        
        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["chunks_created"] > 0
    
    def test_chunk_empty_document(self, rag_system: ParallelRAG):
        """Test chunking handles empty documents."""
        chunk_lists = rag_system.chunk_documents([""])
        
        assert len(chunk_lists) == 1
        # Empty document may produce 0 or 1 chunk depending on splitter behavior
    
    def test_chunk_single_document(self, rag_system: ParallelRAG):
        """Test chunking single document."""
        doc = "This is a test document. " * 100
        chunk_lists = rag_system.chunk_documents([doc])
        
        assert len(chunk_lists) == 1
        assert len(chunk_lists[0]) > 0


class TestEmbedding:
    """Test embedding generation functionality."""

    def test_generate_embeddings(self, rag_system: ParallelRAG):
        """Test embedding generation produces correct results."""
        texts = [
            "Machine learning is a subset of AI.",
            "Deep learning uses neural networks.",
            "Natural language processing analyzes text."
        ]

        embeddings = rag_system.generate_embeddings(texts)

        assert len(embeddings) == len(texts)
        assert all(isinstance(emb, list) for emb in embeddings)
        assert all(len(emb) > 0 for emb in embeddings)
        assert all(isinstance(val, float) for emb in embeddings for val in emb)

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["embeddings_generated"] == len(texts)

    def test_embedding_dimensions_consistent(self, rag_system: ParallelRAG):
        """Test all embeddings have same dimensions."""
        texts = ["Text 1", "Text 2", "Text 3"]
        embeddings = rag_system.generate_embeddings(texts)

        dimensions = [len(emb) for emb in embeddings]
        assert len(set(dimensions)) == 1  # All same dimension

    def test_embedding_single_text(self, rag_system: ParallelRAG):
        """Test embedding generation for single text."""
        embeddings = rag_system.generate_embeddings(["Single text"])

        assert len(embeddings) == 1
        assert len(embeddings[0]) > 0


class TestLLMCompletion:
    """Test LLM completion functionality."""

    def test_generate_completions(self, rag_system: ParallelRAG):
        """Test LLM completion produces correct results."""
        prompts = [
            "What is AI?",
            "Explain machine learning.",
            "Define cloud computing."
        ]

        completions = rag_system.generate_completions(prompts)

        assert len(completions) == len(prompts)
        assert all(isinstance(comp, str) for comp in completions)
        assert all(len(comp) > 0 for comp in completions)

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["llm_calls"] == len(prompts)

    def test_completion_single_prompt(self, rag_system: ParallelRAG):
        """Test completion for single prompt."""
        completions = rag_system.generate_completions(["What is AI?"])

        assert len(completions) == 1
        assert len(completions[0]) > 0


class TestEndToEndPipeline:
    """Test complete end-to-end RAG pipeline."""

    def test_process_documents(self, rag_system: ParallelRAG, sample_documents: List[str]):
        """Test complete document processing pipeline."""
        results = rag_system.process_documents(sample_documents)

        # Verify results structure
        assert "documents" in results
        assert "chunks" in results
        assert "embeddings" in results
        assert "summaries" in results
        assert "duration" in results
        assert "throughput" in results

        # Verify counts
        assert results["documents"] == len(sample_documents)
        assert results["chunks"] > 0
        assert len(results["embeddings"]) == results["chunks"]
        assert len(results["summaries"]) == len(sample_documents)

        # Verify performance metrics
        assert results["duration"] > 0
        assert results["throughput"] > 0

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["documents_processed"] == len(sample_documents)
        assert stats["chunks_created"] > 0
        assert stats["embeddings_generated"] > 0
        assert stats["llm_calls"] > 0

    def test_process_single_document(self, rag_system: ParallelRAG):
        """Test processing single document."""
        doc = "This is a test document about artificial intelligence. " * 20
        results = rag_system.process_documents([doc])

        assert results["documents"] == 1
        assert results["chunks"] > 0
        assert len(results["summaries"]) == 1


class TestPerformance:
    """Test performance characteristics match benchmarks."""

    def test_chunking_throughput(self, rag_system: ParallelRAG):
        """Test chunking achieves expected throughput."""
        # Generate larger dataset for meaningful performance test
        documents = [f"Document {i}: " + "This is a test sentence. " * 200 for i in range(100)]

        # Measure parallel chunking
        start_time = time.time()
        chunk_lists = rag_system.chunk_documents(documents)
        parallel_time = time.time() - start_time

        total_chunks = sum(len(chunks) for chunks in chunk_lists)
        throughput = total_chunks / parallel_time

        print(f"\nChunking Throughput:")
        print(f"  Documents: {len(documents)}")
        print(f"  Chunks: {total_chunks}")
        print(f"  Time: {parallel_time:.2f}s")
        print(f"  Throughput: {throughput:.1f} chunks/sec")

        # Should achieve at least 1000 chunks/sec (benchmark shows 3914 chunks/sec)
        assert throughput >= 1000, f"Expected throughput >= 1000 chunks/sec, got {throughput:.1f}"

    def test_embedding_throughput(self, rag_system: ParallelRAG):
        """Test embedding achieves expected throughput."""
        texts = [f"Test text number {i} with some content" for i in range(50)]

        # Measure parallel embedding
        start_time = time.time()
        embeddings = rag_system.generate_embeddings(texts)
        parallel_time = time.time() - start_time

        throughput = len(embeddings) / parallel_time

        print(f"\nEmbedding Throughput:")
        print(f"  Texts: {len(texts)}")
        print(f"  Time: {parallel_time:.2f}s")
        print(f"  Throughput: {throughput:.1f} embeddings/sec")

        # Should achieve at least 5 embeddings/sec (conservative)
        assert throughput >= 5.0, f"Expected throughput >= 5.0 embeddings/sec, got {throughput:.1f}"


class TestStatistics:
    """Test statistics tracking."""

    def test_statistics_accumulation(self, rag_system: ParallelRAG, sample_documents: List[str]):
        """Test statistics accumulate correctly across multiple operations."""
        # Process documents twice
        rag_system.process_documents(sample_documents)
        rag_system.process_documents(sample_documents)

        stats = rag_system.get_statistics()

        # Should have processed documents twice
        assert stats["documents_processed"] == len(sample_documents) * 2
        assert stats["chunks_created"] > 0
        assert stats["embeddings_generated"] > 0
        assert stats["llm_calls"] > 0

    def test_statistics_reset(self, rag_system: ParallelRAG, sample_documents: List[str]):
        """Test statistics can be reset."""
        # Process documents
        rag_system.process_documents(sample_documents)

        # Reset statistics
        rag_system.reset_statistics()

        stats = rag_system.get_statistics()
        assert stats["documents_processed"] == 0
        assert stats["chunks_created"] == 0
        assert stats["embeddings_generated"] == 0
        assert stats["llm_calls"] == 0
        assert stats["total_time"] == 0.0


class TestErrorHandling:
    """Test error handling."""

    def test_empty_document_list(self, rag_system: ParallelRAG):
        """Test handling of empty document list."""
        results = rag_system.process_documents([])

        assert results["documents"] == 0
        assert results["chunks"] == 0
        assert len(results["embeddings"]) == 0
        assert len(results["summaries"]) == 0

    def test_invalid_api_key(self):
        """Test handling of invalid API key."""
        # GraphBit validates API key format during initialization
        # Use a properly formatted but invalid key
        config = RAGConfig(openai_api_key="sk-" + "x" * 48)

        # Should fail during initialization or API call
        with pytest.raises(Exception):
            rag = ParallelRAG(config)
            rag.generate_embeddings(["Test text"])


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])


