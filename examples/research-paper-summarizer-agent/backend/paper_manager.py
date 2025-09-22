"""
Research Paper Manager module for GraphBit-based paper analysis.

This module provides a comprehensive research paper analysis implementation using GraphBit's
workflow system, with vector database integration for context retrieval and
paper summarization capabilities.
"""

import json
import logging
import os
from typing import Any, Dict, List, Optional
from collections import defaultdict

from dotenv import load_dotenv
from fastapi import WebSocket

import graphbit
from graphbit import LlmConfig, LlmClient, EmbeddingConfig, EmbeddingClient

from .const import ConfigConstants
from .summarizer import summarize_pdf_sections_parallel, chunk_text, chunk_text_with_context, answer_question
from .faiss_store import embed_chunks_batch, create_faiss_index, search_faiss_index
from .utils.caching import save_to_cache, load_from_cache, hash_pdf

load_dotenv()

os.makedirs(ConfigConstants.LOG_DIR, exist_ok=True)
logging.basicConfig(
    filename=os.path.join(ConfigConstants.LOG_DIR, ConfigConstants.LOG_FILE),
    filemode="a",
    format=ConfigConstants.LOG_FORMAT,
    level=logging.INFO
)


class PaperManager:
    """
    PaperManager orchestrates research paper analysis and Q&A functionality.

    This class coordinates between PDF processing, summarization, vector storage,
    and question answering using GraphBit's workflow system.
    """

    def __init__(self, cache_dir: str = ConfigConstants.CACHE_DIR):
        """
        Initialize the PaperManager with necessary configurations.

        Args:
            cache_dir (str, optional): Directory for caching processed papers.
        """
        self.cache_dir = cache_dir

        # Initialize GraphBit
        graphbit.init()

        # Ensure OpenAI API key is present
        openai_api_key = ConfigConstants.OPENAI_API_KEY
        if not openai_api_key:
            raise ValueError("OPENAI_API_KEY environment variable is not set. Please set it in your environment.")

        # Initialize LLM client
        self.llm_config = LlmConfig.openai(openai_api_key, ConfigConstants.LLM_MODEL)
        self.llm_client = LlmClient(self.llm_config)

        # Initialize embedding client
        self.embedding_config = EmbeddingConfig.openai(openai_api_key, ConfigConstants.EMBEDDING_MODEL)
        self.embedding_client = EmbeddingClient(self.embedding_config)

        # Session storage for processed papers
        self.sessions: Dict[str, Dict[str, Any]] = {}

    def process_pdf(self, pdf_path: str) -> tuple[str, Dict[str, str]]:
        """
        Process a PDF file and return session ID and summaries.
        
        Args:
            pdf_path (str): Path to the PDF file.
            
        Returns:
            tuple: (session_id, summaries) where session_id is the hash of the PDF
                   and summaries is a dict of section summaries.
        """
        try:
            # Hash the PDF and check cache
            hash_id = hash_pdf(pdf_path)
            cache_folder = os.path.join(self.cache_dir, hash_id)

            if os.path.exists(cache_folder):
                logging.info(f"Loading cached data for PDF hash: {hash_id}")
                summaries, chunk_dict, chunk_titles, index = load_from_cache(hash_id)
            else:
                logging.info(f"Processing new PDF with hash: {hash_id}")
                # Generate summaries and process sections using parallel processing
                summaries, sections = summarize_pdf_sections_parallel(
                    pdf_path,
                    max_workers=ConfigConstants.MAX_PARALLEL_WORKERS
                )

                # Create chunks for vector storage with enhanced context preservation
                chunk_dict = defaultdict(list)
                for section, content in sections.items():
                    # Use enhanced chunking with section context
                    for chunk in chunk_text_with_context(content, section):
                        chunk_dict[section].append(chunk)

                # Generate embeddings and create FAISS index using batch processing
                chunk_titles, chunk_vectors = embed_chunks_batch(
                    chunk_dict,
                    batch_size=ConfigConstants.EMBEDDING_BATCH_SIZE
                )
                index = create_faiss_index(chunk_vectors)
                
                # Save to cache
                save_to_cache(hash_id, summaries, chunk_dict, chunk_titles, index)

            # Store session data
            self.sessions[hash_id] = {
                "summaries": summaries,
                "chunk_dict": chunk_dict,
                "chunk_titles": chunk_titles,
                "index": index,
            }
            
            logging.info(f"Successfully processed PDF with {len(summaries)} sections")
            return hash_id, summaries

        except Exception as e:
            logging.error(f"Error processing PDF: {str(e)}")
            raise

    def ask_question(self, session_id: str, query: str) -> str:
        """
        Answer a question about a processed paper.
        
        Args:
            session_id (str): Session ID of the processed paper.
            query (str): User's question.
            
        Returns:
            str: Answer to the question.
        """
        try:
            if session_id not in self.sessions:
                raise ValueError(f"Session {session_id} not found. Please upload a PDF first.")

            data = self.sessions[session_id]
            
            # Search for relevant chunks
            results = search_faiss_index(
                data["index"], query, data["chunk_titles"], data["chunk_dict"]
            )
            
            # Combine text from top results
            context = "\n\n".join([chunk for _, chunk in results])
            
            # Generate answer using GraphBit LLM
            answer = answer_question(context, query)
            
            logging.info(f"Successfully answered question for session {session_id}")
            return answer

        except Exception as e:
            logging.error(f"Error answering question: {str(e)}")
            raise

    def get_session_summaries(self, session_id: str) -> Optional[Dict[str, str]]:
        """
        Get summaries for a session.
        
        Args:
            session_id (str): Session ID.
            
        Returns:
            Optional[Dict[str, str]]: Summaries if session exists, None otherwise.
        """
        if session_id in self.sessions:
            return self.sessions[session_id]["summaries"]
        return None

    def list_sessions(self) -> List[str]:
        """
        List all active sessions.
        
        Returns:
            List[str]: List of session IDs.
        """
        return list(self.sessions.keys())

    def clear_session(self, session_id: str) -> bool:
        """
        Clear a specific session.
        
        Args:
            session_id (str): Session ID to clear.
            
        Returns:
            bool: True if session was cleared, False if not found.
        """
        if session_id in self.sessions:
            del self.sessions[session_id]
            logging.info(f"Cleared session {session_id}")
            return True
        return False

    def get_stats(self) -> Dict[str, Any]:
        """
        Get statistics about the paper manager.

        Returns:
            Dict[str, Any]: Statistics including active sessions, cache info, etc.
        """
        # Safely extract model names from config objects
        try:
            llm_model = getattr(self.llm_config, 'model', None)
            if callable(llm_model):
                llm_model = ConfigConstants.LLM_MODEL
            elif not isinstance(llm_model, str):
                llm_model = ConfigConstants.LLM_MODEL
        except Exception:
            llm_model = ConfigConstants.LLM_MODEL

        try:
            embedding_model = getattr(self.embedding_config, 'model', None)
            if callable(embedding_model):
                embedding_model = ConfigConstants.EMBEDDING_MODEL
            elif not isinstance(embedding_model, str):
                embedding_model = ConfigConstants.EMBEDDING_MODEL
        except Exception:
            embedding_model = ConfigConstants.EMBEDDING_MODEL

        return {
            "active_sessions": len(self.sessions),
            "cache_directory": str(self.cache_dir),
            "llm_model": str(llm_model),
            "embedding_model": str(embedding_model),
            "session_ids": list(self.sessions.keys()),
            "total_cached_papers": len([d for d in os.listdir(self.cache_dir) if os.path.isdir(os.path.join(self.cache_dir, d))]) if os.path.exists(self.cache_dir) else 0
        }
