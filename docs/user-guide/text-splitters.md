# Text Splitters

Text splitters are essential components for processing large documents by breaking them into manageable chunks while maintaining context and semantic coherence. GraphBit provides various text splitting strategies optimized for different use cases.

## Overview

Text splitters help you:
- Process large documents that exceed model context windows
- Create embeddings for semantic search
- Parallelize document processing
- Maintain context across chunk boundaries with overlapping

## Available Splitters

### Character Splitter

Splits text based on character count, ideal for simple use cases where exact chunk sizes are needed.

```python
from graphbit import CharacterSplitter

# Initialize GraphBit

# Create a character splitter
splitter = CharacterSplitter(
    chunk_size=1000,      # Maximum characters per chunk
    chunk_overlap=200     # Overlap between chunks
)

# Split text
text = "Your long document text here..."
chunks = splitter.split_text(text)

# Process chunks
for chunk in chunks:
    print(f"Chunk {chunk.chunk_index}: {len(chunk.content)} characters")
    print(f"Position: {chunk.start_index} to {chunk.end_index}")
```

### Token Splitter

Splits text based on token count, useful when working with language models that have token limits.

```python
from graphbit import TokenSplitter

# Create a token splitter
splitter = TokenSplitter(
    chunk_size=100,       # Maximum tokens per chunk
    chunk_overlap=20,     # Token overlap
    token_pattern=None    # Optional custom regex pattern
)

# Custom token pattern example
custom_splitter = TokenSplitter(
    chunk_size=50,
    chunk_overlap=10,
    token_pattern=r'\b\w+\b'  # Split only on words
)
```

### Sentence Splitter

Maintains sentence boundaries, perfect for preserving semantic units.

```python
from graphbit import SentenceSplitter

# Create a sentence splitter
splitter = SentenceSplitter(
    chunk_size=500,       # Target size in characters
    chunk_overlap=1       # Number of sentences to overlap
)

# Custom sentence endings for multilingual text
multilingual_splitter = SentenceSplitter(
    chunk_size=500,
    chunk_overlap=1,
    sentence_endings=[r"\.", r"!", r"\?", r"。", r"！", r"？"]
)
```

### Recursive Splitter

Hierarchically splits text using multiple separators, ideal for structured documents.

```python
from graphbit import RecursiveSplitter

# Create a recursive splitter
splitter = RecursiveSplitter(
    chunk_size=1000,
    chunk_overlap=100
)

# Custom separators for specific document types
custom_splitter = RecursiveSplitter(
    chunk_size=1000,
    chunk_overlap=100,
    separators=["\n\n", "\n", ". ", " ", ""]
)
```

## Configuration-Based Splitters

Use `TextSplitterConfig` for more control and flexibility:

```python
from graphbit import TextSplitterConfig, TextSplitter

# Character configuration
config = TextSplitterConfig.character(
    chunk_size=1000,
    chunk_overlap=200
)

# Token configuration
config = TextSplitterConfig.token(
    chunk_size=100,
    chunk_overlap=20,
    token_pattern=r'\w+'
)

# Code splitter configuration
config = TextSplitterConfig.code(
    chunk_size=500,
    chunk_overlap=50,
    language="python"
)

# Markdown splitter configuration
config = TextSplitterConfig.markdown(
    chunk_size=1000,
    chunk_overlap=100,
    split_by_headers=True
)

# Create splitter from config
splitter = TextSplitter(config)
```

## Advanced Features

### Processing Multiple Documents

```python
from graphbit import CharacterSplitter

splitter = CharacterSplitter(1000, 200)

# Split multiple texts at once
texts = [
    "First document content...",
    "Second document content...",
    "Third document content..."
]

all_chunks = splitter.split_texts(texts)

for doc_idx, chunks in enumerate(all_chunks):
    print(f"Document {doc_idx}: {len(chunks)} chunks")
```

### Working with Chunk Metadata

```python
# after defining splitters
chunks = splitter.split_text(text)

for chunk in chunks:
    # Access chunk properties
    print(f"Content: {chunk.content}")
    print(f"Index: {chunk.chunk_index}")
    print(f"Position: {chunk.start_index} to {chunk.end_index}")
    
    # Access metadata
    metadata = chunk.metadata
    print(f"Length: {metadata['length']}")
```

