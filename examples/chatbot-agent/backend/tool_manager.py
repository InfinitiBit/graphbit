"""
Tool module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import logging
import os
from typing import List

from dotenv import load_dotenv

from graphbit import tool

from .const import VectorDB
from .vectordb_manager import VectorDBManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


@tool(_description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 22, "condition": "sunny"}


class ToolManager:
    """ToolManager handles defining and managing tools for the chatbot agent."""

    def __init__(self, vector_db_manager: VectorDBManager):
        """Initialize the ToolManager."""
        self.vector_db_manager = vector_db_manager

    @tool(
        _description=(
            "Retrieve personal facts using VECTOR SIMILARITY.\n"
            "INPUT must be a SHORT, FIRST-PERSON phrase likely to appear verbatim in the stored text.\n"
            "Rewrite the user's question into such a phrase.\n"
            "Include key entities/names; avoid abstract keywords.\n"
            "Good: 'I work at InfinitiBit', 'my employer is InfinitiBit', 'I live in Dhaka'.\n"
            "Bad: 'employer', 'workplace', 'residence'."
        )
    )
    def get_personal_info(self, vector_similarity_search_query: str) -> List[str]:
        logging.info(f"Retrieving personal info from vectordb for query: {vector_similarity_search_query}")
        print("Tool 1: get_personal_info: ", vector_similarity_search_query)
        result = self.vector_db_manager._retrieve_context(vector_similarity_search_query, collection=VectorDB.PERSONAL_INFO_COLLECTION)
        print("Tool 1: get_personal_info: ", result)
        return result

    @tool(_description="retrieves chat history of the user from vectordb")
    def get_chat_history(self, query: str) -> List[str]:
        logging.info(f"Retrieving chat history from vectordb for query: {query}")
        print("Tool 2: get_chat_history: ", query)
        return self.vector_db_manager._retrieve_context(query, collection=VectorDB.HISTORY_COLLECTION)

    @tool(
        _description=(
            "Save personal info to the vector DB. "
            "CALL THIS when the user reveals new profile facts/ personal information (name, email, phone, job title, company, preferences, location, etc). "
            "Args: doc_content = user information formatted in a way better for searching, "
            "metadata = {'fields': ['name','email',...], 'source': 'chat'}."
        )
    )
    def save_personal_info(self, doc_content: str, metadata: dict) -> None:
        logging.info(f"Saving personal info to vectordb: {doc_content} with metadata: {metadata}")
        print("Tool 3: save_personal_info: ", doc_content)
        self.vector_db_manager._save_to_vectordb(doc_content, metadata, collection=VectorDB.PERSONAL_INFO_COLLECTION)

    @tool(
        _description=(
            "Save each chat turn to the vector DB. "
            "CALL THIS once per turn BEFORE your final reply is sent. "
            "Args: doc_content = 'user: ...\\nassistant: ...', "
            "metadata = {'session_id': '<id>'}."
        )
    )
    def save_chat_history(self, doc_content: str, metadata: dict) -> None:
        logging.info(f"Saving chat history to vectordb: {doc_content} with metadata: {metadata}")
        print("Tool 4: save_chat_history: ", doc_content, metadata)
        self.vector_db_manager._save_to_vectordb(doc_content, metadata, collection=VectorDB.HISTORY_COLLECTION)
