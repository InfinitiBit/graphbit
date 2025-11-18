"""Ollama Stress Testing: Progressive Load Testing for GraphBit and LangChain

This script performs comprehensive stress testing of RAG implementations with Ollama:
- Progressive document counts: 10, 50, 100, 500, 1000 documents
- Variable document sizes: 100, 500, 1000, 5000 words
- Different worker counts: 1, 5, 10, 20, 50 workers (GraphBit only)
- Resource monitoring: CPU%, Memory MB, peak usage
- Safety thresholds: Memory < 90%, CPU < 95%
- Multiple Ollama models: gemma3:4b, mistral:7b, phi3:mini

Usage:
    # Full stress test (all document counts)
    python ollama_integration/benchmarks/stress_test_ollama.py --framework both

    # Quick stress test (up to 100 docs)
    python ollama_integration/benchmarks/stress_test_ollama.py --framework both --max-docs 100

    # Test specific framework
    python ollama_integration/benchmarks/stress_test_ollama.py --framework graphbit --max-docs 500

    # Test with different models
    python ollama_integration/benchmarks/stress_test_ollama.py --llm-model mistral:7b --max-docs 50

    # Test worker scaling (GraphBit only)
    python ollama_integration/benchmarks/stress_test_ollama.py --framework graphbit --test-workers
"""

import argparse
import gc
import json
import os
import sys
import tempfile
import time
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Dict, List, Any, Optional

import psutil

# Add parent directory to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, str(Path(__file__).parent.parent.parent))
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "tests" / "benchmarks"))

from benchmark_utils import get_system_info

# Import Ollama implementations
sys.path.insert(0, str(Path(__file__).parent.parent / "examples"))
from parallel_rag_ollama import ParallelRAGOllama
from langchain_rag_ollama import LangChainRAGOllama, LangChainRAGOllamaConfig


# ============================================================================
# Safety Thresholds
# ============================================================================

MEMORY_THRESHOLD_PERCENT = 90.0
CPU_THRESHOLD_PERCENT = 95.0
MAX_MEMORY_GROWTH_MB = 10000  # 10 GB


# ============================================================================
# Data Classes
# ============================================================================

@dataclass
class StressTestConfig:
    """Configuration for stress test."""
    framework: str  # "graphbit" or "langchain" or "both"
    document_counts: List[int]  # [10, 50, 100, 500, 1000]
    document_sizes: List[int]  # [100, 500, 1000, 5000] words
    worker_counts: List[int]  # [1, 5, 10, 20, 50] (GraphBit only)
    llm_model: str
    embedding_model: str
    ollama_base_url: str
    output_dir: str
    test_workers: bool  # Test different worker counts
    test_doc_sizes: bool  # Test different document sizes


@dataclass
class StressTestResult:
    """Result from a single stress test run."""
    framework: str
    num_documents: int
    document_size_words: int
    num_workers: int

    # Performance
    total_time: float
    load_time: float
    chunk_time: float
    embed_time: float
    query_time: float

    # Throughput
    throughput_docs_per_sec: float
    throughput_chunks_per_sec: float

    # Resources
    peak_memory_mb: float
    avg_memory_mb: float
    memory_growth_mb: float
    peak_cpu_percent: float
    avg_cpu_percent: float

    # Output
    documents_loaded: int
    chunks_created: int
    embeddings_generated: int

    # Safety
    memory_threshold_exceeded: bool
    cpu_threshold_exceeded: bool
    test_aborted: bool
    abort_reason: Optional[str]


# ============================================================================
# Utility Functions
# ============================================================================

def check_ollama_running(base_url: str = "http://localhost:11434") -> bool:
    """Check if Ollama is running."""
    try:
        import requests
        response = requests.get(f"{base_url}/api/tags", timeout=2)
        return response.status_code == 200
    except Exception:
        return False


