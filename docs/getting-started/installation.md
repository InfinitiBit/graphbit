# Installation Guide

This guide will help you install GraphBit on your system and set up your development environment.

## System Requirements

- **Python**: 3.10 or higher (< 3.13)
- **Operating System**: Linux, macOS, or Windows
- **Memory**: 4GB RAM minimum, 8GB recommended for high-throughput workloads
- **Storage**: 1GB free space

## Installation Methods

### Method 1: Install from PyPI (Recommended)

The easiest way to install GraphBit is using pip:

```bash
pip install graphbit
```

### Method 2: Install from Source

For development or the latest features:

```bash
# Clone the repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Set environment variable for compilation
unset ARGV0

# Build and install Python bindings
cd python
maturin develop
```

## Environment Setup

### 1. API Keys Configuration

GraphBit supports multiple LLM providers. Set up API keys for your preferred providers:

```bash
# OpenAI (required for most examples)
export OPENAI_API_KEY="sk-your-openai-api-key-here"

# Anthropic (optional)
export ANTHROPIC_API_KEY="sk-your-anthropic-api-key-here"

# HuggingFace (optional)
export HUGGINGFACE_API_KEY="hf-your-huggingface-token-here"
```

**⚠️ Security Note**: Never commit API keys to version control. Use environment variables or secure secret management.

### 2. Environment File (Recommended)

Create a `.env` file in your project root:

```bash
# Copy the example environment file
cp .env.example .env

# Edit with your API keys
nano .env
```

Example `.env` file:
```
OPENAI_API_KEY=sk-your-openai-api-key-here
ANTHROPIC_API_KEY=sk-your-anthropic-api-key-here
HUGGINGFACE_API_KEY=hf-your-huggingface-token-here
```

### 3. Local LLM Setup (Optional)

To use local models with Ollama:

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Start Ollama server
ollama serve

# Pull a model (in another terminal)
ollama pull llama3.2
ollama pull phi3
```

## Verify Installation

Test your installation with this simple script:

```python
import graphbit
import os

# Initialize GraphBit
graphbit.init()

# Test basic functionality
print(f"GraphBit version: {graphbit.version()}")

# Get system information
system_info = graphbit.get_system_info()
print(f"Python binding version: {system_info['python_binding_version']}")
print(f"Runtime initialized: {system_info['runtime_initialized']}")

# Perform health check
health = graphbit.health_check()
print(f"System healthy: {health['overall_healthy']}")

# Test LLM configuration (requires API key)
if os.getenv("OPENAI_API_KEY"):
    config = graphbit.LlmConfig.openai(
        os.getenv("OPENAI_API_KEY"), 
        "gpt-4o-mini"
    )
    print(f"LLM Provider: {config.provider()}")
    print(f"Model: {config.model()}")
    print("Installation successful!")
else:
    print("No OPENAI_API_KEY found - set up API keys to use LLM features")
```

Save this as `test_installation.py` and run:

```bash
python test_installation.py
```

Expected output:
```
GraphBit version: [version]
Python binding version: [version]
Runtime initialized: True
System healthy: True
LLM Provider: openai
Model: gpt-4o-mini
Installation successful!
```

## Development Installation

For contributors and advanced users:

```bash
# Clone and setup development environment
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit

# Install development dependencies
make dev-setup

# Install pre-commit hooks
make pre-commit-install

# Build Python bindings in development mode
cd python
maturin develop

# Run tests to verify setup
cd ..
make test
```

## Docker Installation (Alternative)

Run GraphBit in a containerized environment:

```bash
# Pull the official image
docker pull graphbit/graphbit:latest

# Run with environment variables
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
           -v $(pwd):/workspace \
           graphbit/graphbit:latest
```

## Troubleshooting

### Common Issues

#### 1. Import Error
```
ImportError: No module named 'graphbit'
```
**Solution**: Ensure you're using the correct Python environment and GraphBit is installed:
```bash
pip list | grep graphbit
pip install --upgrade graphbit
```

#### 2. Rust Compilation Errors
```
error: Microsoft Visual C++ 14.0 is required (Windows)
```
**Solution**: Install Microsoft C++ Build Tools or Visual Studio with C++ support.

For Linux/macOS compilation issues:
```bash
# Ensure ARGV0 is unset for proper compilation
unset ARGV0

# Make sure Rust is properly installed
rustc --version
```

#### 3. API Key Issues
```
Authentication failed
```
**Solution**: Verify your API keys are correctly set:
```bash
echo $OPENAI_API_KEY
```

#### 4. Runtime Initialization Errors
```
Failed to initialize GraphBit runtime
```
**Solution**: Check system health and reinitialize:
```python
import graphbit
graphbit.init(debug=True)  # Enable debug logging
health = graphbit.health_check()
print(health)
```

#### 5. Permission Errors (Linux/macOS)
```bash
# If you get permission errors, try:
pip install --user graphbit

# Or use virtual environment (recommended)
python -m venv graphbit-env
source graphbit-env/bin/activate  # Linux/macOS
# graphbit-env\Scripts\activate   # Windows
pip install graphbit
```

#### 6. Memory Issues
If you encounter memory-related errors, use memory-optimized executor:
```python
import graphbit
config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
executor = graphbit.Executor.new_memory_optimized(config)
```

### Performance Optimization

For optimal performance:

1. **Choose the right executor**:
   - `Executor.new_high_throughput()` for batch processing
   - `Executor.new_low_latency()` for real-time applications
   - `Executor.new_memory_optimized()` for resource-constrained environments

2. **Monitor system health**:
   ```python
   health = graphbit.health_check()
   if not health['overall_healthy']:
       print("System issues detected")
   ```

3. **Configure runtime for your workload**:
   ```python
   # Before calling graphbit.init()
   graphbit.configure_runtime(
       worker_threads=8,
       max_blocking_threads=16
   )
   graphbit.init()
   ```

### Get Help

If you encounter issues:

1. Check the [FAQ](../user-guide/faq.md)
2. Search [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)
3. Create a new issue with:
   - Your operating system and Python version
   - Complete error message
   - Steps to reproduce
   - Output of `graphbit.get_system_info()` and `graphbit.health_check()`

## Next Steps

Once installed, proceed to the [Quick Start Tutorial](quickstart.md) to build your first AI workflow!

## Update GraphBit

Keep GraphBit updated for the latest features and bug fixes:

```bash
# Update from PyPI
pip install --upgrade graphbit

# Update from source
cd graphbit
git pull origin main
cd python
maturin develop
``` 
