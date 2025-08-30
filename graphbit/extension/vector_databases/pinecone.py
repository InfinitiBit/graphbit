"""
Pinecone Extension for GraphBit

This module provides integration with Pinecone vector database.
Supports vector storage, similarity search, and metadata filtering.
"""

import os
from typing import Any, Dict, List, Optional, Type, Union
import logging

from ..base import BaseGraphBitExtension, ExtensionMetadata, ExtensionCategory, DependencyChecker

logger = logging.getLogger(__name__)

# Dependencies for this extension
DEPENDENCIES = ["pinecone"]


def check_dependencies() -> bool:
    """Check if Pinecone dependencies are available."""
    return DependencyChecker.check_dependency("pinecone")


class PineconeExtension(BaseGraphBitExtension):
    """Pinecone vector database extension for GraphBit."""

    def _get_metadata(self) -> ExtensionMetadata:
        """Return Pinecone extension metadata."""
        return ExtensionMetadata(
            name="pinecone",
            version="1.0.0",
            description="Pinecone vector database integration for GraphBit",
            category=ExtensionCategory.VECTOR_DATABASE,
            dependencies=DEPENDENCIES,
            homepage="https://www.pinecone.io/",
            documentation="https://docs.pinecone.io/"
        )

    def _get_client_class(self) -> Type:
        """Return the Pinecone client class."""
        try:
            from pinecone import Pinecone
            return Pinecone
        except ImportError as e:
            raise ImportError(
                "Pinecone client not available. Install with: pip install graphbit[pinecone]"
            ) from e

    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate Pinecone configuration."""
        required_fields = ["api_key"]
        return all(field in config for field in required_fields)


class PineconeClient:
    """
    GraphBit wrapper for Pinecone client with enhanced functionality.

    This class provides a production-grade interface to Pinecone with
    proper error handling, logging, and GraphBit integration.
    """

    def __init__(self, api_key: Optional[str] = None, environment: Optional[str] = None):
        """
        Initialize Pinecone client.

        Args:
            api_key: Pinecone API key (defaults to PINECONE_API_KEY env var)
            environment: Pinecone environment (defaults to PINECONE_ENVIRONMENT env var)
        """
        self.api_key = api_key or os.getenv("PINECONE_API_KEY")
        self.environment = environment or os.getenv("PINECONE_ENVIRONMENT")

        if not self.api_key:
            raise ValueError("Pinecone API key is required")

        self._client = None
        self._initialize_client()

    def _initialize_client(self) -> None:
        """Initialize the Pinecone client."""
        try:
            from pinecone import Pinecone
            self._client = Pinecone(api_key=self.api_key)
            logger.info("Pinecone client initialized successfully")
        except Exception as e:
            logger.error(f"Failed to initialize Pinecone client: {e}")
            raise

    @property
    def client(self):
        """Get the underlying Pinecone client."""
        return self._client

    def create_index(self, name: str, dimension: int, metric: str = "cosine", **kwargs) -> None:
        """
        Create a new Pinecone index.

        Args:
            name: Index name
            dimension: Vector dimension
            metric: Distance metric (cosine, euclidean, dotproduct)
            **kwargs: Additional index configuration
        """
        try:
            from pinecone import ServerlessSpec

            spec = kwargs.get('spec', ServerlessSpec(
                cloud=kwargs.get('cloud', 'aws'),
                region=kwargs.get('region', 'us-east-1')
            ))

            self._client.create_index(
                name=name,
                dimension=dimension,
                metric=metric,
                spec=spec
            )
            logger.info(f"Created Pinecone index: {name}")
        except Exception as e:
            logger.error(f"Failed to create Pinecone index {name}: {e}")
            raise

    def get_index(self, name: str):
        """Get a Pinecone index by name."""
        try:
            return self._client.Index(name)
        except Exception as e:
            logger.error(f"Failed to get Pinecone index {name}: {e}")
            raise

    def list_indexes(self) -> List[str]:
        """List all available indexes."""
        try:
            indexes = self._client.list_indexes()
            return [idx.name for idx in indexes]
        except Exception as e:
            logger.error(f"Failed to list Pinecone indexes: {e}")
            raise

    def delete_index(self, name: str) -> None:
        """Delete a Pinecone index."""
        try:
            self._client.delete_index(name)
            logger.info(f"Deleted Pinecone index: {name}")
        except Exception as e:
            logger.error(f"Failed to delete Pinecone index {name}: {e}")
            raise

    def upsert_vectors(self, index_name: str, vectors: List[tuple], namespace: str = "") -> Dict:
        """
        Upsert vectors to an index.

        Args:
            index_name: Name of the index
            vectors: List of (id, values, metadata) tuples
            namespace: Optional namespace

        Returns:
            Upsert response
        """
        try:
            index = self.get_index(index_name)
            response = index.upsert(vectors=vectors, namespace=namespace)
            logger.info(f"Upserted {len(vectors)} vectors to index {index_name}")
            return response
        except Exception as e:
            logger.error(f"Failed to upsert vectors to index {index_name}: {e}")
            raise

    def query_vectors(self, index_name: str, vector: List[float], top_k: int = 10,
                     include_metadata: bool = True, namespace: str = "",
                     filter_dict: Optional[Dict] = None) -> Dict:
        """
        Query vectors from an index.

        Args:
            index_name: Name of the index
            vector: Query vector
            top_k: Number of results to return
            include_metadata: Whether to include metadata
            namespace: Optional namespace
            filter_dict: Optional metadata filter

        Returns:
            Query response
        """
        try:
            index = self.get_index(index_name)
            response = index.query(
                vector=vector,
                top_k=top_k,
                include_metadata=include_metadata,
                namespace=namespace,
                filter=filter_dict
            )
            logger.info(f"Queried index {index_name} with top_k={top_k}")
            return response
        except Exception as e:
            logger.error(f"Failed to query index {index_name}: {e}")
            raise


# Create extension instance
extension = PineconeExtension()

# Export main classes and functions
__all__ = [
    "PineconeExtension",
    "PineconeClient",
    "check_dependencies",
    "DEPENDENCIES",
    "extension"
]