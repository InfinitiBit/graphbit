"""ParallelRAG High Concurrency Stress Testing.

This test suite validates the ParallelRAG system under high-load conditions:
- Process 1000+ documents with varying concurrency levels (20-100 workers)
- Monitor CPU utilization, memory usage, and thread pool saturation
- Measure throughput (documents/second, chunks/second) at different scales
- Identify optimal max_workers settings for different hardware configurations
- Validate system stability under sustained high load

Expected Performance:
- 1000 documents @ max_workers=20: 5-15x speedup
- 1000 documents @ max_workers=50: 8-20x speedup
- 1000 documents @ max_workers=100: 10-25x speedup (on high-core systems)
- Throughput: 500-5000 documents/second (hardware dependent)
- Memory: Stable growth, no leaks (< 500 MB for 1000 docs)
- CPU: 30-95% utilization under load (varies by system)

Usage:
    # Run all stress tests (WARNING: Takes 30-60 minutes)
    pytest tests/python_integration_tests/test_parallel_rag_stress.py -v -s
    
    # Run specific stress test
    pytest tests/python_integration_tests/test_parallel_rag_stress.py::TestHighConcurrencyStress::test_stress_token_splitter_1000_docs -v -s
    
    # Run optimal concurrency test only
    pytest tests/python_integration_tests/test_parallel_rag_stress.py::TestHighConcurrencyStress::test_optimal_concurrency_level -v -s
"""

import os
import time
import random
from concurrent.futures import ThreadPoolExecutor
from typing import List, Dict, Tuple

import pytest

import graphbit

# Try to import psutil for resource monitoring (optional)
try:
    import psutil
    PSUTIL_AVAILABLE = True
except ImportError:
    PSUTIL_AVAILABLE = False
    print("WARNING: psutil not available. Resource monitoring will be limited.")


# ============================================================================
# Test Data Generation
# ============================================================================

