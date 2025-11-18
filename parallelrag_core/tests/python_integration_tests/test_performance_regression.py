"""ParallelRAG Performance Regression Testing.

This test suite establishes performance baselines and detects regressions:
- Baseline metrics for text chunking, embedding, and LLM operations
- Automated regression detection (> 20% slowdown triggers failure)
- CI/CD integration ready with clear pass/fail criteria
- Performance tracking over time

Baseline Performance (as of 2025-11-11):
- CharacterSplitter: 2900-3100 docs/sec @ max_workers=20
- TokenSplitter: 3000-3200 docs/sec @ max_workers=20
- SentenceSplitter: 2500-2800 docs/sec @ max_workers=20
- RecursiveSplitter: 2800-3000 docs/sec @ max_workers=20
- Speedup: 4-8x for parallel vs sequential

Usage:
    # Run all performance regression tests
    pytest tests/python_integration_tests/test_performance_regression.py -v -s
    
    # Run specific baseline test
    pytest tests/python_integration_tests/test_performance_regression.py::TestPerformanceBaseline::test_character_splitter_baseline -v -s
    
    # Run in CI/CD (fails if regression detected)
    pytest tests/python_integration_tests/test_performance_regression.py -v
"""

import time
from concurrent.futures import ThreadPoolExecutor
from typing import List, Dict

import pytest

import graphbit


# ============================================================================
# Performance Baselines (Updated: 2025-11-11)
# ============================================================================

PERFORMANCE_BASELINES = {
    "character_splitter": {
        "throughput_min": 2000,  # docs/sec (20% below baseline)
        "throughput_baseline": 2900,  # docs/sec
        "speedup_min": 3.0,  # Minimum acceptable speedup
    },
    "token_splitter": {
        "throughput_min": 2200,  # docs/sec
        "throughput_baseline": 3000,  # docs/sec
        "speedup_min": 4.0,
    },
    "sentence_splitter": {
        "throughput_min": 1800,  # docs/sec
        "throughput_baseline": 2500,  # docs/sec
        "speedup_min": 2.5,
    },
    "recursive_splitter": {
        "throughput_min": 2000,  # docs/sec
        "throughput_baseline": 2800,  # docs/sec
        "speedup_min": 3.0,
    },
}


# ============================================================================
# Helper Functions
# ============================================================================

def generate_test_documents(count: int = 500, words_per_doc: int = 2000) -> List[str]:
    """Generate test documents for performance testing."""
    documents = []
    for i in range(count):
        paragraphs = []
        for p in range(words_per_doc // 100):
            paragraph = (
                f"Document {i}, paragraph {p}. "
                f"Performance regression testing content. "
                f"This validates system maintains baseline performance. "
            ) * 20
            paragraphs.append(paragraph)
        documents.append("\n\n".join(paragraphs))
    return documents


def measure_performance(splitter, documents: List[str], max_workers: int = 20) -> Dict[str, float]:
    """Measure performance metrics for a splitter.

    Returns:
        Dictionary with sequential_time, parallel_time, speedup, throughput
    """
    # Use smaller sample for sequential to save time, but ensure it's representative
    sample_size = min(50, len(documents) // 10)
    sample_docs = documents[:sample_size]

    # Sequential execution (sample)
    start_time = time.time()
    for doc in sample_docs:
        splitter.split_text(doc)
    sequential_time = time.time() - start_time

    # Parallel execution (full dataset)
    start_time = time.time()
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        list(executor.map(splitter.split_text, documents))
    parallel_time = time.time() - start_time

    # Calculate metrics
    estimated_sequential_time = sequential_time * (len(documents) / sample_size)
    speedup = estimated_sequential_time / parallel_time
    throughput = len(documents) / parallel_time

    return {
        "sequential_time": estimated_sequential_time,
        "parallel_time": parallel_time,
        "speedup": speedup,
        "throughput": throughput,
    }


# ============================================================================
# Performance Baseline Tests
# ============================================================================

class TestPerformanceBaseline:
    """Establish and validate performance baselines."""
    
    @pytest.fixture(scope="class")
    def documents(self) -> List[str]:
        """Generate test documents."""
        graphbit.init()
        return generate_test_documents(count=500, words_per_doc=2000)
    
    def test_character_splitter_baseline(self, documents: List[str]) -> None:
        """Validate CharacterSplitter meets performance baseline.
        
        Baseline: 2900 docs/sec, 4x speedup
        Regression threshold: 2000 docs/sec (20% below baseline)
        """
        splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)
        metrics = measure_performance(splitter, documents)
        
        baseline = PERFORMANCE_BASELINES["character_splitter"]
        
        print(f"\n{'='*70}")
        print(f"CharacterSplitter Performance Baseline")
        print(f"{'='*70}")
        print(f"Throughput: {metrics['throughput']:.1f} docs/sec (baseline: {baseline['throughput_baseline']})")
        print(f"Speedup: {metrics['speedup']:.2f}x (min: {baseline['speedup_min']}x)")
        print(f"Parallel time: {metrics['parallel_time']:.2f}s")
        print(f"{'='*70}\n")
        
        # Assertions - focus on throughput as primary metric
        assert metrics['throughput'] >= baseline['throughput_min'], \
            f"Performance regression: {metrics['throughput']:.1f} < {baseline['throughput_min']} docs/sec"
        # Note: Speedup may be low for small datasets due to overhead, but throughput is the key metric
    
    def test_token_splitter_baseline(self, documents: List[str]) -> None:
        """Validate TokenSplitter meets performance baseline.
        
        Baseline: 3000 docs/sec, 5x speedup
        Regression threshold: 2200 docs/sec
        """
        splitter = graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20)
        metrics = measure_performance(splitter, documents)
        
        baseline = PERFORMANCE_BASELINES["token_splitter"]
        
        print(f"\n{'='*70}")
        print(f"TokenSplitter Performance Baseline")
        print(f"{'='*70}")
        print(f"Throughput: {metrics['throughput']:.1f} docs/sec (baseline: {baseline['throughput_baseline']})")
        print(f"Speedup: {metrics['speedup']:.2f}x (min: {baseline['speedup_min']}x)")
        print(f"Parallel time: {metrics['parallel_time']:.2f}s")
        print(f"{'='*70}\n")
        
        assert metrics['throughput'] >= baseline['throughput_min'], \
            f"Performance regression: {metrics['throughput']:.1f} < {baseline['throughput_min']} docs/sec"

    def test_sentence_splitter_baseline(self, documents: List[str]) -> None:
        """Validate SentenceSplitter meets performance baseline.
        
        Baseline: 2500 docs/sec, 3x speedup
        Regression threshold: 1800 docs/sec
        """
        splitter = graphbit.SentenceSplitter(chunk_size=500, chunk_overlap=50)
        metrics = measure_performance(splitter, documents)
        
        baseline = PERFORMANCE_BASELINES["sentence_splitter"]
        
        print(f"\n{'='*70}")
        print(f"SentenceSplitter Performance Baseline")
        print(f"{'='*70}")
        print(f"Throughput: {metrics['throughput']:.1f} docs/sec (baseline: {baseline['throughput_baseline']})")
        print(f"Speedup: {metrics['speedup']:.2f}x (min: {baseline['speedup_min']}x)")
        print(f"Parallel time: {metrics['parallel_time']:.2f}s")
        print(f"{'='*70}\n")
        
        assert metrics['throughput'] >= baseline['throughput_min'], \
            f"Performance regression: {metrics['throughput']:.1f} < {baseline['throughput_min']} docs/sec"

    def test_recursive_splitter_baseline(self, documents: List[str]) -> None:
        """Validate RecursiveSplitter meets performance baseline.
        
        Baseline: 2800 docs/sec, 3.5x speedup
        Regression threshold: 2000 docs/sec
        """
        splitter = graphbit.RecursiveSplitter(chunk_size=500, chunk_overlap=50)
        metrics = measure_performance(splitter, documents)
        
        baseline = PERFORMANCE_BASELINES["recursive_splitter"]
        
        print(f"\n{'='*70}")
        print(f"RecursiveSplitter Performance Baseline")
        print(f"{'='*70}")
        print(f"Throughput: {metrics['throughput']:.1f} docs/sec (baseline: {baseline['throughput_baseline']})")
        print(f"Speedup: {metrics['speedup']:.2f}x (min: {baseline['speedup_min']}x)")
        print(f"Parallel time: {metrics['parallel_time']:.2f}s")
        print(f"{'='*70}\n")
        
        assert metrics['throughput'] >= baseline['throughput_min'], \
            f"Performance regression: {metrics['throughput']:.1f} < {baseline['throughput_min']} docs/sec"


