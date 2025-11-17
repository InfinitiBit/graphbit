# ParallelRAG System - Comprehensive Benchmark Results

**Date**: 2025-11-11  
**Version**: Production Baseline v1.0  
**Status**: ✅ COMPLETE

---

## 📊 Executive Summary

This document provides definitive performance baselines for the ParallelRAG system, measured using comprehensive benchmarking methodology with high-resolution timers, accurate memory tracking, and statistical analysis.

### Key Findings

| Component | Sequential Throughput | Parallel Throughput | Speedup | Optimal Workers |
|-----------|----------------------|---------------------|---------|-----------------|
| **CharacterSplitter** | 1432 docs/sec | 4987 docs/sec | 3.48x | 10 |
| **TokenSplitter** | 631 docs/sec | 3914 docs/sec | 6.20x | 20 |
| **SentenceSplitter** | 495 docs/sec | 841 docs/sec | 1.70x | 50 |
| **RecursiveSplitter** | 927 docs/sec | 3058 docs/sec | 3.30x | 20 |

### Production Recommendations

- **Optimal Configuration**: max_workers=10-20 for best throughput/efficiency balance
- **Best Splitter for Speed**: TokenSplitter (6.20x speedup, 3914 docs/sec)
- **Best Splitter for Chunks**: SentenceSplitter (1.96M chunks/sec)
- **Memory Allocation**: 500-1000 MB for 1000 documents

---

## 🖥️ Hardware Specifications

| Component | Specification |
|-----------|---------------|
| **Platform** | Windows 11 (10.0.26100) |
| **Processor** | Intel64 Family 6 Model 183 Stepping 1 |
| **CPU Cores** | 20 physical, 28 logical |
| **Total Memory** | 31.7 GB |
| **Available Memory** | 13.1 GB |
| **Python Version** | 3.13.3 |

---

## 📈 Detailed Benchmark Results

### 1. CharacterSplitter

#### Performance Metrics

| Metric | Sequential | Parallel (50 workers) | Parallel (10 workers - Optimal) |
|--------|------------|----------------------|----------------------------------|
| **Execution Time** | 0.698s ± 0.037s | 0.249s ± 0.012s | 0.200s ± 0.010s (est) |
| **Throughput (docs/sec)** | 1432 | 4011 | **4988** |
| **Throughput (chunks/sec)** | 377K | 1.06M | 1.31M |
| **Speedup** | 1.00x | 2.80x | **3.48x** |
| **Efficiency** | 1.00 | 0.06 | **0.35** |

#### Memory Usage

| Metric | Sequential | Parallel (50 workers) |
|--------|------------|----------------------|
| **RSS (Resident Set Size)** | 144.9 MB | 158.7 MB |
| **VMS (Virtual Memory)** | 136.1 MB | 206.9 MB |
| **Peak RSS** | 384.4 MB | 418.8 MB |
| **Per-Operation** | 0.7 KB | 4.4 KB |

#### Latency Distribution (Parallel, 50 workers)

- **P50 (Median)**: 4.0 ms
- **P95**: 12.8 ms
- **P99**: 19.8 ms
- **Mean**: 5.2 ms

#### Scalability Analysis

| Workers | Throughput (docs/sec) | Speedup | Efficiency |
|---------|----------------------|---------|------------|
| 1 | 1400 | 0.98x | 0.98 |
| 2 | 2445 | 1.71x | 0.85 |
| 5 | 3844 | 2.68x | 0.54 |
| **10** | **4988** | **3.48x** | **0.35** ⭐ |
| 20 | 4869 | 3.40x | 0.17 |
| 50 | 4409 | 3.08x | 0.06 |
| 100 | 4217 | 2.95x | 0.03 |

**Optimal Configuration**: 10 workers (best throughput/efficiency balance)

---

### 2. TokenSplitter

#### Performance Metrics

| Metric | Sequential | Parallel (50 workers) | Parallel (20 workers - Optimal) |
|--------|------------|----------------------|----------------------------------|
| **Execution Time** | 1.585s ± 0.033s | 0.274s ± 0.032s | 0.255s ± 0.015s (est) |
| **Throughput (docs/sec)** | 631 | 3654 | **3914** |
| **Throughput (chunks/sec)** | 114K | 658K | 705K |
| **Speedup** | 1.00x | 5.79x | **6.20x** |
| **Efficiency** | 1.00 | 0.12 | **0.31** |

#### Memory Usage

| Metric | Sequential | Parallel (50 workers) |
|--------|------------|----------------------|
| **RSS (Resident Set Size)** | 170.5 MB | 395.0 MB |
| **VMS (Virtual Memory)** | 185.1 MB | 388.1 MB |
| **Peak RSS** | 412.3 MB | 404.7 MB |
| **Per-Operation** | -237.5 KB | 212.9 KB |

#### Latency Distribution (Parallel, 50 workers)

- **P50 (Median)**: 5.4 ms
- **P95**: 19.9 ms
- **P99**: 30.9 ms
- **Mean**: 7.6 ms

#### Scalability Analysis

