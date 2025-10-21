# Voyage AI Integration with Graphbit

## Overview

This guide explains how to use Voyage AI's embedding models with Graphbit for generating high-quality embeddings. Voyage AI provides state-of-the-art embedding models optimized for retrieval and semantic search tasks, offering better performance than many traditional embedding providers.

---

## Prerequisites

- **Voyage AI API Key**: Obtain from [Voyage AI Console](https://www.voyageai.com/).
- **OpenAI API Key**: For LLM response generation using Graphbit (or another supported LLM provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `voyageai`, `graphbit`, `scikit-learn`, and optionally `python-dotenv` installed.
- **.env file** in your project root with the following variables:
  ```env
  VOYAGE_API_KEY=your_voyage_api_key_here
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Step 1: Initialize Voyage AI Client

Set up the Voyage AI client for embedding generation:

```python
import os
import voyageai
from dotenv import load_dotenv

load_dotenv()

# Initialize Voyage AI client
# This will automatically use the environment variable VOYAGE_API_KEY
vo = voyageai.Client()
# Alternatively, you can use vo = voyageai.Client(api_key="your_api_key_here")
```

---

## Step 2: Generate Embeddings using Voyage AI

Use Voyage AI to generate high-quality embeddings for your texts:

```python
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Voyage AI provides state-of-the-art embedding models for retrieval tasks.",
    "Semantic search enables finding relevant content based on meaning."
]

# Generate embeddings using Voyage AI
result = vo.embed(
    texts,
    model="voyage-3.5",  # Latest model (also supports "voyage-3", "voyage-3-lite", etc.)
    input_type="document"
)
embeddings = result.embeddings
```

---

## Step 3: Integration with Vector Database

Store and search embeddings in your preferred vector database:

```python
import uuid

# Example with in-memory storage for demonstration
vector_store = []

# Store embeddings with metadata
for i, (text, embedding) in enumerate(zip(texts, embeddings)):
    vector_store.append({
        "id": str(uuid.uuid4()),
        "text": text,
        "embedding": embedding,
        "metadata": {"index": i, "source": "voyage_ai_demo"}
    })

print(f"Stored {len(vector_store)} embeddings")
```

---

## Step 4: Perform Similarity Search

Generate query embedding and find similar content:

```python
import numpy as np
from sklearn.metrics.pairwise import cosine_similarity

query = "What is GraphBit used for?"
query_result = vo.embed(
    [query],
    model="voyage-3.5",
    input_type="query"
)
query_embedding = query_result.embeddings[0]

# Calculate similarities
similarities = []
for item in vector_store:
    similarity = cosine_similarity(
        [query_embedding], 
        [item["embedding"]]
    )[0][0]
    similarities.append((item, similarity))

# Sort by similarity (highest first)
similarities.sort(key=lambda x: x[1], reverse=True)

# Display results
print(f"Query: {query}")
print("Top results:")
for item, score in similarities[:2]:
    print(f"Score: {score:.4f}")
    print(f"Text: {item['text']}")
```

---

## Step 5: Batch Processing with Different Input Types

Voyage AI supports different input types for optimal performance:

```python
# For documents (content to be searched)
documents = [
    "GraphBit enables building complex AI workflows with ease.",
    "Vector databases store high-dimensional embeddings efficiently."
]

doc_embeddings = vo.embed(
    documents,
    model="voyage-3.5",
    input_type="document"
).embeddings

# For queries (search terms)
queries = [
    "How to build AI workflows?",
    "What are vector databases?"
]

query_embeddings = vo.embed(
    queries,
    model="voyage-3.5",
    input_type="query"
).embeddings

print(f"Generated {len(doc_embeddings)} document embeddings")
print(f"Generated {len(query_embeddings)} query embeddings")
```

---

## Step 6: Refinement with Rerankers

Use Voyage AI's reranker to improve retrieval quality by reranking the initial results:

```python
# Get top candidates from similarity search
top_candidates = similarities[:5]  # Get top 5 candidates
candidate_docs = [item['text'] for item, score in top_candidates]

# Use Voyage AI reranker to refine results
reranked_results = vo.rerank(
    query=query,
    documents=candidate_docs,
    model="rerank-2.5",
    top_k=3
)

print("Reranked Results:")
for result in reranked_results.results:
    print(f"Document: {result.document}")
    print(f"Relevance Score: {result.relevance_score}")
    print(f"Original Index: {result.index}")
```

---

## Step 7: Generate Response using Graphbit LLM

Use Graphbit's LLM client to generate responses based on retrieved context:

```python
from graphbit import LlmClient, LlmConfig

# Initialize Graphbit LLM client
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
llm_config = LlmConfig.openai(
    model="gpt-4o",
    api_key=OPENAI_API_KEY
)
llm_client = LlmClient(llm_config)

# Get the most relevant document from reranked results
best_match = reranked_results.results[0].document

# Create a prompt with context
prompt = f"""Based on the following context, answer the user's question accurately and concisely:

Context: {best_match}

Question: {query}

Answer:"""

# Generate response using Graphbit LLM
response = llm_client.complete(prompt)
print(f"Generated Response: {response}")
```

---

## Complete RAG Example with Reranking and Response Generation

```python
import os
import uuid
import numpy as np
import voyageai
from sklearn.metrics.pairwise import cosine_similarity
from dotenv import load_dotenv
from graphbit import LlmClient, LlmConfig

load_dotenv()

# Initialize Voyage AI client
vo = voyageai.Client()  # Automatically uses VOYAGE_API_KEY environment variable

# Initialize Graphbit LLM client
llm_config = LlmConfig.openai(
    model="gpt-4o",
    api_key=os.getenv("OPENAI_API_KEY"),
)
llm_client = LlmClient(llm_config)

# Sample texts for embedding
texts = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "Voyage AI provides state-of-the-art embedding models for retrieval tasks.",
    "Semantic search enables finding relevant content based on meaning.",
    "Vector databases enable efficient similarity search over embeddings.",
    "RAG (Retrieval-Augmented Generation) combines retrieval and generation for better AI responses.",
    "Rerankers help improve the quality of retrieved documents by scoring relevance."
]

