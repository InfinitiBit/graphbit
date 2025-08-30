"""
Base extension framework for GraphBit connectors.

This module provides the foundational classes and utilities for building
GraphBit extensions following SOLID principles and production-grade patterns.
"""

import importlib
import logging
from abc import ABC, abstractmethod
from typing import Any, Dict, List, Optional, Type, Union
from dataclasses import dataclass
from enum import Enum


logger = logging.getLogger(__name__)


class ExtensionCategory(Enum):
    """Categories for GraphBit extensions."""
    VECTOR_DATABASE = "vector_database"
    TRADITIONAL_DATABASE = "traditional_database"
    CLOUD_PROVIDER = "cloud_provider"
    SEARCH_ENGINE = "search_engine"


@dataclass
class ExtensionMetadata:
    """Metadata for an extension."""
    name: str
    version: str
    description: str
    category: ExtensionCategory
    dependencies: List[str]
    author: str = "GraphBit Team"
    homepage: Optional[str] = None
    documentation: Optional[str] = None


class DependencyChecker:
    """Utility class for checking extension dependencies."""

    @staticmethod
    def check_dependency(package_name: str) -> bool:
        """Check if a package is installed and importable."""
        try:
            importlib.import_module(package_name)
            return True
        except ImportError:
            return False

    @staticmethod
    def check_dependencies(dependencies: List[str]) -> Dict[str, bool]:
        """Check multiple dependencies and return status for each."""
        return {dep: DependencyChecker.check_dependency(dep) for dep in dependencies}

    @staticmethod
    def get_missing_dependencies(dependencies: List[str]) -> List[str]:
        """Get list of missing dependencies."""
        return [
            dep for dep, available in DependencyChecker.check_dependencies(dependencies).items()
            if not available
        ]


class BaseGraphBitExtension(ABC):
    """
    Abstract base class for all GraphBit extensions.

    This class defines the contract that all extensions must implement,
    following the Interface Segregation Principle and providing a clean
    abstraction for different types of connectors.
    """

    def __init__(self):
        self._metadata = self._get_metadata()
        self._client_class = None
        self._is_initialized = False

    @abstractmethod
    def _get_metadata(self) -> ExtensionMetadata:
        """Return extension metadata."""
        pass

    @abstractmethod
    def _get_client_class(self) -> Type:
        """Return the main client class for this extension."""
        pass

    @abstractmethod
    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate extension-specific configuration."""
        pass

    @property
    def metadata(self) -> ExtensionMetadata:
        """Get extension metadata."""
        return self._metadata

    @property
    def name(self) -> str:
        """Get extension name."""
        return self._metadata.name

    @property
    def category(self) -> ExtensionCategory:
        """Get extension category."""
        return self._metadata.category

    @property
    def dependencies(self) -> List[str]:
        """Get extension dependencies."""
        return self._metadata.dependencies

    def check_dependencies(self) -> bool:
        """Check if all required dependencies are available."""
        missing = DependencyChecker.get_missing_dependencies(self.dependencies)
        if missing:
            logger.warning(f"Extension '{self.name}' missing dependencies: {missing}")
            return False
        return True

    def get_dependency_status(self) -> Dict[str, bool]:
        """Get detailed dependency status."""
        return DependencyChecker.check_dependencies(self.dependencies)

    def get_client_class(self) -> Type:
        """Get the main client class, checking dependencies first."""
        if not self.check_dependencies():
            missing = DependencyChecker.get_missing_dependencies(self.dependencies)
            raise ImportError(
                f"Cannot load extension '{self.name}'. Missing dependencies: {missing}. "
                f"Install with: pip install graphbit[{self.name}]"
            )

        if self._client_class is None:
            self._client_class = self._get_client_class()

        return self._client_class

    def create_client(self, config: Optional[Dict[str, Any]] = None) -> Any:
        """Create a client instance with optional configuration."""
        if config and not self._validate_configuration(config):
            raise ValueError(f"Invalid configuration for extension '{self.name}'")

        client_class = self.get_client_class()

        if config:
            return client_class(**config)
        else:
            return client_class()

    def initialize(self) -> None:
        """Initialize the extension (called once)."""
        if self._is_initialized:
            return

        logger.info(f"Initializing extension: {self.name}")
        self._is_initialized = True

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__}(name='{self.name}', category='{self.category.value}')>"