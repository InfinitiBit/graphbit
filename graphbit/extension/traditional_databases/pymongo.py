"""
PyMongo Extension for GraphBit

This module provides integration with MongoDB using PyMongo.
Supports document storage, querying, and vector operations with embeddings.
"""

import os
from typing import Any, Dict, List, Optional, Type, Union
import logging

from ..base import BaseGraphBitExtension, ExtensionMetadata, ExtensionCategory, DependencyChecker

logger = logging.getLogger(__name__)

# Dependencies for this extension
DEPENDENCIES = ["pymongo"]


def check_dependencies() -> bool:
    """Check if PyMongo dependencies are available."""
    return DependencyChecker.check_dependency("pymongo")


class PyMongoExtension(BaseGraphBitExtension):
    """PyMongo database extension for GraphBit."""

    def _get_metadata(self) -> ExtensionMetadata:
        """Return PyMongo extension metadata."""
        return ExtensionMetadata(
            name="pymongo",
            version="1.0.0",
            description="MongoDB integration for GraphBit using PyMongo",
            category=ExtensionCategory.TRADITIONAL_DATABASE,
            dependencies=DEPENDENCIES,
            homepage="https://www.mongodb.com/",
            documentation="https://pymongo.readthedocs.io/"
        )

    def _get_client_class(self) -> Type:
        """Return the PyMongo client class."""
        try:
            from pymongo import MongoClient
            return MongoClient
        except ImportError as e:
            raise ImportError(
                "PyMongo client not available. Install with: pip install graphbit[pymongo]"
            ) from e

    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate PyMongo configuration."""
        # PyMongo can work with default configuration (localhost:27017)
        return True


class PyMongoClient:
    """
    GraphBit wrapper for PyMongo client with enhanced functionality.

    This class provides a production-grade interface to MongoDB with
    proper error handling, logging, and GraphBit integration.
    """

    def __init__(self, uri: Optional[str] = None, host: Optional[str] = None,
                 port: Optional[int] = None, database: Optional[str] = None, **kwargs):
        """
        Initialize PyMongo client.

        Args:
            uri: MongoDB connection URI (defaults to MONGO_URI env var)
            host: MongoDB host (defaults to localhost)
            port: MongoDB port (defaults to 27017)
            database: Default database name
            **kwargs: Additional PyMongo client options
        """
        self.uri = uri or os.getenv("MONGO_URI")
        self.host = host or "localhost"
        self.port = port or 27017
        self.database_name = database
        self.kwargs = kwargs

        self._client = None
        self._database = None
        self._initialize_client()

    def _initialize_client(self) -> None:
        """Initialize the PyMongo client."""
        try:
            from pymongo import MongoClient

            if self.uri:
                self._client = MongoClient(self.uri, **self.kwargs)
                logger.info(f"PyMongo client initialized with URI")
            else:
                self._client = MongoClient(self.host, self.port, **self.kwargs)
                logger.info(f"PyMongo client initialized: {self.host}:{self.port}")

            # Test connection
            self._client.server_info()

            if self.database_name:
                self._database = self._client[self.database_name]

        except Exception as e:
            logger.error(f"Failed to initialize PyMongo client: {e}")
            raise

    @property
    def client(self):
        """Get the underlying PyMongo client."""
        return self._client

    @property
    def database(self):
        """Get the default database."""
        return self._database

    def get_database(self, name: str):
        """Get a database by name."""
        return self._client[name]

    def get_collection(self, collection_name: str, database_name: Optional[str] = None):
        """Get a collection by name."""
        if database_name:
            return self._client[database_name][collection_name]
        elif self._database:
            return self._database[collection_name]
        else:
            raise ValueError("No database specified")

    def list_databases(self) -> List[str]:
        """List all databases."""
        try:
            return [db['name'] for db in self._client.list_databases()]
        except Exception as e:
            logger.error(f"Failed to list databases: {e}")
            raise

    def list_collections(self, database_name: Optional[str] = None) -> List[str]:
        """List all collections in a database."""
        try:
            db = self.get_database(database_name) if database_name else self._database
            if not db:
                raise ValueError("No database specified")
            return db.list_collection_names()
        except Exception as e:
            logger.error(f"Failed to list collections: {e}")
            raise

    def insert_document(self, collection_name: str, document: Dict[str, Any],
                       database_name: Optional[str] = None) -> Any:
        """Insert a single document."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.insert_one(document)
            logger.info(f"Inserted document with ID: {result.inserted_id}")
            return result
        except Exception as e:
            logger.error(f"Failed to insert document: {e}")
            raise

    def insert_documents(self, collection_name: str, documents: List[Dict[str, Any]],
                        database_name: Optional[str] = None) -> Any:
        """Insert multiple documents."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.insert_many(documents)
            logger.info(f"Inserted {len(result.inserted_ids)} documents")
            return result
        except Exception as e:
            logger.error(f"Failed to insert documents: {e}")
            raise

    def find_documents(self, collection_name: str, filter_dict: Optional[Dict] = None,
                      database_name: Optional[str] = None, limit: Optional[int] = None) -> List[Dict]:
        """Find documents matching a filter."""
        try:
            collection = self.get_collection(collection_name, database_name)
            cursor = collection.find(filter_dict or {})
            if limit:
                cursor = cursor.limit(limit)
            documents = list(cursor)
            logger.info(f"Found {len(documents)} documents")
            return documents
        except Exception as e:
            logger.error(f"Failed to find documents: {e}")
            raise

    def find_one_document(self, collection_name: str, filter_dict: Optional[Dict] = None,
                         database_name: Optional[str] = None) -> Optional[Dict]:
        """Find a single document matching a filter."""
        try:
            collection = self.get_collection(collection_name, database_name)
            document = collection.find_one(filter_dict or {})
            if document:
                logger.info("Found document")
            else:
                logger.info("No document found")
            return document
        except Exception as e:
            logger.error(f"Failed to find document: {e}")
            raise

    def update_document(self, collection_name: str, filter_dict: Dict, update_dict: Dict,
                       database_name: Optional[str] = None, upsert: bool = False) -> Any:
        """Update a single document."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.update_one(filter_dict, update_dict, upsert=upsert)
            logger.info(f"Updated {result.modified_count} document(s)")
            return result
        except Exception as e:
            logger.error(f"Failed to update document: {e}")
            raise

    def update_documents(self, collection_name: str, filter_dict: Dict, update_dict: Dict,
                        database_name: Optional[str] = None) -> Any:
        """Update multiple documents."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.update_many(filter_dict, update_dict)
            logger.info(f"Updated {result.modified_count} document(s)")
            return result
        except Exception as e:
            logger.error(f"Failed to update documents: {e}")
            raise

    def delete_document(self, collection_name: str, filter_dict: Dict,
                       database_name: Optional[str] = None) -> Any:
        """Delete a single document."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.delete_one(filter_dict)
            logger.info(f"Deleted {result.deleted_count} document(s)")
            return result
        except Exception as e:
            logger.error(f"Failed to delete document: {e}")
            raise

    def delete_documents(self, collection_name: str, filter_dict: Dict,
                        database_name: Optional[str] = None) -> Any:
        """Delete multiple documents."""
        try:
            collection = self.get_collection(collection_name, database_name)
            result = collection.delete_many(filter_dict)
            logger.info(f"Deleted {result.deleted_count} document(s)")
            return result
        except Exception as e:
            logger.error(f"Failed to delete documents: {e}")
            raise

    def create_index(self, collection_name: str, index_spec: Union[str, List],
                    database_name: Optional[str] = None, **kwargs) -> str:
        """Create an index on a collection."""
        try:
            collection = self.get_collection(collection_name, database_name)
            index_name = collection.create_index(index_spec, **kwargs)
            logger.info(f"Created index: {index_name}")
            return index_name
        except Exception as e:
            logger.error(f"Failed to create index: {e}")
            raise

    def aggregate(self, collection_name: str, pipeline: List[Dict],
                 database_name: Optional[str] = None) -> List[Dict]:
        """Execute an aggregation pipeline."""
        try:
            collection = self.get_collection(collection_name, database_name)
            results = list(collection.aggregate(pipeline))
            logger.info(f"Aggregation returned {len(results)} results")
            return results
        except Exception as e:
            logger.error(f"Failed to execute aggregation: {e}")
            raise


# Create extension instance
extension = PyMongoExtension()

# Export main classes and functions
__all__ = [
    "PyMongoExtension",
    "PyMongoClient",
    "check_dependencies",
    "DEPENDENCIES",
    "extension"
]