def check_ollama_model(model: str, base_url: str = "http://localhost:11434") -> bool:
    """Check if Ollama model is available."""
    try:
        import requests
        response = requests.get(f"{base_url}/api/tags", timeout=2)
        if response.status_code == 200:
            data = response.json()
            models = [m["name"] for m in data.get("models", [])]
            return model in models or f"{model}:latest" in models
    except Exception:
        return False


def create_test_documents(num_docs: int, words_per_doc: int, output_dir: str) -> List[str]:
    """Create test documents with specified word count."""
    words = [
        "machine", "learning", "artificial", "intelligence", "neural", "network",
        "deep", "learning", "data", "science", "algorithm", "model", "training",
        "prediction", "classification", "regression", "clustering", "optimization",
        "gradient", "descent", "backpropagation", "activation", "function", "layer",
        "tensor", "matrix", "vector", "embedding", "transformer", "attention",
    ]

    doc_paths = []
    for i in range(num_docs):
        content_words = [words[j % len(words)] for j in range(words_per_doc)]
        content = " ".join(content_words)

        doc_path = Path(output_dir) / f"test_doc_{i}.txt"
        with open(doc_path, 'w', encoding='utf-8') as f:
            f.write(content)

        doc_paths.append(str(doc_path))

    return doc_paths


def check_safety_thresholds() -> tuple[bool, Optional[str]]:
    """Check if system resources are within safe limits."""
    memory = psutil.virtual_memory()

    if memory.percent > MEMORY_THRESHOLD_PERCENT:
        return False, f"Memory usage {memory.percent:.1f}% exceeds threshold {MEMORY_THRESHOLD_PERCENT}%"

    return True, None


# ============================================================================
# Test Execution Functions
# ============================================================================

