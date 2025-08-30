"""
ChromaDB Extension for GraphBit

This module provides integration with ChromaDB vector database.
Supports vector storage, similarity search, and metadata filtering.
"""

import os
from typing import Any, Dict, List, Optional, Type, Union
import logging

from ..base import BaseGraphBitExtension, ExtensionMetadata, ExtensionCategory, DependencyChecker

logger = logging.getLogger(__name__)

# Dependencies for this extension
DEPENDENCIES = ["chromadb"]


def check_dependencies() -> bool:
    """Check if ChromaDB dependencies are available."""
    return DependencyChecker.check_dependency("chromadb")


class ChromaDBExtension(BaseGraphBitExtension):
    """ChromaDB vector database extension for GraphBit."""

    def _get_metadata(self) -> ExtensionMetadata:
        """Return ChromaDB extension metadata."""
        return ExtensionMetadata(
            name="chromadb",
            version="1.0.0",
            description="ChromaDB vector database integration for GraphBit",
            category=ExtensionCategory.VECTOR_DATABASE,
            dependencies=DEPENDENCIES,
            homepage="https://www.trychroma.com/",
            documentation="https://docs.trychroma.com/"
        )

    def _get_client_class(self) -> Type:
        """Return the ChromaDB client class."""
        try:
            from chromadb import Client
            return Client
        except ImportError as e:
            raise ImportError(
                "ChromaDB client not available. Install with: pip install graphbit[chromadb]"
            ) from e

    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate ChromaDB configuration."""
        # ChromaDB can work with default configuration
        return True


class ChromaDBClient:
    """
    GraphBit wrapper for ChromaDB client with enhanced functionality.

    This class provides a production-grade interface to ChromaDB with
    proper error handling, logging, and GraphBit integration.
    """

    def __init__(self, host: Optional[str] = None, port: Optional[int] = None,
                 path: Optional[str] = None, **kwargs):
        """
        Initialize ChromaDB client.

        Args:
            host: ChromaDB server host (for client mode)
            port: ChromaDB server port (for client mode)
            path: Path for persistent storage (for embedded mode)
            **kwargs: Additional configuration options
        """
        self.host = host
        self.port = port
        self.path = path
        self.kwargs = kwargs

        self._client = None
        self._initialize_client()

    def _initialize_client(self) -> None:
        """Initialize the ChromaDB client."""
        try:
            import chromadb

            if self.host and self.port:
                # Client mode - connect to ChromaDB server
                self._client = chromadb.HttpClient(
                    host=self.host,
                    port=self.port,
                    **self.kwargs
                )
                logger.info(f"ChromaDB client initialized in client mode: {self.host}:{self.port}")
            elif self.path:
                # Persistent mode - use local storage
                self._client = chromadb.PersistentClient(path=self.path, **self.kwargs)
                logger.info(f"ChromaDB client initialized in persistent mode: {self.path}")
            else:
                # In-memory mode
                self._client = chromadb.Client(**self.kwargs)
                logger.info("ChromaDB client initialized in memory mode")

        except Exception as e:
            logger.error(f"Failed to initialize ChromaDB client: {e}")
            raise

    @property
    def client(self):
        """Get the underlying ChromaDB client."""
        return self._client

    def create_collection(self, name: str, metadata: Optional[Dict] = None,
                         embedding_function: Optional[Any] = None) -> Any:
        """
        Create a new ChromaDB collection.

        Args:
            name: Collection name
            metadata: Optional metadata for the collection
            embedding_function: Optional embedding function

        Returns:
            ChromaDB collection object
        """
        try:
            collection = self._client.create_collection(
                name=name,
                metadata=metadata,
                embedding_function=embedding_function
            )
            logger.info(f"Created ChromaDB collection: {name}")
            return collection
        except Exception as e:
            logger.error(f"Failed to create ChromaDB collection {name}: {e}")
            raise

    def get_collection(self, name: str, embedding_function: Optional[Any] = None) -> Any:
        """Get a ChromaDB collection by name."""
        try:
            collection = self._client.get_collection(
                name=name,
                embedding_function=embedding_function
            )
            return collection
        except Exception as e:
            logger.error(f"Failed to get ChromaDB collection {name}: {e}")
            raise

    def get_or_create_collection(self, name: str, metadata: Optional[Dict] = None,
                                embedding_function: Optional[Any] = None) -> Any:
        """Get or create a ChromaDB collection."""
        try:
            collection = self._client.get_or_create_collection(
                name=name,
                metadata=metadata,
                embedding_function=embedding_function
            )
            return collection
        except Exception as e:
            logger.error(f"Failed to get or create ChromaDB collection {name}: {e}")
            raise

    def list_collections(self) -> List[str]:
        """List all available collections."""
        try:
            collections = self._client.list_collections()
            return [col.name for col in collections]
        except Exception as e:
            logger.error(f"Failed to list ChromaDB collections: {e}")
            raise

    def delete_collection(self, name: str) -> None:
        """Delete a ChromaDB collection."""
        try:
            self._client.delete_collection(name)
            logger.info(f"Deleted ChromaDB collection: {name}")
        except Exception as e:
            logger.error(f"Failed to delete ChromaDB collection {name}: {e}")
            raise


# Create extension instance
extension = ChromaDBExtension()

# Export main classes and functions
__all__ = [
    "ChromaDBExtension",
    "ChromaDBClient",
    "check_dependencies",
    "DEPENDENCIES",
    "extension"
]