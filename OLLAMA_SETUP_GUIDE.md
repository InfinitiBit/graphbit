# 🦙 Ollama Setup Guide for GraphBit & LangChain Benchmarks

**Purpose**: Run GraphBit and LangChain RAG benchmarks with local Ollama models (no API costs!)

---

## 📋 Table of Contents

1. [What is Ollama?](#1-what-is-ollama)
2. [Installation](#2-installation)
3. [Model Setup](#3-model-setup)
4. [Verification](#4-verification)
5. [Integration with GraphBit](#5-integration-with-graphbit)
6. [Integration with LangChain](#6-integration-with-langchain)
7. [Troubleshooting](#7-troubleshooting)

---

## 1. What is Ollama?

**Ollama** is a tool for running large language models locally on your machine.

**Benefits**:
- ✅ **Free**: No API costs
- ✅ **Private**: Data stays on your machine
- ✅ **Fast**: No network latency (for local inference)
- ✅ **Offline**: Works without internet connection

**Supported Models**:
- **LLMs**: llama3, mistral, phi3, gemma, qwen, etc.
- **Embeddings**: nomic-embed-text, mxbai-embed-large, all-minilm

---

## 2. Installation

### Windows

**Option 1: Download Installer** (Recommended)

1. Visit: https://ollama.com/download/windows
2. Download `OllamaSetup.exe`
3. Run the installer
4. Ollama will start automatically

**Option 2: Using Winget**

```powershell
winget install Ollama.Ollama
```

### macOS

**Option 1: Download Installer** (Recommended)

1. Visit: https://ollama.com/download/mac
2. Download `Ollama.dmg`
3. Drag Ollama to Applications
4. Launch Ollama from Applications

**Option 2: Using Homebrew**

```bash
brew install ollama
```

### Linux

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

**Start Ollama Service**:

```bash
# Start Ollama server
ollama serve

# Or run as systemd service
sudo systemctl start ollama
sudo systemctl enable ollama  # Auto-start on boot
```

---

## 3. Model Setup

### Recommended Models for Benchmarks

| Model | Size | Purpose | Download Command |
|-------|------|---------|------------------|
| **nomic-embed-text** | 274 MB | Embeddings (768-dim) | `ollama pull nomic-embed-text` |
| **mxbai-embed-large** | 669 MB | Embeddings (1024-dim) | `ollama pull mxbai-embed-large` |
| **llama3:8b** | 4.7 GB | LLM (fast, high quality) | `ollama pull llama3:8b` |
| **mistral:7b** | 4.1 GB | LLM (fast, efficient) | `ollama pull mistral:7b` |
| **phi3:mini** | 2.3 GB | LLM (very fast, small) | `ollama pull phi3:mini` |

### Pull Models

**Step 1: Pull Embedding Model**

```bash
ollama pull nomic-embed-text
```

**Expected Output**:
```
pulling manifest
pulling 970aa74c0a90... 100% ▕████████████████▏ 274 MB
pulling c71d239df917... 100% ▕████████████████▏  11 KB
pulling ce4a164fc046... 100% ▕████████████████▏   17 B
pulling 31df23ea7daa... 100% ▕████████████████▏  420 B
verifying sha256 digest
writing manifest
success
```

**Step 2: Pull LLM Model**

```bash
ollama pull llama3:8b
```

**Expected Output**:
```
pulling manifest
pulling 6a0746a1ec1a... 100% ▕████████████████▏ 4.7 GB
pulling 4fa551d4f938... 100% ▕████████████████▏  12 KB
pulling 8ab4849b038c... 100% ▕████████████████▏  254 B
pulling 577073ffcc6c... 100% ▕████████████████▏  110 B
pulling 3f8eb4da87fa... 100% ▕████████████████▏  485 B
verifying sha256 digest
writing manifest
success
```

### List Downloaded Models

```bash
ollama list
```

**Expected Output**:
```
NAME                    ID              SIZE    MODIFIED
nomic-embed-text:latest 970aa74c0a90    274 MB  2 minutes ago
llama3:8b               6a0746a1ec1a    4.7 GB  5 minutes ago
```

---

## 4. Verification

### Check Ollama is Running

**Windows/macOS**:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags
```

**Expected Output**:
```json
{"models":[{"name":"nomic-embed-text:latest",...},{"name":"llama3:8b",...}]}
```

### Test Embedding Generation

```bash
curl http://localhost:11434/api/embeddings -d '{
  "model": "nomic-embed-text",
  "prompt": "Hello world"
}'
```

**Expected Output**:
```json
{"embedding":[0.123, -0.456, 0.789, ...]}
```

### Test LLM Completion

```bash
curl http://localhost:11434/api/generate -d '{
  "model": "llama3:8b",
  "prompt": "Why is the sky blue?",
  "stream": false
}'
```

**Expected Output**:
```json
{"model":"llama3:8b","created_at":"...","response":"The sky appears blue because...","done":true}
```

---

## 5. Integration with GraphBit

### GraphBit with Ollama LLM

GraphBit has built-in Ollama support for LLMs:

```python
from graphbit import LlmConfig, LlmClient

# Configure Ollama LLM
llm_config = LlmConfig.ollama("llama3:8b")
llm_client = LlmClient(llm_config)

# Generate completion
response = llm_client.complete("What is machine learning?", max_tokens=500)
print(response)
```

### GraphBit with Ollama Embeddings (via OpenAI-compatible API)

Ollama provides an OpenAI-compatible API endpoint. We can use GraphBit's OpenAI embedding client with a custom base URL:

**⚠️ Note**: GraphBit's `EmbeddingConfig.openai()` doesn't currently expose `base_url` parameter in Python bindings. We'll need to use a workaround or extend the bindings.

**Workaround**: Use LangChain's OllamaEmbeddings for embeddings, GraphBit for everything else.

---

## 6. Integration with LangChain

### LangChain with Ollama

LangChain has full Ollama support:

```python
from langchain_community.embeddings import OllamaEmbeddings
from langchain_community.chat_models import ChatOllama

# Configure Ollama embeddings
embeddings = OllamaEmbeddings(
    model="nomic-embed-text",
    base_url="http://localhost:11434"  # Default Ollama endpoint
)

# Configure Ollama LLM
llm = ChatOllama(
    model="llama3:8b",
    base_url="http://localhost:11434",
    temperature=0.7
)

# Generate embeddings
embedding = embeddings.embed_query("Hello world")
print(f"Embedding dimension: {len(embedding)}")

# Generate completion
response = llm.invoke("What is machine learning?")
print(response.content)
```

---

## 7. Troubleshooting

### Issue 1: Ollama Not Running

**Error**:
```
ConnectionError: Failed to connect to http://localhost:11434
```

**Solution**:

```bash
# Windows/macOS: Start Ollama from Applications menu
# Or restart Ollama service

# Linux: Start Ollama service
sudo systemctl start ollama

# Check if Ollama is running
curl http://localhost:11434/api/tags
```

### Issue 2: Model Not Found

**Error**:
```
Error: model 'llama3:8b' not found
```

**Solution**:

```bash
# Pull the model
ollama pull llama3:8b

# Verify model is downloaded
ollama list
```

### Issue 3: Out of Memory

**Error**:
```
Error: failed to load model: insufficient memory
```

**Solution**:

```bash
# Use a smaller model
ollama pull phi3:mini  # 2.3 GB instead of 4.7 GB

# Or increase system memory/swap
```

### Issue 4: Slow Inference

**Symptoms**: Ollama takes 30+ seconds per request

**Solution**:

```bash
# Use a smaller/faster model
ollama pull phi3:mini

# Or use quantized models (smaller, faster)
ollama pull llama3:8b-q4_0  # 4-bit quantization

# Check GPU support (if available)
ollama run llama3:8b --verbose
```

### Issue 5: Port Already in Use

**Error**:
```
Error: bind: address already in use
```

**Solution**:

```bash
# Find process using port 11434
# Windows
netstat -ano | findstr :11434

# macOS/Linux
lsof -i :11434

# Kill the process or use a different port
OLLAMA_HOST=0.0.0.0:11435 ollama serve
```

---

## 📊 Performance Expectations

### Embedding Generation (nomic-embed-text)

| Batch Size | Time (CPU) | Time (GPU) | Throughput |
|------------|------------|------------|------------|
| 1 text | ~50ms | ~10ms | 20 texts/sec |
| 10 texts | ~200ms | ~30ms | 50 texts/sec |
| 100 texts | ~2s | ~200ms | 50 texts/sec |

### LLM Completion (llama3:8b)

| Tokens | Time (CPU) | Time (GPU) | Tokens/sec |
|--------|------------|------------|------------|
| 100 | ~5s | ~1s | 20 tokens/sec |
| 500 | ~25s | ~5s | 20 tokens/sec |

**💡 Note**: GPU inference is 5-10x faster than CPU. If you have an NVIDIA GPU, Ollama will automatically use it.

---

## 🎯 Recommended Configuration for Benchmarks

### For Fast Benchmarks (< 1 minute)

```bash
# Small, fast models
ollama pull nomic-embed-text  # 274 MB
ollama pull phi3:mini         # 2.3 GB
```

### For Quality Benchmarks (balanced)

```bash
# Medium-sized, high-quality models
ollama pull nomic-embed-text  # 274 MB
ollama pull llama3:8b         # 4.7 GB
```

### For Production-Like Benchmarks

```bash
# Larger, production-quality models
ollama pull mxbai-embed-large  # 669 MB
ollama pull llama3:70b         # 40 GB (requires 64+ GB RAM)
```

---

## 🔗 Additional Resources

- **Ollama Documentation**: https://github.com/ollama/ollama
- **Model Library**: https://ollama.com/library
- **LangChain Ollama Integration**: https://python.langchain.com/docs/integrations/llms/ollama
- **GraphBit Documentation**: See `docs/user-guide/llm-providers.md`

---

## ✅ Quick Start Checklist

- [ ] Install Ollama
- [ ] Pull embedding model: `ollama pull nomic-embed-text`
- [ ] Pull LLM model: `ollama pull llama3:8b`
- [ ] Verify Ollama is running: `curl http://localhost:11434/api/tags`
- [ ] Test embedding generation (see Section 4)
- [ ] Test LLM completion (see Section 4)
- [ ] Run benchmarks with `--use-ollama` flag

---

**You're ready to run cost-free local benchmarks!** 🎉