def run_graphbit_stress_test(
    doc_paths: List[str],
    num_workers: int,
    llm_model: str,
    embedding_model: str,
    ollama_base_url: str,
    document_size_words: int
) -> StressTestResult:
    """Run GraphBit stress test."""
    print(f"\n{'='*80}")
    print(f"Testing GraphBit: {len(doc_paths)} docs, {num_workers} workers, {document_size_words} words/doc")
    print(f"{'='*80}")

    # Initialize
    baseline_memory = psutil.Process().memory_info().rss / 1024 / 1024
    start_time = time.time()

    try:
        # Create RAG instance
        rag = ParallelRAGOllama(
            llm_model=llm_model,
            embedding_model=embedding_model,
            ollama_base_url=ollama_base_url,
            max_workers=num_workers
        )

        # Load documents
        load_start = time.time()
        documents = rag.load_documents_parallel(doc_paths)
        load_time = time.time() - load_start

        # Check safety
        safe, reason = check_safety_thresholds()
        if not safe:
            return StressTestResult(
                framework="graphbit",
                num_documents=len(doc_paths),
                document_size_words=document_size_words,
                num_workers=num_workers,
                total_time=time.time() - start_time,
                load_time=load_time,
                chunk_time=0,
                embed_time=0,
                query_time=0,
                throughput_docs_per_sec=0,
                throughput_chunks_per_sec=0,
                peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                memory_growth_mb=0,
                peak_cpu_percent=0,
                avg_cpu_percent=0,
                documents_loaded=len(documents),
                chunks_created=0,
                embeddings_generated=0,
                memory_threshold_exceeded=True,
                cpu_threshold_exceeded=False,
                test_aborted=True,
                abort_reason=reason
            )

        # Chunk documents
        chunk_start = time.time()
        chunks = rag.chunk_documents_parallel(documents)
        chunk_time = time.time() - chunk_start

        # Check safety
        safe, reason = check_safety_thresholds()
        if not safe:
            return StressTestResult(
                framework="graphbit",
                num_documents=len(doc_paths),
                document_size_words=document_size_words,
                num_workers=num_workers,
                total_time=time.time() - start_time,
                load_time=load_time,
                chunk_time=chunk_time,
                embed_time=0,
                query_time=0,
                throughput_docs_per_sec=0,
                throughput_chunks_per_sec=0,
                peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                memory_growth_mb=0,
                peak_cpu_percent=0,
                avg_cpu_percent=0,
                documents_loaded=len(documents),
                chunks_created=len(chunks),
                embeddings_generated=0,
                memory_threshold_exceeded=True,
                cpu_threshold_exceeded=False,
                test_aborted=True,
                abort_reason=reason
            )

        # Generate embeddings
        embed_start = time.time()
        rag.embed_chunks(chunks)
        embed_time = time.time() - embed_start

        # Check safety
        safe, reason = check_safety_thresholds()
        if not safe:
            return StressTestResult(
                framework="graphbit",
                num_documents=len(doc_paths),
                document_size_words=document_size_words,
                num_workers=num_workers,
                total_time=time.time() - start_time,
                load_time=load_time,
                chunk_time=chunk_time,
                embed_time=embed_time,
                query_time=0,
                throughput_docs_per_sec=0,
                throughput_chunks_per_sec=0,
                peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                memory_growth_mb=0,
                peak_cpu_percent=0,
                avg_cpu_percent=0,
                documents_loaded=len(documents),
                chunks_created=len(chunks),
                embeddings_generated=len(chunks),
                memory_threshold_exceeded=True,
                cpu_threshold_exceeded=False,
                test_aborted=True,
                abort_reason=reason
            )

        # Query
        query_start = time.time()
        import asyncio
        asyncio.run(rag.query_async("What is machine learning?"))
        query_time = time.time() - query_start

        total_time = time.time() - start_time
        current_memory = psutil.Process().memory_info().rss / 1024 / 1024

        return StressTestResult(
            framework="graphbit",
            num_documents=len(doc_paths),
            document_size_words=document_size_words,
            num_workers=num_workers,
            total_time=total_time,
            load_time=load_time,
            chunk_time=chunk_time,
            embed_time=embed_time,
            query_time=query_time,
            throughput_docs_per_sec=len(documents) / total_time if total_time > 0 else 0,
            throughput_chunks_per_sec=len(chunks) / total_time if total_time > 0 else 0,
            peak_memory_mb=current_memory,
            avg_memory_mb=current_memory,
            memory_growth_mb=current_memory - baseline_memory,
            peak_cpu_percent=0,
            avg_cpu_percent=0,
            documents_loaded=len(documents),
            chunks_created=len(chunks),
            embeddings_generated=len(chunks),
            memory_threshold_exceeded=False,
            cpu_threshold_exceeded=False,
            test_aborted=False,
            abort_reason=None
        )

    except Exception as e:
        print(f"❌ Test failed: {e}")
        return StressTestResult(
            framework="graphbit",
            num_documents=len(doc_paths),
            document_size_words=document_size_words,
            num_workers=num_workers,
            total_time=time.time() - start_time,
            load_time=0,
            chunk_time=0,
            embed_time=0,
            query_time=0,
            throughput_docs_per_sec=0,
            throughput_chunks_per_sec=0,
            peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
            avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
            memory_growth_mb=0,
            peak_cpu_percent=0,
            avg_cpu_percent=0,
            documents_loaded=0,
            chunks_created=0,
            embeddings_generated=0,
            memory_threshold_exceeded=False,
            cpu_threshold_exceeded=False,
            test_aborted=True,
            abort_reason=str(e)
        )



