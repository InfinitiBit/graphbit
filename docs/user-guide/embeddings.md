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

## Performance Optimization

### Batch Processing

Optimize performance with batch operations:

```python
def process_large_text_collection(texts, embedding_client, batch_size=100):
    """Process large collections of texts efficiently"""
    all_vectors = []
    
    # Process in batches
    for i in range(0, len(texts), batch_size):
        batch = texts[i:i + batch_size]
        batch_vectors = embedding_client.embed_many(batch)
        all_vectors.extend(batch_vectors)
        
        print(f"Processed batch {i//batch_size + 1}, texts {i+1}-{min(i+batch_size, len(texts))}")
    
    return all_vectors

# Example with large text collection
large_text_collection = [f"Document {i} with some sample content" for i in range(1000)]
vectors = process_large_text_collection(large_text_collection, embedding_client)
print(f"Generated {len(vectors)} embeddings")
```

### Caching Embeddings

Implement caching for repeated embeddings:

```python
import hashlib
import json
import os

class CachedEmbeddingClient:
    def __init__(self, embedding_client, cache_dir="embedding_cache"):
        self.embedding_client = embedding_client
        self.cache_dir = cache_dir
        os.makedirs(cache_dir, exist_ok=True)
    
    def _get_cache_key(self, text):
        """Generate cache key for text"""
        return hashlib.md5(text.encode()).hexdigest()
    
    def _get_cache_path(self, cache_key):
        """Get cache file path"""
        return os.path.join(self.cache_dir, f"{cache_key}.json")
    
    def embed(self, text):
        """Embed text with caching"""
        cache_key = self._get_cache_key(text)
        cache_path = self._get_cache_path(cache_key)
        
        # Check cache first
        if os.path.exists(cache_path):
            with open(cache_path, 'r') as f:
                return json.load(f)
        
        # Generate embedding and cache it
        vector = self.embedding_client.embed(text)
        with open(cache_path, 'w') as f:
            json.dump(vector, f)
        
        return vector
    
    def embed_many(self, texts):
        """Embed multiple texts with caching"""
        vectors = []
        uncached_texts = []
        uncached_indices = []
        
        # Check cache for all texts
        for i, text in enumerate(texts):
            cache_key = self._get_cache_key(text)
            cache_path = self._get_cache_path(cache_key)
            
            if os.path.exists(cache_path):
                with open(cache_path, 'r') as f:
                    vectors.append(json.load(f))
            else:
                vectors.append(None)  # Placeholder
                uncached_texts.append(text)
                uncached_indices.append(i)
        
        # Generate embeddings for uncached texts
        if uncached_texts:
            uncached_vectors = self.embedding_client.embed_many(uncached_texts)
            
            # Cache and insert new embeddings
            for idx, vector in zip(uncached_indices, uncached_vectors):
                cache_key = self._get_cache_key(texts[idx])
                cache_path = self._get_cache_path(cache_key)
                
                with open(cache_path, 'w') as f:
                    json.dump(vector, f)
                
                vectors[idx] = vector
        
        return vectors

# Usage
cached_client = CachedEmbeddingClient(embedding_client)

# First call - will generate and cache
vector1 = cached_client.embed("This text will be cached")

# Second call - will use cache
vector2 = cached_client.embed("This text will be cached")

print(f"Vectors are identical: {vector1 == vector2}")
```

## What's Next

- Learn about [Performance](performance.md) for optimization techniques
- Explore [Monitoring](monitoring.md) for production monitoring  
- Check [Validation](validation.md) for input validation strategies
- See [LLM Providers](llm-providers.md) for language model integration
