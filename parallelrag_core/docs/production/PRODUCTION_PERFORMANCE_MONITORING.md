# Production Performance Monitoring Guide

**Date**: 2025-11-11  
**Status**: Production-Ready  
**Target**: ParallelRAG System Monitoring

---

## 📋 Overview

This guide provides comprehensive performance monitoring strategies for GraphBit's ParallelRAG system in production. GraphBit includes built-in metrics collection for all major components, making it easy to track performance, identify bottlenecks, and optimize resource utilization.

---

## 🎯 Quick Start

### **Basic Monitoring Setup**

```python
import graphbit
import time

# Initialize GraphBit
graphbit.init()

# Create clients
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

# Perform operations
response = llm_client.complete("Test prompt", max_tokens=50)

# Get statistics
stats = llm_client.get_stats()
print(f"Total requests: {stats['total_requests']}")
print(f"Success rate: {stats['success_rate']:.2%}")
print(f"Avg response time: {stats['average_response_time_ms']:.2f}ms")
print(f"Circuit breaker: {stats['circuit_breaker_state']}")
```

---

## 📊 Built-in Metrics

### **1. LLM Client Metrics**

GraphBit's `LlmClient` automatically tracks comprehensive statistics:

```python
import graphbit

llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

# Perform operations
for i in range(100):
    llm_client.complete(f"Prompt {i}", max_tokens=50)

# Get detailed statistics
stats = llm_client.get_stats()

print("=== LLM Client Statistics ===")
print(f"Total requests:       {stats['total_requests']}")
print(f"Successful requests:  {stats['successful_requests']}")
print(f"Failed requests:      {stats['failed_requests']}")
print(f"Success rate:         {stats['success_rate']:.2%}")
print(f"Avg response time:    {stats['average_response_time_ms']:.2f}ms")
print(f"Circuit breaker:      {stats['circuit_breaker_state']}")
print(f"Uptime:               {stats['uptime_seconds']}s")
```

**Available Metrics**:
- `total_requests`: Total number of API calls
- `successful_requests`: Number of successful calls
- `failed_requests`: Number of failed calls
- `success_rate`: Success rate (0.0 - 1.0)
- `average_response_time_ms`: Average response time in milliseconds
- `circuit_breaker_state`: Circuit breaker status (Closed/Open/HalfOpen)
- `uptime_seconds`: Client uptime in seconds

---

### **2. Embedding Client Metrics**

The `EmbeddingClient` provides batch processing statistics:

```python
import graphbit

embed_config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-3-small")
embed_client = graphbit.EmbeddingClient(embed_config)

# Batch processing with statistics
texts_batch = [
    ["text1", "text2", "text3"],
    ["text4", "text5", "text6"],
    ["text7", "text8", "text9"]
]

result = embed_client.embed_batch_parallel(texts_batch, max_concurrency=3)

print("=== Embedding Batch Statistics ===")
print(f"Duration:             {result['duration_ms']}ms")
print(f"Successful requests:  {result['stats']['successful_requests']}")
print(f"Failed requests:      {result['stats']['failed_requests']}")
print(f"Avg response time:    {result['stats']['avg_response_time_ms']:.2f}ms")
print(f"Total embeddings:     {result['stats']['total_embeddings']}")
print(f"Total tokens:         {result['stats']['total_tokens']}")
```

**Available Metrics**:
- `duration_ms`: Total batch processing time
- `successful_requests`: Number of successful batch requests
- `failed_requests`: Number of failed batch requests
- `avg_response_time_ms`: Average response time per batch
- `total_embeddings`: Total number of embeddings generated
- `total_tokens`: Total tokens processed

---

### **3. System Health Metrics**

Monitor overall system health:

```python
import graphbit

graphbit.init()

# Get system information
system_info = graphbit.get_system_info()
print("=== System Information ===")
print(f"GraphBit version:     {system_info.get('version', 'N/A')}")
print(f"Worker threads:       {system_info.get('runtime_worker_threads', 'N/A')}")
print(f"Max blocking threads: {system_info.get('runtime_max_blocking_threads', 'N/A')}")
print(f"Memory allocator:     {system_info.get('memory_allocator', 'N/A')}")

# Health check
health = graphbit.health_check()
print("\n=== Health Check ===")
print(f"Status:               {health.get('status', 'unknown')}")
print(f"Uptime:               {health.get('uptime_seconds', 0)}s")
```

---

## 🔍 Production Monitoring Patterns

### **Pattern 1: Continuous Monitoring Loop**

