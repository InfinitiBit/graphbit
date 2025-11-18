"""
Chatbot Manager module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import json
import logging
import os
from typing import Any, Dict, List, Optional

from dotenv import load_dotenv
from fastapi import WebSocket

from .const import ConfigConstants
from .llm_manager import LLMManager
from .vectordb_manager import VectorDBManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


class ChatbotManager:
    """
    ChatbotManager orchestrates conversation handling for the chatbot.

    This class coordinates between VectorDBManager and LLMManager to provide
    complete chatbot functionality including context retrieval, response generation,
    and conversation memory storage using GraphBit's workflow system.
    """

    def __init__(self, index_name: str = ConfigConstants.VECTOR_DB_INDEX_NAME):
        """
        Initialize the ChatbotManager with necessary configurations.

        Args:
            index_name (str, optional): Name of the vector database index to use.
        """
        self.index_name: str = index_name

        # Ensure OpenAI API key is present
        openai_api_key = ConfigConstants.OPENAI_API_KEY
        if not openai_api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        self.llm_manager = LLMManager(openai_api_key)

        # Initialize ChromaDB
        self.vector_db_manager = VectorDBManager(index_name=self.index_name, llm_manager=self.llm_manager)

        # Session storage for message history
        self.sessions: Dict[str, List[Any]] = {}

    def _create_index(self, file_path: str = ConfigConstants.VECTOR_DB_TEXT_FILE) -> None:
        """Create vector index from a text file."""
        self.vector_db_manager._create_index(file_path)

    def _retrieve_context(self, query: str) -> str:
        """Retrieve relevant context from the vector database."""
        return self.vector_db_manager._retrieve_context(query)

    def _save_to_vectordb(self, query: str, session_id: str, response: str) -> None:
        """Save conversation to vector database by delegating to VectorDBManager."""
        try:
            if response:
                doc_content = f"Question: {query}\nAnswer: {response}"
            else:
                doc_content = f"Question: {query}\nAnswer: No processed summary available"
            metadata = {"session_id": session_id, "type": "qa_pair", "source": "chatbot_response"}
            self.vector_db_manager._save_to_vectordb(doc_content, metadata)
        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def format_prompt_ai_response(self, context: Optional[str] = "", chat_history: Optional[str] = "", query: Optional[str] = "") -> str:
        """
        Build the AI prompt using context, chat history, and the current question.

        Args:
            context (str): Relevant document context.
            chat_history (str): Recent conversation history.
            query (str): User's current question.

        Returns:
            str: Formatted prompt for the AI assistant.
        """
        prompt = f"""You are a helpful and friendly AI assistant. You can answer questions, hold normal conversations, and remember what the user has told you in this session.
You have access to external documents and chat history that you should use to enhance your answer when relevant.
Always try to:
- Understand the intent behind short or vague inputs
- Ask clarifying questions if needed
- Keep the conversation engaging and natural
- Use the chat history for personalization
- Reference the document context when it's clearly relevant

Document Context:
{context}

Recent Chat History:
{chat_history}

Current Question: {query}

Provide a helpful and engaging response:"""
        return prompt

    async def stream_full_chat(self, websocket: WebSocket, session_id: str, prompt: str):
        """Stream chat response tokens to the client via WebSocket."""
        response = ""
        async for token in self.llm_manager.chat_stream(prompt):
            response += token
            await websocket.send_text(json.dumps({"response": token, "session_id": session_id, "type": "chunk"}))
        return response

    async def chat(self, websocket: WebSocket, session_id: str, query: str) -> str:
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
            if session_id not in self.sessions:
                self.sessions[session_id] = []

            user_message = {"role": "user", "content": query}
            self.sessions[session_id].append(user_message)

            # Retrieve Context
            retrieved_docs = self._retrieve_context(query)

            # Get AI response
            prompt = self.format_prompt_ai_response(context=retrieved_docs, query=query)
            stream_response = await self.stream_full_chat(websocket, session_id, prompt)
            await websocket.send_text(json.dumps({"response": "", "session_id": session_id, "type": "end"}))

            # Add AI response to session
            ai_message = {"role": "assistant", "content": stream_response}
            self.sessions[session_id].append(ai_message)

            # Save to vector database
            self._save_to_vectordb(query, session_id, stream_response)

            return stream_response

        except Exception as e:
            logging.error(f"Error in chat: {str(e)}")
            return f"Sorry, I encountered an error: {str(e)}"
