# Contributing to GraphBit

Thank you for your interest in contributing to GraphBit! This guide will help you get started with development and understand our contribution process.

### Development Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/InfinitiBit/graphbit.git
   cd graphbit
   ```

2. **Set Up Development Environment**
   ```bash
   
   # Or manually:
   # Install Python dependencies
   poetry install --with dev,benchmarks
   
   # Build Rust workspace
   cargo build --workspace
   
   # Install pre-commit hooks
   pre-commit install
   ```

3. **Build the Project**
   ```bash
   # Build Rust core
   cargo build
   
   # Run tests
   cargo test
   
   # Build Python bindings
   cd python
   maturin develop

   # Test Python import
   python -c "import graphbit; print(graphbit.version())"
   ```

4. **Verify Installation**
   ```bash
   # Test CLI
   cargo run -- version
   
   # Test Python bindings
   python -c "import graphbit; print(graphbit.version())"
   
   # Run pre-commit on all files
   pre-commit run --all-files
   ```

## 📁 Project Structure

```
graphbit/
├── core/                 # Rust core library
│   ├── src/
│   │   ├── agents.rs     # Agent system
│   │   ├── llm/          # LLM providers
│   │   ├── graph.rs      # Workflow graphs
│   │   ├── validation.rs # Validation system
│   │   ├── workflow.rs   # Execution engine
│   │   └── ...
│   └── Cargo.toml
├── graphbit/          # Python bindings
│   ├── src/
│   └── Cargo.toml
├── examples/             # Example workflows
├── docs/               # Documentation
└── tests/              # Integration tests
```

## 🛠️ Development Workflow

### 1. Code Quality and Style

We use **pre-commit hooks** to ensure consistent code quality across both Rust and Python components. These hooks automatically run when you commit and catch issues early.

#### Pre-commit Setup

```bash
# Install and configure pre-commit hooks (included in dev-setup)
make pre-commit-install

# Run all hooks manually
make pre-commit-run

# Update hook versions
make pre-commit-update
```

#### What Pre-commit Checks

**Rust Code**:
- **Formatting**: `cargo fmt` - Ensures consistent code style
- **Linting**: `cargo clippy` - Catches common mistakes and improves code quality
- **Security**: `cargo audit` - Checks for security vulnerabilities
- **Build**: `cargo check` - Ensures code compiles correctly

**Python Code**:
- **Formatting**: `black` - Code formatting with 200-character line length
- **Import Sorting**: `isort` - Sorts and organizes imports
- **Linting**: `flake8` with plugins - Code quality and style checks
- **Type Checking**: `mypy` - Static type checking
- **Security**: `bandit` - Security vulnerability scanning

**General**:
- **Secret Detection**: `detect-secrets` - Prevents accidental commit of API keys/secrets
- **Spell Checking**: `typos` - Catches typos in code and documentation
- **Markdown**: `markdownlint` - Ensures consistent documentation formatting
- **Commit Messages**: `commitizen` - Enforces conventional commit format

#### Manual Code Quality Commands

If you need to run tools manually:

```bash
# Rust
cargo fmt                    # Format code
cargo clippy                 # Run linter
cargo check --workspace     # Check compilation

# Python
black tests/ benchmarks/     # Format Python code
isort tests/ benchmarks/     # Sort imports
flake8 tests/ benchmarks/    # Lint Python code
mypy tests/ --ignore-missing-imports  # Type checking

# All quality checks
make all-checks              # Runs format-check, lint, test, secrets
```

#### Bypassing Pre-commit (Not Recommended)

In rare cases, you might need to bypass pre-commit:

```bash
# Skip pre-commit hooks (use sparingly)
git commit --no-verify -m "emergency fix"

# Skip specific hooks
SKIP=detect-secrets git commit -m "commit message"
```

### 2. Testing

We maintain comprehensive test coverage:

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_workflow_execution

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

**Integration Tests**:
```bash
# Run integration tests
cargo test --test integration

# Test Python bindings
cd graphbit
maturin develop
python -m pytest tests/ -v
```

### 3. Documentation

Update documentation for any new features:

```bash
# Generate docs
cargo doc --open

