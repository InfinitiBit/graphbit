"""The vector database connection."""

import os

import faiss
import numpy as np
from dotenv import load_dotenv

from graphbit import EmbeddingClient as gb_etc
from graphbit import EmbeddingConfig as gb_ecg

load_dotenv()

embedding_config = gb_ecg.openai(api_key=os.getenv("OPENAI_API_KEY"), model="text-embedding-3-small")
embedding_client = gb_etc(embedding_config)


def embed_chunks(chunk_dict):
    """Embed the chunks."""
    chunk_titles = []
    chunk_vectors = []
    for title, chunk_list in chunk_dict.items():
        for i, chunk in enumerate(chunk_list):
            key = f"{title} - Part {i+1}" if len(chunk_list) > 1 else title
            chunk_titles.append(key)
            vec = embedding_client.embed_many(chunk)
            chunk_vectors.append(vec)
    return chunk_titles, np.array(chunk_vectors)


def create_faiss_index(vectors):
    """Create the faiss index."""
    dim = vectors.shape[1]
    index = faiss.IndexFlatL2(dim)
    index.add(vectors)
    return index


def search_faiss_index(index, query, chunk_titles, chunk_dict):
    """Search the faiss index."""
    query_vec = embedding_client.embed(query)
    D, Index = index.search(np.array([query_vec]), k=4)  # top 4 results
    results = []
    all_chunks = []
    for title, chunk_list in chunk_dict.items():
        for i, chunk in enumerate(chunk_list):
            key = f"{title} - Part {i+1}" if len(chunk_list) > 1 else title
            all_chunks.append((key, chunk))
    for idx in Index[0]:
        results.append(all_chunks[idx])
    return results
