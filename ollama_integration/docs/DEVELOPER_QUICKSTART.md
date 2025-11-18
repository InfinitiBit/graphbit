# 🦙 Ollama Developer Quick Start Guide

**Last Updated**: November 17, 2025
**Validated On**: Windows 11, Intel Core i9 (20 cores), 32GB RAM
**Ollama Version**: Latest
**Python Version**: 3.13.3

---

## 📋 Prerequisites

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **Python** | 3.9+ | 3.11+ |
| **RAM** | 8 GB | 16 GB+ |
| **Disk Space** | 10 GB | 20 GB+ |
| **CPU** | 4 cores | 8+ cores |
| **GPU** | None (CPU works) | NVIDIA RTX 3060+ (10x faster) |
| **OS** | Windows 10/11, macOS 11+, Linux | Any |

### Required Software

- ✅ Python 3.9 or higher
- ✅ pip (Python package manager)
- ✅ Git (for cloning repository)
- ✅ Ollama (local LLM server)

---

## 🚀 Installation Steps

### Step 1: Install Ollama

#### Windows
```powershell
# Download and run installer from https://ollama.com/download
# Or use winget
winget install Ollama.Ollama
```

#### macOS
```bash
# Download and run installer from https://ollama.com/download
# Or use Homebrew
brew install ollama
```

#### Linux
```bash
curl -fsSL https://ollama.com/install.sh | sh
```

### Step 2: Start Ollama Service

#### Windows
```powershell
# Ollama starts automatically after installation
# Check if running:
curl http://localhost:11434/api/tags
```

#### macOS/Linux
```bash
# Start Ollama service
ollama serve

# Or run in background
nohup ollama serve > ollama.log 2>&1 &

# Check if running
curl http://localhost:11434/api/tags
```

### Step 3: Pull Required Models

```bash
# Pull embedding model (274 MB)
ollama pull nomic-embed-text

# Pull LLM model (3.3 GB for gemma3:4b)
ollama pull gemma3:4b

# Verify models are available
ollama list
```

**Expected Output**:
```
NAME                    ID              SIZE      MODIFIED
gemma3:4b              abc123def456    3.3 GB    2 minutes ago
nomic-embed-text       def789ghi012    274 MB    5 minutes ago
```

### Step 4: Install Python Dependencies

```bash
# Clone repository (if not already done)
git clone https://github.com/your-org/graphbit.git
cd graphbit

# Create virtual environment
python -m venv venv

# Activate virtual environment
# Windows PowerShell:
.\venv\Scripts\Activate.ps1
# Windows Command Prompt:
.\venv\Scripts\activate.bat
# macOS/Linux:
source venv/bin/activate

# Install GraphBit
pip install graphbit

# Install LangChain dependencies
pip install langchain langchain-community faiss-cpu

# Install additional dependencies
pip install requests psutil
```

### Step 5: Verify Installation

```bash
# Test Ollama connection
python -c "import requests; print('✅ Ollama running' if requests.get('http://localhost:11434/api/tags').status_code == 200 else '❌ Ollama not running')"

# Test GraphBit import
python -c "import graphbit; print('✅ GraphBit installed')"

# Test LangChain import
python -c "import langchain; print('✅ LangChain installed')"
```

---

## 🎯 Running Demos

### Demo 1: GraphBit with Ollama

**Command**:
```bash
python examples/parallel_rag_ollama.py
```

**Expected Output**:
```text
================================================================================
ParallelRAG with Ollama: Local Document Intelligence
================================================================================
✅ Initialized ParallelRAG with Ollama:
   LLM: gemma3:4b
   Embeddings: nomic-embed-text
   Ollama URL: http://localhost:11434
   Workers: 10
Loading 5 documents in parallel...
 Loaded 5 documents in 0.03s
   Average: 0.005s per document
Chunking 5 documents in parallel...
 Created 5 chunks in 0.00s
Generating embeddings for 5 chunks using Ollama...
 Generated 5 embeddings in 12.92s
   Average: 2.584s per embedding
Storing 5 chunks in vector store...
 Stored 5 chunks

Query: What is machine learning?
Response: Machine learning is a subset of artificial intelligence that enables
systems to learn from data.

================================================================================
 ParallelRAG with Ollama processing complete!
================================================================================
```

**Performance**: ~13 seconds for 5 documents (CPU)

---

### Demo 2: LangChain with Ollama

**Command**:
```bash
python langchain_rag_ollama.py
```

