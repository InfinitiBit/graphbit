# Neo4j Integration with Graphbit

## Overview

This guide demonstrates how to integrate Neo4j, a native graph database, with Graphbit for building graph-based AI applications. This integration enables powerful knowledge graph applications, semantic networks, and graph-based retrieval augmented generation (RAG) by combining Neo4j's graph capabilities with Graphbit's LLM workflow orchestration.

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

## Step 1: Create the Neo4jGraphbit Class

Create a class to manage Neo4j and embedding operations:

```python
import os
from typing import List, Dict, Any, Tuple
import numpy as np
import json
from dotenv import load_dotenv
from neo4j import GraphDatabase
from graphbit import EmbeddingClient, EmbeddingConfig

# Load environment variables
load_dotenv()

# Configuration
NEO4J_URI = os.getenv("NEO4J_URI")
NEO4J_USERNAME = os.getenv("NEO4J_USERNAME")
NEO4J_PASSWORD = os.getenv("NEO4J_PASSWORD")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

class Neo4jGraphbit:
    def __init__(self):
        # Initialize Neo4j driver
        self.driver = GraphDatabase.driver(
            NEO4J_URI,
            auth=(NEO4J_USERNAME, NEO4J_PASSWORD)
        )
        
        # Initialize embedding client
        embedding_config = EmbeddingConfig.openai(
            model="text-embedding-3-small",
            api_key=OPENAI_API_KEY
        )
        self.embedding_client = EmbeddingClient(embedding_config)
        
        # Verify connections
        self._verify_connections()
        
    def _verify_connections(self):
        """Verify Neo4j and embedding client connections."""
        try:
            self.driver.verify_connectivity()
            print("✓ Neo4j connection successful")
            
            # Test embedding generation
            test_embedding = self.embedding_client.embed("test")
            if len(test_embedding) > 0:
                print("✓ Embedding client configured successfully")
        except Exception as e:
            print(f"Connection error: {str(e)}")
            raise
            
    def close(self):
        """Close the Neo4j driver connection."""
        self.driver.close()
```

---

## Step 3: Create Vector-Enabled Graph Schema

Create a schema that supports vector similarity search in Neo4j:

```python
def setup_vector_schema(driver):
    with driver.session() as session:
        # Create document ID constraint
        session.run("""
            CREATE CONSTRAINT document_id_unique IF NOT EXISTS
            FOR (n:Document)
            REQUIRE n.id IS UNIQUE
        """)
        
        # Create vector index for embeddings
        session.run("""
            CREATE VECTOR INDEX documentEmbeddings IF NOT EXISTS
            FOR (n:Document)
            ON n.embedding
            OPTIONS {indexConfig: {
                `vector.dimensions`: 1536,
                `vector.similarity_function`: 'cosine'
            }}
        """)

# Initialize schema
setup_vector_schema(driver)
```

---

## Step 4: Store Documents with Embeddings

Store documents and their embeddings in Neo4j:

```python
def store_document(driver, embedding_client, text, metadata=None):
    # Generate embedding
    embedding = embedding_client.embed(text)
    
    # Prepare metadata
    metadata = metadata or {}
    
    with driver.session() as session:
        result = session.run("""
            CREATE (d:Document {
                id: randomUUID(),
                text: $text,
                embedding: $embedding,
                metadata_str: $metadata_str
            })
            RETURN d.id as id
        """, {
            "text": text,
            "embedding": embedding.tolist(),
            "metadata_str": json.dumps(metadata)
        })
        return result.single()["id"]

# Example usage
documents = [
    {
        "text": "GraphBit is a comprehensive framework for building and orchestrating LLM-powered applications with advanced graph capabilities.",
        "metadata": {"category": "framework", "type": "overview", "component": "core"}
    },
    {
        "text": "GraphBit's Core Features include: LLM workflow management, knowledge graph integration, vector similarity search, and agent orchestration.",
        "metadata": {"category": "features", "type": "core", "component": "features"}
    },
    {
        "text": "GraphBit's agent system enables autonomous AI agents to collaborate, share context, and execute complex tasks using graph-based memory.",
        "metadata": {"category": "feature", "type": "technical", "component": "agents"}
    }
]

# Store documents in batches
neo4j_graphbit.batch_store_documents(documents)
```

---

## Step 5: Perform Vector Similarity Search

Search for similar documents using vector similarity:

```python
def search_similar_documents(driver, embedding_client, query_text, limit=5):
    query_embedding = embedding_client.embed(query_text)
    
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
            "k": limit,
            "embedding": query_embedding.tolist()
        })
        
        return [(record["text"], json.loads(record["metadata_str"]), record["score"]) 
                for record in results]

# Example usage
queries = [
    "GraphBit core capabilities and features",
    "How does GraphBit handle agent collaboration",
    "Knowledge representation and search"
]

for query in queries:
    print(f"\nQuery: {query}")
    results = neo4j_graphbit.search_similar_documents(query, limit=3)
    for text, metadata, score in results:
        print(f"\nScore: {score:.4f}")
        print(f"Text: {text}")
        print(f"Metadata: {metadata}")
```

