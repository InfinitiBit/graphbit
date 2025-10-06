"""
Chatbot Manager module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import logging
import os
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

    def __init__(self, api_key: Optional[str] = ConfigConstants.OPENAI_API_KEY):
        """
        Initialize the EmbeddingManager with the OpenAI API key.

        Args:
            api_key (str): OpenAI API key for accessing the language model.
        """
        # Ensure OpenAI API key is present
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Configure embeddings
        self.embedding_config = EmbeddingConfig.openai(model=ConfigConstants.OPENAI_EMBEDDING_MODEL, api_key=api_key)
        self.embedding_client = EmbeddingClient(self.embedding_config)

    def embed(self, text: str) -> List[float]:
        """Generate embeddings for the given text using the configured embedding model."""
        return self.embedding_client.embed(text)

    def embed_many(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for multiple texts using the configured embedding model."""
        return self.embedding_client.embed_many(texts)

    def sentence_splitter(self, text: str, CHUNK_SIZE: int = ConfigConstants.CHUNK_SIZE, OVERLAP_SIZE: int = ConfigConstants.OVERLAP_SIZE) -> List[str]:
        """Split text into sentences."""
        splitter = SentenceSplitter(chunk_size=CHUNK_SIZE, chunk_overlap=OVERLAP_SIZE)
        return splitter.split_text(text)

    def character_splitter(self, text: str, CHUNK_SIZE: int = ConfigConstants.CHUNK_SIZE, OVERLAP_SIZE: int = ConfigConstants.OVERLAP_SIZE) -> List[str]:
        """Split text into characters."""
        splitter = CharacterSplitter(chunk_size=CHUNK_SIZE, chunk_overlap=OVERLAP_SIZE)
        return splitter.split_text(text)
