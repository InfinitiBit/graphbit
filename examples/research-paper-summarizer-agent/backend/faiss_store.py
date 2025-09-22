import os

import faiss
import numpy as np
from dotenv import load_dotenv

import graphbit
from graphbit import EmbeddingClient, EmbeddingConfig

from .const import ConfigConstants

load_dotenv()

# Initialize GraphBit and create embedding client
graphbit.init()
openai_api_key = ConfigConstants.OPENAI_API_KEY
if not openai_api_key:
    raise ValueError("OPENAI_API_KEY environment variable is not set")

embedding_config = EmbeddingConfig.openai(openai_api_key, ConfigConstants.EMBEDDING_MODEL)
embedder = EmbeddingClient(embedding_config)


def embed_chunks_batch(chunk_dict, batch_size=20):
    """
    Generate embeddings for text chunks using batch processing for better performance.

    Args:
        chunk_dict: Dictionary of section -> list of chunks
        batch_size: Number of chunks to process in each batch

    Returns:
        Tuple of (chunk_titles, chunk_vectors_array)
    """
    chunk_titles = []
    all_chunks = []

    # Collect all chunks and their titles
    for title, chunk_list in chunk_dict.items():
        for i, chunk in enumerate(chunk_list):
            key = f"{title} - Part {i+1}" if len(chunk_list) > 1 else title
            chunk_titles.append(key)
            all_chunks.append(chunk)

    # Process chunks in batches
    chunk_vectors = []
    for i in range(0, len(all_chunks), batch_size):
        batch = all_chunks[i : i + batch_size]

        # Process batch - for now, still individual calls but structured for future batch API
        batch_vectors = []
        for chunk in batch:
            vec = embedder.embed(chunk)
            batch_vectors.append(vec)

        chunk_vectors.extend(batch_vectors)

    return chunk_titles, np.array(chunk_vectors)


def embed_chunks(chunk_dict):
    """Generate embeddings for text chunks using GraphBit embedding client with batch optimization."""
    return embed_chunks_batch(chunk_dict, batch_size=20)


def create_faiss_index(vectors):
    dim = vectors.shape[1]
    index = faiss.IndexFlatL2(dim)
    index.add(vectors)
    return index


def search_faiss_index(index, query, chunk_titles, chunk_dict, k=ConfigConstants.TOP_K_RESULTS):
    """Search FAISS index for similar chunks using GraphBit embedding client."""
    query_vec = embedder.embed(query)
    D, I = index.search(np.array([query_vec]), k=k)
    results = []
    all_chunks = []
    for title, chunk_list in chunk_dict.items():
        for i, chunk in enumerate(chunk_list):
            key = f"{title} - Part {i+1}" if len(chunk_list) > 1 else title
            all_chunks.append((key, chunk))
    for idx in I[0]:
        if idx < len(all_chunks):  # Safety check
            results.append(all_chunks[idx])
    return results
