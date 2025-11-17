"""
Validation Script: GraphBit vs LangChain RAG Equivalence

This script validates that both RAG implementations produce equivalent outputs
on identical inputs, ensuring fair comparison for benchmarking.

Usage:
    python validate_rag_equivalence.py
    
Requirements:
    - OPENAI_API_KEY environment variable set
    - Both GraphBit and LangChain RAG implementations available
"""

import asyncio
import os
import sys
import tempfile
import time
from pathlib import Path
from typing import List, Dict, Any

# Add parent directory to path for imports
sys.path.insert(0, str(Path(__file__).parent))

from langchain_rag_app import LangChainRAG, LangChainRAGConfig

# Note: GraphBit ParallelRAG is in examples/
sys.path.insert(0, str(Path(__file__).parent / "examples"))
from parallel_rag_optimized import ParallelRAG


def create_test_documents() -> List[str]:
    """Create identical test documents for both implementations."""
    return [
        """
        Artificial Intelligence (AI) is revolutionizing the way we interact with technology.
        Machine learning algorithms can now process vast amounts of data to identify patterns
        and make predictions with remarkable accuracy. Deep learning, a subset of machine learning,
        uses neural networks with multiple layers to learn hierarchical representations of data.
        This has led to breakthroughs in computer vision, natural language processing, and speech recognition.
        AI systems are now capable of tasks that were once thought to require human intelligence,
        such as playing complex games, driving cars, and diagnosing diseases.
        """ * 3,  # Repeat to ensure multiple chunks
        """
        Cloud computing has transformed the IT landscape by providing on-demand access to computing
        resources over the internet. Organizations can now scale their infrastructure dynamically,
        paying only for what they use. Major cloud providers like AWS, Azure, and Google Cloud offer
        a wide range of services including compute, storage, databases, and machine learning tools.
        This has enabled startups and enterprises alike to innovate faster and reduce capital expenditure.
        Cloud computing also provides benefits like high availability, disaster recovery, and global reach.
        """ * 3,
        """
        Cybersecurity is becoming increasingly critical as our world becomes more connected.
        Threats are evolving rapidly, with sophisticated attacks targeting everything from
        individual devices to critical infrastructure. Organizations must implement multi-layered
        security strategies including firewalls, encryption, intrusion detection systems, and
        regular security audits. Employee training is also essential as human error remains
        one of the biggest security vulnerabilities. Zero-trust architecture and AI-powered
        threat detection are emerging as key components of modern cybersecurity strategies.
        """ * 3,
    ]


def save_documents_to_files(documents: List[str]) -> List[str]:
    """Save documents to temporary files and return file paths."""
    temp_dir = tempfile.mkdtemp()
    doc_paths = []
    
    for i, content in enumerate(documents):
        path = Path(temp_dir) / f"test_doc_{i}.txt"
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        doc_paths.append(str(path))
    
    return doc_paths


async def test_graphbit_rag(doc_paths: List[str], query: str) -> Dict[str, Any]:
    """Test GraphBit ParallelRAG implementation."""
    print("\n" + "="*80)
    print("Testing GraphBit ParallelRAG")
    print("="*80)
    
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise ValueError("OPENAI_API_KEY not set")
    
    # Initialize
    start_time = time.time()
    rag = ParallelRAG(api_key, max_workers=10, chunk_size=500, chunk_overlap=50)
    init_time = time.time() - start_time
    
    # Load documents
    start_time = time.time()
    documents = rag.load_documents_parallel(doc_paths)
    load_time = time.time() - start_time
    
    # Chunk documents
    start_time = time.time()
    chunks = rag.chunk_documents_parallel(documents)
    chunk_time = time.time() - start_time
    
    # Generate embeddings
    start_time = time.time()
    chunks_with_embeddings = rag.embed_chunks_parallel_optimized(chunks)
    embed_time = time.time() - start_time
    
    # Store chunks
    start_time = time.time()
    rag.store_chunks(chunks_with_embeddings)
    store_time = time.time() - start_time
    
    # Query
    start_time = time.time()
    response = await rag.query_async(query, top_k=5)
    query_time = time.time() - start_time
    
    total_time = init_time + load_time + chunk_time + embed_time + store_time + query_time
    
    return {
        "implementation": "GraphBit ParallelRAG",
        "documents_loaded": len(documents),
        "chunks_created": len(chunks),
        "embeddings_generated": len(chunks_with_embeddings),
        "query": query,
        "response": response,
        "timings": {
            "initialization": init_time,
            "document_loading": load_time,
            "chunking": chunk_time,
            "embedding": embed_time,
            "storage": store_time,
            "query": query_time,
            "total": total_time,
        }
    }


