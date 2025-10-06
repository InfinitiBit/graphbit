"""
Configuration constants for the GraphBit chatbot backend.

This module contains all the configuration constants used throughout the chatbot
application, including file paths, model settings, and API configurations.
"""

import os


class ConfigConstants:
    """Centralized configuration constants for the chatbot backend."""

    VECTOR_DB_CHAT_HISTORY_TEXT_FILE = "backend/data/vectordb_chat_history.txt"
    VECTOR_DB_PERSONAL_INFO_TEXT_FILE = "backend/data/vectordb_personal_info.txt"
    VECTOR_DB_INDEX_NAME = "vector_index_chatbot"
    OPENAI_API_KEY = os.getenv("OPENAI_API_KEY", "")
    CHUNK_SIZE = 10  # Number of sentence in a chunk for sentence splitters
    OVERLAP_SIZE = 1  # Number of sentence to overlap for sentence splitters
    RETRIEVE_CONTEXT_N_RESULTS = 5
    HISTORY_COLLECTION_NAME = "chatbot_history_collection"
    PERSONAL_INFO_COLLECTION_NAME = "chatbot_personal_info_collection"
    OPENAI_LLM_MODEL = "gpt-3.5-turbo"
    OPENAI_EMBEDDING_MODEL = "text-embedding-3-small"
    MAX_TOKENS = 200


COLLECTIONS_TEXT_FILES = {
    ConfigConstants.HISTORY_COLLECTION_NAME: ConfigConstants.VECTOR_DB_CHAT_HISTORY_TEXT_FILE,
    ConfigConstants.PERSONAL_INFO_COLLECTION_NAME: ConfigConstants.VECTOR_DB_PERSONAL_INFO_TEXT_FILE,
}
