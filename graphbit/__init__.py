"""
GraphBit - Advanced workflow automation and AI agent orchestration library

This package provides a comprehensive framework for building AI-powered workflows
with support for multiple LLM providers, vector databases, and cloud services.

The extension system allows for modular integration with third-party services:
- Vector databases (Pinecone, ChromaDB, Qdrant, etc.)
- Traditional databases (MongoDB, PostgreSQL, etc.)
- Cloud providers (AWS, Azure, GCP)
- Search engines (Google Search API)

Usage:
    import graphbit

    # Core functionality
    graphbit.init()

    # Extensions (install separately)
    from graphbit.extension import pymongo
    from graphbit.extension import pinecone
    from graphbit.extension import aws_boto3
"""

import sys
import warnings
from typing import Any, Dict, List, Optional

# Version information
__version__ = "0.1.0"
__author__ = "GraphBit Team"
__description__ = "Advanced workflow automation and AI agent orchestration library"

# Try to import core GraphBit functionality from Rust bindings
try:
    # Import from the compiled Rust module
    from graphbit import (
        init,
        version,
        LlmConfig,
        LlmClient,
        EmbeddingConfig,
        EmbeddingClient,
        Workflow,
        Node,
        Executor,
        WorkflowContext,
        WorkflowResult,
        TextSplitterConfig,
        TextChunk,
        CharacterSplitter,
        TokenSplitter,
        SentenceSplitter,
        RecursiveSplitter,
        PyDocumentLoader,
        PyDocumentLoaderConfig,
        PyDocumentContent
    )

    _RUST_BINDINGS_AVAILABLE = True

except ImportError as e:
    # Fallback if Rust bindings are not available
    warnings.warn(
        "GraphBit Rust bindings not available. Some functionality may be limited. "
        "Please ensure the package is properly installed.",
        ImportWarning
    )
    _RUST_BINDINGS_AVAILABLE = False

    # Define minimal fallback functions
    def init(*args, **kwargs):
        """Fallback init function."""
        pass

    def version():
        """Fallback version function."""
        return __version__


# Extension system imports
try:
    from . import extension
    _EXTENSION_SYSTEM_AVAILABLE = True
except ImportError as e:
    warnings.warn(
        f"GraphBit extension system not available: {e}",
        ImportWarning
    )
    _EXTENSION_SYSTEM_AVAILABLE = False


def get_available_extensions() -> Dict[str, List[str]]:
    """
    Get a list of all available extensions by category.

    Returns:
        Dictionary mapping categories to lists of extension names
    """
    if _EXTENSION_SYSTEM_AVAILABLE:
        try:
            return extension.list_available_extensions()
        except Exception as e:
            warnings.warn(f"Failed to list extensions: {e}")
            return {}
    else:
        return {}


def check_extension_dependencies(extension_name: str) -> bool:
    """
    Check if an extension's dependencies are installed.

    Args:
        extension_name: Name of the extension to check

    Returns:
        True if dependencies are available, False otherwise
    """
    if _EXTENSION_SYSTEM_AVAILABLE:
        try:
            return extension.check_extension_dependencies(extension_name)
        except Exception as e:
            warnings.warn(f"Failed to check extension dependencies: {e}")
            return False
    else:
        return False


def get_system_info() -> Dict[str, Any]:
    """
    Get information about the GraphBit installation and available features.

    Returns:
        Dictionary with system information
    """
    info = {
        "version": __version__,
        "rust_bindings_available": _RUST_BINDINGS_AVAILABLE,
        "extension_system_available": _EXTENSION_SYSTEM_AVAILABLE,
        "available_extensions": get_available_extensions(),
        "python_version": f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
    }

    if _RUST_BINDINGS_AVAILABLE:
        try:
            info["rust_version"] = version()
        except:
            pass

    return info


# Define what gets exported when using "from graphbit import *"
__all__ = [
    # Version and metadata
    "__version__",
    "__author__",
    "__description__",

    # System functions
    "init",
    "version",
    "get_available_extensions",
    "check_extension_dependencies",
    "get_system_info",
]

# Add Rust binding exports if available
if _RUST_BINDINGS_AVAILABLE:
    __all__.extend([
        "LlmConfig",
        "LlmClient",
        "EmbeddingConfig",
        "EmbeddingClient",
        "Workflow",
        "Node",
        "Executor",
        "WorkflowContext",
        "WorkflowResult",
        "TextSplitterConfig",
        "TextChunk",
        "CharacterSplitter",
        "TokenSplitter",
        "SentenceSplitter",
        "RecursiveSplitter",
        "PyDocumentLoader",
        "PyDocumentLoaderConfig",
        "PyDocumentContent"
    ])

# Add extension system if available
if _EXTENSION_SYSTEM_AVAILABLE:
    __all__.append("extension")