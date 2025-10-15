"""
Chatbot Manager module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import logging
import os
from concurrent.futures import ThreadPoolExecutor
from typing import List, Optional

from dotenv import load_dotenv

from graphbit import CharacterSplitter, EmbeddingClient, EmbeddingConfig, SentenceSplitter

from .const import ConfigConstants

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


class EmbeddingManager:
    """
    EmbeddingManager handles the configuration and interaction with the embedding client.
    This class manages the OpenAI embedding clients, providing methods
    for generating embeddings.
    """

    def __init__(self, api_key: str, model: str):
        """
        Initialize the EmbeddingManager with the OpenAI API key.

        Args:
            api_key (str): OpenAI API key for accessing the language model.
        """
        # Ensure OpenAI API key is present
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Configure embeddings
        self.embedding_config = EmbeddingConfig.openai(model=model, api_key=api_key)
        self.embedding_client = EmbeddingClient(self.embedding_config)
        # a dedicated worker thread avoids nesting Tokio runtimes
        max_workers = 1
        self._executor = ThreadPoolExecutor(max_workers=max_workers, thread_name_prefix="gb-embed")

    def _run(self, fn, *args, **kwargs):
        fut = self._executor.submit(fn, *args, **kwargs)
        return fut.result()

    def embed(self, text: str) -> List[float]:
        """Generate embeddings for the given text using the configured embedding model."""
        return self._run(self.embedding_client.embed, text)

    def embed_many(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for multiple texts using the configured embedding model."""
        return self._run(self.embedding_client.embed_many, texts)

    def sentence_splitter(self, text: str, chunk_size: int = ConfigConstants.CHUNK_SIZE, overlap: int = ConfigConstants.OVERLAP_SIZE) -> List[str]:
        """Split text into sentences."""
        splitter = SentenceSplitter(chunk_size=chunk_size, chunk_overlap=overlap)
        chunks = [chunk.content for chunk in splitter.split_text(text)]
        return chunks

    def character_splitter(self, text: str, CHUNK_SIZE: int = ConfigConstants.CHUNK_SIZE, OVERLAP_SIZE: int = ConfigConstants.OVERLAP_SIZE) -> List[str]:
        """Split text into characters."""
        splitter = CharacterSplitter(chunk_size=CHUNK_SIZE, chunk_overlap=OVERLAP_SIZE)
        chunks = [chunk.content for chunk in splitter.split_text(text)]
        return chunks
