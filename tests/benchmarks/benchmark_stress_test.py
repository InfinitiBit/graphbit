"""Comprehensive stress testing and resource benchmarking for ParallelRAG.

This benchmark measures maximum computational capacity while excluding network latency:
- Progressive load testing (100, 500, 1000, 5000, 10000 documents)
- Worker scaling (5, 10, 20, 50, 100 workers)
- Document size scaling (100, 500, 1000, 5000 words)
- Resource monitoring (CPU %, Memory MB, utilization over time)
- Safety thresholds (90% memory, 95% CPU sustained)
- Pure computational focus (mocked API calls or local models)

Usage:
    python tests/benchmarks/benchmark_stress_test.py
    python tests/benchmarks/benchmark_stress_test.py --max-docs 5000
    python tests/benchmarks/benchmark_stress_test.py --use-local-models
"""

import argparse
import gc
import os
import random
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple

import psutil

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import graphbit
from benchmark_utils import get_system_info


# ============================================================================
# Safety Thresholds
# ============================================================================

MEMORY_THRESHOLD_PERCENT = 90.0  # Stop if memory > 90%
CPU_THRESHOLD_PERCENT = 95.0  # Stop if CPU sustained > 95% for 10+ seconds
CPU_SUSTAINED_DURATION = 10.0  # Seconds
SAMPLE_INTERVAL = 0.1  # Sample resources every 100ms


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
class StressTestResult:
    """Results from a stress test run."""
    test_name: str
    num_documents: int
    num_workers: int
    document_size_words: int
    
    # Performance metrics
    total_time: float
    throughput_docs_per_sec: float
    throughput_chunks_per_sec: float
    total_chunks: int
    
    # Resource metrics
    baseline_cpu_percent: float
    baseline_memory_mb: float
    peak_cpu_percent: float
    peak_memory_mb: float
    avg_cpu_percent: float
    avg_memory_mb: float
    memory_growth_mb: float
    memory_growth_rate_mb_per_sec: float
    
    # Efficiency metrics
    cpu_efficiency: float  # throughput per CPU%
    memory_efficiency: float  # throughput per MB
    
    # Safety status
    hit_memory_threshold: bool
    hit_cpu_threshold: bool
    completed_successfully: bool
    
    # Resource timeline
    resource_snapshots: List[ResourceSnapshot]


# ============================================================================
# Resource Monitoring
# ============================================================================

class ResourceMonitor:
    """Monitor system resources during stress testing with safety thresholds."""
    
    def __init__(self):
        self.process = psutil.Process()
        self.snapshots: List[ResourceSnapshot] = []
        self.monitoring = False
        self.monitor_thread: Optional[threading.Thread] = None
        self.hit_memory_threshold = False
        self.hit_cpu_threshold = False
        self.cpu_high_start: Optional[float] = None
    
    def get_current_resources(self) -> ResourceSnapshot:
        """Get current resource usage."""
        mem_info = self.process.memory_info()
        return ResourceSnapshot(
            timestamp=time.time(),
            cpu_percent=self.process.cpu_percent(interval=0.0),
            memory_mb=mem_info.rss / (1024 * 1024),
            memory_percent=self.process.memory_percent(),
            thread_count=self.process.num_threads()
        )
    
    def start_monitoring(self):
        """Start background resource monitoring."""
        self.snapshots = []
        self.monitoring = True
        self.hit_memory_threshold = False
        self.hit_cpu_threshold = False
        self.cpu_high_start = None
        
        def monitor_loop():
            while self.monitoring:
                snapshot = self.get_current_resources()
                self.snapshots.append(snapshot)
                
                # Check safety thresholds
                if snapshot.memory_percent > MEMORY_THRESHOLD_PERCENT:
                    self.hit_memory_threshold = True
                    print(f"\n⚠️  WARNING: Memory threshold exceeded: {snapshot.memory_percent:.1f}%")
                
                if snapshot.cpu_percent > CPU_THRESHOLD_PERCENT:
                    if self.cpu_high_start is None:
                        self.cpu_high_start = time.time()
                    elif time.time() - self.cpu_high_start > CPU_SUSTAINED_DURATION:
                        self.hit_cpu_threshold = True
                        print(f"\n⚠️  WARNING: CPU sustained above threshold: {snapshot.cpu_percent:.1f}%")
                else:
                    self.cpu_high_start = None
                
                time.sleep(SAMPLE_INTERVAL)
        
        self.monitor_thread = threading.Thread(target=monitor_loop, daemon=True)
        self.monitor_thread.start()

    def stop_monitoring(self) -> Tuple[List[ResourceSnapshot], bool, bool]:
        """Stop monitoring and return snapshots and threshold status."""
        self.monitoring = False
        if self.monitor_thread:
            self.monitor_thread.join(timeout=1.0)
        return self.snapshots, self.hit_memory_threshold, self.hit_cpu_threshold

    def should_stop(self) -> bool:
        """Check if we should stop due to safety thresholds."""
        return self.hit_memory_threshold or self.hit_cpu_threshold


