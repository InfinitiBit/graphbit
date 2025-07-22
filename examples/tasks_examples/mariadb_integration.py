"""Integration tasks for storing and querying vector embeddings in MariaDB."""

import json
import os

import mariadb
import numpy as np

import graphbit

# DB config
DB_CONFIG = {
    "user": os.getenv("DB_USER", "root"),
    "password": os.getenv("DB_PASSWORD", "12345"),
    "host": os.getenv("DB_HOST", "localhost"),
    "port": 3306,
    "database": os.getenv("DB_NAME", "vector_db"),
}

DIMENSION = 1536

# 1) Connect to MariaDB
conn = mariadb.connect(**DB_CONFIG)
cursor = conn.cursor()

# 2) Create table (adjusted for MariaDB)
cursor.execute(
    """
CREATE TABLE IF NOT EXISTS graphbit_vector (
    id INT AUTO_INCREMENT PRIMARY KEY,
    text TEXT NOT NULL,
    embedding JSON NOT NULL,
    metadata JSON
)
"""
)
conn.commit()

# 3) Init GraphBit + embed
graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
EMBEDDING_MODEL = "text-embedding-3-small"
embedding_client = graphbit.EmbeddingClient(graphbit.EmbeddingConfig.openai(model=EMBEDDING_MODEL, api_key=OPENAI_API_KEY))

texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Qdrant is an open-source vector database for similarity search.", "OpenAI offers tools for LLMs and embeddings."]
embeds = embedding_client.embed_many(texts)

# 4) Insert into DB
for txt, vec in zip(texts, embeds):
    cursor.execute("INSERT INTO graphbit_vector (text, embedding, metadata) VALUES (?, ?, ?)", (txt, json.dumps(vec), json.dumps({"source": "script"})))
conn.commit()

# 5) Query
query = "What is GraphBit?"
query_vec = np.array(embedding_client.embed(query))

# 6) Fetch + compute cosine similarity in Python
cursor.execute("SELECT id, text, embedding FROM graphbit_vector")
rows = cursor.fetchall()


def cosine_sim(a, b):
    """Compute cosine similarity between two vectors."""
    return np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b))


scored = []
for id_, text, vector_json in rows:
    vec = np.array(json.loads(vector_json))
    score = cosine_sim(query_vec, vec)
    scored.append((id_, text, score))

# 7) Sort and show top matches
top_matches = sorted(scored, key=lambda x: x[2], reverse=True)[:2]
for id_, text, score in top_matches:
    print(f"ID: {id_}\nScore: {score:.4f}\nText: {text}\n---")

cursor.close()
conn.close()
