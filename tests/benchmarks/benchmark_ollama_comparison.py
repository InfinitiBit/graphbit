"""Framework Comparison with Ollama: GraphBit vs LangChain (Local Models)

This benchmark compares GraphBit ParallelRAG and LangChain RAG implementations
using local Ollama models (no API costs).

Features:
- Identical test documents for both frameworks
- Identical configuration (chunk_size=500, chunk_overlap=50)
- Local Ollama models (llama3:8b, nomic-embed-text)
- Resource monitoring (CPU %, Memory MB)
- End-to-end RAG testing (load + chunk + embed + query)
- Side-by-side performance comparison
- Speedup calculations

Usage:
    # Test both frameworks with Ollama
    python tests/benchmarks/benchmark_ollama_comparison.py --framework both
    
    # Test GraphBit only
    python tests/benchmarks/benchmark_ollama_comparison.py --framework graphbit
    
    # Test LangChain only
    python tests/benchmarks/benchmark_ollama_comparison.py --framework langchain
    
    # Custom test parameters
    python tests/benchmarks/benchmark_ollama_comparison.py --max-docs 100 --max-workers 20
    
    # Custom Ollama models
    python tests/benchmarks/benchmark_ollama_comparison.py --llm-model mistral:7b --embedding-model mxbai-embed-large
"""

import argparse
import gc
import json
import os
import sys
import tempfile
import time
from concurrent.futures import ThreadPoolExecutor
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple

import psutil

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from benchmark_utils import get_system_info

# Import Ollama-enabled RAG implementations
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "examples"))
from parallel_rag_ollama import ParallelRAGOllama
from langchain_rag_ollama import LangChainRAGOllama, LangChainRAGOllamaConfig


# ============================================================================
# Safety Thresholds
# ============================================================================

MEMORY_THRESHOLD_PERCENT = 90.0
CPU_THRESHOLD_PERCENT = 95.0
CPU_SUSTAINED_DURATION = 10.0
SAMPLE_INTERVAL = 0.1


# ============================================================================
# Data Classes
# ============================================================================

@dataclass
class ResourceSnapshot:
    """Snapshot of system resources at a point in time."""
    timestamp: float
    cpu_percent: float
    memory_mb: float
    memory_percent: float
    thread_count: int


@dataclass
class FrameworkTestResult:
    """Results from a framework test run with Ollama."""
    framework: str  # "graphbit" or "langchain"
    test_name: str
    num_documents: int
    num_workers: int
    document_size_words: int
    
    # Performance metrics
    total_time: float
    load_time: float
    chunk_time: float
    embed_time: float
    query_time: float
    
    # Output metrics
    documents_loaded: int
    chunks_created: int
    embeddings_generated: int
    queries_processed: int
    
    # Throughput
    throughput_docs_per_sec: float
    throughput_chunks_per_sec: float
    
    # Resource metrics
    baseline_cpu_percent: float
    baseline_memory_mb: float
    peak_cpu_percent: float
    peak_memory_mb: float
    avg_cpu_percent: float
    avg_memory_mb: float
    memory_growth_mb: float
    
    # Efficiency metrics
    cpu_efficiency: float  # throughput / avg_cpu_percent
    memory_efficiency: float  # chunks / memory_growth_mb
    
    # Resource snapshots
    resource_snapshots: List[Dict]


# ============================================================================
# Utility Functions
# ============================================================================

def check_ollama_running(base_url: str = "http://localhost:11434") -> bool:
    """Check if Ollama is running and accessible."""
    try:
        import requests
        response = requests.get(f"{base_url}/api/tags", timeout=2)
        return response.status_code == 200
    except Exception:
        return False


def check_ollama_model(model: str, base_url: str = "http://localhost:11434") -> bool:
    """Check if a specific Ollama model is available."""
    try:
        import requests
        response = requests.get(f"{base_url}/api/tags", timeout=2)
        if response.status_code == 200:
            data = response.json()
            models = [m["name"] for m in data.get("models", [])]
            # Check for exact match or with :latest tag
            return model in models or f"{model}:latest" in models
    except Exception:
        return False


