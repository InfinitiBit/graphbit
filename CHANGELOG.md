# Changelog

## [0.1.0] - 2024-06-11

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
