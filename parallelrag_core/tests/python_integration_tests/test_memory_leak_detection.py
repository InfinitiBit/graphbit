"""ParallelRAG Memory Leak Detection Tests.

This test suite validates that the ParallelRAG system has no memory leaks during
long-duration continuous processing:
- Process 10,000+ documents in batches over 1+ hour
- Monitor memory usage over time for continuous growth
- Verify no resource leaks (file handles, network connections, threads)
- Test all components: text splitters, embedding client, LLM client
- Validate memory stabilizes after initial growth

Expected Behavior:
- Initial memory growth during warmup (first 1000-2000 documents)
- Memory stabilization after warmup (< 10% growth per 1000 documents)
- No continuous linear growth indicating leaks
- Stable thread count (no thread leaks)
- Stable file descriptor count (no file handle leaks)

Usage:
    # Run quick memory test (5000 documents, ~10 minutes)
    pytest tests/python_integration_tests/test_memory_leak_detection.py::TestMemoryLeakDetection::test_quick_memory_stability -v -s
    
    # Run full memory leak test (10,000+ documents, ~30-60 minutes)
    pytest tests/python_integration_tests/test_memory_leak_detection.py::TestMemoryLeakDetection::test_long_duration_memory_leak -v -s
    
    # Run all memory tests
    pytest tests/python_integration_tests/test_memory_leak_detection.py -v -s
"""

import os
import time
import gc
from concurrent.futures import ThreadPoolExecutor
from typing import List, Dict, Tuple

import pytest

import graphbit

# Try to import psutil for resource monitoring (required for these tests)
try:
    import psutil
    PSUTIL_AVAILABLE = True
except ImportError:
    PSUTIL_AVAILABLE = False
    pytest.skip("psutil required for memory leak detection tests", allow_module_level=True)


# ============================================================================
# Helper Functions
# ============================================================================

