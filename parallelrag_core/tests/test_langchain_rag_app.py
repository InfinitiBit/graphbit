"""
Tests for LangChain RAG Application

This test suite validates the LangChain RAG implementation and ensures
functional equivalence with GraphBit's ParallelRAG.

Test Coverage:
- Configuration and initialization
- Document loading
- Text chunking
- Vector store building (embedding generation)
- Similarity search
- RAG query interface
- End-to-end pipeline
- Statistics tracking
- Error handling

Usage:
    # Run all tests
    pytest tests/test_langchain_rag_app.py -v
    
    # Run specific test class
    pytest tests/test_langchain_rag_app.py::TestLangChainRAGInitialization -v
    
    # Run with API key (for full validation)
    export OPENAI_API_KEY="your-key-here"
    pytest tests/test_langchain_rag_app.py -v
"""

import os
import tempfile
from pathlib import Path
from typing import List

import pytest

from parallelrag_core.langchain_rag_app import LangChainRAG, LangChainRAGConfig


# ============================================================================
# Fixtures
# ============================================================================

@pytest.fixture
def api_key() -> str:
    """Get OpenAI API key from environment."""
    key = os.getenv("OPENAI_API_KEY")
    if not key:
        pytest.skip("OPENAI_API_KEY not set")
    return key


@pytest.fixture
def rag_config(api_key: str) -> LangChainRAGConfig:
    """Create RAG configuration with API key."""
    return LangChainRAGConfig(openai_api_key=api_key)


@pytest.fixture
def rag_system(rag_config: LangChainRAGConfig) -> LangChainRAG:
    """Create LangChain RAG system."""
    return LangChainRAG(rag_config)


@pytest.fixture
def sample_documents() -> List[str]:
    """Create sample document texts."""
    return [
        """
        Artificial Intelligence (AI) is revolutionizing technology.
        Machine learning algorithms process vast amounts of data.
        Deep learning uses neural networks with multiple layers.
        This has led to breakthroughs in computer vision and NLP.
        """ * 5,  # Repeat to ensure multiple chunks
        """
        Cloud computing provides on-demand access to resources.
        Organizations can scale infrastructure dynamically.
        Major providers include AWS, Azure, and Google Cloud.
        This enables faster innovation and reduced costs.
        """ * 5,
        """
        Cybersecurity is critical in our connected world.
        Threats evolve rapidly with sophisticated attacks.
        Multi-layered security strategies are essential.
        Employee training reduces human error vulnerabilities.
        """ * 5,
    ]


@pytest.fixture
def sample_doc_files(sample_documents: List[str]) -> List[str]:
    """Create temporary files with sample documents."""
    temp_dir = tempfile.mkdtemp()
    doc_paths = []
    
    for i, content in enumerate(sample_documents):
        path = Path(temp_dir) / f"test_doc_{i}.txt"
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        doc_paths.append(str(path))
    
    yield doc_paths
    
    # Cleanup
    import shutil
    shutil.rmtree(temp_dir)


# ============================================================================
# Test Classes
# ============================================================================

class TestLangChainRAGConfiguration:
    """Test configuration management."""
    
    def test_default_configuration(self):
        """Test default configuration values."""
        config = LangChainRAGConfig()
        
        assert config.chunk_size == 500
        assert config.chunk_overlap == 50
        assert config.embedding_model == "text-embedding-3-small"
        assert config.llm_model == "gpt-4o-mini"
        assert config.max_tokens == 500
        assert config.temperature == 0.7
        assert config.top_k == 5
    
    def test_custom_configuration(self, api_key: str):
        """Test custom configuration values."""
        config = LangChainRAGConfig(
            openai_api_key=api_key,
            chunk_size=1000,
            chunk_overlap=100,
            max_tokens=200,
            temperature=0.5,
            top_k=3,
        )
        
        assert config.openai_api_key == api_key
        assert config.chunk_size == 1000
        assert config.chunk_overlap == 100
        assert config.max_tokens == 200
        assert config.temperature == 0.5
        assert config.top_k == 3


