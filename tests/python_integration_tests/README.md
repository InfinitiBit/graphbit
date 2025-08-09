# GraphBit Python Integration Tests

This directory contains comprehensive integration tests for the GraphBit Python library, covering all major functionality areas.

## Test Categories

### 1. Document Loader Integration Tests (`tests_document_loader.py`)
- **Document Loading**: Text, JSON, CSV, XML, HTML document loading and content extraction
- **Configuration**: DocumentLoaderConfig creation, validation, and property management
- **File Handling**: Unicode support, empty files, large files, size limits
- **Validation**: Input validation, file existence checks, type validation
- **Error Handling**: Non-existent files, unsupported types, size limit exceeded
- **Performance**: Multiple document loading, large file processing
- **Utility Functions**: Document type detection, source validation, metadata extraction

### 2. Embedding Integration Tests (`tests_embeddings.py`)
- **OpenAI Embeddings**: Configuration, single/multiple text embedding, dimensions, consistency
- **HuggingFace Embeddings**: Configuration, text embedding, API integration
- **Utility Functions**: Cosine similarity, embedding requests/responses
- **Cross-Provider Comparison**: Semantic similarity across different providers

### 2. LLM Integration Tests (`tests_llm.py`)
- **OpenAI Models**: GPT-4, GPT-3.5 configuration and workflow execution
- **Anthropic Models**: Claude model configuration and integration
- **HuggingFace Models**: Open-source model integration
- **Configuration**: Retry policies, circuit breakers, pool configurations
- **Agent Capabilities**: Predefined and custom capabilities
- **Cross-Provider**: Multi-provider workflow validation

### 3. Static Workflow Integration Tests (`tests_static_workflow.py`)
- **Workflow Creation**: Basic workflow construction with various node types
- **Node Types**: Agent, condition, transform, delay, document loader nodes
- **Connections**: Data flow, control flow, conditional connections
- **Validation**: Workflow structure validation
- **Serialization**: JSON serialization/deserialization
- **Builder Pattern**: Workflow construction using builder pattern
- **Execution**: Synchronous and asynchronous workflow execution

### 4. Dynamic Workflow Integration Tests (`tests_dynamic_workflow.py`)
- **Dynamic Configuration**: Auto-node generation settings
- **Graph Manager**: Dynamic node generation and workflow completion
- **Analytics**: Performance tracking and generation statistics
- **Auto-Completion**: Intelligent workflow completion
- **Integration**: Hybrid static/dynamic workflows
- **Performance**: Scalability and optimization configurations

### 5. Executor Integration Tests (`tests_executor.py`)
- **Executor Configuration**: Standard, high-throughput, low-latency, memory-optimized modes
- **Statistics Tracking**: Execution metrics, performance monitoring, stats reset
- **Async Operations**: Async workflow execution, concurrent executions
- **Runtime Configuration**: Thread management, timeout handling, performance tuning

### 6. Async Execution Tests (`tests_async_execution.py`)
- **Async LLM Operations**: Async completion, batch processing, chat optimization
- **Async Workflow Execution**: Async executor operations, concurrent workflows
- **Performance Comparison**: Async vs sync performance characteristics
- **Error Handling**: Async error propagation and recovery

### 7. System Functions Tests (`tests_system_functions.py`)
- **System Initialization**: Library initialization, version management, logging configuration
- **System Information**: Runtime statistics, health monitoring, resource tracking
- **Runtime Configuration**: Worker threads, memory management, performance tuning
- **Health Checks**: Component health monitoring, system diagnostics

### 8. Validation Tests (`tests_validation.py`)
- **API Key Validation**: Comprehensive testing across all providers (OpenAI, Anthropic, HuggingFace)
- **Workflow Structure Validation**: Circular dependency detection, invalid connections, orphaned nodes
- **Parameter Validation**: LLM parameters, executor settings, batch operations
- **Node Validation**: Agent, condition, and transform node parameter validation
- **Embedding Validation**: Input validation, similarity computation error handling
- **Cross-Component Validation**: System-wide validation scenarios

### 9. Error Handling Tests (`tests_error_handling.py`)
- **Network Error Handling**: Invalid API keys, connection timeouts, rate limiting
- **Resource Exhaustion**: Memory pressure, concurrent request limits, large input handling
- **Workflow Error Recovery**: Execution errors, invalid workflow structures, empty workflows
- **Async Error Handling**: Async operation error scenarios, error propagation
- **System Error Recovery**: Runtime errors, configuration errors, graceful degradation

### 10. Performance Monitoring Tests (`tests_performance_monitoring.py`)
- **Client Statistics**: LLM client performance tracking, response time monitoring
- **Executor Statistics**: Workflow execution metrics, performance mode comparison
- **System Performance**: Resource utilization, concurrent operations monitoring
- **Performance Benchmarking**: Completion vs batch performance, cross-component benchmarks

### 11. Complex Workflow Pattern Tests (`tests_complex_workflow.py`)
- **Multi-Branch Workflows**: Parallel processing, conditional branching, nested conditionals
- **Transformation Chains**: Sequential and parallel data transformations, conditional transforms
- **Error Handling Patterns**: Try-catch workflows, circuit breaker patterns, retry with backoff
- **Data Pipeline Patterns**: ETL workflows, stream processing, complex pipeline execution
- **Workflow Composition**: Nested patterns, hierarchical structures, pattern performance

