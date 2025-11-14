<div align="center">

# GraphBit - High Performance Agentic Framework

<p align="center">
    <img src="assets/GraphBit_Final_GB_Github_GIF.gif" style="max-width: 100%; height: auto;" alt="Logo" />
</p>
<p align="center">
    <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=1004951&amp;theme=light&amp;t=1757340621693"> <img alt="GraphBit - Developer-first, enterprise-grade LLM framework. | Product Hunt" loading="lazy" width="250" height="54" decoding="async" data-nimg="1" class="w-auto h-[54px] max-w-[250px]" style="color:transparent" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=1004951&amp;theme=light&amp;period=daily&amp;t=1757933101511">
</p>

<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> |
    <a href="https://docs.graphbit.ai/">Docs</a> |
    <a href="https://discord.com/invite/huVJwkyu">Discord</a>
    <br /><br />
</p>

<p align="center">
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/v/graphbit?color=blue&label=PyPI" alt="PyPI"></a>
    <a href="https://pypi.org/project/graphbit/"><img src="https://img.shields.io/pypi/dm/graphbit?color=blue&label=Downloads" alt="PyPI Downloads"></a>
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

**Read this in other languages**: [🇨🇳 简体中文](README.zh-CN.md) | [🇨🇳 繁體中文](README.zh-TW.md) | [🇪🇸 Español](README.es.md) | [🇫🇷 Français](README.fr.md) | [🇩🇪 Deutsch](README.de.md) | [🇯🇵 日本語](README.ja.md) | [🇰🇷 한국어](README.ko.md) | [🇮🇳 हिन्दी](README.hi.md) | [🇸🇦 العربية](README.ar.md) | [🇮🇹 Italiano](README.it.md) | [🇧🇷 Português](README.pt-BR.md) | [🇷🇺 Русский](README.ru.md) | [🇧🇩 বাংলা](README.bn.md)

---

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

## Benchmark

GraphBit was built for efficiency at scale, not theoretical claims, but measured results.

Our internal benchmark suite compared GraphBit to leading Python-based agent frameworks across identical workloads.

| Metric              | GraphBit        | Other Frameworks | Gain                     |
|:--------------------|:---------------:|:----------------:|:-------------------------|
| CPU Usage           | 1.0× baseline   | 68.3× higher     | ~68× CPU                 |
| Memory Footprint    | 1.0× baseline   | 140× higher      | ~140× Memory             |
| Execution Speed     | ≈ equal / faster| —                | Consistent throughput    |
| Determinism         | 100% success    | Variable         | Guaranteed reliability   |

GraphBit consistently delivers production-grade efficiency across LLM calls, tool invocations, and multi-agent chains.

🎥 Watch the benchmark demo: Benchmark Video (coming soon)

## When to Use GraphBit

Choose GraphBit if you need:

- Production-grade multi-agent systems that won't collapse under load
- Type-safe execution and reproducible outputs
- Real-time orchestration for hybrid or streaming AI applications
- Rust-level efficiency with Python-level ergonomics

If you're scaling beyond prototypes or care about runtime determinism, GraphBit is for you.

##  Quick Start

### Installation

Recommended to use virtual environment.

```bash
pip install graphbit
```

### Quick Start Video Tutorial

<div align="center">
  <a href="https://youtu.be/ti0wbHFKKFM?si=hnxi-1W823z5I_zs">
    <img src="https://img.youtube.com/vi/ti0wbHFKKFM/maxresdefault.jpg" alt="GraphBit Quick Start Tutorial" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Watch the Install GraphBit via PyPI | Full Example & Run Guide
 tutorial</em></p>
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

### Building Your First Agent Workflow by GraphBit

<div align="center">
  <a href="https://www.youtube.com/watch?v=gKvkMc2qZcA">
    <img src="https://img.youtube.com/vi/gKvkMc2qZcA/maxresdefault.jpg" alt="Making Agent Workflow by GraphBit" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Watch the Making Agent Workflow by GraphBit tutorial</em></p>
</div>

## Observability & Tracing

GraphBit Tracer captures and monitors LLM calls and AI workflows with minimal configuration. It wraps GraphBit LLM clients and workflow executors to trace prompts, responses, token usage, latency, and errors without changing your code.

