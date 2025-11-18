# Production Error Handling Guide

**Date**: 2025-11-11  
**Status**: Production-Ready  
**Target**: ParallelRAG System Error Handling

---

## 📋 Overview

GraphBit includes comprehensive, production-grade error handling with circuit breakers, automatic retry logic with exponential backoff, timeout handling, and graceful degradation. This guide documents the built-in resilience patterns and provides best practices for production deployments.

---

## 🛡️ Built-in Resilience Patterns

### **1. Circuit Breaker Pattern**

GraphBit's `LlmClient` includes an automatic circuit breaker that prevents cascading failures:

**How It Works**:
1. **Closed State**: Normal operation, requests pass through
2. **Open State**: After N failures, circuit opens and rejects requests
3. **Half-Open State**: After recovery timeout, allows test requests
4. **Auto-Recovery**: Successful test request closes the circuit

**Default Configuration**:
```python
# Built-in defaults (automatically applied)
circuit_breaker_enabled = True
circuit_breaker_threshold = 5  # Open after 5 consecutive failures
circuit_breaker_recovery_timeout = 60  # seconds
```

**Monitoring Circuit Breaker**:
```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

# Check circuit breaker state
stats = llm_client.get_stats()
print(f"Circuit breaker: {stats['circuit_breaker_state']}")
# Output: "Closed (failures: 0)" | "Open" | "HalfOpen"
```

**States Explained**:
- `Closed (failures: N)`: Circuit is closed, N consecutive failures recorded
- `Open`: Circuit is open, requests are rejected (prevents cascading failures)
- `HalfOpen`: Testing recovery, allowing limited requests

---

### **2. Automatic Retry with Exponential Backoff**

All LLM requests automatically retry on failure with exponential backoff:

**Default Configuration**:
```python
# Built-in defaults (automatically applied)
max_retries = 3
base_retry_delay = 100  # milliseconds
max_retry_delay = 5000  # milliseconds (5 seconds)
```

**Retry Behavior**:
- **Attempt 1**: Immediate
- **Attempt 2**: Wait 100ms
- **Attempt 3**: Wait 200ms
- **Attempt 4**: Wait 400ms
- **Max delay**: Capped at 5 seconds

**Example**:
```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

try:
    # Automatically retries up to 3 times on failure
    response = llm_client.complete("Test prompt", max_tokens=50)
except Exception as e:
    # Only raised after all retries exhausted
    print(f"Request failed after retries: {e}")
```

---

### **3. Timeout Handling**

All requests have configurable timeouts to prevent hanging:

**Default Timeouts** (Provider-Specific):
- **OpenAI, Anthropic, Groq, Gemini, XAI**: 60 seconds
- **Ollama** (local inference): 180 seconds
- **Other providers**: 120 seconds

**Timeout Behavior**:
```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

try:
    # Automatically times out after 60 seconds
    response = llm_client.complete("Long prompt...", max_tokens=1000)
except Exception as e:
    if "timed out" in str(e).lower():
        print("Request timed out after 60 seconds")
```

---

### **4. Input Validation**

All inputs are validated before execution:

**Validation Rules**:
- `prompt`: Cannot be empty
- `max_tokens`: Must be > 0 and ≤ 100,000
- `temperature`: Must be between 0.0 and 2.0
- `batch size`: Cannot exceed 1,000 prompts

**Example**:
```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

try:
    # Invalid: empty prompt
    response = llm_client.complete("", max_tokens=50)
except ValueError as e:
    print(f"Validation error: {e}")
    # Output: "Validation error: prompt cannot be empty"

try:
    # Invalid: temperature out of range
    response = llm_client.complete("Test", temperature=3.0)
except ValueError as e:
    print(f"Validation error: {e}")
    # Output: "Validation error: temperature must be between 0.0 and 2.0"
```

---

## 🎯 Error Handling Patterns

### **Pattern 1: Basic Error Handling**

```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

try:
    response = llm_client.complete("Test prompt", max_tokens=50)
    print(f"Success: {response}")
except ValueError as e:
    # Input validation errors
    print(f"Invalid input: {e}")
except RuntimeError as e:
    # Circuit breaker open or request failures
    if "Circuit breaker is open" in str(e):
        print("Service is temporarily unavailable (circuit breaker open)")
    else:
        print(f"Request failed: {e}")
except Exception as e:
    # Unexpected errors
    print(f"Unexpected error: {e}")
```

---

### **Pattern 2: Graceful Degradation**