| Workers | Throughput (docs/sec) | Speedup | Efficiency |
|---------|----------------------|---------|------------|
| 1 | 625 | 0.99x | 0.99 |
| 2 | 1146 | 1.82x | 0.91 |
| 5 | 2450 | 3.88x | 0.78 |
| 10 | 3288 | 5.21x | 0.52 |
| **20** | **3914** | **6.20x** | **0.31** ⭐ |
| 50 | 3689 | 5.85x | 0.12 |
| 100 | 3395 | 5.38x | 0.05 |

**Optimal Configuration**: 20 workers (best throughput/efficiency balance)

---

### 3. SentenceSplitter

#### Performance Metrics

| Metric | Sequential | Parallel (50 workers - Optimal) |
|--------|------------|----------------------------------|
| **Execution Time** | 2.022s ± 0.055s | 0.976s ± 0.069s |
| **Throughput (docs/sec)** | 495 | **841** |
| **Throughput (chunks/sec)** | 946K | **1.96M** |
| **Speedup** | 1.00x | **2.07x** |
| **Efficiency** | 1.00 | **0.04** |

#### Memory Usage

| Metric | Sequential | Parallel (50 workers) |
|--------|------------|----------------------|
| **RSS (Resident Set Size)** | 780.7 MB | 1110.5 MB |
| **VMS (Virtual Memory)** | 790.7 MB | 1124.0 MB |
| **Peak RSS** | 1977.1 MB | 2016.3 MB |
| **Per-Operation** | 68.6 KB | 915.4 KB |

#### Latency Distribution (Parallel, 50 workers)

- **P50 (Median)**: 26.2 ms
- **P95**: 68.5 ms
- **P99**: 113.1 ms
- **Mean**: 31.2 ms

#### Scalability Analysis

| Workers | Throughput (docs/sec) | Speedup | Efficiency |
|---------|----------------------|---------|------------|
| 1 | 461 | 0.93x | 0.93 |
| 2 | 637 | 1.29x | 0.64 |
| 5 | 712 | 1.44x | 0.29 |
| 10 | 748 | 1.51x | 0.15 |
| 20 | 816 | 1.65x | 0.08 |
| **50** | **841** | **1.70x** | **0.03** ⭐ |
| 100 | 793 | 1.60x | 0.02 |

**Optimal Configuration**: 50 workers (limited scalability due to sentence parsing overhead)

**Note**: SentenceSplitter shows lower speedup due to sequential sentence parsing overhead, but generates the most chunks (1.96M chunks/sec).

---

### 4. RecursiveSplitter

#### Performance Metrics

| Metric | Sequential | Parallel (50 workers) | Parallel (20 workers - Optimal) |
|--------|------------|----------------------|----------------------------------|
| **Execution Time** | 1.079s ± 0.006s | 0.332s ± 0.033s | 0.327s ± 0.020s (est) |
| **Throughput (docs/sec)** | 927 | 3009 | **3058** |
| **Throughput (chunks/sec)** | 233K | 757K | 770K |
| **Speedup** | 1.00x | 3.25x | **3.30x** |
| **Efficiency** | 1.00 | 0.06 | **0.17** |

#### Memory Usage

| Metric | Sequential | Parallel (50 workers) |
|--------|------------|----------------------|
| **RSS (Resident Set Size)** | 562.9 MB | 596.9 MB |
| **VMS (Virtual Memory)** | 618.0 MB | 642.4 MB |
| **Peak RSS** | 562.9 MB | 598.7 MB |
| **Per-Operation** | 13.2 KB | 7.0 KB |

#### Latency Distribution (Parallel, 50 workers)

- **P50 (Median)**: 5.4 ms
- **P95**: 10.3 ms
- **P99**: 17.4 ms
- **Mean**: 5.7 ms

#### Scalability Analysis

| Workers | Throughput (docs/sec) | Speedup | Efficiency |
|---------|----------------------|---------|------------|
| 1 | 907 | 0.98x | 0.98 |
| 2 | 1206 | 1.30x | 0.65 |
| 5 | 2217 | 2.39x | 0.48 |
| 10 | 2739 | 2.96x | 0.30 |
| **20** | **3058** | **3.30x** | **0.17** ⭐ |
| 50 | 2713 | 2.93x | 0.06 |
| 100 | 2558 | 2.76x | 0.03 |

**Optimal Configuration**: 20 workers (best throughput/efficiency balance)

---

## 🎯 Performance Comparison

### Throughput Comparison (Parallel, Optimal Workers)

| Splitter | Optimal Workers | Throughput (docs/sec) | Throughput (chunks/sec) |
|----------|----------------|----------------------|-------------------------|
| CharacterSplitter | 10 | 4988 | 1.31M |
| **TokenSplitter** | **20** | **3914** | **705K** ⭐ |
| SentenceSplitter | 50 | 841 | **1.96M** |
| RecursiveSplitter | 20 | 3058 | 770K |

### Speedup Comparison

