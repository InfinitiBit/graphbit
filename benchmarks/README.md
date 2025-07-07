<<<<<<< Updated upstream
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
=======
# GraphBit Framework Benchmarks

This directory contains comprehensive benchmarking tools for comparing GraphBit with other popular AI agent frameworks including LangChain, LangGraph, PydanticAI, LlamaIndex, and CrewAI.

## Overview

The benchmark suite measures and compares:
- **Execution Time**: Task completion speed in milliseconds
- **Memory Usage**: RAM consumption in MB
- **CPU Usage**: Processor utilization percentage  
- **Token Count**: LLM token consumption
- **Throughput**: Tasks completed per second
- **Error Rate**: Failure percentage across scenarios
- **Latency**: Response time measurements

## Benchmark Scenarios

The suite runs six different scenarios to test various aspects of framework performance:

1. **Simple Task**: Basic single LLM call
2. **Sequential Pipeline**: Chain of dependent tasks
3. **Parallel Pipeline**: Concurrent independent tasks
4. **Complex Workflow**: Multi-step workflow with conditional logic
5. **Memory Intensive**: Large data processing tasks
6. **Concurrent Tasks**: Multiple simultaneous operations

## Prerequisites

### API Keys Required

You'll need API keys for the LLM providers you want to test:

- **OpenAI**: Required for most frameworks
- **Anthropic**: Optional, for Claude models
- **HuggingFace**: Optional, for open-source models

### System Requirements

- **Docker**: 4GB+ RAM, 2+ CPU cores (recommended)
- **Local**: Python 3.10+, Rust 1.70+, 8GB+ RAM

## 🐳 Running with Docker (Recommended)

Docker provides a consistent, isolated environment for benchmarking with all dependencies pre-installed.

### Quick Start

1. **Clone and navigate to the project:**
   ```bash
   git clone <repository-url>
   cd graphbit/benchmarks
   ```

2. **Set up environment variables:**
   ```bash
   # Create .env file with your API keys
   cat > .env << EOF
   OPENAI_API_KEY=your_openai_key_here
   ANTHROPIC_API_KEY=your_anthropic_key_here
   HUGGINGFACE_API_KEY=your_huggingface_key_here
   EOF
   ```

3. **Build and run benchmarks:**
   ```bash
   # Build the Docker image
   docker-compose build

   # Run all benchmarks with default settings
   docker-compose run --rm graphbit-benchmark run_benchmark.py

   # Run specific frameworks
   docker-compose run --rm graphbit-benchmark run_benchmark.py --frameworks "graphbit,langchain"

   # Run with custom model
   docker-compose run --rm graphbit-benchmark run_benchmark.py --provider openai --model gpt-4o
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

### Accessing Results

Results are automatically saved to local directories:
- **Results**: `./results/` - JSON data and visualizations
- **Logs**: `./logs/` - Detailed execution logs per framework

```bash
# View results
ls -la results/
cat results/benchmark_results_*.json

# View logs
ls -la logs/
tail -f logs/graphbit.log
```

## 🖥️ Running Locally (Development)

For development, debugging, or when Docker isn't available.

### Prerequisites Setup

1. **Install system dependencies:**
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install build-essential curl git pkg-config libssl-dev

   # macOS
   brew install pkg-config openssl
   ```

2. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Set up Python environment:**
   ```bash
   # Using Poetry (recommended)
   cd /path/to/graphbit
   poetry install --with dev,benchmarks
   
   # Using pip
   pip install -e .[benchmarks]
   ```

### Build GraphBit

1. **Build Rust components:**
   ```bash
   cd python
   unset ARGV0  # Important: prevents build issues
   poetry run maturin develop --release
   ```

2. **Verify installation:**
   ```bash
   poetry run python -c "import graphbit; print('GraphBit installed successfully')"
   ```

### Running Benchmarks

1. **Set environment variables:**
   ```bash
   export OPENAI_API_KEY="your_openai_key_here"
   export ANTHROPIC_API_KEY="your_anthropic_key_here"
   export HUGGINGFACE_API_KEY="your_huggingface_key_here"
   ```

2. **Navigate to benchmarks directory:**
   ```bash
   cd benchmarks
   ```

3. **Run benchmarks:**
   ```bash
   # Basic run
   poetry run python run_benchmark.py

   # With custom options
   poetry run python run_benchmark.py \
     --provider openai \
     --model gpt-4o-mini \
     --frameworks "graphbit,langchain,pydantic_ai" \
     --scenarios "simple_task,complex_workflow" \
     --verbose
   ```

