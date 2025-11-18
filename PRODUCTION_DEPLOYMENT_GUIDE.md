# Production Deployment Guide

**Date**: 2025-11-11  
**Status**: Production-Ready  
**Target**: ParallelRAG System Deployment

---

## 📋 Overview

This guide provides comprehensive instructions for deploying GraphBit's ParallelRAG system in production environments. It covers hardware requirements, configuration, scaling strategies, security, monitoring, and troubleshooting.

---

## 🎯 Quick Start

### **Minimal Production Setup**

```python
import graphbit
import os

# 1. Initialize GraphBit with production runtime settings
graphbit.configure_runtime(
    worker_threads=16,           # 2x CPU cores (8-core machine)
    max_blocking_threads=32,     # 4x CPU cores
    thread_stack_size_mb=1       # 1 MB per thread
)

# 2. Create LLM client
llm_config = graphbit.LlmConfig.openai(
    api_key=os.environ["OPENAI_API_KEY"],
    model="gpt-4o-mini"
)
llm_client = graphbit.LlmClient(llm_config)

# 3. Create embedding client
embed_config = graphbit.EmbeddingConfig.openai(
    api_key=os.environ["OPENAI_API_KEY"],
    model="text-embedding-3-small"
)
embed_client = graphbit.EmbeddingClient(embed_config)

# 4. Process documents in parallel
from concurrent.futures import ThreadPoolExecutor

documents = load_documents()  # Your document loading logic

with ThreadPoolExecutor(max_workers=10) as executor:
    # Parallel chunking
    splitter = graphbit.RecursiveSplitter(chunk_size=1000, chunk_overlap=100)
    chunk_results = list(executor.map(splitter.split_text, documents))
    
    # Parallel embedding
    all_chunks = [chunk for chunks in chunk_results for chunk in chunks]
    chunk_texts = [chunk.content for chunk in all_chunks]
    embeddings = list(executor.map(embed_client.embed, chunk_texts))
    
    # Parallel LLM processing
    queries = generate_queries(all_chunks)  # Your query generation logic
    responses = list(executor.map(
        lambda q: llm_client.complete(q, max_tokens=200),
        queries
    ))

print(f"Processed {len(documents)} documents → {len(all_chunks)} chunks → {len(embeddings)} embeddings")
```

---

## 💻 Hardware Requirements

### **Small Deployment** (1-10 concurrent users)
- **CPU**: 2-4 cores (Intel Xeon, AMD EPYC, or equivalent)
- **RAM**: 4-8 GB
- **Network**: 100 Mbps
- **Storage**: 10 GB SSD
- **Expected Throughput**: 10-50 documents/second

**Runtime Configuration**:
```python
graphbit.configure_runtime(
    worker_threads=4,
    max_blocking_threads=8,
    thread_stack_size_mb=1
)
```

---

### **Medium Deployment** (10-100 concurrent users)
- **CPU**: 8-16 cores (Intel Xeon, AMD EPYC, or equivalent)
- **RAM**: 16-32 GB
- **Network**: 1 Gbps
- **Storage**: 50 GB SSD
- **Expected Throughput**: 100-500 documents/second

**Runtime Configuration**:
```python
graphbit.configure_runtime(
    worker_threads=16,
    max_blocking_threads=32,
    thread_stack_size_mb=1
)
```

---

### **Large Deployment** (100+ concurrent users)
- **CPU**: 32+ cores (Intel Xeon Platinum, AMD EPYC, or equivalent)
- **RAM**: 64+ GB
- **Network**: 10 Gbps
- **Storage**: 200 GB NVMe SSD
- **Expected Throughput**: 1000+ documents/second

**Runtime Configuration**:
```python
graphbit.configure_runtime(
    worker_threads=32,
    max_blocking_threads=64,
    thread_stack_size_mb=2
)
```

---

## 🔐 Environment Variables

### **Required Variables**

```bash
# OpenAI API Key (required for LLM and embeddings)
export OPENAI_API_KEY="sk-proj-..."

# Optional: Logging level
export RUST_LOG="info"  # Options: error, warn, info, debug, trace

# Optional: Python logging
export PYTHONUNBUFFERED=1  # Disable output buffering
```

### **Optional Variables**

```bash
# Anthropic API Key (if using Claude)
export ANTHROPIC_API_KEY="sk-ant-..."

# Groq API Key (if using Groq)
export GROQ_API_KEY="gsk_..."

# Google API Key (if using Gemini)
export GOOGLE_API_KEY="..."

# XAI API Key (if using Grok)
export XAI_API_KEY="..."
```

---

## ⚙️ Configuration Options

### **Runtime Configuration**

```python
import graphbit

# Configure Tokio runtime for production
graphbit.configure_runtime(
    worker_threads=16,           # Number of async worker threads
    max_blocking_threads=32,     # Max threads for blocking operations
    thread_stack_size_mb=1       # Stack size per thread (MB)
)
```

**Parameters**:
- `worker_threads`: Number of async worker threads (default: 2 × CPU cores, max: 32)
- `max_blocking_threads`: Maximum threads for blocking I/O (default: 4 × CPU cores)
- `thread_stack_size_mb`: Stack size per thread in MB (default: 1 MB)

