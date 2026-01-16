<div align="center">

# GraphBit Framework Benchmarks

<p>
    This directory contains comprehensive benchmarking tools for comparing GraphBit with other popular AI agent frameworks including LangChain, LangGraph, PydanticAI, LlamaIndex, CrewAI, and AutoGen.
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
- AutoGen

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

## Environment Setup

`graphbit-benchmarks` supports **both uv and Poetry** for dependency management.

> **Python requirement:** `>=3.10,<3.14`  
> Tested on Python **3.10, 3.11, 3.12, 3.13**. Python 3.9 is not supported because some frameworks (CrewAI, LangChain, etc.) require 3.10+.

### Option 1: Using `uv` (Recommended)

[uv](https://github.com/astral-sh/uv) is a fast Python package installer and dependency manager. It reads from the `[project]` section in `pyproject.toml`.

**Installation:**

```bash
# Install uv via pip
pip install uv

# Or using pipx (recommended for global tools)
pipx install uv

# Or using the standalone installer
curl -LsSf https://astral.sh/uv/install.sh | sh  # Unix/macOS
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"  # Windows
```

**Setup:**

```bash
# Navigate to benchmarks directory
cd benchmarks

# Create virtual environment and install dependencies
uv sync

# Activate the virtual environment
# On Unix/Linux/macOS:
source .venv/bin/activate

# On Windows (PowerShell):
.venv\Scripts\Activate.ps1

# On Windows (Command Prompt):
.venv\Scripts\activate.bat
```

**Verify installation:**

```bash
python -c "import graphbit; print(f'✓ GraphBit {graphbit.__version__}')"
python -c "import langchain; import crewai; print('✓ All frameworks loaded')"
```

### Option 2: Using `poetry`

`poetry` reads from the `[tool.poetry]` section in `pyproject.toml`.

```bash
# Navigate to benchmarks directory
cd benchmarks

# Install dependencies (without creating a package)
poetry install --no-root

# Activate the virtual environment
poetry shell

# Or run commands directly with poetry
poetry run python run_benchmark.py --help
```

---

## Running Benchmarks

Use `run_benchmark.py` to execute all scenarios with your chosen LLM provider and model.

### Basic Usage

**With activated virtual environment:**

```bash
# Using python directly (after activating .venv via uv or poetry)
python -m benchmarks.run_benchmark \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit,langchain \
  --scenarios simple_task,parallel_pipeline
```

**With UV (without activating):**

```bash
# Execute using uv directly
uv run python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit,langchain \
  --scenarios simple_task,parallel_pipeline
```

**With Poetry:**

```bash
# Execute using poetry run
poetry run python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit,langchain \
  --scenarios simple_task,parallel_pipeline
```

### Azure OpenAI Provider

Azure OpenAI uses **deployment names** instead of model names.

**Setup:**

Set three environment variables:

```bash
# Unix/Linux/macOS
export AZURE_OPENAI_API_KEY=your_key_here
export AZURE_OPENAI_ENDPOINT=https://YOUR_RESOURCE.openai.azure.com/
export AZURE_OPENAI_API_VERSION=2024-02-15-preview

# Windows PowerShell
$env:AZURE_OPENAI_API_KEY="your_key_here"
$env:AZURE_OPENAI_ENDPOINT="https://YOUR_RESOURCE.openai.azure.com/"
$env:AZURE_OPENAI_API_VERSION="2025-01-01-preview"
```

**Running Benchmarks:**

```bash
# Basic usage (with environment variables set)
python run_benchmark.py \
  --provider azure_openai \
  --model YOUR_DEPLOYMENT_NAME \
  --frameworks langchain,langgraph

# With CLI arguments (overrides env vars)
python run_benchmark.py \
  --provider azure_openai \
  --model YOUR_DEPLOYMENT_NAME \
  --api-key YOUR_KEY \
  --base-url https://YOUR_RESOURCE.openai.azure.com/

# List available deployment models
python run_benchmark.py --provider azure_openai --list-models
```

**Important Notes:**
- **Deployment Name**: Use your Azure deployment name as the `--model` argument.
- **Endpoint**: Must include `https://` and usually ends with `/`.
- **API Version**: Defaults to `2025-01-01-preview` if not specified.


### Concurrency Control

Use `--concurrency` to define how many tasks run in parallel.
By default, it uses the number of CPU cores available to the process.
Increase this value cautiously to avoid CPU contention.

```bash
# Using activated environment
python benchmarks/run_benchmark.py --provider openai --model gpt-4o --concurrency 8

# Or with uv
uv run python run_benchmark.py --provider openai --model gpt-4o --concurrency 8
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

### Run Multiple Times (Averaging)

By default, each scenario runs **once** (`--num-runs 1`) for quick testing. For **statistically reliable results**, run each scenario multiple times and average:

```bash
# Run each scenario 10 times and average for better statistical reliability
python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --num-runs 10

# Run 20 times for even more reliable results
python run_benchmark.py --num-runs 20

# With uv
uv run python run_benchmark.py --num-runs 10
```

> **💡 Tip:** For production benchmarks or comparative analysis, use `--num-runs 10` or higher to reduce variance and get more reliable metrics. Single runs can be affected by system noise, API latency variations, and other transient factors.

### Sequential Execution Recommended

Run scenarios **sequentially** by default to minimize noisy performance interference.
Only run multiple benchmarks in parallel if you assign each to a unique set of CPU cores with `--cpu-cores`.

### Command-Line Options Reference

### Automatic .env Loading (New)
The benchmark runner now automatically loads credentials from a `.env` file in the benchmarks directory. This eliminates the need to manually set environment variables or pass them via CLI.

1. **Create .env file:**
   ```bash
   cp .env.example .env
   ```
2. **Add credentials:**
   ```bash
   AZURE_OPENAI_API_KEY=your_key
   AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com/
   AZURE_OPENAI_API_VERSION=2025-01-01-preview
   ```
3. **Run benchmarks:**
   ```bash
   python run_benchmark.py --provider azure_openai --model gpt-4o
   ```


### Command-Line Options Reference

| Option          | Description                                                                         | Example                                     |
| --------------- | ----------------------------------------------------------------------------------- | ------------------------------------------- |
| `--provider`    | LLM provider: `openai`, `anthropic`, `ollama`, `azure_openai`                       | `--provider azure_openai`                   |
| `--model`       | Model name or ID for the chosen provider (Default: `gpt-4o`)                        | `--model gpt-5`                             |
| `--frameworks`  | Comma-separated frameworks to benchmark (defaults to all)                           | `--frameworks graphbit,langchain`           |
| `--scenarios`   | Comma-separated benchmark scenarios to run (defaults to all)                        | `--scenarios simple_task,parallel_pipeline` |
| `--concurrency` | Number of tasks to run in parallel (defaults to CPU core count)                     | `--concurrency 8`                           |
| `--cpu-cores`   | Comma-separated list of CPU cores to pin the process to                             | `--cpu-cores 0,1,2,3`                       |
| `--membind`     | Bind memory allocations to a specific NUMA node                                     | `--membind 0`                               |
| `--num-runs`    | Number of times to repeat each scenario (results averaged, default: 1)              | `--num-runs 20`                             |
| `--temperature` | Temperature parameter for LLM (default: 0.1)                                        | `--temperature 0.5`                         |
| `--max-tokens`  | Maximum tokens for LLM responses (default: 2000)                                    | `--max-tokens 1000`                         |
| `--api-key`     | API key for the LLM provider (overrides .env)                                       | `--api-key sk-...`                          |
| `--base-url`    | Base URL for custom endpoints (e.g., Ollama)                                        | `--base-url http://localhost:11434`         |
| `--output`      | Path to save benchmark results JSON file                                            | `--output results.json`                     |
| `--verbose`     | Enable detailed logging                                                             | `--verbose`                                 |
| `--list-models` | List all supported models (includes 2025/2026 releases)                             | `--list-models`                             |

**Supported Models:**
The benchmark suite supports the latest models as of Jan 2026, including:
- **OpenAI**: `gpt-5`, `gpt-4.5`, `gpt-4o`, `o3-mini`, `o1-series`
- **Anthropic**: `claude-4.5-opus`, `claude-4.5-sonnet`, `claude-3-5-sonnet`
- **Ollama**: `llama3.3`, `phi4`, `deepseek-r1`
- **Azure**: Deployment names for all above models


### Quick Start Cheatsheet

**Using activated virtual environment (UV or Poetry):**

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

# Run each scenario 20 times and average
python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --num-runs 20

# List available models for a provider
python run_benchmark.py --provider openai --list-models
```

**Using UV (without activation):**

```bash
# Run all benchmarks
uv run python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini

# Run specific frameworks and scenarios
uv run python run_benchmark.py \
  --frameworks graphbit,crewai \
  --scenarios simple_task,complex_workflow \
  --num-runs 15

# With verbose output
uv run python run_benchmark.py --verbose
```

**Using Poetry:**

```bash
# Run benchmarks
poetry run python run_benchmark.py \
  --provider anthropic \
  --model claude-3-5-sonnet-20241022

# With custom configuration
poetry run python run_benchmark.py \
  --provider anthropic \
  --model claude-3-5-sonnet-20241022 \
  --temperature 0.2 \
  --max-tokens 1000 \
  --frameworks graphbit,pydantic_ai \
  --verbose
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

## Troubleshooting & Support Guide

### Environment Issues

#### UV Installation Problems

**Issue:** `uv: command not found`

```bash
# Solution 1: Install via pip
pip install uv

# Solution 2: Install via pipx (recommended)
pipx install uv

# Solution 3: Use standalone installer
# Unix/macOS:
curl -LsSf https://astral.sh/uv/install.sh | sh

# Windows (PowerShell):
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"

# Verify installation
uv --version
```

**Issue:** `uv sync` fails with dependency resolution errors

```bash
# Try clearing UV cache
uv cache clean

# Retry sync
uv sync

# If still failing, try with verbose output
uv sync --verbose
```

#### Poetry Issues

**Issue:** `poetry: command not found`

```bash
# Install Poetry
curl -sSL https://install.python-poetry.org | python3 -

# Or via pipx
pipx install poetry

# Verify installation
poetry --version
```

**Issue:** Poetry environment not activating

```bash
# Check if virtual environment exists
poetry env list

# If not, create it
poetry install --no-root

# Activate explicitly
poetry shell

# Or find the path and activate manually
poetry env info --path
source $(poetry env info --path)/bin/activate  # Unix/macOS/Linux
```

### Virtual Environment Activation

**Windows PowerShell:**

```powershell
# If you get execution policy error
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Then activate
.venv\Scripts\Activate.ps1
```

**Windows Command Prompt:**

```cmd
.venv\Scripts\activate.bat
```

**Unix/Linux/macOS:**

```bash
source .venv/bin/activate
```

### API Key Issues

**Issue:** `Error: OpenAI API key is required`

```bash
# Solution 1: Set environment variable (Unix/macOS/Linux)
export OPENAI_API_KEY="sk-your-key-here"

# Windows (PowerShell)
$env:OPENAI_API_KEY="sk-your-key-here"

# Windows (Command Prompt)
set OPENAI_API_KEY=sk-your-key-here

# Solution 2: Use .env file
cp .env.example .env
# Edit .env and add your API key

# Solution 3: Pass via command line
python run_benchmark.py --api-key "sk-your-key-here"
```

**Issue:** `Error: Anthropic API key is required`

```bash
# Unix/macOS/Linux
export ANTHROPIC_API_KEY="sk-ant-your-key-here"

# Windows (PowerShell)
$env:ANTHROPIC_API_KEY="sk-ant-your-key-here"

# Or use --api-key flag
python run_benchmark.py --provider anthropic --api-key "sk-ant-your-key-here"
```

### Framework Import Errors

**Issue:** `ModuleNotFoundError: No module named 'graphbit'`

```bash
# Ensure GraphBit is installed
pip install graphbit>=0.6.0

# Or with UV
uv pip install graphbit>=0.6.0

# Verify installation
python -c "import graphbit; print(graphbit.__version__)"
```

**Issue:** Framework-specific import errors (e.g., `crewai`, `langchain`)

```bash
# Reinstall all dependencies
# With UV:
uv sync --reinstall

# With Poetry:
poetry install --no-root --sync

# Verify all frameworks
python -c "import graphbit, langchain, crewai, pydantic_ai; print('All OK')"
```

**Issue:** `⚠ CrewAI not available: No module named 'crewai'`

```bash
# CrewAI requires Python 3.10-3.13
python --version

# If version is correct, reinstall
pip install crewai>=1.6.0

# Or with UV
uv pip install crewai>=1.6.0
```

### Docker Issues

**Issue:** Docker build fails

```bash
# Clean build (remove cache)
docker-compose build --no-cache

# Check Docker daemon is running
docker ps

# Verify .env file exists
ls -la .env  # Unix/macOS/Linux
dir .env     # Windows
```

**Issue:** Cannot mount volumes in Docker

```bash
# Ensure directories exist
mkdir -p logs results

# Check docker-compose.yml volume paths are correct
docker-compose config
```

**Issue:** `OPENAI_API_KEY` not available in Docker container

```bash
# Ensure .env file has the key
cat .env | grep OPENAI_API_KEY

# Or pass directly
docker-compose run -e OPENAI_API_KEY="sk-..." graphbit-benchmark run_benchmark.py
```

### Performance Issues

**Issue:** Benchmarks running very slowly

```bash
# Reduce number of runs
python run_benchmark.py --num-runs 3

# Run specific scenarios only
python run_benchmark.py --scenarios simple_task

# Pin to specific CPU cores
python run_benchmark.py --cpu-cores 0,1,2,3 --concurrency 2

# Check system resources
# Unix/macOS/Linux:
htop
# Windows:
Task Manager or: Get-Process | Sort-Object CPU -Descending | Select-Object -First 10
```

**Issue:** Out of memory errors

```bash
# Reduce concurrency
python run_benchmark.py --concurrency 2

# Run fewer scenarios at once
python run_benchmark.py --scenarios simple_task,sequential_pipeline

# Monitor memory usage
# Unix/macOS/Linux:
free -h
# Windows:
Get-WmiObject -Class Win32_OperatingSystem | Select-Object FreePhysicalMemory,TotalVisibleMemorySize
```

### Verification & Testing

**Check all installations:**

```bash
# Activate environment first
source .venv/bin/activate  # Unix/macOS/Linux
# or
.venv\Scripts\Activate.ps1  # Windows PowerShell

# Verify Python version
python --version  # Should be 3.10, 3.11, 3.12, or 3.13

# Verify all frameworks
python -c "
import graphbit; print(f'✓ GraphBit {graphbit.__version__}')
import langchain; print('✓ LangChain')
import langgraph; print('✓ LangGraph')
import autogen_agentchat; print('✓ AutoGen')
import crewai; print('✓ CrewAI')
import pydantic_ai; print('✓ PydanticAI')
import llama_index; print('✓ LlamaIndex')
print('All 7 frameworks OK!')
"
```

**Test benchmark script:**

```bash
# List available models
python run_benchmark.py --provider openai --list-models

# Show help
python run_benchmark.py --help

# Run a quick test (single scenario, single framework, 1 run)
python run_benchmark.py \
  --provider openai \
  --model gpt-4o-mini \
  --frameworks graphbit \
  --scenarios simple_task \
  --num-runs 1 \
  --verbose
```

### Getting Help

If you're still experiencing issues:

1. **Check logs:** Review `logs/{framework}.log` for detailed error messages
2. **Enable verbose mode:** Use `--verbose` flag for detailed output
3. **GitHub Issues:** Report bugs at [GraphBit GitHub Repository](https://github.com/InfinitiBit/graphbit/issues)
4. **Community Support:** Join our [Discord server](https://discord.com/invite/FMhgB3paMD)
5. **Documentation:** Visit [GraphBit Documentation](https://docs.graphbit.ai)

---
