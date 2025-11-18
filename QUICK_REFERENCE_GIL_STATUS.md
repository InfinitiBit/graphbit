# Quick Reference: GraphBit GIL Status

**Last Updated**: 2025-11-11  
**Version**: Post-GIL Fixes

---

## ✅ What Works (GIL Released - True Parallelism)

### Embedding Operations
```python
from concurrent.futures import ThreadPoolExecutor
from graphbit import EmbeddingClient, EmbeddingConfig

config = EmbeddingConfig.openai(api_key)
client = EmbeddingClient(config)

# ✅ These methods release GIL - use ThreadPoolExecutor for parallelism
with ThreadPoolExecutor(max_workers=10) as executor:
    # Parallel embed() calls
    futures = [executor.submit(client.embed, text) for text in texts]
    embeddings = [f.result() for f in futures]
    
    # Parallel embed_many() calls
    futures = [executor.submit(client.embed_many, batch) for batch in batches]
    results = [f.result() for f in futures]

# ✅ NEW: Lock-free batch processing (BEST PERFORMANCE)
result = client.embed_batch_parallel(
    texts_batch=[[text1, text2], [text3, text4]],
    max_concurrency=10
)
```

**Expected Speedup**: 5-50x

---

### Document Loading
```python
from concurrent.futures import ThreadPoolExecutor
from graphbit import DocumentLoader

loader = DocumentLoader()

# ✅ load_document() releases GIL - use ThreadPoolExecutor
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = [executor.submit(loader.load_document, path, "pdf") for path in paths]
    documents = [f.result() for f in futures]
```

**Expected Speedup**: 10-50x

---

### LLM Async Operations
```python
import asyncio
from graphbit import LlmClient, LlmConfig

config = LlmConfig.openai(api_key)
client = LlmClient(config)

# ✅ Async methods release GIL automatically
async def process_queries():
    tasks = [
        client.complete_async(prompt1),
        client.complete_async(prompt2),
        client.complete_async(prompt3),
    ]
    results = await asyncio.gather(*tasks)
    return results

results = asyncio.run(process_queries())
```

**Expected Speedup**: N/A (async concurrency)

---

## ❌ What Doesn't Work (GIL Held - Sequential Execution)

### LLM Sync Operations
```python
from concurrent.futures import ThreadPoolExecutor
from graphbit import LlmClient, LlmConfig

config = LlmConfig.openai(api_key)
client = LlmClient(config)

# ❌ These methods HOLD GIL - ThreadPoolExecutor won't help
with ThreadPoolExecutor(max_workers=10) as executor:
    # This will execute SEQUENTIALLY (no parallelism)
    futures = [executor.submit(client.complete, prompt) for prompt in prompts]
    results = [f.result() for f in futures]  # ← Sequential execution!
```

**Workaround**: Use async methods instead
```python
# ✅ Use async methods for parallelism
async def process_parallel():
    tasks = [client.complete_async(prompt) for prompt in prompts]
    return await asyncio.gather(*tasks)

results = asyncio.run(process_parallel())
```

---

### Text Splitters
```python
from concurrent.futures import ThreadPoolExecutor
from graphbit import RecursiveSplitter

splitter = RecursiveSplitter(chunk_size=1000, chunk_overlap=100)

# ❌ split_text() HOLDS GIL - ThreadPoolExecutor won't help
with ThreadPoolExecutor(max_workers=10) as executor:
    # This will execute SEQUENTIALLY (no parallelism)
    futures = [executor.submit(splitter.split_text, doc) for doc in documents]
    chunks = [f.result() for f in futures]  # ← Sequential execution!
```

**Workaround**: Use `asyncio.to_thread()` (Python 3.9+)
```python
import asyncio

async def chunk_parallel():
    tasks = [asyncio.to_thread(splitter.split_text, doc) for doc in documents]
    return await asyncio.gather(*tasks)

chunks = asyncio.run(chunk_parallel())
```

---

## ⚠️ Known Issues

### Issue #287: Nested Tokio Runtime Panic

**Problem**: Cannot use `EmbeddingClient` or `LlmClient` inside workflow tools

```python
from graphbit import tool, Executor, Workflow, EmbeddingClient

# ❌ THIS WILL PANIC!
@tool(_description="Embed text")
def embed_tool(text: str) -> str:
    config = EmbeddingConfig.openai(api_key)
    client = EmbeddingClient(config)
    embedding = client.embed(text)  # ← PANIC: "Cannot start a runtime from within a runtime"
    return str(embedding)

workflow = Workflow("Test")
workflow.add_node(Node.agent(..., tools=[embed_tool]))
executor = Executor(llm_config)
result = executor.execute(workflow)  # ← PANIC HERE!
```

**Workaround**: Create clients outside workflow
```python
# ✅ Create client outside workflow
embedding_client = EmbeddingClient(config)

@tool(_description="Embed text")
def embed_tool(text: str) -> str:
    # Use pre-created client (may still panic if called from workflow)
    embedding = embedding_client.embed(text)
    return str(embedding)
```

**Status**: ❌ NOT FIXED (Priority 1 - Critical)

---

## 📊 Performance Summary