# Generate embeddings
result = vo.embed(
    texts,
    model="voyage-3.5",
    input_type="document"
)
embeddings = result.embeddings

# Store in vector store (simplified example)
vector_store = []
for i, (text, embedding) in enumerate(zip(texts, embeddings)):
    vector_store.append({
        "id": str(uuid.uuid4()),
        "text": text,
        "embedding": embedding,
        "metadata": {"index": i, "source": "voyage_demo"}
    })

print(f"Stored {len(vector_store)} embeddings")

# Perform similarity search
query = "What is GraphBit used for?"
query_result = vo.embed(
    [query],
    model="voyage-3.5",
    input_type="query"
)
query_embedding = query_result.embeddings[0]

# Calculate similarities
similarities = []
for item in vector_store:
    similarity = cosine_similarity(
        [query_embedding], 
        [item["embedding"]]
    )[0][0]
    similarities.append((item, similarity))

# Sort results by similarity
similarities.sort(key=lambda x: x[1], reverse=True)

print(f"\nQuery: {query}")
print("Initial Search Results:")
for item, score in similarities[:3]:
    print(f"Score: {score:.4f}")
    print(f"Text: {item['text']}")
    print("---")

# Rerank the top candidates
top_candidates = similarities[:5]
candidate_docs = [item['text'] for item, score in top_candidates]

reranked_results = vo.rerank(
    query=query,
    documents=candidate_docs,
    model="rerank-2.5",
    top_k=3
)

print("\nReranked Results:")
for result in reranked_results.results:
    print(f"Relevance Score: {result.relevance_score}")
    print(f"Document: {result.document}")
    print("---")

# Generate response using the best match
best_match = reranked_results.results[0].document

prompt = f"""Based on the following context, answer the user's question accurately and concisely:

Context: {best_match}

Question: {query}

Answer:"""

response = llm_client.complete(prompt)
print(f"\nGenerated Response: {response}")
```

---

## Available Models

Voyage AI offers several embedding models optimized for different use cases:

- **voyage-3.5**: Latest general-purpose model with best performance
- **voyage-3**: High-performance general-purpose model
- **voyage-3-lite**: Faster, smaller model for cost-sensitive applications  
- **voyage-finance-2**: Specialized model for financial documents
- **voyage-law-2**: Optimized for legal documents
- **voyage-code-2**: Designed for code understanding and retrieval

---

**This integration enables you to leverage Voyage AI's state-of-the-art embedding models with Graphbit for superior semantic search, retrieval-augmented generation, and document understanding workflows.** 