## [0.4.0] - 2025-09-12

# New & Enhanced Features

* Google Search API integration 
* Chatbot development example 
* LLM-Graphbit-Playwright Browser Automation Agent
* Text splitter module
* DeepSeek provider support
* Perplexity provider support
* Complete MkDocs docs site (Material theme)
* Docs site (initial)
* Rust Core & Python binding: Agentic workflow withdep-batching & parent preamble
* Node.js binding
* Tool calling support
* Makefile: cross-platform test infra + install; Rust/Python test targets
* System prompt added
* LLM configuration per node

# Bug Fixes & Stability

* Resolve hook failures & improve security compliance
* Update black & dependency checks
* macOS fallback for `sched_getaffinity`
* Codebase formatted to pass pre-commit
* Benchmark CPU affinity logic cross-platform (macOS support, safe fallback)
* Python tests: add/improve LLM, executor, doc loader tests
* Anthropic LLM config issue fixed 
* Pre-commit hook error fixed 
* Perplexity prompt error fixed 
* Makefile refactor fix 
* LlamaIndex missing dependency fixed 
* Sentence splitter issue fixed 
* Character splitter infinite loop fixed 
* Executor dict→JSON conversion fix 
* Document loader support in Python binding 
* Fixed Rust & Python test bugs 

# Refactors & DX

* Removed docs site & related files/code 
* `graphbit` init call auto by default 
* Remove unnecessary packages from `pyproject.toml` 
* Refactor: import `graphbit` in benchmark 
* API reference folder refactor 
* Node.js bindings & related code refactor
* Remove warning for Python binding  
* Version management: caching + improved reporting 
* `pyproject.toml` cleanup 
* Remove `pyproject.toml` and update `Cargo.toml` for Python consistency 
* Refactor benchmark scripts 
* Getting-started folder refactor 

# Documentation & Guides

* Integrations: MongoDB , Pinecone , Qdrant , PGVector , ChromaDB , FAISS , Milvus , Weaviate , IBM Db2 , Elasticsearch , Azure , GCP , AWS boto3 
* Embeddings docs updated ; `llm-providers.md` ; concepts/embeddings/async-vs-sync ; dynamics-graph ; monitoring ; agents ; document loaders ; performance ; reliability ; memory management ; tool-calling docs ; node types with `llm_config` 
* README updates: root & examples ; architecture diagram & doc validation 
* Benchmarks & reports: comparison summary , updated reports , benchmark README 
* Docs site content sync (2025-09-09) ; docs/examples folder ; CHANGELOG update ; about file ; validation docs ; remove Hugging Face refs ; license added 

# Tests

* LLM clients coverage; align workflow connect tests 
* Doc processing, text splitting, embedding tests 
* Basic unit testing 
* Edge cases: client & configuration failure tests 
* Python: import/init/security/workflow failure tests 
* LLM/agent/workflow tests; validation & integration refactor 
* Types/validation/error unit tests 
* Doc loader/embedding/text splitter tests 
* Python binding tests; Rust helper funcs
* Rust unit tests for agent, concurrency & graph 
* Document loader & serialization tests 
* Rust types & error tests 
* Integration tests: doc load, workflow & error 

# CI/CD & Workflows

* Auto version update on release & changelog generator 
* Validation workflow & runner scripts 
* Comprehensive testing workflow & runner 
* Build workflow w/ artifact generation & verification 
* Deployment action workflow 
* Workflow helper files test suite 
* CI/CD backup copies workflow 
* Pipeline orchestrator workflow 
* Sync docs to `graphbit-docs` repo
* Workflow refactor & updates

# Security & Meta

* Add `SECURITY.md`


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
