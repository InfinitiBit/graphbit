# Dependency Installation Guide

This guide explains how to install GraphBit with different dependency configurations based on your specific needs and use cases.

## Overview

GraphBit uses a modular dependency system that allows you to install only the components you need:

- **Core dependencies**: Essential packages required for basic GraphBit functionality
- **Optional dependencies**: Additional packages grouped by functionality (vector databases, benchmarking tools, or everything graphbit provides support for)
- **Extras groups**: Predefined collections of optional dependencies for common use cases

---

## Core Installation (Minimal Dependencies)

The core installation includes only the essential dependencies needed for basic GraphBit functionality.

### Installation Commands

=== "pip"
    ```bash
    pip install graphbit
    ```

=== "poetry"
    ```bash
    poetry add graphbit
    ```

=== "uv"
    ```bash
    uv add graphbit
    ```

### Core Installation Use Cases

- **Lightweight deployments** where you only need basic workflow functionality
- **Custom integrations** where you'll add your own databases
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

=== "pip"
    ```bash
    pip install "graphbit[benchmark]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[benchmark]"
    ```

=== "uv"
    ```bash
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

=== "pip"
    ```bash
    pip install "graphbit[full]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[full]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[full]"
    ```

**Use cases:**
- Development environments where you need access to all features
- Prototyping with multiple AI providers and databases
- Educational or training environments
- One-stop installation for comprehensive GraphBit usage

#### Vector Database Extras

##### `chromadb` - ChromaDB Integration

=== "pip"
    ```bash
    pip install "graphbit[chromadb]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[chromadb]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[chromadb]"
    ```
**Includes**: `chromadb`
**Use case**: Document similarity search, RAG applications

##### `pinecone` - Pinecone Integration

=== "pip"
    ```bash
    pip install "graphbit[pinecone]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[pinecone]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[pinecone]"
    ```
**Includes**: `pinecone`
**Use case**: Managed vector database for production applications

##### `qdrant` - Qdrant Integration

=== "pip"
    ```bash
    pip install "graphbit[qdrant]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[qdrant]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[qdrant]"
    ```
**Includes**: `qdrant-client`
**Use case**: Open-source vector database with advanced filtering

##### `weaviate` - Weaviate Integration

=== "pip"
    ```bash
    pip install "graphbit[weaviate]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[weaviate]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[weaviate]"
    ```
**Includes**: `weaviate-client`
**Use case**: Knowledge graphs and semantic search

##### `milvus` - Milvus Integration

=== "pip"
    ```bash
    pip install "graphbit[milvus]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[milvus]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[milvus]"
    ```
**Includes**: `pymilvus`
**Use case**: Large-scale vector similarity search

##### `faiss` - FAISS Integration

=== "pip"
    ```bash
    pip install "graphbit[faiss]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[faiss]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[faiss]"
    ```
**Includes**: `faiss-cpu`, `numpy`
**Use case**: High-performance similarity search and clustering

##### `elasticsearch` - Elasticsearch Integration

=== "pip"
    ```bash
    pip install "graphbit[elasticsearch]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[elasticsearch]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[elasticsearch]"
    ```
**Includes**: `elasticsearch`
**Use case**: Full-text search with vector capabilities

#### Database Extras

##### `pgvector` - PostgreSQL with pgvector

=== "pip"
    ```bash
    pip install "graphbit[pgvector]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[pgvector]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[pgvector]"
    ```
**Includes**: `psycopg2`
**Use case**: PostgreSQL with vector extensions

##### `mariadb` - MariaDB Integration

=== "pip"
    ```bash
    pip install "graphbit[mariadb]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[mariadb]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[mariadb]"
    ```
**Includes**: `mariadb`, `numpy`
**Use case**: MariaDB with vector capabilities

##### `mongodb` - MongoDB Integration

=== "pip"
    ```bash
    pip install "graphbit[mongodb]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[mongodb]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[mongodb]"
    ```
**Includes**: `pymongo`
**Use case**: Document database with vector search

##### `db2` - IBM Db2 Integration

=== "pip"
    ```bash
    pip install "graphbit[db2]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[db2]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[db2]"
    ```
**Includes**: `ibm-db`, `numpy`
**Use case**: Enterprise IBM Db2 database

#### Cloud Provider Extras

##### `boto3` - Amazon Web Services

=== "pip"
    ```bash
    pip install "graphbit[boto3]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[boto3]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[boto3]"
    ```
**Includes**: `boto3`
**Use case**: AWS services integration

##### `astradb` - DataStax Astra DB

=== "pip"
    ```bash
    pip install "graphbit[astradb]"
    ```

=== "poetry"
    ```bash
    poetry add "graphbit[astradb]"
    ```

=== "uv"
    ```bash
    uv add "graphbit[astradb]"
    ```
**Includes**: `astrapy`
**Use case**: Managed Cassandra with vector search

---

## Multiple Extras Installation

You can install multiple extras groups simultaneously:

=== "pip"
    ```bash
    # Install multiple specific extras
    pip install "graphbit[chromadb,boto3]"
    
    # Install benchmark tools with specific database
    pip install "graphbit[benchmark,pgvector]"
    
    # Install full AI stack with vector databases
    pip install "graphbit[benchmark,chromadb,pinecone,qdrant]"
    ```

=== "poetry"
    ```bash
    # Install multiple specific extras
    poetry add "graphbit[chromadb,boto3]"
    
    # Install benchmark tools with specific database
    poetry add "graphbit[benchmark,pgvector]"
    
    # Install full AI stack with vector databases
    poetry add "graphbit[benchmark,chromadb,pinecone,qdrant]"
    ```

=== "uv"
    ```bash
    # Install multiple specific extras
    uv add "graphbit[chromadb,boto3]"
    
    # Install benchmark tools with specific database
    uv add "graphbit[benchmark,pgvector]"
    
    # Install full AI stack with vector databases
    uv add "graphbit[benchmark,chromadb,pinecone,qdrant]"
    ```

---

## Installation Scenarios by Use Case

### Scenario 1: 
=== "pip"
    ```bash
    # Minimal installation for simple workflows
    pip install graphbit
    ```
=== "poetry"
    ```bash
    # Minimal installation for simple workflows
    poetry add graphbit
    ```
=== "uv"
    ```bash
    # Minimal installation for simple workflows
    uv add graphbit
    ```

### Scenario 2: RAG Application Development
=== "pip"
    ```bash
    # Vector database
    pip install "graphbit[chromadb]"
    
    # Alternative with managed vector DB
    pip install "graphbit[pinecone]"
    ```

=== "poetry"
    ```bash
    # Vector database
    poetry add "graphbit[chromadb]"
    
    # Alternative with managed vector DB
    poetry add "graphbit[pinecone]"
    ```

=== "uv"
    ```bash
    # Vector database
    uv add "graphbit[chromadb]"
    
    # Alternative with managed vector DB
    uv add "graphbit[pinecone]"
    ```

### Scenario 3: Enterprise Production Environment
=== "pip"
    ```bash
    # PostgreSQL + Boto3
    pip install "graphbit[pgvector,boto3]"
    
    # IBM enterprise stack
    pip install "graphbit[db2,boto3]"
    ```

=== "poetry"
    ```bash
    # PostgreSQL + Boto3
    poetry add "graphbit[pgvector,boto3]"
    
    # IBM enterprise stack
    poetry add "graphbit[db2,boto3]"
    ```

=== "uv"
    ```bash
    # PostgreSQL + Boto3
    uv add "graphbit[pgvector,boto3]"
    
    # IBM enterprise stack
    uv add "graphbit[db2,boto3]"
    ```

### Scenario 4: Research and Benchmarking
=== "pip"
    ```bash
    # Full benchmarking suite
    pip install "graphbit[benchmark]"
    
    # Comprehensive research environment
    pip install "graphbit[full]"
    ```

=== "poetry"
    ```bash
    # Full benchmarking suite
    poetry add "graphbit[benchmark]"
    
    # Comprehensive research environment
    poetry add "graphbit[full]"
    ```

=== "uv"
    ```bash
    # Full benchmarking suite
    uv add "graphbit[benchmark]"
    
    # Comprehensive research environment
    uv add "graphbit[full]"
    ```

### Scenario 5: Multi-Cloud Development
=== "pip"
    ```bash
    # Multiple vector databases for testing
    pip install "graphbit[chromadb,pinecone,qdrant,weaviate]"
    ```

=== "poetry"
    ```bash
    # Multiple vector databases for testing
    poetry add "graphbit[chromadb,pinecone,qdrant,weaviate]"
    ```

=== "uv"
    ```bash
    # Multiple vector databases for testing
    uv add "graphbit[chromadb,pinecone,qdrant,weaviate]"
    ```

---

## Dependency Comparison Table

| Extra Group | AI Providers | Vector DBs | Visualization | Performance | Cloud | Size |
|-------------|--------------|------------|---------------|-------------|-------|------|
| Core | ✅ Built-in | ❌ | ❌ | ❌ | ❌ | Minimal |
| `benchmark` | ✅ All + External Frameworks | ❌ | ✅ Full | ✅ Full | ❌ | Large |
| `chromadb` | ❌ | ChromaDB | ❌ | ❌ | ❌ | Small |
| `pinecone` | ❌ | Pinecone | ❌ | ❌ | ❌ | Small |
| `pgvector` | ❌ | PostgreSQL | ❌ | ❌ | ❌ | Small |
| `boto3` | ❌ | ❌ | ❌ | ❌ | AWS | Small |
| `full` | ✅ All | ✅ All | ✅ Full | ✅ Full | ✅ All | Very Large |

---

## Development vs Production Recommendations

### Development Environment
=== "pip"
    ```bash
    # Option 1: Start minimal, add as needed
    pip install graphbit
    pip install "graphbit[chromadb]"  # Add vector DB when needed

    # Option 2: Full development stack
    pip install "graphbit[benchmark]"  # Includes AI providers + tools

    # Option 3: Complete development environment
    pip install "graphbit[full]"  # Everything available
    ```
=== "poetry"
    ```bash
    # Option 1: Start minimal, add as needed
    poetry add graphbit
    poetry add "graphbit[chromadb]"  # Add vector DB when needed

    # Option 2: Full development stack
    poetry add "graphbit[benchmark]"  # Includes AI providers + tools

    # Option 3: Complete development environment
    poetry add "graphbit[full]"  # Everything available
    ```
=== "uv"
    ```bash
    # Option 1: Start minimal, add as needed
    uv add graphbit
    uv add "graphbit[chromadb]"  # Add vector DB when needed

    # Option 2: Full development stack
    uv add "graphbit[benchmark]"  # Includes AI providers + tools

    # Option 3: Complete development environment
    uv add "graphbit[full]"  # Everything available
    ```


### Production Environment
=== "pip"
    ```bash
    # Minimal production deployment
    pip install "graphbit[pgvector,boto3]"  # Only what you need

    # Specific use case
    pip install "graphbit[pinecone]"  # Managed services only
    ```
=== "poetry"
    ```bash
    # Minimal production deployment
    poetry add "graphbit[pgvector,boto3]"  # Only what you need

    # Specific use case
    poetry add "graphbit[pinecone]"  # Managed services only
    ```
=== "uv"
    ```bash
    # Minimal production deployment
    uv add "graphbit[pgvector,boto3]"  # Only what you need

    # Specific use case
    uv add "graphbit[pinecone]"  # Managed services only
    ```

---

## Troubleshooting Installation Issues

### Common Issues and Solutions

#### 1. Large Installation Size
**Problem**: `pip install "graphbit[full]"` downloads too many packages
**Solution**: Use specific extras instead
=== "pip"
    ```bash
    # Instead of full, use specific combinations
    pip install "graphbit[chromadb,boto3]"
    ```
=== "poetry"
    ```bash
    # Instead of full, use specific combinations
    poetry add "graphbit[chromadb,boto3]"
    ```
=== "uv"
    ```bash
    # Instead of full, use specific combinations
    uv add "graphbit[chromadb,boto3]"
    ```

#### 2. Dependency Conflicts
**Problem**: Version conflicts between optional dependencies
**Solution**: Use virtual environments and specific versions
=== "pip"
    ```bash
    # Create isolated environment
    python -m venv graphbit-env
    source graphbit-env/bin/activate  # Linux/macOS
    pip install "graphbit[your-extras]"
    ```
=== "poetry"
    ```bash
    # Create isolated environment
    python -m venv graphbit-env
    source graphbit-env/bin/activate  # Linux/macOS
    poetry add "graphbit[your-extras]"
    ```
=== "uv"
    ```bash
    # Create isolated environment
    python -m venv graphbit-env
    source graphbit-env/bin/activate  # Linux/macOS
    uv add "graphbit[your-extras]"
    ```

#### 3. Database Driver Installation Failures
**Problem**: Native database drivers fail to compile
**Solutions**:

**PostgreSQL (psycopg2)**:
=== "pip"
    ```bash
    # Install system dependencies first
    # Ubuntu/Debian
    sudo apt-get install libpq-dev python3-dev

    # macOS
    brew install postgresql

    # Then install GraphBit
    pip install "graphbit[pgvector]"
    ```
=== "poetry"
    ```bash
    # Install system dependencies first
    # Ubuntu/Debian
    sudo apt-get install libpq-dev python3-dev

    # macOS
    brew install postgresql

    # Then install GraphBit
    poetry add "graphbit[pgvector]"
    ```
=== "uv"
    ```bash
    # Install system dependencies first
    # Ubuntu/Debian
    sudo apt-get install libpq-dev python3-dev

    # macOS
    brew install postgresql

    # Then install GraphBit
    uv add "graphbit[pgvector]"
    ```

**IBM Db2**:
=== "pip"
    ```bash
    # Requires IBM Db2 client libraries
    # Follow IBM Db2 client installation guide first
    pip install "graphbit[db2]"
    ```
=== "poetry"
    ```bash
    # Requires IBM Db2 client libraries
    # Follow IBM Db2 client installation guide first
    poetry add "graphbit[db2]"
    ```
=== "uv"
    ```bash
    # Requires IBM Db2 client libraries
    # Follow IBM Db2 client installation guide first
    uv add "graphbit[db2]"
    ```

#### 4. Memory Issues During Installation
**Problem**: Installation runs out of memory
**Solution**: Install dependencies separately
=== "pip"
    ```bash
    # Install heavy dependencies first
    pip install numpy pandas matplotlib
    pip install "graphbit[benchmark]"
    ```
=== "poetry"
    ```bash
    # Install heavy dependencies first
    poetry add numpy pandas matplotlib
    poetry add "graphbit[benchmark]"
    ```
=== "uv"
    ```bash
    # Install heavy dependencies first
    uv add numpy pandas matplotlib
    uv add "graphbit[benchmark]"
    ```

#### 5. Network/Proxy Issues
**Problem**: Cannot download packages from PyPI
**Solution**: Configure pip for proxy/private PyPI
=== "pip"
    ```bash
    # Use proxy
    pip install "graphbit[extras]"

    # Use private PyPI
    pip install -i "graphbit[extras]"
    ```
=== "poetry"
    ```bash
    # Use proxy
    poetry add "graphbit[extras]"

    # Use private PyPI
    poetry add -i "graphbit[extras]"
    ```
=== "uv"
    ```bash
    # Use proxy
    uv add "graphbit[extras]"

    # Use private PyPI
    uv add -i "graphbit[extras]"
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