<div align="center">
  <a href="https://www.youtube.com/watch?v=nzwrxSiRl2U">
    <img src="https://img.youtube.com/vi/nzwrxSiRl2U/maxresdefault.jpg" alt="GraphBit Observability & Tracing" style="max-width: 100%; height: auto;">
  </a>
  <p><em>Watch the GraphBit Observability & Tracing tutorial</em></p>
</div>

## High-Level Architecture

<p align="center">
  <img src="assets/architecture.svg" height="250" alt="GraphBit Architecture">
</p>

Three-tier design for reliability and performance:
- **Rust Core** - Workflow engine, agents, and LLM providers
- **Orchestration Layer** - Project management and execution
- **Python API** - PyO3 bindings with async support

## Python API Integrations

GraphBit provides a rich Python API for building and integrating agentic workflows:

- **LLM Clients** - Multi-provider LLM integrations (OpenAI, Anthropic, Azure, and more)
- **Workflows** - Define and manage multi-agent workflow graphs with state management
- **Nodes** - Agent nodes, tool nodes, and custom workflow components
- **Executors** - Workflow execution engine with configuration management
- **Tool System** - Function decorators, registry, and execution framework for agent tools
- **Workflow Results** - Execution results with metadata, timing, and output access
- **Embeddings** - Vector embeddings for semantic search and retrieval
- **Workflow Context** - Shared state and variables across workflow execution
- **Document Loaders** - Load and parse documents from multiple formats (PDF, DOCX, TXT, JSON, CSV, XML, HTML)
- **Text Splitters** - Split documents into chunks (character, token, sentence, recursive)

For the complete list of classes, methods, and usage examples, see the [Python API Reference](docs/api-reference/python-api.md).

## Ecosystem & Extensions

GraphBit's modular architecture supports external integrations:

| Category          | Examples                                                                                      |
|:------------------|:----------------------------------------------------------------------------------------------|
| LLM Providers     | OpenAI, Anthropic, Azure OpenAI, DeepSeek, Together, Ollama, OpenRouter, Fireworks, Mistral AI, Replicate, Perplexity, HuggingFace, AI21, Bytedance, xAI, and more |
| Vector Stores     | Pinecone, Qdrant, Chroma, Milvus, Weaviate, FAISS, Elasticsearch, AstraDB, Redis, and more   |
| Databases         | PostgreSQL (PGVector), MongoDB, MariaDB, IBM DB2, Redis, and more                             |
| Cloud Platforms   | AWS (Boto3), Azure, Google Cloud Platform, and more                                           |
| Search APIs       | Serper, Google Search, GitHub Search, GitLab Search, and more                                 |
| Embedding Models  | OpenAI Embeddings, Voyage AI, and more                                                        |

Extensions are developed and maintained by the community.

<p align="center">
  <img src="assets/Ecosystem.png" alt="GraphBit Ecosystem - Stop Choosing, Start Orchestrating" style="max-width: 100%; height: auto;">
</p>

## Contributing to GraphBit

We welcome contributions. To get started, please see the [Contributing](CONTRIBUTING.md) file for development setup and guidelines.

GraphBit is built by a wonderful community of researchers and engineers.

<a href="https://github.com/Infinitibit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Infinitibit/graphbit" />
</a>

## Security

GraphBit is committed to maintaining security standards for our agentic framework. We recommend using environment variables for API keys, keeping GraphBit updated, and using proper secret management for production environments. If you discover a security vulnerability, please report it responsibly through GitHub Security or via email rather than creating a public issue.

For detailed reporting procedures and response timelines, see our [Security Policy](SECURITY.md).

## License

GraphBit is licensed under a three-tier model: **Model A (Free Use)** for individuals, academic institutions, and small teams (up to 10 employees/users), **Model B (Free Trial)** for 30-day evaluation, and **Model C (Enterprise)** for commercial/production use. Redistribution is prohibited under all models without an explicit Enterprise License.

For complete terms and conditions, see the [Full License](LICENSE.md).

Copyright © 2023–2025 InfinitiBit GmbH. All rights reserved.