**Guidelines**:
- **CPU-bound workloads**: Set `worker_threads = CPU cores`
- **I/O-bound workloads**: Set `worker_threads = 2-4 × CPU cores`
- **High concurrency**: Increase `max_blocking_threads` to 4-8 × CPU cores

---

### **LLM Client Configuration**

```python
import graphbit

# Production LLM configuration
llm_config = graphbit.LlmConfig.openai(
    api_key=os.environ["OPENAI_API_KEY"],
    model="gpt-4o-mini"  # or "gpt-4o", "gpt-3.5-turbo"
)

llm_client = graphbit.LlmClient(llm_config)
```

**Built-in Resilience** (automatically configured):
- **Request timeout**: 60 seconds (OpenAI), 180 seconds (Ollama)
- **Max retries**: 3 attempts
- **Retry delay**: 100ms - 5s (exponential backoff)
- **Circuit breaker**: Opens after 5 failures, recovers after 60s

---

### **Embedding Client Configuration**

```python
import graphbit

# Production embedding configuration
embed_config = graphbit.EmbeddingConfig.openai(
    api_key=os.environ["OPENAI_API_KEY"],
    model="text-embedding-3-small"  # or "text-embedding-3-large"
)

embed_client = graphbit.EmbeddingClient(embed_config)
```

---

## 📈 Scaling Strategies

### **Vertical Scaling** (Scale Up)

**When to Use**:
- Single-server deployment
- Predictable workload
- Cost-effective for small-medium deployments

**How to Scale**:
1. Increase CPU cores (4 → 8 → 16 → 32)
2. Increase RAM (8 GB → 16 GB → 32 GB → 64 GB)
3. Adjust runtime configuration:
   ```python
   graphbit.configure_runtime(
       worker_threads=32,  # Match CPU cores
       max_blocking_threads=128
   )
   ```

---

### **Horizontal Scaling** (Scale Out)

**When to Use**:
- High availability required
- Unpredictable workload
- Large-scale deployments (100+ concurrent users)

**Architecture**:
```
                    Load Balancer
                         |
        +----------------+----------------+
        |                |                |
   Server 1          Server 2        Server 3
   (GraphBit)        (GraphBit)      (GraphBit)
        |                |                |
        +----------------+----------------+
                         |
                  Shared Storage
                  (Documents, Embeddings)
```

**Implementation**:
1. Deploy multiple GraphBit instances
2. Use load balancer (NGINX, HAProxy, AWS ALB)
3. Share document storage (S3, NFS, etc.)
4. Use distributed cache (Redis) for embeddings

---

### **Hybrid Scaling**

**Best Practice**: Combine vertical and horizontal scaling
- **Vertical**: Scale individual servers to 8-16 cores
- **Horizontal**: Add more servers as needed

---

## 🔒 Security Considerations

### **1. API Key Management**

```python
import os

# ✅ Good: Use environment variables
api_key = os.environ["OPENAI_API_KEY"]

# ❌ Bad: Hardcode API keys
api_key = "sk-proj-..."  # NEVER DO THIS!
```

**Best Practices**:
- Store API keys in environment variables or secrets manager (AWS Secrets Manager, HashiCorp Vault)
- Rotate API keys regularly (every 90 days)
- Use separate API keys for dev/staging/production
- Monitor API key usage for anomalies

---

### **2. Input Validation**

```python
# GraphBit automatically validates inputs
try:
    response = llm_client.complete(user_input, max_tokens=50)
except ValueError as e:
    # Handle validation errors
    return {"error": "Invalid input"}
```

**Built-in Validation**:
- Prompt cannot be empty
- max_tokens must be > 0 and ≤ 100,000
- temperature must be 0.0 - 2.0
- Batch size ≤ 1,000

---

### **3. Rate Limiting**

```python
from time import time, sleep

class RateLimiter:
    """Simple rate limiter for API calls."""
    
    def __init__(self, max_requests_per_minute=60):
        self.max_requests = max_requests_per_minute
        self.requests = []
    
    def wait_if_needed(self):
        now = time()
        # Remove requests older than 1 minute
        self.requests = [t for t in self.requests if now - t < 60]
        
        if len(self.requests) >= self.max_requests:
            # Wait until oldest request expires
            sleep_time = 60 - (now - self.requests[0])
            if sleep_time > 0:
                sleep(sleep_time)
            self.requests = []
        
        self.requests.append(now)

# Usage
rate_limiter = RateLimiter(max_requests_per_minute=60)

for prompt in prompts:
    rate_limiter.wait_if_needed()
    response = llm_client.complete(prompt, max_tokens=50)
```

---

## 📊 Monitoring and Alerting

### **1. Health Checks**

