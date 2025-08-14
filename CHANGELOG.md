# Changelog

## [0.4.0] – Draft

- Google Search API integration
- Chatbot development example added
- Text splitter utility added
- LLM-GraphBit-Playwright browser automation agent implemented
- DeepSeek & Perplexity provider support
- Complete MkDocs site (Material theme)
- Rust core and Python binding: agentic workflow with dep-batching and parent preamble
- Document loader support in Python binding
- Cross-platform CPU affinity logic with macOS fallback
- Resolved pre-commit hook failures; improved security compliance
- Updated black and dependency checks; codebase reformatted
- Vector DB docs: FAISS, Milvus, Pinecone, Qdrant, ChromaDB, Weaviate, MongoDB, PGVector, MariaDB, IBM Db2, Elasticsearch, AWS boto3
- Embeddings docs updated; async-vs-sync guide
- AI LLM multi-agent benchmark report summary
- SECURITY.md added


## [0.3.0-alpha] – 2025-07-16

- Improved benchmark runner
- Benchmark documentation updated
- Dockerization support for benchmark
- Production volume mount paths refined
- Centralized control of benchmark run counts
- Explicit flags/config for run counts
- Tarpaulin coverage added for Rust
- Tarpaulin configuration integrated
- Comprehensive Python tests added
- Rust integration test coverage improved
- Root README updated
- Contributing guidelines updated
- Benchmark run consistency improved
- Centralized run configuration committed
- Explicit run control documented


## [0.2.0-alpha] – 2025-06-28

- Ollama Python support added
- Hugging Face Python binding added
- LangGraph integrated into benchmark framework
- New integration tests added
- Python examples for GraphBit added
- Python integration tests expanded
- Fixed Python integration tests for GitHub Actions
- Pre-test commit fixed for all files
- Pre-commit issues resolved
- Makefile fixes
- Benchmark evaluation updated
- Example code updated
- Performance optimizations
- Benchmark and Python binding refactors
- CrewAI benchmark scenarios optimized for performance and reliability
- Root README updated
- Python documentation updated
- GitHub Actions workflow removed


## [0.1.0] - 2025-06-11

- Initial GraphBit release: declarative agentic workflow framework
- Multi-LLM support: OpenAI GPT, Anthropic Claude, Ollama, extensible providers
- Graph-based workflows (DAG), dependency management, topological execution
- Node types: agent, transform, conditional
- Parallel execution with configurable concurrency
- Async/await support throughout
- Intelligent dependency resolution
- Strong typing and validation
- UUID identifiers for all components
- JSON schema validation for LLM outputs
- Full Python API via PyO3 with async support
- Integration examples: FastAPI, Django, Jupyter
- CLI (graphbit): init, validate, run, config, debug/verbose
- JSON workflow configs with env var support and custom files
- Error handling: retries with backoff, comprehensive errors, failure recovery
- Usage tracking: tokens, cost estimation, performance metrics
- Documentation and examples: README, Ollama guide, testing/benchmarking, Python and Rust API docs
- Testing: unit/integration, benchmarking suite, mock LLM providers, CI/CD
- Modular architecture with core modules (agents, llm, graph, validation, workflow, types, errors)
- Dependencies and MSRV: tokio, serde, anyhow/thiserror, uuid, petgraph, reqwest, clap, pyo3, chrono; Rust 1.70+
