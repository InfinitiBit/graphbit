"""
LangChain RAG with Ollama Support - Local LLM-powered RAG

This implementation provides LangChain RAG functionality using local Ollama models:
- Ollama embeddings (nomic-embed-text, mxbai-embed-large)
- Ollama LLMs (llama3, mistral, phi3)
- FAISS vector storage (in-memory)
- 100% local, no API costs

Features:
- Document loading and text chunking
- Local embedding generation with Ollama
- Vector storage with FAISS
- Similarity search and retrieval
- RAG query interface with local LLM
- Statistics tracking

Performance: Uses LangChain's sequential processing (no parallelism)
"""

import os
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Dict, Any, Optional

from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_community.vectorstores import FAISS
from langchain_community.embeddings import OllamaEmbeddings
from langchain_community.chat_models import ChatOllama
from langchain_core.documents import Document


@dataclass
class LangChainRAGOllamaConfig:
    """Configuration for LangChain RAG with Ollama."""
    
    # Text splitting configuration
    chunk_size: int = 500
    chunk_overlap: int = 50
    
    # Ollama configuration
    ollama_base_url: str = "http://localhost:11434"
    embedding_model: str = "nomic-embed-text"
    llm_model: str = "llama3:8b"
    
    # LLM parameters
    max_tokens: int = 500
    temperature: float = 0.7
    
    # Retrieval configuration
    top_k: int = 5
    
    # Statistics
    stats: Dict[str, Any] = field(default_factory=lambda: {
        "documents_loaded": 0,
        "chunks_created": 0,
        "embeddings_generated": 0,
        "queries_processed": 0,
        "total_time": 0.0
    })


