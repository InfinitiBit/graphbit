# Python API Reference

Complete reference for GraphBit's Python API. This document covers all classes, methods, and their usage based on the actual Python binding implementation.

## Module: `graphbit`

### Core Functions

#### `init(log_level=None, enable_tracing=None, debug=None)`
Initialize the GraphBit library with optional configuration.

```python
import graphbit

# Basic initialization
graphbit.init()

# With debugging enabled
graphbit.init(debug=True)

# With custom log level and tracing
graphbit.init(log_level="info", enable_tracing=True)
```

**Parameters**:
- `log_level` (str, optional): Log level ("trace", "debug", "info", "warn", "error"). Default: "warn"
- `enable_tracing` (bool, optional): Enable tracing. Default: False
- `debug` (bool, optional): Enable debug mode (alias for enable_tracing). Default: False

**Returns**: `None`  
**Raises**: `RuntimeError` if initialization fails

#### `version()`
Get the current GraphBit version.

```python
version = graphbit.version()
print(f"GraphBit version: {version}")
```

**Returns**: `str` - Version string (e.g., "0.1.0")

#### `get_system_info()`
Get comprehensive system information and health status.

```python
info = graphbit.get_system_info()
print(f"CPU count: {info['cpu_count']}")
print(f"Runtime initialized: {info['runtime_initialized']}")
```

**Returns**: `dict` - Dictionary containing:
- `version`: GraphBit version
- `python_binding_version`: Python binding version
- `runtime_uptime_seconds`: Runtime uptime
- `runtime_worker_threads`: Number of worker threads
- `cpu_count`: Number of CPU cores
- `runtime_initialized`: Runtime initialization status
- `memory_allocator`: Memory allocator type
- `build_target`: Build target
- `build_profile`: Build profile (debug/release)

#### `health_check()`
Perform comprehensive health checks.

```python
health = graphbit.health_check()
if health['overall_healthy']:
    print("System is healthy")
else:
    print("System has issues")
```

**Returns**: `dict` - Dictionary containing health status information

#### `configure_runtime(worker_threads=None, max_blocking_threads=None, thread_stack_size_mb=None)`
Configure the global runtime with custom settings (advanced).

```python
# Configure runtime before init()
graphbit.configure_runtime(worker_threads=8, max_blocking_threads=16)
graphbit.init()
```

**Parameters**:
- `worker_threads` (int, optional): Number of worker threads
- `max_blocking_threads` (int, optional): Maximum blocking threads
- `thread_stack_size_mb` (int, optional): Thread stack size in MB

#### `shutdown()`
Gracefully shutdown the library (for testing and cleanup).

```python
graphbit.shutdown()
```

---

## LLM Configuration

### `LlmConfig`

Configuration class for Large Language Model providers.

#### Static Methods

##### `LlmConfig.openai(api_key, model=None)`
Create OpenAI provider configuration.

```python
config = graphbit.LlmConfig.openai("sk-...", "gpt-4o-mini")
# With default model
config = graphbit.LlmConfig.openai("sk-...")  # Uses gpt-4o-mini
```

**Parameters**:
- `api_key` (str): OpenAI API key
- `model` (str, optional): Model name. Default: "gpt-4o-mini"

**Returns**: `LlmConfig` instance

##### `LlmConfig.anthropic(api_key, model=None)`
Create Anthropic provider configuration.

```python
config = graphbit.LlmConfig.anthropic("sk-ant-...", "claude-3-5-sonnet-20241022")
# With default model
config = graphbit.LlmConfig.anthropic("sk-ant-...")  # Uses claude-3-5-sonnet-20241022
```

**Parameters**:
- `api_key` (str): Anthropic API key
- `model` (str, optional): Model name. Default: "claude-3-5-sonnet-20241022"

**Returns**: `LlmConfig` instance

##### `LlmConfig.deepseek(api_key, model=None)`
Create DeepSeek provider configuration.

```python
config = graphbit.LlmConfig.deepseek("your-deepseek-api-key", "deepseek-chat")
# With default model
config = graphbit.LlmConfig.deepseek("your-deepseek-api-key")  # Uses deepseek-chat
```

**Parameters**:
- `api_key` (str): DeepSeek API key
- `model` (str, optional): Model name. Default: "deepseek-chat"