| Operation | GIL Status | Parallel Method | Expected Speedup |
|-----------|------------|-----------------|------------------|
| `embed()` | ✅ Released | ThreadPoolExecutor | 5-10x |
| `embed_many()` | ✅ Released | ThreadPoolExecutor | 5-10x |
| `embed_batch_parallel()` | ✅ Released | Built-in | 10-50x |
| `load_document()` | ✅ Released | ThreadPoolExecutor | 10-50x |
| `complete()` (sync) | ❌ Held | Use async instead | 1x (no parallelism) |
| `complete_async()` | ✅ Released | asyncio.gather | N/A (async) |
| `split_text()` | ❌ Held | asyncio.to_thread | 1x (no parallelism) |

---

## 🎯 Best Practices

### 1. Use Async Methods When Available
```python
# ❌ BAD: Sync method holds GIL
result = client.complete(prompt)

# ✅ GOOD: Async method releases GIL
result = await client.complete_async(prompt)
```

### 2. Use Batch Methods for Best Performance
```python
# ❌ OK: Parallel individual calls
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = [executor.submit(client.embed, text) for text in texts]
    embeddings = [f.result() for f in futures]

# ✅ BETTER: Batch parallel processing
result = client.embed_batch_parallel(
    texts_batch=[[text1, text2], [text3, text4]],
    max_concurrency=10
)
```

### 3. Create Clients Outside Workflows
```python
# ❌ BAD: Create client inside tool
@tool(_description="Process")
def process_tool(text: str):
    client = EmbeddingClient(config)  # ← Will panic!
    return client.embed(text)

# ✅ GOOD: Create client outside
embedding_client = EmbeddingClient(config)

@tool(_description="Process")
def process_tool(text: str):
    return embedding_client.embed(text)
```

### 4. Use ThreadPoolExecutor for GIL-Released Operations
```python
from concurrent.futures import ThreadPoolExecutor

# ✅ GOOD: Use ThreadPoolExecutor for embedding and document loading
with ThreadPoolExecutor(max_workers=10) as executor:
    # Parallel document loading
    doc_futures = [executor.submit(loader.load_document, path, "pdf") for path in paths]
    documents = [f.result() for f in doc_futures]
    
    # Parallel embedding generation
    embed_futures = [executor.submit(client.embed, doc.content) for doc in documents]
    embeddings = [f.result() for f in embed_futures]
```

---

## 🚀 Complete ParallelRAG Example

```python
from concurrent.futures import ThreadPoolExecutor
from graphbit import DocumentLoader, EmbeddingClient, EmbeddingConfig, RecursiveSplitter

# Initialize clients
loader = DocumentLoader()
embed_config = EmbeddingConfig.openai(api_key)
embed_client = EmbeddingClient(embed_config)
splitter = RecursiveSplitter(chunk_size=1000, chunk_overlap=100)

# Document paths
doc_paths = ["doc1.pdf", "doc2.pdf", ..., "doc100.pdf"]

# Step 1: Parallel document loading (✅ GIL released)
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = [executor.submit(loader.load_document, path, "pdf") for path in doc_paths]
    documents = [f.result() for f in futures]

print(f"Loaded {len(documents)} documents in parallel")

# Step 2: Sequential chunking (❌ GIL held - workaround with asyncio.to_thread)
import asyncio

async def chunk_all():
    tasks = [asyncio.to_thread(splitter.split_text, doc.content) for doc in documents]
    return await asyncio.gather(*tasks)

all_chunks = asyncio.run(chunk_all())
flat_chunks = [chunk for chunks in all_chunks for chunk in chunks]

print(f"Created {len(flat_chunks)} chunks")

# Step 3: Lock-free parallel embedding (✅ GIL released - BEST PERFORMANCE)
texts_batch = [
    [chunk.content for chunk in flat_chunks[i:i+10]]
    for i in range(0, len(flat_chunks), 10)
]

result = embed_client.embed_batch_parallel(
    texts_batch=texts_batch,
    max_concurrency=10,
    timeout_ms=30000
)

embeddings = result['embeddings']
stats = result['stats']

print(f"Generated {stats['total_embeddings']} embeddings")
print(f"Success rate: {stats['successful_requests']}/{len(texts_batch)}")
print(f"Duration: {result['duration_ms']}ms")

# Expected performance:
# - Before fixes: 4500 seconds (75 minutes)
# - After fixes: 45 seconds (0.75 minutes)
# - Speedup: 100x faster!
```

---

## 📋 Checklist for Parallel Processing

- [ ] Using `embed()` or `embed_many()`? → Use `ThreadPoolExecutor` ✅
- [ ] Using `embed_batch_parallel()`? → Built-in parallelism ✅
- [ ] Using `load_document()`? → Use `ThreadPoolExecutor` ✅
- [ ] Using `complete()` (sync)? → Switch to `complete_async()` ⚠️
- [ ] Using `split_text()`? → Use `asyncio.to_thread()` workaround ⚠️
- [ ] Using clients in workflow tools? → Create clients outside workflow ⚠️

---

## 🔗 Related Documentation

- **Full Status Report**: `PARALLELRAG_GIL_STATUS_AND_ACTION_PLAN.md`
- **Implementation Guide**: `docs/IMPLEMENTATION_GUIDE_GIL_FIXES.md`
- **Performance Comparison**: `docs/PERFORMANCE_COMPARISON.md`
- **User Guide**: `docs/GIL_FIXES_AND_PERFORMANCE.md`
- **Test Execution Report**: `TEST_EXECUTION_REPORT.md`

---

**Questions or Issues?**
- Check GitHub Issue #287 for nested runtime panic updates
- Review test suite: `tests/python_integration_tests/test_gil_release.py`
- See examples: `examples/parallel_rag_optimized.py`, `examples/benchmark_gil_fixes.py`