---

## Step 6: Create Knowledge Graph Relationships

Enhance your graph by creating relationships between documents:

```python
def create_relationship(driver, source_text, target_text, relationship_type):
    with driver.session() as session:
        session.run("""
            MATCH (source:Document {text: $source_text})
            MATCH (target:Document {text: $target_text})
            MERGE (source)-[r:$relationship_type]->(target)
        """, {
            "source_text": source_text,
            "target_text": target_text,
            "relationship_type": relationship_type
        })

# Example: Create relationships between documents
neo4j_graphbit.create_relationship(
    documents[0]["text"],  # Overview document
    documents[1]["text"],  # Core features document
    "HAS_FEATURES"
)

neo4j_graphbit.create_relationship(
    documents[1]["text"],  # Core features document
    documents[2]["text"],  # Agent system document
    "INCLUDES"
)
```

---

## Step 7: Graph-Enhanced RAG Search

Combine vector similarity with graph traversal for enhanced search:

```python
def graph_enhanced_search(driver, embedding_client, query_text, limit=5, max_hops=2):
    query_embedding = embedding_client.embed(query_text)
    
    with driver.session() as session:
        query = f"""
            CALL db.index.vector.queryNodes(
                'documentEmbeddings',
                $k,
                $embedding
            ) YIELD node, score
            MATCH path = (node)-[*0..{max_hops}]->(related:Document)
            RETURN related.text AS text,
                   related.metadata_str AS metadata_str,
                   score,
                   length(path) AS distance
            ORDER BY score DESC, distance
            LIMIT $limit
        """
        results = session.run(query, {
            "k": limit,
            "embedding": query_embedding.tolist(),
            "limit": limit
        })
        
        return [(record["text"], json.loads(record["metadata_str"]), 
                 record["score"], record["distance"])
                for record in results]

# Example: Graph-enhanced search with relationship traversal
query = "GraphBit features and capabilities"
results = neo4j_graphbit.graph_enhanced_search(query, limit=3, max_hops=2)
for text, metadata, score, distance in results:
    print(f"\nScore: {score:.4f}, Hops: {distance}")
    print(f"Text: {text}")
    print(f"Metadata: {metadata}")
print("\n" + "="*50)
```

---

## Complete Implementation

Here's the complete code that implements all the functionality described above:

