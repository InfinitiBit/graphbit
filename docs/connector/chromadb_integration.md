# ChromaDB Integration with Graphbit

## Overview

This guide explains how to use Graphbit to generate embeddings and perform similarity search in ChromaDB, a fast, open-source embedding database for AI applications. You can use ChromaDB to store, index, and search high-dimensional vectors for semantic search and retrieval-augmented generation.

---

## Prerequisites

- **ChromaDB installed and running** (see [ChromaDB documentation](https://docs.trychroma.com/)).
- **OpenAI API Key** (or another supported embedding provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `chromadb`, `graphbit`, and optionally `python-dotenv` installed.
- **.env file** in your project root with the following variables:
  ```env
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Step 1: Connect to ChromaDB and Create Collection

Set up the ChromaDB client and ensure the collection exists:

```python
import os
from chromadb import Client
from chromadb.config import Settings

chromadb_client = Client()

if "chromadb_integration" in [c.name for c in chromadb_client.list_collections()]:
    collection = chromadb_client.get_collection(name="chromadb_integration")
else:
    collection = chromadb_client.create_collection(
        name="chromadb_integration",
        metadata={"hnsw:space": "cosine"}
    )
```

---

## Step 2: Generate Embeddings using Graphbit

Use Graphbit to generate embeddings for your texts:

```python
from graphbit import EmbeddingClient, EmbeddingConfig

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "ChromaDB is a fast, open-source embedding database for AI applications.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeds = embedding_client.embed_many(texts)
```

---

## Step 3: Insert Embeddings into ChromaDB

Insert the generated embeddings into the ChromaDB collection:

```python
collection.add(
    documents=texts,
    embeddings=embeds,
    ids=[f"doc_{i}" for i in range(len(texts))],
    metadatas=[{"source": "initial_knowledge", "chunk_id": i} for i in range(len(texts))]
)
```

---

## Step 4: Vector Search (Similarity Search)

Embed your query and search for similar vectors in ChromaDB:

```python
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

query_result = collection.query(
    query_embeddings=[query_embedding],
    n_results=3,
    include=["documents", "metadatas", "distances"],
)

ids = query_result["ids"][0]
docs = query_result["documents"][0]
distances = query_result["distances"][0]
scores = [1 - d for d in distances]

for doc_id, text, score in zip(ids, docs, scores):
    print(f"ID: {doc_id}\nScore: {score:.4f}\nText: {text}\n---")
```

---

## Full Example

```python
import os
from chromadb import Client
from graphbit import EmbeddingClient, EmbeddingConfig

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

chromadb_client = Client()
if "chromadb_integration" in [c.name for c in chromadb_client.list_collections()]:
    collection = chromadb_client.get_collection(name="chromadb_integration")
else:
    collection = chromadb_client.create_collection(
        name="chromadb_integration",
        metadata={"hnsw:space": "cosine"}
    )

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "ChromaDB is a fast, open-source embedding database for AI applications.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeds = embedding_client.embed_many(texts)

collection.add(
    documents=texts,
    embeddings=embeds,
    ids=[f"doc_{i}" for i in range(len(texts))],
    metadatas=[{"source": "initial_knowledge", "chunk_id": i} for i in range(len(texts))]
)

query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

query_result = collection.query(
    query_embeddings=[query_embedding],
    n_results=3,
    include=["documents", "metadatas", "distances"],
)

ids = query_result["ids"][0]
docs = query_result["documents"][0]
distances = query_result["distances"][0]
scores = [1 - d for d in distances]

for doc_id, text, score in zip(ids, docs, scores):
    print(f"ID: {doc_id}\nScore: {score:.4f}\nText: {text}\n---")
```

---

**This integration enables you to leverage Graphbit's embedding capabilities with ChromaDB for scalable, open-source semantic search and retrieval workflows.** 
