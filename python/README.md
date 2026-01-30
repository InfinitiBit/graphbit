<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="https://raw.githubusercontent.com/InfinitiBit/graphbit/refs/heads/main/assets/logo(circle).png" width="180" alt="GraphBit Logo" />
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> | 
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/FMhgB3paMD">Discord</a>
    <br /><br />
</p>

<p align="center">
    <a href="https://trendshift.io/repositories/14884" target="_blank"><img src="https://trendshift.io/api/badge/repositories/14884" alt="InfinitiBit%2Fgraphbit | Trendshift" width="250" height="55"/></a>
    <br>
    <a href="https://pepy.tech/projects/graphbit"><img src="https://static.pepy.tech/personalized-badge/graphbit?period=total&units=INTERNATIONAL_SYSTEM&left_color=GREY&right_color=GREEN&left_text=Downloads" alt="PyPI Downloads"/></a>
</p>

<p align="center">
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/v/graphbit?color=blue&label=PyPI" alt="PyPI"></a>
    <!-- <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/dm/graphbit?color=blue&label=Downloads" alt="PyPI Downloads"></a> -->
    <a href="https://github.com/InfinitiBit/graphbit/actions/workflows/update-docs.yml"><img src="https://img.shields.io/github/actions/workflow/status/InfinitiBit/graphbit/update-docs.yml?branch=main&label=Build" alt="Build Status"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
    <br>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-orange.svg?logo=rust" alt="Rust Version"></a>
    <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.9--3.13-blue.svg?logo=python&logoColor=white" alt="Python Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/FMhgB3paMD"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

---

