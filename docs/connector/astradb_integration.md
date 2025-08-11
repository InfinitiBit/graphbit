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

## Step 2: General-purpose CRUD Operations

```python
# Create a general collection (non-vector)
general_collection = astra_db.create_collection("general_data")

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
import graphbit

graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
if not OPENAI_API_KEY:
    raise ValueError("Please set the OPENAI_API_KEY environment variable.")

embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = graphbit.EmbeddingClient(embedding_config)

text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)

# Create vector collection
vector_collection = astra_db.create_collection(
    name="vector_data",
    definition={"vector": {"dimension": 1536, "metric": "cosine"}}
)

vector_doc = {
    "_id": "item123", 
    "$vector": embedding, 
    "metadata": {"category": "test"}
}
vector_collection.insert_one(vector_doc)
```

### 3.2. Vector Search Example

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

if best_doc is not None:
    print(f"Most similar document: {best_doc['_id']} with similarity {best_similarity:.4f}")
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
    {
        "_id": f"batch_{idx}", 
        "$vector": emb, 
        "metadata": {"text": text}
    }
    for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
]
vector_collection.insert_many(docs)
print(f"Inserted {len(batch_texts)} documents with OpenAI embeddings.")
```
---

## Full Example

```python
import os
from astrapy import DataAPIClient
import graphbit

graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
ASTRA_DB_APPLICATION_TOKEN = os.getenv("ASTRA_DB_APPLICATION_TOKEN")
ASTRA_DB_API_ENDPOINT = os.getenv("ASTRA_DB_API_ENDPOINT")

client = DataAPIClient(ASTRA_DB_APPLICATION_TOKEN)
astra_db = client.get_database_by_api_endpoint(ASTRA_DB_API_ENDPOINT)

# General CRUD
col = astra_db.create_collection("general_data")
col.insert_one({"name": "Alice", "role": "engineer", "age": 30})
print(col.find_one({"name": "Alice"}))
col.update_one({"name": "Alice"}, {"$set": {"age": 31}})
print(col.find_one({"name": "Alice"}))
col.delete_one({"name": "Alice"})

# Vector storage and search
embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = graphbit.EmbeddingClient(embedding_config)
text = "This is a sample document for vector search."
embedding = embedding_client.embed(text)
vec_col = astra_db.create_collection(
    name="vector_data",
    definition={"vector": {"dimension": 1536, "metric": "cosine"}}
)
vec_col.insert_one({"_id": "item123", "$vector": embedding})

query_embedding = embedding_client.embed("Find documents related to vector search.")
results = list(vec_col.find({}, sort={"$vector": query_embedding}, limit=5))
best_doc = max(results, key=lambda doc: doc.get("$similarity", 0), default=None)
if best_doc:
    print(f"Most similar document: {best_doc['_id']}")

# Batch insert
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)
vec_col.insert_many([
    {"_id": f"batch_{i}", "$vector": emb, "metadata": {"text": text}}
    for i, (text, emb) in enumerate(zip(batch_texts, batch_embeddings))
])
```

---

**This connector pattern enables you to use AstraDB as both a general-purpose and vector database in your AI workflows, orchestrated by Graphbit.**

---

## Environment Setup

Create a `.env` file with your credentials:

```bash
# AstraDB Configuration
ASTRA_DB_APPLICATION_TOKEN=AstraCS:your_token_here
ASTRA_DB_API_ENDPOINT=https://your_database_id-your_region.apps.astra.datastax.com

# OpenAI Configuration
OPENAI_API_KEY=sk-your_openai_key_here
```

---

## Best Practices

1. **Connection Management**:
   - Reuse AstraDB connections across requests
   - Implement connection pooling for high-throughput applications
   - Handle connection timeouts gracefully

2. **Vector Dimensions**:
   - Ensure vector dimensions match your embedding model
   - Common dimensions: 1536 (OpenAI), 768 (sentence-transformers), 384 (MiniLM)

3. **Indexing**:
   - Create vector indexes for better search performance
   - Consider your similarity metric (cosine, euclidean, dot_product)

4. **Batch Operations**:
   - Use `insert_many()` for bulk insertions
   - Batch vector searches when possible

5. **Error Handling**:
   - Implement retry logic for transient failures
   - Monitor rate limits and adjust accordingly

6. **Security**:
   - Never commit tokens to version control
   - Use environment variables or secure secret management
   - Rotate tokens regularly

---

## Troubleshooting

### Common Issues

1. **Authentication Errors**:
   ```python
   # Verify token permissions
   print("Token:", os.getenv("ASTRA_DB_APPLICATION_TOKEN")[:20] + "...")
   ```

2. **Dimension Mismatch**:
   ```python
   # Check embedding dimensions
   test_embedding = embedding_client.embed("test")
   print(f"Embedding dimension: {len(test_embedding)}")
   ```

3. **Collection Not Found**:
   ```python
   # List available collections
   collections = astra_db.list_collection_names()
   print("Available collections:", collections)
   ```

4. **Rate Limiting**:
   ```python
   import time
   # Add delays between requests
   time.sleep(0.1)
   ```

---

## Additional Resources

- [AstraDB Documentation](https://docs.datastax.com/en/astra-serverless/docs/)
- [AstraPy GitHub Repository](https://github.com/datastax/astrapy)
- [Vector Search in AstraDB](https://docs.datastax.com/en/astra-serverless/docs/vector-search/overview.html)
- [DataStax Cassandra Driver](https://docs.datastax.com/en/developer/python-driver/latest/)