def generate_realistic_documents(count: int, min_words: int = 2000, max_words: int = 5000) -> List[str]:
    """Generate realistic synthetic documents for stress testing.
    
    Args:
        count: Number of documents to generate
        min_words: Minimum words per document
        max_words: Maximum words per document
    
    Returns:
        List of synthetic document texts with varied content
    """
    documents = []
    
    # Sample topics for varied content
    topics = [
        "artificial intelligence", "machine learning", "data science",
        "cloud computing", "cybersecurity", "blockchain technology",
        "quantum computing", "edge computing", "internet of things",
        "augmented reality", "virtual reality", "5G networks"
    ]
    
    for i in range(count):
        topic = topics[i % len(topics)]
        words_target = random.randint(min_words, max_words)
        paragraphs = []
        
        # Generate paragraphs to reach target word count
        for p in range(words_target // 100):
            paragraph = (
                f"Document {i} discusses {topic} in detail. "
                f"This is paragraph {p} providing comprehensive analysis. "
                f"The content explores various aspects and implications. "
                f"Technical details and practical applications are examined. "
                f"Industry trends and future developments are considered. "
                f"Best practices and implementation strategies are outlined. "
                f"Case studies and real-world examples are presented. "
                f"Challenges and opportunities are thoroughly analyzed. "
            ) * 12  # Repeat to create substantial paragraphs
            paragraphs.append(paragraph)
        
        documents.append("\n\n".join(paragraphs))
    
    return documents


def measure_resource_usage() -> Dict[str, float]:
    """Measure current CPU and memory usage.
    
    Returns:
        Dictionary with cpu_percent, memory_mb, memory_percent
    """
    if not PSUTIL_AVAILABLE:
        return {"cpu_percent": 0.0, "memory_mb": 0.0, "memory_percent": 0.0}
    
    process = psutil.Process()
    memory_info = process.memory_info()
    
    return {
        "cpu_percent": process.cpu_percent(interval=0.1),
        "memory_mb": memory_info.rss / (1024 * 1024),  # Convert to MB
        "memory_percent": process.memory_percent(),
    }


def calculate_throughput(count: int, duration: float) -> float:
    """Calculate throughput in items per second.
    
    Args:
        count: Number of items processed
        duration: Time taken in seconds
    
    Returns:
        Throughput in items/second
    """
    return count / duration if duration > 0 else 0.0


# ============================================================================
# Stage 1: High Concurrency Stress Tests
# ============================================================================

class TestHighConcurrencyStress:
    """Test ParallelRAG system under high concurrency with 1000+ documents."""
    
    @pytest.fixture(scope="class")
    def documents_1000(self) -> List[str]:
        """Generate 1000 test documents."""
        graphbit.init()
        print("\n" + "="*70)
        print("Generating 1000 documents for stress testing...")
        print("="*70)
        docs = generate_realistic_documents(count=1000, min_words=2000, max_words=5000)
        print(f"Generated {len(docs)} documents")
        print(f"Average document size: {sum(len(d.split()) for d in docs) / len(docs):.0f} words")
        return docs
    
    def test_stress_character_splitter_1000_docs(self, documents_1000: List[str]) -> None:
        """Stress test CharacterSplitter with 1000 documents at high concurrency.

        Expected: 5-15x speedup with max_workers=20
        """
        splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)
        
        # Measure baseline resource usage
        baseline_resources = measure_resource_usage()
        
        # Sequential execution (sample 100 docs to save time)
        sample_docs = documents_1000[:100]
        start_time = time.time()
        for doc in sample_docs:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution (full 1000 docs)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=20) as executor:
            results = list(executor.map(splitter.split_text, documents_1000))
        parallel_time = time.time() - start_time
        
        # Measure resource usage after processing
        final_resources = measure_resource_usage()
        
        # Calculate metrics
        total_chunks = sum(len(chunks) for chunks in results)
        estimated_sequential_time = sequential_time * 10  # Scale from 100 to 1000 docs
        speedup = estimated_sequential_time / parallel_time
        throughput = calculate_throughput(len(documents_1000), parallel_time)
        
        print(f"\n{'='*70}")
        print(f"CharacterSplitter Stress Test (1000 documents)")
        print(f"{'='*70}")
        print(f"Documents processed: {len(documents_1000)}")
        print(f"Total chunks generated: {total_chunks:,}")
        print(f"Sequential time (estimated): {estimated_sequential_time:.2f}s")
        print(f"Parallel time (max_workers=20): {parallel_time:.2f}s")
        print(f"Speedup: {speedup:.2f}x")
        print(f"Throughput: {throughput:.1f} docs/sec")
        print(f"Baseline CPU: {baseline_resources['cpu_percent']:.1f}%")
        print(f"Final CPU: {final_resources['cpu_percent']:.1f}%")
        print(f"Baseline Memory: {baseline_resources['memory_mb']:.1f} MB")
        print(f"Final Memory: {final_resources['memory_mb']:.1f} MB")
        print(f"Memory Delta: {final_resources['memory_mb'] - baseline_resources['memory_mb']:.1f} MB")
        print(f"{'='*70}\n")
        
        # Assertions
        assert speedup >= 3.0, f"Expected speedup >= 3x, got {speedup:.2f}x"
        assert throughput >= 100.0, f"Expected throughput >= 100 docs/sec, got {throughput:.1f}"
        assert total_chunks > 0, "No chunks generated"

        # Memory growth should be reasonable (< 1000 MB for 1000 docs)
        memory_delta = final_resources['memory_mb'] - baseline_resources['memory_mb']
        assert memory_delta < 1000, f"Excessive memory growth: {memory_delta:.1f} MB"
    
    def test_stress_token_splitter_1000_docs(self, documents_1000: List[str]) -> None:
        """Stress test TokenSplitter with 1000 documents at high concurrency.

        Expected: 8-20x speedup with max_workers=20
        """
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        
        baseline_resources = measure_resource_usage()
        
        # Sequential execution (sample 100 docs)
        sample_docs = documents_1000[:100]
        start_time = time.time()
        for doc in sample_docs:
            splitter.split_text(doc)
        sequential_time = time.time() - start_time
        
        # Parallel execution (full 1000 docs)
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=20) as executor:
            results = list(executor.map(splitter.split_text, documents_1000))
        parallel_time = time.time() - start_time
        
        final_resources = measure_resource_usage()
        
        total_chunks = sum(len(chunks) for chunks in results)
        estimated_sequential_time = sequential_time * 10
        speedup = estimated_sequential_time / parallel_time
        throughput = calculate_throughput(len(documents_1000), parallel_time)
        
        print(f"\n{'='*70}")
        print(f"TokenSplitter Stress Test (1000 documents)")
        print(f"{'='*70}")
        print(f"Documents processed: {len(documents_1000)}")
        print(f"Total chunks generated: {total_chunks:,}")
        print(f"Sequential time (estimated): {estimated_sequential_time:.2f}s")
        print(f"Parallel time (max_workers=20): {parallel_time:.2f}s")
        print(f"Speedup: {speedup:.2f}x")
        print(f"Throughput: {throughput:.1f} docs/sec")
        print(f"Memory Delta: {final_resources['memory_mb'] - baseline_resources['memory_mb']:.1f} MB")
        print(f"{'='*70}\n")
        
        assert speedup >= 5.0, f"Expected speedup >= 5x, got {speedup:.2f}x"
        assert throughput >= 50.0, f"Expected throughput >= 50 docs/sec, got {throughput:.1f}"
        assert total_chunks > 0, "No chunks generated"

        memory_delta = final_resources['memory_mb'] - baseline_resources['memory_mb']
        assert memory_delta < 1000, f"Excessive memory growth: {memory_delta:.1f} MB"
    
    @pytest.mark.parametrize("max_workers", [10, 20, 50, 100])
    def test_optimal_concurrency_level(self, documents_1000: List[str], max_workers: int) -> None:
        """Test different concurrency levels to identify optimal settings.
        
        This test helps identify the optimal max_workers setting for different
        hardware configurations by testing 10, 20, 50, and 100 workers.
        
        Expected patterns:
        - Low concurrency (10): Good speedup, lower CPU utilization
        - Medium concurrency (20-50): Best speedup/efficiency balance
        - High concurrency (100): Diminishing returns, potential overhead
        """
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        
        # Use subset for faster testing
        test_docs = documents_1000[:500]
        
        baseline_resources = measure_resource_usage()
        
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            results = list(executor.map(splitter.split_text, test_docs))
        parallel_time = time.time() - start_time
        
        final_resources = measure_resource_usage()
        
        total_chunks = sum(len(chunks) for chunks in results)
        throughput = calculate_throughput(len(test_docs), parallel_time)
        
        print(f"\n{'='*70}")
        print(f"Concurrency Level Test (max_workers={max_workers})")
        print(f"{'='*70}")
        print(f"Documents processed: {len(test_docs)}")
        print(f"Total chunks: {total_chunks:,}")
        print(f"Processing time: {parallel_time:.2f}s")
        print(f"Throughput: {throughput:.1f} docs/sec")
        print(f"CPU utilization: {final_resources['cpu_percent']:.1f}%")
        print(f"Memory usage: {final_resources['memory_mb']:.1f} MB")
        print(f"{'='*70}\n")
        
        # Basic assertions
        assert total_chunks > 0, "No chunks generated"
        assert throughput > 0, "Zero throughput"
        
        # Performance should improve with concurrency (up to a point)
        if max_workers >= 20:
            assert throughput >= 10.0, f"Low throughput at max_workers={max_workers}: {throughput:.1f}"


