"""Integration tasks for Pinecone vector database.

This module contains example tasks demonstrating how to use Pinecone within the project.
"""

import os
import time
import uuid

from pinecone import Pinecone, ServerlessSpec

import graphbit

INDEX_NAME = "graphbit-vector"
PINECONE_API_KEY = os.getenv("PINECONE_API_KEY")

pinecone_client = Pinecone(api_key=PINECONE_API_KEY)
index_name = INDEX_NAME

index_list = pinecone_client.list_indexes()
# Check if the index exists
if index_name not in [idx["name"] for idx in index_list]:
    print(f"Index {index_name} does not exist. Creating it...")
    pinecone_client.create_index(
        name=index_name,
        vector_type="dense",
        dimension=1536,
        metric="cosine",
        spec=ServerlessSpec(cloud="aws", region="us-east-1"),
    )
    # Wait for index to be ready
    while True:
        status = pinecone_client.describe_index(index_name)
        if status["status"]["ready"]:
            break
        print("Waiting for index to be ready...")
        time.sleep(2)
    index = pinecone_client.Index(index_name)
else:
    index = pinecone_client.Index(index_name)

graphbit.init()

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
EMBEDDING_MODEL = "text-embedding-3-small"
embedding_config = graphbit.EmbeddingConfig.openai(model=EMBEDDING_MODEL, api_key=OPENAI_API_KEY)
embedding_client = graphbit.EmbeddingClient(embedding_config)

text = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Pinecone enables vector search over high-dimensional embeddings.", "OpenAI offers tools for LLMs and embeddings."]
embeddings = embedding_client.embed_many(text)
vectors = [(str(uuid.uuid4()), emb, {"text": txt}) for emb, txt in zip(embeddings, text)]

upsert_response = index.upsert(vectors=vectors)
print("Upsert response:", upsert_response)

# Waiting until the vector is available in the index
NAMESPACE = "__default__"
attempt = 0
target_id = vectors[0][0]
while True:
    response = index.fetch(ids=[target_id], namespace=NAMESPACE)
    if response.vectors and target_id in response.vectors:
        print(f"Confirmed upsert: vector {target_id} is available.")
        break
    print(f"Waiting for upsert completion... attempt {attempt}")
    attempt += 1
    time.sleep(2)

query = "What is GraphBit?"
embed_query = embedding_client.embed(query)
print("Query embedding shape:", len(embed_query))

results = index.query(vector=embed_query, top_k=2, include_metadata=True)
print(results)

# Pretty-print results
for match in results["matches"]:
    print(f"Score: {match['score']:.4f}")
    print(f"Text: {match['metadata']['text']}")
    print("---")
