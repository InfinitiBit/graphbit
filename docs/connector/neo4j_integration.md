# Neo4j Integration with Graphbit

## Overview

This guide demonstrates how to integrate Neo4j, a native graph database, with Graphbit for building graph-based AI applications. With this integration, you can store, index, and search high-dimensional embeddings in Neo4j for semantic search, knowledge graphs, and retrieval-augmented generation.

---

## Prerequisites

- **Neo4j running locally or remotely** (see [Neo4j documentation](https://neo4j.com/docs/)).
- **OpenAI API Key**: For embedding generation (or another supported embedding provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `neo4j`, `graphbit`, and optionally `python-dotenv` installed.
- **.env file** in your project root with the following variables:
  ```env
  OPENAI_API_KEY=your_openai_api_key_here
  NEO4J_URI=Neo4j_URI
  NEO4J_USERNAME=neo4j
  NEO4J_PASSWORD=your_password_here
  ```

---

## Step 1: Connect to Neo4j and Initialize Embedding Client

Set up the Neo4j driver and Graphbit embedding client:

```python
import os
import json
from dotenv import load_dotenv
from neo4j import GraphDatabase
from graphbit import EmbeddingClient, EmbeddingConfig

load_dotenv()

NEO4J_URI = os.getenv("NEO4J_URI")
NEO4J_USERNAME = os.getenv("NEO4J_USERNAME")
NEO4J_PASSWORD = os.getenv("NEO4J_PASSWORD")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

driver = GraphDatabase.driver(
    NEO4J_URI,
    auth=(NEO4J_USERNAME, NEO4J_PASSWORD)
)

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=OPENAI_API_KEY
    )
)
```

---

## Step 2: Create Vector Index

Create a vector index in Neo4j to enable similarity search:

```python
with driver.session() as session:
    # Create document ID constraint
    session.run("""
        CREATE CONSTRAINT document_id_unique IF NOT EXISTS
        FOR (n:Document)
        REQUIRE n.id IS UNIQUE
    """)
    
    # Create vector index
    session.run("""
        CREATE VECTOR INDEX documentEmbeddings IF NOT EXISTS
        FOR (n:Document)
        ON n.embedding
        OPTIONS {indexConfig: {
            `vector.dimensions`: 1536,
            `vector.similarity_function`: 'cosine'
        }}
    """)
```

---

## Step 3: Generate and Store Embeddings

Use Graphbit to generate embeddings and store them in Neo4j:

```python
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Neo4j is a native graph database with vector search capabilities.",
    "OpenAI offers tools for LLMs and embeddings."
]

embeddings = embedding_client.embed_many(texts)

with driver.session() as session:
    for text, embedding in zip(texts, embeddings):
        session.run("""
            CREATE (d:Document {
                id: randomUUID(),
                text: $text,
                embedding: $embedding,
                metadata_str: $metadata_str
            })
        """, {
            "text": text,
            "embedding": embedding if isinstance(embedding, list) else embedding.tolist(),
            "metadata_str": json.dumps({"source": "initial_knowledge"})
        })
```

---

## Step 4: Perform Similarity Search

Embed your query and search for similar documents in Neo4j:

```python
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

with driver.session() as session:
    results = session.run("""
        CALL db.index.vector.queryNodes(
            'documentEmbeddings',
            $k,
            $embedding
        ) YIELD node, score
        RETURN node.text AS text, 
               node.metadata_str AS metadata_str,
               score
        ORDER BY score DESC
    """, {
        "k": 3,
        "embedding": query_embedding if isinstance(query_embedding, list) else query_embedding.tolist()
    })
    
    for record in results:
        print(f"Score: {record['score']:.4f}")
        print(f"Text: {record['text']}")
        print(f"Metadata: {json.loads(record['metadata_str'])}\n")
```

---

## Full Example

```python
import os
import json
from dotenv import load_dotenv
from neo4j import GraphDatabase
from graphbit import EmbeddingClient, EmbeddingConfig

load_dotenv()

NEO4J_URI = os.getenv("NEO4J_URI")
NEO4J_USERNAME = os.getenv("NEO4J_USERNAME")
NEO4J_PASSWORD = os.getenv("NEO4J_PASSWORD")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

# Initialize connections
driver = GraphDatabase.driver(
    NEO4J_URI,
    auth=(NEO4J_USERNAME, NEO4J_PASSWORD)
)

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=OPENAI_API_KEY
    )
)

# Create vector index
with driver.session() as session:
    session.run("""
        CREATE CONSTRAINT document_id_unique IF NOT EXISTS
        FOR (n:Document)
        REQUIRE n.id IS UNIQUE
    """)
    
    session.run("""
        CREATE VECTOR INDEX documentEmbeddings IF NOT EXISTS
        FOR (n:Document)
        ON n.embedding
        OPTIONS {indexConfig: {
            `vector.dimensions`: 1536,
            `vector.similarity_function`: 'cosine'
        }}
    """)

# Generate and store embeddings
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Neo4j is a native graph database with vector search capabilities.",
    "OpenAI offers tools for LLMs and embeddings."
]

embeddings = embedding_client.embed_many(texts)

with driver.session() as session:
    for text, embedding in zip(texts, embeddings):
        session.run("""
            CREATE (d:Document {
                id: randomUUID(),
                text: $text,
                embedding: $embedding,
                metadata_str: $metadata_str
            })
        """, {
            "text": text,
            "embedding": embedding if isinstance(embedding, list) else embedding.tolist(),
            "metadata_str": json.dumps({"source": "initial_knowledge"})
        })

# Perform similarity search
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

with driver.session() as session:
    results = session.run("""
        CALL db.index.vector.queryNodes(
            'documentEmbeddings',
            $k,
            $embedding
        ) YIELD node, score
        RETURN node.text AS text, 
               node.metadata_str AS metadata_str,
               score
        ORDER BY score DESC
    """, {
        "k": 3,
        "embedding": query_embedding if isinstance(query_embedding, list) else query_embedding.tolist()
    })
    
    for record in results:
        print(f"Score: {record['score']:.4f}")
        print(f"Text: {record['text']}")
        print(f"Metadata: {json.loads(record['metadata_str'])}\n")

driver.close()
```

---

## Additional Resources

- [Neo4j Vector Search Documentation](https://neo4j.com/docs/cypher-manual/current/indexes-for-vector-search/)
- [Neo4j Python Driver Documentation](https://neo4j.com/docs/python-manual/current/)
- [GraphBit Documentation](../index.md)
