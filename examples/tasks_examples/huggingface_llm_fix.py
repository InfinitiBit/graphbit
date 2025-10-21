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

# Try different HuggingFace LLM models
llm_models_to_try = [
    "gpt2",
    "distilgpt2", 
    "microsoft/DialoGPT-small",
    "microsoft/DialoGPT-base",
    "microsoft/DialoGPT-medium",
    "EleutherAI/gpt-neo-125M",
    "facebook/opt-125m",
    "bigscience/bloom-560m"
]

working_llm_client = None

for model in llm_models_to_try:
    print(f"\n🔍 Trying HuggingFace LLM model: {model}")
    try:
        llm_config = graphbit.LlmConfig.huggingface(
            model=model,
            api_key=os.getenv("HUGGINGFACE_API_KEY", ""),
        )
        llm_client = graphbit.LlmClient(llm_config)
        
        # Test with a simple prompt
        test_result = llm_client.complete("Hello", max_tokens=10)
        print(f"✅ Model {model} works!")
        working_llm_client = llm_client
        break
        
    except Exception as e:
        print(f"❌ Model {model} failed: {e}")

if not working_llm_client:
    print("\n❌ No HuggingFace LLM models worked")
    print("💡 This is because most models are not available on HuggingFace Inference API")
    print("💡 Consider using a different LLM provider")

# Generate embeddings
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.", 
    "Hugging Face provides transformers and models for NLP tasks.", 
    "OpenAI offers tools for LLMs and embeddings."
]

print("\n📊 Generating embeddings...")
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

# Generate LLM response if we have a working client
if working_llm_client and context:
    context_text = "\n".join([f"- {text} (similarity: {sim:.3f})" for text, sim in context])
    prompt = f"""Based on the following context, explain what GraphBit is in one sentence:

Context:
{context_text}

Answer:"""
    
    print("🤖 Generating LLM response with HuggingFace...")
    try:
        result = working_llm_client.complete(prompt, max_tokens=100)
        print(f"✅ LLM Response: {result}")
    except Exception as e:
        print(f"❌ LLM failed: {e}")
        print("\n📊 Showing embedding results instead:")
        for i, (text, sim) in enumerate(context):
            print(f"  {i+1}. Similarity {sim:.3f}: {text[:50]}...")
else:
    print("\n📊 Showing embedding results:")
    for i, (text, sim) in enumerate(context):
        print(f"  {i+1}. Similarity {sim:.3f}: {text[:50]}...")

print(f"\n✅ HuggingFace embeddings working perfectly!")
print(f"💡 Embeddings: intfloat/multilingual-e5-large (HuggingFace)")
if working_llm_client:
    print(f"💡 LLM: Working HuggingFace model found")
else:
    print(f"💡 LLM: No working HuggingFace models available") 