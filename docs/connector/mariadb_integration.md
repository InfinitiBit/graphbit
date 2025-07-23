# MariaDB Integration with Graphbit

## Overview

This guide explains how to use Graphbit to generate embeddings and perform similarity search in MariaDB using its native VECTOR type and vector functions. You can use both MariaDB's built-in vector search and manual similarity search in Python for debugging or prototyping.

---

## Prerequisites

- **MariaDB 11.4+** with native VECTOR support enabled.
- **OpenAI API Key** (or another supported embedding provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `mariadb`, `graphbit`, and `numpy` installed.
- **A MariaDB database and user** with appropriate permissions.

---

## Step 1: Connect to MariaDB

Establish a connection to your MariaDB instance:

```python
import os
import mariadb

conn = mariadb.connect(
    user=os.getenv("DB_USER", "root"),
    password=os.getenv("DB_PASSWORD", "12345"),
    host=os.getenv("DB_HOST", "localhost"),
    port=3306,
    database=os.getenv("DB_NAME", "vector_db"),
)
```

---

## Step 2: Create Table

Create a table with native VECTOR support:

```python
cursor = conn.cursor()

cursor.execute("""
CREATE TABLE IF NOT EXISTS graphbit_vector (
    id INT AUTO_INCREMENT PRIMARY KEY,
    text TEXT NOT NULL,
    embedding VECTOR(1536) NOT NULL,
    metadata JSON
);
""")
conn.commit()
```

---

## Step 3: Generate Embeddings using Graphbit

Use Graphbit to generate embeddings for your texts:

```python
import graphbit

graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "MariaDB supports native vector search for efficient AI similarity queries.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeds = embedding_client.embed_many(texts)
```

---

## Step 4: Insert Embeddings into MariaDB

Insert the generated embeddings into the MariaDB table:

```python
import array
import json

for txt, vec in zip(texts, embeds):
    vec_bytes = array.array('f', vec).tobytes()
    cursor.execute(
        "INSERT INTO graphbit_vector (text, embedding, metadata) VALUES (?, ?, ?)",
        (txt, vec_bytes, json.dumps({"source": "graphbit"}))
    )
conn.commit()
```

---

## Step 5: Vector Search using SQL (Native MariaDB Vector Search)

Embed your query and search for similar vectors in MariaDB using SQL:

```python
query_vec = embedding_client.embed("What is GraphBit?")
query_vec_bytes = array.array('f', query_vec).tobytes()

cursor.execute("""
SELECT id, text, VEC_DISTANCE_COSINE(embedding, ?) AS score
FROM graphbit_vector
ORDER BY score
LIMIT 2;
""", (query_vec_bytes,))

rows = cursor.fetchall()
for id_, text, score in rows:
    print(f"ID: {id_}\nSimilarity Score: {(1-score):.4f}\nText: {text}\n---")
```

---

## Step 6: Vector Search Example (Graphbit Manual Similarity)

You can also fetch all embeddings from MariaDB and perform similarity search in Python using Graphbit. This is useful for debugging or comparing results.

```python
import ast

cursor.execute("SELECT id, text, embedding, metadata FROM graphbit_vector;")
all_rows = cursor.fetchall()

best_score = -1
best_item = None

for row_id, text, embedding_vec_raw, metadata in all_rows:
    if isinstance(embedding_vec_raw, str):
        embedding_vec = ast.literal_eval(embedding_vec_raw)
    elif isinstance(embedding_vec_raw, (bytes, bytearray)):
        embedding_vec = array.array('f')
        embedding_vec.frombytes(embedding_vec_raw)
        embedding_vec = embedding_vec.tolist()

    if len(embedding_vec) != len(query_vec):
        print(f"Skipping row {row_id} due to dimension mismatch.")
        continue

    score = embedding_client.similarity(query_vec, embedding_vec)
    print(f"Row ID: {row_id}, Score: {score:.4f}, Text: {text}")
```

---

## Full Example

```python
import os
import array
import json
import numpy as np
import mariadb
import graphbit

# Step 1: Connect to MariaDB
conn = mariadb.connect(
    user=os.getenv("DB_USER", "root"),
    password=os.getenv("DB_PASSWORD", "12345"),
    host=os.getenv("DB_HOST", "localhost"),
    port=3306,
    database=os.getenv("DB_NAME", "vector_db"),
)
cursor = conn.cursor()

# Step 2: Create Table
cursor.execute("""
CREATE TABLE IF NOT EXISTS graphbit_vector (
    id INT AUTO_INCREMENT PRIMARY KEY,
    text TEXT NOT NULL,
    embedding VECTOR(1536) NOT NULL,
    metadata JSON
);
""")
conn.commit()

# Step 3: Generate Embeddings using Graphbit
graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Qdrant is an open-source vector database for similarity search.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeds = embedding_client.embed_many(texts)

# Step 4: Insert Embeddings into MariaDB
for txt, vec in zip(texts, embeds):
    vec_bytes = array.array('f', vec).tobytes()
    cursor.execute(
        "INSERT INTO graphbit_vector (text, embedding, metadata) VALUES (?, ?, ?)",
        (txt, vec_bytes, json.dumps({"source": "graphbit"}))
    )
conn.commit()

# Step 5: SQL vector search
query_vec = embedding_client.embed("What is GraphBit?")
query_vec_bytes = array.array('f', query_vec).tobytes()
cursor.execute("""
SELECT id, text, VEC_DISTANCE_COSINE(embedding, ?) AS score
FROM graphbit_vector
ORDER BY score
LIMIT 2;
""", (query_vec_bytes,))
rows = cursor.fetchall()
for id_, text, score in rows:
    print(f"[SQL] ID: {id_}\nSimilarity Score: {(1-score):.4f}\nText: {text}\n---")

# Step 6: Manual similarity search
cursor.execute("SELECT id, text, embedding FROM graphbit_vector")
rows = cursor.fetchall()
def decode_vector(vec_bytes):
    arr = array.array('f')
    arr.frombytes(vec_bytes)
    return np.array(arr)
texts = []
vectors = []
ids = []
for id_, text, vec_bytes in rows:
    ids.append(id_)
    texts.append(text)
    vectors.append(decode_vector(vec_bytes))
query_vec = np.array(embedding_client.embed("What is GraphBit?"))
def cosine_similarity(a, b):
    return np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b))
scores = [cosine_similarity(query_vec, vec) for vec in vectors]
ranked = sorted(zip(ids, texts, scores), key=lambda x: x[2], reverse=True)
for id_, text, score in ranked[:2]:
    print(f"[Manual] ID: {id_}\nScore: {score:.4f}\nText: {text}\n---")

# Closing Mariadb
cursor.close()
conn.close()
```

---

**This integration enables you to leverage Graphbit's embedding capabilities with MariaDB's native vector support for scalable, production-grade semantic search and retrieval workflows.** 
