# FAISS Integration with Graphbit

## Overview

This guide explains how to use Graphbit to generate embeddings and perform similarity search using FAISS (Facebook AI Similarity Search), a library for efficient similarity search and clustering of dense vectors. You can use FAISS to store, index, and search high-dimensional vectors for semantic search and retrieval-augmented generation.

---

## Prerequisites

- **OpenAI API Key** (or another supported embedding provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `faiss-cpu`, `numpy`, `graphbit`, and optionally installed.


---

## Step 1: Initialize Graphbit

Set up Graphbit:

```python
from graphbit import EmbeddingConfig, EmbeddingClient

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key="openai_api_key",
    )
)
```

---
---

## Step 2: Generate Embeddings

Generate embeddings for your texts:

```python
import numpy as np

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "FAISS is a library for efficient similarity search and clustering of dense vectors.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeddings = embedding_client.embed_many(texts)
embeddings = np.array(embeddings).astype('float32')
```

---

## Step 3: Create FAISS Index

Create a FAISS index for similarity search:

```python
import faiss

dimension = embeddings.shape[1]
index = faiss.IndexFlatIP(dimension)
```

---

## Step 4: Add Embeddings to FAISS Index

Add the generated embeddings to the FAISS index:

```python
index.add(embeddings)
```

---

## Step 5: Vector Search (Similarity Search)

Embed your query and search for similar vectors in FAISS:

```python
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)
query_embedding = np.array(query_embedding).astype('float32').reshape(1, -1)

scores, indices = index.search(query_embedding, k=3)
```

---

## Full Example

```python
import faiss
import numpy as np
from graphbit import EmbeddingConfig, EmbeddingClient

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key="openai_api_key",
    )
)

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "FAISS is a library for efficient similarity search and clustering of dense vectors.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeddings = embedding_client.embed_many(texts)
embeddings = np.array(embeddings).astype('float32')

dimension = embeddings.shape[1]
index = faiss.IndexFlatIP(dimension)
index.add(embeddings)

query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)
query_embedding = np.array(query_embedding).astype('float32').reshape(1, -1)

scores, indices = index.search(query_embedding, k=3)

for idx, score in zip(indices[0], scores[0]):
    print(f"ID: doc_{idx}\nScore: {score:.4f}\nText: {texts[idx]}\n---")
```

---

**This integration enables you to leverage Graphbit's embedding capabilities with FAISS for efficient, scalable semantic search and retrieval workflows.** 
