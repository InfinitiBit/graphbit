# 🦙 GraphBit Ollama Integration

**Complete integration of GraphBit and LangChain RAG implementations with local Ollama models**

This directory contains everything you need to run GraphBit and LangChain RAG systems with local Ollama models, enabling cost-free testing and deployment with locally-hosted LLMs.

---

## 📁 Directory Structure

```
ollama_integration/
├── examples/              # RAG implementation examples
│   ├── parallel_rag_ollama.py      # GraphBit ParallelRAG with Ollama
│   └── langchain_rag_ollama.py     # LangChain RAG with Ollama
├── benchmarks/            # Performance benchmarking tools
│   ├── benchmark_ollama_comparison.py  # Framework comparison benchmark
│   └── stress_test_ollama.py           # Stress testing suite
├── docs/                  # Comprehensive documentation
│   ├── SETUP_GUIDE.md              # Installation and setup
│   ├── INTEGRATION_README.md       # Integration overview
│   ├── DEVELOPER_QUICKSTART.md     # Quick start guide
│   ├── TEST_RESULTS.md             # Validation test results
│   └── VALIDATION_SUMMARY.md       # Complete validation summary
├── test_results/          # Test output and results
│   ├── benchmark_results.json      # Benchmark comparison results
│   └── stress_test_results/        # Stress test outputs
│       └── progressive_load_results.json
└── README.md              # This file
```

---

## 🚀 Quick Start

### 1. Install Ollama

**Windows**:
```bash
winget install Ollama.Ollama
```

**macOS**:
```bash
brew install ollama
```

**Linux**:
```bash
curl -fsSL https://ollama.com/install.sh | sh
```

### 2. Pull Required Models

```bash
# Start Ollama server
ollama serve

# Pull LLM model (choose one)
ollama pull gemma3:4b      # Recommended for testing (3.3 GB)
ollama pull llama3:8b      # Better quality (4.7 GB)
ollama pull mistral:7b     # Alternative (4.1 GB)

# Pull embedding model
ollama pull nomic-embed-text  # Recommended (274 MB)
```

### 3. Run Examples

**GraphBit ParallelRAG with Ollama**:
```bash
python ollama_integration/examples/parallel_rag_ollama.py
```

**LangChain RAG with Ollama**:
```bash
python ollama_integration/examples/langchain_rag_ollama.py
```

### 4. Run Benchmarks

**Framework Comparison**:
```bash
python ollama_integration/benchmarks/benchmark_ollama_comparison.py --framework both --max-docs 5
```

**Stress Testing**:
```bash
python ollama_integration/benchmarks/stress_test_ollama.py --framework both --max-docs 50
```

---

## 📊 Performance Results

### Framework Comparison (5 documents, CPU)

| Metric | GraphBit | LangChain | Speedup |
|--------|----------|-----------|---------|
| **Total Time** | 46.08s | 57.68s | **1.25x** |
| **Throughput** | 0.11 docs/sec | 0.09 docs/sec | **1.25x** |
| **Peak Memory** | 87.77 MB | 97.52 MB | **1.11x less** |

**Key Findings**:
- ✅ GraphBit is 1.25x faster overall
- ✅ GraphBit uses 10 MB less memory
- ✅ GraphBit's parallel loading provides speedup even with small datasets
- ⚠️ Embedding generation is the bottleneck on CPU (~2s per embedding)

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[SETUP_GUIDE.md](docs/SETUP_GUIDE.md)** | Complete installation and setup instructions for Windows/macOS/Linux |
| **[DEVELOPER_QUICKSTART.md](docs/DEVELOPER_QUICKSTART.md)** | Step-by-step quick start guide with troubleshooting |
| **[INTEGRATION_README.md](docs/INTEGRATION_README.md)** | Technical integration overview and architecture |
| **[TEST_RESULTS.md](docs/TEST_RESULTS.md)** | Validation test results and bug fixes |
| **[VALIDATION_SUMMARY.md](docs/VALIDATION_SUMMARY.md)** | Complete validation summary and achievements |

---

## 🎯 Use Cases

1. **Cost-Free Development**: Test RAG systems without API costs
2. **Privacy-Sensitive Applications**: Keep all data local
3. **Offline Deployment**: Run RAG systems without internet
4. **Performance Benchmarking**: Compare GraphBit vs LangChain
5. **Workshop Demonstrations**: Demo RAG systems with local models
6. **Production Deployment**: Deploy RAG with local LLMs

---

## ⚡ Features

### GraphBit ParallelRAG with Ollama
- ✅ Built-in Ollama LLM support via `LlmConfig.ollama()`
- ✅ True parallel document loading (GIL-releasing)
- ✅ Async LLM queries for non-blocking operations
- ✅ 1.25x faster than LangChain
- ⚠️ Uses LangChain's OllamaEmbeddings (no native embedding support yet)

### LangChain RAG with Ollama
- ✅ Full Ollama support (OllamaEmbeddings + ChatOllama)
- ✅ FAISS vector storage
- ✅ Simple, straightforward API
- ⚠️ Sequential processing (no parallelism)
- ⚠️ Slower than GraphBit

---

## 🔧 Configuration

### Recommended Models

**For Testing (CPU)**:
- LLM: `gemma3:4b` (3.3 GB, fast)
- Embedding: `nomic-embed-text` (274 MB, 768-dim)

**For Production (GPU)**:
- LLM: `llama3:8b` or `mistral:7b` (better quality)
- Embedding: `nomic-embed-text` or `mxbai-embed-large`

### System Requirements

**Minimum**:
- 8 GB RAM
- 10 GB disk space
- CPU: 4+ cores

**Recommended**:
- 16+ GB RAM
- 20+ GB disk space
- GPU: NVIDIA RTX 3060+ (for 10x faster embeddings)

---

## 🐛 Known Issues

1. **LangChain Deprecation Warning**: `OllamaEmbeddings` deprecated in LangChain 0.3.1
   - **Solution**: Update to `langchain_ollama` package (see docs/DEVELOPER_QUICKSTART.md)

2. **Slow Embedding Generation on CPU**: ~2s per embedding
   - **Solution**: Use GPU-accelerated Ollama (10x faster) or smaller models

---

## ✅ Validation Status

- [x] All implementations tested and validated
- [x] 3 critical bugs fixed
- [x] Performance baselines established
- [x] 100% test success rate
- [x] Production-ready

**Last Validated**: November 17, 2025  
**Test Environment**: Windows 11, Intel Core i9, 32GB RAM

---

## 🤝 Contributing

See the main [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

---

## 📄 License

See the main [LICENSE.md](../LICENSE.md) for license information.

---

**Status**: ✅ **PRODUCTION READY**  
**Version**: 1.0  
**Completion Date**: November 17, 2025

