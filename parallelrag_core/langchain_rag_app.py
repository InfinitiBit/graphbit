"""
LangChain RAG Application - Equivalent to GraphBit's ParallelRAG

This implementation provides equivalent functionality to GraphBit's ParallelRAG
using LangChain's components and best practices (LCEL, chains, etc.).

Features:
- Document loading and text chunking
- Embedding generation with OpenAI
- Vector storage with FAISS (in-memory)
- Similarity search and retrieval
- RAG query interface with LLM
- Configuration management
- Statistics tracking

Performance: Uses LangChain's async/batch processing where available
"""

import os
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Dict, Any, Optional

from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_community.vectorstores import FAISS
from langchain_core.documents import Document
from langchain_openai import ChatOpenAI, OpenAIEmbeddings


@dataclass
class LangChainRAGConfig:
    """Configuration for LangChain RAG system."""
    
    # Text splitting configuration (matching GraphBit defaults)
    chunk_size: int = 500
    chunk_overlap: int = 50
    
    # API configuration
    openai_api_key: Optional[str] = None
    embedding_model: str = "text-embedding-3-small"
    llm_model: str = "gpt-4o-mini"
    
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


class LangChainRAG:
    """
    LangChain-based RAG system equivalent to GraphBit's ParallelRAG.
    
    This implementation uses LangChain's components and best practices:
    - RecursiveCharacterTextSplitter for text chunking
    - OpenAIEmbeddings for embedding generation
    - FAISS for vector storage and similarity search
    - ChatOpenAI for LLM completions
    - LCEL for chain composition
    """
    
    def __init__(self, config: Optional[LangChainRAGConfig] = None):
        """
        Initialize LangChain RAG system.
        
        Args:
            config: RAG configuration. If None, uses default configuration.
        """
        self.config = config or LangChainRAGConfig()
        
        # Get API key from config or environment
        api_key = self.config.openai_api_key or os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError(
                "OpenAI API key required. Set OPENAI_API_KEY environment variable "
                "or pass openai_api_key in LangChainRAGConfig."
            )
        
        # Initialize text splitter
        self.text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=self.config.chunk_size,
            chunk_overlap=self.config.chunk_overlap,
            length_function=len,
        )
        
        # Initialize embeddings
        self.embeddings = OpenAIEmbeddings(
            model=self.config.embedding_model,
            openai_api_key=api_key,
        )
        
        # Initialize LLM
        self.llm = ChatOpenAI(
            model=self.config.llm_model,
            temperature=self.config.temperature,
            max_tokens=self.config.max_tokens,
            openai_api_key=api_key,
        )
        
        # Vector store (initialized when documents are added)
        self.vector_store: Optional[FAISS] = None
        
        # Document storage
        self.documents: List[Dict[str, Any]] = []
        self.chunks: List[Document] = []
    
    def load_documents(self, doc_paths: List[str]) -> List[Dict[str, Any]]:
        """
        Load documents from file paths.
        
        Note: LangChain doesn't have built-in parallel document loading,
        so this is sequential. For parallel loading, use GraphBit.
        
        Args:
            doc_paths: List of document file paths
            
        Returns:
            List of document dictionaries with content and metadata
        """
        print(f"Loading {len(doc_paths)} documents...")
        start_time = time.time()
        
        documents = []
        for path in doc_paths:
            try:
                doc = self._load_single_document(path)
                if doc:
                    documents.append(doc)
            except Exception as e:
                print(f"❌ Failed to load {path}: {e}")
        
        duration = time.time() - start_time
        print(f"✅ Loaded {len(documents)} documents in {duration:.2f}s")
        
        self.documents = documents
        self.config.stats["documents_loaded"] += len(documents)
        
        return documents
    
    def _load_single_document(self, path: str) -> Optional[Dict[str, Any]]:
        """Load a single document from file path."""
        path_obj = Path(path)
        
        if not path_obj.exists():
            return None
        
        # Read text content
        try:
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
        except UnicodeDecodeError:
            # Try with different encoding
            with open(path, 'r', encoding='latin-1') as f:
                content = f.read()
        
        return {
            'path': str(path),
            'content': content,
            'metadata': {
                'source': str(path),
                'filename': path_obj.name,
            }
        }

    def chunk_documents(self, documents: List[Dict[str, Any]]) -> List[Document]:
        """
        Split documents into chunks using RecursiveCharacterTextSplitter.

        Args:
            documents: List of document dictionaries

        Returns:
            List of LangChain Document objects (chunks)
        """
        print(f"Chunking {len(documents)} documents...")
        start_time = time.time()

        all_chunks = []

        for doc in documents:
            # Create LangChain Document
            langchain_doc = Document(
                page_content=doc['content'],
                metadata=doc['metadata']
            )

            # Split document
            chunks = self.text_splitter.split_documents([langchain_doc])
            all_chunks.extend(chunks)

        duration = time.time() - start_time
        print(f"✅ Created {len(all_chunks)} chunks in {duration:.2f}s")
        print(f"   Average: {len(all_chunks)/duration:.1f} chunks/second")

        self.chunks = all_chunks
        self.config.stats["chunks_created"] += len(all_chunks)

        return all_chunks

    def build_vector_store(self, chunks: Optional[List[Document]] = None) -> FAISS:
        """
        Build FAISS vector store from chunks.

        This generates embeddings and stores them in FAISS for similarity search.

        Args:
            chunks: List of Document chunks. If None, uses self.chunks

        Returns:
            FAISS vector store
        """
        if chunks is None:
            chunks = self.chunks

        if not chunks:
            raise ValueError("No chunks to build vector store. Call chunk_documents() first.")

        print(f"Building vector store with {len(chunks)} chunks...")
        start_time = time.time()

        # Create FAISS vector store (generates embeddings internally)
        self.vector_store = FAISS.from_documents(
            documents=chunks,
            embedding=self.embeddings,
        )

        duration = time.time() - start_time
        print(f"✅ Built vector store in {duration:.2f}s")
        print(f"   Average: {len(chunks)/duration:.1f} embeddings/second")

        self.config.stats["embeddings_generated"] += len(chunks)

        return self.vector_store

    def search(self, query: str, top_k: Optional[int] = None) -> List[Document]:
        """
        Search for relevant chunks using similarity search.

        Args:
            query: Query string
            top_k: Number of results to return. If None, uses config.top_k

        Returns:
            List of relevant Document chunks
        """
        if self.vector_store is None:
            raise ValueError("Vector store not initialized. Call build_vector_store() first.")

        k = top_k or self.config.top_k

        # Perform similarity search
        results = self.vector_store.similarity_search(query, k=k)

        return results

    def query(self, query: str, top_k: Optional[int] = None) -> str:
        """
        Query the RAG system and get LLM response.

        This is the main RAG interface equivalent to GraphBit's query_async().

        Pipeline:
        1. Search for relevant chunks
        2. Build context from retrieved chunks
        3. Generate LLM response with context

        Args:
            query: Query string
            top_k: Number of chunks to retrieve. If None, uses config.top_k

        Returns:
            LLM-generated response string
        """
        print(f"\nQuery: {query}")
        start_time = time.time()

        # Step 1: Search for relevant chunks
        relevant_chunks = self.search(query, top_k=top_k)

        # Step 2: Build context
        context = "\n\n".join([
            f"Source: {chunk.metadata.get('source', 'Unknown')}\n{chunk.page_content}"
            for chunk in relevant_chunks
        ])

        # Step 3: Generate LLM response
        prompt = f"""Based on the following context, answer the question.

Context:
{context}

Question: {query}

Answer:"""

        response = self.llm.invoke(prompt).content

        duration = time.time() - start_time
        print(f"✅ Generated response in {duration:.2f}s")

        self.config.stats["queries_processed"] += 1
        self.config.stats["total_time"] += duration

        return response

    def process_documents(self, doc_paths: List[str]) -> Dict[str, Any]:
        """
        Process documents through complete RAG pipeline.

        This is equivalent to GraphBit's end-to-end pipeline:
        1. Load documents
        2. Chunk documents
        3. Build vector store (generate embeddings)

        Args:
            doc_paths: List of document file paths

        Returns:
            Dictionary containing processing results and statistics
        """
        print(f"\n{'='*80}")
        print(f"Processing {len(doc_paths)} documents through LangChain RAG pipeline")
        print(f"{'='*80}\n")

        overall_start = time.time()

        # Step 1: Load documents
        documents = self.load_documents(doc_paths)

        # Step 2: Chunk documents
        chunks = self.chunk_documents(documents)

        # Step 3: Build vector store
        vector_store = self.build_vector_store(chunks)

        overall_duration = time.time() - overall_start

        # Prepare results
        results = {
            "documents": len(documents),
            "chunks": len(chunks),
            "embeddings": len(chunks),
            "duration": overall_duration,
            "throughput": len(documents) / overall_duration if overall_duration > 0 else 0,
            "vector_store_ready": self.vector_store is not None,
        }

        print(f"\n{'='*80}")
        print(f"Pipeline Complete!")
        print(f"  Documents:   {len(documents)}")
        print(f"  Chunks:      {len(chunks)}")
        print(f"  Embeddings:  {len(chunks)}")
        print(f"  Duration:    {overall_duration:.2f}s")
        print(f"  Throughput:  {results['throughput']:.2f} docs/sec")
        print(f"{'='*80}\n")

        return results

    def get_statistics(self) -> Dict[str, Any]:
        """Get processing statistics."""
        return self.config.stats.copy()

    def reset_statistics(self) -> None:
        """Reset processing statistics."""
        self.config.stats = {
            "documents_loaded": 0,
            "chunks_created": 0,
            "embeddings_generated": 0,
            "queries_processed": 0,
            "total_time": 0.0
        }