```python
import graphbit

def health_check():
    """Production health check endpoint."""
    try:
        # Check system health
        health = graphbit.health_check()
        
        # Check LLM client
        llm_stats = llm_client.get_stats()
        
        return {
            "status": "healthy" if health.get("status") == "ok" else "unhealthy",
            "uptime_seconds": health.get("uptime_seconds", 0),
            "llm_success_rate": llm_stats["success_rate"],
            "circuit_breaker": llm_stats["circuit_breaker_state"]
        }
    except Exception as e:
        return {
            "status": "unhealthy",
            "error": str(e)
        }
```

---

### **2. Metrics Collection**

```python
# Collect metrics every 60 seconds
import time

while True:
    stats = llm_client.get_stats()
    
    # Log metrics
    print(f"[{time.time()}] LLM Stats:")
    print(f"  Total requests: {stats['total_requests']}")
    print(f"  Success rate: {stats['success_rate']:.1%}")
    print(f"  Avg response time: {stats['average_response_time_ms']:.0f}ms")
    print(f"  Circuit breaker: {stats['circuit_breaker_state']}")
    
    time.sleep(60)
```

See [Production Performance Monitoring](PRODUCTION_PERFORMANCE_MONITORING.md) for comprehensive monitoring patterns.

---

### **3. Alerting Thresholds**

**Critical Alerts** (Immediate Action):
- Success rate < 90%
- Circuit breaker state = Open
- Average response time > 5000ms
- Error rate > 10%

**Warning Alerts** (Monitor Closely):
- Success rate < 95%
- Average response time > 2000ms
- Error rate > 5%
- Memory usage > 80%

---

## 🐛 Troubleshooting Guide

### **Issue 1: High Latency**

**Symptoms**: Average response time > 2000ms

**Causes**:
- Insufficient worker threads
- Network latency
- Large prompts or max_tokens

**Solutions**:
1. Increase worker threads:
   ```python
   graphbit.configure_runtime(worker_threads=32)
   ```
2. Reduce prompt size or max_tokens
3. Check network connectivity to API provider

---

### **Issue 2: Circuit Breaker Open**

**Symptoms**: Requests fail with "Circuit breaker is open"

**Causes**:
- API provider outage
- Invalid API key
- Rate limit exceeded
- Network issues

**Solutions**:
1. Check API provider status
2. Verify API key is valid
3. Wait 60 seconds for circuit breaker recovery
4. Check error logs for root cause

---

### **Issue 3: Out of Memory**

**Symptoms**: Process crashes with OOM error

**Causes**:
- Too many concurrent requests
- Large batch sizes
- Insufficient RAM

**Solutions**:
1. Reduce max_workers in ThreadPoolExecutor
2. Reduce batch sizes
3. Increase RAM (vertical scaling)
4. Reduce thread_stack_size_mb:
   ```python
   graphbit.configure_runtime(thread_stack_size_mb=1)
   ```

---

### **Issue 4: Low Throughput**

**Symptoms**: Processing < 10 documents/second

**Causes**:
- Sequential processing (not using parallelism)
- Insufficient worker threads
- Rate limiting

**Solutions**:
1. Use ThreadPoolExecutor for parallel processing
2. Increase worker threads and max_workers
3. Check rate limits and adjust accordingly

---

## 🎯 Performance Tuning

### **Optimize for Throughput**

```python
# High-throughput configuration
graphbit.configure_runtime(
    worker_threads=32,
    max_blocking_threads=128,
    thread_stack_size_mb=1
)

# Use high concurrency
with ThreadPoolExecutor(max_workers=50) as executor:
    results = list(executor.map(process_document, documents))
```

---

### **Optimize for Latency**

```python
# Low-latency configuration
graphbit.configure_runtime(
    worker_threads=8,
    max_blocking_threads=16,
    thread_stack_size_mb=2
)

# Use moderate concurrency
with ThreadPoolExecutor(max_workers=10) as executor:
    results = list(executor.map(process_document, documents))
```

---

### **Optimize for Memory**

```python
# Memory-constrained configuration
graphbit.configure_runtime(
    worker_threads=4,
    max_blocking_threads=8,
    thread_stack_size_mb=1
)

# Process in smaller batches
batch_size = 10
for i in range(0, len(documents), batch_size):
    batch = documents[i:i+batch_size]
    process_batch(batch)
```

---

## 📚 Related Documentation

- [Production Runtime Configuration](PRODUCTION_RUNTIME_CONFIGURATION.md)
- [Production Performance Monitoring](PRODUCTION_PERFORMANCE_MONITORING.md)
- [Production Error Handling](PRODUCTION_ERROR_HANDLING.md)
- [Production Readiness Checklist](PRODUCTION_READINESS_CHECKLIST.md)

---

## 🎉 Summary

**Key Takeaways**:
1. ✅ **Hardware**: Scale vertically (2-32 cores) or horizontally (multiple servers)
2. ✅ **Configuration**: Adjust runtime settings based on workload
3. ✅ **Security**: Use environment variables, validate inputs, implement rate limiting
4. ✅ **Monitoring**: Track health, metrics, and set up alerts
5. ✅ **Troubleshooting**: Follow systematic approach to diagnose and fix issues

**Production-Ready**: GraphBit's ParallelRAG system is battle-tested and ready for production deployment with comprehensive documentation and support.