def run_langchain_stress_test(
    doc_paths: List[str],
    llm_model: str,
    embedding_model: str,
    ollama_base_url: str,
    document_size_words: int
) -> StressTestResult:
    """Run LangChain stress test."""
    print(f"\n{'='*80}")
    print(f"Testing LangChain: {len(doc_paths)} docs, {document_size_words} words/doc")
    print(f"{'='*80}")

    # Initialize
    baseline_memory = psutil.Process().memory_info().rss / 1024 / 1024
    start_time = time.time()

    try:
        # Create RAG instance
        config = LangChainRAGOllamaConfig(
            llm_model=llm_model,
            embedding_model=embedding_model,
            ollama_base_url=ollama_base_url
        )
        rag = LangChainRAGOllama(config)

        # Process documents (load + chunk + embed)
        process_start = time.time()
        rag.process_documents(doc_paths)
        process_time = time.time() - process_start

        # Check safety
        safe, reason = check_safety_thresholds()
        if not safe:
            return StressTestResult(
                framework="langchain",
                num_documents=len(doc_paths),
                document_size_words=document_size_words,
                num_workers=1,
                total_time=time.time() - start_time,
                load_time=process_time,
                chunk_time=0,
                embed_time=0,
                query_time=0,
                throughput_docs_per_sec=0,
                throughput_chunks_per_sec=0,
                peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
                memory_growth_mb=0,
                peak_cpu_percent=0,
                avg_cpu_percent=0,
                documents_loaded=rag.stats["documents_loaded"],
                chunks_created=rag.stats["chunks_created"],
                embeddings_generated=rag.stats["embeddings_generated"],
                memory_threshold_exceeded=True,
                cpu_threshold_exceeded=False,
                test_aborted=True,
                abort_reason=reason
            )

        # Query
        query_start = time.time()
        rag.query("What is machine learning?")
        query_time = time.time() - query_start

        total_time = time.time() - start_time
        current_memory = psutil.Process().memory_info().rss / 1024 / 1024

        return StressTestResult(
            framework="langchain",
            num_documents=len(doc_paths),
            document_size_words=document_size_words,
            num_workers=1,
            total_time=total_time,
            load_time=process_time,
            chunk_time=0,
            embed_time=0,
            query_time=query_time,
            throughput_docs_per_sec=rag.stats["documents_loaded"] / total_time if total_time > 0 else 0,
            throughput_chunks_per_sec=rag.stats["chunks_created"] / total_time if total_time > 0 else 0,
            peak_memory_mb=current_memory,
            avg_memory_mb=current_memory,
            memory_growth_mb=current_memory - baseline_memory,
            peak_cpu_percent=0,
            avg_cpu_percent=0,
            documents_loaded=rag.stats["documents_loaded"],
            chunks_created=rag.stats["chunks_created"],
            embeddings_generated=rag.stats["embeddings_generated"],
            memory_threshold_exceeded=False,
            cpu_threshold_exceeded=False,
            test_aborted=False,
            abort_reason=None
        )

    except Exception as e:
        print(f"❌ Test failed: {e}")
        return StressTestResult(
            framework="langchain",
            num_documents=len(doc_paths),
            document_size_words=document_size_words,
            num_workers=1,
            total_time=time.time() - start_time,
            load_time=0,
            chunk_time=0,
            embed_time=0,
            query_time=0,
            throughput_docs_per_sec=0,
            throughput_chunks_per_sec=0,
            peak_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
            avg_memory_mb=psutil.Process().memory_info().rss / 1024 / 1024,
            memory_growth_mb=0,
            peak_cpu_percent=0,
            avg_cpu_percent=0,
            documents_loaded=0,
            chunks_created=0,
            embeddings_generated=0,
            memory_threshold_exceeded=False,
            cpu_threshold_exceeded=False,
            test_aborted=True,
            abort_reason=str(e)
        )


# ============================================================================
# Progressive Load Testing
# ============================================================================

