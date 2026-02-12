## [0.6.4] - 2026-02-12

### 🐛 Bug Fixes

- **AzureLLM Max Tokens Warning Bug** by @Junaid-Hossain in [#437](https://github.com/InfinitiBit/graphbit/pull/437) ([1216173](https://github.com/InfinitiBit/graphbit/commit/1216173)) on 2026-02-12
  - Fixed issue with max tokens warning in AzureLLM provider

---
**Total Changes**: 1
**Changes by Category**: 🐛 Bug Fixes: 1

## [0.6.3] - 2026-02-10

### ✨ New Features

- **Deepseek Streaming Support** by @Junaid-Hossain in [#429](https://github.com/InfinitiBit/graphbit/pull/429) ([b757028](https://github.com/InfinitiBit/graphbit/commit/b757028)) on 2026-02-10
  - Added streaming support for Deepseek LLM provider
  - Enhanced real-time response capabilities

### 🔧 Maintenance

- **LLM Workflow Enhancements** by @Junaid-Hossain in [#432](https://github.com/InfinitiBit/graphbit/pull/432) ([6a72724](https://github.com/InfinitiBit/graphbit/commit/6a72724)) on 2026-02-10
  - Implemented cumulative token budget tracking across workflow
  - Added per-node LLM configuration support
  - Fixed Azure OpenAI empty-response handling

- **Release v0.6.3** in [#434](https://github.com/InfinitiBit/graphbit/pull/434) ([3b8c16b](https://github.com/InfinitiBit/graphbit/commit/3b8c16b)) on 2026-02-10

---
**Total Changes**: 3
**Changes by Category**: ✨ New Features: 1 | 🔧 Maintenance: 2

## [0.6.2] - 2026-02-10

### ✨ New Features

- Add autogent missing deps by @jaid-jashim in [#416](https://github.com/InfinitiBit/graphbit/pull/416) ([24ae0f7](https://github.com/InfinitiBit/graphbit/commit/24ae0f79)) on 2026-02-02
- Xai Streaming Support by @junaid-hossain in [#392](https://github.com/InfinitiBit/graphbit/pull/392) ([310a70b](https://github.com/InfinitiBit/graphbit/commit/310a70be)) on 2026-01-30
- Anthropic Streaming Support by @junaid-hossain in [#390](https://github.com/InfinitiBit/graphbit/pull/390) ([828cd83](https://github.com/InfinitiBit/graphbit/commit/828cd830)) on 2026-01-30
- Implementation of streaming feature by @junaid-hossain in [#386](https://github.com/InfinitiBit/graphbit/pull/386) ([a787963](https://github.com/InfinitiBit/graphbit/commit/a7879633)) on 2026-01-30
- Py bridge llm config context manager resource management by @jaid-jashim in [#377](https://github.com/InfinitiBit/graphbit/pull/377) ([7c42931](https://github.com/InfinitiBit/graphbit/commit/7c429315)) on 2026-01-30
- Litellm Embeddings Implementation by @junaid-hossain in [#364](https://github.com/InfinitiBit/graphbit/pull/364) ([3c52559](https://github.com/InfinitiBit/graphbit/commit/3c525595)) on 2026-01-30
- Add Litellm as python wrapper by @junaid-hossain in [#318](https://github.com/InfinitiBit/graphbit/pull/318) ([0bd15d1](https://github.com/InfinitiBit/graphbit/commit/0bd15d12)) on 2026-01-30
- Add autogen in the graphbit benchmark module by @shahid-ullah in [#368](https://github.com/InfinitiBit/graphbit/pull/368) ([faf729c](https://github.com/InfinitiBit/graphbit/commit/faf729c1)) on 2026-01-30
- Platform-optimized memory allocators by @junaid-hossain in [#375](https://github.com/InfinitiBit/graphbit/pull/375) ([48e2091](https://github.com/InfinitiBit/graphbit/commit/48e2091d)) on 2026-01-30
- Perplexity Streaming Support by @junaid-hossain in [#391](https://github.com/InfinitiBit/graphbit/pull/391) ([b3f9f0b](https://github.com/InfinitiBit/graphbit/commit/b3f9f0be)) on 2026-01-30

### 🐛 Bug Fixes

- Resolve litellm duplication and instance id by @junaid-hossain in [#410](https://github.com/InfinitiBit/graphbit/pull/410) ([b3785dd](https://github.com/InfinitiBit/graphbit/commit/b3785dd4)) on 2026-01-30
- Fixed pyproject no root issue by @humayrakhanom in [#388](https://github.com/InfinitiBit/graphbit/pull/388) ([975f2ab](https://github.com/InfinitiBit/graphbit/commit/975f2abd)) on 2026-01-29

### 🗑️ Removed

- Remove autogen dependencies from root pyproject.toml by @junaid-hossain in [#409](https://github.com/InfinitiBit/graphbit/pull/409) ([c3aab78](https://github.com/InfinitiBit/graphbit/commit/c3aab781)) on 2026-01-30
- Remove cargo lock file by @azman-ib in [#408](https://github.com/InfinitiBit/graphbit/pull/408) ([dfc3279](https://github.com/InfinitiBit/graphbit/commit/dfc32790)) on 2026-01-30

### 📚 Documentation

- Add prominent Munich, Germany badge with official Bundesflagge colors by @jaid-jashim in [#414](https://github.com/InfinitiBit/graphbit/pull/414) ([e981cdb](https://github.com/InfinitiBit/graphbit/commit/e981cdb3)) on 2026-01-31
- Add Grant Thornton logo and elevate production case study in README by @jaid-jashim in [#413](https://github.com/InfinitiBit/graphbit/pull/413) ([d472674](https://github.com/InfinitiBit/graphbit/commit/d4726747)) on 2026-01-31
- Revise production use case for Grant Thornton by @jaid-jashim in [#412](https://github.com/InfinitiBit/graphbit/pull/412) ([6ac9561](https://github.com/InfinitiBit/graphbit/commit/6ac9561e)) on 2026-01-31
- Revise README for clarity and production usage by @jaid-jashim in [#411](https://github.com/InfinitiBit/graphbit/pull/411) ([1f1c077](https://github.com/InfinitiBit/graphbit/commit/1f1c077d)) on 2026-01-31
- Streaming feature Documentation by @junaid-hossain in [#387](https://github.com/InfinitiBit/graphbit/pull/387) ([5977b60](https://github.com/InfinitiBit/graphbit/commit/5977b60f)) on 2026-01-30
- Litellm llm providers and embeddings documentation by @junaid-hossain in [#365](https://github.com/InfinitiBit/graphbit/pull/365) ([deb63a2](https://github.com/InfinitiBit/graphbit/commit/deb63a29)) on 2026-01-30
- Update info of project license for Apache License 2.0 by @asifiibrahim in [#407](https://github.com/InfinitiBit/graphbit/pull/407) ([412ce4b](https://github.com/InfinitiBit/graphbit/commit/412ce4b4)) on 2026-01-30
- Update license to Apache License 2.0 info for pypi by @zobaid in [#403](https://github.com/InfinitiBit/graphbit/pull/403) ([df23ba9](https://github.com/InfinitiBit/graphbit/commit/df23ba94)) on 2026-01-30
- Add comment for python binding doc by @saddat-hasan in [#406](https://github.com/InfinitiBit/graphbit/pull/406) ([02c2b0e](https://github.com/InfinitiBit/graphbit/commit/02c2b0e2)) on 2026-01-30
- Add doc details for perplexity streaming support by @rifat-infinitibit in [#405](https://github.com/InfinitiBit/graphbit/pull/405) ([8b05c8d](https://github.com/InfinitiBit/graphbit/commit/8b05c8d3)) on 2026-01-30
- Add rust core contributor documentation by @shoaib-hossain in [#401](https://github.com/InfinitiBit/graphbit/pull/401) ([2233a43](https://github.com/InfinitiBit/graphbit/commit/2233a432)) on 2026-01-30
- Add Code of Conduct by @hazrat-ali in [#404](https://github.com/InfinitiBit/graphbit/pull/404) ([4d9b889](https://github.com/InfinitiBit/graphbit/commit/4d9b8893)) on 2026-01-30
- Update contributing guidelines by @md-erfanul-islam-bhuiyan in [#402](https://github.com/InfinitiBit/graphbit/pull/402) ([b322e3f](https://github.com/InfinitiBit/graphbit/commit/b322e3fe)) on 2026-01-30
- **security**: Align security policy with OSS (Apache-2.0) by @minhaz-infinitibit in [#399](https://github.com/InfinitiBit/graphbit/pull/399) ([b41bb87](https://github.com/InfinitiBit/graphbit/commit/b41bb873)) on 2026-01-30

### 🔧 Maintenance

- **github**: Add pull request template by @md-rahmat-ullah in [#415](https://github.com/InfinitiBit/graphbit/pull/415) ([1eb7f63](https://github.com/InfinitiBit/graphbit/commit/1eb7f637)) on 2026-02-01
- Azure LLM Provider Support by @anick-ib in [#400](https://github.com/InfinitiBit/graphbit/pull/400) ([5de04dd](https://github.com/InfinitiBit/graphbit/commit/5de04dd4)) on 2026-01-30
- **license**: Replace custom license with Apache-2.0 by @md-rahmat-ullah in [#394](https://github.com/InfinitiBit/graphbit/pull/394) ([7ae1366](https://github.com/InfinitiBit/graphbit/commit/7ae13660)) on 2026-01-29

---
**Total Changes**: 31
**Changes by Category**: ✨ New Features: 10 | 🐛 Bug Fixes: 2 | 🗑️ Removed: 2 | 📚 Documentation: 14 | 🔧 Maintenance: 3

## [0.6.1] - 2026-01-12

### 📚 Documentation

- Update license date info by @Jaid-Jashim in [#383](https://github.com/InfinitiBit/graphbit/pull/383) ([f4b1210](https://github.com/InfinitiBit/graphbit/commit/f4b1210)) on 2026-01-12

### 🔧 Maintenance

- Modified the workflow to be fully manual trigger ([964211c](https://github.com/InfinitiBit/graphbit/commit/964211c)) on 2025-12-26
- Add manual trigger CI workflow for js-ts by @Jaid-Jashim in [#329](https://github.com/InfinitiBit/graphbit/pull/329) ([1c1d965](https://github.com/InfinitiBit/graphbit/commit/1c1d965)) on 2025-12-14
- Add custom issue templates by @Jaid-Jashim in [#326](https://github.com/InfinitiBit/graphbit/pull/326) ([692e738](https://github.com/InfinitiBit/graphbit/commit/692e738)) on 2025-12-13
- Add templates for issue: bug report by @Jaid-Jashim in [#324](https://github.com/InfinitiBit/graphbit/pull/324) ([ede2387](https://github.com/InfinitiBit/graphbit/commit/ede2387)) on 2025-12-12

---
**Total Changes**: 5
**Changes by Category**: 📚 Documentation: 1 | 🔧 Maintenance: 4

## [0.6.0] - 2025-12-20

### 🔧 Maintenance

- Version alignment and package metadata updates
- Consolidated release preparation

---
**Total Changes**: Minor maintenance release
**Changes by Category**: 🔧 Maintenance: 1

## [0.5.3] - 2025-12-19

### 🐛 Bug Fixes

- Bugfix: Native Binding Crashes and NPM Dependencies by @Jaid-Jashim in [#302](https://github.com/InfinitiBit/graphbit/pull/302) ([6ae493f](https://github.com/InfinitiBit/graphbit/commit/6ae493f)) on 2025-12-19

### 🔧 Maintenance

- Disable optimized build and publish workflow for PyPI by @Jaid-Jashim in [#279](https://github.com/InfinitiBit/graphbit/pull/279) ([bb0bbeb](https://github.com/InfinitiBit/graphbit/commit/bb0bbeb)) on 2025-10-24

### 📚 Documentation

- Prepared Redis vector search documentation ([8135951](https://github.com/InfinitiBit/graphbit/commit/8135951)) on 2025-11-08

---
**Total Changes**: 3
**Changes by Category**: 🐛 Bug Fixes: 1 | 🔧 Maintenance: 1 | 📚 Documentation: 1

## [0.5.1] - 2025-10-24

### 🐛 Bug Fixes

- Update GraphBit version and description by @Md-Rahmat-Ullah in [#251](https://github.com/InfinitiBit/graphbit/pull/251) ([6478f0c](https://github.com/InfinitiBit/graphbit/commit/6478f0c)) on 2025-10-24

### 🔧 Maintenance

- Align Python packaging metadata and crate names for PyPI by @Md-Rahmat-Ullah in [#248](https://github.com/InfinitiBit/graphbit/pull/248) ([3ec3a69](https://github.com/InfinitiBit/graphbit/commit/3ec3a69)) on 2025-10-07
- Create build-artifacts-only workflow ([543ccec](https://github.com/InfinitiBit/graphbit/commit/543ccec)) on 2025-10-08

---
**Total Changes**: 3
**Changes by Category**: 🐛 Bug Fixes: 1 | 🔧 Maintenance: 2

## [0.5.0] - 2025-10-03

### 🐛 Bug Fixes

- Fix failing tests in replicate AI by @tanbir in [#244](https://github.com/InfinitiBit/graphbit/pull/244) ([86bb8c2](https://github.com/InfinitiBit/graphbit/commit/86bb8c2)) on 2025-10-03

### 📚 Documentation

- Update CHANGELOG.md ([64c4e12](https://github.com/InfinitiBit/graphbit/commit/64c4e12)) on 2025-09-18
- Update README.md with new links and installation instructions by @Jaid-Jashim in [#189](https://github.com/InfinitiBit/graphbit/pull/189) ([97f40df](https://github.com/InfinitiBit/graphbit/commit/97f40df)) on 2025-09-13

---
**Total Changes**: 3
**Changes by Category**: 🐛 Bug Fixes: 1 | 📚 Documentation: 2

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