**Expected Output**:
```text
================================================================================
LangChain RAG with Ollama: Local Document Intelligence
================================================================================
✅ Initialized LangChain RAG with Ollama:
   LLM: gemma3:4b
   Embeddings: nomic-embed-text
   Ollama URL: http://localhost:11434
Processing 5 documents...
 Loaded 5 documents in 0.11s
 Created 5 chunks
 Generated 5 embeddings in 10.78s
 Stored 5 chunks in FAISS vector store

Query: What is machine learning?
Response: Machine learning is a subset of artificial intelligence that enables
systems to learn and improve from experience without being explicitly programmed...

================================================================================
 LangChain RAG with Ollama processing complete!
================================================================================
```

**Performance**: ~11 seconds for 5 documents (CPU)

---

### Demo 3: Framework Comparison Benchmark

**Command**:
```bash
python tests/benchmarks/benchmark_ollama_comparison.py --framework both --max-docs 5 --max-workers 5
```

**Expected Output**:
```
================================================================================
Framework Comparison Benchmark with Ollama: GraphBit vs LangChain
================================================================================

✅ Ollama is running at http://localhost:11434
✅ LLM model 'gemma3:4b' is available
✅ Embedding model 'nomic-embed-text' is available

Testing GraphBit ParallelRAG with Ollama
 GraphBit Results:
  Total Time: 46.08s
  Throughput: 0.11 docs/sec
  Peak Memory: 87.77 MB

Testing LangChain RAG with Ollama
 LangChain Results:
  Total Time: 57.68s
  Throughput: 0.09 docs/sec
  Peak Memory: 97.52 MB

Comparison Summary
Speedup: 1.25x (GraphBit is 1.25x faster than LangChain)

Results saved to: test_ollama_results.json
```

**Performance**: GraphBit is 1.25x faster with 10 MB less memory usage

---

## 🧪 Running Stress Tests

### Quick Stress Test (10-50 documents)

**Command**:
```bash
python tests/benchmarks/stress_test_ollama.py --framework both --max-docs 50
```

**Expected Duration**: 5-10 minutes (CPU)

### Full Stress Test (10-1000 documents)

**Command**:
```bash
python tests/benchmarks/stress_test_ollama.py --framework both --max-docs 1000
```

**Expected Duration**: 2-4 hours (CPU), 15-30 minutes (GPU)

### Worker Scaling Test (GraphBit only)

**Command**:
```bash
python tests/benchmarks/stress_test_ollama.py --framework graphbit --test-workers --max-docs 50
```

**Expected Duration**: 10-20 minutes (CPU)

---

## 📊 Expected Performance Baselines

### CPU Performance (Intel Core i9, 20 cores, 32GB RAM)

| Operation | GraphBit | LangChain | Notes |
|-----------|----------|-----------|-------|
| **Document Loading** | 0.005s/doc | 0.02s/doc | GraphBit 4x faster (parallel) |
| **Chunking** | <0.001s/doc | <0.001s/doc | Both very fast |
| **Embedding** | 2.08s/chunk | 2.08s/chunk | Same (Ollama bottleneck) |
| **Query** | 4.40s | 5.74s | GraphBit 1.3x faster |
| **Overall** | 1.25x faster | Baseline | GraphBit advantage |

### GPU Performance (NVIDIA RTX 3080, estimated)

| Operation | Expected Time | Speedup vs CPU |
|-----------|---------------|----------------|
| **Embedding** | 0.20s/chunk | **10x faster** |
| **Query** | 0.50s | **8x faster** |
| **Overall** | **10x faster** | Significant improvement |

---

## 🔧 Troubleshooting

### Issue #1: Ollama Not Running

**Error**:
```
❌ Ollama is not running at http://localhost:11434
```

**Solution**:
```bash
# Windows: Restart Ollama service
# Check Task Manager for "Ollama" process
# If not running, launch Ollama from Start Menu

# macOS/Linux: Start Ollama service
ollama serve

# Verify it's running
curl http://localhost:11434/api/tags
```

---

### Issue #2: Model Not Available

**Error**:
```
❌ LLM model 'gemma3:4b' is not available
```

**Solution**:
```bash
# Pull the model
ollama pull gemma3:4b

# Verify it's available
ollama list

# Expected output should include:
# gemma3:4b    abc123def456    3.3 GB    ...
```

---

### Issue #3: LangChain Deprecation Warning

**Warning**:
```
LangChainDeprecationWarning: The class `OllamaEmbeddings` was deprecated in LangChain 0.3.1
```

**Impact**: Low (warnings only, functionality works)

**Solution** (optional):
```bash
# Install new package
pip install -U langchain-ollama

# Update imports in your code:
# Old:
from langchain_community.embeddings import OllamaEmbeddings
# New:
from langchain_ollama import OllamaEmbeddings
```

---

### Issue #4: Slow Embedding Generation

**Observation**: Embedding generation takes ~2s per embedding on CPU

