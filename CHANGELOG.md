# Changelog

## [0.4.0] – Draft

- Rust core and Python binding: agentic workflow with dep-batching and parent preamble
- SECURITY.md added
- Document loader support in Python binding
- LLM-GraphBit-Playwright browser automation agent implemented
- Google Search API integration
- DeepSeek & Perplexity provider support
- Vector DB docs: FAISS, Milvus, Pinecone, Qdrant, ChromaDB, Weaviate, MongoDB, PGVector, MariaDB, IBM Db2, Elasticsearch, AWS boto3
- Embeddings docs updated; async-vs-sync guide
- Chatbot development example added
- Text splitter utility added
- Complete MkDocs site (Material theme)
- AI LLM multi-agent benchmark report summary
- Cross-platform CPU affinity logic with macOS fallback
- Resolved pre-commit hook failures; improved security compliance
- Updated black and dependency checks; codebase reformatted


## [0.3.0-alpha] – 2025-07-16

- Comprehensive Python tests added
- Rust integration test coverage improved
- Improved benchmark runner
- Benchmark run consistency improved
- Centralized control of benchmark run counts
- Centralized run configuration committed
- Explicit flags/config for run counts
- Explicit run control documented
- Dockerization support for benchmark
- Production volume mount paths refined
- Tarpaulin coverage added for Rust
- Tarpaulin configuration integrated
- Benchmark documentation updated
- Root README updated
- Contributing guidelines updated


## [0.2.0-alpha] – 2025-06-28

- Ollama Python support added
- Hugging Face Python binding added
- LangGraph integrated into benchmark framework
- CrewAI benchmark scenarios optimized for performance and reliability
- Performance optimizations
- Benchmark and Python binding refactors
- New integration tests added
- Python integration tests expanded
- Fixed Python integration tests for GitHub Actions
- Python examples for GraphBit added
- Benchmark evaluation updated
- Example code updated
- Python documentation updated
- Pre-commit issues resolved
- Pre-test commit fixed for all files
- Makefile fixes
- Root README updated
- GitHub Actions workflow removed


## [0.1.0] - 2025-06-11

- Initial GraphBit release: declarative agentic workflow framework
- Modular architecture with core modules (agents, llm, graph, validation, workflow, types, errors)
- Multi-LLM support: OpenAI GPT, Anthropic Claude, Ollama, extensible providers
- Graph-based workflows (DAG), dependency management, topological execution
- Node types: agent, transform, conditional
- Parallel execution with configurable concurrency
- Async/await support throughout
- Full Python API via PyO3 with async support
- Strong typing and validation
- JSON schema validation for LLM outputs
- UUID identifiers for all components
- Intelligent dependency resolution
- Error handling: retries with backoff, comprehensive errors, failure recovery
- Usage tracking: tokens, cost estimation, performance metrics
- CLI (graphbit): init, validate, run, config, debug/verbose
- JSON workflow configs with env var support and custom files
- Integration examples: FastAPI, Django, Jupyter
- Documentation and examples: README, Ollama guide, testing/benchmarking, Python and Rust API docs
- Testing: unit/integration, benchmarking suite, mock LLM providers, CI/CD
- Dependencies and MSRV: tokio, serde, anyhow/thiserror, uuid, petgraph, reqwest, clap, pyo3, chrono; Rust 1.70+
