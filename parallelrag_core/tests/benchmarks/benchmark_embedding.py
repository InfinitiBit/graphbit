"""Comprehensive benchmarking for embedding generation.

This benchmark measures:
- Embedding generation with OpenAI API
- Sequential vs parallel execution
- Speedup, efficiency, scalability
- Throughput (chunks/sec, tokens/sec)
- Memory usage (RSS, VMS, peak)
- Latency percentiles (P50, P95, P99)
- API rate limiting behavior
- Scalability with different worker counts (1, 2, 5, 10, 20, 50, 100)

Usage:
    export OPENAI_API_KEY="your-key-here"
    python tests/benchmarks/benchmark_embedding.py
"""

import os
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

def generate_benchmark_chunks(count: int = 1000, words_per_chunk: int = 100) -> List[str]:
    """Generate realistic text chunks for embedding benchmarking.
    
    Args:
        count: Number of chunks to generate
        words_per_chunk: Approximate words per chunk
    
    Returns:
        List of text chunks
    """
    chunks = []
    
    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain technology",
        "quantum computing", "robotics", "Internet of Things",
        "edge computing", "5G networks", "autonomous vehicles"
    ]
    
    for i in range(count):
        topic = topics[i % len(topics)]
        chunk = (
            f"Chunk {i} discusses {topic}. "
            f"This text explores various aspects of {topic}. "
            f"The field of {topic} has seen significant advances. "
            f"Researchers are investigating new approaches to {topic}. "
            f"Industry applications of {topic} are expanding rapidly. "
        ) * (words_per_chunk // 25)  # Repeat to reach target word count
        chunks.append(chunk)
    
    return chunks


def estimate_token_count(text: str) -> int:
    """Estimate token count (rough approximation: 1 token ≈ 4 characters)."""
    return len(text) // 4


# ============================================================================
# Benchmark Functions
# ============================================================================

def benchmark_embedding_sequential(
    embed_client,
    chunks: List[str],
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark embedding generation in sequential mode.
    
    Args:
        embed_client: Embedding client instance
        chunks: List of text chunks to embed
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_sequential():
        embeddings = []
        for chunk in chunks:
            emb = embed_client.embed(chunk)
            embeddings.append(emb)
        return embeddings
    
    # Measure timing
    timing = measure_execution_time(run_sequential, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_sequential, len(chunks))
    
    # Calculate total tokens
    total_tokens = sum(estimate_token_count(chunk) for chunk in chunks)
    
    # Calculate throughput
    throughput = calculate_throughput(len(chunks), total_tokens, timing.mean)
    
    return BenchmarkResult(
        name="Embedding_Sequential",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={
            "mode": "sequential",
            "chunks": len(chunks),
            "tokens": total_tokens,
            "tokens_per_sec": total_tokens / timing.mean
        }
    )


def benchmark_embedding_parallel(
    embed_client,
    chunks: List[str],
    max_workers: int = 50,
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark embedding generation in parallel mode.
    
    Args:
        embed_client: Embedding client instance
        chunks: List of text chunks to embed
        max_workers: Number of parallel workers
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_parallel():
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            embeddings = list(executor.map(embed_client.embed, chunks))
        return embeddings
    
    # Measure timing
    timing = measure_execution_time(run_parallel, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_parallel, len(chunks))
    
    # Calculate total tokens
    total_tokens = sum(estimate_token_count(chunk) for chunk in chunks)
    
    # Calculate throughput
    throughput = calculate_throughput(len(chunks), total_tokens, timing.mean)
    
    return BenchmarkResult(
        name=f"Embedding_Parallel_{max_workers}workers",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={
            "mode": "parallel",
            "max_workers": max_workers,
            "chunks": len(chunks),
            "tokens": total_tokens,
            "tokens_per_sec": total_tokens / timing.mean
        }
    )


def benchmark_embedding_scalability(
    embed_client,
    chunks: List[str],
    worker_counts: List[int] = [1, 2, 5, 10, 20, 50, 100]
) -> List[BenchmarkResult]:
    """Benchmark embedding with different worker counts to measure scalability.
    
    Args:
        embed_client: Embedding client instance
        chunks: List of text chunks to embed
        worker_counts: List of worker counts to test
    
    Returns:
        List of BenchmarkResult for each worker count
    """
    results = []
    
    for workers in worker_counts:
        result = benchmark_embedding_parallel(embed_client, chunks, max_workers=workers, iterations=3)
        results.append(result)
    
    return results


def benchmark_embedding_latency(
    embed_client,
    chunks: List[str],
    max_workers: int = 50
) -> dict:
    """Measure per-chunk embedding latency distribution.
    
    Args:
        embed_client: Embedding client instance
        chunks: List of text chunks to embed
        max_workers: Number of parallel workers
    
    Returns:
        Dictionary with latency measurements
    """
    latencies = []
    
    def embed_with_timing(chunk):
        start = time.perf_counter()
        emb = embed_client.embed(chunk)
        end = time.perf_counter()
        latencies.append(end - start)
        return emb
    
    # Process all chunks and collect latencies
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        list(executor.map(embed_with_timing, chunks))
    
    # Calculate percentiles
    latency_result = measure_latency_percentiles(latencies)
    
    return {
        "latency": latency_result,
        "num_samples": len(latencies)
    }


# ============================================================================
# Main Benchmark Runner
# ============================================================================

def run_all_embedding_benchmarks():
    """Run comprehensive embedding benchmarks."""
    print("="*80)
    print("COMPREHENSIVE EMBEDDING BENCHMARK")
    print("="*80)
    
    # Check for API key
    if not os.environ.get("OPENAI_API_KEY"):
        print("\n❌ ERROR: OPENAI_API_KEY environment variable not set")
        print("   Please set it before running this benchmark:")
        print('   export OPENAI_API_KEY="your-key-here"')
        return None
    
    # Initialize GraphBit
    graphbit.init()
    
    # Generate test chunks (use smaller sample for cost control)
    print("\n🔄 Generating 200 benchmark chunks...")
    chunks = generate_benchmark_chunks(count=200, words_per_chunk=100)
    total_tokens = sum(estimate_token_count(chunk) for chunk in chunks)
    print(f"✅ Generated {len(chunks)} chunks (~{total_tokens:,} tokens)")
    print(f"   Estimated API cost: ${total_tokens * 0.00002 / 1000:.4f}")
    
    # System info
    print("\n📊 System Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")
    
    # Create embedding client
    print("\n🔄 Creating embedding client...")
    embed_config = graphbit.EmbeddingConfig.openai(
        api_key=os.environ["OPENAI_API_KEY"],
        model="text-embedding-3-small"
    )
    embed_client = graphbit.EmbeddingClient(embed_config)
    print("✅ Embedding client created")
    
    # Sequential benchmark (use smaller sample)
    print(f"\n{'='*80}")
    print("Sequential Benchmark (50 chunks)")
    print(f"{'='*80}")
    sample_chunks = chunks[:50]
    seq_result = benchmark_embedding_sequential(embed_client, sample_chunks, iterations=3)
    print(f"✅ Sequential: {seq_result.timing}")
    print(f"   Throughput: {seq_result.throughput}")
    print(f"   Tokens/sec: {seq_result.metadata['tokens_per_sec']:.1f}")
    print(f"   Memory: {seq_result.memory}")
    
    # Parallel benchmark (full dataset)
    print(f"\n{'='*80}")
    print("Parallel Benchmark (200 chunks, 20 workers)")
    print(f"{'='*80}")
    par_result = benchmark_embedding_parallel(embed_client, chunks, max_workers=20, iterations=3)
    print(f"✅ Parallel: {par_result.timing}")
    print(f"   Throughput: {par_result.throughput}")
    print(f"   Tokens/sec: {par_result.metadata['tokens_per_sec']:.1f}")
    print(f"   Memory: {par_result.memory}")
    
    # Calculate parallelism metrics (scale sequential time)
    estimated_seq_time = seq_result.timing.mean * (len(chunks) / len(sample_chunks))
    parallelism = measure_parallelism(estimated_seq_time, par_result.timing.mean, 20)
    print(f"   Parallelism: {parallelism}")

    # Scalability benchmark (use smaller sample for cost)
    print(f"\n{'='*80}")
    print("Scalability Benchmark (100 chunks, varying workers)")
    print(f"{'='*80}")
    scalability_chunks = chunks[:100]
    scalability_results = benchmark_embedding_scalability(
        embed_client, scalability_chunks, [1, 2, 5, 10, 20]
    )
    print(f"✅ Scalability results:")
    for result in scalability_results:
        workers = result.metadata["max_workers"]
        speedup = estimated_seq_time * (len(scalability_chunks) / len(chunks)) / result.timing.mean
        efficiency = speedup / workers
        print(f"   {workers:3d} workers: {result.throughput.operations_per_sec:7.1f} chunks/sec, "
              f"{result.metadata['tokens_per_sec']:8.1f} tokens/sec, "
              f"Speedup={speedup:5.2f}x, Efficiency={efficiency:.2f}")
    
    # Latency benchmark
    print(f"\n{'='*80}")
    print("Latency Benchmark (100 chunks)")
    print(f"{'='*80}")
    latency_data = benchmark_embedding_latency(embed_client, scalability_chunks, max_workers=20)
    print(f"✅ Latency: {latency_data['latency']}")
    
    print(f"\n{'='*80}")
    print("BENCHMARK COMPLETE")
    print(f"{'='*80}")
    
    return {
        "sequential": seq_result,
        "parallel": par_result,
        "parallelism": parallelism,
        "scalability": scalability_results,
        "latency": latency_data
    }


if __name__ == "__main__":
    results = run_all_embedding_benchmarks()

