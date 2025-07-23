"""Integration ChromaDB vector database with GraphBit."""

import os

from chromadb import Client

import graphbit

chromadb_client = Client()

if "chatbot_memory" in [c.name for c in chromadb_client.list_collections()]:
    collection = chromadb_client.get_collection(name="chromadb_integration")
else:
    collection = chromadb_client.create_collection(name="chromadb_integration", metadata={"hnsw:space": "cosine"})

graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

texts = [
    "GraphBit enables orchestration of LLM workflows with flexible integrations.",
    "ChromaDB is a fast, open-source embedding database for AI applications.",
    "OpenAI provides models and tools for embeddings and natural language understanding.",
]
embeds = embedding_client.embed_many(texts)

collection.add(documents=texts, embeddings=embeds, ids=[f"doc_{i}" for i in range(len(texts))], metadatas=[{"source": "initial_knowledge", "chunk_id": i} for i in range(len(texts))])

query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)

query_result = collection.query(
    query_embeddings=[query_embedding],
    n_results=3,
    include=["documents", "metadatas", "distances"],
)

ids = query_result["ids"][0]
docs = query_result["documents"][0]
distances = query_result["distances"][0]

scores = [1 - d for d in distances]

# Print results
for doc_id, text, score in zip(ids, docs, scores):
    print(f"ID: {doc_id}\nScore: {score:.4f}\nText: {text}\n---")