**Solution**:
1. **Use GPU** (10x faster):
   - Install CUDA-enabled Ollama
   - Verify GPU is detected: `nvidia-smi`

2. **Use smaller models** (for testing):
   ```bash
   # Smaller embedding model (faster but less accurate)
   ollama pull all-minilm

   # Use in code:
   embedding_model="all-minilm"
   ```

3. **Reduce document count** (for testing):
   ```bash
   # Test with fewer documents
   python tests/benchmarks/benchmark_ollama_comparison.py --max-docs 5
   ```

---

### Issue #5: Out of Memory

**Error**:
```
MemoryError: Unable to allocate array
```

**Solution**:
1. **Reduce document count**:
   ```bash
   python tests/benchmarks/stress_test_ollama.py --max-docs 50
   ```

2. **Reduce worker count** (GraphBit):
   ```bash
   python examples/parallel_rag_ollama.py  # Uses 10 workers by default
   # Edit file to use fewer workers: max_workers=5
   ```

3. **Close other applications** to free up RAM

4. **Use smaller models**:
   ```bash
   ollama pull phi3:mini  # 2.3 GB instead of 3.3 GB
   ```

---

### Issue #6: Connection Timeout

**Error**:
```
requests.exceptions.ConnectionError: Connection refused
```

**Solution**:
1. **Check Ollama is running**:
   ```bash
   curl http://localhost:11434/api/tags
   ```

2. **Check firewall settings** (Windows):
   - Allow Ollama through Windows Firewall
   - Port 11434 should be open

3. **Check Ollama URL**:
   ```bash
   # If Ollama is running on different port/host
   python examples/parallel_rag_ollama.py --ollama-url http://localhost:11434
   ```

---

### Issue #7: Import Errors

**Error**:
```
ModuleNotFoundError: No module named 'graphbit'
```

**Solution**:
```bash
# Activate virtual environment
# Windows PowerShell:
.\venv\Scripts\Activate.ps1
# macOS/Linux:
source venv/bin/activate

# Install dependencies
pip install graphbit langchain langchain-community faiss-cpu
```

---

### Issue #8: DocumentLoader API Error

**Error**:
```
TypeError: load_document() got an unexpected keyword argument 'doc_type'
```

**Solution**: This bug has been fixed in the latest version. Update your code:
```bash
git pull origin main
```

---

### Issue #9: Division by Zero

**Error**:
```
ZeroDivisionError: float division by zero
```

**Solution**: This bug has been fixed in the latest version. Update your code:
```bash
git pull origin main
```

---

### Issue #10: GPU Not Detected

**Observation**: Ollama is using CPU instead of GPU

**Solution**:
1. **Verify GPU is available**:
   ```bash
   nvidia-smi  # Should show your GPU
   ```

2. **Install CUDA-enabled Ollama**:
   - Download GPU version from https://ollama.com/download
   - Reinstall Ollama

3. **Check Ollama logs**:
   ```bash
   # Windows: Check Event Viewer
   # macOS/Linux: Check ollama.log
   tail -f ollama.log
   ```

---

## ✅ Verification Checklist

Before running tests, verify:

- [ ] Ollama is installed and running
- [ ] Required models are pulled (gemma3:4b, nomic-embed-text)
- [ ] Python dependencies are installed
- [ ] Virtual environment is activated
- [ ] Ollama is accessible at http://localhost:11434
- [ ] At least 8 GB RAM available
- [ ] At least 10 GB disk space available

---

## 📚 Next Steps

1. **Run demos** to verify everything works
2. **Run benchmarks** to compare GraphBit vs LangChain
3. **Run stress tests** to find performance limits
4. **Read OLLAMA_BENCHMARK_GUIDE.md** for detailed benchmark documentation
5. **Read OLLAMA_TEST_RESULTS.md** for validated test results

---

## 🎯 Quick Command Reference

```bash
# Start Ollama
ollama serve

# Pull models
ollama pull gemma3:4b
ollama pull nomic-embed-text

# List models
ollama list

# Run GraphBit demo
python examples/parallel_rag_ollama.py

# Run LangChain demo
python langchain_rag_ollama.py

# Run benchmark
python tests/benchmarks/benchmark_ollama_comparison.py --framework both --max-docs 5

# Run stress test
python tests/benchmarks/stress_test_ollama.py --framework both --max-docs 50

# Run worker scaling test
python tests/benchmarks/stress_test_ollama.py --framework graphbit --test-workers --max-docs 50
```

---

**Status**: ✅ **VALIDATED AND READY TO USE**
**Last Validated**: November 17, 2025
**Validation Environment**: Windows 11, Intel Core i9, 32GB RAM, gemma3:4b + nomic-embed-text
