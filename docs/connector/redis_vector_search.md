# Redis Vector Search Integration with Graphbit

## Overview

This guide shows how to connect Redis Stack to Graphbit for vector similarity search with high-dimensional embeddings.

---

## Prerequisites

- **Redis Stack** with RediSearch 2.4+ and vector search enabled
- **OpenAI API Key** for embedding generation
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md))
- **Python environment** with `redis`, `graphbit`, and `python-dotenv` installed
- **.env file** in your project root:
  ```env
  OPENAI_API_KEY=your_openai_api_key_here
  REDIS_URL=redis://localhost:6379/0
  ```

---

## Step 1: Start Redis Stack

```bash
docker run -p 6379:6379 -it --rm redis/redis-stack-server:latest
```

---

## Step 2: Create Index and Schema

```python
import os
from dotenv import load_dotenv
load_dotenv()
import numpy as np
from redis import Redis
from redis.commands.search.field import VectorField, TextField, NumericField
from redis.commands.search.index_definition import IndexDefinition, IndexType
from redis.commands.search.query import Query

redis_client = Redis.from_url(os.getenv("REDIS_URL"))

INDEX_NAME = "idx:graphbit:v1"
VECTOR_FIELD = "embedding"

try:
    redis_client.ft(INDEX_NAME).info()
except Exception:
    hnsw_params = {
        "TYPE": "FLOAT32",
        "DIM": 1536,
        "DISTANCE_METRIC": "COSINE",
        "INITIAL_CAP": 1000,
        "M": 16,
        "EF_CONSTRUCTION": 200,
    }
    schema = (
        TextField("text"),
        NumericField("created_at"),
        VectorField("embedding", "HNSW", hnsw_params, as_name=VECTOR_FIELD),
    )
    definition = IndexDefinition(prefix=["doc:"], index_type=IndexType.HASH)
    redis_client.ft(INDEX_NAME).create_index(fields=schema, definition=definition)
```

---

## Step 3: Generate and Store Embeddings

```python
import uuid
import time
from graphbit import EmbeddingConfig, EmbeddingClient


EMBEDDING_MODEL = "text-embedding-3-small"
embedding_client = EmbeddingClient(EmbeddingConfig.openai(model=EMBEDDING_MODEL, api_key=os.getenv("OPENAI_API_KEY")))

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Redis Stack supports vector similarity search via RediSearch.",
    "Store vectors as FLOAT32 and keep metadata nearby for filtering."
]

embs = embedding_client.embed_many(texts)

pipe = redis_client.pipeline()
for txt, vec in zip(texts, embs):
    doc_id = f"doc:{uuid.uuid4()}"
    vec_bytes = np.array(vec, dtype=np.float32).tobytes()
    pipe.hset(doc_id, mapping={
        "text": txt,
        "created_at": int(time.time()),
        "embedding": vec_bytes
    })
pipe.execute()
```

---

## Step 4: Perform Similarity Search

```python
query = "What is GraphBit?"
query_vec = embedding_client.embed(query)

query_vec_bytes = np.array(query_vec, dtype=np.float32).tobytes()
results = redis_client.ft(INDEX_NAME).search(
    Query(f"*=>[KNN 3 @{VECTOR_FIELD} $vec AS score]").return_fields("text", "score").sort_by("score"), 
    query_params={"vec": query_vec_bytes}
)

for doc in results.docs:
    score = getattr(doc, 'score', '0')
    try:
        score_float = float(score)
        print(f"Score: {score_float:.4f}")
    except ValueError:
        print(f"Score: {score}")
    print(f"Text: {doc.text}")
```

---

## Full Example

```python
import os
import uuid
import time
import numpy as np
from dotenv import load_dotenv
from redis import Redis
from redis.commands.search.field import VectorField, TextField, NumericField
from redis.commands.search.index_definition import IndexDefinition, IndexType
from redis.commands.search.query import Query
from graphbit import EmbeddingConfig, EmbeddingClient

load_dotenv()
redis_client = Redis.from_url(os.getenv("REDIS_URL"))

INDEX_NAME = "idx:graphbit:v1"
VECTOR_FIELD = "embedding"
try:
    redis_client.ft(INDEX_NAME).info()
except Exception:
    hnsw_params = {
        "TYPE": "FLOAT32",
        "DIM": 1536,
        "DISTANCE_METRIC": "COSINE",
        "INITIAL_CAP": 1000,
        "M": 16,
        "EF_CONSTRUCTION": 200,
    }
    schema = (
        TextField("text"),
        NumericField("created_at"),
        VectorField("embedding", "HNSW", hnsw_params, as_name=VECTOR_FIELD),
    )
    definition = IndexDefinition(prefix=["doc:"], index_type=IndexType.HASH)
    redis_client.ft(INDEX_NAME).create_index(fields=schema, definition=definition)

EMBEDDING_MODEL = "text-embedding-3-small"
embedding_client = EmbeddingClient(EmbeddingConfig.openai(model=EMBEDDING_MODEL, api_key=os.getenv("OPENAI_API_KEY")))

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Redis Stack supports vector similarity search via RediSearch.",
    "Store vectors as FLOAT32 and keep metadata nearby for filtering."
]

embs = embedding_client.embed_many(texts)

pipe = redis_client.pipeline()
for txt, vec in zip(texts, embs):
    doc_id = f"doc:{uuid.uuid4()}"
    vec_bytes = np.array(vec, dtype=np.float32).tobytes()
    pipe.hset(doc_id, mapping={
        "text": txt,
        "created_at": int(time.time()),
        "embedding": vec_bytes
    })
pipe.execute()

query = "What is GraphBit?"
query_vec = embedding_client.embed(query)

query_vec_bytes = np.array(query_vec, dtype=np.float32).tobytes()
results = redis_client.ft(INDEX_NAME).search(
    Query(f"*=>[KNN 3 @{VECTOR_FIELD} $vec AS score]").return_fields("text", "score").sort_by("score"), 
    query_params={"vec": query_vec_bytes}
)

for doc in results.docs:
    score = getattr(doc, 'score', '0')
    try:
        score_float = float(score)
        print(f"Score: {score_float:.4f}")
    except ValueError:
        print(f"Score: {score}")
    print(f"Text: {doc.text}")
```

**This integration enables you to leverage Graphbit's embedding capabilities with Redis Stack's vector search for scalable semantic search workflows.**