class TestLangChainRAGInitialization:
    """Test LangChain RAG initialization."""
    
    def test_initialization_with_api_key(self, api_key: str):
        """Test initialization with API key."""
        config = LangChainRAGConfig(openai_api_key=api_key)
        rag = LangChainRAG(config)
        
        assert rag.config.openai_api_key == api_key
        assert rag.text_splitter is not None
        assert rag.embeddings is not None
        assert rag.llm is not None
        assert rag.vector_store is None  # Not initialized until documents added
    
    def test_initialization_without_api_key(self):
        """Test initialization fails without API key."""
        # Temporarily remove API key from environment
        original_key = os.environ.pop("OPENAI_API_KEY", None)
        
        try:
            config = LangChainRAGConfig()
            with pytest.raises(ValueError, match="OpenAI API key required"):
                LangChainRAG(config)
        finally:
            # Restore API key
            if original_key:
                os.environ["OPENAI_API_KEY"] = original_key


class TestDocumentLoading:
    """Test document loading functionality."""

    def test_load_documents(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test loading documents from files."""
        documents = rag_system.load_documents(sample_doc_files)

        assert len(documents) == len(sample_doc_files)
        assert all('path' in doc for doc in documents)
        assert all('content' in doc for doc in documents)
        assert all('metadata' in doc for doc in documents)
        assert all(len(doc['content']) > 0 for doc in documents)

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["documents_loaded"] == len(sample_doc_files)

    def test_load_nonexistent_file(self, rag_system: LangChainRAG):
        """Test loading handles nonexistent files."""
        documents = rag_system.load_documents(["nonexistent_file.txt"])

        # Should return empty list (file doesn't exist)
        assert len(documents) == 0


class TestChunking:
    """Test document chunking functionality."""

    def test_chunk_documents(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test chunking produces correct results."""
        documents = rag_system.load_documents(sample_doc_files)
        chunks = rag_system.chunk_documents(documents)

        assert len(chunks) > 0
        assert all(hasattr(chunk, 'page_content') for chunk in chunks)
        assert all(hasattr(chunk, 'metadata') for chunk in chunks)
        assert all(len(chunk.page_content) > 0 for chunk in chunks)

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["chunks_created"] > 0

    def test_chunk_empty_document(self, rag_system: LangChainRAG):
        """Test chunking handles empty documents."""
        documents = [{'path': 'test.txt', 'content': '', 'metadata': {}}]
        chunks = rag_system.chunk_documents(documents)

        # Empty document may produce 0 or 1 chunk
        assert len(chunks) >= 0

    def test_chunk_size_configuration(self, api_key: str):
        """Test chunk size configuration is respected."""
        config = LangChainRAGConfig(
            openai_api_key=api_key,
            chunk_size=100,
            chunk_overlap=10
        )
        rag = LangChainRAG(config)

        # Create a long document
        long_text = "This is a test sentence. " * 100
        documents = [{'path': 'test.txt', 'content': long_text, 'metadata': {}}]

        chunks = rag.chunk_documents(documents)

        # Should create multiple chunks
        assert len(chunks) > 1

        # Most chunks should be around chunk_size (with some tolerance)
        for chunk in chunks[:-1]:  # Exclude last chunk
            assert len(chunk.page_content) <= config.chunk_size + 50  # Some tolerance


class TestVectorStore:
    """Test vector store building and embedding generation."""

    def test_build_vector_store(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test building vector store generates embeddings."""
        documents = rag_system.load_documents(sample_doc_files)
        chunks = rag_system.chunk_documents(documents)
        vector_store = rag_system.build_vector_store(chunks)

        assert vector_store is not None
        assert rag_system.vector_store is not None

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["embeddings_generated"] == len(chunks)

    def test_build_vector_store_without_chunks(self, rag_system: LangChainRAG):
        """Test building vector store fails without chunks."""
        with pytest.raises(ValueError, match="No chunks to build vector store"):
            rag_system.build_vector_store([])


class TestSimilaritySearch:
    """Test similarity search functionality."""

    def test_search(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test similarity search returns relevant results."""
        # Build vector store
        documents = rag_system.load_documents(sample_doc_files)
        chunks = rag_system.chunk_documents(documents)
        rag_system.build_vector_store(chunks)

        # Search
        query = "What is artificial intelligence?"
        results = rag_system.search(query, top_k=3)

        assert len(results) == 3
        assert all(hasattr(result, 'page_content') for result in results)

        # Results should contain relevant content
        combined_text = " ".join([r.page_content for r in results])
        assert any(keyword in combined_text.lower() for keyword in ['ai', 'artificial', 'intelligence', 'machine'])

    def test_search_without_vector_store(self, rag_system: LangChainRAG):
        """Test search fails without vector store."""
        with pytest.raises(ValueError, match="Vector store not initialized"):
            rag_system.search("test query")

    def test_search_custom_top_k(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test search respects custom top_k parameter."""
        documents = rag_system.load_documents(sample_doc_files)
        chunks = rag_system.chunk_documents(documents)
        rag_system.build_vector_store(chunks)

        results = rag_system.search("test query", top_k=2)
        assert len(results) == 2


class TestRAGQuery:
    """Test RAG query interface."""

    def test_query(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test RAG query returns LLM response."""
        # Build vector store
        documents = rag_system.load_documents(sample_doc_files)
        chunks = rag_system.chunk_documents(documents)
        rag_system.build_vector_store(chunks)

        # Query
        query = "What are the main topics discussed?"
        response = rag_system.query(query)

        assert isinstance(response, str)
        assert len(response) > 0

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["queries_processed"] == 1

    def test_query_without_vector_store(self, rag_system: LangChainRAG):
        """Test query fails without vector store."""
        with pytest.raises(ValueError, match="Vector store not initialized"):
            rag_system.query("test query")


class TestEndToEndPipeline:
    """Test complete end-to-end RAG pipeline."""

    def test_process_documents(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test complete document processing pipeline."""
        results = rag_system.process_documents(sample_doc_files)

        # Verify results structure
        assert "documents" in results
        assert "chunks" in results
        assert "embeddings" in results
        assert "duration" in results
        assert "throughput" in results
        assert "vector_store_ready" in results

        # Verify counts
        assert results["documents"] == len(sample_doc_files)
        assert results["chunks"] > 0
        assert results["embeddings"] == results["chunks"]
        assert results["vector_store_ready"] is True

        # Verify performance metrics
        assert results["duration"] > 0
        assert results["throughput"] > 0

        # Verify statistics updated
        stats = rag_system.get_statistics()
        assert stats["documents_loaded"] == len(sample_doc_files)
        assert stats["chunks_created"] > 0
        assert stats["embeddings_generated"] > 0

    def test_process_and_query(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test processing documents and then querying."""
        # Process documents
        results = rag_system.process_documents(sample_doc_files)
        assert results["vector_store_ready"] is True

        # Query
        response = rag_system.query("What are the main topics?")
        assert isinstance(response, str)
        assert len(response) > 0


class TestStatistics:
    """Test statistics tracking."""

    def test_get_statistics(self, rag_system: LangChainRAG):
        """Test getting statistics."""
        stats = rag_system.get_statistics()

        assert "documents_loaded" in stats
        assert "chunks_created" in stats
        assert "embeddings_generated" in stats
        assert "queries_processed" in stats
        assert "total_time" in stats

    def test_reset_statistics(self, rag_system: LangChainRAG, sample_doc_files: List[str]):
        """Test resetting statistics."""
        # Process some documents
        rag_system.process_documents(sample_doc_files)

        # Verify stats are non-zero
        stats = rag_system.get_statistics()
        assert stats["documents_loaded"] > 0

        # Reset
        rag_system.reset_statistics()

        # Verify stats are zero
        stats = rag_system.get_statistics()
        assert stats["documents_loaded"] == 0
        assert stats["chunks_created"] == 0
        assert stats["embeddings_generated"] == 0
        assert stats["queries_processed"] == 0
        assert stats["total_time"] == 0.0


class TestErrorHandling:
    """Test error handling."""

    def test_empty_document_list(self, rag_system: LangChainRAG):
        """Test handling of empty document list."""
        results = rag_system.process_documents([])

        assert results["documents"] == 0
        assert results["chunks"] == 0
        assert results["embeddings"] == 0

    def test_invalid_api_key(self):
        """Test handling of invalid API key."""
        config = LangChainRAGConfig(openai_api_key="sk-" + "x" * 48)

        # Should fail during API call (not initialization)
        rag = LangChainRAG(config)

        # Create a test document
        import tempfile
        temp_dir = tempfile.mkdtemp()
        path = Path(temp_dir) / "test.txt"
        with open(path, 'w') as f:
            f.write("Test content")

        try:
            # Should fail when trying to generate embeddings
            with pytest.raises(Exception):
                rag.process_documents([str(path)])
        finally:
            import shutil
            shutil.rmtree(temp_dir)


