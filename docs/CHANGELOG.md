# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Placeholder for future features

### Changed
- Placeholder for future changes

### Deprecated
- Placeholder for future deprecations

### Removed
- Placeholder for future removals

### Fixed
- Placeholder for future fixes

### Security
- Placeholder for future security updates

---

## [0.1.0] - 2024-12-20

### Added
- **Core Framework**: Initial release of GraphBit declarative agentic workflow automation framework
- **Multi-LLM Support**: 
  - OpenAI integration with GPT models
  - Anthropic integration with Claude models
  - Ollama integration for local LLM inference
  - Extensible provider system for custom LLM integrations
- **Graph-Based Workflows**: 
  - Directed acyclic graph (DAG) workflow representation
  - Dependency management and topological execution
  - Node types: agent nodes, transform nodes, conditional nodes
- **Concurrent Execution**: 
  - Parallel node execution with configurable concurrency limits
  - Async/await support throughout the framework
  - Intelligent dependency resolution
- **Type Safety**: 
  - Strong typing with comprehensive validation
  - UUID-based identifiers for all components
  - JSON schema validation for LLM outputs
- **Python Bindings**: 
  - Full Python API via PyO3
  - Async support in Python
  - Integration examples for FastAPI, Django, and Jupyter
- **CLI Tool** (`graphbit`):
  - Project initialization with `graphbit init`
  - Workflow validation with `graphbit validate`
  - Workflow execution with `graphbit run`
  - Configuration management
  - Debug and verbose modes
- **Declarative Configuration**: 
  - JSON-based workflow definitions
  - Environment variable support
  - Custom configuration files
- **Error Handling & Reliability**:
  - Built-in retry logic with exponential backoff
  - Comprehensive error types and handling
  - Graceful failure recovery
- **Usage Tracking**:
  - Token usage monitoring
  - Cost estimation for API-based providers
  - Performance metrics collection
- **Documentation & Examples**:
  - Comprehensive README with architecture overview
  - Ollama integration guide
  - Testing guide with benchmarking
  - Extensive example collection
  - Python and Rust API documentation
- **Testing Infrastructure**:
  - Unit and integration tests
  - Benchmarking suite
  - Mock LLM providers for testing
  - CI/CD configuration

### Technical Details
- **Language**: Rust with Python bindings
- **Minimum Rust Version**: 1.70+
- **Architecture**: Modular design with clear separation of concerns
- **Core Modules**:
  - `agents/`: Agent abstraction and LLM-backed implementations
  - `llm/`: Multi-provider LLM integration
  - `graph/`: Graph-based workflow representation and execution
  - `validation/`: JSON schema and custom validation system
  - `workflow/`: Workflow execution engine
  - `types/`: Strong type system
  - `errors/`: Comprehensive error handling

### Dependencies
- **Rust Workspace Dependencies**:
  - `tokio` 1.0+ for async runtime
  - `serde` 1.0+ for serialization
  - `anyhow` and `thiserror` for error handling
  - `uuid` 1.0+ for unique identifiers
  - `petgraph` 0.6+ for graph algorithms
  - `reqwest` 0.11+ for HTTP client
  - `clap` 4.0+ for CLI interface
  - `pyo3` 0.20+ for Python bindings
  - `chrono` 0.4+ for date/time handling


### Repository
- https://github.com/InfinitiBit/graphbit

---

## Contributing

When adding entries to this changelog:

1. **Follow the format**: Use the sections (Added, Changed, Deprecated, Removed, Fixed, Security)
2. **Be specific**: Include enough detail for users to understand the impact
3. **Link to issues**: Reference GitHub issues/PRs where applicable
4. **Date releases**: Use ISO date format (YYYY-MM-DD)
5. **Use semantic versioning**: Follow semver for version numbers

### Types of Changes
- **Added** for new features
- **Changed** for changes in existing functionality  
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** in case of vulnerabilities 
