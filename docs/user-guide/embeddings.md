# Embeddings

GraphBit provides vector embedding capabilities for semantic search, similarity analysis, and other AI-powered text operations. This guide covers configuration and usage for working with embeddings.

## Overview

GraphBit's embedding system supports:
- **Provider** - OpenAI embedding models
- **Unified Interface** - Consistent API across all providers
- **Batch Processing** - Efficient processing of multiple texts
- **Similarity Calculations** - Built-in cosine similarity functions

## Configuration

### OpenAI Configuration

Configure OpenAI embedding provider:

```python
import os

from graphbit import EmbeddingConfig, EmbeddingClient

# Basic OpenAI configuration
openai_embedding_config = EmbeddingConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="text-embedding-3-small"  # Optional - defaults to text-embedding-3-small
)

# Create embedding client
openai_embedding_client = EmbeddingClient(openai_embedding_config)

print(f"Provider: OpenAI")
print(f"Model: {openai_embedding_config.model}")
```

#### Single Text Embedding

Generate embeddings for individual texts:

```python
# Embed single text
text = "GraphBit is a powerful framework for AI agent workflows"
vector = embedding_client.embed(text)

print(f"Text: {text}")
print(f"Vector dimension: {len(vector)}")
print(f"First 5 values: {vector[:5]}")
```

#### Batch Text Embeddings

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

### Litellm Configuration

Configure Litellm embedding provider:

```python
import os

from graphbit import EmbeddingConfig, EmbeddingClient

# Litellm openai configuration
openai_embedding_config = EmbeddingConfig.litellm(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="text-embedding-3-small"
)

# Litellm mistal AI configuration
mistal_embedding_config = EmbeddingConfig.litellm(
    api_key=os.getenv("MISTRAL_API_KEY"),
    model="mistral/mistral-embed"
)

# Create embedding clients
openai_embedding_client = EmbeddingClient(openai_embedding_config)

mistral_embedding_client = EmbeddingClient(mistral_embedding_config)
```

#### Single Text Embedding

Generate embeddings for individual texts:

```python
# Embed single text
text = "GraphBit is a powerful framework for AI agent workflows"

# Openai embedding
openai_embedding = openai_embedding_client.embed(text)

# Mistral embedding
mistral_embedding = mistral_embedding_client.embed(text)

print(f"Text: {text}")
print(f"Openai Embedding dimension: {len(openai_embedding)}")
print(f"Openai Embedding first 5 values: {openai_embedding[:5]}")
print(f"Mistral Embedding dimension: {len(mistral_embedding)}")
print(f"Mistral Embedding first 5 values: {mistral_embedding[:5]}")
```

#### Batch Text Embeddings

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

# Openai Embeddings
openai_embeddings = openai_embedding_client.embed_many(texts)

# Mistral Embeddings
mistral_embeddings = mistral_embedding_client.embed_many(texts)

print(f"Openai generated {len(openai_embeddings)} embeddings")
for i, (text, openai_embedding) in enumerate(zip(texts, openai_embeddings)):
    print(f"Text {i+1}: {text[:50]}...")
    print(f"Openai embedding dimension: {len(openai_embedding)}")

print(f"Mistral generated {len(mistral_embeddings)} embeddings")
for i, (text, mistral_embedding) in enumerate(zip(texts, mistral_embeddings)):
    print(f"Text {i+1}: {text[:50]}...")
    print(f"Mistral embedding dimension: {len(mistral_embedding)}")
```

## Similarity Calculations

### Cosine Similarity

Calculate similarity between vectors:

```python
from graphbit import EmbeddingClient

# Generate embeddings for comparison
text1 = "Artificial intelligence and machine learning"
text2 = "AI and ML technologies"

vector1 = embedding_client.embed(text1)
vector2 = embedding_client.embed(text2)

# Calculate similarities
similarity_1_2 = EmbeddingClient.similarity(vector1, vector2)

print(f"Similarity between text1 and text2: {similarity_1_2:.3f}")
```

### Finding Most Similar Texts

```python
from graphbit import EmbeddingClient

def find_most_similar(query_text, candidate_texts, embedding_client, threshold=0.7):
    """Find most similar texts to a query"""
    query_vector = embedding_client.embed(query_text)
    candidate_vectors = embedding_client.embed_many(candidate_texts)
    
    similarities = []
    for i, candidate_vector in enumerate(candidate_vectors):
        similarity = EmbeddingClient.similarity(query_vector, candidate_vector)
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
