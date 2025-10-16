# Installation Guide

This guide will help you install GraphBit on your system and set up your development environment.

## System Requirements

- **Python**: 3.10 or higher (<= 3.13)
- **Operating System**: Linux, macOS, or Windows
- **Memory**: 4GB RAM minimum, 8GB recommended for high-throughput workloads
- **Storage**: 1GB free space

---

## Installation 

```bash
pip install graphbit
```

---

## Verify Installation

Test your installation with this simple script:

```python
import os

from graphbit import version, get_system_info, health_check, LlmConfig

# Test basic functionality
print(f"GraphBit version: {version()}")

# Get system information
system_info = get_system_info()
print(f"Python binding version: {system_info['python_binding_version']}")
print(f"Runtime initialized: {system_info['runtime_initialized']}")

# Perform health check
health = health_check()
print(f"System healthy: {health['overall_healthy']}")

# Test LLM configuration (requires API key)
if os.getenv("OPENAI_API_KEY"):
    config = LlmConfig.openai(
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

---

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
```

## Troubleshooting

### Common Issues

#### 1. Runtime Initialization Errors
```
Failed to initialize GraphBit runtime
```
**Solution**: Check system health and reinitialize:
```python
from graphbit import init, health_check

init(debug=True)  # Enable debug logging
health = health_check()
print(health)
```

#### 2. Environment Setup (Linux/macOS)
```bash
# Use virtual environment (recommended)
python -m venv graphbit-env
source graphbit-env/bin/activate  # Linux/macOS
# graphbit-env\Scripts\activate   # Windows
pip install graphbit
```

### Get Help

If you encounter issues:

1. Check the [FAQ](https://github.com/InfinitiBit/graphbit/discussions)
2. Search [GitHub Issues](https://github.com/InfinitiBit/graphbit/issues)
3. Create a new issue with:
   - Your operating system and Python version
   - Complete error message
   - Steps to reproduce
   - Output of `get_system_info()` and `health_check()`

---

## Update GraphBit

Keep GraphBit updated for the latest features and bug fixes:

```bash
pip install --upgrade graphbit
``` 

---

## Next Steps

Once installed, proceed to the [Quick Start Tutorial](quickstart.md) to build your first AI workflow!

---
