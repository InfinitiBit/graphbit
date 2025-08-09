# IBM Db2 Integration with Graphbit

## Overview

This guideline explains how to use IBM Db2 as a vector database within the Graphbit ecosystem, leveraging OpenAI embeddings. You will learn how to connect, store, and search data and vectors using IBM Db2's native capabilities.

---

## Prerequisites

- **IBM Db2 12.1.2** installed and running
- **Python environment** with `ibm-db` and `graphbit` installed:
  ```bash
  pip install ibm-db graphbit python-dotenv numpy
  ```
- **Environment variables** for your configuration:
  ```bash
  OPENAI_API_KEY=<your-openai-api-key>
  DB2_DATABASE=<your-database-name>
  DB2_HOST=<your-host>
  DB2_PORT=<your-port>
  DB2_USERNAME=<your-username>
  DB2_PASSWORD=<your_password>
  ```

---

## Step 1: Connect to IBM Db2 and Ensure Table Exists

```python
import ibm_db
import os
import json
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Connect to IBM Db2
database = os.getenv("DB2_DATABASE", "your-database-name")
host = os.getenv("DB2_HOST", "your-host")
port = os.getenv("DB2_PORT", "your-port")
username = os.getenv("DB2_USERNAME", "your-username")
password = os.getenv("DB2_PASSWORD", "your_password")

conn_str = f"DATABASE={database};HOSTNAME={host};PORT={port};PROTOCOL=TCPIP;UID={username};PWD={password};"
connection = ibm_db.connect(conn_str, "", "")

# Try to connect to Db2
conn = ibm_db.connect(conn_str, "", "")
print(f"Connection successful!")

# Create table if it doesn't exist
table_name = "your-table-name"

# Drop the table before creating it again
drop_table_sql = f"DROP TABLE IF EXISTS {table_name}"
ibm_db.exec_immediate(conn, drop_table_sql)

create_table_sql = f"""
CREATE TABLE {table_name} (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    text_content CLOB,
    embedding BLOB,
    metadata CLOB
)
"""
try:
    ibm_db.exec_immediate(conn, create_table_sql)
    print(f"Table '{table_name}' created successfully.")
except Exception as e:
    print(f"Table creation error (may already exist): {str(e)}")
```

---

## Step 2: Store and Search Vectors with OpenAI Embeddings

### 2.1. Generate and Store an Embedding

```python
import json
from graphbit import EmbeddingConfig as gb_ecg
from graphbit import EmbeddingClient as gb_etc

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = gb_ecg.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = gb_etc(embedding_config)

# Insert a single embedding
doc_text = "This is a sample document for vector search."
try:
    embedding = embedding_client.embed(doc_text)
    print(f"Generated embedding with {len(embedding)} dimensions")
    #print(f"Generated embedding: {embedding}") 
    
    # Convert embedding to bytes for BLOB storage
    embedding_bytes = struct.pack(f'{len(embedding)}f', *embedding)
    print(f"Embedding bytes length: {len(embedding_bytes)}")

    # Prepare metadata as JSON string
    metadata = {"category": "test", "source": "sample"}
    metadata_json = json.dumps(metadata)

    insert_sql = f"""
    INSERT INTO {table_name} (id, text_content, embedding, metadata)
    VALUES ('item_1', ?, ?, ?)
    """
    stmt = ibm_db.prepare(conn, insert_sql)
    ibm_db.bind_param(stmt, 1, doc_text)
    ibm_db.bind_param(stmt, 2, embedding_bytes)
    ibm_db.bind_param(stmt, 3, metadata_json)
    ibm_db.execute(stmt)
    print(f"Inserted embedding for item 'item123'.")
except Exception as e:
    print(f"Error inserting embedding: {str(e)}")
```

### 2.2. Batch embedding insert

```python
batch_texts = [
    "Graph databases are great for relationships.",
    "Vector search enables semantic retrieval.",
    "OpenAI provides powerful embedding models.",
]

try:
    batch_embeddings = embedding_client.embed_many(batch_texts)
    print(f"Generated {len(batch_embeddings)} batch embeddings")
    # print(f"Generated batch embeddings: {batch_embeddings}") 

    for idx, (text, emb) in enumerate(zip(batch_texts, batch_embeddings)):
        # Convert embedding to bytes for BLOB storage
        embedding_bytes = struct.pack(f'{len(emb)}f', *emb)
        print(f"Embedding bytes length: {len(embedding_bytes)}")
        
        # Prepare metadata as JSON string
        metadata = {"text": text, "batch_id": "batch__{str(uuid.uuid4())}", "index": idx}
        metadata_json = json.dumps(metadata)
        
        insert_sql_batch = f"""
        INSERT INTO {table_name} (id, text_content, embedding, metadata)
        VALUES ('batch_{idx}', ?, ?, ?)
        """
        stmt_batch = ibm_db.prepare(conn, insert_sql_batch)
        ibm_db.bind_param(stmt_batch, 1, text)
        ibm_db.bind_param(stmt_batch, 2, embedding_bytes)
        ibm_db.bind_param(stmt_batch, 3, metadata_json)
        ibm_db.execute(stmt_batch)
        print(f"Inserted embedding for item 'batch_{idx}'.")
except Exception as e:
    print(f"Error inserting batch embeddings: {str(e)}")
```

### 2.3. Vector Similarity Search using Graphbit

```python
query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)

# Get the stored metadata to regenerate embeddings
select_metadata_sql = f"""
SELECT id, text_content, metadata 
FROM {table_name}
"""
stmt_metadata = ibm_db.prepare(conn, select_metadata_sql)
ibm_db.execute(stmt_metadata)

stored_embeddings = []
while ibm_db.fetch_row(stmt_metadata):
    item_id = ibm_db.result(stmt_metadata, 0)
    text_content = ibm_db.result(stmt_metadata, 1)
    metadata = ibm_db.result(stmt_metadata, 2)
    
    print(f"Processing {item_id}: text_content = {text_content[:50]}...")
    
    if text_content:
        try:
            # Regenerate the embedding from the original text
            regenerated_embedding = embedding_client.embed(text_content)
            print(f"  Regenerated embedding: {len(regenerated_embedding)} dimensions")
            
            stored_embeddings.append((item_id, regenerated_embedding, metadata))
        except Exception as e:
            print(f"  Error regenerating embedding for {item_id}: {str(e)}")
    else:
        print(f"  No text content for {item_id}")

# Perform vector search on retrieved embeddings
print(f"\nPerforming vector search on {len(stored_embeddings)} retrieved embeddings...")

best_score = -1
best_item = None

for item_id, stored_embedding, metadata in stored_embeddings:
    try:
        # Calculate the similarity score between the query and the stored embedding
        score = gb_etc.similarity(query_embedding, stored_embedding)
        print(f"Similarity score for {item_id}: {score:.4f}")
        
        if score > best_score:
            best_score = score
            best_item = (item_id, metadata)
    except Exception as e:
        print(f"Error calculating similarity for {item_id}: {str(e)}")
        continue

# Print the most similar item
if best_item is not None:
    print(f"\nMost similar document: {best_item[0]} with score {best_score:.4f}")
    print(f"Metadata: {best_item[1]}")
else:
    print("No documents found in vector table.")

```

---

**This connector pattern enables you to use IBM Db2 as a vector database in your AI workflows, orchestrated by Graphbit.** 
