"""
Google Search API Extension for GraphBit

This module provides integration with Google Custom Search API.
Supports web search, image search, and custom search engines.
"""

import os
from typing import Any, Dict, List, Optional, Type, Union
import logging

from ..base import BaseGraphBitExtension, ExtensionMetadata, ExtensionCategory, DependencyChecker

logger = logging.getLogger(__name__)

# Dependencies for this extension
DEPENDENCIES = ["googleapiclient"]


def check_dependencies() -> bool:
    """Check if Google API client dependencies are available."""
    return DependencyChecker.check_dependency("googleapiclient")


class GoogleSearchAPIExtension(BaseGraphBitExtension):
    """Google Search API extension for GraphBit."""

    def _get_metadata(self) -> ExtensionMetadata:
        """Return Google Search API extension metadata."""
        return ExtensionMetadata(
            name="google_search_api",
            version="1.0.0",
            description="Google Custom Search API integration for GraphBit",
            category=ExtensionCategory.SEARCH_ENGINE,
            dependencies=DEPENDENCIES,
            homepage="https://developers.google.com/custom-search",
            documentation="https://developers.google.com/custom-search/v1/overview"
        )

    def _get_client_class(self) -> Type:
        """Return the Google API client build function."""
        try:
            from googleapiclient.discovery import build
            return build
        except ImportError as e:
            raise ImportError(
                "Google API client not available. Install with: pip install graphbit[google_search_api]"
            ) from e

    def _validate_configuration(self, config: Dict[str, Any]) -> bool:
        """Validate Google Search API configuration."""
        required_fields = ["api_key", "search_engine_id"]
        return all(field in config for field in required_fields)


class GoogleSearchAPIClient:
    """
    GraphBit wrapper for Google Custom Search API with enhanced functionality.

    This class provides a production-grade interface to Google Search with
    proper error handling, logging, and GraphBit integration.
    """

    def __init__(self, api_key: Optional[str] = None,
                 search_engine_id: Optional[str] = None, **kwargs):
        """
        Initialize Google Search API client.

        Args:
            api_key: Google API key (defaults to GOOGLE_API_KEY env var)
            search_engine_id: Custom Search Engine ID (defaults to GOOGLE_SEARCH_ENGINE_ID env var)
            **kwargs: Additional configuration options
        """
        self.api_key = api_key or os.getenv("GOOGLE_API_KEY")
        self.search_engine_id = search_engine_id or os.getenv("GOOGLE_SEARCH_ENGINE_ID")
        self.kwargs = kwargs

        if not self.api_key:
            raise ValueError("Google API key is required")
        if not self.search_engine_id:
            raise ValueError("Google Search Engine ID is required")

        self._service = None
        self._initialize_service()

    def _initialize_service(self) -> None:
        """Initialize the Google Custom Search service."""
        try:
            from googleapiclient.discovery import build

            self._service = build(
                "customsearch",
                "v1",
                developerKey=self.api_key,
                **self.kwargs
            )
            logger.info("Google Search API service initialized successfully")

        except Exception as e:
            logger.error(f"Failed to initialize Google Search API service: {e}")
            raise

    @property
    def service(self):
        """Get the underlying Google API service."""
        return self._service

    def search(self, query: str, num_results: int = 10, start_index: int = 1,
               search_type: Optional[str] = None, file_type: Optional[str] = None,
               site_search: Optional[str] = None, **kwargs) -> Dict[str, Any]:
        """
        Perform a search using Google Custom Search API.

        Args:
            query: Search query
            num_results: Number of results to return (max 10 per request)
            start_index: Starting index for results (1-based)
            search_type: Type of search ('image' for image search)
            file_type: File type filter (e.g., 'pdf', 'doc')
            site_search: Restrict search to specific site
            **kwargs: Additional search parameters

        Returns:
            Search results dictionary
        """
        try:
            search_params = {
                "q": query,
                "cx": self.search_engine_id,
                "num": min(num_results, 10),  # API limit is 10 per request
                "start": start_index,
                **kwargs
            }

            if search_type:
                search_params["searchType"] = search_type
            if file_type:
                search_params["fileType"] = file_type
            if site_search:
                search_params["siteSearch"] = site_search

            result = self._service.cse().list(**search_params).execute()

            logger.info(f"Search completed for query: '{query}' - {len(result.get('items', []))} results")
            return result

        except Exception as e:
            logger.error(f"Failed to perform search: {e}")
            raise

    def search_multiple_pages(self, query: str, total_results: int = 20,
                             **kwargs) -> List[Dict[str, Any]]:
        """
        Perform a multi-page search to get more than 10 results.

        Args:
            query: Search query
            total_results: Total number of results to retrieve
            **kwargs: Additional search parameters

        Returns:
            List of all search result items
        """
        all_items = []
        start_index = 1
        results_per_page = 10

        while len(all_items) < total_results:
            remaining = total_results - len(all_items)
            num_results = min(remaining, results_per_page)

            try:
                result = self.search(
                    query=query,
                    num_results=num_results,
                    start_index=start_index,
                    **kwargs
                )

                items = result.get('items', [])
                if not items:
                    break  # No more results available

                all_items.extend(items)
                start_index += len(items)

                # Check if we've reached the end of available results
                total_available = int(result.get('searchInformation', {}).get('totalResults', 0))
                if start_index > total_available:
                    break

            except Exception as e:
                logger.warning(f"Error retrieving page starting at {start_index}: {e}")
                break

        logger.info(f"Multi-page search completed: {len(all_items)} total results")
        return all_items

    def image_search(self, query: str, num_results: int = 10, **kwargs) -> Dict[str, Any]:
        """
        Perform an image search.

        Args:
            query: Search query
            num_results: Number of results to return
            **kwargs: Additional search parameters

        Returns:
            Image search results
        """
        return self.search(
            query=query,
            num_results=num_results,
            search_type="image",
            **kwargs
        )


# Create extension instance
extension = GoogleSearchAPIExtension()

# Export main classes and functions
__all__ = [
    "GoogleSearchAPIExtension",
    "GoogleSearchAPIClient",
    "check_dependencies",
    "DEPENDENCIES",
    "extension"
]