# Embeddings

GraphBit provides vector embedding capabilities for semantic search, similarity analysis, and other AI-powered text operations. This guide covers configuration and usage for working with embeddings.

## Overview

GraphBit's embedding system supports:
- **Multiple Providers** - OpenAI and HuggingFace embedding models
- **Unified Interface** - Consistent API across all providers
- **Batch Processing** - Efficient processing of multiple texts
- **Similarity Calculations** - Built-in cosine similarity functions

## Configuration

### OpenAI Configuration

Configure OpenAI embedding provider:

```python
import graphbit
import os

# Initialize GraphBit
graphbit.init()

# Basic OpenAI configuration
embedding_config = graphbit.EmbeddingConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="text-embedding-3-small"  # Optional - defaults to text-embedding-3-small
)

print(f"Provider: OpenAI")
print(f"Model: {embedding_config.model}")
```

### HuggingFace Configuration

Configure HuggingFace embedding provider:

```python
# HuggingFace configuration
embedding_config = graphbit.EmbeddingConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="sentence-transformers/all-MiniLM-L6-v2"
)

print(f"Provider: HuggingFace")
print(f"Model: {embedding_config.model}")
```

## Basic Usage

### Creating Embedding Client

```python
# Create embedding client
embedding_client = graphbit.EmbeddingClient(embedding_config)
```

### Single Text Embedding

Generate embeddings for individual texts:

```python
# Embed single text
text = "GraphBit is a powerful framework for AI agent workflows"
vector = embedding_client.embed(text)

print(f"Text: {text}")
print(f"Vector dimension: {len(vector)}")
print(f"First 5 values: {vector[:5]}")
```

### Batch Text Embeddings

Process multiple texts efficiently:

```python
# Embed multiple texts
texts = [
    "Machine learning is transforming industries",
    "Natural language processing enables computers to understand text", 
    "Deep learning models require large datasets",
    "AI ethics is becoming increasingly important",
    "Transformer architectures revolutionized NLP"
]

vectors = embedding_client.embed_many(texts)

print(f"Generated {len(vectors)} embeddings")
for i, (text, vector) in enumerate(zip(texts, vectors)):
    print(f"Text {i+1}: {text[:50]}...")
    print(f"Vector dimension: {len(vector)}")
```

## Similarity Calculations

### Cosine Similarity

Calculate similarity between vectors:

```python
# Generate embeddings for comparison
text1 = "Artificial intelligence and machine learning"
text2 = "AI and ML technologies"

vector1 = embedding_client.embed(text1)
vector2 = embedding_client.embed(text2)

# Calculate similarities
similarity_1_2 = graphbit.EmbeddingClient.similarity(vector1, vector2)

print(f"Similarity between text1 and text2: {similarity_1_2:.3f}")
```

### Finding Most Similar Texts

```python
def find_most_similar(query_text, candidate_texts, embedding_client, threshold=0.7):
    """Find most similar texts to a query"""
    query_vector = embedding_client.embed(query_text)
    candidate_vectors = embedding_client.embed_many(candidate_texts)
    
    similarities = []
    for i, candidate_vector in enumerate(candidate_vectors):
        similarity = graphbit.EmbeddingClient.similarity(query_vector, candidate_vector)
        similarities.append((i, candidate_texts[i], similarity))
    
    # Sort by similarity (highest first)
    similarities.sort(key=lambda x: x[2], reverse=True)
    
    # Filter by threshold
    results = [(text, sim) for _, text, sim in similarities if sim >= threshold]
    
    return results

# Example usage
query = "machine learning algorithms"
candidates = [
    "Deep learning neural networks",
    "Supervised learning models",
    "Recipe for chocolate cake",
    "Natural language processing",
    "Computer vision techniques",
    "Sports news update"
]

similar_texts = find_most_similar(query, candidates, embedding_client, threshold=0.5)

print(f"Query: {query}")
print("Most similar texts:")
for text, similarity in similar_texts:
    print(f"- {text} (similarity: {similarity:.3f})")
```

## What's Next

- Learn about [Performance](performance.md) for optimization techniques
- Explore [Monitoring](monitoring.md) for production monitoring  
- Check [Validation](validation.md) for input validation strategies
- See [LLM Providers](llm-providers.md) for language model integration
