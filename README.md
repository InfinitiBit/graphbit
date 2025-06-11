<div align="center">

# GraphBit - High Performance Agentic Framework

[![Build Status](https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/python-integration-tests.yml?branch=main)](https://github.com/InfinitiBit/graphbit/actions/workflows/python-integration-tests.yml)
[![Test Coverage](https://img.shields.io/codecov/c/github/InfinitiBit/graphbit)](https://codecov.io/gh/InfinitiBit/graphbit)
[![PyPI](https://img.shields.io/pypi/v/graphbit)](https://pypi.org/project/graphbit/)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Python Version](https://img.shields.io/pypi/pyversions/graphbit)](https://pypi.org/project/graphbit/)

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

A framework for building reliable AI agent workflows with strong type safety, comprehensive error handling, and predictable performance. Built with a Rust core and Python bindings.

##  Key Features

- **Type Safety** - Strong typing throughout the execution pipeline
- **Reliability** - Circuit breakers, retry policies, and error handling
- **Multi-LLM Support** - OpenAI, Anthropic, Ollama, HuggingFace
- **Resource Management** - Concurrency controls and memory optimization
- **Observability** - Built-in metrics and execution tracing

##  Quick Start

### Installation
```bash
pip install graphbit
```

### Environment Setup
First, set up your API keys:
```bash
# Copy the example environment file
cp .env.example .env

# Or set directly
export OPENAI_API_KEY=your_openai_api_key_here
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Security Note**: Never commit API keys to version control. Always use environment variables or secure secret management.

### Basic Usage
```python
import graphbit
import os

# Initialize and configure
graphbit.init()
config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create executor with reliability features
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(graphbit.PyRetryConfig.default()) \
    .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default())

# Build workflow
builder = graphbit.PyWorkflowBuilder("Analysis Pipeline")

analyzer = graphbit.PyWorkflowNode.agent_node(
    "Data Analyzer", 
    "Analyzes input data", 
    "analyzer-001",
    "Analyze the following data: {input}"
)

processor = graphbit.PyWorkflowNode.agent_node(
    "Data Processor",
    "Processes analyzed results",
    "processor-001", 
    "Process and summarize: {analyzed_data}"
)

# Connect and execute
id1 = builder.add_node(analyzer)
id2 = builder.add_node(processor)
builder.connect(id1, id2, graphbit.PyWorkflowEdge.data_flow())

workflow = builder.build()
result = executor.execute(workflow)
print(f"Completed in {result.execution_time_ms()}ms")
```

## 🏗️ Architecture

Three-tier design for reliability and performance:
- **Python API** - PyO3 bindings with async support
- **CLI Tool** - Project management and execution
- **Rust Core** - Workflow engine, agents, and LLM providers

## ⚙️ Configuration

```python
# High-throughput
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)

# Low-latency (fail fast)
executor = graphbit.PyWorkflowExecutor.new_low_latency(config).without_retries()

# Memory-optimized
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)
```

## Observability

```python
# Monitor execution
stats = await executor.get_concurrency_stats()
result = executor.execute(workflow)
print(f"Active tasks: {stats.current_active_tasks}")
print(f"Execution time: {result.execution_time_ms()}ms")
```

## Contributing

See the [Contributing](CONTRIBUTING.md) file for development setup and guidelines.
