# Dependency Installation Guide

This guide explains how to install GraphBit with different dependency configurations based on your specific needs and use cases.

## Overview

GraphBit uses a modular dependency system that allows you to install only the components you need:

- **Core dependencies**: Essential packages required for basic GraphBit functionality
- **Optional dependencies**: Additional packages grouped by functionality (AI providers, vector databases, benchmarking tools)
- **Extras groups**: Predefined collections of optional dependencies for common use cases

---

## Core Installation (Minimal Dependencies)

The core installation includes only the essential dependencies needed for basic GraphBit functionality.

### What's Included in Core

- `python-dotenv` - Environment variable management
- `rich` - Rich text and beautiful formatting
- `typer` - CLI framework
- `aiofiles` - Async file operations

### Installation Commands

```bash
# Using pip (recommended)
pip install graphbit

# Using Poetry
poetry add graphbit

# Using uv
uv add graphbit
```

### Core Installation Use Cases

- **Lightweight deployments** where you only need basic workflow functionality
- **Custom integrations** where you'll add your own AI providers or databases
- **Minimal Docker images** for production environments
- **Development environments** where you want to add dependencies incrementally

---

## Optional Dependency Groups (Extras)

GraphBit provides several extras groups that bundle related optional dependencies for specific use cases.

### Available Extras Groups

#### `benchmark` - Benchmarking and Performance Testing
Includes comprehensive tools for performance analysis and AI framework comparisons.

**What's included:**
- **Visualization**: `matplotlib`, `seaborn`, `pandas`, `numpy`
- **Performance monitoring**: `psutil`, `memory-profiler`, `tabulate`
- **AI frameworks**: `anthropic`, `openai`, `langchain`, `langchain-openai`, `langchain-anthropic`, `langchain-ollama`, `langgraph`, `llama-index`, `llama-index-llms-anthropic`, `llama-index-cli`, `crewai`, `pydantic-ai`
- **CLI tools**: `click`

**Installation:**
```bash
# Using pip
pip install "graphbit[benchmark]"

# Using Poetry
poetry add "graphbit[benchmark]"

# Using uv
uv add "graphbit[benchmark]"
```

**Use cases:**
- Performance benchmarking against other AI frameworks
- Analyzing workflow execution metrics
- Comparing different LLM providers
- Research and development environments

#### `full` - Complete Installation
Includes all available optional dependencies for maximum functionality.

**What's included:**
- All dependencies from `benchmark` group
- All vector database connectors
- All AI providers currently supported by GraphBit.
- All utility libraries

**Installation:**
```bash
# Using pip
pip install "graphbit[full]"

# Using Poetry
poetry add "graphbit[full]"

# Using uv
uv add "graphbit[full]"
```

**Use cases:**
- Development environments where you need access to all features
- Prototyping with multiple AI providers and databases
- Educational or training environments
- One-stop installation for comprehensive GraphBit usage

#### Vector Database Extras

##### `chromadb` - ChromaDB Integration
```bash
pip install "graphbit[chromadb]"
```
**Includes**: `chromadb`
**Use case**: Document similarity search, RAG applications

##### `pinecone` - Pinecone Integration
```bash
pip install "graphbit[pinecone]"
```
**Includes**: `pinecone`
**Use case**: Managed vector database for production applications

##### `qdrant` - Qdrant Integration
```bash
pip install "graphbit[qdrant]"
```
**Includes**: `qdrant-client`
**Use case**: Open-source vector database with advanced filtering

##### `weaviate` - Weaviate Integration
```bash
pip install "graphbit[weaviate]"
```
**Includes**: `weaviate-client`
**Use case**: Knowledge graphs and semantic search

##### `milvus` - Milvus Integration
```bash
pip install "graphbit[milvus]"
```
**Includes**: `pymilvus`
**Use case**: Large-scale vector similarity search

##### `faiss` - FAISS Integration
```bash
pip install "graphbit[faiss]"
```
**Includes**: `faiss-cpu`, `numpy`
**Use case**: High-performance similarity search and clustering

##### `elasticsearch` - Elasticsearch Integration
```bash
pip install "graphbit[elasticsearch]"
```
**Includes**: `elasticsearch`
**Use case**: Full-text search with vector capabilities

#### Database Extras

##### `pgvector` - PostgreSQL with pgvector
```bash
pip install "graphbit[pgvector]"
```
**Includes**: `psycopg2`
**Use case**: PostgreSQL with vector extensions

##### `mariadb` - MariaDB Integration
```bash
pip install "graphbit[mariadb]"
```
**Includes**: `mariadb`, `numpy`
**Use case**: MariaDB with vector capabilities

##### `mongodb` - MongoDB Integration
```bash
pip install "graphbit[mongodb]"
```
**Includes**: `pymongo`
**Use case**: Document database with vector search

##### `db2` - IBM Db2 Integration
```bash
pip install "graphbit[db2]"
```
**Includes**: `ibm-db`, `numpy`
**Use case**: Enterprise IBM Db2 database

#### Cloud Provider Extras

##### `boto3` - Amazon Web Services
```bash
pip install "graphbit[boto3]"
```
**Includes**: `boto3`
**Use case**: AWS services integration

##### `astradb` - DataStax Astra DB
```bash
pip install "graphbit[astradb]"
```
**Includes**: `astrapy`
**Use case**: Managed Cassandra with vector search

---

## Multiple Extras Installation

You can install multiple extras groups simultaneously:

```bash
# Install multiple specific extras
pip install "graphbit[chromadb,boto3]"

# Install benchmark tools with specific database
pip install "graphbit[benchmark,pgvector]"

# Install full AI stack with vector databases
pip install "graphbit[benchmark,chromadb,pinecone,qdrant]"
```

