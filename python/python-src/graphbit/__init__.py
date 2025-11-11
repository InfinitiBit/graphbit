"""
GraphBit - Advanced workflow automation and AI agent orchestration library.

This package provides both Rust-based high-performance components and pure Python
extensions for maximum flexibility.
"""

# Try to import all Rust-based components from the compiled module
try:
    from graphbit.graphbit import *  # noqa: F401, F403
except ImportError:
    # If graphbit.graphbit doesn't exist (e.g., in development/testing),
    # try importing from the installed graphbit module
    try:
        import graphbit as _graphbit_module
        # Re-export everything from the main graphbit module
        for attr in dir(_graphbit_module):
            if not attr.startswith('_'):
                globals()[attr] = getattr(_graphbit_module, attr)
    except (ImportError, AttributeError):
        # Neither import worked, we're probably in a test environment
        # Just continue without the Rust components
        pass

# Make vllm submodule available
from . import vllm  # noqa: F401

__all__ = ['vllm']

