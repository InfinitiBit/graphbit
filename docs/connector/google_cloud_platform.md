# Google Cloud Platform Integration with Graphbit

## Overview

This guide explains how to connect Google Cloud Platform (GCP) services to Graphbit, with a focus on AlloyDB for vector storage. This integration enables you to leverage Google's enterprise-grade database within your Graphbit workflows, providing access to AlloyDB's PostgreSQL-compatible vector database with built-in pgvector support.

---

## Prerequisites

- **Google Cloud Project**: Set up a project in [Google Cloud Console](https://console.cloud.google.com/).
- **AlloyDB Instance**: Create an AlloyDB instance in your Google Cloud project (see [AlloyDB documentation](https://cloud.google.com/alloydb/docs)).
- **.env file** in your project root with the following variables:
  ```env
  ALLOYDB_HOST=<your-alloydb-instance-ip>
  ALLOYDB_PORT=<your-alloydb-instance-port> # default 5432
  ALLOYDB_DATABASE=<your_database_name>
  ALLOYDB_USER=<your_username>
  ALLOYDB_PASSWORD=<your_password>
  OPENAI_API_KEY=<your_openai_api_key>
  ```

---

## Step 1: AlloyDB Setup

### Connect to AlloyDB and Ensure Table Exists

```python
import psycopg2
import os

# Connect to AlloyDB 
conn = psycopg2.connect(
    dbname=os.getenv("ALLOYDB_DATABASE"),
    user=os.getenv("ALLOYDB_USER"),
    password=os.getenv("ALLOYDB_PASSWORD"),
    host=os.getenv("ALLOYDB_HOST"),
    port=os.getenv("ALLOYDB_PORT", "5432")
)
cur = conn.cursor()

# Ensure pgvector extension and table exist
cur.execute("CREATE EXTENSION IF NOT EXISTS vector;")
cur.execute("""
CREATE TABLE IF NOT EXISTS alloydb_vectors (
    id SERIAL PRIMARY KEY,
    item_id TEXT,
    embedding VECTOR(1536),
    metadata JSONB
);
""")
cur.execute("""
CREATE INDEX IF NOT EXISTS idx_alloydb_embedding_vector ON alloydb_vectors USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
""")
conn.commit()
```

---

## Step 2: Store and Search Vectors with OpenAI Embeddings

### 2.1. Generate and Store an Embedding

```python
import json
from graphbit import EmbeddingClient, EmbeddingConfig 

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = EmbeddingClient(embedding_config)

# Insert a single embedding
doc_text = "This is a sample document for vector search."
embedding = embedding_client.embed(doc_text)

cur.execute(
    """
    INSERT INTO alloydb_vectors (item_id, embedding, metadata)
    VALUES (%s, %s, %s)
    """,
    ("item123", embedding, json.dumps({"category": "test"}))
)
conn.commit()
print("Inserted embedding for item123.")
```

### Step 2.2: Batch Embedding Example

```python
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)
for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings)):
    cur.execute(
        """
        INSERT INTO alloydb_vectors (item_id, embedding, metadata)
        VALUES (%s, %s, %s)
        """,
        (f"batch_{idx}", emb, json.dumps({"text": text}))
    )
conn.commit()
print(f"Inserted {len(batch_texts)} documents with embeddings.")
```

---

## 3. Vector Search using GraphBit

```python
import ast

query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)

cur.execute("SELECT item_id, embedding, metadata FROM alloydb_vectors;")
all_rows = cur.fetchall()

best_score = -1
best_item = None
for item_id, embedding_vec, metadata in all_rows:
    # Convert the embedding from string to list if needed
    if isinstance(embedding_vec, str):
        embedding_vec = ast.literal_eval(embedding_vec)
    score = embedding_client.similarity(query_embedding, embedding_vec)
    if score > best_score:
        best_score = score
        best_item = (item_id, metadata)
if best_item is not None:
    print(f"Most similar document: {best_item[0]} with score {best_score:.4f}")
else:
    print("No documents found in vector table.")
```

---

**This integration enables you to leverage Google's enterprise-grade database and AI capabilities within your Graphbit workflows, providing access to AlloyDB's high-performance vector database with built-in pgvector support, powerful AI models, and scalable infrastructure for production AI applications.** 
