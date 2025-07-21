"""
Chatbot Manager module for GraphBit-based conversational AI.

This module provides a comprehensive chatbot implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
memory storage capabilities.
"""

import logging
import os
from typing import List, Optional

from chromadb import Client
from chromadb.config import Settings
from dotenv import load_dotenv

from .const import ConfigConstants
from .llm_manager import LLMManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


class VectorDBManager:
    """
    VectorDBManager handles the initialization and management of the vector database.

    This class manages ChromaDB operations including collection creation, document
    indexing, similarity search, and conversation history storage for the chatbot.
    """

    def __init__(self, index_name: str = ConfigConstants.VECTOR_DB_INDEX_NAME, llm_manager: Optional[LLMManager] = None):
        """
        Initialize the VectorDBManager with the specified index name and LLM manager.

        Args:
            index_name (str, optional): Name of the vector database index to use.
            llm_manager (Optional[LLMManager], optional): LLM manager instance for
                                                        generating embeddings.
        """
        if llm_manager is None:
            llm_manager = LLMManager(ConfigConstants.OPENAI_API_KEY)
        self.llm_manager = llm_manager

        # Initialize ChromaDB
        self.index_name: str = index_name
        self.chroma_client: Optional[Client] = None
        self.collection = None

        self._init_vectorstore()

    def _init_vectorstore(self) -> None:
        """
        Initialize ChromaDB client and create or load the chatbot memory collection.

        This method sets up the persistent ChromaDB client and either loads an existing
        collection or creates a new one named 'chatbot_memory'.
        """
        try:
            self.chroma_client = Client(Settings(persist_directory=self.index_name, is_persistent=True))
            if self.chroma_client is not None:
                if ConfigConstants.COLLECTION_NAME in [c.name for c in self.chroma_client.list_collections()]:
                    self.collection = self.chroma_client.get_collection(name=ConfigConstants.COLLECTION_NAME)
                    logging.info("Loaded existing ChromaDB collection")
                else:
                    self.collection = self.chroma_client.create_collection(name=ConfigConstants.COLLECTION_NAME)
                    logging.info("Created new ChromaDB collection")

        except Exception as e:
            logging.error(f"Error initializing vector store: {str(e)}")
            self.chroma_client = None
            self.collection = None

    def _create_index(self, file_path: str = ConfigConstants.VECTOR_DB_TEXT_FILE) -> None:
        """
        Create vector index from a text file by chunking and embedding the content.

        This method reads content from the specified file, splits it into chunks,
        generates embeddings for each chunk, and stores them in the vector database.

        Args:
            file_path (str, optional): Path to the text file to index.
        """
        try:
            content = self.get_or_create_initial_file(file_path)

            chunks = self._split_text(content, chunk_size=ConfigConstants.CHUNK_SIZE, overlap=ConfigConstants.OVERLAP_SIZE)

            if self.collection and chunks:
                embeddings = self.llm_manager.embed_many(chunks)

                for i, (chunk, embedding) in enumerate(zip(chunks, embeddings)):
                    doc_id = f"doc_{i}"
                    self.collection.add(documents=[chunk], embeddings=[embedding], ids=[doc_id], metadatas=[{"source": "initial_knowledge", "chunk_id": i}])

                logging.info(f"Vector store created with {len(chunks)} chunks")
            else:
                logging.warning("No content to index or collection not available")

        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")
            raise

    def get_or_create_initial_file(self, file_path: str = ConfigConstants.VECTOR_DB_TEXT_FILE) -> str:
        """Ensure the initial knowledge file exists and return its content."""
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        if not os.path.exists(file_path):
            with open(file_path, "w", encoding="utf-8") as f:
                f.write("Conversation History:\n")
                f.write("This is the initial knowledge base for the chatbot.\n")
                f.write("The chatbot can answer questions and hold conversations.\n")
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read()
        return content

    def _split_text(self, text: str, chunk_size: int = ConfigConstants.CHUNK_SIZE, overlap: int = ConfigConstants.OVERLAP_SIZE) -> List[str]:
        """
        Split text into overlapping chunks for vector indexing.

        Args:
            text (str): Text to split.
            chunk_size (int): Max chunk size.
            overlap (int): Overlap between chunks.

        Returns:
            List[str]: List of non-empty text chunks.
        """
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

    def _save_to_vectordb(self, doc_content: str, metadata: dict) -> None:
        """
        Save document content after embedding to the vector database with metadata.

        Args:
            doc_content (str): The document content to save.
            metadata (dict): Metadata associated with the document, including
                           session_id, type, and source information.
        """
        try:
            if not self.collection:
                logging.warning("Vector store not initialized, skipping save")
                return

            with open(ConfigConstants.VECTOR_DB_TEXT_FILE, "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")

            session_id = metadata.get("session_id", "default")
            doc_id = f"session_{session_id}_{hash(doc_content)}"
            doc_embedding = self.llm_manager.embed(doc_content)

            # Add to vector store
            self.collection.add(documents=[doc_content], embeddings=[doc_embedding], ids=[doc_id], metadatas=[metadata])
            logging.info(f"Saved conversation to vector DB for session {session_id}")

        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def _retrieve_context(self, query: str) -> str:
        """
        Retrieve relevant context from the vector database based on similarity search.

        This method generates embeddings for the query and searches the vector
        database for the most similar documents to provide context for responses.

        Args:
            query (str): The user query to search for relevant context.

        Returns:
            str: Concatenated relevant documents as context, or error message
                 if retrieval fails or no documents are found.
        """
        try:
            if not self.collection:
                return "No vector store available"

            query_embedding = self.llm_manager.embed(query)

            results = self.collection.query(query_embeddings=[query_embedding], n_results=ConfigConstants.RETRIEVE_CONTEXT_N_RESULTS)

            if "documents" in results and results["documents"]:
                context_docs = [doc for docs in results["documents"] for doc in docs]
                context = "\n\n".join(context_docs)
                logging.info(f"Retrieved {len(context_docs)} documents for context")
                return context
            else:
                logging.info("No documents found in similarity search")
                return "No relevant context found in vector database"

        except Exception as e:
            logging.error(f"Error retrieving context: {str(e)}")
            return f"Error retrieving context: {str(e)}"
