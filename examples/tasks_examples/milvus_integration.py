"""Milvus integration example with GraphBit for vector embeddings and similarity search."""

import os

from pymilvus import MilvusClient

from graphbit import EmbeddingClient, EmbeddingConfig

embedding_client = EmbeddingClient(
    EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY", ""),
    )
)

texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Milvus is an open-source vector database for scalable similarity search and AI applications.",
    "OpenAI offers tools for LLMs and embeddings.",
]
embeddings = embedding_client.embed_many(texts)

vectordb_client = MilvusClient("graphbit_vector.db")
dimension = len(embeddings[0])

collection_name = "graphbit_collection"
if vectordb_client.has_collection(collection_name=collection_name):
    vectordb_client.drop_collection(collection_name=collection_name)

vectordb_client.create_collection(collection_name, dimension=dimension)

vectors = [{"id": i, "vector": embedding, "text": texts[i]} for i, embedding in enumerate(embeddings)]
vectordb_client.insert(collection_name=collection_name, data=vectors)

# Query
queries = ["What is GraphBit?", "What is Milvus?"]
query_embeddings = embedding_client.embed_many(queries)

search_results = vectordb_client.search(
    collection_name=collection_name,
    data=query_embeddings,
    limit=3,
    output_fields=["text"],
)

for idx, result in enumerate(search_results):
    print(f"Query {idx}: {queries[idx]}")
    for item in result:
        print(f"  id: {item['id']}, Text: {item.get('entity', {}).get('text', '')}, Score: {item.get('distance', 0):.4f}")
