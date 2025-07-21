"""
Configuration constants for the GraphBit chatbot backend.

This module contains all the configuration constants used throughout the chatbot
application, including file paths, model settings, and API configurations.
"""

import os


class ConfigConstants:
    """Centralized configuration constants for the chatbot backend."""

    VECTOR_DB_TEXT_FILE = "backend/data/vectordb.txt"
    VECTOR_DB_INDEX_NAME = "vector_index_chatbot"
    OPENAI_API_KEY = os.getenv("OPENAI_API_KEY", "")
    CHUNK_SIZE = 1000
    OVERLAP_SIZE = 100
    RETRIEVE_CONTEXT_N_RESULTS = 5
    COLLECTION_NAME = "chatbot_memory"
    OPENAI_LLM_MODEL = "gpt-3.5-turbo"
    OPENAI_EMBEDDING_MODEL = "text-embedding-3-small"
    MAX_TOKENS = 200
