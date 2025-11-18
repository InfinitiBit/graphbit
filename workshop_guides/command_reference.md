# Non-Ollama ParallelRAG Command Reference

**Purpose**: Quick reference for all executable commands in workshop demonstrations  
**Last Updated**: November 17, 2025  
**Scope**: Cloud-based LLM providers (OpenAI, Anthropic) - **EXCLUDES** Ollama commands

---

## ⚙️ Prerequisites

### Environment Setup
```bash
# Set OpenAI API key (REQUIRED for most demos)
export OPENAI_API_KEY="sk-your-api-key-here"

# Set Anthropic API key (OPTIONAL - for Anthropic demos)
export ANTHROPIC_API_KEY="sk-ant-your-api-key-here"

# Install dependencies
pip install graphbit langchain langchain-openai langchain-anthropic
pip install langgraph crewai llama-index pydantic-ai
pip install matplotlib seaborn numpy pandas psutil
```

### Verify Installation
```bash
# Check GraphBit installation
python -c "import graphbit; print(f'GraphBit version: {graphbit.__version__}')"

# Check API key
python -c "import os; print('OpenAI API key:', 'SET' if os.getenv('OPENAI_API_KEY') else 'NOT SET')"
```

---

## 🚀 Quick Demo Commands (5 minutes)

### Demo 1: Basic ParallelRAG Example
```bash
# Run optimized ParallelRAG example
# Runtime: ~2-3 minutes
# API Cost: ~$0.01-0.02
python examples/parallel_rag_optimized.py
```

**What it demonstrates**:
- GIL-releasing document loading (10-50x speedup)
- Parallel embedding generation (5-10x speedup)
- Async LLM queries (5-20x speedup)

### Demo 2: Production-Ready RAG Application
```bash
# Run production RAG app with optimal configurations
# Runtime: ~2-3 minutes
# API Cost: ~$0.01-0.02
python parallel_rag_app.py
```

**What it demonstrates**:
- Optimal worker counts (20 workers)
- TokenSplitter (best performance)
- Production error handling

---

## 📊 Framework Comparison Commands (10 minutes)

### GraphBit vs LangChain Comparison
```bash
# Compare GraphBit and LangChain with 100 documents
# Runtime: ~5-10 minutes
# API Cost: ~$0.05-0.10
python tests/benchmarks/benchmark_framework_comparison.py \
  --framework both \
  --max-docs 100 \
  --max-workers 20 \
  --output framework_comparison_results.json
```

**What it demonstrates**:
- Side-by-side performance comparison
- Speedup calculations (GraphBit typically 1.2-2x faster)
- Resource usage (CPU%, Memory MB)

### Multi-Framework Comparison (6 Frameworks)
```bash
# Compare GraphBit, LangChain, LangGraph, CrewAI, LlamaIndex, PydanticAI
# Runtime: ~10-15 minutes
# API Cost: ~$0.10-0.20
cd benchmarks
python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --num-runs 3 \
  --verbose
```

**What it demonstrates**:
- GraphBit vs 5 other frameworks
- Multiple scenarios (simple, sequential, parallel, complex)
- Averaged results over multiple runs

### Test Specific Framework
```bash
# Test only GraphBit
python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks graphbit

# Test only LangChain
python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks langchain

# Test GraphBit and LangChain only
python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks graphbit,langchain
```

---

## 🔥 Stress Test Commands (15 minutes)

### Progressive Load Testing
```bash
# Test with increasing document counts: 100, 500, 1000 documents
# Runtime: ~10-15 minutes
# API Cost: NONE (uses mocked API calls)
python tests/benchmarks/benchmark_stress_test.py \
  --max-docs 1000 \
  --max-workers 20
```

**What it demonstrates**:
- Scalability from 100 to 1000+ documents
- Resource monitoring (CPU%, Memory MB)
- Safety thresholds (90% memory, 95% CPU)

### Worker Scaling Test
```bash
# Test different worker counts: 5, 10, 20, 50 workers
# Runtime: ~10-15 minutes
# API Cost: NONE (uses mocked API calls)
python tests/benchmarks/benchmark_stress_test.py \
  --max-docs 1000 \
  --max-workers 50
```

**What it demonstrates**:
- Optimal worker count identification
- Throughput vs worker count
- Diminishing returns at high concurrency

### Worker Optimization
```bash
# Find optimal worker count for your hardware
# Runtime: ~15-20 minutes
# API Cost: NONE (only tests load/chunk operations)
python test_worker_optimization.py
```

**What it demonstrates**:
- Tests 1, 5, 10, 20, 30, 50 workers
- Identifies optimal worker count
- Saves results to `worker_optimization_results.json`

---

## 📈 Visualization Commands (5 minutes)

### Generate Performance Charts
```bash
# Create 5 performance charts from benchmark results
# Runtime: ~30 seconds
# Requires: graphbit_stress_50k.json, langchain_stress_50k.json
python create_visualizations.py
```

**Generated charts**:
1. `chart_total_time.png` - Total Time vs Document Count
2. `chart_throughput.png` - Throughput vs Document Count
3. `chart_speedup.png` - GraphBit Speedup vs LangChain
4. `chart_component_breakdown.png` - Component Time Breakdown
5. `chart_extended_capacity.png` - Extended Capacity Results

### Generate Resource Charts
```bash
# Create 3 resource utilization charts
# Runtime: ~30 seconds
# Requires: graphbit_stress_50k.json, graphbit_max_capacity_*.json
python create_resource_charts.py
```

**Generated charts**:
1. `chart_memory_usage.png` - Memory Usage Across Document Scales
2. `chart_cpu_utilization.png` - CPU Utilization Patterns
3. `chart_resource_efficiency.png` - Throughput per GB Memory

### Generate Additional Charts
```bash
# Create 4 additional analysis charts
# Runtime: ~30 seconds
# Requires: Multiple JSON result files
python create_additional_visualizations.py
```

**Generated charts**:
1. `chart_worker_optimization.png` - Worker Count Optimization
2. `chart_document_size_impact.png` - Document Size Impact
3. `chart_cost_comparison.png` - Cost Comparison GraphBit vs LangChain
4. `chart_scaling_efficiency.png` - Scaling Efficiency (100-500K docs)

---

## 🔍 Component-Specific Benchmarks

### Chunking Benchmarks
```bash
# Benchmark all 4 text splitters
# Runtime: ~5-10 minutes
# API Cost: NONE
python tests/benchmarks/benchmark_chunking.py
```

### Embedding Benchmarks
```bash
# Benchmark embedding generation
# Runtime: ~5-10 minutes
# API Cost: ~$0.05-0.10 (500 chunks)
python tests/benchmarks/benchmark_embedding.py
```

### LLM Benchmarks
```bash
# Benchmark LLM completions
# Runtime: ~5-10 minutes
# API Cost: ~$0.05-0.10 (100 prompts)
python tests/benchmarks/benchmark_llm.py
```

---

## 📝 Notes

- **API Costs**: Most demos cost $0.01-0.20. Stress tests use mocked calls (FREE).
- **Runtime Estimates**: Based on typical hardware (4-8 cores, 16GB RAM).
- **Output Files**: All benchmarks save JSON results for later visualization.
- **Error Handling**: All scripts include proper error handling and validation.