**Available Models**:
- `deepseek-chat`: General conversation and instruction following
- `deepseek-coder`: Specialized for code generation and programming tasks
- `deepseek-reasoner`: Advanced reasoning and mathematical problem solving

**Returns**: `LlmConfig` instance

##### `LlmConfig.huggingface(api_key, model=None, base_url=None)`
Create HuggingFace provider configuration.

```python
config = graphbit.LlmConfig.huggingface("hf_...", "microsoft/DialoGPT-medium")
# With default model
config = graphbit.LlmConfig.huggingface("hf_...")  # Uses microsoft/DialoGPT-medium
# With custom endpoint
config = graphbit.LlmConfig.huggingface("hf_...", "mistralai/Mistral-7B-Instruct-v0.1", 
                                        base_url="https://my-endpoint.huggingface.co")
```

**Parameters**:
- `api_key` (str): HuggingFace API key
- `model` (str, optional): Model name. Default: "microsoft/DialoGPT-medium"
- `base_url` (str, optional): Custom API endpoint. Default: HuggingFace Inference API

**Returns**: `LlmConfig` instance

##### `LlmConfig.ollama(model=None)`
Create Ollama provider configuration.

```python
config = graphbit.LlmConfig.ollama("llama3.2")
# With default model
config = graphbit.LlmConfig.ollama()  # Uses llama3.2
```

**Parameters**:
- `model` (str, optional): Model name. Default: "llama3.2"

**Returns**: `LlmConfig` instance

#### Instance Methods

##### `provider()`
Get the provider name.

```python
provider = config.provider()  # "openai", "anthropic", "ollama"
```

##### `model()`
Get the model name.

```python
model = config.model()  # "gpt-4o-mini", "claude-3-5-sonnet-20241022", etc.
```

---

## LLM Client

### `LlmClient`

Production-grade LLM client with resilience patterns.

#### Constructor

##### `LlmClient(config, debug=None)`
Create a new LLM client.

```python
client = graphbit.LlmClient(config)
# With debugging
client = graphbit.LlmClient(config, debug=True)
```

**Parameters**:
- `config` (LlmConfig): LLM configuration
- `debug` (bool, optional): Enable debug mode. Default: False

#### Methods

##### `complete(prompt, max_tokens=None, temperature=None)`
Synchronous completion with resilience.

```python
response = client.complete("Write a short story about a robot")
print(response)

# With parameters
response = client.complete(
    "Explain quantum computing",
    max_tokens=500,
    temperature=0.7
)
```

**Parameters**:
- `prompt` (str): Input prompt
- `max_tokens` (int, optional): Maximum tokens to generate (1-100000)
- `temperature` (float, optional): Sampling temperature (0.0-2.0)

**Returns**: `str` - Generated text
**Raises**: `ValueError` for invalid parameters

##### `complete_async(prompt, max_tokens=None, temperature=None)`
Asynchronous completion with full resilience.

```python
import asyncio

async def generate():
    response = await client.complete_async("Tell me a joke")
    return response

result = asyncio.run(generate())
```

**Parameters**: Same as `complete()`
**Returns**: `Awaitable[str]` - Generated text

##### `complete_batch(prompts, max_tokens=None, temperature=None, max_concurrency=None)`
Ultra-fast batch processing with controlled concurrency.

```python
import asyncio

prompts = [
    "Summarize AI trends",
    "Explain blockchain",
    "Describe quantum computing"
]

async def batch_generate():
    responses = await client.complete_batch(prompts, max_concurrency=5)
    return responses

results = asyncio.run(batch_generate())
```

**Parameters**:
- `prompts` (List[str]): List of prompts (max 1000)
- `max_tokens` (int, optional): Maximum tokens per prompt
- `temperature` (float, optional): Sampling temperature
- `max_concurrency` (int, optional): Maximum concurrent requests. Default: CPU count * 2

**Returns**: `Awaitable[List[str]]` - List of generated responses

##### `chat_optimized(messages, max_tokens=None, temperature=None)`
Optimized chat completion with message validation.

```python
import asyncio

messages = [
    ("system", "You are a helpful assistant"),
    ("user", "What is Python?"),
    ("assistant", "Python is a programming language"),
    ("user", "Tell me more about its features")
]

async def chat():
    response = await client.chat_optimized(messages)
    return response

result = asyncio.run(chat())
```

