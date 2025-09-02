<div align="center">

# GraphBit Framework Benchmarks

<p>
    This directory contains comprehensive benchmarking tools for comparing GraphBit with other popular AI agent frameworks including LangChain, LangGraph, PydanticAI, LlamaIndex, and CrewAI.
</p>

</div>

---

## Overview

The benchmark suite measures and compares:
- **Execution Time**: Task completion speed in milliseconds
- **Memory Usage**: RAM consumption in MB
- **CPU Usage**: Processor utilization percentage  
- **Token Count**: LLM token consumption
- **Throughput**: Tasks completed per second
- **Error Rate**: Failure percentage across scenarios
- **Latency**: Response time measurements

---

## Frameworks

- GraphBit
- LangChain
- LangGraph
- PydanticAI
- LlamaIndex
- CrewAI

---

## Benchmark Scenarios

The suite runs six different scenarios to test various aspects of framework performance:

1. **Simple Task**: Basic single LLM call
2. **Sequential Pipeline**: Chain of dependent tasks
3. **Parallel Pipeline**: Concurrent independent tasks
4. **Complex Workflow**: Multi-step workflow with conditional logic
5. **Memory Intensive**: Large data processing tasks
6. **Concurrent Tasks**: Multiple simultaneous operations

---

## Running Benchmarks

Use `run_benchmark.py` (or the module variant) to execute all scenarios with your chosen LLM provider and model.

### Example

```bash
python -m benchmarks.run_benchmark \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit,langchain \
  --scenarios simple_task,parallel_pipeline
```

### Concurrency Control

Use `--concurrency` to define how many tasks run in parallel.
By default, it uses the number of CPU cores available to the process.
Increase this value cautiously to avoid CPU contention.

```bash
python benchmarks/run_benchmark.py --provider openai --model gpt-4o-mini --concurrency 8
```

### CPU Core Pinning

Use `--cpu-cores` to specify a comma-separated list of cores (e.g. `0,1,2,3`).
This helps ensure more reproducible results by isolating benchmark workloads from other system processes.

```bash
python benchmarks/run_benchmark.py --cpu-cores 0,1,2,3 --concurrency 2
```

### Memory Binding

Use `--membind` to bind the benchmark process's memory allocations to a specific
NUMA node. This can improve consistency on multi-socket systems.

```bash
python benchmarks/run_benchmark.py --membind 0
```

If `libnuma` is not available, the benchmark will attempt to re-execute itself
under `numactl`. Make sure `numactl` is installed and in your `PATH` when using
`--membind`.

### Sequential Execution Recommended

Run scenarios **sequentially** by default to minimize noisy performance interference.
Only run multiple benchmarks in parallel if you assign each to a unique set of CPU cores with `--cpu-cores`.

| Option          | Description                                                                         | Example                                     |
| --------------- | ----------------------------------------------------------------------------------- | ------------------------------------------- |
| `--provider`    | LLM provider to use (e.g., `openai`, `anthropic`)                                   | `--provider openai`                         |
| `--model`       | Model name or ID for the chosen provider                                            | `--model gpt-4o-mini`                       |
| `--frameworks`  | Comma-separated list of frameworks to benchmark (e.g., `graphbit,langchain,crewai`) | `--frameworks graphbit,langchain`           |
| `--scenarios`   | Comma-separated list of benchmark scenarios to run                                  | `--scenarios simple_task,parallel_pipeline` |
| `--concurrency` | Number of tasks to run in parallel. Defaults to CPU cores count.                    | `--concurrency 8`                           |
| `--cpu-cores`   | Comma-separated list of specific CPU cores to pin the process to                    | `--cpu-cores 0,1,2,3`                       |
| `--membind`     | Bind memory allocations to a specific NUMA node                   | `--membind 0`                               |
| `--output`      | Path to save benchmark results JSON file                                            | `--output results.json`                     |
| `--verbose`     | Enable detailed logging                                                             | `--verbose`                                 |
| `--dry-run`     | Show which benchmarks would be run, but do not execute them                         | `--dry-run`                                 |


### Quick Start Cheatsheet

```bash
# Run all benchmarks sequentially with default concurrency (auto-detects CPU cores)
python -m benchmarks.run_benchmark \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit,langchain \
  --scenarios simple_task,parallel_pipeline

# Run with explicit concurrency
python benchmarks/run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --concurrency 8

# Pin to specific CPU cores for reproducible results
python benchmarks/run_benchmark.py \
  --cpu-cores 0,1,2,3 \
  --frameworks graphbit \
  --scenarios parallel_pipeline \
  --concurrency 2

# Just preview what will run without executing
python benchmarks/run_benchmark.py \
  --frameworks graphbit,langchain \
  --dry-run
```

### Docker Commands Reference

```bash
# Show available options
docker-compose run --rm graphbit-benchmark run_benchmark.py --help

# List available models for a provider
docker-compose run --rm graphbit-benchmark run_benchmark.py --provider openai --list-models

# Run specific scenarios only
docker-compose run --rm graphbit-benchmark run_benchmark.py --scenarios "simple_task,parallel_pipeline"

# Run with verbose output
docker-compose run --rm graphbit-benchmark run_benchmark.py --verbose

# Run with custom configuration
docker-compose run --rm graphbit-benchmark run_benchmark.py \
  --provider anthropic \
  --model claude-3-5-sonnet-20241022 \
  --temperature 0.2 \
  --max-tokens 1000 \
  --frameworks "graphbit,pydantic_ai" \
  --verbose
```

---

## Results

Results are saved in multiple formats:

1. **JSON Data**: `results/benchmark_results_YYYYMMDD_HHMMSS.json`
   - Raw performance metrics
   - Error details
   - Configuration used

2. **Visualizations**: `results/benchmark_comparison_YYYYMMDD_HHMMSS.png`
   - Performance comparison charts
   - Memory usage graphs
   - Execution time analysis

3. **Logs**: `logs/{framework}.log`
   - Detailed execution logs per framework
   - LLM responses and errors
   - Debug information

---
