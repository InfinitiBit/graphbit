# Benchmark Suite

This directory contains the **GraphBit benchmark suite** used to compare different agent frameworks (like GraphBit, LangChain, CrewAI, etc.) on performance and concurrency.

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

## Concurrency Control

Use `--concurrency` to define how many tasks run in parallel.
By default, it uses the number of CPU cores available to the process.
Increase this value cautiously to avoid CPU contention.

```bash
python benchmarks/run_benchmark.py --provider openai --model gpt-4o-mini --concurrency 8
```

## CPU Core Pinning

Use `--cpu-cores` to specify a comma-separated list of cores (e.g. `0,1,2,3`).
This helps ensure more reproducible results by isolating benchmark workloads from other system processes.

```bash
python benchmarks/run_benchmark.py --cpu-cores 0,1,2,3 --concurrency 2
```

## Memory Binding

Use `--membind` to bind the benchmark process's memory allocations to a specific
NUMA node. This can improve consistency on multi-socket systems.

```bash
python benchmarks/run_benchmark.py --membind 0
```

If `libnuma` is not available, the benchmark will attempt to re-execute itself
under `numactl`. Make sure `numactl` is installed and in your `PATH` when using
`--membind`.

## Sequential Execution Recommended

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

Quick Start Cheatsheet

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
