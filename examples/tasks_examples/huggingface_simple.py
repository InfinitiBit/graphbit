import asyncio
import os

import graphbit

# Initialize Graphbit
graphbit.init()

# Configure Hugging Face embedding model - use the working model
embedding_client = graphbit.EmbeddingClient(
    graphbit.EmbeddingConfig.huggingface(
        model="intfloat/multilingual-e5-large",  # Use the working model
        api_key=os.getenv("HUGGINGFACE_API_KEY", ""),
    )
)

# Configure Hugging Face LLM
llm_config = graphbit.LlmConfig.huggingface(
    model="microsoft/DialoGPT-medium",
    api_key=os.getenv("HUGGINGFACE_API_KEY", ""),
)
llm_client = graphbit.LlmClient(llm_config)

# Generate embeddings
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.", 
    "Hugging Face provides transformers and models for NLP tasks.", 
    "OpenAI offers tools for LLMs and embeddings."
]

print("📊 Generating embeddings...")
embeddings = embedding_client.embed_many(texts)
print(f"✅ Generated {len(embeddings)} embeddings")

query = "GraphBit"
print(f"🔍 Generating query embedding for: '{query}'")
query_embedding = embedding_client.embed(query)
print(f"✅ Query embedding generated with {len(query_embedding)} dimensions")

# Calculate similarities
context = []
threshold = 0.3
print("🎯 Calculating similarities...")

for i, (text, embedding) in enumerate(zip(texts, embeddings)):
    similarity = embedding_client.similarity(query_embedding, embedding)
    print(f"  Similarity {i+1}: {similarity:.3f}")
    if similarity > threshold:
        context.append((text, similarity))

print(f"✅ Found {len(context)} texts above threshold")

# Generate LLM response
if context:
    context_text = "\n".join([f"- {text} (similarity: {sim:.3f})" for text, sim in context])
    prompt = f"""Based on the following context, explain what GraphBit is in one sentence:

Context:
{context_text}

Answer:"""
else:
    prompt = "Explain what GraphBit is in one sentence."

print("🤖 Generating LLM response...")
try:
    # Use synchronous version to avoid async issues
    result = llm_client.complete(prompt, max_tokens=100)
    print(f"✅ LLM Response: {result}")
except Exception as e:
    print(f"❌ LLM failed: {e}")
    print("📊 Showing embedding results instead:")
    for i, (text, sim) in enumerate(context):
        print(f"  {i+1}. Similarity {sim:.3f}: {text[:50]}...") 