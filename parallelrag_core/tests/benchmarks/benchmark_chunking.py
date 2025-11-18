"""Comprehensive benchmarking for text chunking operations.

This benchmark measures:
- All 4 text splitters (Character, Token, Sentence, Recursive)
- Sequential vs parallel execution
- Speedup, efficiency, scalability
- Throughput (docs/sec, chunks/sec)
- Memory usage (RSS, VMS, peak)
- Latency percentiles (P50, P95, P99)
- Scalability with different worker counts (1, 2, 5, 10, 20, 50, 100)

Usage:
    python tests/benchmarks/benchmark_chunking.py
"""

import random
import time
from concurrent.futures import ThreadPoolExecutor
from typing import List

import graphbit

from parallelrag_core.tests.benchmarks.benchmark_utils import (
    BenchmarkResult,
    calculate_throughput,
    get_system_info,
    measure_execution_time,
    measure_latency_percentiles,
    measure_memory_usage,
    measure_parallelism,
)


# ============================================================================
# Test Data Generation
# ============================================================================

def generate_benchmark_documents(count: int = 1000, words_per_doc: int = 2000) -> List[str]:
    """Generate realistic synthetic documents for benchmarking.
    
    Args:
        count: Number of documents to generate
        words_per_doc: Approximate words per document
    
    Returns:
        List of synthetic document texts
    """
    documents = []
    
    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain technology",
        "quantum computing", "robotics", "Internet of Things",
        "edge computing", "5G networks", "autonomous vehicles"
    ]
    
    for i in range(count):
        topic = topics[i % len(topics)]
        paragraphs = []
        
        for p in range(words_per_doc // 100):
            paragraph = (
                f"Document {i} discusses {topic} in detail. "
                f"This paragraph explores various aspects of {topic}. "
                f"The field of {topic} has seen significant advances recently. "
                f"Researchers are investigating new approaches to {topic}. "
                f"Industry applications of {topic} are expanding rapidly. "
                f"Future developments in {topic} look promising. "
                f"Challenges in {topic} include scalability and efficiency. "
                f"Best practices for {topic} are still evolving. "
            ) * 12  # Repeat to create substantial paragraphs
            paragraphs.append(paragraph)
        
        documents.append("\n\n".join(paragraphs))
    
    return documents


# ============================================================================
# Benchmark Functions
# ============================================================================

def benchmark_splitter_sequential(
    splitter,
    documents: List[str],
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark a splitter in sequential mode.
    
    Args:
        splitter: Text splitter instance
        documents: List of documents to process
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_sequential():
        results = []
        for doc in documents:
            chunks = splitter.split_text(doc)
            results.append(chunks)
        return results
    
    # Measure timing
    timing = measure_execution_time(run_sequential, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_sequential, len(documents))
    
    # Get total chunks for throughput calculation
    results = run_sequential()
    total_chunks = sum(len(chunks) for chunks in results)
    
    # Calculate throughput
    throughput = calculate_throughput(len(documents), total_chunks, timing.mean)
    
    return BenchmarkResult(
        name=f"{splitter.__class__.__name__}_Sequential",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={"mode": "sequential", "documents": len(documents), "chunks": total_chunks}
    )


def benchmark_splitter_parallel(
    splitter,
    documents: List[str],
    max_workers: int = 50,
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark a splitter in parallel mode.
    
    Args:
        splitter: Text splitter instance
        documents: List of documents to process
        max_workers: Number of parallel workers
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_parallel():
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            results = list(executor.map(splitter.split_text, documents))
        return results
    
    # Measure timing
    timing = measure_execution_time(run_parallel, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_parallel, len(documents))
    
    # Get total chunks for throughput calculation
    results = run_parallel()
    total_chunks = sum(len(chunks) for chunks in results)
    
    # Calculate throughput
    throughput = calculate_throughput(len(documents), total_chunks, timing.mean)
    
    return BenchmarkResult(
        name=f"{splitter.__class__.__name__}_Parallel_{max_workers}workers",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={
            "mode": "parallel",
            "max_workers": max_workers,
            "documents": len(documents),
            "chunks": total_chunks
        }
    )


def benchmark_splitter_scalability(
    splitter,
    documents: List[str],
    worker_counts: List[int] = [1, 2, 5, 10, 20, 50, 100]
) -> List[BenchmarkResult]:
    """Benchmark a splitter with different worker counts to measure scalability.
    
    Args:
        splitter: Text splitter instance
        documents: List of documents to process
        worker_counts: List of worker counts to test
    
    Returns:
        List of BenchmarkResult for each worker count
    """
    results = []
    
    for workers in worker_counts:
        result = benchmark_splitter_parallel(splitter, documents, max_workers=workers, iterations=3)
        results.append(result)
    
    return results


def benchmark_splitter_latency(
    splitter,
    documents: List[str],
    max_workers: int = 50
) -> dict:
    """Measure per-document latency distribution.
    
    Args:
        splitter: Text splitter instance
        documents: List of documents to process
        max_workers: Number of parallel workers
    
    Returns:
        Dictionary with latency measurements
    """
    latencies = []
    
    def process_with_timing(doc):
        start = time.perf_counter()
        chunks = splitter.split_text(doc)
        end = time.perf_counter()
        latencies.append(end - start)
        return chunks
    
    # Process all documents and collect latencies
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        list(executor.map(process_with_timing, documents))
    
    # Calculate percentiles
    latency_result = measure_latency_percentiles(latencies)
    
    return {
        "latency": latency_result,
        "num_samples": len(latencies)
    }


# ============================================================================
# Main Benchmark Runner
# ============================================================================

def run_all_chunking_benchmarks():
    """Run comprehensive chunking benchmarks for all splitters."""
    print("="*80)
    print("COMPREHENSIVE CHUNKING BENCHMARK")
    print("="*80)
    
    # Initialize GraphBit
    graphbit.init()
    
    # Generate test documents
    print("\n🔄 Generating 1000 benchmark documents...")
    documents = generate_benchmark_documents(count=1000, words_per_doc=2000)
    print(f"✅ Generated {len(documents)} documents")
    
    # System info
    print("\n📊 System Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")
    
    # Define splitters
    splitters = {
        "CharacterSplitter": graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50),
        "TokenSplitter": graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20),
        "SentenceSplitter": graphbit.SentenceSplitter(chunk_size=500, chunk_overlap=50),
        "RecursiveSplitter": graphbit.RecursiveSplitter(chunk_size=500, chunk_overlap=50),
    }
    
    all_results = {}
    
    for name, splitter in splitters.items():
        print(f"\n{'='*80}")
        print(f"Benchmarking: {name}")
        print(f"{'='*80}")
        
        # Sequential benchmark
        print(f"\n🔄 Running sequential benchmark...")
        seq_result = benchmark_splitter_sequential(splitter, documents, iterations=3)
        print(f"✅ Sequential: {seq_result.timing}")
        print(f"   Throughput: {seq_result.throughput}")
        print(f"   Memory: {seq_result.memory}")
        
        # Parallel benchmark (50 workers)
        print(f"\n🔄 Running parallel benchmark (50 workers)...")
        par_result = benchmark_splitter_parallel(splitter, documents, max_workers=50, iterations=3)
        print(f"✅ Parallel: {par_result.timing}")
        print(f"   Throughput: {par_result.throughput}")
        print(f"   Memory: {par_result.memory}")
        
        # Calculate parallelism metrics
        parallelism = measure_parallelism(seq_result.timing.mean, par_result.timing.mean, 50)
        print(f"   Parallelism: {parallelism}")
        
        # Scalability benchmark
        print(f"\n🔄 Running scalability benchmark (1, 2, 5, 10, 20, 50, 100 workers)...")
        scalability_results = benchmark_splitter_scalability(splitter, documents, [1, 2, 5, 10, 20, 50, 100])
        print(f"✅ Scalability results:")
        for result in scalability_results:
            workers = result.metadata["max_workers"]
            speedup = seq_result.timing.mean / result.timing.mean
            efficiency = speedup / workers
            print(f"   {workers:3d} workers: {result.throughput.operations_per_sec:7.1f} docs/sec, "
                  f"Speedup={speedup:5.2f}x, Efficiency={efficiency:.2f}")
        
        # Latency benchmark
        print(f"\n🔄 Running latency benchmark...")
        latency_data = benchmark_splitter_latency(splitter, documents, max_workers=50)
        print(f"✅ Latency: {latency_data['latency']}")
        
        # Store results
        all_results[name] = {
            "sequential": seq_result,
            "parallel": par_result,
            "parallelism": parallelism,
            "scalability": scalability_results,
            "latency": latency_data
        }
    
    print(f"\n{'='*80}")
    print("BENCHMARK COMPLETE")
    print(f"{'='*80}")
    
    return all_results


if __name__ == "__main__":
    results = run_all_chunking_benchmarks()

