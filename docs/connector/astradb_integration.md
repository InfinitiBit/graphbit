# AstraDB Integration with Graphbit

## Overview

This guide explains how to integrate DataStax AstraDB with Graphbit for vector storage and similarity search. AstraDB is a cloud-native, multi-cloud database-as-a-service built on Apache Cassandra, with native vector search capabilities perfect for AI applications.

---

## Prerequisites

- **AstraDB account** with a database created ([Get started here](https://astra.datastax.com))
- **Application Token** with appropriate permissions
- **Python environment** with required packages:
  ```bash
  pip install astrapy graphbit
  ```
- **OpenAI API Key** (or another supported embedding provider)
- **Environment variables** configured:
  ```bash
  export OPENAI_API_KEY="your_openai_api_key_here"
  export ASTRA_DB_APPLICATION_TOKEN="your_astra_token_here"
  export ASTRA_DB_API_ENDPOINT="your_astra_db_api_endpoint"
  ```

---

## Step 1: Connect to AstraDB

You can connect to AstraDB using your Application Token and API Endpoint. Here's how to connect:

```python
from astrapy import DataAPIClient
import os

# AstraDB connection details
ASTRA_DB_APPLICATION_TOKEN = os.getenv("ASTRA_DB_APPLICATION_TOKEN")
ASTRA_DB_API_ENDPOINT = os.getenv("ASTRA_DB_API_ENDPOINT")

try:
    client = DataAPIClient(ASTRA_DB_APPLICATION_TOKEN)
    astra_db = client.get_database_by_api_endpoint(ASTRA_DB_API_ENDPOINT)
    # Test connection by listing collections
    collections = astra_db.list_collection_names()
    print("Connected to AstraDB successfully!")
except Exception as e:
    print(f"Failed to connect to AstraDB: {e}")
    exit(1)
```

> **Tip:** You can get your Application Token and API Endpoint from your AstraDB dashboard at [astra.datastax.com](https://astra.datastax.com).

---



## Step 2: Generate and Store Embeddings

### 2.1 Configure Embedding Model & Create Vector Collection

```python
from graphbit import EmbeddingConfig, EmbeddingClient

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
if not OPENAI_API_KEY:
    raise ValueError("Please set the OPENAI_API_KEY environment variable.")

# Create embedding client
embedding_config = EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = EmbeddingClient(embedding_config)

# Create vector collection in AstraDB
vector_collection = astra_db.create_collection(
    name="vector_data",
    definition={"vector": {"dimension": 1536, "metric": "cosine"}}
)

```

### 2.2 Insert Single Embedded Document

```python
text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)

vector_doc = {
    "_id": "item123", 
    "$vector": embedding, 
    "metadata": {"category": "test"}
}
vector_collection.insert_one(vector_doc)
print("Inserted single document into AstraDB vector collection.")

```

### 2.3 Insert Batch Embedded Documents

```python
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)

docs = [
    {
        "_id": f"batch_{idx}", 
        "$vector": emb, 
        "metadata": {"text": text}
    }
    for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
]
vector_collection.insert_many(docs)
print(f"Inserted {len(batch_texts)} documents into AstraDB vector collection.")
```

---

## Step 3: Search Vectors

```python
query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)

results = list(vector_collection.find(
    {},
    sort={"$vector": query_embedding},
    limit=5
))

best_doc = None
best_similarity = -1
for doc in results:
    similarity = doc.get("$similarity", 0)
    if similarity > best_similarity:
        best_similarity = similarity
        best_doc = doc

if best_doc:
    print(f"Most similar document: {best_doc['_id']} with similarity {best_similarity:.4f}")
else:
    print("No documents found in vector collection.")
```

---
