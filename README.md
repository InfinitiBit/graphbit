<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="assets/Logo.png" width="140px" alt="Logo" />
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> | 
    <a href="https://graphbit-docs.vercel.app/docs">Docs</a> |
    <a href="https://discord.gg/8TvUK6uf">Discord</a> |
    <a href="https://docs.google.com/spreadsheets/d/1deQk0p7cCJUeeZw3t8FimxVg4jc99w0Bw1XQyLPN0Zk/edit?usp=sharing">Roadmap</a> 
    <br /><br />
</p>

[![Build Status](https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/python-integration-tests.yml?branch=main)](https://github.com/InfinitiBit/graphbit/actions/workflows/python-integration-tests.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md)
[![Test Coverage](https://img.shields.io/codecov/c/github/InfinitiBit/graphbit)](https://codecov.io/gh/InfinitiBit/graphbit)
[![Rust Coverage](https://img.shields.io/badge/Rust%20Coverage-47.25%25-yellow)](https://github.com/InfinitiBit/graphbit)
[![PyPI](https://img.shields.io/pypi/v/graphbit)](https://pypi.org/project/graphbit/)
[![Downloads](https://img.shields.io/pypi/dm/graphbit)](https://pypi.org/project/graphbit/)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Python Version](https://img.shields.io/pypi/pyversions/graphbit)](https://pypi.org/project/graphbit/)

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

Graphbit is an **industry-grade agentic AI framework** built for developers and AI teams that demand stability, scalability, and low resource usage. 

Written in **Rust** for maximum performance and safety, it delivers up to **68× lower CPU usage** and **140× lower memory** footprint than certain leading alternatives while consistently using far fewer resources than the rest, all while maintaining comparable throughput and execution speed. See [benchmarks](benchmarks/Report/Ai_LLM_Multi_Agent_Framework_Benchmark_Comparison_Report.md) for more details.

Designed to run **multi-agent workflows in parallel**, Graphbit persists memory across steps, recovers from failures, and ensures **100% task success** under load. Its lightweight, resource-efficient architecture enables deployment in both **high-scale enterprise environments** and **low-resource edge scenarios**. With built-in observability and concurrency support, Graphbit eliminates the bottlenecks that slow decision-making and erode ROI. 

##  Key Features

- **Tool Selection** - LLMs intelligently select tools based on descriptions
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
import os

from graphbit import LlmConfig, LlmClient, Executor, Workflow, Node
from graphbit.tools.decorators import tool

# Initialize and configure
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create executor
executor = Executor(config)

# Create tools with clear descriptions for LLM selection
@tool(description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(description="Perform mathematical calculations and return results")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Build workflow
workflow = Workflow("Analysis Pipeline")

analyzer = Node.agent(
    name="Smart Agent",
    prompt="What's the weather in Paris and calculate 15 + 27?",
    tools=[get_weather, calculate]
)

processor = Node.agent(
    name = "Data Processor",
    prompt = "Process and summarize the data from Data Analyzer."
)

agent_with_tools = graphbit.Node.agent(
    name="Weather Assistant",
    prompt= f"You are a weather assistant. Use tools when needed: {input}",
    agent_id="weather_agent",  # Optional, auto-generated if not provided
    tools=[get_weather]  # List of decorated tool functions
)

# Connect and execute
id1 = workflow.add_node(analyzer)
id2 = workflow.add_node(processor)
id3 = workflow.add_node(agent_with_tools)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_successful()}")
```

## Architecture

Three-tier design for reliability and performance:
- **Python API** - PyO3 bindings with async support
- **CLI Tool** - Project management and execution
- **Rust Core** - Workflow engine, agents, and LLM providers

## Python API Integrations

GraphBit provides a rich Python API for building and integrating agentic workflows, including executors, nodes, LLM clients, and embeddings. For the complete list of classes, methods, and usage examples, see the [Python API Reference](docs/api-reference/python-api.md).

## Contributing to GraphBit

We welcome contributions. To get started, please see the [Contributing](CONTRIBUTING.md) file for development setup and guidelines.


GraphBit is built by a wonderful community of researchers and engineers.

<a href="https://github.com/InfinitiBit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=InfinitiBit/graphbit&columns=10" />
</a> 
