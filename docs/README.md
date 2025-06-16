# GraphBit Documentation

Welcome to the comprehensive documentation for **GraphBit** - a high-performance AI agent workflow automation framework that combines Rust's performance with Python's ease of use.

## Quick Navigation

### 🚀 Getting Started
- [Installation Guide](getting-started/installation.md) - Install GraphBit on your system
- [Quick Start Tutorial](getting-started/quickstart.md) - Build your first workflow in 5 minutes
- [Basic Examples](getting-started/examples.md) - Simple examples to get you started

### 📚 User Guide
- [Core Concepts](user-guide/concepts.md) - Understand workflows, agents, and nodes
- [Workflow Builder](user-guide/workflow-builder.md) - Creating and connecting workflow nodes
- [Agent Configuration](user-guide/agents.md) - Setting up AI agents with different capabilities
- [LLM Providers](user-guide/llm-providers.md) - Working with OpenAI, Anthropic, Ollama, and more
- [Dynamic Graph Generation](user-guide/dynamics-graph.md) - Auto-generating workflow structures
- [Data Validation](user-guide/validation.md) - Input validation and data quality checks
- [Performance Optimization](user-guide/performance.md) - Tuning for speed and efficiency
- [Monitoring & Observability](user-guide/monitoring.md) - Metrics collection and debugging
- [Embeddings & Vector Search](user-guide/embeddings.md) - Text embeddings and similarity search
- [Reliability & Fault Tolerance](user-guide/reliability.md) - Circuit breakers, retries, and error handling

### 🔧 API Reference
- [Python API](api-reference/python-api.md) - Complete Python API documentation
- [Configuration Options](api-reference/configuration.md) - All configuration parameters
- [Node Types](api-reference/node-types.md) - Agent, condition, transform, and delay nodes
- [Execution Patterns](api-reference/execution.md) - Sync, async, batch, and concurrent execution

### 🎯 Advanced Topics
- [Custom Extensions](advanced/extensions.md) - Building custom node types and providers
- [Plugin Development](advanced/plugins.md) - Creating GraphBit plugins
- [Advanced Patterns](advanced/patterns.md) - Complex workflow design patterns
- [Integration Guides](advanced/integrations.md) - Connecting with external systems

### 🛠️ Development
- [Architecture Overview](development/architecture.md) - System design and components
- [Contributing Guide](development/contributing.md) - How to contribute to GraphBit
- [Building from Source](development/building.md) - Development setup and compilation
- [Testing](development/testing.md) - Running tests and quality checks

### 📋 Examples & Use Cases
- [Content Generation Pipeline](examples/content-generation.md) - Multi-agent content creation
- [Data Processing Workflow](examples/data-processing.md) - ETL pipelines with AI agents
- [Code Review Automation](examples/code-review.md) - Automated code analysis
- [Customer Support Bot](examples/customer-support.md) - Multi-stage inquiry processing

## What is GraphBit?

GraphBit is a declarative framework for building reliable AI agent workflows with strong type safety, comprehensive error handling, and predictable performance. It features:

- **🔒 Type Safety** - Strong typing throughout the execution pipeline
- **🛡️ Reliability** - Circuit breakers, retry policies, and error handling  
- **🤖 Multi-LLM Support** - OpenAI, Anthropic, Ollama, HuggingFace
- **⚡ Performance** - Rust core with Python bindings for optimal speed
- **📊 Observability** - Built-in metrics and execution tracing
- **🔧 Resource Management** - Concurrency controls and memory optimization

## Architecture

GraphBit uses a three-tier architecture:

```
┌─────────────────┐
│   Python API    │  ← PyO3 bindings with async support
├─────────────────┤
│   CLI Tool      │  ← Project management and execution
├─────────────────┤
│   Rust Core     │  ← Workflow engine, agents, LLM providers
└─────────────────┘
```

## Community & Support

- **GitHub**: [github.com/InfinitiBit/graphbit](https://github.com/InfinitiBit/graphbit)
- **Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Contributing**: See our [contributing guide](development/contributing.md)

## License

GraphBit is released under the [Proprietary License](../LICENSE).

---

*Ready to build your first AI workflow? Start with our [Quick Start Tutorial](getting-started/quickstart.md)!* 