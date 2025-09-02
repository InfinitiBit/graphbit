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

## Next Steps

After successful installation:
1. Run tests: `make test`
2. Try examples: `make examples`
3. Start development: `make dev-setup`

For more commands, run: `make help`
