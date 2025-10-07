"""
Chatbot Manager module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import json
import logging
import os
from typing import List

from dotenv import load_dotenv
from fastapi import WebSocket

from .agent_manager import AgentManager
from .const import COLLECTIONS_TEXT_FILES, ConfigConstants, VectorDB
from .embedding_manager import EmbeddingManager
from .tool_manager import ToolManager
from .vectordb_manager import VectorDBManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


class ChatbotManager:
    """
    ChatbotManager orchestrates conversation handling for the chatbot.

    This class coordinates between VectorDBManager, EmbeddingManager, and AgentManager to provide
    complete chatbot functionality including context retrieval, response generation,
    and conversation memory storage using GraphBit's workflow system.
    """

    def __init__(self, index_name: str = ConfigConstants.VECTOR_DB_INDEX_NAME):
        """
        Initialize the ChatbotManager with necessary configurations.

        Args:
            index_name (str, optional): Name of the vector database index to use.
        """
        logging.info("Initializing ChatbotManager")
        self.index_name: str = index_name

        # Ensure OpenAI API key is present
        openai_api_key = ConfigConstants.OPENAI_API_KEY
        if not openai_api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Ensure OpenAI embedding model is present
        openai_embedding_model = ConfigConstants.OPENAI_EMBEDDING_MODEL
        if not openai_embedding_model:
            raise ValueError("OPENAI_EMBEDDING_MODEL environment variable is not set. Please set it in your environment.")

        # Ensure OpenAI LLM model is present
        llm_model = ConfigConstants.OPENAI_LLM_MODEL
        if not llm_model:
            raise ValueError("OPENAI_LLM_MODEL environment variable is not set. Please set it in your environment.")

        # Initialize EmbeddingManager
        self.embedding_manager = EmbeddingManager(openai_api_key, openai_embedding_model)

        # Initialize ChromaDB
        self.vector_db_manager = VectorDBManager(index_name=self.index_name, embedding_manager=self.embedding_manager)

        # Initialize Tool Manager
        self.tool_manager = ToolManager(vector_db_manager=self.vector_db_manager)

        # Initialize AgentManager
        self.agent_manager = AgentManager(openai_api_key, model=llm_model, tool_manager=self.tool_manager)

    def _create_index(self, collection: List[VectorDB] = [VectorDB.HISTORY_COLLECTION, VectorDB.PERSONAL_INFO_COLLECTION]) -> None:
        """Create vector index from a text file."""
        for collection in collection:
            self.vector_db_manager._create_index(collection)

    async def stream_full_chat(self, websocket: WebSocket, session_id: str, prompt: str):  # needed
        """Stream chat response tokens to the client via WebSocket."""
        response = ""
        async for token in self.agent_manager.chat_stream(prompt):
            response += token
            await websocket.send_text(json.dumps({"response": token, "session_id": session_id, "type": "chunk"}))
        return response

    async def chat(self, websocket: WebSocket, session_id: str, query: str) -> str:  # needed
        """
        Handle a chat message: manage session, retrieve context, generate and stream response, and store conversation.

        Args:
            websocket (WebSocket): WebSocket connection for streaming.
            session_id (str): Unique chat session ID.
            query (str): User's input message.

        Returns:
            str: Generated chatbot response or error message.
        """
        try:
            user_message = {"role": "user", "content": query}
            logging.info(f"User message: {user_message}")
            print(user_message)

            stream_response = await self.stream_full_chat(websocket, session_id, query)
            await websocket.send_text(json.dumps({"response": "", "session_id": session_id, "type": "end"}))

            # Add AI response to session
            ai_message = {"role": "assistant", "content": stream_response}
            logging.info(f"AI message: {ai_message}")

            return stream_response

        except Exception as e:
            logging.error(f"Error in chat: {str(e)}")
            return f"Sorry, I encountered an error: {str(e)}"
