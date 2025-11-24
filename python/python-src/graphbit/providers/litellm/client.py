from .llm import LiteLLMLLM
from .embeddings import LiteLLMEmbeddings
from typing import Optional


class Litellm:
    """
    Unified LiteLLM integration for LLM and embeddings.

    This class provides a single interface to access both chat completion
    and embedding functionality through LiteLLM's unified API.

    Example:
        >>> from graphbit.providers import Litellm
        >>> import os
        >>>
        >>> # Initialize client
        >>> litellm = Litellm(api_key=os.getenv("OPENAI_API_KEY"))
        >>>
        >>> # Use chat completion
        >>> response = litellm.llm.chat(
        ...     model="gpt-3.5-turbo",
        ...     messages=[{"role": "user", "content": "Hello!"}]
        ... )
        >>> content = litellm.llm.get_output_content(response)
        >>> print(content)
        >>>
        >>> # Use embeddings
        >>> embeddings = litellm.embeddings.embed(
        ...     model="text-embedding-ada-002",
        ...     text="Hello world"
        ... )
        >>> print(f"Embedding dimension: {len(embeddings)}")
    """

    def __init__(self, api_key: Optional[str] = None, **kwargs):
        """
        Initialize LiteLLM client with unified access to LLM and embeddings.

        Args:
            api_key: API key for authentication (optional, can use environment variables)
            **kwargs: Additional configuration options
        """
        self.api_key = api_key
        self.llm = LiteLLMLLM(api_key=api_key, **kwargs)
        self.embeddings = LiteLLMEmbeddings(api_key=api_key, **kwargs)
