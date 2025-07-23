import os
import faiss
import numpy as np
import graphbit

# 1️⃣ Initialize GraphBit + OpenAI embedding client
graphbit.init()
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.openai(
        model="text-embedding-3-small",
        api_key=os.getenv("OPENAI_API_KEY"),
    )
)

# 2️⃣ Prepare documents & get embeddings
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "MariaDB supports native vector search for efficient AI similarity queries.",
    "OpenAI offers tools for LLMs and embeddings."
]
embeddings = embedding_client.embed_many(texts)
embeddings = np.array(embeddings).astype('float32')

# 3️⃣ Normalize embeddings for cosine similarity
faiss.normalize_L2(embeddings)

# 4️⃣ Create FAISS index (cosine = L2 index + normalized vectors)
dimension = embeddings.shape[1]
index = faiss.IndexFlatIP(dimension)  # Inner product on normalized = cosine similarity
index.add(embeddings)

# 5️⃣ Perform search
query = "What is GraphBit?"
query_embedding = embedding_client.embed(query)
query_embedding = np.array(query_embedding).astype('float32').reshape(1, -1)
faiss.normalize_L2(query_embedding)

scores, indices = index.search(query_embedding, k=3)

# 6️⃣ Print results
for idx, score in zip(indices[0], scores[0]):
    print(f"ID: doc_{idx}\nScore: {score:.4f}\nText: {texts[idx]}\n---")
