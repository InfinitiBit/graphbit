"""
Tool module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import logging
import os
from typing import Optional

from chromadb import Client
from chromadb.config import Settings
from dotenv import load_dotenv

from graphbit import tool

from .const import COLLECTIONS_TEXT_FILES, ConfigConstants
from .vectordb_manager import VectorDBManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


@tool(_description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}


class ToolManager:
    """
    ToolManager handles defining and managing tools for the chatbot agent.
    """

    def __init__(self):
        """
        Initialize the ToolManager.
        """
        self.vector_db_manager = VectorDBManager()

    @tool(_description="retrieves personal info of a user from vectordb")
    def get_personal_info(self, query: str):
        return self.vector_db_manager._retrieve_context(query, collection=ConfigConstants.PERSONAL_INFO_COLLECTION_NAME)

    @tool(_description="retrieves chat history of a user from vectordb")
    def get_chat_history(self, query: str):
        return self.vector_db_manager._retrieve_context(query, collection=ConfigConstants.HISTORY_COLLECTION_NAME)

    @tool(_description="save personal info of a user to vectordb")
    def save_personal_info(self, doc_content: str, metadata: dict):
        self.vector_db_manager._save_to_vectordb(doc_content, metadata, collection=ConfigConstants.PERSONAL_INFO_COLLECTION_NAME)

    @tool(_description="save chat history of a user to vectordb")
    def save_chat_history(self, doc_content: str, metadata: dict):
        self.vector_db_manager._save_to_vectordb(doc_content, metadata, collection=ConfigConstants.HISTORY_COLLECTION_NAME)