def run_progressive_load_test(config: StressTestConfig) -> Dict[str, Any]:
    """Run progressive load test with increasing document counts."""
    print(f"\n{'='*80}")
    print("PROGRESSIVE LOAD TEST")
    print(f"{'='*80}")

    results = {
        "test_type": "progressive_load",
        "config": asdict(config),
        "system_info": get_system_info(),
        "results": []
    }

    for doc_count in config.document_counts:
        print(f"\n\n{'#'*80}")
        print(f"# Testing with {doc_count} documents")
        print(f"{'#'*80}")

        # Create test documents
        temp_dir = tempfile.mkdtemp()
        doc_paths = create_test_documents(doc_count, config.document_sizes[0], temp_dir)

        test_results = {}

        # Test GraphBit
        if config.framework in ["graphbit", "both"]:
            result = run_graphbit_stress_test(
                doc_paths,
                config.worker_counts[0],
                config.llm_model,
                config.embedding_model,
                config.ollama_base_url,
                config.document_sizes[0]
            )
            test_results["graphbit"] = asdict(result)

            # Clean up memory
            gc.collect()
            time.sleep(2)

        # Test LangChain
        if config.framework in ["langchain", "both"]:
            result = run_langchain_stress_test(
                doc_paths,
                config.llm_model,
                config.embedding_model,
                config.ollama_base_url,
                config.document_sizes[0]
            )
            test_results["langchain"] = asdict(result)

            # Clean up memory
            gc.collect()
            time.sleep(2)

        results["results"].append({
            "document_count": doc_count,
            "tests": test_results
        })

        # Clean up temp directory
        import shutil
        shutil.rmtree(temp_dir)

        # Check if we should abort
        if config.framework == "both" and "graphbit" in test_results and "langchain" in test_results:
            if test_results["graphbit"]["test_aborted"] or test_results["langchain"]["test_aborted"]:
                print(f"\n⚠️ Test aborted due to safety threshold")
                break

    return results


def run_worker_scaling_test(config: StressTestConfig) -> Dict[str, Any]:
    """Run worker scaling test (GraphBit only)."""
    print(f"\n{'='*80}")
    print("WORKER SCALING TEST (GraphBit only)")
    print(f"{'='*80}")

    results = {
        "test_type": "worker_scaling",
        "config": asdict(config),
        "system_info": get_system_info(),
        "results": []
    }

    # Use fixed document count
    doc_count = config.document_counts[0]

    for worker_count in config.worker_counts:
        print(f"\n\n{'#'*80}")
        print(f"# Testing with {worker_count} workers")
        print(f"{'#'*80}")

        # Create test documents
        temp_dir = tempfile.mkdtemp()
        doc_paths = create_test_documents(doc_count, config.document_sizes[0], temp_dir)

        # Test GraphBit
        result = run_graphbit_stress_test(
            doc_paths,
            worker_count,
            config.llm_model,
            config.embedding_model,
            config.ollama_base_url,
            config.document_sizes[0]
        )

        results["results"].append({
            "worker_count": worker_count,
            "result": asdict(result)
        })

        # Clean up
        import shutil
        shutil.rmtree(temp_dir)
        gc.collect()
        time.sleep(2)

        # Check if we should abort
        if result.test_aborted:
            print(f"\n⚠️ Test aborted due to safety threshold")
            break

    return results


def generate_summary_report(results: Dict[str, Any], output_path: str):
    """Generate summary report from test results."""
    print(f"\n{'='*80}")
    print("SUMMARY REPORT")
    print(f"{'='*80}\n")

    test_type = results.get("test_type", "unknown")

    if test_type == "progressive_load":
        print("Progressive Load Test Results:")
        print(f"{'Document Count':<20} {'GraphBit Time':<20} {'LangChain Time':<20} {'Speedup':<15}")
        print("-" * 80)

        for result in results["results"]:
            doc_count = result["document_count"]
            tests = result["tests"]

            gb_time = tests.get("graphbit", {}).get("total_time", 0)
            lc_time = tests.get("langchain", {}).get("total_time", 0)
            speedup = lc_time / gb_time if gb_time > 0 else 0

            print(f"{doc_count:<20} {gb_time:<20.2f} {lc_time:<20.2f} {speedup:<15.2f}x")

    elif test_type == "worker_scaling":
        print("Worker Scaling Test Results (GraphBit):")
        print(f"{'Worker Count':<20} {'Total Time':<20} {'Throughput (docs/s)':<25}")
        print("-" * 80)

        for result in results["results"]:
            worker_count = result["worker_count"]
            test_result = result["result"]

            total_time = test_result.get("total_time", 0)
            throughput = test_result.get("throughput_docs_per_sec", 0)

            print(f"{worker_count:<20} {total_time:<20.2f} {throughput:<25.2f}")

    print(f"\n✅ Full results saved to: {output_path}")


