# GraphBit Installation Guide

## Quick Start

For a complete, automated installation that handles all dependencies and builds:

```bash
make install
```

This single command will:
1. ✅ **Check and install Poetry** if not present
2. 🐍 **Validate Python environment** (conda/venv/poetry)
3. 📦 **Install Python dependencies** with Poetry
4. 🦀 **Fetch Rust dependencies** with Cargo
5. 🔨 **Build Rust workspace** (`cargo build --workspace`)
6. 🐍 **Install Python extension** in development mode (`maturin develop`)
7. ✅ **Validate installation** with comprehensive checks

## Alternative Install Commands

### Quick Install (Skip Environment Checks)
```bash
make install-quick
```
Use this if you already have Poetry and your environment set up correctly.

### Force Install (No Safety Checks)
```bash
make install-force
```
⚠️ **Use with caution** - bypasses all safety checks.

### Development Build Only
```bash
make build-dev
```
Just builds the Rust workspace and installs the Python extension module.

## What Gets Installed

### Python Dependencies
- **Core**: click, rich, typer, python-dotenv
- **AI/ML**: openai, anthropic, langchain, langgraph, llama-index, crewai, pydantic-ai
- **Development**: pytest, black, flake8, mypy, pre-commit
- **Benchmarking**: numpy, pandas, matplotlib, seaborn

### Rust Components
- All workspace crates are built in development mode
- Python extension module is installed via Maturin

## Environment Support

The install process automatically detects and works with:
- **Poetry environments** (recommended)
- **Conda environments** (graphbit environment)
- **Virtual environments** (.venv directory)

## Troubleshooting

### Poetry Not Found
The installer will automatically download and install Poetry. If this fails:
1. Visit: https://python-poetry.org/docs/#installation
2. Install manually, then run `make install` again

### Environment Issues
- **Conda**: Ensure `graphbit` environment exists (`conda create -n graphbit python=3.11`)
- **Venv**: Create with `python -m venv .venv` and activate
- **Poetry**: Will create environment automatically

### Build Failures
- Ensure Rust toolchain is installed: https://rustup.rs/
- Check that Python 3.10+ is available
- Verify all dependencies in pyproject.toml are compatible

## Validation

After installation, the system validates:
- ✅ Poetry configuration
- ✅ Python environment functionality  
- ✅ Rust toolchain availability
- ✅ Core Python dependencies
- ✅ GraphBit Python extension module import

## Testing

### Comprehensive Test Suite with Coverage
```bash
make test        # Run all tests (Rust + Python) with coverage reporting
make tests       # Alias for 'test'
```

All test commands now include comprehensive coverage reporting:
- **Rust tests**: Use `cargo llvm-cov` with HTML output
- **Python tests**: Use `pytest` with branch coverage and HTML reports
- **Environment**: Automatically sets `TEST_REMOTE_URLS=true` for Python tests

### Granular Test Commands
```bash
make test-rust                    # All Rust tests with llvm-cov coverage
make test-python                  # All Python tests with pytest coverage
make test-rust-unit              # Only Rust unit tests with coverage
make test-rust-integration       # Only Rust integration tests with coverage
make test-python-unit            # Only Python unit tests with coverage
make test-python-integration     # Only Python integration tests with coverage
```

### Advanced Testing
```bash
make test-all      # Comprehensive tests with pre-validation and coverage
make test-coverage # Dedicated coverage reporting command
make test-quick    # Quick tests with coverage for development iteration
```

### Coverage Reports
After running tests, coverage reports are generated:
- **Rust**: HTML coverage report via `cargo llvm-cov`
- **Python**: HTML coverage report via `pytest --cov` with branch coverage
- **Exclusions**: Automatically excludes `core/src/llm/`, `core/src/embeddings.rs`, and `python/src/`
- **Cross-Platform**: Optimized for both Windows PowerShell and Unix environments

### Cross-Platform Support
The test commands automatically adapt to your platform:
- **Windows**: Uses PowerShell with proper environment variable syntax
- **Unix/Linux/macOS**: Uses bash with export syntax
- **Environment Variables**: `TEST_REMOTE_URLS=true` is set automatically on both platforms
- **Coverage Config**: Handles platform-specific coverage configuration seamlessly

## Next Steps

After successful installation:
1. **Verify setup**: `make test` - Run comprehensive test suite
2. **Try examples**: `make examples` - Run example workflows
3. **Development**: `make dev-setup` - Additional development tools
4. **Get help**: `make help` - See all available commands

## Quick Development Workflow

```bash
# Initial setup (run once)
make install

# Development cycle
make test-quick      # Fast testing during development
make format          # Format code
make lint           # Check code quality
make test           # Full test suite before commits
```