### Local Development Commands

```bash
# Install dependencies
poetry install

# Run linting
poetry run flake8 frameworks/
poetry run mypy frameworks/

# Run specific framework test
poetry run python -c "
from frameworks.graphbit_benchmark import GraphBitBenchmark
import asyncio
config = {'model': 'gpt-4o-mini', 'api_key': 'your_key'}
benchmark = GraphBitBenchmark(config)
asyncio.run(benchmark.run_simple_task())
"
```

## Configuration Options

### Command Line Arguments

| Option | Default | Description |
|--------|---------|-------------|
| `--provider` | `openai` | LLM provider (openai, anthropic, ollama, huggingface) |
| `--model` | `gpt-4o-mini` | Model name to use |
| `--temperature` | `0.1` | Temperature parameter (0.0-2.0) |
| `--max-tokens` | `2000` | Maximum tokens per response |
| `--api-key` | (env var) | API key override |
| `--base-url` | None | Custom API endpoint (for Ollama) |
| `--frameworks` | All | Comma-separated framework list |
| `--scenarios` | All | Comma-separated scenario list |
| `--verbose` | False | Enable detailed output |
| `--list-models` | False | Show available models and exit |

### Framework Options

Available frameworks:
- `graphbit` - GraphBit (this library)
- `langchain` - LangChain
- `langgraph` - LangGraph  
- `pydantic_ai` - PydanticAI
- `llamaindex` - LlamaIndex
- `crewai` - CrewAI

### Scenario Options

Available scenarios:
- `simple_task` - Basic single LLM call
- `sequential_pipeline` - Chain of dependent tasks
- `parallel_pipeline` - Concurrent independent tasks
- `complex_workflow` - Multi-step workflow
- `memory_intensive` - Large data processing
- `concurrent_tasks` - Multiple simultaneous operations

## Results and Analysis

### Output Files

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

### Reading Results

```bash
# View latest results
cat results/benchmark_results_*.json | jq '.frameworks.GraphBit.results'

# Compare execution times
cat results/benchmark_results_*.json | jq '.frameworks | to_entries | map({framework: .key, avg_time: (.value.results | to_entries | map(.value.execution_time_ms) | add / length)})'

# Check error rates
cat results/benchmark_results_*.json | jq '.frameworks | to_entries | map({framework: .key, errors: .value.errors})'
```

## Troubleshooting

### Common Issues

1. **API Key Errors**:
   ```bash
   ERROR: OpenAI API key not found
   ```
   **Solution**: Ensure environment variables are set correctly


2. **Memory Errors**:
   ```bash
   ERROR: Out of memory during benchmark
   ```
   **Solution**: Increase Docker memory limits or close other applications

3. **Import Errors**:
   ```bash
   ImportError: No module named 'graphbit'
   ```
   **Solution**: Rebuild with `maturin develop --release`

### Docker Issues

```bash
docker-compose build --no-cache

docker-compose logs graphbit-benchmark

docker-compose run --rm graphbit-benchmark bash
```

### Performance Issues

```bash
docker stats graphbit-benchmark

docker-compose run --rm graphbit-benchmark run_benchmark.py --scenarios "simple_task"

docker-compose run --rm graphbit-benchmark run_benchmark.py --model gpt-3.5-turbo
```

## Development

### Adding New Frameworks

1. Create `frameworks/your_framework_benchmark.py`:
   ```python
   from .common import BaseBenchmark, BenchmarkMetrics

   class YourFrameworkBenchmark(BaseBenchmark):
       async def setup(self):
           # Initialize your framework
           pass

       async def run_simple_task(self) -> BenchmarkMetrics:
           # Implement benchmark logic
           pass
   ```

2. Add to `run_benchmark.py`:
   ```python
   from frameworks.your_framework_benchmark import YourFrameworkBenchmark

   # Add to frameworks dict
   FrameworkType.YOUR_FRAMEWORK: {
       "name": "YourFramework",
       "benchmark": YourFrameworkBenchmark(self.config),
       # ...
   }
   ```

### Custom Scenarios

Add new scenarios to `BenchmarkScenario` enum and implement in each framework benchmark class.

## Support

For issues and questions:
- **Issues**: Create a GitHub issue with benchmark logs
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check the main GraphBit documentation

## License

This benchmark suite is part of the GraphBit project and follows the same license terms. 
>>>>>>> Stashed changes