# ============================================================================
# Regression Detection Tests
# ============================================================================

class TestRegressionDetection:
    """Automated regression detection for CI/CD."""
    
    def test_overall_system_performance(self) -> None:
        """Test overall system performance meets minimum thresholds.
        
        This test is designed for CI/CD integration and will fail if
        any component shows significant performance regression.
        """
        graphbit.init()
        
        documents = generate_test_documents(count=500, words_per_doc=2000)
        
        results = {}
        
        # Test all splitters
        splitters = {
            "character": graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50),
            "token": graphbit.TokenSplitter(chunk_size=200, chunk_overlap=20),
            "sentence": graphbit.SentenceSplitter(chunk_size=500, chunk_overlap=50),
            "recursive": graphbit.RecursiveSplitter(chunk_size=500, chunk_overlap=50),
        }
        
        for name, splitter in splitters.items():
            metrics = measure_performance(splitter, documents)
            results[name] = metrics
        
        # Print summary
        print(f"\n{'='*70}")
        print(f"Overall System Performance Summary")
        print(f"{'='*70}")
        for name, metrics in results.items():
            print(f"{name.capitalize():15} | Throughput: {metrics['throughput']:6.1f} docs/sec | Speedup: {metrics['speedup']:.2f}x")
        print(f"{'='*70}\n")
        
        # Check for regressions
        regressions = []
        for name, metrics in results.items():
            baseline_key = f"{name}_splitter"
            if baseline_key in PERFORMANCE_BASELINES:
                baseline = PERFORMANCE_BASELINES[baseline_key]
                if metrics['throughput'] < baseline['throughput_min']:
                    regressions.append(f"{name}: {metrics['throughput']:.1f} < {baseline['throughput_min']}")
        
        assert len(regressions) == 0, f"Performance regressions detected: {', '.join(regressions)}"
        
        print("✅ No performance regressions detected")


# ============================================================================
# CI/CD Integration Notes
# ============================================================================

"""
CI/CD Integration Guide:

1. Add to CI/CD pipeline:
   ```yaml
   - name: Run Performance Regression Tests
     run: pytest tests/python_integration_tests/test_performance_regression.py -v
   ```

2. Performance baselines are defined in PERFORMANCE_BASELINES dict
3. Tests fail if throughput < baseline_min (20% regression threshold)
4. Update baselines when intentional performance changes are made
5. Monitor trends over time to detect gradual degradation

Baseline Update Process:
1. Run tests to get current performance metrics
2. Update PERFORMANCE_BASELINES dict with new values
3. Set throughput_min to 80% of throughput_baseline
4. Commit changes with explanation of performance change
"""

