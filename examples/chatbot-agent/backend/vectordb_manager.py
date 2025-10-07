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

from .const import COLLECTIONS_TEXT_FILES, ConfigConstants, VectorDB
from .embedding_manager import EmbeddingManager

load_dotenv()

os.makedirs("logs", exist_ok=True)
logging.basicConfig(filename="logs/chatbot.log", filemode="a", format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO)


class VectorDBManager:
    """
    VectorDBManager handles the initialization and management of the vector database.

    This class manages ChromaDB operations including collection creation, document
    indexing, similarity search, and conversation history storage for the chatbot.
    """

    def __init__(self, index_name: str, embedding_manager: EmbeddingManager):
        """
        Initialize the VectorDBManager with the specified index name and Embedding manager.

        Args:
            index_name (str, optional): Name of the vector database index to use.
            embedding_manager (Optional[EmbeddingManager], optional): Embedding manager instance for
                                                        generating embeddings.
        """
        logging.info("Initializing VectorDBManager")
        self.embedding_manager = embedding_manager

        # Initialize ChromaDB
        self.index_name: str = index_name
        self.chroma_client: Optional[Client] = None
        self.chat_history_collection = None
        self.personal_info_collection = None

        self._init_vectorstores()

    def _init_vectorstores(self) -> None:
        """
        Initialize ChromaDB client and create or load the chatbot memory collection.

        This method sets up the persistent ChromaDB client and either loads an existing
        collection or creates a new one named 'chatbot_memory'.
        """
        try:
            self.chroma_client = Client(Settings(persist_directory=self.index_name, is_persistent=True))
            if self.chroma_client is not None:
                if VectorDB.HISTORY_COLLECTION in [c.name for c in self.chroma_client.list_collections()]:
                    self.chat_history_collection = self.chroma_client.get_collection(name=VectorDB.HISTORY_COLLECTION)
                    logging.info("Loaded existing ChromaDB collection")
                else:
                    self.chat_history_collection = self.chroma_client.create_collection(name=VectorDB.HISTORY_COLLECTION)
                    logging.info("Created new ChromaDB collection")

                if VectorDB.PERSONAL_INFO_COLLECTION in [c.name for c in self.chroma_client.list_collections()]:
                    self.personal_info_collection = self.chroma_client.get_collection(name=VectorDB.PERSONAL_INFO_COLLECTION)
                    logging.info("Loaded existing ChromaDB collection")
                else:
                    self.personal_info_collection = self.chroma_client.create_collection(name=VectorDB.PERSONAL_INFO_COLLECTION)
                    logging.info("Created new ChromaDB collection")
            else:
                logging.error("Failed to initialize ChromaDB client")

        except Exception as e:
            logging.error(f"Error initializing vector store: {str(e)}")
            self.chroma_client = None
            self.chat_history_collection = None

    def _get_collection(self, collection: VectorDB) -> Optional[VectorDB]:
        if collection == VectorDB.HISTORY_COLLECTION:
            return self.chat_history_collection
        elif collection == VectorDB.PERSONAL_INFO_COLLECTION:
            return self.personal_info_collection
        else:
            return None

    def _create_index(self, collection: VectorDB, file_path: str = None) -> None:
        """
        Create vector index from a text file by chunking and embedding the content.

        This method reads content from the specified file, splits it into chunks,
        generates embeddings for each chunk, and stores them in the vector database.

        Args:
            file_path (str, optional): Path to the text file to index.
        """
        try:
            if not file_path:
                file_path = COLLECTIONS_TEXT_FILES[collection]
            initial_data = self.get_or_create_initial_file(file_path)
            chunks = self.embedding_manager.sentence_splitter(initial_data, chunk_size=ConfigConstants.CHUNK_SIZE, overlap=ConfigConstants.OVERLAP_SIZE)

            collection = self._get_collection(collection)
            print(f"chunks: {chunks}")
            print(f"collection: {collection}")

            if collection and chunks:
                embeddings = self.embedding_manager.embed_many(chunks)
                print(f"embeddings: {embeddings}")
                doc_ids = [f"doc_{i}" for i in range(len(chunks))]
                print(f"doc_ids: {doc_ids}")
                metadatas = [{"source": "initial_knowledge", "chunk_id": i} for i in range(len(chunks))]
                print(f"metadatas: {metadatas}")
                collection.add(documents=chunks, embeddings=embeddings, ids=doc_ids, metadatas=metadatas)
                logging.info(f"Vectorstore created with {len(chunks)} chunks for {collection}")
                print(f"Vectorstore created with {len(chunks)} chunks for {collection}")
            else:
                logging.warning("No content to index or collection not available for chat history")

        except Exception as e:
            logging.error(f"Error creating vector index: {str(e)}")
            raise

    def get_or_create_initial_file(self, file_path: str = ConfigConstants.VECTOR_DB_CHAT_HISTORY_TEXT_FILE) -> str:
        """Ensure the initial knowledge file exists and return its content."""
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        if not os.path.exists(file_path):
            with open(file_path, "w", encoding="utf-8") as f:
                f.write("Conversation History:\n")
                f.write("This is the initial knowledge base for the chatbot.\n")
                f.write("The chatbot can answer questions and hold conversations.\n")
        with open(file_path, "r", encoding="utf-8") as f:
            chat_history = f.read()
        return chat_history

    def _save_to_vectordb(self, doc_content: str, metadata: dict, collection: VectorDB = VectorDB.HISTORY_COLLECTION) -> None:
        """
        Save document content after embedding to the vector database with metadata.

        Args:
            doc_content (str): The document content to save.
            metadata (dict): Metadata associated with the document, including
                           session_id, type, and source information.
            collection (str): The collection to save the document to.
        """
        try:
            collection = self._get_collection(collection)
            if COLLECTIONS_TEXT_FILES[collection] and not collection:
                logging.warning(f"Vectorstore collection: {collection} not initialized, skipping save")
                return

            with open(COLLECTIONS_TEXT_FILES[collection], "a", encoding="utf-8") as f:
                f.write(f"\n{doc_content}\n")

            session_id = metadata.get("session_id", "default")
            doc_id = f"session_{session_id}_{hash(doc_content)}"
            doc_embedding = self.embedding_manager.embed(doc_content)

            # Add to vector store
            collection.add(documents=[doc_content], embeddings=[doc_embedding], ids=[doc_id], metadatas=[metadata])
            logging.info(f"Saved conversation to vector DB collection: {collection} for session {session_id}")

        except Exception as e:
            logging.error(f"Error saving to vector DB: {str(e)}")

    def _retrieve_context(self, query: str, collection: str = VectorDB.HISTORY_COLLECTION) -> List[str]:
        """
        Retrieve relevant context from the vector database based on similarity search.

        This method generates embeddings for the query and searches the vector
        database for the most similar documents to provide context for responses.

        Args:
            query (str): The user query to search for relevant context.
            collection (str): The collection to retrieve the context from.

        Returns:
            str: Concatenated relevant documents as context, or error message
                 if retrieval fails or no documents are found.
        """
        try:
            collection = self._get_collection(collection)
            if not collection:
                return "No vector store available"

            query_embedding = self.embedding_manager.embed(query)

            results = collection.query(query_embeddings=[query_embedding], n_results=ConfigConstants.RETRIEVE_CONTEXT_N_RESULTS)

            if "documents" in results and results["documents"]:
                context_docs = [doc for docs in results["documents"] for doc in docs]
                context = "\n\n".join(context_docs)
                logging.info(f"Retrieved {len(context_docs)} documents for {collection}")
                return context
            else:
                logging.info("No documents found in similarity search")
                return "No relevant context found in vector database"

        except Exception as e:
            logging.error(f"Error retrieving context: {str(e)}")
            return f"Error retrieving context: {str(e)}"