**Parameters**:
- `messages` (List[Tuple[str, str]]): List of (role, content) tuples
- `max_tokens` (int, optional): Maximum tokens to generate
- `temperature` (float, optional): Sampling temperature

**Returns**: `Awaitable[str]` - Generated response

##### `complete_stream(prompt, max_tokens=None, temperature=None)`
Stream completion (alias for async complete).

```python
import asyncio

async def stream():
    response = await client.complete_stream("Write a poem")
    return response

result = asyncio.run(stream())
```

##### `get_stats()`
Get comprehensive client statistics.

```python
stats = client.get_stats()
print(f"Total requests: {stats['total_requests']}")
print(f"Success rate: {stats['success_rate']}")
print(f"Average response time: {stats['average_response_time_ms']}ms")
```

**Returns**: `dict` - Dictionary containing performance metrics

##### `warmup()`
Warm up the client to avoid initialization overhead.

```python
import asyncio

async def prepare():
    await client.warmup()
    print("Client warmed up")

asyncio.run(prepare())
```

##### `reset_stats()`
Reset client statistics.

```python
client.reset_stats()
```

---

## Document Processing

### `DocumentLoaderConfig`

Configuration class for document loading operations.

#### Constructor

##### `DocumentLoaderConfig(max_file_size=None, default_encoding=None, preserve_formatting=None)`
Create a new document loader configuration.

```python
# Default configuration
config = graphbit.DocumentLoaderConfig()

# Custom configuration
config = graphbit.DocumentLoaderConfig(
    max_file_size=50_000_000,      # 50MB limit
    default_encoding="utf-8",       # Text encoding
    preserve_formatting=True        # Keep document formatting
)
```

**Parameters**:
- `max_file_size` (int, optional): Maximum file size in bytes. Must be greater than 0
- `default_encoding` (str, optional): Default text encoding for text files. Cannot be empty
- `preserve_formatting` (bool, optional): Whether to preserve document formatting. Default: False

#### Properties

##### `max_file_size`
Get or set the maximum file size limit.

```python
size = config.max_file_size
config.max_file_size = 100_000_000  # 100MB
```

##### `default_encoding`
Get or set the default text encoding.

```python
encoding = config.default_encoding
config.default_encoding = "utf-8"
```

##### `preserve_formatting`
Get or set the formatting preservation flag.

```python
preserve = config.preserve_formatting
config.preserve_formatting = True
```

##### `extraction_settings`
Get or set extraction settings as a dictionary.

```python
settings = config.extraction_settings
config.extraction_settings = {"pdf_parser": "advanced", "ocr_enabled": True}
```

### `DocumentContent`

Contains the extracted content and metadata from a loaded document.

#### Properties

##### `source`
Get the source path or URL of the document.

```python
source_path = content.source
```

##### `document_type`
Get the detected document type.

```python
doc_type = content.document_type  # "pdf", "txt", "docx", etc.
```

##### `content`
Get the extracted text content.

```python
text = content.content
```

##### `file_size`
Get the file size in bytes.

```python
size = content.file_size
```

##### `extracted_at`
Get the extraction timestamp as a UTC timestamp.

```python
timestamp = content.extracted_at
```

##### `metadata`
Get document metadata as a dictionary.

```python
metadata = content.metadata
print(f"Author: {metadata.get('author', 'Unknown')}")
print(f"Pages: {metadata.get('pages', 'N/A')}")
```

#### Methods

##### `content_length()`
Get the length of extracted content.

```python
length = content.content_length()
```

##### `is_empty()`
Check if the extracted content is empty.

```python
if content.is_empty():
    print("No content extracted")
```

##### `preview(max_length=500)`
Get a preview of the content.

```python
preview = content.preview(200)  # First 200 characters
full_preview = content.preview()  # First 500 characters (default)
```

### `DocumentLoader`

Main class for loading and processing documents from various sources.

#### Constructor

##### `DocumentLoader(config=None)`
Create a new document loader.

```python
# With default configuration
loader = graphbit.DocumentLoader()

# With custom configuration
config = graphbit.DocumentLoaderConfig(max_file_size=10_000_000)
loader = graphbit.DocumentLoader(config)
```

#### Methods

##### `load_document(source_path, document_type)`
Load and extract content from a document.

```python
# Load a PDF document
content = loader.load_document("/path/to/document.pdf", "pdf")
print(f"Extracted {content.content_length()} characters")

# Load a text file
content = loader.load_document("data/report.txt", "txt")
print(content.content)

# Load a Word document
content = loader.load_document("docs/manual.docx", "docx")
print(f"Document metadata: {content.metadata}")
```

