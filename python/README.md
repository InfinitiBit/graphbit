<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="https://raw.githubusercontent.com/InfinitiBit/graphbit/refs/heads/main/assets/logo(circle).png" style="max-width: 200px; height: auto;" alt="GraphBit Logo" />
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> | 
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/huVJwkyu">Discord</a>
    <br /><br />
</p>

<p align="center">
    <a href="https://trendshift.io/repositories/14884" target="_blank"><img src="https://trendshift.io/api/badge/repositories/14884" alt="InfinitiBit%2Fgraphbit | Trendshift" style="width: 250px; height: 55px;" width="250" height="55"/></a>
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
    <a href="https://www.python.org"><img src="https://img.shields.io/badge/python-3.10--3.13-blue.svg?logo=python&logoColor=white" alt="Python Version"></a>
    <a href="https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/license-Custom-lightgrey.svg" alt="License"></a>

</p>
<p align="center">
    <a href="https://www.youtube.com/@graphbitAI"><img src="https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white" alt="YouTube"></a>
    <a href="https://x.com/graphbit_ai"><img src="https://img.shields.io/badge/X-000000?logo=x&logoColor=white" alt="X"></a>
    <a href="https://discord.com/invite/huVJwkyu"><img src="https://img.shields.io/badge/Discord-7289da?logo=discord&logoColor=white" alt="Discord"></a>
    <a href="https://www.linkedin.com/showcase/graphbitai/"><img src="https://img.shields.io/badge/LinkedIn-0077B5?logo=linkedin&logoColor=white" alt="LinkedIn"></a>
</p>

**Type-Safe AI Agent Workflows with Rust Performance**

</div>

---

**Read this in other languages**: [🇨🇳 简体中文](README_Multi_Lingual_i18n_Files/README.zh-CN.md) | [🇨🇳 繁體中文](README_Multi_Lingual_i18n_Files/README.zh-TW.md) | [🇪🇸 Español](README_Multi_Lingual_i18n_Files/README.es.md) | [🇫🇷 Français](README_Multi_Lingual_i18n_Files/README.fr.md) | [🇩🇪 Deutsch](README_Multi_Lingual_i18n_Files/README.de.md) | [🇯🇵 日本語](README_Multi_Lingual_i18n_Files/README.ja.md) | [🇰🇷 한국어](README_Multi_Lingual_i18n_Files/README.ko.md) | [🇮🇳 हिन्दी](README_Multi_Lingual_i18n_Files/README.hi.md) | [🇸🇦 العربية](README_Multi_Lingual_i18n_Files/README.ar.md) | [🇮🇹 Italiano](README_Multi_Lingual_i18n_Files/README.it.md) | [🇧🇷 Português](README_Multi_Lingual_i18n_Files/README.pt-BR.md) | [🇷🇺 Русский](README_Multi_Lingual_i18n_Files/README.ru.md) | [🇧🇩 বাংলা](README_Multi_Lingual_i18n_Files/README.bn.md)

---


Graphbit is an **industry-grade agentic AI framework** built for developers and AI teams that demand stability, scalability, and low resource usage. 

Written in **Rust** for maximum performance and safety, it delivers up to **68× lower CPU usage** and **140× lower memory** footprint than certain leading alternatives while consistently using far fewer resources than the rest, all while maintaining comparable throughput and execution speed. See [benchmarks](benchmarks/report/framework-benchmark-report.md) for more details.

Designed to run **multi-agent workflows in parallel**, Graphbit persists memory across steps, recovers from failures, and ensures **100% task success** under load. Its lightweight, resource-efficient architecture enables deployment in both **high-scale enterprise environments** and **low-resource edge scenarios**. With built-in observability and concurrency support, Graphbit eliminates the bottlenecks that slow decision-making and erode ROI. 

##  Key Features

- **Tool Selection** - LLMs intelligently select tools based on descriptions
- **Type Safety** - Strong typing throughout the execution pipeline
- **Reliability** - Circuit breakers, retry policies, and error handling
- **Multi-LLM Support** - OpenAI, Azure OpenAI, Anthropic, OpenRouter, DeepSeek, Replicate, Ollama, TogetherAI
- **Resource Management** - Concurrency controls and memory optimization
- **Observability** - Built-in metrics and execution tracing

##  Quick Start

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

## High-Level Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/InfinitiBit/graphbit/092af98af8bd0f00ec924d3879dbcb98353cfd8d/assets/architecture.svg" height="250" alt="GraphBit Architecture">
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

