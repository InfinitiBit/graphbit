"""
Configuration constants for the Research Paper Summarizer Agent.

This module contains all configuration constants used throughout the application,
including API keys, model settings, and application parameters.
"""

import os
from dotenv import load_dotenv

load_dotenv()


class ConfigConstants:
    """Configuration constants for the Research Paper Summarizer application."""
    
    # API Keys
    OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
    
    # LLM Configuration
    LLM_MODEL = "gpt-4o"
    LLM_TEMPERATURE = 0.1
    LLM_MAX_TOKENS = 2048
    
    # Embedding Configuration
    EMBEDDING_MODEL = "text-embedding-3-small"
    
    # Cache Configuration
    CACHE_DIR = "examples/research-paper-summarizer-agent/cache"
    
    # PDF Processing Configuration
    MAX_CHUNK_WORDS = 250  # Optimized for faster processing while maintaining quality
    MIN_CHUNK_LENGTH = 50   # Increased minimum to ensure meaningful chunks
    MAX_SECTION_LENGTH = 15000  # Optimized to reduce LLM processing time

    # Performance Configuration
    MAX_PARALLEL_WORKERS = 4  # Maximum parallel workers for summarization
    EMBEDDING_BATCH_SIZE = 25  # Batch size for embedding generation
    SUMMARIZATION_TIMEOUT = 30  # Timeout for individual section summarization (seconds)
    
    # Search Configuration
    TOP_K_RESULTS = 4
    
    # Section Headers for PDF parsing
    SECTION_HEADERS = [
        r'abstract', r'introduction', r'background', r'related work',
        r'methods', r'methodology', r'experiment', r'results', r'discussion', 
        r'conclusion', r'references', r'acknowledgments'
    ]
    
    # Logging Configuration
    LOG_DIR = "logs"
    LOG_FILE = "paper_summarizer.log"
    LOG_FORMAT = "%(asctime)s - %(levelname)s - %(message)s"
    
    # Server Configuration
    SERVER_HOST = "localhost"
    SERVER_PORT = 8000
    
    # File Upload Configuration
    MAX_FILE_SIZE = 50 * 1024 * 1024  # 50MB
    ALLOWED_EXTENSIONS = [".pdf"]
    
    # Session Configuration
    SESSION_TIMEOUT = 3600  # 1 hour in seconds
