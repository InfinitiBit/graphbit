"""MariaDB integration example for storing and querying vector embeddings using GraphBit."""

import array
import ast
import json
import os

import mariadb

import graphbit

# 1. Connect to MariaDB
conn = mariadb.connect(
    user=os.getenv("DB_USER", "root"),
    password=os.getenv("DB_PASSWORD", "12345"),
    host=os.getenv("DB_HOST", "localhost"),
    port=3306,
    database=os.getenv("DB_NAME", "vector_db"),
)
cursor = conn.cursor()

# 2️2. Create table with native VECTOR support + HNSW index
cursor.execute(
    """
CREATE TABLE IF NOT EXISTS graphbit_vector (
    id INT AUTO_INCREMENT PRIMARY KEY,
    text TEXT NOT NULL,
    embedding VECTOR(1536) NOT NULL,
    metadata JSON
);
"""
)
conn.commit()

# 3️3. Initialize GraphBit embedding
graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

# 4️4. Insert sample text embeddings
texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Qdrant is an open-source vector database for similarity search.", "OpenAI offers tools for LLMs and embeddings."]
embeds = embedding_client.embed_many(texts)

for txt, vec in zip(texts, embeds):
    # Convert to binary
    vec_bytes = array.array("f", vec).tobytes()
    cursor.execute("INSERT INTO graphbit_vector (text, embedding, metadata) VALUES (?, ?, ?)", (txt, vec_bytes, json.dumps({"source": "graphbit"})))

conn.commit()

# 5️5. Run vector similarity query using SQL (cosine distance)
query_vec = embedding_client.embed("What is GraphBit?")
query_vec_bytes = array.array("f", query_vec).tobytes()

cursor.execute(
    """
SELECT id, text, VEC_DISTANCE_COSINE(embedding, ?) AS score
FROM graphbit_vector
ORDER BY score
LIMIT 2;
""",
    (query_vec_bytes,),
)


rows = cursor.fetchall()
for id_, text, score in rows:
    print(f"ID: {id_}\nSimilarity Score: {(1-score):.4f}\nText: {text}\n---")


cursor.execute("SELECT id, text, embedding, metadata FROM graphbit_vector;")
all_rows = cursor.fetchall()

best_score = -1
best_item = None

for row_id, text, embedding_vec_raw, _metadata in all_rows:
    if isinstance(embedding_vec_raw, str):
        embedding_vec = ast.literal_eval(embedding_vec_raw)
    elif isinstance(embedding_vec_raw, (bytes, bytearray)):
        embedding_vec = array.array("f")
        embedding_vec.frombytes(embedding_vec_raw)
        embedding_vec = embedding_vec.tolist()

    if len(embedding_vec) != len(query_vec):
        print(f"Skipping row {row_id} due to dimension mismatch.")
        continue

    score = embedding_client.similarity(query_vec, embedding_vec)
    print(f"Row ID: {row_id}, Score: {score:.4f}, Text: {text}")

cursor.close()
conn.close()
