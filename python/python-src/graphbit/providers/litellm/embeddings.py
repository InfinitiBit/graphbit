from litellm import embedding
from typing import List, Optional, Union
import os


class LiteLLMEmbeddings:
    """
    LiteLLM wrapper for embeddings functionality.

    Supports embeddings from multiple providers through a unified interface.
    """

    def __init__(self, api_key: Optional[str] = None, **kwargs):
        """
        Initialize LiteLLM embeddings client.

        Args:
            api_key: API key for the provider (optional, can be set via environment variables)
            **kwargs: Additional configuration options
        """
        self.api_key = api_key
        self.config = kwargs

    def embed(
        self,
        model: str,
        text: Union[str, List[str]],
        dimensions: Optional[int] = None,
        encoding_format: Optional[str] = None,
        input_type: Optional[str] = None,
        **kwargs
    ) -> Union[List[float], List[List[float]]]:
        """
        Generate embeddings for the given text.

        Args:
            model: Model identifier (e.g., "text-embedding-ada-002", "embed-english-v3.0")
            text: Text string or list of text strings to embed
            dimensions: Number of dimensions for the output embeddings (optional)
            encoding_format: Format to return embeddings ("float" or "base64")
            input_type: Type of input for certain models (e.g., "search_document", "search_query")
            **kwargs: Additional provider-specific parameters

        Returns:
            List of embeddings (single list for one text, list of lists for multiple texts)

        Example:
            >>> embeddings = LiteLLMEmbeddings(api_key="your-key")
            >>> result = embeddings.embed(
            ...     model="text-embedding-ada-002",
            ...     text="Hello world"
            ... )
        """
        # Set API key if provided
        if self.api_key:
            # LiteLLM uses environment variables for authentication
            if "OPENAI_API_KEY" not in os.environ:
                os.environ["OPENAI_API_KEY"] = self.api_key

        # Normalize input to list format
        input_texts = [text] if isinstance(text, str) else text

        # Build embedding parameters
        embedding_params = {
            "model": model,
            "input": input_texts,
        }

        # Add optional parameters if provided
        if dimensions is not None:
            embedding_params["dimensions"] = dimensions
        if encoding_format is not None:
            embedding_params["encoding_format"] = encoding_format
        if input_type is not None:
            embedding_params["input_type"] = input_type

        # Merge with any additional kwargs
        embedding_params.update(kwargs)

        try:
            response = embedding(**embedding_params)

            # Extract embeddings from response
            embeddings = [item["embedding"] for item in response.data]

            # Return single embedding if input was a single string
            if isinstance(text, str):
                return embeddings[0]
            return embeddings

        except Exception as e:
            raise Exception(f"LiteLLM embedding failed: {str(e)}")

    def embed_batch(
        self,
        model: str,
        texts: List[str],
        dimensions: Optional[int] = None,
        encoding_format: Optional[str] = None,
        input_type: Optional[str] = None,
        **kwargs
    ) -> List[List[float]]:
        """
        Generate embeddings for a batch of texts.

        Args:
            model: Model identifier
            texts: List of text strings to embed
            dimensions: Number of dimensions for the output embeddings (optional)
            encoding_format: Format to return embeddings ("float" or "base64")
            input_type: Type of input for certain models
            **kwargs: Additional provider-specific parameters

        Returns:
            List of embeddings, one for each input text

        Example:
            >>> embeddings = LiteLLMEmbeddings(api_key="your-key")
            >>> results = embeddings.embed_batch(
            ...     model="text-embedding-ada-002",
            ...     texts=["Hello", "World"]
            ... )
        """
        result = self.embed(
            model=model,
            text=texts,
            dimensions=dimensions,
            encoding_format=encoding_format,
            input_type=input_type,
            **kwargs
        )
        # embed() returns List[List[float]] for list input
        return result

    def get_embedding_dimension(self, model: str, sample_text: str = "test") -> int:
        """
        Get the dimension of embeddings for a given model.

        Args:
            model: Model identifier
            sample_text: Sample text to use for testing (default: "test")

        Returns:
            Dimension of the embedding vector
        """
        try:
            sample_embedding = self.embed(model=model, text=sample_text)
            return len(sample_embedding)
        except Exception as e:
            raise Exception(f"Failed to get embedding dimension: {str(e)}")