### 12. Runtime Configuration Tests (`tests_runtime_configuration.py`)
- **Advanced Configuration**: Pre/post-init configuration, multiple configuration attempts
- **Runtime State Management**: Initialization states, uptime tracking, health monitoring
- **Concurrent Operations**: Concurrent initialization, configuration, system info access
- **Resource Management**: Memory configuration, thread pool management, performance under load
- **Error Recovery**: Configuration error recovery, initialization error handling, graceful degradation

## Prerequisites

### Required Dependencies
```bash
# Install GraphBit with development dependencies
pip install -e .
pip install pytest pytest-asyncio
```

### API Keys (Optional but Recommended)
Set environment variables for full test coverage:

```bash
export OPENAI_API_KEY="your-openai-api-key"
export ANTHROPIC_API_KEY="your-anthropic-api-key"  
export HUGGINGFACE_API_KEY="your-huggingface-api-key"
```

**Note**: Tests will skip API-dependent functionality if keys are not provided.

## Running Tests

### Run All Integration Tests
```bash
# Using the test runner (recommended)
python test_runner.py

# Using pytest directly
pytest -v
```

### Run Specific Test Categories
```bash
# Embeddings only
python test_runner.py embedding

# LLM only  
python test_runner.py llm

# Static workflows only
python test_runner.py static

# Dynamic workflows only
python test_runner.py dynamic

# Validation tests only
python test_runner.py validation

# Performance monitoring only
python test_runner.py performance

# Error handling tests only
python test_runner.py error

# Complex workflow patterns only
python test_runner.py complex
```

### Run Individual Test Files
```bash
# Embedding tests
pytest tests_embeddings.py -v

# LLM tests
pytest tests_llm.py -v

# Static workflow tests
pytest tests_static_workflow.py -v

# Dynamic workflow tests
pytest tests_dynamic_workflow.py -v

# Comprehensive validation tests
pytest tests_validation_comprehensive.py -v

# Advanced error handling tests
pytest tests_error_handling_advanced.py -v

# Performance monitoring tests
pytest tests_performance_monitoring.py -v

# Complex workflow pattern tests
pytest tests_complex_workflow_patterns.py -v

# Advanced runtime configuration tests
pytest tests_runtime_configuration_advanced.py -v
```

### Run Specific Test Classes or Methods
```bash
# Run specific test class
pytest tests_embeddings.py::TestOpenAIEmbeddings -v

# Run specific test method
pytest tests_llm.py::TestOpenAILLM::test_openai_gpt4_config_creation -v

# Run tests matching pattern
pytest -k "embedding" -v
pytest -k "workflow and static" -v
```

## Build and Environment Setup

Before running tests, ensure GraphBit is properly built:

```bash
# Activate conda environment
conda activate graphbit

# Set OpenAI API key
export OPENAI_API_KEY=your_actual_api_key_here

# Build Rust components
cargo build --release

# Build Python bindings  
maturin develop --release
```

## Test Features

### Comprehensive Coverage
- **Configuration Testing**: All provider configurations and customizations
- **Functionality Testing**: Core features like embedding generation, LLM interactions
- **Integration Testing**: Cross-component interaction and workflow execution
- **Error Handling**: Graceful handling of API failures and missing keys
- **Performance Testing**: Basic performance and scalability considerations

### Smart Skipping
Tests automatically skip when:
- Required API keys are not available
- Provider services are unreachable
- Dependencies are missing
- Environment setup fails

### Detailed Reporting
- Individual test status and timing
- API key availability status
- Comprehensive error reporting
- Test execution summary

## Example Output

```
============================================================
GraphBit Python Integration Test Suite
============================================================
✓ GraphBit initialized (version: 0.1.0)

API Key Status:
  OPENAI_API_KEY: ✓ Available
  ANTHROPIC_API_KEY: ✗ Not set
  HUGGINGFACE_API_KEY: ✗ Not set

============================================================
INTEGRATION TEST SUMMARY
============================================================
  ✓ PASSED |     5.23s | Embedding Integration Tests
  ✓ PASSED |     3.45s | LLM Integration Tests  
  ✓ PASSED |     2.17s | Static Workflow Integration Tests
  ✓ PASSED |     4.89s | Dynamic Workflow Integration Tests
------------------------------------------------------------
Total: 4/4 modules passed
Total execution time: 15.74 seconds

All integration tests completed successfully!
```

## Troubleshooting

### Common Issues

1. **GraphBit not found**: Ensure `maturin develop --release` was run
2. **API key errors**: Set required environment variables
3. **Import errors**: Activate the correct conda environment
4. **Build errors**: Run `cargo build --release` first

### Debug Mode
```bash
# Run with verbose output
pytest -v -s

# Run with debug information
pytest --tb=long -v

# Run specific failing test
pytest tests_embeddings.py::TestOpenAIEmbeddings::test_openai_single_embedding -v -s
```

## Contributing

When adding new integration tests:

1. Follow the existing pattern of test classes and fixtures
2. Use appropriate pytest markers (`@pytest.mark.asyncio`, `@pytest.mark.integration`)
3. Handle missing API keys gracefully with `pytest.skip()`
4. Add comprehensive docstrings
5. Update this README with new test categories

## Notes

- Tests focus on **integration** rather than output validation
- Many tests verify API structure and successful execution rather than specific results
- Cross-provider tests validate compatibility and consistency
- Performance tests provide basic metrics without strict assertions 
