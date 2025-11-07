<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="assets/logo(circle).png" width="160px" alt="Logo" />
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> | 
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/huVJwkyu">Discord</a>
    <br /><br />
</p>

[![Build Status](https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main)](https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Python Version](https://img.shields.io/badge/python-3.10--3.13-blue.svg)](https://www.python.org)

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

GraphBit is an open-source agentic AI framework for developers who need deterministic, concurrent, and low-overhead execution. 

## Why GraphBit? 
Efficiency decides who scales, GraphBit is built for developers who need deterministic, concurrent, and ultra-efficient AI execution without the overhead. 

Built with a Rust core and a minimal Python layer, GraphBit delivers up to 68× lower CPU usage and 140× lower memory footprint than other frameworks, while maintaining equal or greater throughput. 

It powers multi-agent workflows that run in parallel, persist memory across steps, self-recover from failures, and ensure 100 % task reliability. 
GraphBit is built for production workloads, from enterprise AI systems to low-resource edge deployments.  


##  Key Features

- **Tool Selection** - LLMs intelligently choose tools based on descriptions 
- **Type Safety** -  Strong typing through every execution layer
- **Reliability** - Circuit breakers, retry policies, and error handling and fault recovery 
- **Multi-LLM Support** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI and more
- **Resource Management** - Concurrency controls and memory optimization
- **Observability** - Built-in tracing, structured logs, and performance metrics

##  Quick Start

### Installation 

Recommended to use virtual environment.

```bash
pip install graphbit
```

### Quick Start Video Tutorial

Watch this quick video guide to get started with GraphBit:

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Click the image above to watch the Quick Start tutorial on YouTube</em></p>
</div>

### Environment Setup

Set up API keys you want to use in your project:
```bash
# OpenAI (optional – required if using OpenAI models)
export OPENAI_API_KEY=your_openai_api_key_here

# Anthropic (optional – required if using Anthropic models)
export ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

> **Security Note**: Never commit API keys to version control. Always use environment variables or secure secret management.

### Basic Usage
```python
import os

from graphbit import LlmConfig, Executor, Workflow, Node, tool

# Initialize and configure
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create executor
executor = Executor(config)

# Create tools with clear descriptions for LLM selection
@tool(_description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}

@tool(_description="Perform mathematical calculations and return results")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"

# Build workflow
workflow = Workflow("Analysis Pipeline")

# Create agent nodes
smart_agent = Node.agent(
    name="Smart Agent",
    prompt="What's the weather in Paris and calculate 15 + 27?",
    system_prompt="You are an assistant skilled in weather lookup and math calculations. Use tools to answer queries accurately.",
    tools=[get_weather, calculate]
)

processor = Node.agent(
    name="Data Processor",
    prompt="Process the results obtained from Smart Agent.",
    system_prompt="""You process and organize results from other agents.

    - Summarize and clarify key points
    - Structure your output for easy reading
    - Focus on actionable insights
    """
)

# Connect and execute
id1 = workflow.add_node(smart_agent)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_success()}")
print("\nSmart Agent Output: \n", result.get_node_output("Smart Agent"))
print("\nData Processor Output: \n", result.get_node_output("Data Processor"))
```

## High-Level Architecture

<p align="center">
  <img src="assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Three-tier design for reliability and performance:
- **Rust Core** - Workflow engine, agents, and LLM providers
- **Orchestration Layer** - Project management and execution
- **Python API** - PyO3 bindings with async support

## Python API Integrations

GraphBit provides a rich Python API for building and integrating agentic workflows, including executors, nodes, LLM clients, and embeddings. For the complete list of classes, methods, and usage examples, see the [Python API Reference](docs/api-reference/python-api.md).

## Contributing to GraphBit

We welcome contributions. To get started, please see the [Contributing](CONTRIBUTING.md) file for development setup and guidelines.

GraphBit is built by a wonderful community of researchers and engineers.

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>