**Parameters**:
- `source_path` (str): Path to the document file. Cannot be empty
- `document_type` (str): Type of document. Cannot be empty

**Returns**: `DocumentContent` - The extracted content and metadata
**Raises**: `ValueError` for invalid parameters, `RuntimeError` for loading errors

#### Static Methods

##### `DocumentLoader.supported_types()`
Get list of supported document types.

```python
types = graphbit.DocumentLoader.supported_types()
print(f"Supported formats: {types}")
# Output: ['txt', 'pdf', 'docx', 'json', 'csv', 'xml', 'html']
```

##### `DocumentLoader.detect_document_type(file_path)`
Detect document type from file extension.

```python
doc_type = graphbit.DocumentLoader.detect_document_type("report.pdf")
print(f"Detected type: {doc_type}")  # "pdf"

# Returns None if type cannot be detected
unknown_type = graphbit.DocumentLoader.detect_document_type("file.unknown")
print(unknown_type)  # None
```

##### `DocumentLoader.validate_document_source(source_path, document_type)`
Validate document source and type combination.

```python
try:
    graphbit.DocumentLoader.validate_document_source("report.pdf", "pdf")
    print("Valid document source")
except Exception as e:
    print(f"Invalid: {e}")
```

### Document Processing Examples

#### Basic Document Loading
```python
import graphbit

# Initialize GraphBit
graphbit.init()

# Create loader with default settings
loader = graphbit.DocumentLoader()

# Load different document types
pdf_content = loader.load_document("research_paper.pdf", "pdf")
txt_content = loader.load_document("notes.txt", "txt")
docx_content = loader.load_document("proposal.docx", "docx")

# Access content
print(f"PDF content preview: {pdf_content.preview()}")
print(f"Text file size: {txt_content.file_size} bytes")
print(f"Word doc metadata: {docx_content.metadata}")
```

#### Advanced Configuration
```python
# Configure for large documents with formatting preservation
config = graphbit.DocumentLoaderConfig(
    max_file_size=100_000_000,     # 100MB limit
    default_encoding="utf-8",       # UTF-8 encoding
    preserve_formatting=True        # Keep formatting
)

# Set custom extraction settings
config.extraction_settings = {
    "pdf_parser": "advanced",
    "ocr_enabled": True,
    "extract_images": False,
    "table_detection": True
}

# Create loader with custom config
loader = graphbit.DocumentLoader(config)

# Load with enhanced processing
content = loader.load_document("complex_document.pdf", "pdf")
```

#### Document Type Detection and Validation
```python
import os

def process_document(file_path):
    # Auto-detect document type
    doc_type = graphbit.DocumentLoader.detect_document_type(file_path)
    
    if doc_type is None:
        print(f"Unsupported file type: {file_path}")
        return None
    
    # Validate before processing
    try:
        graphbit.DocumentLoader.validate_document_source(file_path, doc_type)
    except Exception as e:
        print(f"Validation failed: {e}")
        return None
    
    # Load the document
    loader = graphbit.DocumentLoader()
    content = loader.load_document(file_path, doc_type)
    
    return content

# Process multiple files
files = ["report.pdf", "data.csv", "manual.docx", "notes.txt"]
for file_path in files:
    if os.path.exists(file_path):
        content = process_document(file_path)
        if content:
            print(f"Processed {file_path}: {content.content_length()} characters")
```

#### Batch Document Processing
```python
import os
import asyncio

def load_documents_from_directory(directory_path):
    """Load all supported documents from a directory"""
    loader = graphbit.DocumentLoader()
    supported_types = set(graphbit.DocumentLoader.supported_types())
    
    contents = []
    
    for filename in os.listdir(directory_path):
        file_path = os.path.join(directory_path, filename)
        
        if os.path.isfile(file_path):
            # Auto-detect type
            doc_type = graphbit.DocumentLoader.detect_document_type(file_path)
            
            if doc_type in supported_types:
                try:
                    content = loader.load_document(file_path, doc_type)
                    contents.append({
                        'filename': filename,
                        'type': doc_type,
                        'content': content,
                        'size': content.file_size,
                        'length': content.content_length()
                    })
                    print(f"Loaded: {filename} ({doc_type})")
                except Exception as e:
                    print(f"Failed to load {filename}: {e}")
            else:
                print(f"Skipped unsupported file: {filename}")
    
    return contents

# Process all documents in a directory
document_data = load_documents_from_directory("./documents")

# Summary
total_docs = len(document_data)
total_size = sum(doc['size'] for doc in document_data)
total_content = sum(doc['length'] for doc in document_data)

print(f"\nSummary:")
print(f"Documents processed: {total_docs}")
print(f"Total file size: {total_size:,} bytes")
print(f"Total extracted content: {total_content:,} characters")
```

