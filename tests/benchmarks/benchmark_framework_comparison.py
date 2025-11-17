"""Framework Comparison Stress Testing: GraphBit vs LangChain RAG

This benchmark compares GraphBit ParallelRAG and LangChain RAG implementations
with identical workloads to measure comparative performance.

Features:
- Identical test documents for both frameworks
- Identical configuration (chunk_size=500, chunk_overlap=50)
- Resource monitoring (CPU %, Memory MB)
- Safety thresholds (90% memory, 95% CPU sustained)
- Side-by-side performance comparison
- Speedup calculations

Usage:
    # Test both frameworks
    python tests/benchmarks/benchmark_framework_comparison.py --framework both
    
    # Test GraphBit only
    python tests/benchmarks/benchmark_framework_comparison.py --framework graphbit
    
    # Test LangChain only
    python tests/benchmarks/benchmark_framework_comparison.py --framework langchain
    
    # Custom test parameters
    python tests/benchmarks/benchmark_framework_comparison.py --max-docs 1000 --max-workers 20
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

import graphbit
from benchmark_utils import get_system_info

# Import LangChain RAG
from langchain_rag_app import LangChainRAG, LangChainRAGConfig

# Import GraphBit ParallelRAG
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "examples"))
from parallel_rag_optimized import ParallelRAG


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
    """Results from a framework stress test run."""
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
    store_time: float
    
    # Output metrics
    documents_loaded: int
    chunks_created: int
    embeddings_generated: int
    
    # Throughput metrics
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
    cpu_efficiency: float  # throughput per CPU%
    memory_efficiency: float  # throughput per MB
    
    # Safety status
    hit_memory_threshold: bool
    hit_cpu_threshold: bool
    completed_successfully: bool
    
    # Resource timeline
    resource_snapshots: List[ResourceSnapshot]


@dataclass
class ComparisonResult:
    """Comparison between GraphBit and LangChain results."""
    test_name: str
    num_documents: int
    
    graphbit_result: Optional[FrameworkTestResult]
    langchain_result: Optional[FrameworkTestResult]
    
    # Speedup calculations
    total_time_speedup: Optional[float] = None
    load_time_speedup: Optional[float] = None
    chunk_time_speedup: Optional[float] = None
    embed_time_speedup: Optional[float] = None
    throughput_speedup: Optional[float] = None
    
    # Resource comparison
    cpu_usage_ratio: Optional[float] = None
    memory_usage_ratio: Optional[float] = None
    
    def __post_init__(self):
        """Calculate speedup metrics."""
        if self.graphbit_result and self.langchain_result:
            # Time speedups (LangChain / GraphBit)
            self.total_time_speedup = self.langchain_result.total_time / self.graphbit_result.total_time
            self.load_time_speedup = self.langchain_result.load_time / self.graphbit_result.load_time if self.graphbit_result.load_time > 0 else None
            self.chunk_time_speedup = self.langchain_result.chunk_time / self.graphbit_result.chunk_time if self.graphbit_result.chunk_time > 0 else None
            self.embed_time_speedup = self.langchain_result.embed_time / self.graphbit_result.embed_time if self.graphbit_result.embed_time > 0 else None
            self.throughput_speedup = self.graphbit_result.throughput_docs_per_sec / self.langchain_result.throughput_docs_per_sec
            
            # Resource ratios (GraphBit / LangChain)
            self.cpu_usage_ratio = self.graphbit_result.avg_cpu_percent / self.langchain_result.avg_cpu_percent if self.langchain_result.avg_cpu_percent > 0 else None
            self.memory_usage_ratio = self.graphbit_result.avg_memory_mb / self.langchain_result.avg_memory_mb if self.langchain_result.avg_memory_mb > 0 else None


# ============================================================================
# Resource Monitoring (reuse from benchmark_stress_test.py)
# ============================================================================

class ResourceMonitor:
    """Monitor system resources during stress testing with safety thresholds."""

    def __init__(self):
        self.process = psutil.Process()
        self.snapshots: List[ResourceSnapshot] = []
        self.monitoring = False
        self.monitor_thread: Optional[object] = None
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
        import threading

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

                if snapshot.cpu_percent > CPU_THRESHOLD_PERCENT:
                    if self.cpu_high_start is None:
                        self.cpu_high_start = time.time()
                    elif time.time() - self.cpu_high_start > CPU_SUSTAINED_DURATION:
                        self.hit_cpu_threshold = True
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
    """Generate synthetic documents for stress testing."""
    documents = []

    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain",
        "quantum computing", "edge computing", "IoT",
        "augmented reality", "virtual reality", "5G networks"
    ]

    for i in range(count):
        topic = topics[i % len(topics)]
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


# ============================================================================
# Framework Testing Functions
# ============================================================================

def test_graphbit_rag(
    doc_paths: List[str],
    num_workers: int,
    monitor: ResourceMonitor,
    test_name: str,
    words_per_doc: int
) -> FrameworkTestResult:
    """Test GraphBit ParallelRAG implementation."""
    # Get baseline resources
    baseline = monitor.get_current_resources()

    # Initialize (no API key needed for chunking-only test)
    api_key = os.getenv("OPENAI_API_KEY", "dummy-key-for-chunking-test")
    rag = ParallelRAG(api_key, max_workers=num_workers, chunk_size=500, chunk_overlap=50)

    # Start monitoring
    monitor.start_monitoring()

    total_start = time.perf_counter()

    # Load documents
    load_start = time.perf_counter()
    documents = rag.load_documents_parallel(doc_paths)
    load_time = time.perf_counter() - load_start

    # Chunk documents
    chunk_start = time.perf_counter()
    chunks = rag.chunk_documents_parallel(documents)
    chunk_time = time.perf_counter() - chunk_start

    # For this test, we'll skip embedding and just measure chunking performance
    # to avoid API costs. Set embed_time and store_time to 0.
    embed_time = 0.0
    store_time = 0.0

    total_time = time.perf_counter() - total_start

    # Stop monitoring
    snapshots, hit_memory, hit_cpu = monitor.stop_monitoring()

    # Calculate metrics
    if snapshots:
        avg_cpu = sum(s.cpu_percent for s in snapshots) / len(snapshots)
        avg_memory = sum(s.memory_mb for s in snapshots) / len(snapshots)
        peak_cpu = max(s.cpu_percent for s in snapshots)
        peak_memory = max(s.memory_mb for s in snapshots)
    else:
        avg_cpu = baseline.cpu_percent
        avg_memory = baseline.memory_mb
        peak_cpu = baseline.cpu_percent
        peak_memory = baseline.memory_mb

    memory_growth = peak_memory - baseline.memory_mb
    throughput_docs = len(documents) / total_time if total_time > 0 else 0
    throughput_chunks = len(chunks) / total_time if total_time > 0 else 0
    cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0
    memory_efficiency = throughput_docs / avg_memory if avg_memory > 0 else 0

    return FrameworkTestResult(
        framework="graphbit",
        test_name=test_name,
        num_documents=len(documents),
        num_workers=num_workers,
        document_size_words=words_per_doc,
        total_time=total_time,
        load_time=load_time,
        chunk_time=chunk_time,
        embed_time=embed_time,
        store_time=store_time,
        documents_loaded=len(documents),
        chunks_created=len(chunks),
        embeddings_generated=0,  # Skipped for this test
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
        hit_memory_threshold=hit_memory,
        hit_cpu_threshold=hit_cpu,
        completed_successfully=not (hit_memory or hit_cpu),
        resource_snapshots=snapshots
    )


def test_langchain_rag(
    doc_paths: List[str],
    num_workers: int,
    monitor: ResourceMonitor,
    test_name: str,
    words_per_doc: int
) -> FrameworkTestResult:
    """Test LangChain RAG implementation."""
    # Get baseline resources
    baseline = monitor.get_current_resources()

    # Initialize (no API key needed for chunking-only test)
    api_key = os.getenv("OPENAI_API_KEY", "dummy-key-for-chunking-test")
    config = LangChainRAGConfig(
        openai_api_key=api_key,
        chunk_size=500,
        chunk_overlap=50,
    )
    rag = LangChainRAG(config)

    # Start monitoring
    monitor.start_monitoring()

    total_start = time.perf_counter()

    # Load documents
    load_start = time.perf_counter()
    documents = rag.load_documents(doc_paths)
    load_time = time.perf_counter() - load_start

    # Chunk documents
    chunk_start = time.perf_counter()
    chunks = rag.chunk_documents(documents)
    chunk_time = time.perf_counter() - chunk_start

    # For this test, we'll skip embedding to avoid API costs
    embed_time = 0.0
    store_time = 0.0

    total_time = time.perf_counter() - total_start

    # Stop monitoring
    snapshots, hit_memory, hit_cpu = monitor.stop_monitoring()

    # Calculate metrics
    if snapshots:
        avg_cpu = sum(s.cpu_percent for s in snapshots) / len(snapshots)
        avg_memory = sum(s.memory_mb for s in snapshots) / len(snapshots)
        peak_cpu = max(s.cpu_percent for s in snapshots)
        peak_memory = max(s.memory_mb for s in snapshots)
    else:
        avg_cpu = baseline.cpu_percent
        avg_memory = baseline.memory_mb
        peak_cpu = baseline.cpu_percent
        peak_memory = baseline.memory_mb

    memory_growth = peak_memory - baseline.memory_mb
    throughput_docs = len(documents) / total_time if total_time > 0 else 0
    throughput_chunks = len(chunks) / total_time if total_time > 0 else 0
    cpu_efficiency = throughput_docs / avg_cpu if avg_cpu > 0 else 0
    memory_efficiency = throughput_docs / avg_memory if avg_memory > 0 else 0

    return FrameworkTestResult(
        framework="langchain",
        test_name=test_name,
        num_documents=len(documents),
        num_workers=num_workers,  # LangChain doesn't use workers for loading/chunking
        document_size_words=words_per_doc,
        total_time=total_time,
        load_time=load_time,
        chunk_time=chunk_time,
        embed_time=embed_time,
        store_time=store_time,
        documents_loaded=len(documents),
        chunks_created=len(chunks),
        embeddings_generated=0,  # Skipped for this test
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
        hit_memory_threshold=hit_memory,
        hit_cpu_threshold=hit_cpu,
        completed_successfully=not (hit_memory or hit_cpu),
        resource_snapshots=snapshots
    )


# ============================================================================
# Comparison Testing
# ============================================================================

def run_comparison_test(
    num_documents: int,
    num_workers: int,
    words_per_doc: int,
    frameworks: List[str]
) -> ComparisonResult:
    """Run comparison test for specified frameworks."""
    test_name = f"Load+Chunk_{num_documents}docs_{num_workers}workers"

    print(f"\n{'='*80}")
    print(f"Test: {test_name}")
    print(f"{'='*80}")

    # Generate test documents
    print(f"📝 Generating {num_documents} documents ({words_per_doc} words each)...")
    documents = generate_test_documents(num_documents, words_per_doc)
    doc_paths = save_documents_to_files(documents)

    graphbit_result = None
    langchain_result = None

    # Test GraphBit
    if "graphbit" in frameworks:
        print(f"\n🔵 Testing GraphBit ParallelRAG...")
        monitor = ResourceMonitor()
        gc.collect()
        time.sleep(1)  # Let system stabilize

        try:
            graphbit_result = test_graphbit_rag(
                doc_paths, num_workers, monitor, test_name, words_per_doc
            )
            print(f"✅ GraphBit completed in {graphbit_result.total_time:.2f}s")
            print(f"   Load: {graphbit_result.load_time:.2f}s, Chunk: {graphbit_result.chunk_time:.2f}s")
            print(f"   Throughput: {graphbit_result.throughput_docs_per_sec:.1f} docs/sec, {graphbit_result.throughput_chunks_per_sec:.1f} chunks/sec")
            print(f"   Chunks created: {graphbit_result.chunks_created}")
        except Exception as e:
            print(f"❌ GraphBit failed: {e}")
            import traceback
            traceback.print_exc()

    # Test LangChain
    if "langchain" in frameworks:
        print(f"\n🟢 Testing LangChain RAG...")
        monitor = ResourceMonitor()
        gc.collect()
        time.sleep(1)  # Let system stabilize

        try:
            langchain_result = test_langchain_rag(
                doc_paths, num_workers, monitor, test_name, words_per_doc
            )
            print(f"✅ LangChain completed in {langchain_result.total_time:.2f}s")
            print(f"   Load: {langchain_result.load_time:.2f}s, Chunk: {langchain_result.chunk_time:.2f}s")
            print(f"   Throughput: {langchain_result.throughput_docs_per_sec:.1f} docs/sec, {langchain_result.throughput_chunks_per_sec:.1f} chunks/sec")
            print(f"   Chunks created: {langchain_result.chunks_created}")
        except Exception as e:
            print(f"❌ LangChain failed: {e}")
            import traceback
            traceback.print_exc()

    # Cleanup temp files
    import shutil
    shutil.rmtree(Path(doc_paths[0]).parent, ignore_errors=True)

    # Create comparison
    comparison = ComparisonResult(
        test_name=test_name,
        num_documents=num_documents,
        graphbit_result=graphbit_result,
        langchain_result=langchain_result
    )

    # Print comparison
    if graphbit_result and langchain_result:
        print(f"\n📊 Comparison:")
        print(f"   Total Time Speedup: {comparison.total_time_speedup:.2f}x")
        print(f"   Load Time Speedup: {comparison.load_time_speedup:.2f}x" if comparison.load_time_speedup else "   Load Time Speedup: N/A")
        print(f"   Chunk Time Speedup: {comparison.chunk_time_speedup:.2f}x" if comparison.chunk_time_speedup else "   Chunk Time Speedup: N/A")
        print(f"   Throughput Speedup: {comparison.throughput_speedup:.2f}x")

    return comparison


# ============================================================================
# Results Reporting
# ============================================================================

def print_comparison_summary(comparisons: List[ComparisonResult]):
    """Print summary of all comparison results."""
    print(f"\n{'='*80}")
    print("FRAMEWORK COMPARISON SUMMARY")
    print(f"{'='*80}\n")

    if not comparisons:
        print("No results to display")
        return

    # Filter valid comparisons (both frameworks tested)
    valid_comparisons = [c for c in comparisons if c.graphbit_result and c.langchain_result]

    if not valid_comparisons:
        print("No valid comparisons (need both frameworks tested)")
        return

    # Calculate average speedups
    avg_total_speedup = sum(c.total_time_speedup for c in valid_comparisons) / len(valid_comparisons)
    avg_throughput_speedup = sum(c.throughput_speedup for c in valid_comparisons) / len(valid_comparisons)

    print("🏆 Overall Performance:")
    print(f"   Average Total Time Speedup: {avg_total_speedup:.2f}x (GraphBit faster)")
    print(f"   Average Throughput Speedup: {avg_throughput_speedup:.2f}x (GraphBit faster)")

    # Best speedups
    best_total = max(valid_comparisons, key=lambda c: c.total_time_speedup)
    best_throughput = max(valid_comparisons, key=lambda c: c.throughput_speedup)

    print(f"\n   Best Total Time Speedup: {best_total.total_time_speedup:.2f}x")
    print(f"     ({best_total.num_documents} documents)")
    print(f"   Best Throughput Speedup: {best_throughput.throughput_speedup:.2f}x")
    print(f"     ({best_throughput.num_documents} documents)")

    # Detailed comparison table
    print(f"\n{'='*80}")
    print("Detailed Comparison:")
    print(f"{'='*80}")
    print(f"{'Test':<30} {'Docs':<8} {'GraphBit(s)':<15} {'LangChain(s)':<15} {'Speedup':<10}")
    print(f"{'-'*80}")

    for c in valid_comparisons:
        print(f"{c.test_name:<30} {c.num_documents:<8} "
              f"{c.graphbit_result.total_time:<15.2f} "
              f"{c.langchain_result.total_time:<15.2f} "
              f"{c.total_time_speedup:<10.2f}x")

    # Throughput comparison
    print(f"\n{'='*80}")
    print("Throughput Comparison:")
    print(f"{'='*80}")
    print(f"{'Test':<30} {'Docs':<8} {'GraphBit(d/s)':<15} {'LangChain(d/s)':<15} {'Speedup':<10}")
    print(f"{'-'*80}")

    for c in valid_comparisons:
        print(f"{c.test_name:<30} {c.num_documents:<8} "
              f"{c.graphbit_result.throughput_docs_per_sec:<15.1f} "
              f"{c.langchain_result.throughput_docs_per_sec:<15.1f} "
              f"{c.throughput_speedup:<10.2f}x")


def save_results_to_json(comparisons: List[ComparisonResult], output_file: str):
    """Save comparison results to JSON file."""
    results_data = []

    for c in comparisons:
        result = {
            "test_name": c.test_name,
            "num_documents": c.num_documents,
            "graphbit": asdict(c.graphbit_result) if c.graphbit_result else None,
            "langchain": asdict(c.langchain_result) if c.langchain_result else None,
            "speedups": {
                "total_time": c.total_time_speedup,
                "load_time": c.load_time_speedup,
                "chunk_time": c.chunk_time_speedup,
                "embed_time": c.embed_time_speedup,
                "throughput": c.throughput_speedup,
            } if c.graphbit_result and c.langchain_result else None
        }
        results_data.append(result)

    with open(output_file, 'w') as f:
        json.dump(results_data, f, indent=2, default=str)

    print(f"\n💾 Results saved to: {output_file}")


# ============================================================================
# Main Function
# ============================================================================

def main():
    """Run framework comparison tests."""
    parser = argparse.ArgumentParser(description="Framework Comparison Stress Testing")
    parser.add_argument("--framework", choices=["graphbit", "langchain", "both"], default="both",
                       help="Framework to test")
    parser.add_argument("--max-docs", type=int, default=1000, help="Maximum documents to test")
    parser.add_argument("--max-workers", type=int, default=20, help="Maximum workers for GraphBit")
    parser.add_argument("--words-per-doc", type=int, default=500, help="Words per document")
    parser.add_argument("--output", type=str, default="framework_comparison_results.json",
                       help="Output JSON file")
    args = parser.parse_args()

    # Determine which frameworks to test
    if args.framework == "both":
        frameworks = ["graphbit", "langchain"]
    else:
        frameworks = [args.framework]

    # Initialize GraphBit if testing it
    if "graphbit" in frameworks:
        print("🔄 Initializing GraphBit...")
        graphbit.init()
        print("✅ GraphBit initialized\n")

    # Print system info
    print("📊 System Information:")
    sys_info = get_system_info()
    for key, value in sys_info.items():
        print(f"  {key}: {value}")

    # Run comparison tests
    comparisons = []

    # Progressive load tests - dynamically generate based on max_docs
    document_counts = [100, 500, 1000]
    if args.max_docs >= 5000:
        document_counts.append(5000)
    if args.max_docs >= 10000:
        document_counts.append(10000)
    if args.max_docs >= 25000:
        document_counts.append(25000)
    if args.max_docs >= 50000:
        document_counts.append(50000)
    if args.max_docs >= 100000:
        document_counts.append(100000)
    if args.max_docs >= 250000:
        document_counts.append(250000)
    if args.max_docs >= 500000:
        document_counts.append(500000)

    for num_docs in document_counts:
        comparison = run_comparison_test(
            num_documents=num_docs,
            num_workers=args.max_workers,
            words_per_doc=args.words_per_doc,
            frameworks=frameworks
        )
        comparisons.append(comparison)

        # Check if we should stop
        if comparison.graphbit_result and comparison.graphbit_result.hit_memory_threshold:
            print(f"\n⚠️  GraphBit hit memory threshold - stopping tests")
            break
        if comparison.langchain_result and comparison.langchain_result.hit_memory_threshold:
            print(f"\n⚠️  LangChain hit memory threshold - stopping tests")
            break

    # Print summary
    print_comparison_summary(comparisons)

    # Save results
    save_results_to_json(comparisons, args.output)

    print(f"\n{'='*80}")
    print("✅ Framework comparison complete!")
    print(f"{'='*80}\n")


if __name__ == "__main__":
    main()

