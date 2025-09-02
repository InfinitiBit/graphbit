# MongoDB Integration with Graphbit


## Overview

This guideline explains how to use MongoDB as both a general-purpose and a vector database within the Graphbit ecosystem, leveraging OpenAI embeddings. You will learn how to connect, store, and search data and vectors.

---

## Prerequisites

- **MongoDB** running locally or in the cloud (e.g., MongoDB Atlas)
- **Python environment** with `pymongo` and `graphbit` installed:
  ```bash
  pip install pymongo graphbit
  ```
- **OpenAI API Key** for embeddings
- **Environment variable** for your OpenAI API key:
  ```bash
  export OPENAI_API_KEY=sk-...
  ```

---

## Step 1: Connect to MongoDB

You can connect to either a local MongoDB instance or a cloud-hosted MongoDB Atlas cluster by changing the `MONGO_URI`. Here’s how to do both:

```python
from pymongo import MongoClient
import os

# For local MongoDB
MONGO_URI = "mongodb://localhost:27017"

# For MongoDB Atlas (replace <username>, <password>, and <cluster-url> with your details)
# MONGO_URI = "mongodb+srv://<username>:<password>@<cluster-url>/"

try:
    client = MongoClient(MONGO_URI, serverSelectionTimeoutMS=5000)
    client.server_info()  # Force connection
except Exception as e:
    print(f"Failed to connect to MongoDB: {e}")
    exit(1)

db = client["graphbit_demo"]
```

> **Tip:** To use MongoDB Atlas, simply comment out the local URI and uncomment the Atlas URI, filling in your credentials.

---

## Step 2: General-purpose CRUD Operations

```python
general_collection = db["general_data"]

# CREATE: Insert a document
doc = {"name": "Alice", "role": "engineer", "age": 30}
insert_result = general_collection.insert_one(doc)
print(f"Inserted document ID: {insert_result.inserted_id}")

# READ: Find a single document
found_doc = general_collection.find_one({"name": "Alice"})
print(f"Found document: {found_doc}")

# READ: Find all documents (returns a cursor)
all_docs = list(general_collection.find({}))
print(f"All documents: {all_docs}")

# UPDATE: Update a document
general_collection.update_one({"name": "Alice"}, {"$set": {"age": 31}})
updated_doc = general_collection.find_one({"name": "Alice"})
print(f"Updated document: {updated_doc}")

# DELETE: Delete a document
general_collection.delete_one({"name": "Alice"})
print(f"Document deleted. Remaining: {list(general_collection.find({}))}")

```

---

## Step 3: Store and Search Vectors with OpenAI Embeddings

### 3.1. Generate and Store an Embedding

```python
from graphbit import EmbeddingConfig, EmbeddingClient

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
if not OPENAI_API_KEY:
    raise ValueError("Please set the OPENAI_API_KEY environment variable.")

embedding_config = EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = EmbeddingClient(embedding_config)

text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)

vector_collection = db["vector_data"]
vector_doc = {"item_id": "item123", "embedding": embedding, "metadata": {"category": "test"}}
vector_collection.insert_one(vector_doc)
```

### 3.2. Vector Search Example

```python
query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)

results = vector_collection.find({})
best_score = -1
best_doc = None
for doc in results:
    score = EmbeddingClient.similarity(query_embedding, doc["embedding"])
    if score > best_score:
        best_score = score
        best_doc = doc
if best_doc is not None:
    print(f"Most similar document: {best_doc['item_id']} with score {best_score:.4f}")
else:
    print("No documents found in vector collection.")
```

---

## Step 4: Batch Embedding Example

```python
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)

docs = [
    {"item_id": f"batch_{idx}", "embedding": emb, "metadata": {"text": text}}
    for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
]
vector_collection.insert_many(docs)
print(f"Inserted {len(batch_texts)} documents with OpenAI embeddings.")
```

---

## Full Example

```python
import os
from pymongo import MongoClient
from graphbit import EmbeddingConfig, EmbeddingClient

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
MONGO_URI = "mongodb://localhost:27017"
client = MongoClient(MONGO_URI)
db = client["graphbit_demo"]

# General CRUD
col = db["general_data"]
col.insert_one({"name": "Alice", "role": "engineer", "age": 30})
print(col.find_one({"name": "Alice"}))
col.update_one({"name": "Alice"}, {"$set": {"age": 31}})
print(col.find_one({"name": "Alice"}))
col.delete_one({"name": "Alice"})

# Vector storage and search
embedding_config = EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = EmbeddingClient(embedding_config)
text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)
vec_col = db["vector_data"]
vec_col.insert_one({"item_id": "item123", "embedding": embedding})

query_embedding = embedding_client.embed("Find documents related to vector search.")
best_doc = max(vec_col.find({}), key=lambda doc: EmbeddingClient.similarity(query_embedding, doc["embedding"]), default=None)
if best_doc:
    print(f"Most similar document: {best_doc['item_id']}")

# Batch insert
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)
vec_col.insert_many([
    {"item_id": f"batch_{i}", "embedding": emb, "metadata": {"text": text}}
    for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
])
```

---

**This connector pattern enables you to use MongoDB as both a general-purpose and vector database in your AI workflows, orchestrated by Graphbit.** 