```python
import graphbit
import time
import json
from datetime import datetime

class ProductionMonitor:
    """Continuous monitoring for production ParallelRAG system."""
    
    def __init__(self, llm_client, embed_client=None, interval_seconds=60):
        self.llm_client = llm_client
        self.embed_client = embed_client
        self.interval = interval_seconds
        self.metrics_history = []
    
    def collect_metrics(self):
        """Collect current metrics from all clients."""
        timestamp = datetime.now().isoformat()
        
        metrics = {
            "timestamp": timestamp,
            "llm": self.llm_client.get_stats(),
        }
        
        if self.embed_client:
            # Note: EmbeddingClient doesn't have get_stats(), 
            # metrics are returned per batch operation
            metrics["embedding"] = {"note": "Metrics collected per batch"}
        
        # Get system health
        metrics["system"] = graphbit.health_check()
        
        return metrics
    
    def monitor_loop(self, duration_seconds=None):
        """Run continuous monitoring loop."""
        start_time = time.time()
        
        print(f"Starting production monitoring (interval: {self.interval}s)")
        
        try:
            while True:
                # Collect metrics
                metrics = self.collect_metrics()
                self.metrics_history.append(metrics)
                
                # Log metrics
                self.log_metrics(metrics)
                
                # Check if duration limit reached
                if duration_seconds and (time.time() - start_time) >= duration_seconds:
                    break
                
                # Wait for next interval
                time.sleep(self.interval)
                
        except KeyboardInterrupt:
            print("\nMonitoring stopped by user")
    
    def log_metrics(self, metrics):
        """Log metrics to console (replace with your logging system)."""
        llm_stats = metrics["llm"]
        print(f"\n[{metrics['timestamp']}] Metrics:")
        print(f"  LLM Requests: {llm_stats['total_requests']} "
              f"(Success: {llm_stats['success_rate']:.1%}, "
              f"Avg: {llm_stats['average_response_time_ms']:.0f}ms)")
        print(f"  Circuit Breaker: {llm_stats['circuit_breaker_state']}")
        print(f"  System Status: {metrics['system'].get('status', 'unknown')}")
    
    def export_metrics(self, filename="metrics.json"):
        """Export collected metrics to JSON file."""
        with open(filename, 'w') as f:
            json.dump(self.metrics_history, f, indent=2)
        print(f"Metrics exported to {filename}")

# Usage
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

monitor = ProductionMonitor(llm_client, interval_seconds=30)
monitor.monitor_loop(duration_seconds=300)  # Monitor for 5 minutes
monitor.export_metrics("production_metrics.json")
```

---

### **Pattern 2: Prometheus Metrics Exporter**

```python
import graphbit
from prometheus_client import Counter, Gauge, Histogram, start_http_server
import time

class PrometheusMetricsExporter:
    """Export GraphBit metrics to Prometheus."""
    
    def __init__(self, llm_client, port=8000):
        self.llm_client = llm_client
        self.port = port
        
        # Define Prometheus metrics
        self.llm_requests_total = Counter(
            'graphbit_llm_requests_total',
            'Total number of LLM requests'
        )
        self.llm_requests_success = Counter(
            'graphbit_llm_requests_success',
            'Number of successful LLM requests'
        )
        self.llm_requests_failed = Counter(
            'graphbit_llm_requests_failed',
            'Number of failed LLM requests'
        )
        self.llm_response_time = Histogram(
            'graphbit_llm_response_time_ms',
            'LLM response time in milliseconds',
            buckets=[10, 50, 100, 250, 500, 1000, 2500, 5000, 10000]
        )
        self.llm_success_rate = Gauge(
            'graphbit_llm_success_rate',
            'LLM success rate (0.0 - 1.0)'
        )
        self.circuit_breaker_state = Gauge(
            'graphbit_circuit_breaker_open',
            'Circuit breaker state (1=open, 0=closed)'
        )
        
        # Track last known values to calculate deltas
        self.last_stats = None
    
    def update_metrics(self):
        """Update Prometheus metrics from GraphBit stats."""
        stats = self.llm_client.get_stats()
        
        if self.last_stats:
            # Calculate deltas
            new_requests = stats['total_requests'] - self.last_stats['total_requests']
            new_success = stats['successful_requests'] - self.last_stats['successful_requests']
            new_failed = stats['failed_requests'] - self.last_stats['failed_requests']
            
            # Update counters
            self.llm_requests_total.inc(new_requests)
            self.llm_requests_success.inc(new_success)
            self.llm_requests_failed.inc(new_failed)
        
        # Update gauges
        self.llm_success_rate.set(stats['success_rate'])
        self.llm_response_time.observe(stats['average_response_time_ms'])
        
        # Circuit breaker state
        cb_open = 1 if 'Open' in stats['circuit_breaker_state'] else 0
        self.circuit_breaker_state.set(cb_open)
        
        self.last_stats = stats
    
    def start(self, update_interval=15):
        """Start Prometheus metrics server."""
        # Start HTTP server for Prometheus scraping
        start_http_server(self.port)
        print(f"Prometheus metrics server started on port {self.port}")
        print(f"Metrics available at http://localhost:{self.port}/metrics")
        
        try:
            while True:
                self.update_metrics()
                time.sleep(update_interval)
        except KeyboardInterrupt:
            print("\nMetrics exporter stopped")

# Usage
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

exporter = PrometheusMetricsExporter(llm_client, port=8000)
exporter.start(update_interval=15)  # Update every 15 seconds
```

