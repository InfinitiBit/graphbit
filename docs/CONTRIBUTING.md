# Contributing to GraphBit

Thank you for your interest in contributing to GraphBit! This guide will help you get started with development and understand our contribution process.

### Development Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/InfinitiBit/graphbit.git
   cd graphbit
   ```

2. **Build the Project**
   ```bash
   # Build Rust core
   cargo build
   
   # Run tests
   cargo test
   
   # Build Python bindings
   cd graphbit
   maturin develop

   # Test Python import
   python -c "import graphbit; print(graphbit.version())"
   ```

3. **Verify Installation**
   ```bash
   # Test CLI
   cargo run -- version
   
   # Test Python bindings
   python -c "import graphbit; print(graphbit.version())"
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

### 1. Code Style

We follow standard Rust formatting and linting practices:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for common issues
cargo check
```

**Python Code**: Follow Black formatting:
```bash
# Install Black
pip install black

# Format Python code
black examples/ tests/
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
   ```

   **Commit Message Format**:
   - `feat:` New features
   - `fix:` Bug fixes
   - `docs:` Documentation changes
   - `test:` Test additions/modifications
   - `refactor:` Code refactoring
   - `chore:` Maintenance tasks

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


---

Thank you for contributing to GraphBit! Together, we're building the future of agentic workflow automation. 🚀 
