# Framework Comparison: ParallelRAG vs. LangChain vs. LangGraph vs. CrewAI

**Date**: 2025-11-17  
**Purpose**: Fair comparison of computational resource usage across Python agentic frameworks  
**Focus**: CPU, Memory, Throughput (excluding network latency)  
**Status**: ✅ **INFRASTRUCTURE READY**

---

## Executive Summary

This document provides a comprehensive framework for comparing ParallelRAG (GraphBit) against alternative Python agentic frameworks using **identical workloads** and **fair measurement methodologies**.

### Available Frameworks

| Framework | Status | Benchmark File | Notes |
|-----------|--------|----------------|-------|
| **GraphBit** | ✅ READY | `benchmarks/frameworks/graphbit_benchmark.py` | Direct API calls, minimal overhead |
| **LangChain** | ✅ READY | `benchmarks/frameworks/langchain_benchmark.py` | LCEL-based implementation |
| **LangGraph** | ✅ READY | `benchmarks/frameworks/langgraph_benchmark.py` | Graph-based workflows |
| **CrewAI** | ✅ READY | `benchmarks/frameworks/crewai_benchmark.py` | Agent-based approach |
| **LlamaIndex** | ✅ READY | `benchmarks/frameworks/llamaindex_benchmark.py` | RAG-focused framework |
| **PydanticAI** | ✅ READY | `benchmarks/frameworks/pydantic_ai_benchmark.py` | Type-safe AI framework |

---

## Benchmark Infrastructure

### Common Utilities (`benchmarks/frameworks/common.py`)

The benchmark infrastructure provides:

1. **Performance Monitoring**:
   - `PerformanceMonitor` class with `psutil` integration
   - CPU usage tracking (user + system time)
   - Memory usage tracking (RSS, VMS, peak memory)
   - Execution time measurement (high-resolution timers)

2. **Standardized Metrics**:
   - `BenchmarkMetrics` dataclass
   - Execution time (ms)
   - Memory usage (MB)
   - CPU usage (%)
   - Token count
   - Throughput (tasks/sec)
   - Error rate

3. **LLM Configuration**:
   - `LLMConfig` dataclass for provider-agnostic configuration
   - Support for OpenAI, Anthropic, Ollama
   - Standardized temperature and max_tokens

4. **Benchmark Scenarios**:
   - Simple Task
   - Sequential Pipeline
   - Parallel Pipeline
   - Complex Workflow
   - Memory Intensive
   - Concurrent Tasks

### Measurement Methodology

```python
class PerformanceMonitor:
    def start_monitoring(self):
        gc.collect()  # Clean up before starting
        tracemalloc.start()
        self.start_time = time.perf_counter()
        self.start_memory = self.process.memory_info().rss / 1024 / 1024  # MB
        self.start_cpu_times = self.process.cpu_times()
    
    def stop_monitoring(self) -> BenchmarkMetrics:
        end_time = time.perf_counter()
        end_memory = self.process.memory_info().rss / 1024 / 1024  # MB
        end_cpu_times = self.process.cpu_times()
        
        # Calculate CPU usage
        cpu_time_used = (end_cpu_times.user - self.start_cpu_times.user) + 
                        (end_cpu_times.system - self.start_cpu_times.system)
        wall_time = end_time - self.start_time
        cpu_usage_percent = (cpu_time_used / wall_time) * 100
        
        return BenchmarkMetrics(
            execution_time_ms=(end_time - self.start_time) * 1000,
            memory_usage_mb=end_memory - self.start_memory,
            cpu_usage_percent=cpu_usage_percent,
            ...
        )
```

---

## Running Framework Comparisons

### Prerequisites

```bash
# Install all frameworks
pip install graphbit langchain langchain-openai langchain-anthropic langchain-ollama
pip install langgraph crewai llama-index pydantic-ai

# Set API keys
export OPENAI_API_KEY="your-key"
export ANTHROPIC_API_KEY="your-key"  # Optional
```

### Basic Comparison

```bash
# Run all framework benchmarks
cd benchmarks
python run_benchmark.py --provider openai --model gpt-4o-mini --num-runs 10

# Run specific framework
python run_benchmark.py --framework graphbit --provider openai --model gpt-4o-mini

# Run with local models (no network latency)
python run_benchmark.py --provider ollama --model llama3.2
```

### Configuration

```python
from benchmarks.frameworks.common import LLMConfig, LLMProvider

# OpenAI configuration
config = {
    "llm_config": LLMConfig(
        provider=LLMProvider.OPENAI,
        model="gpt-4o-mini",
        temperature=0.1,
        max_tokens=2000,
        api_key=os.getenv("OPENAI_API_KEY")
    )
}

# Ollama configuration (local, no network)
config = {
    "llm_config": LLMConfig(
        provider=LLMProvider.OLLAMA,
        model="llama3.2",
        temperature=0.1,
        max_tokens=2000,
        base_url="http://localhost:11434"
    )
}
```

---

## Excluding Network Latency

### Strategy 1: Use Local Models (Recommended)

**Ollama** provides local LLM inference with zero network latency:

```bash
# Install Ollama
# Download from https://ollama.ai

# Pull model
ollama pull llama3.2

# Run benchmarks with local model
python run_benchmark.py --provider ollama --model llama3.2
```

