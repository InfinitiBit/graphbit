"""Shared utilities for accurate performance benchmarking.

This module provides utilities for measuring:
- True execution time (wall-clock time with high-resolution timers)
- True memory footprint (RSS, VMS, peak usage)
- Throughput (operations per second)
- Parallelism metrics (speedup, efficiency, scalability)
- Latency percentiles (P50, P95, P99)
- CPU utilization and thread count
"""

import gc
import os
import platform
import statistics
import time
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from typing import Any, Callable, Dict, List, Optional, Tuple

import psutil


# ============================================================================
# Data Classes for Results
# ============================================================================

@dataclass
class TimingResult:
    """Results from timing measurements."""
    mean: float  # Mean execution time (seconds)
    std_dev: float  # Standard deviation (seconds)
    min: float  # Minimum time (seconds)
    max: float  # Maximum time (seconds)
    iterations: int  # Number of iterations
    total_time: float  # Total time across all iterations (seconds)
    
    def __str__(self) -> str:
        return f"{self.mean:.4f}s ± {self.std_dev:.4f}s (min={self.min:.4f}s, max={self.max:.4f}s, n={self.iterations})"


@dataclass
class MemoryResult:
    """Results from memory measurements."""
    rss_mb: float  # Resident Set Size in MB (actual physical memory)
    vms_mb: float  # Virtual Memory Size in MB (total virtual memory)
    peak_rss_mb: float  # Peak RSS during execution
    per_operation_kb: float  # Memory per operation in KB
    
    def __str__(self) -> str:
        return f"RSS={self.rss_mb:.1f}MB, VMS={self.vms_mb:.1f}MB, Peak={self.peak_rss_mb:.1f}MB, Per-Op={self.per_operation_kb:.1f}KB"


@dataclass
class ThroughputResult:
    """Results from throughput measurements."""
    operations_per_sec: float  # Operations per second
    items_per_sec: float  # Items per second (e.g., docs, chunks, tokens)
    total_operations: int  # Total operations performed
    total_items: int  # Total items processed
    
    def __str__(self) -> str:
        return f"{self.operations_per_sec:.1f} ops/sec, {self.items_per_sec:.1f} items/sec"


@dataclass
class ParallelismResult:
    """Results from parallelism measurements."""
    speedup: float  # Sequential time / Parallel time
    efficiency: float  # Speedup / number_of_workers (ideal = 1.0)
    cpu_utilization: float  # Percentage of CPU cores used (0-100)
    thread_count: int  # Number of active threads
    
    def __str__(self) -> str:
        return f"Speedup={self.speedup:.2f}x, Efficiency={self.efficiency:.2f}, CPU={self.cpu_utilization:.1f}%, Threads={self.thread_count}"


@dataclass
class LatencyResult:
    """Results from latency measurements."""
    p50: float  # 50th percentile (median) in seconds
    p95: float  # 95th percentile in seconds
    p99: float  # 99th percentile in seconds
    mean: float  # Mean latency in seconds
    
    def __str__(self) -> str:
        return f"P50={self.p50*1000:.1f}ms, P95={self.p95*1000:.1f}ms, P99={self.p99*1000:.1f}ms, Mean={self.mean*1000:.1f}ms"


@dataclass
class BenchmarkResult:
    """Complete benchmark results."""
    name: str
    timing: TimingResult
    memory: MemoryResult
    throughput: ThroughputResult
    parallelism: Optional[ParallelismResult] = None
    latency: Optional[LatencyResult] = None
    metadata: Optional[Dict[str, Any]] = None


# ============================================================================
# Timing Utilities
# ============================================================================

def measure_execution_time(
    func: Callable,
    iterations: int = 5,
    warmup_iterations: int = 2,
    *args,
    **kwargs
) -> TimingResult:
    """Measure execution time with warmup and multiple iterations.
    
    Args:
        func: Function to benchmark
        iterations: Number of timed iterations
        warmup_iterations: Number of warmup iterations (not timed)
        *args: Arguments to pass to func
        **kwargs: Keyword arguments to pass to func
    
    Returns:
        TimingResult with mean, std_dev, min, max, iterations, total_time
    """
    # Warmup runs to eliminate cold-start effects
    for _ in range(warmup_iterations):
        func(*args, **kwargs)
    
    # Force garbage collection before timing
    gc.collect()
    
    # Timed iterations
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args, **kwargs)
        end = time.perf_counter()
        times.append(end - start)
        gc.collect()  # Clean up between iterations
    
    return TimingResult(
        mean=statistics.mean(times),
        std_dev=statistics.stdev(times) if len(times) > 1 else 0.0,
        min=min(times),
        max=max(times),
        iterations=iterations,
        total_time=sum(times)
    )


def measure_latency_percentiles(latencies: List[float]) -> LatencyResult:
    """Calculate latency percentiles from a list of latencies.
    
    Args:
        latencies: List of latency measurements in seconds
    
    Returns:
        LatencyResult with P50, P95, P99, mean
    """
    sorted_latencies = sorted(latencies)
    n = len(sorted_latencies)
    
    return LatencyResult(
        p50=sorted_latencies[int(n * 0.50)],
        p95=sorted_latencies[int(n * 0.95)],
        p99=sorted_latencies[int(n * 0.99)],
        mean=statistics.mean(latencies)
    )


# ============================================================================
# Memory Utilities
# ============================================================================

