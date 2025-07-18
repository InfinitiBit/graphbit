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

from chromadb import Client
from chromadb.config import Settings
from dotenv import load_dotenv
from fastapi import WebSocket

import graphbit

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)

VECTOR_DB_TEXT_FILE = "backend/data/vectordb.txt"
VECTOR_DB_INDEX_NAME = "vector_index_chatbot"
CHUNK_SIZE = 1000
OVERLAP_SIZE = 100


class ChatbotManager:
    """
    ChatbotManager orchestrates conversation handling for the chatbot.

    This class manages context retrieval, response generation, and memory storage
    using GraphBit's workflow system and a vector database for persistent memory.
    """

    def __init__(self, index_name: str = VECTOR_DB_INDEX_NAME):
        """
        Initialize the ChatbotManager with necessary configurations.

        Args:
            index_name (str): Name of the vector database index to use.
        """
        graphbit.init()

        self.index_name: str = index_name

        # Ensure OpenAI API key is present
        openai_api_key = os.getenv("OPENAI_API_KEY")
        if not openai_api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Configure LLM
        self.llm_config = graphbit.LlmConfig.openai(model="gpt-3.5-turbo", api_key=openai_api_key)
        self.llm_client = graphbit.LlmClient(self.llm_config)

        # Configure embeddings
        self.embedding_config = graphbit.EmbeddingConfig.openai(model="text-embedding-3-small", api_key=openai_api_key)
        self.embedding_client = graphbit.EmbeddingClient(self.embedding_config)

        # Initialize ChromaDB
        self.chroma_client: Optional[Client] = None
        self.collection = None
        self._init_vectorstore()

        # Session storage for message history
        self.sessions: Dict[str, List[Any]] = {}

    def _init_vectorstore(self) -> None:
        try:
            self.chroma_client = Client(Settings(persist_directory=self.index_name, is_persistent=True))
            if self.chroma_client is not None:
                if "chatbot_memory" in [c.name for c in self.chroma_client.list_collections()]:
                    self.collection = self.chroma_client.get_collection(name="chatbot_memory")
                    logging.info("Loaded existing ChromaDB collection")
                else:
                    self.collection = self.chroma_client.create_collection(name="chatbot_memory")
                    logging.info("Created new ChromaDB collection")

        except Exception as e:
            logging.error(f"Error initializing vector store: {str(e)}")
            self.chroma_client = None
            self.collection = None

    def _create_index(self, file_path: str = VECTOR_DB_TEXT_FILE) -> None:
        try:
            os.makedirs(os.path.dirname(file_path), exist_ok=True)

            if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write("Conversation History:\n")
                    f.write("This is the initial knowledge base for the chatbot.\n")
                    f.write("The chatbot can answer questions and hold conversations.\n")

            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()

            chunks = self._split_text(content, chunk_size=CHUNK_SIZE, overlap=OVERLAP_SIZE)

            if self.collection and chunks:
                embeddings = self.embedding_client.embed_many(chunks)

                for i, (chunk, embedding) in enumerate(zip(chunks, embeddings)):
                    doc_id = f"doc_{i}"
                    self.collection.add(documents=[chunk], embeddings=[embedding], ids=[doc_id], metadatas=[{"source": "initial_knowledge", "chunk_id": i}])

                logging.info(f"Vector store created with {len(chunks)} chunks")
            else:
                logging.warning("No content to index or collection not available")

        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")
            raise

    def _split_text(self, text: str, chunk_size: int = CHUNK_SIZE, overlap: int = OVERLAP_SIZE) -> List[str]:
        chunks = []
        start = 0

        while start < len(text):
            end = start + chunk_size
            chunk = text[start:end]

            if end < len(text):
                last_space = chunk.rfind(" ")
                if last_space > chunk_size:
                    chunk = chunk[:last_space]
                    end = start + last_space

            chunks.append(chunk.strip())
            start = end - overlap

            if start >= len(text):
                break

        return [chunk for chunk in chunks if chunk.strip()]

    def _retrieve_context(self, query: str) -> str:
        try:
            if not self.collection:
                return "No vector store available"

            query_embedding = self.embedding_client.embed(query)

            results = self.collection.query(query_embeddings=[query_embedding], n_results=5)

            if results["documents"] and results["documents"][0]:
                context_docs = results["documents"][0]
                context = "\n\n".join(context_docs)
                logging.info(f"Retrieved {len(context_docs)} documents for context")
                return context
            else:
                logging.info("No documents found in similarity search")
                return "No relevant context found in vector database"

        except Exception as e:
            logging.error(f"Error retrieving context: {str(e)}")
            return f"Error retrieving context: {str(e)}"

    def _save_to_vectordb(self, query: str, session_id: str, response: str) -> None:
        try:
            if not self.collection:
                logging.warning("Vector store not initialized, skipping save")
                return

            if response:
                doc_content = f"Question: {query}\nAnswer: {response}"
            else:
                doc_content = f"Question: {query}\nAnswer: No processed summary available"

            doc_id = f"session_{session_id}_{len(self.sessions.get(session_id, []))}"

            doc_embedding = self.embedding_client.embed(doc_content)

            with open(VECTOR_DB_TEXT_FILE, "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")

            # Add to vector store
            self.collection.add(documents=[doc_content], embeddings=[doc_embedding], ids=[doc_id], metadatas=[{"session_id": session_id, "type": "qa_pair", "source": "chatbot_response"}])

            logging.info(f"Saved conversation to vector DB for session {session_id}")

        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def extract_output(self, context, fallback_name="Response Generator") -> str:
        """
        Extract meaningful output from workflow execution context.

        Args:
            context: The workflow execution context containing results.
            fallback_name (str): Fallback name to use if no output is found.

        Returns:
            str: Extracted output string or fallback message.
        """
        all_results = context.get_all_variables()
        if all_results:
            for value in all_results.values():
                value_str = str(value).strip()
                if value_str and value_str.lower() not in ["null", "none", '"null"', '"none"']:
                    return value_str
        return f"{fallback_name} completed successfully, but no detailed output was captured."

    def format_prompt(self, context: Optional[str] = "", chat_history: Optional[str] = "", query: Optional[str] = "") -> str:
        """
        Format the prompt for the AI assistant, including context, chat history, and the current question.

        Args:
            context (str, optional): Retrieved document context relevant to the query.
            chat_history (str, optional): Recent conversation history.
            query (str, optional): The user's current question.

        Returns:
            str: Formatted prompt string.
        """
        prompt = f"""You are a helpful and friendly AI assistant.

Document Context:
{context}

Recent Chat History:
{chat_history}

Current Question: {query}

Provide a helpful and engaging response:"""
        return prompt

    async def chat_stream(self, prompt: str):
        """
        Stream chat response tokens from the LLM client.

        This method provides a streaming interface for chat completions, yielding
        response tokens as they are generated by the language model.

        Args:
            prompt (str): The input prompt to send to the language model.

        Yields:
            str: Individual response tokens from the streaming completion.
        """
        response = await self.llm_client.complete_stream(prompt, max_tokens=200)
        for chunk in response:
            yield chunk

    async def full_chat_stream(self, websocket: WebSocket, session_id: str, prompt: str):
        """
        Streams chat response tokens to the client via WebSocket.

        Args:
            websocket (WebSocket): The WebSocket connection to send data through.
            session_id (str): The identifier for the chat session.
            prompt (str): The input prompt for the chat model.

        Returns:
            str: The full accumulated response from the chat stream.
        """
        response = ""
        async for token in self.chat_stream(prompt):
            response += token
            await websocket.send_text(json.dumps({"response": token, "session_id": session_id, "type": "chunk"}))
        return response

    async def chat(self, websocket: WebSocket, session_id: str, query: str) -> str:
        """
        Process a chat message and generate a response.

        Args:
            session_id (str): Unique identifier for the chat session.
            query (str): User's input message.

        Returns:
            str: Generated response from the chatbot.
        """
        try:
            if session_id not in self.sessions:
                self.sessions[session_id] = []

            user_message = {"role": "user", "content": query}
            self.sessions[session_id].append(user_message)

            # Retrieve Context
            retrieved_docs = self._retrieve_context(query)

            # Get AI response
            prompt = self.format_prompt(context=retrieved_docs, query=query)
            stream_response = await self.full_chat_stream(websocket, session_id, prompt)
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
