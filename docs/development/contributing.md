# Contributing to GraphBit

Welcome to the GraphBit project! We're excited that you're interested in contributing to our high-performance AI agent workflow framework.

> **Note**: This document provides a quick overview for documentation purposes. For complete development setup and detailed contributing guidelines, please see the main [CONTRIBUTING.md](../../CONTRIBUTING.md) file in the project root.

## Quick Start for Contributors

### 1. Development Setup

```bash
# Clone the repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit

# Set up development environment
make dev-setup

# Install pre-commit hooks
make pre-commit-install

# Run tests to verify setup
make test
```

### 2. Project Structure

```
graphbit/
├── core/                 # Rust core library
├── python/              # Python bindings
├── src/                 # CLI application
├── docs/                # Documentation (this folder)
├── examples/            # Example workflows
├── tests/               # Integration tests
└── benchmarks/          # Performance benchmarks
```

## Ways to Contribute

### 🐛 Bug Reports
- Use GitHub issues to report bugs
- Include system information and steps to reproduce
- Provide minimal reproducible examples

### ✨ Feature Requests
- Discuss new features in GitHub discussions
- Consider backward compatibility
- Include use cases and rationale

### 📚 Documentation
- Improve existing documentation
- Add examples and tutorials
- Fix typos and improve clarity

### 🧹 Code Contributions
- Follow coding standards (see main CONTRIBUTING.md)
- Write tests for new features
- Ensure all quality checks pass

## Development Workflow

1. **Fork** the repository on GitHub
2. **Create** a feature branch from `main`
3. **Make** your changes following our coding standards
4. **Test** your changes thoroughly
5. **Submit** a pull request

### Code Quality

We maintain high code quality standards:

- **Rust**: `cargo fmt`, `cargo clippy`, `cargo test`
- **Python**: `black`, `isort`, `flake8`, `mypy`, `pytest`
- **Pre-commit**: Automated quality checks on every commit

### Testing

```bash
# Run all tests
make test

# Run specific test categories
cargo test --workspace          # Rust tests
python -m pytest tests/ -v     # Python tests
make integration-test           # Integration tests
```

## Architecture Overview

GraphBit uses a three-tier architecture:

- **Python API**: PyO3 bindings with async support
- **CLI Tool**: Project management and execution
- **Rust Core**: Workflow engine, agents, LLM providers

Key components:
- **Workflow Engine**: Graph execution and dependency management
- **Agent System**: AI-powered processing components
- **LLM Providers**: Multi-provider abstraction (OpenAI, Anthropic, etc.)
- **Type System**: Strong typing with comprehensive validation

## Coding Standards

### Rust Code
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Write comprehensive documentation
- Include unit tests for new functionality

### Python Code
- Follow PEP 8 with 200-character line length
- Use type hints for all public APIs
- Write docstrings for classes and functions
- Include type checking with `mypy`

### Documentation
- Write clear, concise documentation
- Include code examples
- Keep examples up-to-date
- Use consistent formatting

## Performance Considerations

When contributing code:
- Consider memory allocation patterns
- Use async/await for I/O operations
- Implement proper error handling
- Add benchmarks for performance-critical code

## Security Guidelines

- Never commit API keys or secrets
- Validate all inputs
- Use secure communication (HTTPS/TLS)
- Follow secure coding practices

## Community Guidelines

- Be respectful and inclusive
- Help newcomers get started
- Share knowledge and expertise
- Follow our code of conduct

## Getting Help

- **Documentation**: Check the [docs](../README.md) first
- **GitHub Issues**: Search existing issues
- **Discussions**: Ask questions in GitHub discussions
- **Discord**: Join our community Discord (if available)

## Recognition

Contributors are recognized in:
- CHANGELOG.md for significant contributions
- GitHub contributors page
- Release notes for major features

## Next Steps

Ready to contribute? Here are some good first steps:

1. **Read** the full [CONTRIBUTING.md](../../CONTRIBUTING.md)
2. **Browse** the [good first issue](https://github.com/InfinitiBit/graphbit/labels/good%20first%20issue) label
3. **Join** our community discussions
4. **Start** with documentation improvements or small bug fixes

Thank you for contributing to GraphBit! 🚀

---

For complete development setup instructions, coding standards, and detailed guidelines, please refer to the main [CONTRIBUTING.md](../../CONTRIBUTING.md) file. 