```python
"""
Neo4j Integration Example for GraphBit
This code demonstrates the integration between Neo4j and GraphBit,
including vector similarity search and graph-based operations.
"""

import os
from typing import List, Dict, Any, Tuple
import numpy as np
import json
from dotenv import load_dotenv
from neo4j import GraphDatabase
from graphbit import EmbeddingClient, EmbeddingConfig

# Load environment variables
load_dotenv()

# Configuration
NEO4J_URI = os.getenv("NEO4J_URI")
NEO4J_USERNAME = os.getenv("NEO4J_USERNAME")
NEO4J_PASSWORD = os.getenv("NEO4J_PASSWORD")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

class Neo4jGraphbit:
    def __init__(self):
        # Initialize Neo4j driver
        self.driver = GraphDatabase.driver(
            NEO4J_URI,
            auth=(NEO4J_USERNAME, NEO4J_PASSWORD)
        )
        
        # Initialize embedding client
        embedding_config = EmbeddingConfig.openai(
            model="text-embedding-3-small",
            api_key=OPENAI_API_KEY
        )
        self.embedding_client = EmbeddingClient(embedding_config)
        
        # Verify connections
        self._verify_connections()
        
    def _verify_connections(self):
        """Verify Neo4j and embedding client connections."""
        try:
            self.driver.verify_connectivity()
            print("✓ Neo4j connection successful")
            
            # Test embedding generation
            test_embedding = self.embedding_client.embed("test")
            if len(test_embedding) > 0:
                print("✓ Embedding client configured successfully")
        except Exception as e:
            print(f"Connection error: {str(e)}")
            raise

    def setup_vector_schema(self):
        """Initialize Neo4j schema with vector search capabilities."""
        with self.driver.session() as session:
            # Create constraints
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
        print("✓ Vector schema initialized")

    def store_document(self, text: str, metadata: Dict[str, Any] = None) -> str:
        """Store a single document with its embedding."""
        metadata = metadata or {}
        embedding = self.embedding_client.embed(text)
        
        with self.driver.session() as session:
            result = session.run("""
                CREATE (d:Document {
                    id: randomUUID(),
                    text: $text,
                    embedding: $embedding,
                    metadata_str: $metadata_str
                })
                RETURN d.id as id
            """, {
                "text": text,
                "embedding": embedding if isinstance(embedding, list) else embedding.tolist(),
                "metadata_str": json.dumps(metadata)
            })
            return result.single()["id"]

    def batch_store_documents(self, documents: List[Dict[str, Any]], batch_size: int = 100):
        """Store multiple documents in batches."""
        total_stored = 0
        for i in range(0, len(documents), batch_size):
            batch = documents[i:i + batch_size]
            texts = [doc["text"] for doc in batch]
            embeddings = self.embedding_client.embed_many(texts)
            
            with self.driver.session() as session:
                for doc, embedding in zip(batch, embeddings):
                    session.run("""
                        CREATE (d:Document {
                            id: randomUUID(),
                            text: $text,
                            embedding: $embedding,
                            metadata_str: $metadata_str
                        })
                    """, {
                        "text": doc["text"],
                        "embedding": embedding if isinstance(embedding, list) else embedding.tolist(),
                        "metadata_str": json.dumps(doc.get("metadata", {}))
                    })
            total_stored += len(batch)
            print(f"✓ Stored {total_stored}/{len(documents)} documents")

    def create_relationship(self, source_text: str, target_text: str, relationship_type: str):
        """Create a relationship between two documents."""
        with self.driver.session() as session:
            query = f"""
                MATCH (source:Document {{text: $source_text}})
                MATCH (target:Document {{text: $target_text}})
                MERGE (source)-[r:{relationship_type}]->(target)
            """
            session.run(query, {
                "source_text": source_text,
                "target_text": target_text
            })

    def search_similar_documents(self, query_text: str, limit: int = 5) -> List[Tuple[str, Dict, float]]:
        """Search for similar documents using vector similarity."""
        query_embedding = self.embedding_client.embed(query_text)
        
        with self.driver.session() as session:
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
                "k": limit,
                "embedding": query_embedding if isinstance(query_embedding, list) else query_embedding.tolist()
            })
            
            return [(record["text"], json.loads(record["metadata_str"]), record["score"]) 
                    for record in results]

    def graph_enhanced_search(self, query_text: str, limit: int = 5, max_hops: int = 2):
        """Perform graph-enhanced vector similarity search."""
        query_embedding = self.embedding_client.embed(query_text)
        
        with self.driver.session() as session:
            query = f"""
                CALL db.index.vector.queryNodes(
                    'documentEmbeddings',
                    $k,
                    $embedding
                ) YIELD node, score
                MATCH path = (node)-[*0..{max_hops}]->(related:Document)
                RETURN related.text AS text,
                       related.metadata_str AS metadata_str,
                       score,
                       length(path) AS distance
                ORDER BY score DESC, distance
                LIMIT $limit
            """
            results = session.run(query, {
                "k": limit,
                "embedding": query_embedding if isinstance(query_embedding, list) else query_embedding.tolist(),
                "limit": limit
            })
            
            return [(record["text"], json.loads(record["metadata_str"]), 
                     record["score"], record["distance"])
                    for record in results]

    def close(self):
        """Close the Neo4j driver connection."""
        self.driver.close()


# Example usage
if __name__ == "__main__":
    try:
        # Initialize and set up
        neo4j_graphbit = Neo4jGraphbit()
        neo4j_graphbit.setup_vector_schema()
        
        # Prepare sample documents
        documents = [
            {
                "text": "GraphBit is a comprehensive framework for building and orchestrating LLM-powered applications with advanced graph capabilities.",
                "metadata": {"category": "framework", "type": "overview", "component": "core"}
            },
            {
                "text": "GraphBit's Core Features include: LLM workflow management, knowledge graph integration, vector similarity search, and agent orchestration.",
                "metadata": {"category": "features", "type": "core", "component": "features"}
            },
            {
                "text": "GraphBit's agent system enables autonomous AI agents to collaborate, share context, and execute complex tasks using graph-based memory.",
                "metadata": {"category": "feature", "type": "technical", "component": "agents"}
            }
        ]
        
        # Store documents
        neo4j_graphbit.batch_store_documents(documents)
        
        # Create relationships
        neo4j_graphbit.create_relationship(
            documents[0]["text"],
            documents[1]["text"],
            "HAS_FEATURES"
        )
        
        neo4j_graphbit.create_relationship(
            documents[1]["text"],
            documents[2]["text"],
            "INCLUDES"
        )
        
        # Perform searches
        query = "GraphBit core capabilities"
        print(f"\nVector Search Results for: {query}")
        results = neo4j_graphbit.search_similar_documents(query, limit=2)
        for text, metadata, score in results:
            print(f"\nScore: {score:.4f}")
            print(f"Text: {text}")
            print(f"Metadata: {metadata}")
        
        print("\nGraph-Enhanced Search Results:")
        results = neo4j_graphbit.graph_enhanced_search(query, limit=2)
        for text, metadata, score, distance in results:
            print(f"\nScore: {score:.4f}, Hops: {distance}")
            print(f"Text: {text}")
            print(f"Metadata: {metadata}")
            
    finally:
        neo4j_graphbit.close()
```

---

## Additional Resources

- [Neo4j Vector Search Documentation](https://neo4j.com/blog/developer/graph-metadata-filtering-vector-search-rag/)
- [Neo4j Python Driver Documentation](https://neo4j.com/docs/python-manual/current/)
- [GraphBit Documentation](../index.md)