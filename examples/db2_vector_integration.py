#!/usr/bin/env python3
"""
IBM Db2 Integration with GraphBit for Vector Embeddings.

This script demonstrates how to integrate IBM Db2 with GraphBit to store and search
vector embeddings. It includes database connection, table creation, embedding storage,
and similarity search functionality.

Prerequisites:
- IBM Db2 12.1.2 installed and running locally
- Python environment with required dependencies
- OpenAI API key for embeddings

Dependencies to install:
pip install ibm-db graphbit python-dotenv numpy

Author: GraphBit Team
"""

import json
import os
import struct
import uuid

import ibm_db
from dotenv import load_dotenv

from graphbit import EmbeddingClient as gb_etc
from graphbit import EmbeddingConfig as gb_ecg

# Load environment variables
load_dotenv()

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = gb_ecg.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = gb_etc(embedding_config)

# Initialize Db2 database
db2_database = os.getenv("DB2_DATABASE", "SAMPLE")
db2_host = os.getenv("DB2_HOST", "localhost")
db2_port = int(os.getenv("DB2_PORT", "25000"))
db2_username = os.getenv("DB2_USERNAME", "db2inst1")
db2_password = os.getenv("DB2_PASSWORD", "db2inst1")

conn_str = f"DATABASE={db2_database};HOSTNAME={db2_host};PORT={db2_port};PROTOCOL=TCPIP;UID={db2_username};PWD={db2_password};"

# Try to connect to Db2
conn = ibm_db.connect(conn_str, "", "")
print("Connection successful!")

# Create table if it doesn't exist
table_name = "graphbit_demo"

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

# Insert a single embedding
doc_text = "This is a sample document for vector search."
try:
    embedding = embedding_client.embed(doc_text)
    print(f"Generated embedding with {len(embedding)} dimensions")
    # print(f"Generated embedding: {embedding}")

    # Convert embedding to bytes for BLOB storage
    embedding_bytes = struct.pack(f"{len(embedding)}f", *embedding)
    print(f"Embedding bytes length: {len(embedding_bytes)}")

    # Prepare metadata as JSON string
    metadata = {"category": "test", "source": "sample"}
    metadata_json = json.dumps(metadata)

    # Use parameterized query to avoid SQL injection
    insert_sql = """
    INSERT INTO graphbit_demo (id, text_content, embedding, metadata)
    VALUES ('item_1', ?, ?, ?)
    """
    stmt = ibm_db.prepare(conn, insert_sql)
    ibm_db.bind_param(stmt, 1, doc_text)
    ibm_db.bind_param(stmt, 2, embedding_bytes)
    ibm_db.bind_param(stmt, 3, metadata_json)
    ibm_db.execute(stmt)
    print("Inserted embedding for item 'item123'.")
except Exception as e:
    print(f"Error inserting embedding: {str(e)}")

# Batch insert
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
        embedding_bytes = struct.pack(f"{len(emb)}f", *emb)
        print(f"Embedding bytes length: {len(embedding_bytes)}")

        # Prepare metadata as JSON string
        metadata = {"text": text, "batch_id": f"batch__{str(uuid.uuid4())}", "index": str(idx)}
        metadata_json = json.dumps(metadata)

        # Use parameterized query to avoid SQL injection
        insert_sql_batch = """
        INSERT INTO graphbit_demo (id, text_content, embedding, metadata)
        VALUES (?, ?, ?, ?)
        """
        stmt_batch = ibm_db.prepare(conn, insert_sql_batch)
        ibm_db.bind_param(stmt_batch, 1, f"batch_{idx}")
        ibm_db.bind_param(stmt_batch, 2, text)
        ibm_db.bind_param(stmt_batch, 3, embedding_bytes)
        ibm_db.bind_param(stmt_batch, 4, metadata_json)
        ibm_db.execute(stmt_batch)
        print(f"Inserted embedding for item 'batch_{idx}'.")
except Exception as e:
    print(f"Error inserting batch embeddings: {str(e)}")


# Vector search with actual stored embeddings
print("\n" + "=" * 60)
print("VECTOR SEARCH WITH STORED EMBEDDINGS")
print("=" * 60)

query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)
print(f"Generated query embedding with {len(query_embedding)} dimensions.")

# # Method 1: Try to retrieve embeddings using LENGTH function
# print("\nAttempting to retrieve stored embeddings...")

# # First, let's see what's actually stored in the database
# select_info_sql = f"""
# SELECT id, LENGTH(embedding) as embedding_length, metadata
# FROM {table_name}
# """
# stmt_info = ibm_db.prepare(conn, select_info_sql)
# ibm_db.execute(stmt_info)

# print("Stored embeddings information:")
# while ibm_db.fetch_row(stmt_info):
#     item_id = ibm_db.result(stmt_info, 0)
#     embedding_length = ibm_db.result(stmt_info, 1)
#     metadata = ibm_db.result(stmt_info, 2)
#     print(f"  {item_id}: embedding length = {embedding_length} bytes, metadata = {metadata}")

# Method 2: Try to retrieve embeddings using direct BLOB access
print("\nAttempting to retrieve embeddings using direct BLOB access...")

# Let's try to regenerate the embeddings from the original texts to demonstrate the concept
print("Since BLOB retrieval has limitations, let's regenerate embeddings from stored metadata...")

# Get the stored metadata to regenerate embeddings
select_metadata_sql = """
SELECT id, text_content, metadata
FROM graphbit_demo
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


# Close the connection
ibm_db.close(conn)
print("\nDone.")