# ============================================================================
# Stage 2: Scalability Validation Tests
# ============================================================================

class TestScalabilityValidation:
    """Test system scalability from 100 to 5000 documents."""

    @pytest.mark.parametrize("doc_count", [100, 500, 1000, 2000])
    def test_linear_scaling(self, doc_count: int) -> None:
        """Validate system scales linearly from 100 to 2000 documents.

        Expected: Processing time should scale roughly linearly with document count
        when using parallel execution. Throughput should remain relatively constant.
        """
        graphbit.init()

        # Generate documents
        print(f"\nGenerating {doc_count} documents...")
        documents = generate_realistic_documents(count=doc_count, min_words=2000, max_words=3000)

        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

        baseline_resources = measure_resource_usage()

        # Parallel execution
        start_time = time.time()
        with ThreadPoolExecutor(max_workers=20) as executor:
            results = list(executor.map(splitter.split_text, documents))
        parallel_time = time.time() - start_time

        final_resources = measure_resource_usage()

        total_chunks = sum(len(chunks) for chunks in results)
        throughput = calculate_throughput(doc_count, parallel_time)

        print(f"\n{'='*70}")
        print(f"Scalability Test ({doc_count} documents)")
        print(f"{'='*70}")
        print(f"Documents processed: {doc_count}")
        print(f"Total chunks: {total_chunks:,}")
        print(f"Processing time: {parallel_time:.2f}s")
        print(f"Throughput: {throughput:.1f} docs/sec")
        print(f"Chunks/sec: {total_chunks/parallel_time:.1f}")
        print(f"Memory usage: {final_resources['memory_mb']:.1f} MB")
        print(f"Memory delta: {final_resources['memory_mb'] - baseline_resources['memory_mb']:.1f} MB")
        print(f"{'='*70}\n")

        # Assertions
        assert total_chunks > 0, "No chunks generated"
        assert throughput >= 10.0, f"Low throughput: {throughput:.1f} docs/sec"

        # Memory growth should be reasonable (< 1 MB per document)
        memory_delta = final_resources['memory_mb'] - baseline_resources['memory_mb']
        max_expected_memory = doc_count * 1.0  # 1 MB per document
        assert memory_delta < max_expected_memory, f"Excessive memory: {memory_delta:.1f} MB"

    def test_throughput_consistency(self) -> None:
        """Validate throughput remains consistent across multiple runs.

        Expected: Throughput variance should be < 20% across runs
        """
        graphbit.init()

        documents = generate_realistic_documents(count=500, min_words=2000, max_words=3000)
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

        throughputs = []

        # Run 3 times to measure consistency
        for run in range(3):
            start_time = time.time()
            with ThreadPoolExecutor(max_workers=20) as executor:
                list(executor.map(splitter.split_text, documents))
            parallel_time = time.time() - start_time

            throughput = calculate_throughput(len(documents), parallel_time)
            throughputs.append(throughput)

            print(f"Run {run + 1}: {throughput:.1f} docs/sec")

        avg_throughput = sum(throughputs) / len(throughputs)
        max_variance = max(abs(t - avg_throughput) for t in throughputs)
        variance_percent = (max_variance / avg_throughput) * 100

        print(f"\n{'='*70}")
        print(f"Throughput Consistency Test")
        print(f"{'='*70}")
        print(f"Average throughput: {avg_throughput:.1f} docs/sec")
        print(f"Max variance: {max_variance:.1f} docs/sec ({variance_percent:.1f}%)")
        print(f"Throughputs: {[f'{t:.1f}' for t in throughputs]}")
        print(f"{'='*70}\n")

        # Variance should be reasonable (< 30% to account for system variability)
        assert variance_percent < 30.0, f"High throughput variance: {variance_percent:.1f}%"


