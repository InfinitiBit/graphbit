"""This script demonstrates integration between GraphBit's embedding client and FAISS for semantic search."""

import os

import faiss
import numpy as np

import graphbit

# Initialize GraphBit + OpenAI embedding client
graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

# Prepare documents & get embeddings
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "FAISS is a library for efficient similarity search and clustering of dense vectors.",
    "OpenAI offers tools for LLMs and embeddings.",
]
embeddings = embedding_client.embed_many(texts)
embeddings = np.array(embeddings).astype("float32")

# Create FAISS index (cosine = L2 index)
dimension = embeddings.shape[1]
index = faiss.IndexFlatIP(dimension)
index.add(embeddings)

# Perform search
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)
query_embedding = np.array(query_embedding).astype("float32").reshape(1, -1)

scores, indices = index.search(query_embedding, k=3)

# Print results
for idx, score in zip(indices[0], scores[0]):
    print(f"ID: doc_{idx}\nScore: {score:.4f}\nText: {texts[idx]}\n---")