**Prometheus Configuration** (`prometheus.yml`):
```yaml
scrape_configs:
  - job_name: 'graphbit'
    scrape_interval: 15s
    static_configs:
      - targets: ['localhost:8000']
```

---

### **Pattern 3: ParallelRAG Pipeline Monitoring**

```python
import graphbit
from concurrent.futures import ThreadPoolExecutor
import time

class ParallelRAGMonitor:
    """Monitor complete ParallelRAG pipeline performance."""
    
    def __init__(self, splitter, embed_client, llm_client):
        self.splitter = splitter
        self.embed_client = embed_client
        self.llm_client = llm_client
        self.pipeline_metrics = {
            "total_documents": 0,
            "total_chunks": 0,
            "total_embeddings": 0,
            "total_llm_calls": 0,
            "chunking_time_ms": 0,
            "embedding_time_ms": 0,
            "llm_time_ms": 0,
            "total_time_ms": 0
        }
    
    def process_documents(self, documents, max_workers=10):
        """Process documents through complete RAG pipeline with monitoring."""
        start_time = time.time()
        
        # Stage 1: Chunking
        chunking_start = time.time()
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            chunk_results = list(executor.map(self.splitter.split_text, documents))
        all_chunks = [chunk for chunks in chunk_results for chunk in chunks]
        chunking_time = (time.time() - chunking_start) * 1000
        
        # Stage 2: Embedding
        embedding_start = time.time()
        chunk_texts = [chunk.content for chunk in all_chunks]
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            embeddings = list(executor.map(self.embed_client.embed, chunk_texts))
        embedding_time = (time.time() - embedding_start) * 1000
        
        # Stage 3: LLM (sample queries)
        llm_start = time.time()
        sample_queries = [f"Query about: {chunk.content[:50]}" for chunk in all_chunks[:5]]
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            responses = list(executor.map(
                lambda q: self.llm_client.complete(q, max_tokens=50),
                sample_queries
            ))
        llm_time = (time.time() - llm_start) * 1000
        
        total_time = (time.time() - start_time) * 1000
        
        # Update metrics
        self.pipeline_metrics["total_documents"] += len(documents)
        self.pipeline_metrics["total_chunks"] += len(all_chunks)
        self.pipeline_metrics["total_embeddings"] += len(embeddings)
        self.pipeline_metrics["total_llm_calls"] += len(responses)
        self.pipeline_metrics["chunking_time_ms"] += chunking_time
        self.pipeline_metrics["embedding_time_ms"] += embedding_time
        self.pipeline_metrics["llm_time_ms"] += llm_time
        self.pipeline_metrics["total_time_ms"] += total_time
        
        return {
            "chunks": all_chunks,
            "embeddings": embeddings,
            "responses": responses,
            "metrics": {
                "chunking_time_ms": chunking_time,
                "embedding_time_ms": embedding_time,
                "llm_time_ms": llm_time,
                "total_time_ms": total_time,
                "throughput_docs_per_sec": len(documents) / (total_time / 1000)
            }
        }
    
    def get_summary(self):
        """Get pipeline performance summary."""
        m = self.pipeline_metrics
        return {
            "total_documents": m["total_documents"],
            "total_chunks": m["total_chunks"],
            "total_embeddings": m["total_embeddings"],
            "total_llm_calls": m["total_llm_calls"],
            "avg_chunking_time_ms": m["chunking_time_ms"] / max(m["total_documents"], 1),
            "avg_embedding_time_ms": m["embedding_time_ms"] / max(m["total_embeddings"], 1),
            "avg_llm_time_ms": m["llm_time_ms"] / max(m["total_llm_calls"], 1),
            "total_pipeline_time_ms": m["total_time_ms"],
            "llm_client_stats": self.llm_client.get_stats()
        }

# Usage
graphbit.init()

splitter = graphbit.CharacterSplitter(chunk_size=500, chunk_overlap=50)
embed_config = graphbit.EmbeddingConfig.openai(api_key, "text-embedding-3-small")
embed_client = graphbit.EmbeddingClient(embed_config)
llm_config = graphbit.LlmConfig.openai(api_key, "gpt-4o-mini")
llm_client = graphbit.LlmClient(llm_config)

monitor = ParallelRAGMonitor(splitter, embed_client, llm_client)

# Process documents
documents = ["Document 1 content...", "Document 2 content...", "Document 3 content..."]
result = monitor.process_documents(documents, max_workers=10)

print("=== Pipeline Metrics ===")
print(f"Chunking time:   {result['metrics']['chunking_time_ms']:.2f}ms")
print(f"Embedding time:  {result['metrics']['embedding_time_ms']:.2f}ms")
print(f"LLM time:        {result['metrics']['llm_time_ms']:.2f}ms")
print(f"Total time:      {result['metrics']['total_time_ms']:.2f}ms")
print(f"Throughput:      {result['metrics']['throughput_docs_per_sec']:.2f} docs/sec")

# Get cumulative summary
summary = monitor.get_summary()
print("\n=== Cumulative Summary ===")
print(f"Total documents:  {summary['total_documents']}")
print(f"Total chunks:     {summary['total_chunks']}")
print(f"Total embeddings: {summary['total_embeddings']}")
print(f"LLM success rate: {summary['llm_client_stats']['success_rate']:.2%}")
```