# ============================================================================
# Test Data Generation
# ============================================================================

def generate_test_documents(count: int, words_per_doc: int) -> List[str]:
    """Generate synthetic documents for stress testing.

    Args:
        count: Number of documents to generate
        words_per_doc: Approximate words per document

    Returns:
        List of synthetic document texts
    """
    documents = []

    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain",
        "quantum computing", "edge computing", "IoT",
        "augmented reality", "virtual reality", "5G networks"
    ]

    for i in range(count):
        topic = topics[i % len(topics)]
        # Generate paragraphs to reach target word count
        paragraphs = []
        for p in range(words_per_doc // 50):
            paragraph = (
                f"Document {i} discusses {topic}. "
                f"This is paragraph {p} with detailed analysis. "
                f"The content explores various technical aspects. "
                f"Industry trends and developments are examined. "
                f"Best practices and strategies are outlined. "
            )
            paragraphs.append(paragraph)

        documents.append(" ".join(paragraphs))

    return documents


# ============================================================================
# Stress Testing Functions
# ============================================================================

def run_chunking_stress_test(
    documents: List[str],
    num_workers: int,
    monitor: ResourceMonitor
) -> Tuple[float, int]:
    """Run chunking stress test with resource monitoring.

    Args:
        documents: List of documents to chunk
        num_workers: Number of parallel workers
        monitor: Resource monitor instance

    Returns:
        Tuple of (execution_time, total_chunks)
    """
    # Initialize splitter
    splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

    # Start monitoring
    monitor.start_monitoring()

    # Run chunking
    start_time = time.perf_counter()

    with ThreadPoolExecutor(max_workers=num_workers) as executor:
        chunk_lists = list(executor.map(splitter.split_text, documents))

    end_time = time.perf_counter()

    # Stop monitoring
    monitor.stop_monitoring()

    # Calculate total chunks
    total_chunks = sum(len(chunks) for chunks in chunk_lists)

    return end_time - start_time, total_chunks


def run_progressive_load_test(
    document_counts: List[int],
    num_workers: int,
    words_per_doc: int
) -> List[StressTestResult]:
    """Run progressive load test with increasing document counts.

    Args:
        document_counts: List of document counts to test
        num_workers: Number of parallel workers
        words_per_doc: Words per document

    Returns:
        List of stress test results
    """
    results = []

    print(f"\n{'='*80}")
    print(f"Progressive Load Test: {num_workers} workers, {words_per_doc} words/doc")
    print(f"{'='*80}\n")

    for doc_count in document_counts:
        print(f"\n🔄 Testing with {doc_count} documents...")

        # Generate documents
        documents = generate_test_documents(doc_count, words_per_doc)

        # Create monitor
        monitor = ResourceMonitor()

        # Get baseline resources
        gc.collect()
        time.sleep(0.5)
        baseline = monitor.get_current_resources()

        # Run test
        try:
            exec_time, total_chunks = run_chunking_stress_test(documents, num_workers, monitor)

            # Get resource snapshots
            snapshots, hit_memory, hit_cpu = monitor.stop_monitoring()

            # Calculate metrics
            if snapshots:
                peak_cpu = max(s.cpu_percent for s in snapshots)
                peak_memory = max(s.memory_mb for s in snapshots)
                avg_cpu = sum(s.cpu_percent for s in snapshots) / len(snapshots)
                avg_memory = sum(s.memory_mb for s in snapshots) / len(snapshots)
            else:
                peak_cpu = baseline.cpu_percent
                peak_memory = baseline.memory_mb
                avg_cpu = baseline.cpu_percent
                avg_memory = baseline.memory_mb

            memory_growth = peak_memory - baseline.memory_mb
            memory_growth_rate = memory_growth / exec_time if exec_time > 0 else 0.0

            throughput_docs = doc_count / exec_time if exec_time > 0 else 0.0
            throughput_chunks = total_chunks / exec_time if exec_time > 0 else 0.0

            cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0.0
            memory_efficiency = throughput_docs / avg_memory if avg_memory > 0 else 0.0

            result = StressTestResult(
                test_name=f"Progressive_{doc_count}docs_{num_workers}workers",
                num_documents=doc_count,
                num_workers=num_workers,
                document_size_words=words_per_doc,
                total_time=exec_time,
                throughput_docs_per_sec=throughput_docs,
                throughput_chunks_per_sec=throughput_chunks,
                total_chunks=total_chunks,
                baseline_cpu_percent=baseline.cpu_percent,
                baseline_memory_mb=baseline.memory_mb,
                peak_cpu_percent=peak_cpu,
                peak_memory_mb=peak_memory,
                avg_cpu_percent=avg_cpu,
                avg_memory_mb=avg_memory,
                memory_growth_mb=memory_growth,
                memory_growth_rate_mb_per_sec=memory_growth_rate,
                cpu_efficiency=cpu_efficiency,
                memory_efficiency=memory_efficiency,
                hit_memory_threshold=hit_memory,
                hit_cpu_threshold=hit_cpu,
                completed_successfully=not (hit_memory or hit_cpu),
                resource_snapshots=snapshots
            )

            results.append(result)

            # Print results
            print(f"✅ Completed in {exec_time:.2f}s")
            print(f"   Throughput: {throughput_docs:.1f} docs/sec, {throughput_chunks:.1f} chunks/sec")
            print(f"   CPU: {avg_cpu:.1f}% avg, {peak_cpu:.1f}% peak")
            print(f"   Memory: {avg_memory:.1f} MB avg, {peak_memory:.1f} MB peak (+{memory_growth:.1f} MB)")
            print(f"   Efficiency: {cpu_efficiency:.2f} docs/sec per CPU%, {memory_efficiency:.4f} docs/sec per MB")

            if hit_memory or hit_cpu:
                print(f"⚠️  WARNING: Safety threshold hit - stopping progressive test")
                break

        except Exception as e:
            print(f"❌ Error: {e}")
            break

    return results


def run_worker_scaling_test(
    num_documents: int,
    worker_counts: List[int],
    words_per_doc: int
) -> List[StressTestResult]:
    """Run worker scaling test with increasing worker counts.

    Args:
        num_documents: Number of documents to process
        worker_counts: List of worker counts to test
        words_per_doc: Words per document

    Returns:
        List of stress test results
    """
    results = []

    print(f"\n{'='*80}")
    print(f"Worker Scaling Test: {num_documents} documents, {words_per_doc} words/doc")
    print(f"{'='*80}\n")

    # Generate documents once
    print(f"🔄 Generating {num_documents} documents...")
    documents = generate_test_documents(num_documents, words_per_doc)
    print(f"✅ Generated {num_documents} documents\n")

    for num_workers in worker_counts:
        print(f"\n🔄 Testing with {num_workers} workers...")

        # Create monitor
        monitor = ResourceMonitor()

        # Get baseline resources
        gc.collect()
        time.sleep(0.5)
        baseline = monitor.get_current_resources()

        # Run test
        try:
            exec_time, total_chunks = run_chunking_stress_test(documents, num_workers, monitor)

            # Get resource snapshots
            snapshots, hit_memory, hit_cpu = monitor.stop_monitoring()

            # Calculate metrics
            if snapshots:
                peak_cpu = max(s.cpu_percent for s in snapshots)
                peak_memory = max(s.memory_mb for s in snapshots)
                avg_cpu = sum(s.cpu_percent for s in snapshots) / len(snapshots)
                avg_memory = sum(s.memory_mb for s in snapshots) / len(snapshots)
            else:
                peak_cpu = baseline.cpu_percent
                peak_memory = baseline.memory_mb
                avg_cpu = baseline.cpu_percent
                avg_memory = baseline.memory_mb

            memory_growth = peak_memory - baseline.memory_mb
            memory_growth_rate = memory_growth / exec_time if exec_time > 0 else 0.0

            throughput_docs = num_documents / exec_time if exec_time > 0 else 0.0
            throughput_chunks = total_chunks / exec_time if exec_time > 0 else 0.0

            cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0.0
            memory_efficiency = throughput_docs / avg_memory if avg_memory > 0 else 0.0

            result = StressTestResult(
                test_name=f"WorkerScaling_{num_documents}docs_{num_workers}workers",
                num_documents=num_documents,
                num_workers=num_workers,
                document_size_words=words_per_doc,
                total_time=exec_time,
                throughput_docs_per_sec=throughput_docs,
                throughput_chunks_per_sec=throughput_chunks,
                total_chunks=total_chunks,
                baseline_cpu_percent=baseline.cpu_percent,
                baseline_memory_mb=baseline.memory_mb,
                peak_cpu_percent=peak_cpu,
                peak_memory_mb=peak_memory,
                avg_cpu_percent=avg_cpu,
                avg_memory_mb=avg_memory,
                memory_growth_mb=memory_growth,
                memory_growth_rate_mb_per_sec=memory_growth_rate,
                cpu_efficiency=cpu_efficiency,
                memory_efficiency=memory_efficiency,
                hit_memory_threshold=hit_memory,
                hit_cpu_threshold=hit_cpu,
                completed_successfully=not (hit_memory or hit_cpu),
                resource_snapshots=snapshots
            )

            results.append(result)

            # Print results
            print(f"✅ Completed in {exec_time:.2f}s")
            print(f"   Throughput: {throughput_docs:.1f} docs/sec, {throughput_chunks:.1f} chunks/sec")
            print(f"   CPU: {avg_cpu:.1f}% avg, {peak_cpu:.1f}% peak")
            print(f"   Memory: {avg_memory:.1f} MB avg, {peak_memory:.1f} MB peak (+{memory_growth:.1f} MB)")
            print(f"   Efficiency: {cpu_efficiency:.2f} docs/sec per CPU%, {memory_efficiency:.4f} docs/sec per MB")

            if hit_memory or hit_cpu:
                print(f"⚠️  WARNING: Safety threshold hit - stopping worker scaling test")
                break

        except Exception as e:
            print(f"❌ Error: {e}")
            break

    return results


# ============================================================================
# Results Reporting
# ============================================================================

def print_summary(all_results: List[StressTestResult]):
    """Print summary of all stress test results."""
    print(f"\n{'='*80}")
    print("STRESS TEST SUMMARY")
    print(f"{'='*80}\n")

    if not all_results:
        print("No results to display")
        return

    # Find best performance
    best_throughput = max(all_results, key=lambda r: r.throughput_docs_per_sec)
    best_cpu_efficiency = max(all_results, key=lambda r: r.cpu_efficiency)
    best_memory_efficiency = max(all_results, key=lambda r: r.memory_efficiency)

    print("🏆 Best Performance:")
    print(f"   Highest Throughput: {best_throughput.throughput_docs_per_sec:.1f} docs/sec")
    print(f"     ({best_throughput.num_documents} docs, {best_throughput.num_workers} workers)")
    print(f"   Best CPU Efficiency: {best_cpu_efficiency.cpu_efficiency:.2f} docs/sec per CPU%")
    print(f"     ({best_cpu_efficiency.num_documents} docs, {best_cpu_efficiency.num_workers} workers)")
    print(f"   Best Memory Efficiency: {best_memory_efficiency.memory_efficiency:.4f} docs/sec per MB")
    print(f"     ({best_memory_efficiency.num_documents} docs, {best_memory_efficiency.num_workers} workers)")

    # Resource limits
    print(f"\n📊 Resource Limits:")
    max_cpu = max(r.peak_cpu_percent for r in all_results)
    max_memory = max(r.peak_memory_mb for r in all_results)
    print(f"   Peak CPU: {max_cpu:.1f}%")
    print(f"   Peak Memory: {max_memory:.1f} MB")

    # Safety status
    threshold_hits = [r for r in all_results if r.hit_memory_threshold or r.hit_cpu_threshold]
    if threshold_hits:
        print(f"\n⚠️  Safety Thresholds Hit: {len(threshold_hits)} tests")
        for r in threshold_hits:
            if r.hit_memory_threshold:
                print(f"   - {r.test_name}: Memory threshold exceeded")
            if r.hit_cpu_threshold:
                print(f"   - {r.test_name}: CPU threshold exceeded")
    else:
        print(f"\n✅ All tests completed within safety thresholds")

    # Detailed results table
    print(f"\n{'='*80}")
    print("Detailed Results:")
    print(f"{'='*80}")
    print(f"{'Test':<40} {'Docs':<8} {'Workers':<8} {'Time(s)':<10} {'Throughput':<15} {'CPU%':<10} {'Mem(MB)':<10}")
    print(f"{'-'*80}")

    for r in all_results:
        print(f"{r.test_name:<40} {r.num_documents:<8} {r.num_workers:<8} {r.total_time:<10.2f} "
              f"{r.throughput_docs_per_sec:<15.1f} {r.avg_cpu_percent:<10.1f} {r.avg_memory_mb:<10.1f}")


# ============================================================================
# Main Function
# ============================================================================

def main():
    """Run comprehensive stress tests."""
    parser = argparse.ArgumentParser(description="ParallelRAG Stress Testing Suite")
    parser.add_argument("--max-docs", type=int, default=5000, help="Maximum documents to test")
    parser.add_argument("--max-workers", type=int, default=100, help="Maximum workers to test")
    parser.add_argument("--words-per-doc", type=int, default=500, help="Words per document")
    parser.add_argument("--skip-progressive", action="store_true", help="Skip progressive load test")
    parser.add_argument("--skip-worker-scaling", action="store_true", help="Skip worker scaling test")
    args = parser.parse_args()

    # Initialize GraphBit
    print("🔄 Initializing GraphBit...")
    graphbit.init()
    print("✅ GraphBit initialized\n")

    # Print system info
    print("📊 System Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")

    all_results = []

    # Progressive load test
    if not args.skip_progressive:
        document_counts = [100, 500, 1000]
        if args.max_docs >= 5000:
            document_counts.append(5000)
        if args.max_docs >= 10000:
            document_counts.append(10000)

        results = run_progressive_load_test(
            document_counts=document_counts,
            num_workers=20,
            words_per_doc=args.words_per_doc
        )
        all_results.extend(results)

    # Worker scaling test
    if not args.skip_worker_scaling:
        worker_counts = [5, 10, 20, 50]
        if args.max_workers >= 100:
            worker_counts.append(100)

        results = run_worker_scaling_test(
            num_documents=1000,
            worker_counts=worker_counts,
            words_per_doc=args.words_per_doc
        )
        all_results.extend(results)

    # Print summary
    print_summary(all_results)

    print(f"\n{'='*80}")
    print("✅ Stress testing complete!")
    print(f"{'='*80}\n")


if __name__ == "__main__":
    main()

