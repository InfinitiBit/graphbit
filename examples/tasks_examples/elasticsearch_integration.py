"""This example demonstrates how to integrate GraphBit with Elasticsearch for semantic search using embeddings."""

import os
import uuid

from elasticsearch import Elasticsearch
from elasticsearch.helpers import bulk

from graphbit import EmbeddingClient, EmbeddingConfig

# Initialize Elasticsearch
ELASTICSEARCH_URL = os.getenv("ELASTICSEARCH_URL", "")
ELASTICSEARCH_API_KEY = os.getenv("ELASTICSEARCH_API_KEY", "")
client = Elasticsearch(ELASTICSEARCH_URL, api_key=ELASTICSEARCH_API_KEY)

# Initialize Embedding Client
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY", "")
embedding_config = EmbeddingConfig.openai(model="text-embedding-3-small", api_key=OPENAI_API_KEY)
embedding_client = EmbeddingClient(embedding_config)

# Generate Embeddings
texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Elasticsearch is a powerful full-text search engine.", "OpenAI provides APIs for language and embedding models."]
embeddings = embedding_client.embed_many(texts)

# Create Index
if client.indices.exists(index="graphbit_vectordb"):
    client.indices.delete(index="graphbit_vectordb")

index_body = {
    "mappings": {
        "properties": {
            "text": {"type": "text"},
            "embedding": {
                "type": "dense_vector",
                "dims": len(embeddings[0]),
                "index": True,
                "similarity": "cosine",
            },
        }
    }
}
client.indices.create(index="graphbit_vectordb", body=index_body)

# Insert Vectors
vectors = [
    {
        "_op_type": "index",
        "_index": "graphbit_vectordb",
        "_id": str(uuid.uuid4()),
        "_source": {"text": text, "embedding": embedding},
    }
    for text, embedding in zip(texts, embeddings)
]
bulk(client, vectors)

# Embed Query Text
query_text = "What is GraphBit?"
query_vector = embedding_client.embed(query_text)

# Perform Semantic Search
response = client.search(
    index="graphbit_vectordb",
    knn={
        "field": "embedding",
        "k": 3,
        "num_candidates": 10,
        "query_vector": query_vector,
    },
    _source=["text"],
)

# Display Results
for hit in response["hits"]["hits"]:
    print(f"Score: {hit['_score']:.4f}, Text: {hit['_source']['text']}")