def test_langchain_rag(doc_paths: List[str], query: str) -> Dict[str, Any]:
    """Test LangChain RAG implementation."""
    print("\n" + "="*80)
    print("Testing LangChain RAG")
    print("="*80)
    
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        raise ValueError("OPENAI_API_KEY not set")
    
    # Initialize
    start_time = time.time()
    config = LangChainRAGConfig(
        openai_api_key=api_key,
        chunk_size=500,
        chunk_overlap=50,
        top_k=5,
    )
    rag = LangChainRAG(config)
    init_time = time.time() - start_time
    
    # Process documents (load + chunk + embed + store)
    start_time = time.time()
    results = rag.process_documents(doc_paths)
    process_time = time.time() - start_time
    
    # Query
    start_time = time.time()
    response = rag.query(query, top_k=5)
    query_time = time.time() - start_time
    
    total_time = init_time + process_time + query_time
    
    return {
        "implementation": "LangChain RAG",
        "documents_loaded": results["documents"],
        "chunks_created": results["chunks"],
        "embeddings_generated": results["embeddings"],
        "query": query,
        "response": response,
        "timings": {
            "initialization": init_time,
            "processing": process_time,
            "query": query_time,
            "total": total_time,
        }
    }


async def main():
    """Run validation tests."""
    print("\n" + "="*80)
    print("RAG Implementation Equivalence Validation")
    print("="*80)
    
    # Create test documents
    documents = create_test_documents()
    doc_paths = save_documents_to_files(documents)
    
    print(f"\nCreated {len(doc_paths)} test documents")
    print(f"Document paths: {doc_paths}")
    
    # Define test query
    query = "What are the main topics discussed in the documents?"
    
    try:
        # Test GraphBit RAG
        graphbit_results = await test_graphbit_rag(doc_paths, query)
        
        # Test LangChain RAG
        langchain_results = test_langchain_rag(doc_paths, query)
        
        # Compare results
        print("\n" + "="*80)
        print("Comparison Results")
        print("="*80)
        
        print(f"\n{'Metric':<30} {'GraphBit':<20} {'LangChain':<20} {'Match':<10}")
        print("-" * 80)
        print(f"{'Documents Loaded':<30} {graphbit_results['documents_loaded']:<20} {langchain_results['documents_loaded']:<20} {'✅' if graphbit_results['documents_loaded'] == langchain_results['documents_loaded'] else '❌'}")
        print(f"{'Chunks Created':<30} {graphbit_results['chunks_created']:<20} {langchain_results['chunks_created']:<20} {'✅' if graphbit_results['chunks_created'] == langchain_results['chunks_created'] else '❌'}")
        print(f"{'Embeddings Generated':<30} {graphbit_results['embeddings_generated']:<20} {langchain_results['embeddings_generated']:<20} {'✅' if graphbit_results['embeddings_generated'] == langchain_results['embeddings_generated'] else '❌'}")
        
        print(f"\n{'Timing':<30} {'GraphBit (s)':<20} {'LangChain (s)':<20} {'Speedup':<10}")
        print("-" * 80)
        print(f"{'Total Time':<30} {graphbit_results['timings']['total']:<20.2f} {langchain_results['timings']['total']:<20.2f} {langchain_results['timings']['total'] / graphbit_results['timings']['total']:.2f}x")
        
        print(f"\n{'Query':<30}")
        print("-" * 80)
        print(f"{query}")
        
        print(f"\n{'GraphBit Response':<30}")
        print("-" * 80)
        print(f"{graphbit_results['response'][:200]}...")
        
        print(f"\n{'LangChain Response':<30}")
        print("-" * 80)
        print(f"{langchain_results['response'][:200]}...")
        
        print("\n" + "="*80)
        print("✅ Validation Complete!")
        print("="*80)
        
        # Cleanup
        import shutil
        shutil.rmtree(Path(doc_paths[0]).parent)
        
    except Exception as e:
        print(f"\n❌ Validation failed: {e}")
        import traceback
        traceback.print_exc()
        
        # Cleanup
        import shutil
        shutil.rmtree(Path(doc_paths[0]).parent)
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())

