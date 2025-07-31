"""
This script demonstrates the integration of Google Cloud Platform (GCP) services with Graphbit.

It includes examples for:

- Text generation using Vertex AI.
- Storing and indexing embeddings in AlloyDB (PostgreSQL).
- Using Graphbit for embedding management and vector search.
"""

import ast
import json
import os

import psycopg2
from dotenv import load_dotenv

from graphbit import EmbeddingClient as gb_etc
from graphbit import EmbeddingConfig as gb_ecg

# from google.cloud import aiplatform
# from vertexai.language_models import TextGenerationModel


# Load environment variables
load_dotenv()

# # Initialize Vertex AI
# aiplatform.init(project=os.getenv("GOOGLE_CLOUD_PROJECT"), location=os.getenv("GOOGLE_CLOUD_REGION", "us-central1"))

# # Simple text completion using Vertex AI

# prompt = "Hello, how are you?"
# model = TextGenerationModel.from_pretrained("text-bison@001")
# response = model.predict(prompt)
# print(response.text)

# Define the connection details
dbname = os.getenv("DB_NAME")  # Replace with your actual database name
user = os.getenv("DB_USER")  # Your PostgreSQL username
password = os.getenv("DB_PASSWORD")  # Your PostgreSQL password
host = os.getenv("DB_HOST")  # Public IP address (or use private IP if applicable)
port = "5432"  # Default PostgreSQL port

# Establish connection with AlloyDB

conn = psycopg2.connect(
    dbname=dbname,
    user=user,
    password=password,
    host=host,
    port=port,
)
cur = conn.cursor()
print("Connection successful")
# Your queries or logic go here

# Ensure pgvector extension and table exist
cur.execute("CREATE EXTENSION IF NOT EXISTS vector;")
cur.execute(
    """
CREATE TABLE IF NOT EXISTS alloydb_vectors (
    id SERIAL PRIMARY KEY,
    item_id TEXT,
    embedding VECTOR(1536),
    metadata JSONB
);
"""
)
cur.execute(
    """
CREATE INDEX IF NOT EXISTS idx_alloydb_embedding_vector ON alloydb_vectors USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
"""
)
conn.commit()
print("Connection successful!")

# Initialize Graphbit and embedding client

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
embedding_config = gb_ecg.openai(OPENAI_API_KEY, "text-embedding-3-small")
embedding_client = gb_etc(embedding_config)

# Insert a single embedding
doc_text = "This is a sample document for vector search."
embedding = embedding_client.embed(doc_text)
cur.execute(
    """
    INSERT INTO alloydb_vectors (item_id, embedding, metadata)
    VALUES (%s, %s, %s)
    """,
    ("item123", embedding, json.dumps({"category": "test"})),
)
conn.commit()
print("Inserted embedding for item123.")

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
        INSERT INTO alloydb_vectors (item_id, embedding, metadata)
        VALUES (%s, %s, %s)
        """,
        (f"batch_{idx}", emb, json.dumps({"text": text})),
    )
conn.commit()
print(f"Inserted {len(batch_texts)} documents with embeddings.")


# Vector search using Graphbit

query_text = "Find documents related to vector search."
query_embedding = embedding_client.embed(query_text)
cur.execute("SELECT item_id, embedding, metadata FROM alloydb_vectors;")
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
    print(f"[Manual] Most similar document: {best_item[0]} with score {best_score:.4f}")
else:
    print("[Manual] No documents found in vector table.")

# Cleanup
cur.close()
conn.close()
print("Done.")