#### Error Handling Best Practices
```python
import graphbit

def safe_document_loading(file_path, doc_type=None):
    """Safely load a document with comprehensive error handling"""
    
    try:
        # Auto-detect type if not provided
        if doc_type is None:
            doc_type = graphbit.DocumentLoader.detect_document_type(file_path)
            if doc_type is None:
                raise ValueError(f"Cannot detect document type for: {file_path}")
        
        # Validate source
        graphbit.DocumentLoader.validate_document_source(file_path, doc_type)
        
        # Create loader with reasonable limits
        config = graphbit.DocumentLoaderConfig(
            max_file_size=50_000_000,  # 50MB limit
            default_encoding="utf-8"
        )
        loader = graphbit.DocumentLoader(config)
        
        # Load document
        content = loader.load_document(file_path, doc_type)
        
        # Validate result
        if content.is_empty():
            print(f"Warning: No content extracted from {file_path}")
            return None
        
        return content
        
    except ValueError as e:
        print(f"Invalid input: {e}")
        return None
    except RuntimeError as e:
        print(f"Loading failed: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None

# Usage
content = safe_document_loading("document.pdf")
if content:
    print(f"Successfully loaded document: {content.source}")
    print(f"Content preview: {content.preview(100)}")
```

---

## Embeddings

### `EmbeddingConfig`

Configuration for embedding providers.

#### Static Methods

##### `EmbeddingConfig.openai(api_key, model=None)`
Create OpenAI embeddings configuration.

```python
config = graphbit.EmbeddingConfig.openai("sk-...", "text-embedding-3-small")
# With default model
config = graphbit.EmbeddingConfig.openai("sk-...")  # Uses text-embedding-3-small
```

**Parameters**:
- `api_key` (str): OpenAI API key
- `model` (str, optional): Model name. Default: "text-embedding-3-small"

##### `EmbeddingConfig.huggingface(api_key, model)`
Create HuggingFace embeddings configuration.

```python
config = graphbit.EmbeddingConfig.huggingface("hf_...", "sentence-transformers/all-MiniLM-L6-v2")
```

**Parameters**:
- `api_key` (str): HuggingFace API token
- `model` (str): Model name from HuggingFace hub

### `EmbeddingClient`

Client for generating text embeddings.

#### Constructor

##### `EmbeddingClient(config)`
Create embedding client.

```python
client = graphbit.EmbeddingClient(config)
```

#### Methods

##### `embed(text)`
Generate embedding for single text.

```python
embedding = client.embed("Hello world")
print(f"Embedding dimension: {len(embedding)}")
```

**Parameters**:
- `text` (str): Input text

**Returns**: `List[float]` - Embedding vector

##### `embed_many(texts)`
Generate embeddings for multiple texts.

```python
texts = ["Text 1", "Text 2", "Text 3"]
embeddings = client.embed_many(texts)
print(f"Generated {len(embeddings)} embeddings")
```

**Parameters**:
- `texts` (List[str]): List of input texts

**Returns**: `List[List[float]]` - List of embedding vectors

##### `similarity(a, b)` (static)
Calculate cosine similarity between two embeddings.

```python
similarity = graphbit.EmbeddingClient.similarity(embed1, embed2)
print(f"Similarity: {similarity}")
```

**Parameters**:
- `a` (List[float]): First embedding vector
- `b` (List[float]): Second embedding vector

**Returns**: `float` - Cosine similarity (-1.0 to 1.0)

---

## Workflow Components

### `Node`

Factory class for creating different types of workflow nodes.

#### Static Methods

##### `Node.agent(name, prompt, agent_id=None)`
Create an AI agent node.

```python
agent = graphbit.Node.agent(
    name="Content Analyzer",
    prompt="Analyze the sentiment of: {input}",
    agent_id="analyzer"  # Optional, auto-generated if not provided
)
```