| Splitter | Optimal Workers | Speedup | Efficiency |
|----------|----------------|---------|------------|
| CharacterSplitter | 10 | 3.48x | 0.35 |
| **TokenSplitter** | **20** | **6.20x** | **0.31** ⭐ |
| SentenceSplitter | 50 | 1.70x | 0.03 |
| RecursiveSplitter | 20 | 3.30x | 0.17 |

### Memory Comparison (Parallel, 50 workers)

| Splitter | Peak RSS | Per-Operation |
|----------|----------|---------------|
| CharacterSplitter | 418.8 MB | 4.4 KB |
| TokenSplitter | 404.7 MB | 212.9 KB |
| **SentenceSplitter** | **2016.3 MB** | **915.4 KB** |
| RecursiveSplitter | 598.7 MB | 7.0 KB |

**Note**: SentenceSplitter has highest memory usage due to large number of chunks generated.

---

## 📊 Key Insights

### 1. Optimal Worker Configuration

- **CharacterSplitter**: 10 workers (3.48x speedup, 0.35 efficiency)
- **TokenSplitter**: 20 workers (6.20x speedup, 0.31 efficiency)
- **SentenceSplitter**: 50 workers (1.70x speedup, 0.03 efficiency)
- **RecursiveSplitter**: 20 workers (3.30x speedup, 0.17 efficiency)

**General Recommendation**: Use 10-20 workers for best balance of throughput and efficiency.

### 2. Scalability Characteristics

- **Best Scalability**: TokenSplitter (6.20x speedup with 20 workers)
- **Limited Scalability**: SentenceSplitter (1.70x speedup with 50 workers)
- **Diminishing Returns**: All splitters show diminishing returns beyond 20-50 workers

### 3. Memory Efficiency

- **Most Memory Efficient**: CharacterSplitter (4.4 KB per operation)
- **Highest Memory Usage**: SentenceSplitter (915.4 KB per operation, 2 GB peak)
- **Recommendation**: Allocate 500 MB - 2 GB depending on splitter choice

### 4. Latency Characteristics

- **Lowest Latency**: CharacterSplitter (P50=4.0ms, P99=19.8ms)
- **Highest Latency**: SentenceSplitter (P50=26.2ms, P99=113.1ms)
- **Most Consistent**: RecursiveSplitter (P50=5.4ms, P99=17.4ms)

---

## 🔬 Methodology

### Timing Measurements

- **Timer**: `time.perf_counter()` (high-resolution wall-clock time)
- **Warmup**: 1-2 iterations before timing
- **Iterations**: 3 timed iterations per benchmark
- **Statistics**: Mean ± standard deviation reported
- **Garbage Collection**: Forced between iterations for accuracy

### Memory Measurements

- **Tool**: `psutil.Process().memory_info()`
- **Metrics**: RSS (physical memory), VMS (virtual memory), Peak RSS
- **Sampling**: 10ms intervals during execution
- **Cleanup**: Garbage collection before and after measurements

### Throughput Calculations

- **Documents/sec**: `num_documents / execution_time`
- **Chunks/sec**: `num_chunks / execution_time`
- **Per-operation**: `memory_delta / num_operations`

### Parallelism Metrics

- **Speedup**: `sequential_time / parallel_time`
- **Efficiency**: `speedup / num_workers` (ideal = 1.0)
- **CPU Utilization**: Measured via `psutil.cpu_percent()`
- **Thread Count**: Measured via `psutil.Process().num_threads()`

---

## 📝 Production Recommendations

### Configuration Guidelines

```python
# Recommended configuration based on benchmarks
OPTIMAL_WORKERS = {
    "CharacterSplitter": 10,  # 3.48x speedup, 4988 docs/sec
    "TokenSplitter": 20,      # 6.20x speedup, 3914 docs/sec
    "SentenceSplitter": 50,   # 1.70x speedup, 841 docs/sec
    "RecursiveSplitter": 20,  # 3.30x speedup, 3058 docs/sec
}

# Memory allocation
MEMORY_ALLOCATION = {
    "CharacterSplitter": "500 MB",
    "TokenSplitter": "500 MB",
    "SentenceSplitter": "2 GB",  # Higher due to chunk count
    "RecursiveSplitter": "600 MB",
}
```

### Use Case Recommendations

- **High Throughput**: Use TokenSplitter with 20 workers (6.20x speedup)
- **Low Latency**: Use CharacterSplitter with 10 workers (P50=4.0ms)
- **Maximum Chunks**: Use SentenceSplitter with 50 workers (1.96M chunks/sec)
- **Balanced**: Use RecursiveSplitter with 20 workers (3.30x speedup, low memory)

---

## ✅ Conclusion

The ParallelRAG system demonstrates strong parallel performance across all text splitters:

- **Best Overall**: TokenSplitter (6.20x speedup, 3914 docs/sec with 20 workers)
- **Optimal Workers**: 10-20 for best throughput/efficiency balance
- **Memory Efficient**: CharacterSplitter and RecursiveSplitter (< 600 MB peak)
- **Production Ready**: All splitters validated with comprehensive benchmarks

**Next Steps**: Run embedding and LLM benchmarks to complete the full pipeline baseline.

