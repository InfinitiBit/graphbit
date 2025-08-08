<div align="center">

# GraphBit - High Performance Agentic Framework


<p align="center">
    <img src="assets/Logo.png" width="140px" alt="Logo" />
</p>


<!-- Added placeholders for links, fill it up when the corresponding links are available. -->
<p align="center">
    <a href="https://graphbit.ai/">Website</a> |
    <a href=" ">Blog</a> |
    <a href="https://graphbit-docs.vercel.app/docs">Docs</a> 
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
config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")

# Create executor
executor = graphbit.Executor(config)

# Build workflow
workflow = graphbit.Workflow("Analysis Pipeline")

analyzer = graphbit.Node.agent(
    "Data Analyzer", 
    "Analyze the following data: {input}"
)

processor = graphbit.Node.agent(
    "Data Processor",
    "Process and summarize: {analyzed_data}"
)

# Connect and execute
id1 = workflow.add_node(analyzer)
id2 = workflow.add_node(processor)
workflow.connect(id1, id2)

result = executor.execute(workflow)
print(f"Workflow completed: {result.is_successful()}")
```

## Architecture

Three-tier design for reliability and performance:
- **Python API** - PyO3 bindings with async support
- **CLI Tool** - Project management and execution
- **Rust Core** - Workflow engine, agents, and LLM providers

## Observability

```python
# Monitor execution
stats = executor.get_stats()
result = executor.execute(workflow)
print(f"Total executions: {stats['total_executions']}")
print(f"Success rate: {stats['success_rate']:.2%}")
```

## Contributing to GraphBit

We welcome contributions. To get started, please see the [Contributing](CONTRIBUTING.md) file for development setup and guidelines.


GraphBit is built by a wonderful community of researchers and engineers.

<!-- Remove the following comment and add the code once the repository is public to make the contributors automated. -->
<!-- <a href="https://github.com/InfinitiBit/graphbit/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=InfinitiBit/graphbit&columns=10" />
</a> -->


<!-- Since repo is currently private, using manual way to view the current contributors. Remove the following section once the repo is public. -->
<img src="https://github.com/yeahia-sarker.png" width="40" style="border-radius: 50%" alt="yeahia-sarker" />
<img src="https://github.com/jj-devhub.png" width="40" style="border-radius: 50%" alt="jj-devhub" />
<img src="https://github.com/JunaidHossain04.png" width="40" style="border-radius: 50%" alt="JunaidHossain04" />
<img src="https://github.com/tanimahossain-infiniti.png" width="40" style="border-radius: 50%" alt="tanimahossain-infiniti" />