**Parameters**:
- `name` (str): Human-readable node name
- `prompt` (str): LLM prompt template with variables
- `agent_id` (str, optional): Unique agent identifier. Auto-generated if not provided

**Returns**: `Node` instance

##### `Node.transform(name, transformation)`
Create a data transformation node.

```python
transformer = graphbit.Node.transform(
    name="Uppercase",
    transformation="uppercase"
)
```

**Parameters**:
- `name` (str): Node name
- `transformation` (str): Transformation type

**Returns**: `Node` instance

##### `Node.condition(name, expression)`
Create a condition node for branching logic.

```python
condition = graphbit.Node.condition(
    name="Quality Check",
    expression="quality_score > 0.8"
)
```

**Parameters**:
- `name` (str): Node name
- `expression` (str): Boolean expression to evaluate

**Returns**: `Node` instance

#### Instance Methods

##### `id()`
Get the node ID.

##### `name()`
Get the node name.

### `Workflow`

Represents a complete workflow.

#### Constructor

##### `Workflow(name)`
Create a new workflow.

```python
workflow = graphbit.Workflow("My Workflow")
```

#### Methods

##### `add_node(node)`
Add a node to the workflow.

```python
node_id = workflow.add_node(my_node)
print(f"Added node with ID: {node_id}")
```

**Parameters**:
- `node` (Node): Node to add

**Returns**: `str` - Unique node ID

##### `connect(from_id, to_id)`
Connect two nodes.

```python
workflow.connect(node1_id, node2_id)
```

**Parameters**:
- `from_id` (str): Source node ID
- `to_id` (str): Target node ID

##### `validate()`
Validate the workflow structure.

```python
try:
    workflow.validate()
    print("Workflow is valid")
except Exception as e:
    print(f"Invalid workflow: {e}")
```

### `WorkflowResult`

Contains workflow execution results.

#### Methods

##### `is_success()`
Check if workflow completed successfully.

```python
if result.is_success():
    print("Workflow completed successfully")
```

##### `is_failed()`
Check if workflow failed.

```python
if result.is_failed():
    print("Workflow failed")
```

##### `state()`
Get the workflow state.

```python
state = result.state()
print(f"Workflow state: {state}")
```

##### `execution_time_ms()`
Get execution time in milliseconds.

```python
time_ms = result.execution_time_ms()
print(f"Executed in {time_ms}ms")
```

##### `get_variable(key)`
Get a variable value.

```python
output = result.get_variable("output")
if output:
    print(f"Result: {output}")
```

##### `get_all_variables()`
Get all variables as a dictionary.

```python
all_vars = result.get_all_variables()
for key, value in all_vars.items():
    print(f"{key}: {value}")
```

##### `variables()`
Get all variables as a list of tuples.

```python
vars_list = result.variables()
for key, value in vars_list:
    print(f"{key}: {value}")
```

---

## Workflow Execution

### `Executor`

Production-grade workflow executor with comprehensive features.

#### Constructors

##### `Executor(config, lightweight_mode=None, timeout_seconds=None, debug=None)`
Create a basic executor.

```python
executor = graphbit.Executor(llm_config)
# With configuration
executor = graphbit.Executor(
    llm_config, 
    lightweight_mode=False,
    timeout_seconds=300,
    debug=True
)
```

**Parameters**:
- `config` (LlmConfig): LLM configuration
- `lightweight_mode` (bool, optional): Enable lightweight mode (low latency)
- `timeout_seconds` (int, optional): Execution timeout (1-3600 seconds)
- `debug` (bool, optional): Enable debug mode

##### `Executor.new_high_throughput(llm_config, timeout_seconds=None, debug=None)` (static)
Create executor optimized for high throughput.

```python
executor = graphbit.Executor.new_high_throughput(llm_config)
```

##### `Executor.new_low_latency(llm_config, timeout_seconds=None, debug=None)` (static)
Create executor optimized for low latency.

```python
executor = graphbit.Executor.new_low_latency(llm_config, timeout_seconds=30)
```

##### `Executor.new_memory_optimized(llm_config, timeout_seconds=None, debug=None)` (static)
Create executor optimized for memory usage.

```python
executor = graphbit.Executor.new_memory_optimized(llm_config)
```

#### Configuration Methods