def get_memory_usage() -> Tuple[float, float]:
    """Get current memory usage in MB.
    
    Returns:
        Tuple of (RSS in MB, VMS in MB)
    """
    process = psutil.Process()
    mem_info = process.memory_info()
    rss_mb = mem_info.rss / (1024 * 1024)
    vms_mb = mem_info.vms / (1024 * 1024)
    return rss_mb, vms_mb


def measure_memory_usage(
    func: Callable,
    num_operations: int,
    *args,
    **kwargs
) -> MemoryResult:
    """Measure memory usage during function execution.
    
    Args:
        func: Function to benchmark
        num_operations: Number of operations performed (for per-operation calculation)
        *args: Arguments to pass to func
        **kwargs: Keyword arguments to pass to func
    
    Returns:
        MemoryResult with RSS, VMS, peak RSS, per-operation memory
    """
    # Force garbage collection before measurement
    gc.collect()
    
    # Get initial memory
    initial_rss, initial_vms = get_memory_usage()
    
    # Execute function and track peak memory
    peak_rss = initial_rss
    
    def track_memory():
        nonlocal peak_rss
        while True:
            rss, _ = get_memory_usage()
            peak_rss = max(peak_rss, rss)
            time.sleep(0.01)  # Sample every 10ms
    
    # Start memory tracking thread
    import threading
    tracker = threading.Thread(target=track_memory, daemon=True)
    tracker.start()
    
    # Execute function
    func(*args, **kwargs)
    
    # Force garbage collection after execution
    gc.collect()
    
    # Get final memory
    final_rss, final_vms = get_memory_usage()
    
    # Calculate per-operation memory
    memory_delta_kb = (final_rss - initial_rss) * 1024
    per_operation_kb = memory_delta_kb / num_operations if num_operations > 0 else 0.0
    
    return MemoryResult(
        rss_mb=final_rss,
        vms_mb=final_vms,
        peak_rss_mb=peak_rss,
        per_operation_kb=per_operation_kb
    )


# ============================================================================
# Throughput Utilities
# ============================================================================

def calculate_throughput(
    num_operations: int,
    num_items: int,
    execution_time: float
) -> ThroughputResult:
    """Calculate throughput metrics.
    
    Args:
        num_operations: Number of operations performed
        num_items: Number of items processed (e.g., docs, chunks, tokens)
        execution_time: Total execution time in seconds
    
    Returns:
        ThroughputResult with operations/sec and items/sec
    """
    operations_per_sec = num_operations / execution_time if execution_time > 0 else 0.0
    items_per_sec = num_items / execution_time if execution_time > 0 else 0.0
    
    return ThroughputResult(
        operations_per_sec=operations_per_sec,
        items_per_sec=items_per_sec,
        total_operations=num_operations,
        total_items=num_items
    )


# ============================================================================
# Parallelism Utilities
# ============================================================================

def measure_parallelism(
    sequential_time: float,
    parallel_time: float,
    num_workers: int
) -> ParallelismResult:
    """Calculate parallelism metrics.
    
    Args:
        sequential_time: Time for sequential execution (seconds)
        parallel_time: Time for parallel execution (seconds)
        num_workers: Number of parallel workers
    
    Returns:
        ParallelismResult with speedup, efficiency, CPU utilization, thread count
    """
    speedup = sequential_time / parallel_time if parallel_time > 0 else 0.0
    efficiency = speedup / num_workers if num_workers > 0 else 0.0
    
    # Get CPU utilization (average over 1 second)
    cpu_percent = psutil.cpu_percent(interval=0.1)
    
    # Get thread count
    process = psutil.Process()
    thread_count = process.num_threads()
    
    return ParallelismResult(
        speedup=speedup,
        efficiency=efficiency,
        cpu_utilization=cpu_percent,
        thread_count=thread_count
    )


# ============================================================================
# System Information
# ============================================================================

def get_system_info() -> Dict[str, Any]:
    """Get system information for benchmark context.
    
    Returns:
        Dictionary with system information
    """
    return {
        "platform": platform.platform(),
        "processor": platform.processor(),
        "python_version": platform.python_version(),
        "cpu_count": psutil.cpu_count(logical=False),
        "cpu_count_logical": psutil.cpu_count(logical=True),
        "total_memory_gb": psutil.virtual_memory().total / (1024**3),
        "available_memory_gb": psutil.virtual_memory().available / (1024**3),
    }


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_benchmark(
    name: str,
    func: Callable,
    num_operations: int,
    num_items: int,
    iterations: int = 5,
    warmup_iterations: int = 2,
    *args,
    **kwargs
) -> BenchmarkResult:
    """Run a complete benchmark with all measurements.
    
    Args:
        name: Name of the benchmark
        func: Function to benchmark
        num_operations: Number of operations performed
        num_items: Number of items processed
        iterations: Number of timed iterations
        warmup_iterations: Number of warmup iterations
        *args: Arguments to pass to func
        **kwargs: Keyword arguments to pass to func
    
    Returns:
        BenchmarkResult with all measurements
    """
    # Measure timing
    timing = measure_execution_time(func, iterations, warmup_iterations, *args, **kwargs)
    
    # Measure memory
    memory = measure_memory_usage(func, num_operations, *args, **kwargs)
    
    # Calculate throughput
    throughput = calculate_throughput(num_operations, num_items, timing.mean)
    
    return BenchmarkResult(
        name=name,
        timing=timing,
        memory=memory,
        throughput=throughput,
        metadata=get_system_info()
    )

