import asyncio
import os

import graphbit

# Initialize Graphbit
graphbit.init()

# Configure Hugging Face embedding model
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.huggingface(
        model="sentence-transformers/all-MiniLM-L6-v2",
        api_key=os.getenv("HUGGINGFACE_API_KEY", ""),  # Optional for local models
    )
)

# Configure Hugging Face LLM
llm_config = graphbit.LlmConfig.huggingface(
    model="microsoft/DialoGPT-medium",
    api_key=os.getenv("HUGGINGFACE_API_KEY", ""),
)
llm_client = graphbit.LlmClient(llm_config)

# Generate embeddings
texts = ["GraphBit is a framework for LLM workflows and agent orchestration.", "Hugging Face provides transformers and models for NLP tasks.", "OpenAI offers tools for LLMs and embeddings."]
embeddings = embedding_client.embed_many(texts)
query = "GraphBit"
query_embedding = embedding_client.embed(query)

context = []
threshold = 0.5
for text, embedding in zip(texts, embeddings):
    similarity = embedding_client.similarity(query_embedding, embedding)
    if similarity > threshold:
        context.append((text, similarity))

# Simple prompt for text generation
prompt = f"""Explain what GraphBit is in one sentence.
Context: {context}"""
result = asyncio.run(llm_client.complete_async(prompt, max_tokens=50))

print(f"LLM Response: {result}")
