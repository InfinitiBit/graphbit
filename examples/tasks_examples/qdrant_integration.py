"""Integration tasks for Qdrant vector database."""

import os
import uuid

from qdrant_client import QdrantClient
from qdrant_client.models import Distance, PointStruct, VectorParams

import graphbit

COLLECTION = "graphbit-vector"
DIMENSION = 1536

# 1) Connect
client = QdrantClient(host="localhost", port=6333)

# 2) Ensure collection exists
if not client.collection_exists(COLLECTION):
    client.create_collection(
        collection_name=COLLECTION,
        vectors_config=VectorParams(size=DIMENSION, distance=Distance.COSINE),
    )

# 3) Init GraphBit + embed
graphbit.init()
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
EMBEDDING_MODEL = "text-embedding-3-small"
embedding_client = graphbit.EmbeddingClient(graphbit.EmbeddingConfig.openai(model=EMBEDDING_MODEL, api_key=OPENAI_API_KEY))

texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Qdrant is an open-source vector database for similarity search.", "OpenAI offers tools for LLMs and embeddings."]
embeds = embedding_client.embed_many(texts)

# 4) Upsert (wait=True blocks until write is applied)
points = [PointStruct(id=str(uuid.uuid4()), vector=vec, payload={"text": txt}) for vec, txt in zip(embeds, texts)]
client.upsert(collection_name=COLLECTION, points=points, wait=True)

# 5) Retrieve one point to double-check (optional)
check_id = points[0].id
retrieved = client.retrieve(collection_name=COLLECTION, ids=[check_id], with_payload=True)
assert retrieved, "Upsert failed!"

# 6) Query
query = "What is GraphBit?"
query_vec = embedding_client.embed(query)
response = client.query_points(collection_name=COLLECTION, query=query_vec, limit=2, with_payload=True)

# 7) Process response
response_points = response.points
result = []
for point in response_points:
    result.append({"id": point.id, "text": point.payload.get("text", ""), "score": point.score})
print(result)
