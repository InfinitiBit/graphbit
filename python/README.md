# GraphBit Python API

Python bindings for GraphBit - a declarative agentic workflow automation framework.

## Installation

### From PyPI (Recommended)

```bash
pip install graphbit
```

### From Source

```bash
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/
cargo build --release
cd python/
cargo build --release
maturin develop --release
```

## Quick Start

```python
import graphbit
import os

graphbit.init()

api_key = os.getenv("OPENAI_API_KEY")
config = graphbit.LlmConfig.openai(api_key, "gpt-4")

workflow = graphbit.Workflow("Hello World")

node = graphbit.Node.agent(
    "Greeter", 
    "Say hello to the world!", 
    "greeter"
)

node_id = workflow.add_node(node)
workflow.validate()

executor = graphbit.Executor(config)
context = executor.execute(workflow)

print(f"Result: {context.get_variable('node_result_1')}")
```

## Core Classes

### LlmConfig

Configuration for LLM providers.

```python
openai_config = graphbit.LlmConfig.openai(api_key, "gpt-4")

anthropic_config = graphbit.LlmConfig.anthropic(api_key, "claude-3-sonnet-20240229")

fireworks_config = graphbit.LlmConfig.fireworks(api_key, "accounts/fireworks/models/llama-v3p1-8b-instruct")
```

### EmbeddingConfig & EmbeddingClient

Service for generating text embeddings.

```python
config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-ada-002")
embedding_client = graphbit.EmbeddingClient(config)

# Single embedding
embedding = embedding_client.embed("Hello world")

# Batch embedding
embeddings = embedding_client.embed_many(["First text", "Second text"])
```

### Workflow

Pattern for creating workflows.

```python
workflow = graphbit.Workflow("My Workflow")

agent_node = graphbit.Node.agent(
    name="Analyzer", 
    prompt=f"Analyzes input data: {input}",
    agent_id="analyzer"
)

node_id = workflow.add_node(agent_node)

transform_node = graphbit.Node.transform(
    "Uppercase",
    "uppercase"
)

transform_id = workflow.add_node(transform_node)

workflow.connect(node_id, transform_id)

workflow.validate()
```

### Node

Factory for creating different types of workflow nodes.

```python
# Agent node
agent = graphbit.Node.agent(
    "Content Writer", 
    f"Write an article about {topic}", 
    "writer"
)

# Transform node
transform = graphbit.Node.transform(
    "Uppercase", 
    "uppercase"
)
# Available transformations: "uppercase", "lowercase", "json_extract", "split", "join"

# Condition node
condition = graphbit.Node.condition(
    "Quality Check", 
    "quality_score > 0.8"
)
```

### DocumentLoaderConfig & DocumentLoader

Service for loading documents

```python
from graphbit import DocumentLoaderConfig, DocumentLoader

# Basic configuration
config = DocumentLoaderConfig(
    max_file_size=50_000_000,    # 50MB limit
    default_encoding="utf-8",    # Text encoding
    preserve_formatting=True     # Keep formatting
)
loader = DocumentLoader(config)

# PDF processing
config = DocumentLoaderConfig(preserve_formatting=True)
config.extraction_settings = {
    "ocr_enabled": True,
    "table_detection": True
}
loader = DocumentLoader(config)
content = loader.load_document("report.pdf", "pdf")

# Text files processing
config = DocumentLoaderConfig(default_encoding="utf-8")
loader = DocumentLoader(config)
content = loader.load_document("notes.txt", "txt")

# Structured data processing
# JSON, CSV, XML automatically parsed as text
loader = DocumentLoader()
json_content = loader.load_document("data.json", "json")
csv_content = loader.load_document("data.csv", "csv")
```

### TextSplitterConfig & TextSplitter

Factory for creating different types of test splitters.

```python
# Character configuration
config = graphbit.TextSplitterConfig.character(
    chunk_size=1000,
    chunk_overlap=200
)

# Token configuration
config = graphbit.TextSplitterConfig.token(
    chunk_size=100,
    chunk_overlap=20,
    token_pattern=r'\w+'
)

# Code splitter configuration
config = graphbit.TextSplitterConfig.code(
    chunk_size=500,
    chunk_overlap=50,
    language="python"
)

# Markdown splitter configuration
config = graphbit.TextSplitterConfig.markdown(
    chunk_size=1000,
    chunk_overlap=100,
    split_by_headers=True
)

# Create a character splitter
splitter = graphbit.CharacterSplitter(
    chunk_size=1000,      # Maximum characters per chunk
    chunk_overlap=200     # Overlap between chunks
)

# Create a token splitter
splitter = graphbit.TokenSplitter(
    chunk_size=100,       # Maximum tokens per chunk
    chunk_overlap=20,     # Token overlap
    token_pattern=None    # Optional custom regex pattern
)

# Create a sentence splitter
splitter = graphbit.SentenceSplitter(
    chunk_size=500,       # Target size in characters
    chunk_overlap=1       # Number of sentences to overlap
    sentence_endings=None # Optional list of sentence endings
)

# Create a recursive splitter
splitter = graphbit.RecursiveSplitter(
    chunk_size=1000,      # Target size in characters
    chunk_overlap=100     # Overlap between chunks
    separators=None       # Optional list of separators

)
```

### Executor

Executes workflows with the configured LLM.

```python
# Basic executor configuration
executor = graphbit.Executor(openai_config)

# High-throughput executor configuration
high_throughput_executor = graphbit.Executor.new_high_throughput(config)

# Low-latency executor configuration
low_latency_executor = graphbit.Executor.new_low_latency(config)

# Memory-optimized executor configuration
memory_optimized_executor = graphbit.Executor.new_memory_optimized(config)

context = executor.execute(workflow)

print(f"Workflow Status: {context.state()}")
print(f"Workflow Success: {context.is_success()}")
print(f"Workflow Failed: {context.is_failed()}")
print(f"Workflow Execution time: {context.execution_time_ms()}ms")
print(f"All variables: {context.get_all_variables()}")
```

## API Reference Summary

### Core Classes
- `LlmConfig`: LLM provider configuration
- `EmbeddingConfig`: Embedding service configuration
- `EmbeddingClient`: Text embedding generation
- `Node`: Workflow node factory (agent, transform, condition)
- `Workflow`: Workflow definition and operations
- `DocumentLoaderConfig`: Documents loading service configuration
- `DocumentLoader`: Loading documents 
- `TextSplitterConfig`: Text splitter service configuration
- `TextSplitter`: Text splitters for text processing
- `Executor`: Workflow execution engine

This API provides everything needed to build agentic workflows with Python, from simple automation to multi-step pipelines.
