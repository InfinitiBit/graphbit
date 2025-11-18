"""Comprehensive benchmarking for LLM completion.

This benchmark measures:
- LLM completion with OpenAI API
- Sequential vs parallel execution
- Speedup, efficiency, scalability
- Throughput (prompts/sec, tokens/sec)
- Memory usage (RSS, VMS, peak)
- Latency percentiles (P50, P95, P99)
- API rate limiting behavior
- Scalability with different worker counts (1, 2, 5, 10, 20, 50)

Usage:
    export OPENAI_API_KEY="your-key-here"
    python tests/benchmarks/benchmark_llm.py
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

def generate_benchmark_prompts(count: int = 100) -> List[str]:
    """Generate varied prompts for LLM benchmarking.
    
    Args:
        count: Number of prompts to generate
    
    Returns:
        List of prompts
    """
    prompts = []
    
    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain technology",
        "quantum computing", "robotics", "Internet of Things",
        "edge computing", "5G networks", "autonomous vehicles"
    ]
    
    prompt_templates = [
        "Explain {topic} in one sentence.",
        "What are the key benefits of {topic}?",
        "Describe a real-world application of {topic}.",
        "What are the main challenges in {topic}?",
        "How does {topic} impact modern technology?",
    ]
    
    for i in range(count):
        topic = topics[i % len(topics)]
        template = prompt_templates[i % len(prompt_templates)]
        prompts.append(template.format(topic=topic))
    
    return prompts


def estimate_token_count(text: str) -> int:
    """Estimate token count (rough approximation: 1 token ≈ 4 characters)."""
    return len(text) // 4


# ============================================================================
# Benchmark Functions
# ============================================================================

def benchmark_llm_sequential(
    llm_client,
    prompts: List[str],
    max_tokens: int = 50,
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark LLM completion in sequential mode.
    
    Args:
        llm_client: LLM client instance
        prompts: List of prompts to complete
        max_tokens: Maximum tokens per completion
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_sequential():
        responses = []
        for prompt in prompts:
            response = llm_client.complete(prompt, max_tokens=max_tokens)
            responses.append(response)
        return responses
    
    # Measure timing
    timing = measure_execution_time(run_sequential, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_sequential, len(prompts))
    
    # Calculate total tokens (input + output)
    responses = run_sequential()
    input_tokens = sum(estimate_token_count(prompt) for prompt in prompts)
    output_tokens = sum(estimate_token_count(response) for response in responses)
    total_tokens = input_tokens + output_tokens
    
    # Calculate throughput
    throughput = calculate_throughput(len(prompts), total_tokens, timing.mean)
    
    return BenchmarkResult(
        name="LLM_Sequential",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={
            "mode": "sequential",
            "prompts": len(prompts),
            "input_tokens": input_tokens,
            "output_tokens": output_tokens,
            "total_tokens": total_tokens,
            "tokens_per_sec": total_tokens / timing.mean
        }
    )


def benchmark_llm_parallel(
    llm_client,
    prompts: List[str],
    max_tokens: int = 50,
    max_workers: int = 20,
    iterations: int = 3
) -> BenchmarkResult:
    """Benchmark LLM completion in parallel mode.
    
    Args:
        llm_client: LLM client instance
        prompts: List of prompts to complete
        max_tokens: Maximum tokens per completion
        max_workers: Number of parallel workers
        iterations: Number of benchmark iterations
    
    Returns:
        BenchmarkResult with all measurements
    """
    def run_parallel():
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            responses = list(executor.map(
                lambda p: llm_client.complete(p, max_tokens=max_tokens),
                prompts
            ))
        return responses
    
    # Measure timing
    timing = measure_execution_time(run_parallel, iterations=iterations, warmup_iterations=1)
    
    # Measure memory
    memory = measure_memory_usage(run_parallel, len(prompts))
    
    # Calculate total tokens (input + output)
    responses = run_parallel()
    input_tokens = sum(estimate_token_count(prompt) for prompt in prompts)
    output_tokens = sum(estimate_token_count(response) for response in responses)
    total_tokens = input_tokens + output_tokens
    
    # Calculate throughput
    throughput = calculate_throughput(len(prompts), total_tokens, timing.mean)
    
    return BenchmarkResult(
        name=f"LLM_Parallel_{max_workers}workers",
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata={
            "mode": "parallel",
            "max_workers": max_workers,
            "prompts": len(prompts),
            "input_tokens": input_tokens,
            "output_tokens": output_tokens,
            "total_tokens": total_tokens,
            "tokens_per_sec": total_tokens / timing.mean
        }
    )


def benchmark_llm_scalability(
    llm_client,
    prompts: List[str],
    max_tokens: int = 50,
    worker_counts: List[int] = [1, 2, 5, 10, 20, 50]
) -> List[BenchmarkResult]:
    """Benchmark LLM with different worker counts to measure scalability.
    
    Args:
        llm_client: LLM client instance
        prompts: List of prompts to complete
        max_tokens: Maximum tokens per completion
        worker_counts: List of worker counts to test
    
    Returns:
        List of BenchmarkResult for each worker count
    """
    results = []
    
    for workers in worker_counts:
        result = benchmark_llm_parallel(llm_client, prompts, max_tokens, max_workers=workers, iterations=3)
        results.append(result)
    
    return results


def benchmark_llm_latency(
    llm_client,
    prompts: List[str],
    max_tokens: int = 50,
    max_workers: int = 20
) -> dict:
    """Measure per-prompt completion latency distribution.
    
    Args:
        llm_client: LLM client instance
        prompts: List of prompts to complete
        max_tokens: Maximum tokens per completion
        max_workers: Number of parallel workers
    
    Returns:
        Dictionary with latency measurements
    """
    latencies = []
    
    def complete_with_timing(prompt):
        start = time.perf_counter()
        response = llm_client.complete(prompt, max_tokens=max_tokens)
        end = time.perf_counter()
        latencies.append(end - start)
        return response
    
    # Process all prompts and collect latencies
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        list(executor.map(complete_with_timing, prompts))
    
    # Calculate percentiles
    latency_result = measure_latency_percentiles(latencies)
    
    return {
        "latency": latency_result,
        "num_samples": len(latencies)
    }


# ============================================================================
# Main Benchmark Runner
# ============================================================================

def run_all_llm_benchmarks():
    """Run comprehensive LLM benchmarks."""
    print("="*80)
    print("COMPREHENSIVE LLM BENCHMARK")
    print("="*80)
    
    # Check for API key
    if not os.environ.get("OPENAI_API_KEY"):
        print("\n❌ ERROR: OPENAI_API_KEY environment variable not set")
        print("   Please set it before running this benchmark:")
        print('   export OPENAI_API_KEY="your-key-here"')
        return None
    
    # Initialize GraphBit
    graphbit.init()
    
    # Generate test prompts (use smaller sample for cost control)
    print("\n🔄 Generating 100 benchmark prompts...")
    prompts = generate_benchmark_prompts(count=100)
    print(f"✅ Generated {len(prompts)} prompts")
    print(f"   Estimated API cost: $0.02-0.05")
    
    # System info
    print("\n📊 System Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")
    
    # Create LLM client
    print("\n🔄 Creating LLM client...")
    llm_config = graphbit.LlmConfig.openai(
        api_key=os.environ["OPENAI_API_KEY"],
        model="gpt-4o-mini"
    )
    llm_client = graphbit.LlmClient(llm_config)
    print("✅ LLM client created")
    
    # Sequential benchmark (use smaller sample)
    print(f"\n{'='*80}")
    print("Sequential Benchmark (20 prompts)")
    print(f"{'='*80}")
    sample_prompts = prompts[:20]
    seq_result = benchmark_llm_sequential(llm_client, sample_prompts, max_tokens=50, iterations=3)
    print(f"✅ Sequential: {seq_result.timing}")
    print(f"   Throughput: {seq_result.throughput}")
    print(f"   Tokens/sec: {seq_result.metadata['tokens_per_sec']:.1f}")
    print(f"   Memory: {seq_result.memory}")
    
    # Parallel benchmark (full dataset)
    print(f"\n{'='*80}")
    print("Parallel Benchmark (100 prompts, 20 workers)")
    print(f"{'='*80}")
    par_result = benchmark_llm_parallel(llm_client, prompts, max_tokens=50, max_workers=20, iterations=3)
    print(f"✅ Parallel: {par_result.timing}")
    print(f"   Throughput: {par_result.throughput}")
    print(f"   Tokens/sec: {par_result.metadata['tokens_per_sec']:.1f}")
    print(f"   Memory: {par_result.memory}")
    
    # Calculate parallelism metrics (scale sequential time)
    estimated_seq_time = seq_result.timing.mean * (len(prompts) / len(sample_prompts))
    parallelism = measure_parallelism(estimated_seq_time, par_result.timing.mean, 20)
    print(f"   Parallelism: {parallelism}")
    
    # Scalability benchmark
    print(f"\n{'='*80}")
    print("Scalability Benchmark (100 prompts, varying workers)")
    print(f"{'='*80}")
    scalability_results = benchmark_llm_scalability(llm_client, prompts, max_tokens=50, worker_counts=[1, 2, 5, 10, 20])
    print(f"✅ Scalability results:")
    for result in scalability_results:
        workers = result.metadata["max_workers"]
        speedup = estimated_seq_time / result.timing.mean
        efficiency = speedup / workers
        print(f"   {workers:3d} workers: {result.throughput.operations_per_sec:7.1f} prompts/sec, "
              f"{result.metadata['tokens_per_sec']:8.1f} tokens/sec, "
              f"Speedup={speedup:5.2f}x, Efficiency={efficiency:.2f}")
    
    # Latency benchmark
    print(f"\n{'='*80}")
    print("Latency Benchmark (100 prompts)")
    print(f"{'='*80}")
    latency_data = benchmark_llm_latency(llm_client, prompts, max_tokens=50, max_workers=20)
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
    results = run_all_llm_benchmarks()

