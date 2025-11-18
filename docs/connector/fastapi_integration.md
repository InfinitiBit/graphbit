# FastAPI Integration with GraphBit

## Overview

This guide explains how to integrate FastAPI with GraphBit to build production-ready REST APIs and WebSocket endpoints powered by LLM workflows and agent orchestration. FastAPI's async capabilities combined with GraphBit's AI features enable you to create high-performance, scalable AI-powered applications.

---

## Prerequisites

- **Python environment** with `fastapi`, `uvicorn`, `graphbit`, and optionally `python-dotenv` installed.
- **OpenAI API Key** (or another supported LLM provider)
- **.env file** in your project root with the following variables:
  ```env
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Step 1: Basic FastAPI Application with GraphBit LLM

Create a simple REST API endpoint that uses GraphBit's LLM client for text generation:

```python
import os
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from dotenv import load_dotenv
from graphbit import LlmClient, LlmConfig

load_dotenv()

app = FastAPI(title="GraphBit FastAPI Integration")

# Initialize GraphBit LLM client
llm_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)
llm_client = LlmClient(llm_config)

class PromptRequest(BaseModel):
    prompt: str
    max_tokens: int = 150

@app.post("/generate/")
async def generate_text(request: PromptRequest):
    """Generate text using GraphBit LLM."""
    try:
        response = llm_client.complete(request.prompt, max_tokens=request.max_tokens)
        return {"response": response}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
```

---

## Step 2: Semantic Search API with Embeddings

Build an endpoint that performs semantic search using GraphBit's embedding capabilities:

```python
from graphbit import EmbeddingClient, EmbeddingConfig

# Initialize embedding client
embedding_config = EmbeddingConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="text-embedding-3-small"
)
embedding_client = EmbeddingClient(embedding_config)

# In-memory document store
documents = [
    "GraphBit is a framework for LLM workflows and agent orchestration.",
    "FastAPI is a modern web framework for building APIs with Python.",
    "Semantic search enables finding similar content based on meaning."
]
doc_embeddings = embedding_client.embed_many(documents)

class SearchRequest(BaseModel):
    query: str
    top_k: int = 3

@app.post("/search/")
async def semantic_search(request: SearchRequest):
    """Perform semantic search over documents."""
    try:
        query_embedding = embedding_client.embed(request.query)
        scores = [
            EmbeddingClient.similarity(query_embedding, doc_emb)
            for doc_emb in doc_embeddings
        ]
        results = sorted(
            zip(documents, scores),
            key=lambda x: x[1],
            reverse=True
        )[:request.top_k]
        return {
            "results": [
                {"document": doc, "score": float(score)}
                for doc, score in results
            ]
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
```

---

## Step 3: Background Task Processing

Use FastAPI's background tasks with GraphBit for async processing:

```python
from fastapi import BackgroundTasks

class DocumentRequest(BaseModel):
    text: str

def process_document_background(text: str):
    """Process document in background."""
    embedding = embedding_client.embed(text)
    documents.append(text)
    doc_embeddings.append(embedding)
    print(f"Processed document: {text[:50]}...")

@app.post("/documents/")
async def add_document(request: DocumentRequest, background_tasks: BackgroundTasks):
    """Add document with background processing."""
    background_tasks.add_task(process_document_background, request.text)
    return {"message": "Document queued for processing"}
```

---

## Running the Application

Save your code as `main.py` and run with Uvicorn:

```bash
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

Access the interactive API documentation at `http://localhost:8000/docs`

---

**This integration enables you to build production-ready AI-powered APIs with FastAPI and GraphBit, leveraging async capabilities for high-performance LLM workflows and real-time streaming.**
