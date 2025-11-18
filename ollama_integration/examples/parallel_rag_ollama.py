"""
ParallelRAG with Ollama Support: Local LLM-powered Document Intelligence

This implementation extends ParallelRAG to support local Ollama models for:
1. LLM completions (using GraphBit's built-in Ollama support)
2. Embeddings (using LangChain's OllamaEmbeddings as a bridge)

Benefits:
- No API costs (100% local)
- Data privacy (no external API calls)
- Works offline
- Fast inference (especially with GPU)

Performance: Processes 100+ documents locally without API costs
"""

import asyncio
import os
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import List, Dict, Any, Optional

from graphbit import (
    DocumentLoader,
    LlmClient,
    LlmConfig,
    RecursiveSplitter,
)

# Import LangChain's Ollama embeddings
try:
    from langchain_community.embeddings import OllamaEmbeddings
except ImportError:
    print("⚠️  Warning: langchain-community not installed. Install with: pip install langchain-community")
    OllamaEmbeddings = None


class ParallelRAGOllama:
    """
    ParallelRAG with Ollama support for local LLM and embeddings.
    
    Key Features:
    - GraphBit's LlmClient with Ollama backend (GIL-releasing)
    - LangChain's OllamaEmbeddings for local embeddings
    - True parallel document loading (10-50x speedup)
    - Async LLM queries (5-20x speedup)
    - 100% local, no API costs
    """

    def __init__(
        self,
        llm_model: str = "llama3:8b",
        embedding_model: str = "nomic-embed-text",
        ollama_base_url: str = "http://localhost:11434",
        max_workers: int = 10,
        chunk_size: int = 500,
        chunk_overlap: int = 50,
    ):
        """
        Initialize ParallelRAG with Ollama models.
        
        Args:
            llm_model: Ollama LLM model name (e.g., "llama3:8b", "mistral:7b")
            embedding_model: Ollama embedding model (e.g., "nomic-embed-text")
            ollama_base_url: Ollama server URL (default: http://localhost:11434)
            max_workers: Number of parallel workers
            chunk_size: Text chunk size in characters
            chunk_overlap: Overlap between chunks
        """
        self.max_workers = max_workers
        self.ollama_base_url = ollama_base_url
        
        # Initialize GraphBit components
        self.loader = DocumentLoader()
        self.splitter = RecursiveSplitter(chunk_size=chunk_size, chunk_overlap=chunk_overlap)
        
        # LLM client with Ollama (GraphBit built-in support)
        llm_config = LlmConfig.ollama(llm_model)
        self.llm_client = LlmClient(llm_config)
        
        # Embedding client with Ollama (via LangChain)
        if OllamaEmbeddings is None:
            raise ImportError(
                "langchain-community is required for Ollama embeddings. "
                "Install with: pip install langchain-community"
            )
        
        self.embeddings = OllamaEmbeddings(
            model=embedding_model,
            base_url=ollama_base_url,
        )
        
        # In-memory vector store
        self.vector_store: Dict[str, Any] = {}
        
        print(f"✅ Initialized ParallelRAG with Ollama:")
        print(f"   LLM: {llm_model}")
        print(f"   Embeddings: {embedding_model}")
        print(f"   Ollama URL: {ollama_base_url}")
        print(f"   Workers: {max_workers}")

    def load_documents_parallel(self, doc_paths: List[str]) -> List[Dict[str, Any]]:
        """
        Load documents in parallel with TRUE parallelism (GIL released).
        
        Expected speedup: 10-50x vs sequential loading
        """
        print(f"Loading {len(doc_paths)} documents in parallel...")
        start_time = time.time()
        
        # Use ThreadPoolExecutor - GIL is released during load_document()
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [
                executor.submit(self._load_single_document, path)
                for path in doc_paths
            ]
            documents = [f.result() for f in futures if f.result() is not None]
        
        duration = time.time() - start_time
        print(f" Loaded {len(documents)} documents in {duration:.2f}s")
        if len(documents) > 0:
            print(f"   Average: {duration/len(documents):.3f}s per document")

        return documents

    def _load_single_document(self, path: str) -> Optional[Dict[str, Any]]:
        """Load a single document (called from thread pool)."""
        try:
            # Determine document type from extension
            ext = Path(path).suffix.lower()
            doc_type_map = {
                ".txt": "txt",
                ".pdf": "pdf",
                ".docx": "docx",
                ".json": "json",
                ".csv": "csv",
            }
            doc_type = doc_type_map.get(ext, "txt")

            # GIL is released during this call
            doc_content = self.loader.load_document(path, doc_type)

            # Convert DocumentContent to dict
            return {
                "source": doc_content.source,
                "content": doc_content.content,
                "document_type": doc_content.document_type,
                "file_size": doc_content.file_size,
            }
        except Exception as e:
            print(f" Failed to load {path}: {e}")
            return None

    def chunk_documents_parallel(self, documents: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """
        Chunk documents in parallel.
        
        Expected speedup: 5-10x vs sequential chunking
        """
        print(f"Chunking {len(documents)} documents in parallel...")
        start_time = time.time()
        
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [
                executor.submit(self._chunk_single_document, doc)
                for doc in documents
            ]
            all_chunks = []
            for future in futures:
                chunks = future.result()
                all_chunks.extend(chunks)
        
        duration = time.time() - start_time
        print(f" Created {len(all_chunks)} chunks in {duration:.2f}s")
        
        return all_chunks

    def _chunk_single_document(self, doc: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Chunk a single document."""
        text = doc["content"]
        chunks = self.splitter.split_text(text)

        # Convert TextChunk objects to dicts
        chunk_dicts = []
        for chunk in chunks:
            chunk_dict = {
                "content": chunk.content,
                "metadata": {
                    **doc.get("metadata", {}),
                    "chunk_index": chunk.chunk_index,
                    "start_index": chunk.start_index,
                    "end_index": chunk.end_index,
                }
            }
            chunk_dicts.append(chunk_dict)

        return chunk_dicts

    def embed_chunks(self, chunks: List[Dict]) -> List[Dict]:
        """
        Generate embeddings for chunks using Ollama.

        Note: LangChain's OllamaEmbeddings doesn't support parallel batch processing
        like GraphBit's embed_batch_parallel(), so this will be slower.
        """
        print(f"Generating embeddings for {len(chunks)} chunks using Ollama...")
        start_time = time.time()

        # Extract texts
        texts = [chunk["content"] for chunk in chunks]

        # Generate embeddings (sequential via LangChain)
        embeddings = self.embeddings.embed_documents(texts)

        # Add embeddings to chunks
        for chunk, embedding in zip(chunks, embeddings):
            chunk["embedding"] = embedding

        duration = time.time() - start_time
        print(f" Generated {len(embeddings)} embeddings in {duration:.2f}s")
        print(f"   Average: {duration/len(embeddings):.3f}s per embedding")

        return chunks

    def store_chunks(self, chunks: List[Dict]) -> None:
        """Store chunks in vector store."""
        print(f"Storing {len(chunks)} chunks in vector store...")

        for i, chunk in enumerate(chunks):
            chunk_id = f"chunk_{i}"
            self.vector_store[chunk_id] = chunk

        print(f" Stored {len(chunks)} chunks")

    def search(self, query: str, top_k: int = 5) -> List[Dict]:
        """
        Search for similar chunks using cosine similarity.
        """
        # Generate query embedding
        query_embedding = self.embeddings.embed_query(query)

        # Calculate similarities
        from graphbit import EmbeddingClient

        similarities = []
        for chunk_id, chunk in self.vector_store.items():
            if "embedding" in chunk:
                similarity = EmbeddingClient.similarity(query_embedding, chunk["embedding"])
                similarities.append((chunk_id, chunk, similarity))

        # Sort by similarity (descending)
        similarities.sort(key=lambda x: x[2], reverse=True)

        # Return top-k results
        return [{"chunk_id": cid, "chunk": chunk, "similarity": sim}
                for cid, chunk, sim in similarities[:top_k]]

    async def query_async(self, query: str, top_k: int = 5) -> str:
        """
        Query the RAG system asynchronously.

        Steps:
        1. Search for relevant chunks
        2. Build context from top-k chunks
        3. Generate response using Ollama LLM
        """
        # Search for relevant chunks
        results = self.search(query, top_k=top_k)

        if not results:
            return "No relevant information found."

        # Build context
        context_parts = []
        for i, result in enumerate(results, 1):
            chunk = result["chunk"]
            similarity = result["similarity"]
            context_parts.append(
                f"[Document {i}] (Relevance: {similarity:.2f})\n{chunk['content']}"
            )

        context = "\n\n".join(context_parts)

        # Build prompt
        prompt = f"""Based on the following context, answer the question.

Context:
{context}

Question: {query}

Answer:"""

        # Generate response using Ollama LLM (async)
        response = await self.llm_client.complete_async(prompt, max_tokens=500)

        return response


async def main():
    """Demonstrate ParallelRAG with Ollama."""

    print("=" * 80)
    print("ParallelRAG with Ollama: Local Document Intelligence")
    print("=" * 80)

    # Check if Ollama is running
    import requests
    try:
        response = requests.get("http://localhost:11434/api/tags", timeout=2)
        if response.status_code != 200:
            print("❌ Ollama is not running. Please start Ollama first.")
            print("   See OLLAMA_SETUP_GUIDE.md for instructions.")
            return
    except Exception as e:
        print(f"❌ Cannot connect to Ollama: {e}")
        print("   Please start Ollama first. See OLLAMA_SETUP_GUIDE.md")
        return

    # Initialize ParallelRAG with Ollama
    # Using gemma3:4b for testing (llama3:8b not available)
    rag = ParallelRAGOllama(
        llm_model="gemma3:4b",
        embedding_model="nomic-embed-text",
        max_workers=10
    )

    # For demo, create sample documents
    import tempfile
    temp_dir = tempfile.mkdtemp()

    sample_docs = [
        "Machine learning is a subset of artificial intelligence that enables systems to learn from data.",
        "Deep learning uses neural networks with multiple layers to process complex patterns.",
        "Natural language processing helps computers understand and generate human language.",
        "Computer vision enables machines to interpret and understand visual information.",
        "Reinforcement learning trains agents through trial and error with rewards.",
    ]

    doc_paths = []
    for i, content in enumerate(sample_docs):
        path = Path(temp_dir) / f"sample_doc_{i}.txt"
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        doc_paths.append(str(path))

    # Step 1: Load documents in parallel
    documents = rag.load_documents_parallel(doc_paths)

    # Step 2: Chunk documents in parallel
    chunks = rag.chunk_documents_parallel(documents)

    # Step 3: Generate embeddings
    chunks_with_embeddings = rag.embed_chunks(chunks)

    # Step 4: Store chunks
    rag.store_chunks(chunks_with_embeddings)

    # Step 5: Query the system
    query = "What is machine learning?"
    print(f"\nQuery: {query}")
    response = await rag.query_async(query)
    print(f"Response: {response}")

    # Cleanup
    import shutil
    shutil.rmtree(temp_dir)

    print("\n" + "=" * 80)
    print(" ParallelRAG with Ollama processing complete!")
    print("=" * 80)


if __name__ == "__main__":
    asyncio.run(main())