def create_test_documents(num_docs: int, words_per_doc: int, output_dir: str) -> List[str]:
    """
    Create test documents with specified word count.
    
    Args:
        num_docs: Number of documents to create
        words_per_doc: Number of words per document
        output_dir: Directory to save documents
        
    Returns:
        List of document file paths
    """
    # Sample words for generating content
    words = [
        "machine", "learning", "artificial", "intelligence", "neural", "network",
        "deep", "learning", "data", "science", "algorithm", "model", "training",
        "prediction", "classification", "regression", "clustering", "optimization",
        "gradient", "descent", "backpropagation", "activation", "function", "layer",
        "tensor", "matrix", "vector", "embedding", "transformer", "attention",
    ]
    
    doc_paths = []
    
    for i in range(num_docs):
        # Generate document content
        content_words = []
        for j in range(words_per_doc):
            word = words[j % len(words)]
            content_words.append(word)
        
        content = " ".join(content_words)
        
        # Save to file
        doc_path = Path(output_dir) / f"test_doc_{i}.txt"
        with open(doc_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        doc_paths.append(str(doc_path))
    
    return doc_paths


def get_resource_snapshot() -> ResourceSnapshot:
    """Get current system resource usage."""
    process = psutil.Process()
    memory_info = process.memory_info()

    return ResourceSnapshot(
        timestamp=time.time(),
        cpu_percent=process.cpu_percent(),
        memory_mb=memory_info.rss / (1024 * 1024),
        memory_percent=process.memory_percent(),
        thread_count=process.num_threads(),
    )


# ============================================================================
# Test Functions
# ============================================================================

async def test_graphbit_ollama(
    doc_paths: List[str],
    max_workers: int,
    llm_model: str,
    embedding_model: str,
    ollama_base_url: str,
) -> FrameworkTestResult:
    """
    Test GraphBit ParallelRAG with Ollama.

    Args:
        doc_paths: List of document paths
        max_workers: Number of parallel workers
        llm_model: Ollama LLM model name
        embedding_model: Ollama embedding model name
        ollama_base_url: Ollama server URL

    Returns:
        Test results
    """
    print(f"\n{'='*80}")
    print(f"Testing GraphBit ParallelRAG with Ollama")
    print(f"{'='*80}")

    # Get baseline resources
    gc.collect()
    time.sleep(0.5)
    baseline = get_resource_snapshot()

    # Initialize RAG
    rag = ParallelRAGOllama(
        llm_model=llm_model,
        embedding_model=embedding_model,
        ollama_base_url=ollama_base_url,
        max_workers=max_workers,
    )

    # Track resource snapshots
    snapshots = []

    # Step 1: Load documents
    load_start = time.time()
    documents = rag.load_documents_parallel(doc_paths)
    load_time = time.time() - load_start
    snapshots.append(asdict(get_resource_snapshot()))

    # Step 2: Chunk documents
    chunk_start = time.time()
    chunks = rag.chunk_documents_parallel(documents)
    chunk_time = time.time() - chunk_start
    snapshots.append(asdict(get_resource_snapshot()))

    # Step 3: Generate embeddings
    embed_start = time.time()
    chunks_with_embeddings = rag.embed_chunks(chunks)
    embed_time = time.time() - embed_start
    snapshots.append(asdict(get_resource_snapshot()))

    # Step 4: Store chunks
    rag.store_chunks(chunks_with_embeddings)
    snapshots.append(asdict(get_resource_snapshot()))

    # Step 5: Query
    query_start = time.time()
    response = await rag.query_async("What is machine learning?")
    query_time = time.time() - query_start
    snapshots.append(asdict(get_resource_snapshot()))

    # Get final resources
    final = get_resource_snapshot()

    # Calculate metrics
    total_time = load_time + chunk_time + embed_time + query_time
    throughput_docs = len(documents) / total_time if total_time > 0 else 0
    throughput_chunks = len(chunks) / total_time if total_time > 0 else 0

    # Calculate resource metrics
    peak_cpu = max(s["cpu_percent"] for s in snapshots)
    peak_memory = max(s["memory_mb"] for s in snapshots)
    avg_cpu = sum(s["cpu_percent"] for s in snapshots) / len(snapshots)
    avg_memory = sum(s["memory_mb"] for s in snapshots) / len(snapshots)
    memory_growth = peak_memory - baseline.memory_mb

    cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0
    memory_efficiency = len(chunks) / memory_growth if memory_growth > 0 else 0

    print(f"\n GraphBit Results:")
    print(f"  Total Time: {total_time:.2f}s")
    print(f"  Load Time: {load_time:.2f}s")
    print(f"  Chunk Time: {chunk_time:.2f}s")
    print(f"  Embed Time: {embed_time:.2f}s")
    print(f"  Query Time: {query_time:.2f}s")
    print(f"  Throughput: {throughput_docs:.2f} docs/sec")
    print(f"  Peak Memory: {peak_memory:.2f} MB")
    print(f"  Avg CPU: {avg_cpu:.1f}%")

    return FrameworkTestResult(
        framework="graphbit",
        test_name=f"Ollama_{len(doc_paths)}docs_{max_workers}workers",
        num_documents=len(doc_paths),
        num_workers=max_workers,
        document_size_words=200,  # Default
        total_time=total_time,
        load_time=load_time,
        chunk_time=chunk_time,
        embed_time=embed_time,
        query_time=query_time,
        documents_loaded=len(documents),
        chunks_created=len(chunks),
        embeddings_generated=len(chunks),
        queries_processed=1,
        throughput_docs_per_sec=throughput_docs,
        throughput_chunks_per_sec=throughput_chunks,
        baseline_cpu_percent=baseline.cpu_percent,
        baseline_memory_mb=baseline.memory_mb,
        peak_cpu_percent=peak_cpu,
        peak_memory_mb=peak_memory,
        avg_cpu_percent=avg_cpu,
        avg_memory_mb=avg_memory,
        memory_growth_mb=memory_growth,
        cpu_efficiency=cpu_efficiency,
        memory_efficiency=memory_efficiency,
        resource_snapshots=snapshots,
    )


def test_langchain_ollama(
    doc_paths: List[str],
    llm_model: str,
    embedding_model: str,
    ollama_base_url: str,
) -> FrameworkTestResult:
    """
    Test LangChain RAG with Ollama.

    Args:
        doc_paths: List of document paths
        llm_model: Ollama LLM model name
        embedding_model: Ollama embedding model name
        ollama_base_url: Ollama server URL

    Returns:
        Test results
    """
    print(f"\n{'='*80}")
    print(f"Testing LangChain RAG with Ollama")
    print(f"{'='*80}")

    # Get baseline resources
    gc.collect()
    time.sleep(0.5)
    baseline = get_resource_snapshot()

    # Initialize RAG
    config = LangChainRAGOllamaConfig(
        ollama_base_url=ollama_base_url,
        embedding_model=embedding_model,
        llm_model=llm_model,
    )
    rag = LangChainRAGOllama(config)

    # Track resource snapshots
    snapshots = []

    # Process documents (load + chunk + embed)
    start_time = time.time()
    results = rag.process_documents(doc_paths)
    process_time = time.time() - start_time
    snapshots.append(asdict(get_resource_snapshot()))

    # Query
    query_start = time.time()
    response = rag.query("What is machine learning?")
    query_time = time.time() - query_start
    snapshots.append(asdict(get_resource_snapshot()))

    # Get final resources
    final = get_resource_snapshot()

    # Calculate metrics
    total_time = process_time + query_time
    load_time = results.get("load_time", 0)
    chunk_time = results.get("chunk_time", 0)
    embed_time = results.get("embed_time", 0)

    num_docs = results.get("documents", 0)
    num_chunks = results.get("chunks", 0)

    throughput_docs = num_docs / total_time if total_time > 0 else 0
    throughput_chunks = num_chunks / total_time if total_time > 0 else 0

    # Calculate resource metrics
    peak_cpu = max(s["cpu_percent"] for s in snapshots)
    peak_memory = max(s["memory_mb"] for s in snapshots)
    avg_cpu = sum(s["cpu_percent"] for s in snapshots) / len(snapshots)
    avg_memory = sum(s["memory_mb"] for s in snapshots) / len(snapshots)
    memory_growth = peak_memory - baseline.memory_mb

    cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0
    memory_efficiency = num_chunks / memory_growth if memory_growth > 0 else 0

    print(f"\n LangChain Results:")
    print(f"  Total Time: {total_time:.2f}s")
    print(f"  Load Time: {load_time:.2f}s")
    print(f"  Chunk Time: {chunk_time:.2f}s")
    print(f"  Embed Time: {embed_time:.2f}s")
    print(f"  Query Time: {query_time:.2f}s")
    print(f"  Throughput: {throughput_docs:.2f} docs/sec")
    print(f"  Peak Memory: {peak_memory:.2f} MB")
    print(f"  Avg CPU: {avg_cpu:.1f}%")

    return FrameworkTestResult(
        framework="langchain",
        test_name=f"Ollama_{len(doc_paths)}docs",
        num_documents=len(doc_paths),
        num_workers=1,  # LangChain is sequential
        document_size_words=200,  # Default
        total_time=total_time,
        load_time=load_time,
        chunk_time=chunk_time,
        embed_time=embed_time,
        query_time=query_time,
        documents_loaded=num_docs,
        chunks_created=num_chunks,
        embeddings_generated=num_chunks,
        queries_processed=1,
        throughput_docs_per_sec=throughput_docs,
        throughput_chunks_per_sec=throughput_chunks,
        baseline_cpu_percent=baseline.cpu_percent,
        baseline_memory_mb=baseline.memory_mb,
        peak_cpu_percent=peak_cpu,
        peak_memory_mb=peak_memory,
        avg_cpu_percent=avg_cpu,
        avg_memory_mb=avg_memory,
        memory_growth_mb=memory_growth,
        cpu_efficiency=cpu_efficiency,
        memory_efficiency=memory_efficiency,
        resource_snapshots=snapshots,
    )


# ============================================================================
# Main Function
# ============================================================================

async def main():
    """Main benchmark function."""
    parser = argparse.ArgumentParser(
        description="Framework Comparison Benchmark with Ollama (GraphBit vs LangChain)"
    )

    parser.add_argument(
        "--framework",
        type=str,
        choices=["graphbit", "langchain", "both"],
        default="both",
        help="Framework to test (default: both)"
    )

    parser.add_argument(
        "--max-docs",
        type=int,
        default=10,
        help="Maximum number of documents to test (default: 10)"
    )

    parser.add_argument(
        "--max-workers",
        type=int,
        default=10,
        help="Maximum number of parallel workers for GraphBit (default: 10)"
    )

    parser.add_argument(
        "--words-per-doc",
        type=int,
        default=200,
        help="Number of words per document (default: 200)"
    )

    parser.add_argument(
        "--llm-model",
        type=str,
        default="llama3:8b",
        help="Ollama LLM model (default: llama3:8b)"
    )

    parser.add_argument(
        "--embedding-model",
        type=str,
        default="nomic-embed-text",
        help="Ollama embedding model (default: nomic-embed-text)"
    )

    parser.add_argument(
        "--ollama-url",
        type=str,
        default="http://localhost:11434",
        help="Ollama server URL (default: http://localhost:11434)"
    )

    parser.add_argument(
        "--output",
        type=str,
        default="ollama_comparison_results.json",
        help="Output JSON file (default: ollama_comparison_results.json)"
    )

    args = parser.parse_args()

    # Print header
    print("=" * 80)
    print("Framework Comparison Benchmark with Ollama: GraphBit vs LangChain")
    print("=" * 80)

    # Check if Ollama is running
    print("\nChecking Ollama status...")
    if not check_ollama_running(args.ollama_url):
        print(f"❌ Ollama is not running at {args.ollama_url}")
        print("   Please start Ollama first. See OLLAMA_SETUP_GUIDE.md")
        return

    print(f"✅ Ollama is running at {args.ollama_url}")

    # Check if models are available
    print(f"\nChecking Ollama models...")
    if not check_ollama_model(args.llm_model, args.ollama_url):
        print(f"❌ LLM model '{args.llm_model}' not found")
        print(f"   Pull it with: ollama pull {args.llm_model}")
        return

    print(f"✅ LLM model '{args.llm_model}' is available")

    if not check_ollama_model(args.embedding_model, args.ollama_url):
        print(f"❌ Embedding model '{args.embedding_model}' not found")
        print(f"   Pull it with: ollama pull {args.embedding_model}")
        return

    print(f"✅ Embedding model '{args.embedding_model}' is available")

    # Print system information
    print("\nSystem Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")

    # Print test configuration
    print("\nTest Configuration:")
    print(f"  Framework: {args.framework}")
    print(f"  Max Documents: {args.max_docs}")
    print(f"  Max Workers: {args.max_workers}")
    print(f"  Words per Document: {args.words_per_doc}")
    print(f"  LLM Model: {args.llm_model}")
    print(f"  Embedding Model: {args.embedding_model}")
    print(f"  Ollama URL: {args.ollama_url}")

    # Create test documents
    print(f"\nCreating {args.max_docs} test documents ({args.words_per_doc} words each)...")
    temp_dir = tempfile.mkdtemp()
    doc_paths = create_test_documents(args.max_docs, args.words_per_doc, temp_dir)
    print(f"✅ Created {len(doc_paths)} documents in {temp_dir}")

    # Run tests
    results = {}

    if args.framework in ["graphbit", "both"]:
        graphbit_result = await test_graphbit_ollama(
            doc_paths,
            args.max_workers,
            args.llm_model,
            args.embedding_model,
            args.ollama_url,
        )
        results["graphbit"] = asdict(graphbit_result)

    if args.framework in ["langchain", "both"]:
        langchain_result = test_langchain_ollama(
            doc_paths,
            args.llm_model,
            args.embedding_model,
            args.ollama_url,
        )
        results["langchain"] = asdict(langchain_result)

    # Calculate speedups if both frameworks were tested
    if args.framework == "both":
        print(f"\n{'='*80}")
        print("Comparison Summary")
        print(f"{'='*80}")

        graphbit_time = results["graphbit"]["total_time"]
        langchain_time = results["langchain"]["total_time"]
        speedup = langchain_time / graphbit_time if graphbit_time > 0 else 0

        print(f"\nSpeedup: {speedup:.2f}x (GraphBit is {speedup:.2f}x faster than LangChain)")

        print(f"\nGraphBit Advantages:")
        print(f"  ✅ {speedup:.2f}x faster total time")
        print(f"  ✅ {results['langchain']['load_time'] / results['graphbit']['load_time']:.2f}x faster document loading")
        print(f"  ✅ {results['graphbit']['throughput_docs_per_sec']:.2f} docs/sec vs {results['langchain']['throughput_docs_per_sec']:.2f} docs/sec")

        results["speedups"] = {
            "total_time_speedup": speedup,
            "load_time_speedup": results['langchain']['load_time'] / results['graphbit']['load_time'] if results['graphbit']['load_time'] > 0 else 0,
            "embed_time_speedup": results['langchain']['embed_time'] / results['graphbit']['embed_time'] if results['graphbit']['embed_time'] > 0 else 0,
        }

    # Save results to JSON
    output_data = {
        "test_name": f"Ollama_{args.max_docs}docs_{args.max_workers}workers",
        "num_documents": args.max_docs,
        "configuration": {
            "framework": args.framework,
            "max_workers": args.max_workers,
            "words_per_doc": args.words_per_doc,
            "llm_model": args.llm_model,
            "embedding_model": args.embedding_model,
            "ollama_url": args.ollama_url,
        },
        "system_info": sys_info,
        "results": results,
    }

    with open(args.output, 'w') as f:
        json.dump(output_data, f, indent=2)

    print(f"\nResults saved to: {args.output}")

    # Cleanup
    import shutil
    shutil.rmtree(temp_dir)
    print(f"✅ Cleaned up temporary directory")


if __name__ == "__main__":
    import asyncio
    asyncio.run(main())