##### `configure(timeout_seconds=None, max_retries=None, enable_metrics=None, debug=None)`
Configure the executor with new settings.

```python
executor.configure(
    timeout_seconds=600,
    max_retries=5,
    enable_metrics=True,
    debug=False
)
```

##### `set_lightweight_mode(enabled)`
Legacy method for backward compatibility.

```python
executor.set_lightweight_mode(True)
```

##### `is_lightweight_mode()`
Check if lightweight mode is enabled.

```python
is_lightweight = executor.is_lightweight_mode()
```

#### Execution Methods

##### `execute(workflow)`
Execute a workflow synchronously.

```python
result = executor.execute(workflow)
if result.is_success():
    print("Success!")
```

**Parameters**:
- `workflow` (Workflow): Workflow to execute

**Returns**: `WorkflowResult` - Execution result

##### `run_async(workflow)`
Execute a workflow asynchronously.

```python
import asyncio

async def run_workflow():
    result = await executor.run_async(workflow)
    return result

result = asyncio.run(run_workflow())
```

#### Statistics Methods

##### `get_stats()`
Get comprehensive execution statistics.

```python
stats = executor.get_stats()
print(f"Total executions: {stats['total_executions']}")
print(f"Success rate: {stats['success_rate']}")
print(f"Average duration: {stats['average_duration_ms']}ms")
```

##### `reset_stats()`
Reset execution statistics.

```python
executor.reset_stats()
```

##### `get_execution_mode()`
Get the current execution mode.

```python
mode = executor.get_execution_mode()
print(f"Execution mode: {mode}")
```

---

## Error Handling

GraphBit uses standard Python exceptions:

- `ValueError` - Invalid parameters or workflow structure
- `RuntimeError` - Execution errors
- `TimeoutError` - Operation timeouts

```python
try:
    result = executor.execute(workflow)
except ValueError as e:
    print(f"Invalid workflow: {e}")
except RuntimeError as e:
    print(f"Execution failed: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

---

## Usage Examples

### Basic Workflow
```python
import graphbit
import os

# Initialize
graphbit.init()

# Configure LLM
config = graphbit.LlmConfig.openai(os.getenv("OPENAI_API_KEY"))

# Create workflow
workflow = graphbit.Workflow("Analysis Workflow")
agent = graphbit.Node.agent(
    "Analyzer", 
    "Analyze the following text: {input}"
)
node_id = workflow.add_node(agent)
workflow.validate()

# Execute
executor = graphbit.Executor(config)
result = executor.execute(workflow)

if result.is_success():
    output = result.get_variable("output")
    print(f"Analysis result: {output}")
```

### Advanced LLM Usage
```python
# Create client with debugging
client = graphbit.LlmClient(config, debug=True)

# Batch processing
prompts = [
    "Summarize: AI is transforming industries",
    "Explain: Machine learning algorithms",
    "Analyze: The future of automation"
]

import asyncio
async def process_batch():
    responses = await client.complete_batch(prompts, max_concurrency=3)
    return responses

results = asyncio.run(process_batch())

# Chat conversation
messages = [
    ("system", "You are a helpful AI assistant"),
    ("user", "What is machine learning?"),
    ("assistant", "Machine learning is a subset of AI..."),
    ("user", "Can you give me an example?")
]

async def chat():
    response = await client.chat_optimized(messages, temperature=0.7)
    return response

chat_result = asyncio.run(chat())
```

### High-Performance Execution
```python
# Create high-throughput executor
executor = graphbit.Executor.new_high_throughput(
    llm_config, 
    timeout_seconds=600,
    debug=False
)

# Configure for production
executor.configure(
    timeout_seconds=300,
    max_retries=3,
    enable_metrics=True
)

# Execute workflow
result = executor.execute(workflow)

# Monitor performance
stats = executor.get_stats()
print(f"Execution stats: {stats}")
```

### Embeddings Usage
```python
# Configure embeddings
embed_config = graphbit.EmbeddingConfig.openai(os.getenv("OPENAI_API_KEY"))
embed_client = graphbit.EmbeddingClient(embed_config)

# Generate embeddings
texts = ["Hello world", "Goodbye world", "Machine learning"]
embeddings = embed_client.embed_many(texts)

# Calculate similarity
similarity = graphbit.EmbeddingClient.similarity(embeddings[0], embeddings[1])
print(f"Similarity between texts: {similarity}")
``` 
