"""
GraphBit Python bindings and tools package.

This package provides Python bindings for GraphBit and additional
tools for building AI workflows with tool calling capabilities.
"""

__version__ = "0.1.0"
__author__ = "GraphBit Team"

# Import main GraphBit functionality (this will be available when the Rust bindings are built)
try:
    from ..graphbit import *  # Import from the compiled Rust module
except ImportError:
    # Fallback for development/testing when Rust bindings might not be available
    pass
