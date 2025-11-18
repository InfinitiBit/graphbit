# ParallelRAG Core

**High-Performance RAG Implementation using GraphBit**

This directory contains the complete ParallelRAG implementation, including applications, examples, benchmarks, tests, and comprehensive documentation.

---

## 📁 Directory Structure

```
parallelrag_core/
├── parallel_rag_app.py              # Production-ready ParallelRAG application
├── langchain_rag_app.py             # LangChain RAG application (for comparison)
├── examples/                        # Example scripts and applications
├── benchmarks/                      # Benchmark scripts and frameworks
├── tests/                           # Test files (unit, integration, benchmarks)
├── docs/                            # Comprehensive documentation
├── visualizations/                  # Visualization scripts
├── scripts/                         # Utility scripts
├── data/                            # Data files (benchmark results, charts, samples)
└── test_results/                    # Test result files
```

---

## 🚀 Quick Start

### 1. Run Production ParallelRAG Application

```bash
# Set your OpenAI API key
export OPENAI_API_KEY="your-api-key-here"

# Run the application
python parallelrag_core/parallel_rag_app.py
```

### 2. Run Optimized ParallelRAG Example

```bash
python parallelrag_core/examples/parallel_rag_optimized.py
```

### 3. Run Framework Comparison Benchmark

```bash
python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py \
  --framework both \
  --max-docs 100
```

---

## 📚 Key Components

### Applications

- **`parallel_rag_app.py`**: Production-ready ParallelRAG with optimal configurations
  - 19.22x end-to-end speedup
  - Parallel chunking, embedding, and LLM processing
  - Statistics tracking and monitoring

- **`langchain_rag_app.py`**: LangChain RAG implementation for comparison
  - Equivalent functionality to ParallelRAG
  - Uses LangChain's components and best practices

### Examples

- **`examples/parallel_rag_optimized.py`**: Optimized ParallelRAG demonstration
  - GIL-releasing document loading
  - Lock-free parallel batch processing
  - Async LLM processing

- **`examples/tasks_examples/`**: Task workflow examples
  - Simple, sequential, and complex workflows
  - Local model and cloud API examples

### Benchmarks

- **`benchmarks/run_benchmark.py`**: Multi-framework comparison runner
  - Supports 6 frameworks (GraphBit, LangChain, LangGraph, CrewAI, LlamaIndex, PydanticAI)
  - Comprehensive performance metrics

- **`tests/benchmarks/`**: Specialized benchmark scripts
  - Framework comparison
  - Stress testing
  - Component-level benchmarks (chunking, embedding, LLM)

### Tests

- **`tests/test_parallel_rag_app.py`**: ParallelRAG application tests (21 tests)
- **`tests/test_langchain_rag_app.py`**: LangChain RAG application tests (20 tests)
- **`tests/python_integration_tests/`**: Integration tests
- **`tests/python_unit_tests/`**: Unit tests

### Documentation

- **`docs/benchmarks/`**: Benchmark results and performance analysis
- **`docs/implementation/`**: Implementation guides and GIL fixes
- **`docs/rag/`**: RAG specifications and comparisons
- **`docs/applications/`**: Application documentation
- **`docs/production/`**: Production deployment guides
- **`docs/project/`**: Project management documentation
- **`docs/phases/`**: Phase-by-phase implementation documentation
- **`docs/testing/`**: Testing and validation documentation
- **`docs/marketing/`**: Marketing materials and presentations
- **`docs/analysis/`**: Technical analysis documents

---

## 📊 Performance Highlights

### GraphBit ParallelRAG vs LangChain RAG

| Metric | GraphBit | LangChain | Speedup |
|--------|----------|-----------|---------|
| **Chunking** | 0.32s | 2.00s | **6.20x** |
| **Embedding** | 0.29s | 10.10s | **34.81x** |
| **LLM** | 0.53s | 10.10s | **19.04x** |
| **End-to-End** | 1.14s | 21.90s | **19.22x** |

### Key Features

- ✅ **GIL-Releasing**: True parallelism for I/O operations
- ✅ **Lock-Free**: Parallel batch processing without locks
- ✅ **Async LLM**: Non-blocking LLM operations
- ✅ **Optimal Workers**: 20 workers for all operations (based on benchmarks)

---

## 🧪 Running Tests

### Run All Tests

```bash
pytest parallelrag_core/tests/ -v
```

### Run Specific Test Suite

```bash
# ParallelRAG app tests
pytest parallelrag_core/tests/test_parallel_rag_app.py -v

# LangChain RAG app tests
pytest parallelrag_core/tests/test_langchain_rag_app.py -v

# Benchmark tests
pytest parallelrag_core/tests/benchmarks/ -v
```

---

## 📈 Visualizations

Generate performance charts:

```bash
# Main visualizations
python parallelrag_core/visualizations/create_visualizations.py

# Resource charts
python parallelrag_core/visualizations/create_resource_charts.py

# Additional visualizations
python parallelrag_core/visualizations/create_additional_visualizations.py
```

---

## 📖 Documentation

For detailed documentation, see:

- **[Benchmark Results](docs/benchmarks/BENCHMARK_RESULTS.md)**: Comprehensive benchmark data
- **[Framework Comparison](docs/benchmarks/FRAMEWORK_COMPARISON.md)**: GraphBit vs other frameworks
- **[ParallelRAG App Documentation](docs/applications/PARALLEL_RAG_APP_DOCUMENTATION.md)**: Complete API reference
- **[Production Deployment Guide](docs/production/PRODUCTION_DEPLOYMENT_GUIDE.md)**: Production best practices
- **[RAG Implementation Summary](docs/rag/RAG_IMPLEMENTATION_SUMMARY.md)**: RAG architecture overview

---

## 🔧 Development

### Import Structure

```python
# Import ParallelRAG application
from parallelrag_core.parallel_rag_app import ParallelRAG, RAGConfig

# Import LangChain RAG application
from parallelrag_core.langchain_rag_app import LangChainRAG, LangChainRAGConfig

# Import benchmark utilities
from parallelrag_core.tests.benchmarks.benchmark_utils import get_system_info
```

### Package Structure

All directories are proper Python packages with `__init__.py` files, enabling clean imports and modular organization.

---

## 📝 License

This is part of the GraphBit project. See the main project license for details.

---

## 🤝 Contributing

For contribution guidelines, see the main project's `CONTRIBUTING.md`.

---

## 📞 Support

For issues or questions:
1. Check the documentation in `docs/`
2. Review benchmark results and test cases
3. Examine integration tests for usage examples
4. Consult the main GraphBit library documentation