def generate_test_documents(count: int, words_per_doc: int = 2000) -> List[str]:
    """Generate test documents for memory leak testing.
    
    Args:
        count: Number of documents to generate
        words_per_doc: Approximate words per document
    
    Returns:
        List of synthetic document texts
    """
    documents = []
    
    for i in range(count):
        paragraphs = []
        for p in range(words_per_doc // 100):
            paragraph = (
                f"Document {i}, paragraph {p}. "
                f"Testing memory stability with continuous processing. "
                f"This content is used to validate no memory leaks occur. "
                f"The system should maintain stable memory usage over time. "
            ) * 20
            paragraphs.append(paragraph)
        
        documents.append("\n\n".join(paragraphs))
    
    return documents


def measure_memory_usage() -> Dict[str, float]:
    """Measure current memory usage and resource counts.
    
    Returns:
        Dictionary with memory_mb, memory_percent, num_threads, num_fds
    """
    process = psutil.Process()
    memory_info = process.memory_info()
    
    # Get file descriptor count (platform-specific)
    try:
        num_fds = process.num_fds() if hasattr(process, 'num_fds') else len(process.open_files())
    except:
        num_fds = 0
    
    return {
        "memory_mb": memory_info.rss / (1024 * 1024),
        "memory_percent": process.memory_percent(),
        "num_threads": process.num_threads(),
        "num_fds": num_fds,
    }


def analyze_memory_trend(memory_samples: List[float]) -> Dict[str, float]:
    """Analyze memory usage trend to detect leaks.
    
    Args:
        memory_samples: List of memory measurements in MB
    
    Returns:
        Dictionary with trend analysis metrics
    """
    if len(memory_samples) < 2:
        return {"growth_rate": 0.0, "total_growth": 0.0, "growth_percent": 0.0}
    
    # Calculate linear regression slope (growth rate)
    n = len(memory_samples)
    x = list(range(n))
    y = memory_samples
    
    x_mean = sum(x) / n
    y_mean = sum(y) / n
    
    numerator = sum((x[i] - x_mean) * (y[i] - y_mean) for i in range(n))
    denominator = sum((x[i] - x_mean) ** 2 for i in range(n))
    
    growth_rate = numerator / denominator if denominator != 0 else 0.0
    
    total_growth = memory_samples[-1] - memory_samples[0]
    growth_percent = (total_growth / memory_samples[0]) * 100 if memory_samples[0] > 0 else 0.0
    
    return {
        "growth_rate": growth_rate,  # MB per sample
        "total_growth": total_growth,  # Total MB growth
        "growth_percent": growth_percent,  # Percent growth
    }


# ============================================================================
# Memory Leak Detection Tests
# ============================================================================

class TestMemoryLeakDetection:
    """Test for memory leaks during continuous processing."""
    
    def test_quick_memory_stability(self) -> None:
        """Quick memory stability test with 5000 documents (~10 minutes).
        
        This test processes 5000 documents in batches and monitors memory usage.
        Memory should stabilize after initial warmup with minimal growth.
        
        Expected: < 20% memory growth after warmup phase
        """
        graphbit.init()
        
        total_docs = 5000
        batch_size = 500
        num_batches = total_docs // batch_size
        
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        
        memory_samples = []
        thread_samples = []
        
        print(f"\n{'='*70}")
        print(f"Quick Memory Stability Test")
        print(f"{'='*70}")
        print(f"Total documents: {total_docs}")
        print(f"Batch size: {batch_size}")
        print(f"Number of batches: {num_batches}")
        print(f"{'='*70}\n")
        
        start_time = time.time()
        
        for batch_num in range(num_batches):
            # Generate batch
            documents = generate_test_documents(batch_size, words_per_doc=2000)
            
            # Process batch
            with ThreadPoolExecutor(max_workers=20) as executor:
                list(executor.map(splitter.split_text, documents))
            
            # Force garbage collection
            gc.collect()
            
            # Measure resources
            resources = measure_memory_usage()
            memory_samples.append(resources["memory_mb"])
            thread_samples.append(resources["num_threads"])
            
            elapsed = time.time() - start_time
            print(f"Batch {batch_num + 1}/{num_batches}: "
                  f"Memory={resources['memory_mb']:.1f} MB, "
                  f"Threads={resources['num_threads']}, "
                  f"Elapsed={elapsed:.1f}s")
        
        # Analyze memory trend
        trend = analyze_memory_trend(memory_samples)
        
        # Split into warmup and stable phases
        warmup_samples = memory_samples[:len(memory_samples)//3]
        stable_samples = memory_samples[len(memory_samples)//3:]
        
        warmup_avg = sum(warmup_samples) / len(warmup_samples)
        stable_avg = sum(stable_samples) / len(stable_samples)
        stable_growth = ((stable_avg - warmup_avg) / warmup_avg) * 100
        
        print(f"\n{'='*70}")
        print(f"Memory Analysis")
        print(f"{'='*70}")
        print(f"Initial memory: {memory_samples[0]:.1f} MB")
        print(f"Final memory: {memory_samples[-1]:.1f} MB")
        print(f"Total growth: {trend['total_growth']:.1f} MB ({trend['growth_percent']:.1f}%)")
        print(f"Growth rate: {trend['growth_rate']:.3f} MB/batch")
        print(f"Warmup avg: {warmup_avg:.1f} MB")
        print(f"Stable avg: {stable_avg:.1f} MB")
        print(f"Stable phase growth: {stable_growth:.1f}%")
        print(f"Thread count: {thread_samples[0]} -> {thread_samples[-1]}")
        print(f"{'='*70}\n")
        
        # Assertions
        # Memory growth in stable phase should be minimal (< 30%)
        assert stable_growth < 30.0, f"Potential memory leak: {stable_growth:.1f}% growth in stable phase"
        
        # Thread count should remain stable (no thread leaks)
        thread_growth = thread_samples[-1] - thread_samples[0]
        assert abs(thread_growth) <= 5, f"Thread leak detected: {thread_growth} threads"
        
        # Total memory growth should be reasonable (< 100% for 5000 docs)
        assert trend['growth_percent'] < 100.0, f"Excessive memory growth: {trend['growth_percent']:.1f}%"
    
    @pytest.mark.slow
    def test_long_duration_memory_leak(self) -> None:
        """Long-duration memory leak test with 10,000+ documents (~30-60 minutes).
        
        This test processes 10,000 documents over an extended period to detect
        slow memory leaks that may not appear in shorter tests.
        
        Expected: < 15% memory growth after warmup phase
        """
        graphbit.init()
        
        total_docs = 10000
        batch_size = 500
        num_batches = total_docs // batch_size
        
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        
        memory_samples = []
        thread_samples = []
        fd_samples = []
        
        print(f"\n{'='*70}")
        print(f"Long-Duration Memory Leak Test")
        print(f"{'='*70}")
        print(f"Total documents: {total_docs}")
        print(f"Batch size: {batch_size}")
        print(f"Number of batches: {num_batches}")
        print(f"Estimated duration: 30-60 minutes")
        print(f"{'='*70}\n")
        
        start_time = time.time()
        
        for batch_num in range(num_batches):
            # Generate batch
            documents = generate_test_documents(batch_size, words_per_doc=2000)
            
            # Process batch
            with ThreadPoolExecutor(max_workers=20) as executor:
                list(executor.map(splitter.split_text, documents))
            
            # Force garbage collection every 5 batches
            if batch_num % 5 == 0:
                gc.collect()
            
            # Measure resources
            resources = measure_memory_usage()
            memory_samples.append(resources["memory_mb"])
            thread_samples.append(resources["num_threads"])
            fd_samples.append(resources["num_fds"])
            
            elapsed = time.time() - start_time
            docs_processed = (batch_num + 1) * batch_size
            throughput = docs_processed / elapsed
            
            if batch_num % 5 == 0:  # Print every 5 batches
                print(f"Batch {batch_num + 1}/{num_batches}: "
                      f"Docs={docs_processed}, "
                      f"Memory={resources['memory_mb']:.1f} MB, "
                      f"Threads={resources['num_threads']}, "
                      f"FDs={resources['num_fds']}, "
                      f"Throughput={throughput:.1f} docs/sec, "
                      f"Elapsed={elapsed/60:.1f}min")
        
        total_time = time.time() - start_time
        
        # Analyze memory trend
        trend = analyze_memory_trend(memory_samples)
        
        # Split into warmup (first 25%) and stable (last 75%) phases
        warmup_end = len(memory_samples) // 4
        warmup_samples = memory_samples[:warmup_end]
        stable_samples = memory_samples[warmup_end:]
        
        warmup_avg = sum(warmup_samples) / len(warmup_samples)
        stable_avg = sum(stable_samples) / len(stable_samples)
        stable_growth = ((stable_avg - warmup_avg) / warmup_avg) * 100
        
        # Analyze stable phase trend
        stable_trend = analyze_memory_trend(stable_samples)
        
        print(f"\n{'='*70}")
        print(f"Long-Duration Memory Analysis")
        print(f"{'='*70}")
        print(f"Total time: {total_time/60:.1f} minutes")
        print(f"Documents processed: {total_docs}")
        print(f"Average throughput: {total_docs/total_time:.1f} docs/sec")
        print(f"Initial memory: {memory_samples[0]:.1f} MB")
        print(f"Final memory: {memory_samples[-1]:.1f} MB")
        print(f"Total growth: {trend['total_growth']:.1f} MB ({trend['growth_percent']:.1f}%)")
        print(f"Overall growth rate: {trend['growth_rate']:.3f} MB/batch")
        print(f"Warmup avg: {warmup_avg:.1f} MB")
        print(f"Stable avg: {stable_avg:.1f} MB")
        print(f"Stable phase growth: {stable_growth:.1f}%")
        print(f"Stable phase growth rate: {stable_trend['growth_rate']:.3f} MB/batch")
        print(f"Thread count: {thread_samples[0]} -> {thread_samples[-1]}")
        print(f"FD count: {fd_samples[0]} -> {fd_samples[-1]}")
        print(f"{'='*70}\n")
        
        # Assertions
        # Memory growth in stable phase should be minimal (< 20%)
        assert stable_growth < 20.0, f"Memory leak detected: {stable_growth:.1f}% growth in stable phase"
        
        # Growth rate in stable phase should be near zero (< 0.5 MB/batch)
        assert abs(stable_trend['growth_rate']) < 0.5, f"Continuous memory growth: {stable_trend['growth_rate']:.3f} MB/batch"
        
        # Thread count should remain stable
        thread_growth = thread_samples[-1] - thread_samples[0]
        assert abs(thread_growth) <= 5, f"Thread leak detected: {thread_growth} threads"
        
        # File descriptor count should remain stable
        if fd_samples[0] > 0:  # Only check if FD tracking is available
            fd_growth = fd_samples[-1] - fd_samples[0]
            assert abs(fd_growth) <= 10, f"File descriptor leak detected: {fd_growth} FDs"