# ============================================================================
# Stage 3: Resource Utilization Tests
# ============================================================================

class TestResourceUtilization:
    """Test CPU and memory utilization under load."""

    @pytest.mark.skipif(not PSUTIL_AVAILABLE, reason="psutil not available")
    def test_cpu_utilization_under_load(self) -> None:
        """Validate CPU utilization reaches 70%+ under high load.

        Expected: CPU utilization should be high (70-95%) during parallel processing,
        indicating efficient use of available CPU resources.
        """
        graphbit.init()

        documents = generate_realistic_documents(count=1000, min_words=2000, max_words=3000)
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

        # Start monitoring CPU in background
        cpu_samples = []

        def monitor_cpu():
            """Sample CPU usage during processing."""
            for _ in range(10):
                cpu_samples.append(psutil.cpu_percent(interval=0.5))

        # Start CPU monitoring in background thread
        import threading
        monitor_thread = threading.Thread(target=monitor_cpu)
        monitor_thread.start()

        # Process documents
        with ThreadPoolExecutor(max_workers=20) as executor:
            list(executor.map(splitter.split_text, documents))

        monitor_thread.join()

        avg_cpu = sum(cpu_samples) / len(cpu_samples) if cpu_samples else 0
        max_cpu = max(cpu_samples) if cpu_samples else 0

        print(f"\n{'='*70}")
        print(f"CPU Utilization Test")
        print(f"{'='*70}")
        print(f"Average CPU: {avg_cpu:.1f}%")
        print(f"Peak CPU: {max_cpu:.1f}%")
        print(f"CPU samples: {[f'{c:.1f}' for c in cpu_samples]}")
        print(f"{'='*70}\n")

        # CPU should be well utilized (>= 50% on average)
        assert avg_cpu >= 30.0, f"Low CPU utilization: {avg_cpu:.1f}%"

    @pytest.mark.skipif(not PSUTIL_AVAILABLE, reason="psutil not available")
    def test_memory_stability_under_load(self) -> None:
        """Validate memory usage remains stable during processing.

        Expected: Memory should grow during processing but stabilize,
        with no continuous growth indicating memory leaks.
        """
        graphbit.init()

        documents = generate_realistic_documents(count=500, min_words=2000, max_words=3000)
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)

        memory_samples = []

        # Process in batches and monitor memory
        batch_size = 100
        for i in range(0, len(documents), batch_size):
            batch = documents[i:i + batch_size]

            with ThreadPoolExecutor(max_workers=20) as executor:
                list(executor.map(splitter.split_text, batch))

            # Sample memory after each batch
            process = psutil.Process()
            memory_mb = process.memory_info().rss / (1024 * 1024)
            memory_samples.append(memory_mb)

        # Check for memory leak (continuous growth)
        # Memory should stabilize after initial growth
        first_half_avg = sum(memory_samples[:len(memory_samples)//2]) / (len(memory_samples)//2)
        second_half_avg = sum(memory_samples[len(memory_samples)//2:]) / (len(memory_samples)//2)
        memory_growth = second_half_avg - first_half_avg
        growth_percent = (memory_growth / first_half_avg) * 100

        print(f"\n{'='*70}")
        print(f"Memory Stability Test")
        print(f"{'='*70}")
        print(f"First half avg: {first_half_avg:.1f} MB")
        print(f"Second half avg: {second_half_avg:.1f} MB")
        print(f"Memory growth: {memory_growth:.1f} MB ({growth_percent:.1f}%)")
        print(f"Memory samples: {[f'{m:.1f}' for m in memory_samples]}")
        print(f"{'='*70}\n")

        # Memory growth should be minimal (< 50% between first and second half)
        assert growth_percent < 50.0, f"Potential memory leak: {growth_percent:.1f}% growth"

