"""
LLM Manager module for Research Paper Summarizer.

This module provides a centralized LLM client management with automatic tracing
support using GraphBit's tracer framework.
"""

import logging
import os
from typing import Optional

from dotenv import load_dotenv

from graphbit import LlmClient, LlmConfig
from graphbit_tracer import AutoTracer

from .constant import ConfigConstants

load_dotenv()

os.makedirs(ConfigConstants.LOG_DIR, exist_ok=True)
logging.basicConfig(
    filename=os.path.join(ConfigConstants.LOG_DIR, ConfigConstants.LOG_FILE),
    filemode="a",
    format=ConfigConstants.LOG_FORMAT,
    level=logging.INFO,
)


class LLMManager:
    """
    LLMManager handles the configuration and interaction with the language model client.

    This class manages the LLM client with automatic tracing support, providing
    a centralized way to access the LLM client throughout the application.
    """

    def __init__(
        self,
        api_key: Optional[str] = None,
        tracing_api_key: Optional[str] = None,
        traceable_project: Optional[str] = None,
        tracing_api_url: Optional[str] = None,
    ):
        """
        Initialize the LLMManager with API keys and tracing configuration.

        Args:
            api_key (str, optional): OpenAI API key. Defaults to ConfigConstants.OPENAI_API_KEY.
            tracing_api_key (str, optional): GraphBit tracing API key. Defaults to ConfigConstants.GRAPHBIT_TRACING_API_KEY.
            traceable_project (str, optional): Traceable project name. Defaults to ConfigConstants.GRAPHBIT_TRACEABLE_PROJECT.
            tracing_api_url (str, optional): Tracing API URL. Defaults to ConfigConstants.GRAPHBIT_TRACING_API_URL.
        """
        # Use provided values or fall back to ConfigConstants
        api_key = api_key or ConfigConstants.OPENAI_API_KEY
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Configure LLM
        self.llm_config = LlmConfig.openai(model=ConfigConstants.LLM_MODEL, api_key=api_key)
        self._llm_client = LlmClient(self.llm_config)
        self._traced_client = None

        # Control tracing
        self._tracing_api_key = tracing_api_key or ConfigConstants.GRAPHBIT_TRACING_API_KEY
        self._traceable_project = traceable_project or ConfigConstants.GRAPHBIT_TRACEABLE_PROJECT
        self._tracing_api_url = tracing_api_url or ConfigConstants.GRAPHBIT_TRACING_API_URL
        self._tracer = None
        self._tracing_initialized = False

    async def _ensure_traced_client(self):
        """Ensure the traced LLM client is initialized (lazy initialization)."""
        if not self._tracing_initialized and self._tracing_api_key and self._traceable_project and self._tracing_api_url:
            try:
                self._tracer = await AutoTracer.create()
                self._traced_client = self._tracer.wrap_client(self._llm_client, self.llm_config)
                self._tracing_initialized = True
                logging.info("LLM client wrapped with tracer")
            except Exception as e:
                logging.warning(f"Failed to initialize tracing, falling back to non-traced client: {e}")
                self._traced_client = self._llm_client
                self._tracing_initialized = True

    @property
    def llm_client(self):
        """Get the LLM client (traced if available, otherwise base client)."""
        if self._traced_client is not None:
            return self._traced_client
        return self._llm_client

    async def complete_async(self, prompt: str, max_tokens: int = None, temperature: float = None):
        """
        Complete a prompt using the LLM client with automatic tracing.

        Args:
            prompt (str): The input prompt.
            max_tokens (int, optional): Maximum tokens to generate.
            temperature (float, optional): Temperature for generation.

        Returns:
            str: The generated response.
        """
        # Ensure traced client is initialized before making LLM calls
        await self._ensure_traced_client()

        # Use default values from config if not provided
        max_tokens = max_tokens or ConfigConstants.LLM_MAX_TOKENS
        temperature = temperature if temperature is not None else ConfigConstants.LLM_TEMPERATURE

        response = await self.llm_client.complete_async(
            prompt=prompt, max_tokens=max_tokens, temperature=temperature
        )

        # Send traces to API if tracing is enabled
        if self._tracing_initialized and self._tracer:
            try:
                results = await self._tracer.send_to_api()
                logging.info(f"Traces sent - Sent: {results['sent']}, Failed: {results['failed']}")
            except Exception as e:
                logging.warning(f"Failed to send traces to API: {e}")

        return response

    async def shutdown(self):
        """Shutdown the tracer and cleanup resources."""
        if self._tracer:
            try:
                await self._tracer.shutdown()
                logging.info("Tracer shutdown successfully")
            except Exception as e:
                logging.warning(f"Failed to shutdown tracer: {e}")