class LangChainRAGOllama:
    """
    LangChain-based RAG system with Ollama support.
    
    This implementation uses:
    - RecursiveCharacterTextSplitter for text chunking
    - OllamaEmbeddings for local embedding generation
    - FAISS for vector storage and similarity search
    - ChatOllama for local LLM completions
    - 100% local, no API costs
    """
    
    def __init__(self, config: Optional[LangChainRAGOllamaConfig] = None):
        """
        Initialize LangChain RAG with Ollama.
        
        Args:
            config: RAG configuration. If None, uses default configuration.
        """
        self.config = config or LangChainRAGOllamaConfig()
        
        # Initialize text splitter
        self.text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=self.config.chunk_size,
            chunk_overlap=self.config.chunk_overlap,
            length_function=len,
        )
        
        # Initialize Ollama embeddings
        self.embeddings = OllamaEmbeddings(
            model=self.config.embedding_model,
            base_url=self.config.ollama_base_url,
        )
        
        # Initialize Ollama LLM
        self.llm = ChatOllama(
            model=self.config.llm_model,
            base_url=self.config.ollama_base_url,
            temperature=self.config.temperature,
        )
        
        # Vector store (initialized when documents are processed)
        self.vector_store: Optional[FAISS] = None
        
        print(f"✅ Initialized LangChain RAG with Ollama:")
        print(f"   LLM: {self.config.llm_model}")
        print(f"   Embeddings: {self.config.embedding_model}")
        print(f"   Ollama URL: {self.config.ollama_base_url}")

    def _load_documents(self, doc_paths: List[str]) -> List[Document]:
        """
        Load documents from file paths.
        
        Note: Sequential loading (no parallelism in LangChain)
        """
        documents = []
        
        for path in doc_paths:
            try:
                with open(path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                doc = Document(
                    page_content=content,
                    metadata={"source": path, "filename": Path(path).name}
                )
                documents.append(doc)
            except Exception as e:
                print(f"⚠️  Failed to load {path}: {e}")
        
        return documents

    def process_documents(self, doc_paths: List[str]) -> Dict[str, Any]:
        """
        Process documents: load, chunk, embed, and store.
        
        Args:
            doc_paths: List of document file paths
            
        Returns:
            Dictionary with processing statistics
        """
        start_time = time.time()
        
        print(f"Processing {len(doc_paths)} documents...")
        
        # Load documents
        load_start = time.time()
        documents = self._load_documents(doc_paths)
        load_time = time.time() - load_start
        print(f" Loaded {len(documents)} documents in {load_time:.2f}s")
        
        # Split into chunks
        chunk_start = time.time()
        chunks = self.text_splitter.split_documents(documents)
        chunk_time = time.time() - chunk_start
        print(f" Created {len(chunks)} chunks")
        
        # Create vector store with embeddings
        embed_start = time.time()
        self.vector_store = FAISS.from_documents(chunks, self.embeddings)
        embed_time = time.time() - embed_start
        print(f" Generated {len(chunks)} embeddings in {embed_time:.2f}s")
        
        print(f" Stored {len(chunks)} chunks in FAISS vector store")
        
        # Update statistics
        duration = time.time() - start_time
        self.config.stats["documents_loaded"] += len(documents)
        self.config.stats["chunks_created"] += len(chunks)
        self.config.stats["embeddings_generated"] += len(chunks)
        self.config.stats["total_time"] += duration
        
        return {
            "duration": duration,
            "load_time": load_time,
            "chunk_time": chunk_time,
            "embed_time": embed_time,
            "documents": len(documents),
            "chunks": len(chunks),
        }

    def query(self, query: str, top_k: Optional[int] = None) -> str:
        """
        Query the RAG system.

        Args:
            query: User query
            top_k: Number of chunks to retrieve (default: config.top_k)

        Returns:
            LLM-generated response
        """
        if self.vector_store is None:
            raise ValueError("No documents processed. Call process_documents() first.")

        start_time = time.time()

        # Retrieve relevant chunks
        k = top_k or self.config.top_k
        docs = self.vector_store.similarity_search(query, k=k)

        # Build context from retrieved documents
        context = "\n\n".join([doc.page_content for doc in docs])

        # Build prompt
        prompt = f"""Based on the following context, answer the question.

Context:
{context}

Question: {query}

Answer:"""

        # Generate response using Ollama LLM
        response = self.llm.invoke(prompt)

        # Update statistics
        duration = time.time() - start_time
        self.config.stats["queries_processed"] += 1

        return response.content

    def get_statistics(self) -> Dict[str, Any]:
        """Get cumulative statistics."""
        return self.config.stats.copy()


def main():
    """Demonstrate LangChain RAG with Ollama."""

    print("=" * 80)
    print("LangChain RAG with Ollama: Local Document Intelligence")
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

    # Sample documents
    sample_docs = [
        """
        Machine learning is a subset of artificial intelligence that enables systems to learn
        and improve from experience without being explicitly programmed. It focuses on developing
        computer programs that can access data and use it to learn for themselves.
        """,
        """
        Deep learning is a subset of machine learning that uses neural networks with multiple
        layers (hence "deep") to progressively extract higher-level features from raw input.
        For example, in image processing, lower layers may identify edges, while higher layers
        may identify concepts relevant to a human such as digits or letters or faces.
        """,
        """
        Natural language processing (NLP) is a branch of artificial intelligence that helps
        computers understand, interpret and manipulate human language. NLP draws from many
        disciplines, including computer science and computational linguistics, in its pursuit
        to fill the gap between human communication and computer understanding.
        """,
        """
        Computer vision is an interdisciplinary field that deals with how computers can gain
        high-level understanding from digital images or videos. From the perspective of
        engineering, it seeks to automate tasks that the human visual system can do.
        """,
        """
        Reinforcement learning is an area of machine learning concerned with how intelligent
        agents ought to take actions in an environment in order to maximize the notion of
        cumulative reward. Reinforcement learning is one of three basic machine learning
        paradigms, alongside supervised learning and unsupervised learning.
        """
    ]

    # Save sample documents to files
    import tempfile
    temp_dir = tempfile.mkdtemp()
    doc_paths = []

    for i, content in enumerate(sample_docs):
        path = Path(temp_dir) / f"sample_doc_{i}.txt"
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        doc_paths.append(str(path))

    # Create RAG system
    rag = LangChainRAGOllama()

    # Process documents
    results = rag.process_documents(doc_paths)

    # Query the system
    query = "What is machine learning?"
    print(f"\nQuery: {query}")
    response = rag.query(query)
    print(f"Response: {response}")

    # Display statistics
    print("\n\nCumulative Statistics:")
    print("="*80)
    stats = rag.get_statistics()
    for key, value in stats.items():
        print(f"  {key}: {value}")

    # Cleanup
    import shutil
    shutil.rmtree(temp_dir)

    print("\n" + "=" * 80)
    print(" LangChain RAG with Ollama processing complete!")
    print("=" * 80)


if __name__ == "__main__":
    main()


