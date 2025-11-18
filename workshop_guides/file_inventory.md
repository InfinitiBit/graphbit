# Non-Ollama ParallelRAG File Inventory

**Purpose**: Complete inventory of all non-Ollama GraphBit ParallelRAG files for workshop demonstrations  
**Last Updated**: November 17, 2025  
**Scope**: Cloud-based LLM providers (OpenAI, Anthropic) - **EXCLUDES** `ollama_integration/` directory

---

## 📋 File Inventory Table

| File Path | File Type | LLM Provider(s) | Primary Purpose | Lines of Code | Category |
|-----------|-----------|-----------------|-----------------|---------------|----------|
| `examples/parallel_rag_optimized.py` | Example | OpenAI | Optimized ParallelRAG with GIL-releasing architecture | 354 | Example |
| `parallel_rag_app.py` | Example | OpenAI | Production-ready RAG with optimal configurations | 334 | Example |
| `langchain_rag_app.py` | Example | OpenAI | LangChain RAG for comparison benchmarks | ~300 | Example |
| `tests/benchmarks/benchmark_framework_comparison.py` | Benchmark | OpenAI | GraphBit vs LangChain RAG comparison | 733 | Benchmark |
| `tests/benchmarks/benchmark_stress_test.py` | Stress Test | Mock/Local | Progressive load testing (100-10K docs) | ~600 | Stress Test |
| `tests/benchmarks/benchmark_chunking.py` | Benchmark | N/A | Text splitter performance benchmarks | ~300 | Benchmark |
| `tests/benchmarks/benchmark_embedding.py` | Benchmark | OpenAI | Embedding generation benchmarks | ~300 | Benchmark |
| `tests/benchmarks/benchmark_llm.py` | Benchmark | OpenAI | LLM completion benchmarks | ~300 | Benchmark |
| `tests/benchmarks/benchmark_utils.py` | Utility | N/A | Shared benchmark utilities | ~300 | Utility |
| `benchmarks/run_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | Multi-framework comparison runner | 748 | Benchmark |
| `benchmarks/frameworks/graphbit_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | GraphBit framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/langchain_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | LangChain framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/langgraph_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | LangGraph framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/crewai_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | CrewAI framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/llamaindex_benchmark.py` | Benchmark | OpenAI, Anthropic, Ollama | LlamaIndex framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/pydantic_ai_benchmark.py` | Benchmark | OpenAI | PydanticAI framework implementation | ~200 | Benchmark |
| `benchmarks/frameworks/common.py` | Utility | N/A | Shared framework benchmark utilities | ~300 | Utility |
| `create_visualizations.py` | Visualization | N/A | Performance chart generation (5 charts) | 230 | Visualization |
| `create_resource_charts.py` | Visualization | N/A | Resource utilization charts (3 charts) | ~240 | Visualization |
| `create_additional_visualizations.py` | Visualization | N/A | Additional performance charts (4 charts) | ~305 | Visualization |
| `test_worker_optimization.py` | Benchmark | OpenAI | Worker count optimization testing | ~150 | Benchmark |
| `validate_rag_equivalence.py` | Validation | OpenAI | RAG implementation equivalence testing | ~200 | Validation |

---

## 📊 Summary Statistics

### By Category
- **Examples**: 3 files (~988 lines)
- **Benchmarks**: 11 files (~4,331 lines)
- **Stress Tests**: 1 file (~600 lines)
- **Visualizations**: 3 files (~775 lines)
- **Utilities**: 2 files (~600 lines)
- **Validation**: 1 file (~200 lines)

**Total**: 21 files, ~7,494 lines of code

### By LLM Provider
- **OpenAI**: 15 files (primary provider)
- **Anthropic**: 4 files (secondary provider)
- **Mock/Local**: 2 files (for stress testing without API costs)
- **Provider-Agnostic**: 5 files (utilities, visualizations)

### By Purpose
- **Performance Benchmarking**: 11 files
- **RAG Implementation Examples**: 3 files
- **Stress Testing**: 1 file
- **Visualization**: 3 files
- **Utilities**: 2 files
- **Validation**: 1 file

---

## 🎯 Key Files for Workshop Demonstrations

### Quick Demo (5 minutes)
1. **`examples/parallel_rag_optimized.py`** - Best for showing GraphBit's core capabilities
2. **`parallel_rag_app.py`** - Production-ready example with optimal configurations

### Framework Comparison (10 minutes)
1. **`tests/benchmarks/benchmark_framework_comparison.py`** - GraphBit vs LangChain
2. **`benchmarks/run_benchmark.py`** - Multi-framework comparison (6 frameworks)

### Scalability Demo (15 minutes)
1. **`tests/benchmarks/benchmark_stress_test.py`** - Progressive load testing
2. **`test_worker_optimization.py`** - Worker count optimization

### Visualization Demo (5 minutes)
1. **`create_visualizations.py`** - Main performance charts
2. **`create_resource_charts.py`** - Resource utilization charts

---

## 📝 Notes

- All files use **cloud-based LLM providers** (OpenAI, Anthropic)
- **Ollama integration** is in separate `ollama_integration/` directory (excluded from this inventory)
- Most benchmarks support **multiple providers** via command-line arguments
- All visualization scripts generate **PNG/SVG** output files
- Benchmark utilities provide **consistent metrics** across all tests

---

## 🔗 Related Documentation

- **Main Workshop Guide**: `workshop_guides/NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md`
- **Command Reference**: `workshop_guides/command_reference.md`
- **Code Architecture**: `workshop_guides/code_architecture_map.md`
- **Demo Scripts**: `workshop_guides/demo_scripts/`


