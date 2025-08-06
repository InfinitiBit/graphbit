"""The caching module."""

import hashlib
import json
import os

import faiss

CACHE_DIR = "cache"


def hash_pdf(pdf_path):
    """Hash the PDF."""
    hasher = hashlib.sha256()
    with open(pdf_path, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def save_to_cache(hash_id, summaries, chunk_dict, chunk_titles, faiss_index):
    """Save the data to the cache."""
    folder = os.path.join(CACHE_DIR, hash_id)
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "summaries.json"), "w") as f:
        json.dump(summaries, f)
    with open(os.path.join(folder, "chunk_dict.json"), "w") as f:
        json.dump(chunk_dict, f)
    with open(os.path.join(folder, "chunk_titles.json"), "w") as f:
        json.dump(chunk_titles, f)
    faiss.write_index(faiss_index, os.path.join(folder, "faiss.index"))


def load_from_cache(hash_id):
    """Load the data from the cache."""
    folder = os.path.join(CACHE_DIR, hash_id)
    with open(os.path.join(folder, "summaries.json")) as f:
        summaries = json.load(f)
    with open(os.path.join(folder, "chunk_dict.json")) as f:
        chunk_dict = json.load(f)
    with open(os.path.join(folder, "chunk_titles.json")) as f:
        chunk_titles = json.load(f)
    faiss_index = faiss.read_index(os.path.join(folder, "faiss.index"))
    return summaries, chunk_dict, chunk_titles, faiss_index