# Check doc links
cargo doc --no-deps
```

## 🎯 Contribution Guidelines

### Issue Types

- **🐛 Bug Reports**: Issues with existing functionality
- **✨ Feature Requests**: New capabilities or enhancements
- **📚 Documentation**: Improvements to docs or examples
- **🧹 Maintenance**: Code cleanup, refactoring, dependencies

### Pull Request Process

1. **Fork the Repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/InfinitiBit/graphbit.git
   ```

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Write code following our style guidelines
   - Add tests for new functionality
   - Update documentation as needed
   - Ensure all tests pass

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   
   # Or use commitizen for guided commit messages
   cz commit
   ```

   **Commit Message Format** (enforced by pre-commit):
   
   We follow [Conventional Commits](https://www.conventionalcommits.org/) format:
   ```
   <type>(<scope>): <description>
   
   [optional body]
   
   [optional footer(s)]
   ```

   **Types**:
   - `feat:` ✨ New features
   - `fix:` 🐛 Bug fixes
   - `docs:` 📚 Documentation changes
   - `style:` 💎 Code style changes (formatting, no logic changes)
   - `refactor:` 📦 Code refactoring (no feature change)
   - `perf:` 🚀 Performance improvements
   - `test:` 🚨 Test additions/modifications
   - `build:` 🛠 Build system or external dependency changes
   - `ci:` ⚙️ CI configuration changes
   - `chore:` ♻️ Maintenance tasks
   - `revert:` 🗑 Reverts a previous commit

   **Examples**:
   ```bash
   feat(workflow): add dynamic node generation
   fix(llm): handle timeout errors in OpenAI provider
   docs(api): update Python bindings documentation
   test(core): add integration tests for workflow execution
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   # Create pull request on GitHub
   ```

### Code Review Checklist

Before submitting your PR, ensure:

- [ ] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [ ] All tests pass (`cargo test --workspace`)
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] Breaking changes are documented
- [ ] Performance impact is considered

## 🧪 Testing Guidelines

### Unit Tests

Write unit tests for individual functions and modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("Test", "Description");
        assert_eq!(workflow.name, "Test");
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Create integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use graphbit_core::*;

#[tokio::test]
async fn test_full_workflow_execution() {
    // Test complete workflow scenarios
}
```

## 🐍 Python Integration Tests

GraphBit includes comprehensive Python integration tests that validate all major functionality areas. These tests focus on **integration** rather than output validation, ensuring that components work together correctly.

### Test Structure

Located in `tests/python_integration_tests/`, the test suite includes:

```
tests/python_integration_tests/
├── tests_embeddings.py        # Embedding providers integration
├── tests_llm.py              # LLM models and configurations  
├── tests_static_workflow.py   # Static workflow functionality
├── tests_dynamic_workflow.py  # Dynamic workflow and auto-completion
├── test_runner.py            # Comprehensive test runner
├── run_tests.sh             # Shell script for easy execution
├── pytest.ini              # Pytest configuration
└── README.md               # Detailed test documentation
```

### Test Categories

#### 1. **Embedding Integration Tests** (`tests_embeddings.py`)
- **OpenAI Embeddings**: Configuration, single/multiple text embedding, dimensions, consistency
- **HuggingFace Embeddings**: Configuration, text embedding, API integration  
- **Utility Functions**: Cosine similarity, embedding requests/responses
- **Cross-Provider Testing**: Compatibility across different embedding providers

#### 2. **LLM Integration Tests** (`tests_llm.py`)
- **OpenAI Models**: GPT-4, GPT-3.5 configuration and workflow execution
- **Anthropic Models**: Claude model configuration and integration
- **HuggingFace Models**: Open-source model integration
- **Advanced Configuration**: Retry policies, circuit breakers, memory pool configurations
- **Agent Capabilities**: Predefined and custom agent capabilities
- **Multi-Provider Support**: Cross-provider workflow validation

#### 3. **Static Workflow Tests** (`tests_static_workflow.py`)
- **Workflow Creation**: Various node types (agent, condition, transform, delay, document loader)
- **Node Connections**: Data flow, control flow, conditional connections
- **Workflow Validation**: Structure validation and error handling
- **Serialization**: JSON serialization/deserialization with roundtrip testing
- **Builder Pattern**: Workflow construction using builder pattern
- **Execution**: Both synchronous and asynchronous workflow execution

#### 4. **Dynamic Workflow Tests** (`tests_dynamic_workflow.py`)
- **Dynamic Configuration**: Auto-node generation settings and customization
- **Graph Manager**: Dynamic node generation and intelligent workflow completion
- **Analytics**: Performance tracking and generation statistics
- **Auto-Completion Engine**: Intelligent workflow completion with objectives
- **Hybrid Workflows**: Integration of static and dynamic workflow components
- **Performance Testing**: Scalability configurations and optimization settings

### Running Python Integration Tests

#### Prerequisites
```bash
# Activate conda environment
conda activate graphbit

# Set API keys (optional but recommended)
export OPENAI_API_KEY="your-openai-api-key"
export ANTHROPIC_API_KEY="your-anthropic-api-key"  
export HUGGINGFACE_API_KEY="your-huggingface-api-key"

# Build GraphBit (required before running tests)
cargo build --release
maturin develop --release
```

#### Running Tests

```bash
# Navigate to test directory
cd tests/python_integration_tests

# Run all integration tests
python test_runner.py

# Run specific test categories
python test_runner.py embedding    # Embedding tests only
python test_runner.py llm         # LLM tests only
python test_runner.py static      # Static workflow tests
python test_runner.py dynamic     # Dynamic workflow tests

# Using the shell script
./run_tests.sh all                # All tests
./run_tests.sh embedding          # Category-specific
./run_tests.sh info              # Show test information

# Using pytest directly
pytest tests_embeddings.py -v                    # Single file
pytest -k "openai" -v                           # Pattern matching
pytest tests_llm.py::TestOpenAILLM -v           # Specific class
```

