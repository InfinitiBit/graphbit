import asyncio
import os

import graphbit

# Check API key first
api_key = os.getenv("HUGGINGFACE_API_KEY")
if not api_key:
    print("❌ HUGGINGFACE_API_KEY not set")
    print("💡 To fix this:")
    print("1. Get your API key from: https://huggingface.co/settings/tokens")
    print("2. Set it in your environment:")
    print("   export HUGGINGFACE_API_KEY='your-api-key-here'")
    print("3. Or run this script with the API key:")
    print("   HUGGINGFACE_API_KEY='your-key' poetry run python huggingface_integration.py")
    exit(1)

print("✅ API key found")

# Initialize Graphbit
graphbit.init()

# Try different models that don't require 'sentences' parameter
models_to_try = [
    "sentence-transformers/paraphrase-MiniLM-L3-v2",
    "sentence-transformers/all-mpnet-base-v2", 
    "sentence-transformers/all-MiniLM-L12-v2",
    "sentence-transformers/msmarco-MiniLM-L-6-v3",
    # Try some non-sentence-transformers models
    "intfloat/multilingual-e5-large",
    "BAAI/bge-large-en-v1.5",
    "BAAI/bge-base-en-v1.5"
]

working_embedding_client = None

for model in models_to_try:
    print(f"\n🔍 Trying model: {model}")
    try:
        embedding_client = graphbit.EmbeddingClient(
            graphbit.EmbeddingConfig.huggingface(
                model=model,
                api_key=api_key,
            )
        )
        
        # Test with single embedding
        test_embedding = embedding_client.embed("test")
        print(f"✅ Model {model} works! Generated {len(test_embedding)}-dimensional embedding")
        working_embedding_client = embedding_client
        break
        
    except Exception as e:
        print(f"❌ Model {model} failed: {e}")

if not working_embedding_client:
    print("\n❌ No embedding models worked")
    print("💡 This might be due to:")
    print("1. Invalid API key")
    print("2. Network connectivity issues")
    print("3. HuggingFace API rate limiting")
    print("4. The GraphBit code needs to be updated to handle sentence similarity models")
    exit(1)

# Configure Hugging Face LLM
try:
    llm_config = graphbit.LlmConfig.huggingface(
        model="microsoft/DialoGPT-medium",
        api_key=api_key,
    )
    llm_client = graphbit.LlmClient(llm_config)
    print("✅ LLM client created")
except Exception as e:
    print(f"❌ LLM client failed: {e}")
    print("💡 Will continue with embeddings only")

# Generate embeddings with error handling
try:
    texts = [
        "GraphBit is a framework for LLM workflows and agent orchestration.", 
        "Hugging Face provides transformers and models for NLP tasks.", 
        "OpenAI offers tools for LLMs and embeddings."
    ]
    
    print("📊 Generating embeddings...")
    embeddings = working_embedding_client.embed_many(texts)
    print(f"✅ Generated {len(embeddings)} embeddings")
    
    query = "GraphBit"
    print(f"🔍 Generating query embedding for: '{query}'")
    query_embedding = working_embedding_client.embed(query)
    print(f"✅ Query embedding generated with {len(query_embedding)} dimensions")

    # Calculate similarities with error handling
    context = []
    threshold = 0.3  # Lower threshold for better results
    print("🎯 Calculating similarities...")
    
    for i, (text, embedding) in enumerate(zip(texts, embeddings)):
        try:
            similarity = working_embedding_client.similarity(query_embedding, embedding)
            print(f"  Similarity {i+1}: {similarity:.3f}")
            if similarity > threshold:
                context.append((text, similarity))
        except Exception as e:
            print(f"  ❌ Error calculating similarity for text {i+1}: {e}")
    
    print(f"✅ Found {len(context)} texts above threshold")

    # Try LLM if available
    if 'llm_client' in locals():
        # Simple prompt for text generation
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
            # Use asyncio.run() which creates a new event loop
            result = asyncio.run(llm_client.complete_async(prompt, max_tokens=100))
            print(f"✅ LLM Response: {result}")
        except Exception as e:
            print(f"❌ LLM failed: {e}")
            print("📊 Showing embedding results instead:")
            for i, (text, sim) in enumerate(context):
                print(f"  {i+1}. Similarity {sim:.3f}: {text[:50]}...")
    else:
        print("📊 Embedding-only mode - showing similarity results:")
        for i, (text, sim) in enumerate(context):
            print(f"  {i+1}. Similarity {sim:.3f}: {text[:50]}...")

except Exception as e:
    print(f"❌ Error during execution: {e}")
    print("💡 Please check your API key and internet connection")