---

## 📈 Key Performance Indicators (KPIs)

### **1. Throughput Metrics**
- **Documents/second**: Number of documents processed per second
- **Chunks/second**: Number of text chunks generated per second
- **Embeddings/second**: Number of embeddings generated per second
- **LLM calls/second**: Number of LLM API calls per second

### **2. Latency Metrics**
- **p50 latency**: Median response time
- **p95 latency**: 95th percentile response time
- **p99 latency**: 99th percentile response time
- **Average response time**: Mean response time

### **3. Reliability Metrics**
- **Success rate**: Percentage of successful requests
- **Error rate**: Percentage of failed requests
- **Circuit breaker trips**: Number of circuit breaker activations
- **Retry attempts**: Number of retry attempts

### **4. Resource Metrics**
- **CPU utilization**: Percentage of CPU used
- **Memory usage**: RAM consumption
- **Thread pool saturation**: Active threads / max threads
- **Network I/O**: Bytes sent/received

---

## ⚠️ Alerting Thresholds

### **Critical Alerts** (Immediate Action Required)

```python
# Example alert conditions
def check_critical_alerts(stats):
    alerts = []
    
    # Success rate below 90%
    if stats['success_rate'] < 0.90:
        alerts.append(f"CRITICAL: Success rate {stats['success_rate']:.1%} < 90%")
    
    # Circuit breaker open
    if 'Open' in stats['circuit_breaker_state']:
        alerts.append("CRITICAL: Circuit breaker is OPEN")
    
    # Average response time > 5 seconds
    if stats['average_response_time_ms'] > 5000:
        alerts.append(f"CRITICAL: Avg response time {stats['average_response_time_ms']:.0f}ms > 5000ms")
    
    return alerts
```

**Recommended Thresholds**:
- Success rate < 90%
- Circuit breaker state = Open
- Average response time > 5000ms
- Error rate > 10%

### **Warning Alerts** (Monitor Closely)

**Recommended Thresholds**:
- Success rate < 95%
- Average response time > 2000ms
- Error rate > 5%
- Memory usage > 80%

---

## 🎯 Best Practices

### **1. Monitor Continuously**
- Collect metrics every 15-60 seconds
- Store metrics for historical analysis
- Set up automated alerting

### **2. Track Trends**
- Monitor metrics over time (hourly, daily, weekly)
- Identify performance degradation early
- Correlate metrics with deployments

### **3. Use Dashboards**
- Create visual dashboards (Grafana, Datadog, etc.)
- Display key metrics prominently
- Enable drill-down for detailed analysis

### **4. Reset Statistics Periodically**
```python
# Reset LLM client statistics
llm_client.reset_stats()
```

---

## 📚 Related Documentation

- [Production Runtime Configuration](PRODUCTION_RUNTIME_CONFIGURATION.md)
- [Production Deployment Guide](PRODUCTION_DEPLOYMENT_GUIDE.md)
- [Monitoring Guide](docs/user-guide/monitoring.md)

---

## 🎉 Summary

**Key Takeaways**:
1. ✅ **Built-in metrics** for LLM, embedding, and system health
2. ✅ **Prometheus integration** for production monitoring
3. ✅ **Pipeline monitoring** for end-to-end performance tracking
4. ✅ **Alerting thresholds** for proactive issue detection
5. ✅ **Best practices** for continuous monitoring

**Production-Ready**: GraphBit's monitoring infrastructure is comprehensive and battle-tested for production ParallelRAG deployments.