**Benefits**:
- ✅ Zero network latency
- ✅ Consistent performance
- ✅ No API costs
- ✅ Reproducible results

**Limitations**:
- ⚠️ Slower inference than cloud APIs
- ⚠️ Requires local GPU/CPU resources
- ⚠️ Different model quality than GPT-4

### Strategy 2: Mock API Responses

For pure computational benchmarking, mock API responses:

```python
# Example: Mock embedding responses
class MockEmbeddingClient:
    def embed(self, text: str) -> List[float]:
        # Return deterministic embedding based on text hash
        import hashlib
        hash_val = int(hashlib.md5(text.encode()).hexdigest(), 16)
        return [(hash_val >> i) & 1 for i in range(1536)]

# Example: Mock LLM responses
class MockLLMClient:
    def complete(self, prompt: str, **kwargs) -> str:
        # Return deterministic response based on prompt
        return f"Mock response for: {prompt[:50]}..."
```

### Strategy 3: Cached Responses

Cache API responses for repeated benchmarks:

```python
import functools
import hashlib

@functools.lru_cache(maxsize=1000)
def cached_llm_call(prompt: str, max_tokens: int, temperature: float) -> str:
    # First call hits API, subsequent calls use cache
    return llm_client.complete(prompt, max_tokens=max_tokens, temperature=temperature)
```

---

## Fair Comparison Guidelines

### 1. Identical Workloads

Ensure all frameworks process the same tasks:

```python
# Common prompts defined in benchmarks/frameworks/common.py
SIMPLE_TASK_PROMPT = "Explain quantum computing in 2-3 sentences."
SEQUENTIAL_TASKS = [
    "Summarize the key concepts of machine learning.",
    "Explain how neural networks work.",
    "Describe the applications of deep learning."
]
PARALLEL_TASKS = [
    "What is artificial intelligence?",
    "What is machine learning?",
    "What is deep learning?"
]
```

### 2. Identical LLM Configuration

Use the same model, temperature, and max_tokens:

```python
# Standard configuration
DEFAULT_TEMPERATURE = 0.1
DEFAULT_MAX_TOKENS = 2000
DEFAULT_MODEL = "gpt-4o-mini"  # or "llama3.2" for local
```

### 3. Identical Measurement Methodology

All frameworks use the same `PerformanceMonitor`:

```python
monitor = PerformanceMonitor()
monitor.start_monitoring()

# Framework-specific execution
result = await framework.run_task()

metrics = monitor.stop_monitoring()
```

### 4. Multiple Runs for Statistical Significance

```bash
# Run 10 times and average results
python run_benchmark.py --num-runs 10
```

### 5. Controlled Environment

- ✅ Close unnecessary applications
- ✅ Disable background processes
- ✅ Use consistent hardware
- ✅ Run at similar times (avoid thermal throttling)

---

## Metrics to Compare

### Primary Metrics

| Metric | Unit | Description | Lower is Better |
|--------|------|-------------|-----------------|
| **Execution Time** | ms | Wall-clock time | ✅ |
| **Memory Usage** | MB | RSS memory delta | ✅ |
| **CPU Usage** | % | CPU time / wall time | ⚠️ (depends on parallelism) |
| **Throughput** | tasks/sec | Tasks completed per second | ❌ (higher is better) |

### Efficiency Metrics

| Metric | Formula | Description |
|--------|---------|-------------|
| **CPU Efficiency** | throughput / cpu_usage | Tasks per CPU% |
| **Memory Efficiency** | throughput / memory_usage | Tasks per MB |
| **Time Efficiency** | 1 / execution_time | Inverse of time |

---

## Expected Results (Preliminary)

Based on existing benchmarks in `benchmarks/report/framework-benchmark-report.md`:

| Framework | Execution Time | Memory Usage | CPU Usage | Notes |
|-----------|---------------|--------------|-----------|-------|
| **GraphBit** | ✅ **FASTEST** | ✅ **LOWEST** | ⚠️ **HIGHEST** | Direct API, minimal overhead |
| **LangChain** | ⚠️ MODERATE | ⚠️ MODERATE | ✅ MODERATE | LCEL overhead |
| **LangGraph** | ⚠️ MODERATE | ⚠️ MODERATE | ✅ MODERATE | Graph overhead |
| **CrewAI** | ❌ SLOWEST | ❌ HIGHEST | ✅ LOWEST | Agent coordination overhead |

**Note**: These are preliminary results. Run your own benchmarks for definitive comparison.

---

## Next Steps

1. **Run Baseline Benchmarks**:
   ```bash
   python benchmarks/run_benchmark.py --provider ollama --model llama3.2 --num-runs 10
   ```

2. **Analyze Results**:
   - Review `benchmarks/report/framework-benchmark-report.md`
   - Compare execution time, memory, CPU usage
   - Calculate efficiency metrics

3. **Optimize Configurations**:
   - Tune worker counts for each framework
   - Adjust batch sizes
   - Enable/disable features for fair comparison

4. **Document Findings**:
   - Update this document with actual results
   - Create visualizations (charts, graphs)
   - Publish comparison report

---

## Conclusion

The benchmark infrastructure is **ready for comprehensive framework comparison**. Use local models (Ollama) or mocked responses to exclude network latency and focus on pure computational performance.

**Status**: ✅ **INFRASTRUCTURE COMPLETE - READY FOR BENCHMARKING**