**Read this in other languages**: [🇨🇳 简体中文](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.zh-CN.md) | [🇨🇳 繁體中文](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.zh-TW.md) | [🇪🇸 Español](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.es.md) | [🇫🇷 Français](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.fr.md) | [🇩🇪 Deutsch](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.de.md) | [🇯🇵 日本語](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.ja.md) | [🇰🇷 한국어](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.ko.md) | [🇮🇳 हिन्दी](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.hi.md) | [🇸🇦 العربية](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.ar.md) | [🇮🇹 Italiano](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.it.md) | [🇧🇷 Português](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.pt-BR.md) | [🇷🇺 Русский](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.ru.md) | [🇧🇩 বাংলা](https://github.com/InfinitiBit/graphbit/blob/main/README_Multi_Lingual_i18n_Files/README.bn.md)

---

## What is GraphBit?

GraphBit is a source-available agentic AI framework for developers who need deterministic, concurrent, and low-overhead execution.

Built with a **Rust core** and a minimal **Python layer**, GraphBit delivers up to **68× lower CPU usage** and **140× lower memory footprint** than other frameworks, while maintaining equal or greater throughput.

It powers multi-agent workflows that run in parallel, persist memory across steps, self-recover from failures, and ensure **100% task reliability**. GraphBit is built for production workloads, from enterprise AI systems to low-resource edge deployments.

---

## Why GraphBit?

Efficiency decides who scales. GraphBit is built for developers who need deterministic, concurrent, and ultra-efficient AI execution without the overhead.

- **Rust-Powered Performance** - Maximum speed and memory safety at the core
- **Production-Ready** - Circuit breakers, retry policies, and fault recovery built-in
- **Resource Efficient** - Run on enterprise servers or low-resource edge devices
- **Multi-Agent Ready** - Parallel execution with shared memory across workflow steps
- **Observable** - Built-in tracing, structured logs, and performance metrics

---

## Benchmark

GraphBit was built for efficiency at scale—not theoretical claims, but measured results.

Our internal benchmark suite compared GraphBit to leading Python-based agent frameworks across identical workloads.

| Metric              | GraphBit        | Other Frameworks | Gain                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU Usage           | 1.0× baseline   | 68.3× higher     | ~68× CPU                 |
| Memory Footprint    | 1.0× baseline   | 140× higher      | ~140× Memory             |
| Execution Speed     | ≈ equal / faster| —                | Consistent throughput    |
| Determinism         | 100% success    | Variable         | Guaranteed reliability   |

GraphBit consistently delivers production-grade efficiency across LLM calls, tool invocations, and multi-agent chains.

**[View Full Benchmark Report](https://github.com/InfinitiBit/graphbit/blob/main/benchmarks/report/framework-benchmark-report.md)** for detailed methodology, test scenarios, and complete results.

**[Watch Benchmark Demo Video](https://www.youtube.com/watch?v=MaCl5oENeAY)** to see GraphBit's performance in action.

---

## When to Use GraphBit

Choose GraphBit if you need:

- **Production-grade multi-agent systems** that won't collapse under load
- **Type-safe execution** and reproducible outputs
- **Real-time orchestration** for hybrid or streaming AI applications
- **Rust-level efficiency** with Python-level ergonomics

If you're scaling beyond prototypes or care about runtime determinism, GraphBit is for you.

---

## Key Features

- **Tool Selection** - LLMs intelligently choose tools based on descriptions
- **Type Safety** - Strong typing through every execution layer
- **Reliability** - Circuit breakers, retry policies, error handling and fault recovery
- **Multi-LLM Support** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI and more
- **Resource Management** - Concurrency controls and memory optimization
- **Observability** - Built-in tracing, structured logs, and performance metrics

---

## Quick Start

### Installation

Recommended to use virtual environment.

```bash
pip install graphbit
```

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

**[Watch Quick Start Video Tutorial](https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs)** - Complete walkthrough: Install GraphBit via PyPI, setup, and run your first workflow

**[Watch: Building Your First Agent Workflow](https://www.youtube.com/watch?v=gKvkMc2qZcA)** - Advanced tutorial on creating multi-agent workflows with GraphBit

---

## High-Level Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/InfinitiBit/graphbit/092af98af8bd0f00ec924d3879dbcb98353cfd8d/assets/architecture.svg" width="600" alt="GraphBit Architecture">
</p>

Three-tier design for reliability and performance:
- **Rust Core** - Workflow engine, agents, and LLM providers
- **Orchestration Layer** - Project management and execution
- **Python API** - PyO3 bindings with async support

---

## Python API Integrations

GraphBit provides a rich Python API for building and integrating agentic workflows:

- **LLM Clients** - Multi-provider integrations (OpenAI, Anthropic, Azure, and more)
- **Workflows** - Define and manage multi-agent workflow graphs with state management
- **Nodes** - Agent nodes, tool nodes, and custom workflow components
- **Executors** - Workflow execution engine with configuration management
- **Tool System** - Function decorators, registry, and execution framework
- **Embeddings** - Vector embeddings for semantic search and retrieval
- **Document Loaders** - Load and parse documents (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Text Splitters** - Split documents into chunks (character, token, sentence, recursive)

For the complete list of classes, methods, and usage examples, see the [Python API Reference](https://docs.graphbit.ai/).

**[Watch: GraphBit Observability & Tracing](https://www.youtube.com/watch?v=nzwrxSiRl2U)** - Learn how to monitor and trace your AI workflows

---

## Contributing to GraphBit

We welcome contributions. To get started, please see the [Contributing](https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md) file for development setup and guidelines.

GraphBit is built by a wonderful community of researchers and engineers.

---

## Security

GraphBit is committed to maintaining security standards for our agentic framework. We recommend:

- Using environment variables for API keys
- Keeping GraphBit updated to the latest version
- Using proper secret management for production environments

If you discover a security vulnerability, please report it responsibly through [GitHub Security](https://github.com/InfinitiBit/graphbit/security) or via email rather than creating a public issue.

For detailed reporting procedures and response timelines, see our [Security Policy](https://github.com/InfinitiBit/graphbit/blob/main/SECURITY.md).

---

## License

GraphBit package is licensed under the Apache License, Version 2.0.

For complete terms and conditions, see the [Full License](https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md).

---

<div align="center">

**Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.**

</div>