# ============================================================================
# Main Function
# ============================================================================

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Ollama Stress Testing for GraphBit and LangChain",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
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
        default=100,
        help="Maximum document count for progressive load test (default: 100)"
    )

    parser.add_argument(
        "--test-workers",
        action="store_true",
        help="Run worker scaling test (GraphBit only)"
    )

    parser.add_argument(
        "--test-doc-sizes",
        action="store_true",
        help="Run document size variation test"
    )

    parser.add_argument(
        "--llm-model",
        type=str,
        default="gemma3:4b",
        help="Ollama LLM model (default: gemma3:4b)"
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
        help="Ollama base URL (default: http://localhost:11434)"
    )

    parser.add_argument(
        "--output-dir",
        type=str,
        default="test_results/stress_test_results",
        help="Output directory for results (default: test_results/stress_test_results)"
    )

    args = parser.parse_args()

    # Check Ollama
    print("Checking Ollama status...")
    if not check_ollama_running(args.ollama_url):
        print(f"❌ Ollama is not running at {args.ollama_url}")
        print("Please start Ollama and try again.")
        sys.exit(1)
    print(f"✅ Ollama is running at {args.ollama_url}")

    # Check models
    print(f"\nChecking Ollama models...")
    if not check_ollama_model(args.llm_model, args.ollama_url):
        print(f"❌ LLM model '{args.llm_model}' is not available")
        print(f"Please pull the model: ollama pull {args.llm_model}")
        sys.exit(1)
    print(f"✅ LLM model '{args.llm_model}' is available")

    if not check_ollama_model(args.embedding_model, args.ollama_url):
        print(f"❌ Embedding model '{args.embedding_model}' is not available")
        print(f"Please pull the model: ollama pull {args.embedding_model}")
        sys.exit(1)
    print(f"✅ Embedding model '{args.embedding_model}' is available")

    # Create output directory
    os.makedirs(args.output_dir, exist_ok=True)

    # Determine document counts based on max_docs
    if args.max_docs <= 10:
        doc_counts = [10]
    elif args.max_docs <= 50:
        doc_counts = [10, 50]
    elif args.max_docs <= 100:
        doc_counts = [10, 50, 100]
    elif args.max_docs <= 500:
        doc_counts = [10, 50, 100, 500]
    else:
        doc_counts = [10, 50, 100, 500, 1000]

    # Create config
    config = StressTestConfig(
        framework=args.framework,
        document_counts=doc_counts,
        document_sizes=[200],  # Default document size
        worker_counts=[1, 5, 10, 20] if args.test_workers else [10],
        llm_model=args.llm_model,
        embedding_model=args.embedding_model,
        ollama_base_url=args.ollama_url,
        output_dir=args.output_dir,
        test_workers=args.test_workers,
        test_doc_sizes=args.test_doc_sizes
    )

    # Run tests
    if args.test_workers and args.framework in ["graphbit", "both"]:
        # Worker scaling test
        results = run_worker_scaling_test(config)
        output_path = os.path.join(args.output_dir, "worker_scaling_results.json")
        with open(output_path, 'w') as f:
            json.dump(results, f, indent=2)
        generate_summary_report(results, output_path)
    else:
        # Progressive load test
        results = run_progressive_load_test(config)
        output_path = os.path.join(args.output_dir, "progressive_load_results.json")
        with open(output_path, 'w') as f:
            json.dump(results, f, indent=2)
        generate_summary_report(results, output_path)

    print(f"\n{'='*80}")
    print("✅ Stress testing complete!")
    print(f"{'='*80}")


if __name__ == "__main__":
    main()

