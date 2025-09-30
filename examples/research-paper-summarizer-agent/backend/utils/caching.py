"""
Caching utilities for the Research Paper Summarizer Agent.

This module provides functions for caching processed PDF data including
summaries, chunks, embeddings, and FAISS indices to improve performance
on repeated access to the same documents.
"""

import hashlib
import json
import os
from typing import Optional

import faiss

from ..constant import ConfigConstants

CACHE_DIR = ConfigConstants.CACHE_DIR


def hash_pdf(pdf_path: str) -> str:
    """
    Generate a SHA256 hash of a PDF file for caching purposes.

    Args:
        pdf_path (str): Path to the PDF file.

    Returns:
        str: SHA256 hash of the file.
    """
    hasher = hashlib.sha256()
    with open(pdf_path, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def save_to_cache(hash_id: str, summaries: dict, chunk_dict: dict, chunk_titles: list, faiss_index) -> None:
    """
    Save processed PDF data to cache.

    Args:
        hash_id (str): Unique hash identifier for the PDF.
        summaries (dict): Section summaries.
        chunk_dict (dict): Text chunks organized by section.
        chunk_titles (list): List of chunk titles.
        faiss_index: FAISS index for similarity search.
    """
    folder = os.path.join(CACHE_DIR, hash_id)
    os.makedirs(folder, exist_ok=True)

    with open(os.path.join(folder, "summaries.json"), "w", encoding="utf-8") as f:
        json.dump(summaries, f, indent=2, ensure_ascii=False)

    with open(os.path.join(folder, "chunk_dict.json"), "w", encoding="utf-8") as f:
        json.dump(chunk_dict, f, indent=2, ensure_ascii=False)

    with open(os.path.join(folder, "chunk_titles.json"), "w", encoding="utf-8") as f:
        json.dump(chunk_titles, f, indent=2, ensure_ascii=False)

    faiss.write_index(faiss_index, os.path.join(folder, "faiss.index"))


def load_from_cache(hash_id: str) -> tuple:
    """
    Load processed PDF data from cache.

    Args:
        hash_id (str): Unique hash identifier for the PDF.

    Returns:
        tuple: (summaries, chunk_dict, chunk_titles, faiss_index)
    """
    folder = os.path.join(CACHE_DIR, hash_id)

    with open(os.path.join(folder, "summaries.json"), "r", encoding="utf-8") as f:
        summaries = json.load(f)

    with open(os.path.join(folder, "chunk_dict.json"), "r", encoding="utf-8") as f:
        chunk_dict = json.load(f)

    with open(os.path.join(folder, "chunk_titles.json"), "r", encoding="utf-8") as f:
        chunk_titles = json.load(f)

    faiss_index = faiss.read_index(os.path.join(folder, "faiss.index"))

    return summaries, chunk_dict, chunk_titles, faiss_index


def cache_exists(hash_id: str) -> bool:
    """
    Check if cache exists for a given hash ID.

    Args:
        hash_id (str): Unique hash identifier for the PDF.

    Returns:
        bool: True if cache exists, False otherwise.
    """
    folder = os.path.join(CACHE_DIR, hash_id)
    required_files = ["summaries.json", "chunk_dict.json", "chunk_titles.json", "faiss.index"]

    return os.path.exists(folder) and all(os.path.exists(os.path.join(folder, file)) for file in required_files)


def clear_cache(hash_id: Optional[str] = None) -> bool:
    """
    Clear cache for a specific hash ID or all cache.

    Args:
        hash_id (str, optional): Specific hash ID to clear. If None, clears all cache.

    Returns:
        bool: True if successful, False otherwise.
    """
    try:
        if hash_id:
            folder = os.path.join(CACHE_DIR, hash_id)
            if os.path.exists(folder):
                import shutil

                shutil.rmtree(folder)
                return True
        else:
            if os.path.exists(CACHE_DIR):
                import shutil

                shutil.rmtree(CACHE_DIR)
                os.makedirs(CACHE_DIR, exist_ok=True)
                return True
        return False
    except Exception:
        return False
