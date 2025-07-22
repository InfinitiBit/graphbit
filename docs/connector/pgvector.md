# PGVector Integration with Graphbit


## Overview

This guideline explains how to use PostgreSQL with the PGVector extension as a vector database within the Graphbit ecosystem, leveraging OpenAI embeddings. You will learn how to connect, store, and search data and vectors.

---

## Prerequisites

- **PostgreSQL** with the [PGVector extension](https://github.com/pgvector/pgvector) installed and enabled
- **Python environment** with `psycopg2` and `graphbit` installed:
  ```bash
  pip install psycopg2 graphbit
  ```
- **OpenAI API Key** for embeddings
- **Environment variable** for your OpenAI API key:
  ```bash
  export OPENAI_API_KEY=sk-...
  ```
- **A PostgreSQL database** (e.g., `vector_db`) and user with appropriate permissions

---

## Step 1: Connect to PostgreSQL and Ensure Table Exists

```python
import psycopg2
import os

# Connect to PostgreSQL
conn = psycopg2.connect(
    dbname="vector_db",
    user="postgres",
    password="your_password",
    host="localhost",
    port=5432
)
cur = conn.cursor()

# Ensure PGVector extension and table exist
cur.execute("CREATE EXTENSION IF NOT EXISTS vector;")
cur.execute("""
CREATE TABLE IF NOT EXISTS vector_data (
    id SERIAL PRIMARY KEY,
    item_id TEXT,
    embedding VECTOR(1536),
    metadata JSONB
);
""")
cur.execute("""
CREATE INDEX IF NOT EXISTS idx_embedding_vector ON vector_data USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
""")
conn.commit()
```

> **Note:**
> The dimension in `VECTOR(1536)` must match your embedding model’s output.
>
> **Common Graphbit-supported Openai embedding models:**
>
> | Model Name                  | Dimension |
> |-----------------------------|-----------|
> | text-embedding-ada-002      | 1536      |
> | text-embedding-3-small      | 1536      |
> | text-embedding-3-large      | 3072      |
>
> If you use a different model, check its documentation for the correct dimension.

---

## Step 2: Store and Search Vectors with OpenAI Embeddings

### 2.1. Generate and Store an Embedding

```python
import graphbit
import json

graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = graphbit.EmbeddingClient(embedding_config)

# Generate embedding from text
doc_text = "This is a sample document for vector search."
embedding = embedding_client.embed(doc_text)

# Insert embedding into PGVector table
cur.execute(
    """
    INSERT INTO vector_data (item_id, embedding, metadata)
    VALUES (%s, %s, %s)
    """,
    ("item123", embedding, json.dumps({"category": "test"}))
)
conn.commit()
print("Inserted embedding for item123.")
```

### 2.2. Vector Search Example (SQL/PGVector)

```python
query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)
cur.execute(
    """
    SELECT item_id, metadata, embedding <#> %s::vector AS distance
    FROM vector_data
    ORDER BY embedding <#> %s::vector ASC
    LIMIT 1;
    """,
    (query_embedding, query_embedding)
)
result = cur.fetchone()
if result:
    print(f"Most similar item: {result[0]}, distance: {result[2]:.4f}")
else:
    print("No similar items found.")
```

### 2.3. Vector Search Example (Graphbit Manual Similarity)

```python
import ast
cur.execute("SELECT item_id, embedding, metadata FROM vector_data;")
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

## Step 3: Batch Embedding Example

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
        INSERT INTO vector_data (item_id, embedding, metadata)
        VALUES (%s, %s, %s)
        """,
        (f"batch_{idx}", emb, json.dumps({"text": text}))
    )
conn.commit()
print(f"Inserted {len(batch_texts)} documents with embeddings.")
```

---

## Full Example

```python
import os
import psycopg2
import graphbit
import json
import ast

graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = graphbit.EmbeddingConfig.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = graphbit.EmbeddingClient(embedding_config)

conn = psycopg2.connect(
    dbname="vector_db",
    user="postgres",
    password="your_password",
    host="localhost",
    port=5432
)
cur = conn.cursor()

# Ensure PGVector extension and table exist
cur.execute("CREATE EXTENSION IF NOT EXISTS vector;")
cur.execute("""
CREATE TABLE IF NOT EXISTS vector_data (
    id SERIAL PRIMARY KEY,
    item_id TEXT,
    embedding VECTOR(1536),
    metadata JSONB
);
""")
cur.execute("""
CREATE INDEX IF NOT EXISTS idx_embedding_vector ON vector_data USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
""")
conn.commit()

# Insert a single embedding
doc_text = "This is a sample document for vector search."
embedding = embedding_client.embed(doc_text)
cur.execute(
    """
    INSERT INTO vector_data (item_id, embedding, metadata)
    VALUES (%s, %s, %s)
    """,
    ("item123", embedding, json.dumps({"category": "test"}))
)
conn.commit()

# Vector search (SQL/PGVector)
query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)
cur.execute(
    """
    SELECT item_id, metadata, embedding <#> %s::vector AS distance
    FROM vector_data
    ORDER BY embedding <#> %s::vector ASC
    LIMIT 1;
    """,
    (query_embedding, query_embedding)
)
result = cur.fetchone()
if result:
    print(f"Most similar item: {result[0]}, distance: {result[2]:.4f}")
else:
    print("No similar items found.")

# Vector search (Graphbit manual similarity)
cur.execute("SELECT item_id, embedding, metadata FROM vector_data;")
all_rows = cur.fetchall()
best_score = -1
best_item = None
for item_id, embedding_vec, metadata in all_rows:
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

# Batch insert
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]
batch_embeddings = embedding_client.embed_many(batch_texts)
for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings)):
    cur.execute(
        """
        INSERT INTO vector_data (item_id, embedding, metadata)
        VALUES (%s, %s, %s)
        """,
        (f"batch_{idx}", emb, json.dumps({"text": text}))
    )
conn.commit()
print(f"Inserted {len(batch_texts)} documents with embeddings.")

# Cleanup
cur.close()
conn.close()
print("Done.")
```

---

**This connector pattern enables you to use PostgreSQL with PGVector as a vector database in your AI workflows, orchestrated by Graphbit.** 
