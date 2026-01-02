from litellm import completion
from typing import List, Optional, Dict, Any
import os

class LiteLLMLLM:
    """
    LiteLLM wrapper for chat completion functionality.

    Supports 100+ LLM providers through a unified interface.
    """

    def __init__(self, api_key: Optional[str] = None, **kwargs):
        """
        Initialize LiteLLM client.

        Args:
            api_key: API key for the provider (optional, can be set via environment variables)
            **kwargs: Additional configuration options
        """
        self.api_key = api_key
        self.config = kwargs

    def chat(
        self,
        model: str,
        messages: List[Dict[str, str]],
        temperature: Optional[float] = None,
        max_tokens: Optional[int] = None,
        top_p: Optional[float] = None,
        stream: bool = False,
        **kwargs
    ) -> Any:
        """
        Chat with a model using LiteLLM.

        Args:
            model: Model identifier (e.g., "gpt-3.5-turbo", "claude-3-sonnet-20240229")
            messages: List of message dictionaries with 'role' and 'content' keys
            temperature: Sampling temperature (0.0 to 2.0)
            max_tokens: Maximum tokens to generate
            top_p: Nucleus sampling parameter
            stream: Whether to stream the response
            **kwargs: Additional provider-specific parameters

        Returns:
            Response object from LiteLLM completion

        Example:
            >>> llm = LiteLLMLLM(api_key="your-key")
            >>> response = llm.chat(
            ...     model="gpt-3.5-turbo",
            ...     messages=[{"role": "user", "content": "Hello!"}]
            ... )
        """
        # Set API key if provided
        if self.api_key:
            # LiteLLM uses environment variables for authentication
            # The specific env var depends on the provider
            # For now, we'll set OPENAI_API_KEY as a common case
            # Users can override by setting the appropriate env var themselves
            if "OPENAI_API_KEY" not in os.environ:
                os.environ["OPENAI_API_KEY"] = self.api_key

        # Build completion parameters
        completion_params = {
            "model": model,
            "messages": messages,
        }

        # Add optional parameters if provided
        if temperature is not None:
            completion_params["temperature"] = temperature
        if max_tokens is not None:
            completion_params["max_tokens"] = max_tokens
        if top_p is not None:
            completion_params["top_p"] = top_p
        if stream:
            completion_params["stream"] = stream

        # Merge with any additional kwargs
        completion_params.update(kwargs)

        try:
            response = completion(**completion_params)
            return response
        except Exception as e:
            raise Exception(f"LiteLLM completion failed: {str(e)}")

    def get_output_content(self, response: Any) -> str:
        """
        Extract the output content from the response.

        Args:
            response: Response object from LiteLLM completion

        Returns:
            The text content from the response
        """
        try:
            return response.choices[0].message.content
        except (AttributeError, IndexError, KeyError) as e:
            raise Exception(f"Failed to extract content from response: {str(e)}")