```python
import graphbit

class RobustRAGPipeline:
    """RAG pipeline with graceful degradation."""
    
    def __init__(self, llm_client, embed_client, fallback_response="Unable to process request"):
        self.llm_client = llm_client
        self.embed_client = embed_client
        self.fallback_response = fallback_response
    
    def process_query(self, query, context_chunks):
        """Process query with graceful degradation."""
        try:
            # Try to generate embeddings
            embeddings = []
            for chunk in context_chunks:
                try:
                    embedding = self.embed_client.embed(chunk)
                    embeddings.append(embedding)
                except Exception as e:
                    print(f"Warning: Failed to embed chunk: {e}")
                    # Continue with partial embeddings
            
            if not embeddings:
                print("Warning: No embeddings generated, using fallback")
                return self.fallback_response
            
            # Try to generate LLM response
            try:
                prompt = f"Context: {context_chunks[0]}\n\nQuery: {query}"
                response = self.llm_client.complete(prompt, max_tokens=200)
                return response
            except RuntimeError as e:
                if "Circuit breaker is open" in str(e):
                    print("LLM service unavailable, using fallback")
                    return self.fallback_response
                raise
                
        except Exception as e:
            print(f"Pipeline error: {e}")
            return self.fallback_response

# Usage
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

embed_config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-3-small")
embed_client = graphbit.EmbeddingClient(embed_config)

pipeline = RobustRAGPipeline(llm_client, embed_client)
result = pipeline.process_query("What is AI?", ["AI is artificial intelligence..."])
print(result)
```

---

### **Pattern 3: Retry with Custom Logic**

```python
import graphbit
import time

def retry_with_custom_logic(func, max_attempts=5, base_delay=1.0):
    """Custom retry logic with exponential backoff."""
    for attempt in range(max_attempts):
        try:
            return func()
        except RuntimeError as e:
            if "Circuit breaker is open" in str(e):
                # Circuit breaker open - wait longer
                delay = base_delay * (2 ** attempt)
                print(f"Circuit breaker open, waiting {delay}s before retry {attempt + 1}/{max_attempts}")
                time.sleep(delay)
            else:
                # Other runtime error - re-raise
                raise
        except Exception as e:
            # Unexpected error - re-raise
            raise
    
    raise RuntimeError(f"Failed after {max_attempts} attempts")

# Usage
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

result = retry_with_custom_logic(
    lambda: llm_client.complete("Test prompt", max_tokens=50),
    max_attempts=5,
    base_delay=2.0
)
```

---

### **Pattern 4: Parallel Execution with Error Handling**

```python
import graphbit
from concurrent.futures import ThreadPoolExecutor, as_completed

def process_documents_robust(documents, llm_client, max_workers=10):
    """Process documents in parallel with robust error handling."""
    results = []
    errors = []
    
    def process_single(doc):
        try:
            return llm_client.complete(f"Summarize: {doc}", max_tokens=100)
        except Exception as e:
            return {"error": str(e), "document": doc[:50]}
    
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(process_single, doc): doc for doc in documents}
        
        for future in as_completed(futures):
            doc = futures[future]
            try:
                result = future.result()
                if isinstance(result, dict) and "error" in result:
                    errors.append(result)
                else:
                    results.append(result)
            except Exception as e:
                errors.append({"error": str(e), "document": doc[:50]})
    
    return {
        "results": results,
        "errors": errors,
        "success_rate": len(results) / len(documents) if documents else 0
    }

# Usage
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

documents = ["Doc 1...", "Doc 2...", "Doc 3..."]
result = process_documents_robust(documents, llm_client)

print(f"Processed: {len(result['results'])}/{len(documents)}")
print(f"Errors: {len(result['errors'])}")
print(f"Success rate: {result['success_rate']:.1%}")
```

---

## 🚨 Error Types and Handling

### **1. Validation Errors** (`ValueError`)

**Causes**:
- Empty prompt
- Invalid max_tokens (≤ 0 or > 100,000)
- Invalid temperature (< 0.0 or > 2.0)
- Empty batch
- Batch size > 1,000

**Handling**:
```python
try:
    response = llm_client.complete("", max_tokens=50)
except ValueError as e:
    print(f"Fix input: {e}")
    # Correct the input and retry
```

---

### **2. Circuit Breaker Errors** (`RuntimeError`)

**Causes**:
- Circuit breaker is open (too many failures)

