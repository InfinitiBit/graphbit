"""This script demonstrates how to integrate GraphBit with Weaviate, a vector database."""

import os
import uuid

from weaviate import connect_to_weaviate_cloud
from weaviate.classes.init import Auth

from graphbit import EmbeddingClient, EmbeddingConfig

weaviate_url = os.environ["WEAVIATE_URL"]
weaviate_api_key = os.environ["WEAVIATE_API_KEY"]

vectordb_config = connect_to_weaviate_cloud(
    cluster_url=weaviate_url,
    auth_credentials=Auth.api_key(weaviate_api_key),
)

print(vectordb_config.is_ready())

vectordb_client = vectordb_config.collections.get("Graphbit_VectorDB")
if vectordb_client is None:
    vectordb_client = vectordb_config.collections.create(
        name="Graphbit_VectorDB",
        vector_config={
            "vectorizer": "none",
            "vectorIndexConfig": {
                "distance": "cosine",
            },
        },
    )

embedding_client = EmbeddingClient(EmbeddingConfig.openai(model="text-embedding-3-small", api_key=os.getenv("OPENAI_API_KEY", "")))

texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Weaviate is a vector database for AI applications.", "OpenAI provides APIs for embeddings and LLMs."]

embeddings = embedding_client.embed_many(texts)

for text, vector in zip(texts, embeddings):
    vectordb_client.data.insert(
        properties={"text": text},
        vector=vector,
        uuid=uuid.uuid4(),
    )

query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

results = vectordb_client.query.near_vector(near_vector=query_embedding, limit=3, return_metadata=["distance"], return_properties=["text"])

for obj in results.objects:
    print("Text:", obj.properties["text"])
    print("Distance:", obj.metadata.distance)
    print("Score:", 1 - obj.metadata.distance)

vectordb_config.collections.delete("Graphbit_VectorDB")

vectordb_config.close()