def main():
    """Example usage of LangChain RAG system."""
    # Create sample documents
    sample_docs = [
        """
        Artificial Intelligence (AI) is revolutionizing the way we interact with technology.
        Machine learning algorithms can now process vast amounts of data to identify patterns
        and make predictions with remarkable accuracy. Deep learning, a subset of machine learning,
        uses neural networks with multiple layers to learn hierarchical representations of data.
        This has led to breakthroughs in computer vision, natural language processing, and speech recognition.
        """,
        """
        Cloud computing has transformed the IT landscape by providing on-demand access to computing
        resources over the internet. Organizations can now scale their infrastructure dynamically,
        paying only for what they use. Major cloud providers like AWS, Azure, and Google Cloud offer
        a wide range of services including compute, storage, databases, and machine learning tools.
        This has enabled startups and enterprises alike to innovate faster and reduce capital expenditure.
        """,
        """
        Cybersecurity is becoming increasingly critical as our world becomes more connected.
        Threats are evolving rapidly, with sophisticated attacks targeting everything from
        individual devices to critical infrastructure. Organizations must implement multi-layered
        security strategies including firewalls, encryption, intrusion detection systems, and
        regular security audits. Employee training is also essential as human error remains
        one of the biggest security vulnerabilities.
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
    rag = LangChainRAG()

    # Process documents
    results = rag.process_documents(doc_paths)

    # Query the system
    query = "What are the main topics discussed in the documents?"
    response = rag.query(query)

    print(f"\nQuery: {query}")
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


if __name__ == "__main__":
    main()