#### Smart Test Features

- **Graceful Skipping**: Tests automatically skip when API keys are missing or services unavailable
- **Environment Detection**: Automatic detection of available providers and configurations  
- **Comprehensive Reporting**: Detailed test summaries with timing and status information
- **Error Handling**: Robust error handling with informative failure messages

### Contributing to Python Integration Tests

When adding new Python integration tests:

1. **Follow Existing Patterns**:
   ```python
   class TestNewFeature:
       """Integration tests for new feature"""
       
       @pytest.fixture
       def api_key(self):
           """Get API key from environment"""
           api_key = os.getenv("API_KEY")
           if not api_key:
               pytest.skip("API_KEY not set")
           return api_key
       
       def test_feature_creation(self, api_key):
           """Test feature creation and properties"""
           # Test implementation
   ```

2. **Use Appropriate Fixtures**: Leverage pytest fixtures for setup and teardown
3. **Handle Missing Dependencies**: Use `pytest.skip()` for missing API keys or services
4. **Add Comprehensive Docstrings**: Document what each test validates
5. **Update Test Runner**: Add new test modules to `test_runner.py` if needed
6. **Update Documentation**: Update the README.md in the test directory

### Test Quality Guidelines

- **Focus on Integration**: Test component interaction rather than isolated functionality
- **API Structure Validation**: Verify that APIs work correctly rather than specific outputs
- **Cross-Provider Compatibility**: Ensure consistent behavior across different providers
- **Performance Awareness**: Include basic performance and scalability considerations
- **Error Scenarios**: Test graceful handling of common error conditions

### Example Test Output

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

🎉 All integration tests completed successfully!
```

### Example Tests

Test examples to ensure they work:

```bash
# Validate example workflows
cargo run -- validate examples/workflows/data-analysis.json
cargo run -- validate examples/workflows/content-pipeline.json
```

## 🔧 Adding New Features

### 1. LLM Providers

To add a new LLM provider:

1. Create `core/src/llm/your_provider.rs`
2. Implement `LlmProviderTrait`
3. Add to `LlmProviderFactory`
4. Add configuration options
5. Write tests

**Example**:
```rust
// core/src/llm/your_provider.rs
pub struct YourProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl LlmProviderTrait for YourProvider {
    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        // Implementation
    }
    
    // Other trait methods...
}
```

### 2. Node Types

To add a new workflow node type:

1. Add variant to `NodeType` enum in `graph.rs`
2. Update node execution in `workflow.rs`
3. Add validation logic
4. Update Python bindings if needed
5. Add documentation and examples

### 3. Validation Rules

To add custom validation:

1. Implement `CustomValidator` trait
2. Register in `TypeValidator`
3. Add configuration options
4. Write comprehensive tests

## 🐛 Debugging

### Logging

Enable detailed logging for debugging:

```bash
# Set log level
export RUST_LOG=debug

# Run with logging
cargo run -- run examples/workflows/example.json
```

### Common Issues

**Build Failures**:
```bash
# Clean and rebuild
cargo clean
cargo build

# Check Rust version
rustc --version
```

**Python Binding Issues**:
```bash
# Rebuild Python bindings
cd graphbit
maturin develop --release
```

## 📊 Performance Considerations

### Benchmarking

Run benchmarks to measure performance:

```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
cargo bench
```

### Memory Usage

Monitor memory usage:

```bash
# Install valgrind (Linux)
sudo apt install valgrind

# Run with memory checking
valgrind --tool=memcheck cargo run -- run workflow.json
```

### Profiling

Profile CPU usage:

```bash
# Install perf (Linux)
sudo apt install linux-tools-generic

# Profile execution
perf record cargo run -- run workflow.json
perf report
```

## 🚀 Release Process

### Version Bumping

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create git tag
4. Build and test release

```bash
# Tag release
git tag v0.2.0
git push origin v0.2.0

# Build release
cargo build --release
```

### Publishing

```bash
# Publish to crates.io
cargo publish -p graphbit-core
cargo publish -p graphbit

# Build Python wheels
cd graphbit
maturin build --release
maturin upload
```

## 🎯 Priority Areas

We're especially looking for contributions in:

- **🧠 Additional LLM Providers**: Groq, Cohere, local models
- **🔧 Advanced Node Types**: Database connectors, API integrators
- **📊 Monitoring & Observability**: Metrics, tracing, dashboards
- **🚀 Performance Optimizations**: Streaming, caching, parallelization
- **📱 UI/UX**: Web interface, workflow visualizer
- **📚 Documentation**: Tutorials, API docs, best practices

## 💬 Getting Help

- **Discord**: [Join our community](https://discord.gg/graphbit)
- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Email**: [maintainers@graphbit.dev](mailto:maintainers@graphbit.dev)


Thank you for contributing to GraphBit! Together, we're building the future of agentic workflow automation. 🚀 