---

## Installation Scenarios by Use Case

### Scenario 1: Basic AI Workflow Development
```bash
# Minimal installation for simple workflows
pip install graphbit
```

### Scenario 2: RAG Application Development
```bash
# Vector database
pip install "graphbit[chromadb]"

# Alternative with managed vector DB
pip install "graphbit[pinecone]"
```

### Scenario 3: Enterprise Production Environment
```bash
# PostgreSQL + Boto3
pip install "graphbit[pgvector,boto3]"

# IBM enterprise stack
pip install "graphbit[db2,boto3]"
```

### Scenario 4: Research and Benchmarking
```bash
# Full benchmarking suite
pip install "graphbit[benchmark]"

# Comprehensive research environment
pip install "graphbit[full]"
```

### Scenario 5: Multi-Cloud Development
```bash
# Multiple vector databases for testing
pip install "graphbit[chromadb,pinecone,qdrant,weaviate]"
```

---

## Dependency Comparison Table

| Extra Group | AI Providers | Vector DBs | Visualization | Performance | Cloud | Size |
|-------------|--------------|------------|---------------|-------------|-------|------|
| Core | ❌ | ❌ | ❌ | ❌ | ❌ | Minimal |
| `benchmark` | ✅ All | ❌ | ✅ Full | ✅ Full | ❌ | Large |
| `chromadb` | ❌ | ChromaDB | ❌ | ❌ | ❌ | Small |
| `pinecone` | ❌ | Pinecone | ❌ | ❌ | ❌ | Small |
| `pgvector` | ❌ | PostgreSQL | ❌ | ❌ | ❌ | Small |
| `boto3` | ❌ | ❌ | ❌ | ❌ | AWS | Small |
| `full` | ✅ All | ✅ All | ✅ Full | ✅ Full | ✅ All | Very Large |

---

## Development vs Production Recommendations

### Development Environment
```bash
# Option 1: Start minimal, add as needed
pip install graphbit
pip install "graphbit[chromadb]"  # Add vector DB when needed

# Option 2: Full development stack
pip install "graphbit[benchmark]"  # Includes AI providers + tools

# Option 3: Complete development environment
pip install "graphbit[full]"  # Everything available
```

### Production Environment
```bash
# Minimal production deployment
pip install "graphbit[pgvector,boto3]"  # Only what you need

# Specific use case
pip install "graphbit[pinecone]"  # Managed services only
```

---

## Troubleshooting Installation Issues

### Common Issues and Solutions

#### 1. Large Installation Size
**Problem**: `pip install "graphbit[full]"` downloads too many packages
**Solution**: Use specific extras instead
```bash
# Instead of full, use specific combinations
pip install "graphbit[chromadb,boto3]"
```

#### 2. Dependency Conflicts
**Problem**: Version conflicts between optional dependencies
**Solution**: Use virtual environments and specific versions
```bash
# Create isolated environment
python -m venv graphbit-env
source graphbit-env/bin/activate  # Linux/macOS
pip install "graphbit[your-extras]"
```

#### 3. Database Driver Installation Failures
**Problem**: Native database drivers fail to compile
**Solutions**:

**PostgreSQL (psycopg2)**:
```bash
# Install system dependencies first
# Ubuntu/Debian
sudo apt-get install libpq-dev python3-dev

# macOS
brew install postgresql

# Then install GraphBit
pip install "graphbit[pgvector]"
```

**IBM Db2**:
```bash
# Requires IBM Db2 client libraries
# Follow IBM Db2 client installation guide first
pip install "graphbit[db2]"
```

#### 4. Memory Issues During Installation
**Problem**: Installation runs out of memory
**Solution**: Install dependencies separately
```bash
# Install heavy dependencies first
pip install numpy pandas matplotlib
pip install "graphbit[benchmark]"
```

#### 5. Network/Proxy Issues
**Problem**: Cannot download packages from PyPI
**Solution**: Configure pip for proxy/private PyPI
```bash
# Use proxy
pip install "graphbit[extras]"

# Use private PyPI
pip install -i "graphbit[extras]"
```

### Verification Commands

After installation, verify your setup:

```python
from graphbit import init, version, health_check

# Initialize and check system
init()
print(f"GraphBit version: {version()}")

# Check system health
health = health_check()
print(f"System healthy: {health['overall_healthy']}")

# Verify specific integrations
try:
    # Test vector database (example with ChromaDB)
    import chromadb
    print("ChromaDB available")
except ImportError:
    print("ChromaDB not installed")

try:
    # Test AI provider (example with OpenAI)
    import openai
    print("OpenAI available")
except ImportError:
    print("OpenAI not installed")
```

### Getting Help

If you encounter installation issues:

1. **Check system requirements**: Ensure Python 3.10+ and sufficient disk space
2. **Update pip**: `pip install --upgrade pip`
3. **Use virtual environments**: Isolate GraphBit dependencies
4. **Check the [Installation Guide](installation.md)** for system-specific instructions
5. **Search [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)** for similar problems
6. **Create a new issue** with:
   - Your operating system and Python version
   - Complete error message
   - Installation command used
   - Output of `pip list` and `python --version`

---

## Next Steps

Once you have GraphBit installed with the appropriate dependencies:

1. **Basic usage**: Follow the [Quick Start Tutorial](quickstart.md)
2. **Configuration**: Set up your [API keys and environment](../api-reference/configuration.md)
3. **Examples**: Explore [use case examples](../examples/) for your specific installation
4. **Development**: See the [Contributing Guide](../development/contributing.md) for development setup

---

*Choose the installation method that best fits your use case. You can always add more extras later as your needs evolve!*
