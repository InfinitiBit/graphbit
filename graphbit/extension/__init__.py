"""
GraphBit Extension System

This module provides a comprehensive extension system for GraphBit connectors.
Extensions are organized by category and support lazy loading with optional dependencies.

Usage:
    from graphbit.extension import pymongo
    from graphbit.extension import pinecone
    from graphbit.extension import aws_boto3

Installation:
    pip install graphbit[pymongo]
    pip install graphbit[pinecone]
    pip install graphbit[aws_boto3]
    pip install graphbit[all]  # Install all extensions
"""

import importlib
import sys
from typing import Any, Dict, List, Optional, Type
from abc import ABC, abstractmethod
import warnings


class ExtensionError(Exception):
    """Base exception for extension-related errors."""
    pass


class DependencyNotFoundError(ExtensionError):
    """Raised when a required dependency is not installed."""
    pass


class ExtensionNotFoundError(ExtensionError):
    """Raised when an extension is not found."""
    pass


class BaseExtension(ABC):
    """
    Abstract base class for all GraphBit extensions.

    This class defines the interface that all extensions must implement,
    following the Interface Segregation Principle.
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Return the extension name."""
        pass

    @property
    @abstractmethod
    def dependencies(self) -> List[str]:
        """Return list of required dependencies."""
        pass

    @property
    @abstractmethod
    def category(self) -> str:
        """Return the extension category."""
        pass

    @abstractmethod
    def check_dependencies(self) -> bool:
        """Check if all required dependencies are available."""
        pass

    @abstractmethod
    def get_client_class(self) -> Type:
        """Return the main client class for this extension."""
        pass


class ExtensionRegistry:
    """
    Registry for managing GraphBit extensions.

    This class implements the Singleton pattern and provides centralized
    management of all available extensions.
    """

    _instance: Optional['ExtensionRegistry'] = None
    _extensions: Dict[str, BaseExtension] = {}

    def __new__(cls) -> 'ExtensionRegistry':
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def register(self, extension: BaseExtension) -> None:
        """Register an extension."""
        self._extensions[extension.name] = extension

    def get(self, name: str) -> Optional[BaseExtension]:
        """Get an extension by name."""
        return self._extensions.get(name)

    def list_by_category(self, category: str) -> List[BaseExtension]:
        """List all extensions in a category."""
        return [ext for ext in self._extensions.values() if ext.category == category]

    def list_all(self) -> List[BaseExtension]:
        """List all registered extensions."""
        return list(self._extensions.values())


class LazyExtensionLoader:
    """
    Lazy loader for GraphBit extensions.

    This class implements lazy loading to avoid importing dependencies
    until they are actually needed, improving startup performance.
    """

    def __init__(self, extension_name: str, module_path: str):
        self.extension_name = extension_name
        self.module_path = module_path
        self._loaded_module = None

    def __getattr__(self, name: str) -> Any:
        """Lazy load the extension module when accessed."""
        if self._loaded_module is None:
            self._load_module()
        return getattr(self._loaded_module, name)

    def _load_module(self) -> None:
        """Load the extension module and check dependencies."""
        try:
            # Import the extension module
            self._loaded_module = importlib.import_module(self.module_path)

            # Check if the extension has dependency requirements
            if hasattr(self._loaded_module, 'check_dependencies'):
                if not self._loaded_module.check_dependencies():
                    dependencies = getattr(self._loaded_module, 'DEPENDENCIES', [])
                    raise DependencyNotFoundError(
                        f"Extension '{self.extension_name}' requires dependencies: {dependencies}. "
                        f"Install with: pip install graphbit[{self.extension_name}]"
                    )
        except ImportError as e:
            raise ExtensionNotFoundError(
                f"Extension '{self.extension_name}' not found. "
                f"Install with: pip install graphbit[{self.extension_name}]"
            ) from e


# Extension categories
VECTOR_DATABASES = [
    'pinecone', 'qdrant', 'weaviate', 'chromadb',
    'milvus', 'faiss', 'astradb'
]

TRADITIONAL_DATABASES = [
    'pymongo', 'pgvector', 'mariadb', 'db2', 'elasticsearch'
]

CLOUD_PROVIDERS = [
    'aws_boto3', 'azure', 'google_cloud_platform'
]

SEARCH_ENGINES = [
    'google_search_api'
]

ALL_EXTENSIONS = (
    VECTOR_DATABASES +
    TRADITIONAL_DATABASES +
    CLOUD_PROVIDERS +
    SEARCH_ENGINES
)


def _create_lazy_loader(extension_name: str) -> LazyExtensionLoader:
    """Create a lazy loader for an extension."""
    # Determine the category and module path
    if extension_name in VECTOR_DATABASES:
        module_path = f"graphbit.extension.vector_databases.{extension_name}"
    elif extension_name in TRADITIONAL_DATABASES:
        module_path = f"graphbit.extension.traditional_databases.{extension_name}"
    elif extension_name in CLOUD_PROVIDERS:
        module_path = f"graphbit.extension.cloud_providers.{extension_name}"
    elif extension_name in SEARCH_ENGINES:
        module_path = f"graphbit.extension.search_engines.{extension_name}"
    else:
        raise ExtensionNotFoundError(f"Unknown extension: {extension_name}")

    return LazyExtensionLoader(extension_name, module_path)


def __getattr__(name: str) -> Any:
    """
    Dynamic attribute access for extensions.

    This allows imports like: from graphbit.extension import pymongo
    """
    if name in ALL_EXTENSIONS:
        return _create_lazy_loader(name)

    raise AttributeError(f"Extension '{name}' not found. Available extensions: {ALL_EXTENSIONS}")


def list_available_extensions() -> Dict[str, List[str]]:
    """List all available extensions by category."""
    return {
        'vector_databases': VECTOR_DATABASES,
        'traditional_databases': TRADITIONAL_DATABASES,
        'cloud_providers': CLOUD_PROVIDERS,
        'search_engines': SEARCH_ENGINES
    }


def check_extension_dependencies(extension_name: str) -> bool:
    """Check if an extension's dependencies are installed."""
    try:
        loader = _create_lazy_loader(extension_name)
        # This will trigger dependency checking
        _ = loader.__dict__
        return True
    except (DependencyNotFoundError, ExtensionNotFoundError):
        return False


# Initialize the registry
registry = ExtensionRegistry()

__all__ = [
    'BaseExtension',
    'ExtensionRegistry',
    'LazyExtensionLoader',
    'ExtensionError',
    'DependencyNotFoundError',
    'ExtensionNotFoundError',
    'list_available_extensions',
    'check_extension_dependencies',
    'registry'
] + ALL_EXTENSIONS