**Handling**:
```python
try:
    response = llm_client.complete("Test", max_tokens=50)
except RuntimeError as e:
    if "Circuit breaker is open" in str(e):
        # Wait for recovery timeout (60 seconds default)
        print("Service temporarily unavailable, try again later")
        time.sleep(60)
        # Retry after recovery timeout
```

---

### **3. Timeout Errors** (`RuntimeError`)

**Causes**:
- Request exceeded timeout (60-180 seconds depending on provider)

**Handling**:
```python
try:
    response = llm_client.complete("Very long prompt...", max_tokens=5000)
except RuntimeError as e:
    if "timed out" in str(e).lower():
        print("Request took too long, try with shorter prompt or lower max_tokens")
```

---

### **4. API Errors** (`RuntimeError`)

**Causes**:
- Invalid API key
- Rate limit exceeded
- Network errors
- Provider-specific errors

**Handling**:
```python
try:
    response = llm_client.complete("Test", max_tokens=50)
except RuntimeError as e:
    error_msg = str(e).lower()
    if "authentication" in error_msg or "api key" in error_msg:
        print("Invalid API key")
    elif "rate limit" in error_msg:
        print("Rate limited, wait before retrying")
        time.sleep(60)
    elif "network" in error_msg:
        print("Network error, check connection")
    else:
        print(f"API error: {e}")
```

---

## 📊 Monitoring Error Rates

### **Track Error Metrics**

```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

# Perform operations
for i in range(100):
    try:
        llm_client.complete(f"Prompt {i}", max_tokens=50)
    except Exception:
        pass  # Errors are tracked automatically

# Check error metrics
stats = llm_client.get_stats()
print(f"Total requests:     {stats['total_requests']}")
print(f"Successful:         {stats['successful_requests']}")
print(f"Failed:             {stats['failed_requests']}")
print(f"Success rate:       {stats['success_rate']:.1%}")
print(f"Circuit breaker:    {stats['circuit_breaker_state']}")
```

---

## ⚠️ Production Best Practices

### **1. Always Handle Errors**
```python
# ❌ Bad: No error handling
response = llm_client.complete("Test", max_tokens=50)

# ✅ Good: Comprehensive error handling
try:
    response = llm_client.complete("Test", max_tokens=50)
except ValueError as e:
    # Handle validation errors
    pass
except RuntimeError as e:
    # Handle runtime errors (circuit breaker, timeouts, API errors)
    pass
```

### **2. Monitor Circuit Breaker State**
```python
# Check circuit breaker before critical operations
stats = llm_client.get_stats()
if "Open" in stats['circuit_breaker_state']:
    print("Warning: Circuit breaker is open, service may be degraded")
```

### **3. Implement Graceful Degradation**
```python
# Provide fallback responses when services are unavailable
try:
    response = llm_client.complete(prompt, max_tokens=50)
except RuntimeError:
    response = "Service temporarily unavailable. Please try again later."
```

### **4. Log Errors with Context**
```python
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

try:
    response = llm_client.complete(prompt, max_tokens=50)
except Exception as e:
    logger.error(f"LLM request failed: {e}", extra={
        "prompt_length": len(prompt),
        "max_tokens": 50,
        "circuit_breaker": llm_client.get_stats()['circuit_breaker_state']
    })
```

### **5. Set Up Alerts**
```python
# Alert when error rate exceeds threshold
stats = llm_client.get_stats()
if stats['success_rate'] < 0.90:  # < 90% success rate
    send_alert(f"High error rate: {stats['success_rate']:.1%}")

if "Open" in stats['circuit_breaker_state']:
    send_alert("Circuit breaker is OPEN - service degraded")
```

---

## 🎯 Summary

**Built-in Resilience Features**:
1. ✅ **Circuit Breaker**: Automatic failure detection and recovery
2. ✅ **Retry Logic**: Exponential backoff with 3 retries
3. ✅ **Timeout Handling**: Provider-specific timeouts (60-180s)
4. ✅ **Input Validation**: Comprehensive validation before execution
5. ✅ **Error Tracking**: Automatic statistics collection

**Production-Ready**: GraphBit's error handling is comprehensive, battle-tested, and requires no additional configuration for production use.

---

## 📚 Related Documentation

- [Production Runtime Configuration](PRODUCTION_RUNTIME_CONFIGURATION.md)
- [Production Performance Monitoring](PRODUCTION_PERFORMANCE_MONITORING.md)
- [Production Deployment Guide](PRODUCTION_DEPLOYMENT_GUIDE.md)