### Creating Documents for Vector Stores

```python
from graphbit import TextSplitter, TextSplitterConfig

splitter = TextSplitter(
    TextSplitterConfig.character(1000, 200)
)

# Create documents with metadata
documents = splitter.create_documents(text)

# Documents are dictionaries ready for vector stores
for doc in documents:
    print(doc['content'])
    print(doc['start_index'])
    print(doc['end_index'])
    print(doc['chunk_index'])
```

## Best Practices

### 1. Choose the Right Splitter

- **Character Splitter**: Simple documents, consistent chunk sizes
- **Token Splitter**: Working with LLMs, precise token control
- **Sentence Splitter**: Maintaining semantic boundaries
- **Recursive Splitter**: Structured documents, code files

### 2. Optimize Chunk Size

Consider:
- Model context window limits
- Embedding model requirements
- Processing efficiency
- Semantic coherence

Common sizes:
- Embeddings: 500-1000 characters
- LLM processing: 2000-4000 characters
- Summarization: 1000-2000 characters

### 3. Use Appropriate Overlap

- Small overlap (10-20%): General documents
- Medium overlap (20-30%): Technical content
- Large overlap (30-50%): Dense information

### 4. Handle Special Content

#### Code Files
```python
from graphbit import TextSplitterConfig

config = TextSplitterConfig.code(
    chunk_size=1000,
    chunk_overlap=100,
    language="python"
)
config.set_trim_whitespace(False)  # Preserve formatting
```

#### Markdown Documents
```python
from graphbit import TextSplitterConfig

config = TextSplitterConfig.markdown(
    chunk_size=1500,
    chunk_overlap=200,
    split_by_headers=True
)
```

#### Unicode and Multilingual Text
```python
from graphbit import CharacterSplitter

# All splitters handle Unicode correctly
splitter = CharacterSplitter(100, 20)
text = "Hello 世界! Emoji support 🚀"
chunks = splitter.split_text(text)  # Works seamlessly
```

## Integration with GraphBit Workflows

Text splitters integrate seamlessly with other GraphBit components:

```python
from graphbit import RecursiveSplitter, EmbeddingClient, EmbeddingConfig

# Initialize

# Create components
splitter = RecursiveSplitter(1000, 100)
embedder = EmbeddingClient(
    EmbeddingConfig.openai("your-api-key")
)

# Process document
text = "Your large document..."
chunks = splitter.split_text(text)

# Generate embeddings for each chunk
embeddings = []
for chunk in chunks:
    embedding = embedder.embed_text(chunk.content)
    embeddings.append({
        'content': chunk.content,
        'embedding': embedding,
        'metadata': chunk.metadata
    })
```

## Error Handling

```python
from graphbit import CharacterSplitter

try:
    # Invalid configuration
    splitter = CharacterSplitter(
        chunk_size=0,  # Error: must be > 0
        chunk_overlap=0
    )
except Exception as e:
    print(f"Configuration error: {e}")

# Safe splitting with validation
def safe_split(text, chunk_size=1000, chunk_overlap=200):
    if chunk_size <= 0:
        raise ValueError("Chunk size must be positive")
    if chunk_overlap >= chunk_size:
        raise ValueError("Overlap must be less than chunk size")
    
    splitter = CharacterSplitter(chunk_size, chunk_overlap)
    return splitter.split_text(text)
```

## Performance Considerations

1. **Memory Usage**: Text splitters process text efficiently without loading entire documents into memory
2. **Processing Speed**: Character and recursive splitters are fastest; token splitters are slower due to regex processing
3. **Unicode Handling**: All splitters correctly handle multi-byte characters without performance penalties

## Summary

GraphBit's text splitters provide:
- Multiple splitting strategies for different use cases
- Proper Unicode and multilingual support
- Configurable overlap for context preservation
- Integration with GraphBit's workflow system
- Production-ready error handling and validation

Choose the appropriate splitter based on your content type and processing requirements, and leverage the configuration options to fine-tune behavior for optimal results. 
