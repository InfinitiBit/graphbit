# GraphBit Documentation

Welcome to GraphBit - a high-performance agentic workflow automation framework built with Rust and Python bindings. GraphBit enables developers to create reliable, type-safe AI workflows with strong error handling, concurrency control, and multi-LLM provider support.

## 🚀 Quick Start

### Installation & Setup
```bash
# Install via pip
pip install graphbit

# Or install from source
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit && make install
```

### Your First Workflow
```python
import graphbit
import os

# Initialize GraphBit
graphbit.init()

# Configure LLM provider
config = graphbit.PyLlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"), 
    model="gpt-4o-mini"
)

# Build workflow
builder = graphbit.PyWorkflowBuilder("Hello GraphBit")
node = graphbit.PyWorkflowNode.agent_node(
    name="Greeter",
    description="Generates personalized greetings", 
    agent_id="greeter-001",
    prompt="Generate a warm, personalized greeting for {name}"
)

node_id = builder.add_node(node)
workflow = builder.build()

# Execute with reliability features
executor = graphbit.PyWorkflowExecutor(config) \
    .with_retry_config(graphbit.PyRetryConfig.default()) \
    .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default())

result = executor.execute(workflow)
print(f"Completed in {result.execution_time_ms()}ms")
```

**Learn more**: [Installation Guide](00-installation-setup.md) | [Getting Started](01-getting-started-workflows.md)

---

## 📚 Documentation Index

### Core Concepts & Architecture
- **[Installation & Setup](00-installation-setup.md)** - Environment setup, dependencies, API keys
- **[Getting Started](01-getting-started-workflows.md)** - First workflows, basic concepts
- **[Complete API Reference](05-complete-api-reference.md)** - Full Python API documentation
- **[Quick API Reference](06-api-quick-reference.md)** - Common patterns and shortcuts

### Advanced Features & Use Cases
- **[Use Case Examples](02-use-case-examples.md)** - Real-world workflow examples
- **[Local LLM Integration](03-local-llm-integration.md)** - Ollama, HuggingFace setup
- **[Document Processing](04-document-processing-guide.md)** - PDF, text, markdown processing
- **[Embeddings Guide](07-embeddings-guide.md)** - Semantic search, RAG patterns
- **[HuggingFace Integration](08-huggingface-integration.md)** - Models, embeddings, deployment

### Project Management
- **[Python API Summary](python-api-summary.md)** - Quick reference for Python bindings
- **[Contributing Guide](CONTRIBUTING.md)** - Development setup, guidelines
- **[Changelog](CHANGELOG.md)** - Release notes and updates

---

## 🏗️ Architecture Overview

GraphBit follows a three-tier architecture designed for performance, reliability, and ease of use:

```
┌─────────────────┐
│   Python API    │ ← PyO3 Bindings with async support
├─────────────────┤
│   CLI Tool      │ ← Project management & execution
├─────────────────┤
│   Rust Core     │ ← Workflow engine, agents, LLM providers
└─────────────────┘
```

### Core Components

- **Workflow Engine**: Graph-based execution with dependency management
- **Agent System**: LLM-backed intelligent agents with configurable capabilities
- **LLM Providers**: Multi-provider support (OpenAI, Anthropic, Ollama, HuggingFace)
- **Reliability**: Circuit breakers, retry policies, comprehensive error handling
- **Concurrency**: Semaphore-based resource management and throttling
- **Type Safety**: Strong typing throughout with UUID-based identifiers

---

## 🧠 Embeddings & Semantic Analysis

GraphBit provides powerful embeddings support for semantic search, similarity analysis, and RAG (Retrieval-Augmented Generation) workflows.

### Quick Embeddings Example
```python
import asyncio
import graphbit

async def embeddings_demo():
    # Configure embedding service
    config = graphbit.PyEmbeddingConfig.openai(
        api_key="your-openai-key",
        model="text-embedding-3-small"
    ).with_timeout(30).with_max_batch_size(100)
    
    service = graphbit.PyEmbeddingService(config)
    
    # Generate embeddings
    texts = ["Machine learning fundamentals", "Deep learning algorithms", "Natural language processing"]
    embeddings = await service.embed_texts(texts)
    
    # Calculate semantic similarity
    similarity = graphbit.PyEmbeddingService.cosine_similarity(
        embeddings[0], embeddings[1]
    )
    print(f"Similarity: {similarity:.3f}")
    
    # Get model information
    dimensions = await service.get_dimensions()
    provider, model = service.get_provider_info()
    print(f"Model: {provider}/{model} ({dimensions} dimensions)")

asyncio.run(embeddings_demo())
```

### RAG (Retrieval-Augmented Generation) Pipeline
```python
async def create_rag_workflow(knowledge_base, question):
    """Complete RAG pipeline with semantic search and generation"""
    
    # Setup embedding service
    embedding_config = graphbit.PyEmbeddingConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="text-embedding-3-small"
    )
    embedding_service = graphbit.PyEmbeddingService(embedding_config)
    
    # Create semantic search
    query_embedding = await embedding_service.embed_text(question)
    doc_embeddings = await embedding_service.embed_texts(knowledge_base)
    
    # Find most relevant documents
    similarities = []
    for i, doc_embedding in enumerate(doc_embeddings):
        similarity = graphbit.PyEmbeddingService.cosine_similarity(
            query_embedding, doc_embedding
        )
        similarities.append((knowledge_base[i], similarity))
    
    # Get top 3 most relevant documents
    top_docs = sorted(similarities, key=lambda x: x[1], reverse=True)[:3]
    context = "\n\n".join([doc for doc, _ in top_docs])
    
    # Create RAG workflow
    builder = graphbit.PyWorkflowBuilder("RAG Pipeline")
    
    rag_node = graphbit.PyWorkflowNode.agent_node(
        name="RAG Agent",
        description="Answers questions using retrieved context",
        agent_id="rag-agent",
        prompt=f"""Context:\n{context}\n\nQuestion: {question}\n\nPlease provide a comprehensive answer based on the context above."""
    )
    
    builder.add_node(rag_node)
    workflow = builder.build()
    
    # Execute RAG workflow
    llm_config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4"
    )
    executor = graphbit.PyWorkflowExecutor(llm_config)
    result = executor.execute(workflow)
    
    return result.get_variable("output")
```

**Learn more**: [Embeddings Guide](07-embeddings-guide.md) | [HuggingFace Integration](08-huggingface-integration.md)

---

## 🔄 Dynamic Workflow Generation

GraphBit supports both static workflows (predefined structure) and dynamic workflows (generated at runtime based on context and objectives).

### Static Workflows
Perfect for predictable, repeatable processes:

```python
def create_content_pipeline():
    """Static workflow - structure defined at design time"""
    builder = graphbit.PyWorkflowBuilder("Content Creation Pipeline")
    
    # Research phase
    researcher = graphbit.PyWorkflowNode.agent_node(
        name="Content Researcher",
        description="Researches topics and trends",
        agent_id="researcher",
        prompt="Research comprehensive information about: {topic}"
    )
    
    # Writing phase  
    writer = graphbit.PyWorkflowNode.agent_node(
        name="Content Writer",
        description="Creates engaging content",
        agent_id="writer",
        prompt="Write compelling content based on research: {research_data}"
    )
    
    # Quality control
    editor = graphbit.PyWorkflowNode.agent_node(
        name="Content Editor", 
        description="Reviews and improves content",
        agent_id="editor",
        prompt="Review and improve this content for clarity and engagement: {draft_content}"
    )
    
    # Connect nodes
    research_id = builder.add_node(researcher)
    writer_id = builder.add_node(writer)
    editor_id = builder.add_node(editor)
    
    builder.connect(research_id, writer_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(writer_id, editor_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Dynamic Workflow Generation
AI-powered workflow creation that adapts to requirements:

```python
async def create_adaptive_workflow(objective, constraints, llm_config):
    """Dynamic workflow - structure determined by AI at runtime"""
    
    # Configure dynamic graph manager
    dynamic_config = graphbit.PyDynamicGraphConfig() \
        .with_max_auto_nodes(10) \
        .with_confidence_threshold(0.7) \
        .with_generation_temperature(0.3) \
        .enable_validation()
    
    manager = graphbit.PyDynamicGraphManager(llm_config, dynamic_config)
    manager.initialize(llm_config, dynamic_config)
    
    # Start with base workflow
    builder = graphbit.PyWorkflowBuilder("Adaptive Workflow")
    
    # Create initial context
    context = graphbit.PyWorkflowContext()
    context.set_variable("objective", objective)
    context.set_variable("constraints", constraints)
    
    # Let AI generate optimal workflow structure
    objectives = [objective, "Ensure quality output", "Minimize execution time"]
    auto_completion = graphbit.PyWorkflowAutoCompletion(manager)
    
    base_workflow = builder.build()
    result = auto_completion.complete_workflow(
        base_workflow, context, objectives
    )
    
    # Get analytics
    analytics = result.get_analytics()
    print(f"Generated {analytics.total_nodes_generated()} nodes")
    print(f"Success rate: {analytics.success_rate():.2%}")
    print(f"Average confidence: {analytics.avg_confidence():.2f}")
    
    return base_workflow
```

**Learn more**: [Complete API Reference](05-complete-api-reference.md#dynamic-workflows)

---

## ⚙️ Multi-LLM Provider Support

GraphBit supports multiple LLM providers with a unified interface:

### Provider Configuration
```python
# OpenAI (cloud)
openai_config = graphbit.PyLlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o"
)

# Anthropic (cloud)
anthropic_config = graphbit.PyLlmConfig.anthropic(
    api_key=os.getenv("ANTHROPIC_API_KEY"),
    model="claude-3-5-sonnet-20241022"
)

# HuggingFace (cloud/local)
hf_config = graphbit.PyLlmConfig.huggingface(
    api_key=os.getenv("HF_API_KEY"),
    model="microsoft/DialoGPT-medium"
)

# Ollama (local)
ollama_config = graphbit.PyLlmConfig.ollama(
    model="llama3.1:8b",
    base_url="http://localhost:11434"
)
```

### Performance Profiles
```python
# High-throughput configuration (batch processing)
executor = graphbit.PyWorkflowExecutor.new_high_throughput(config) \
    .with_max_node_execution_time(300000)  # 5 minutes

# Low-latency configuration (real-time applications)  
executor = graphbit.PyWorkflowExecutor.new_low_latency(config) \
    .without_retries() \
    .with_fail_fast(True)

# Memory-optimized configuration (resource-constrained environments)
executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config) \
    .with_max_node_execution_time(60000)  # 1 minute
```

**Learn more**: [Local LLM Integration](03-local-llm-integration.md) | [HuggingFace Integration](08-huggingface-integration.md)

---

## 🛡️ Reliability & Error Handling

GraphBit provides comprehensive reliability features for production deployments:

### Retry Policies
```python
# Configure exponential backoff retry
retry_config = graphbit.PyRetryConfig(max_attempts=3) \
    .with_exponential_backoff(
        initial_delay_ms=1000,
        multiplier=2.0,
        max_delay_ms=10000
    ) \
    .with_jitter(0.1)

executor = executor.with_retry_config(retry_config)
```

### Circuit Breakers
```python
# Prevent cascading failures
circuit_breaker = graphbit.PyCircuitBreakerConfig(
    failure_threshold=5,
    recovery_timeout_ms=30000
)

executor = executor.with_circuit_breaker_config(circuit_breaker)
```

### Concurrency Control
```python
# Monitor resource usage
stats = await executor.get_concurrency_stats()
print(f"Active tasks: {stats.current_active_tasks}")
```

**Learn more**: [Complete API Reference](05-complete-api-reference.md#reliability-features)

---

## 📄 Document Processing

GraphBit provides built-in document processing capabilities for various formats:

### Document Loader Nodes
```python
# PDF document processing
pdf_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Processor",
    description="Extracts and processes PDF content",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)

# Text file processing
text_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="Text Processor", 
    description="Processes plain text files",
    document_type="text",
    source_path="/path/to/document.txt"
)

# Markdown processing
md_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="Markdown Processor",
    description="Processes markdown documents", 
    document_type="markdown",
    source_path="/path/to/document.md"
)

# HTML processing
html_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="HTML Processor",
    description="Extracts content from HTML files",
    document_type="html", 
    source_path="/path/to/document.html"
)
```

### Document Analysis Pipeline
```python
def create_document_analysis_workflow():
    builder = graphbit.PyWorkflowBuilder("Document Analysis")
    
    # Load document
    loader = graphbit.PyWorkflowNode.document_loader_node(
        "Document Loader", "Loads source document", 
        "pdf", "/path/to/report.pdf"
    )
    
    # Extract key information
    analyzer = graphbit.PyWorkflowNode.agent_node(
        "Document Analyzer", "Analyzes document content",
        "analyzer", "Extract key insights from: {document_content}"
    )
    
    # Generate summary
    summarizer = graphbit.PyWorkflowNode.agent_node(
        "Summarizer", "Creates executive summary",
        "summarizer", "Create executive summary of: {analysis}"
    )
    
    # Connect pipeline
    loader_id = builder.add_node(loader)
    analyzer_id = builder.add_node(analyzer)
    summarizer_id = builder.add_node(summarizer)
    
    builder.connect(loader_id, analyzer_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(analyzer_id, summarizer_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

**Learn more**: [Document Processing Guide](04-document-processing-guide.md)

---

## 🎯 Node Types & Capabilities

GraphBit supports various node types for different workflow requirements:

### Agent Nodes
```python
# Basic agent node
agent = graphbit.PyWorkflowNode.agent_node(
    name="Content Analyzer",
    description="Analyzes content quality and structure", 
    agent_id="analyzer-v1",
    prompt="Analyze the quality and structure of: {content}"
)

# Agent with custom configuration
advanced_agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Technical Writer",
    description="Creates technical documentation",
    agent_id="tech-writer",
    prompt="Write technical documentation for: {feature}",
    max_tokens=2000,
    temperature=0.3
)
```

### Condition Nodes
```python
# Quality gate
quality_check = graphbit.PyWorkflowNode.condition_node(
    name="Quality Gate",
    description="Validates content meets quality standards",
    expression="quality_score > 0.8 && readability_score > 0.7"
)

# Content filter
content_filter = graphbit.PyWorkflowNode.condition_node(
    name="Content Filter",
    description="Filters inappropriate content",
    expression="toxicity_score < 0.1 && sentiment != 'negative'"
)
```

### Transform Nodes
```python
# Text transformations
uppercase = graphbit.PyWorkflowNode.transform_node(
    "Uppercase Transform", "Converts to uppercase", "uppercase"
)

json_extractor = graphbit.PyWorkflowNode.transform_node(
    "JSON Extractor", "Extracts JSON data", "json_extract"
)

# Available transformations: uppercase, lowercase, json_extract, split, join
```

### Delay Nodes
```python
# Rate limiting
delay = graphbit.PyWorkflowNode.delay_node(
    name="Rate Limiter",
    description="Prevents API rate limiting",
    duration_seconds=30
)
```

**Learn more**: [Complete API Reference](05-complete-api-reference.md#node-types)

---

## 🚦 Workflow Control Flow

Control how data and execution flows through your workflows:

### Edge Types
```python
# Data flow (passes output to input)
data_edge = graphbit.PyWorkflowEdge.data_flow()

# Control flow (execution order only)
control_edge = graphbit.PyWorkflowEdge.control_flow()

# Conditional execution
conditional_edge = graphbit.PyWorkflowEdge.conditional("approval_status == 'approved'")
```

### Complex Workflow Example
```python
def create_content_approval_workflow():
    builder = graphbit.PyWorkflowBuilder("Content Approval Pipeline")
    
    # Content creation
    creator = graphbit.PyWorkflowNode.agent_node(
        "Content Creator", "Creates initial content",
        "creator", "Create content about: {topic}"
    )
    
    # Quality assessment
    assessor = graphbit.PyWorkflowNode.agent_node(
        "Quality Assessor", "Assesses content quality", 
        "assessor", "Rate content quality (0-1): {content}"
    )
    
    # Quality gate
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        "Quality Gate", "Checks if content meets standards",
        "quality_score >= 0.7"
    )
    
    # Content reviewer (for high quality content)
    reviewer = graphbit.PyWorkflowNode.agent_node(
        "Content Reviewer", "Reviews approved content",
        "reviewer", "Review and finalize: {content}"
    )
    
    # Content improver (for low quality content)
    improver = graphbit.PyWorkflowNode.agent_node(
        "Content Improver", "Improves low quality content",
        "improver", "Improve this content: {content}"
    )
    
    # Build workflow
    creator_id = builder.add_node(creator)
    assessor_id = builder.add_node(assessor)
    gate_id = builder.add_node(quality_gate)
    reviewer_id = builder.add_node(reviewer)
    improver_id = builder.add_node(improver)
    
    # Connect nodes
    builder.connect(creator_id, assessor_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(assessor_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    
    # Conditional branches
    builder.connect(gate_id, reviewer_id, 
                   graphbit.PyWorkflowEdge.conditional("quality_score >= 0.7"))
    builder.connect(gate_id, improver_id,
                   graphbit.PyWorkflowEdge.conditional("quality_score < 0.7"))
    
    return builder.build()
```

---

## 🔧 CLI Tools

GraphBit includes a powerful CLI for project management and workflow execution:

### Project Management
```bash
# Initialize new project
graphbit init my-workflow-project --path ./projects/

# Validate workflow definition
graphbit validate workflow.json

# Execute workflow
graphbit run workflow.json --config production.json

# Get version info
graphbit version
```

### Environment Configuration
```bash
# Set up API keys
export OPENAI_API_KEY="your-openai-key"
export ANTHROPIC_API_KEY="your-anthropic-key"
export HF_API_KEY="your-huggingface-key"

# Activate conda environment (for development)
conda activate graphbit
```

---

## 📊 Performance & Monitoring

GraphBit provides built-in monitoring and performance analytics:

### Execution Analytics
```python
# Monitor workflow execution
result = executor.execute(workflow)

print(f"Execution time: {result.execution_time_ms()}ms")
print(f"Workflow state: {result.state()}")

# Get detailed statistics
if result.get_stats():
    stats = result.get_stats()
    print(f"Total nodes: {stats.total_nodes}")
    print(f"Success rate: {stats.successful_nodes / stats.total_nodes:.2%}")
    print(f"Average node time: {stats.avg_execution_time_ms:.2f}ms")
```

### Concurrency Monitoring
```python
# Real-time concurrency statistics
stats = await executor.get_concurrency_stats()
print(f"Active tasks: {stats.current_active_tasks}")
print(f"Peak concurrency: {stats.peak_active_tasks}")
print(f"Total acquisitions: {stats.total_permit_acquisitions}")
print(f"Average wait time: {stats.avg_wait_time_ms:.2f}ms")
```

---

## 📖 Learning Resources

### Documentation Sections
1. **[Installation & Setup](00-installation-setup.md)** - Get up and running quickly
2. **[Getting Started](01-getting-started-workflows.md)** - Build your first workflows  
3. **[Use Case Examples](02-use-case-examples.md)** - Real-world applications
4. **[Local LLM Integration](03-local-llm-integration.md)** - Self-hosted models
5. **[Document Processing](04-document-processing-guide.md)** - Handle various file formats
6. **[Complete API Reference](05-complete-api-reference.md)** - Full API documentation
7. **[Quick Reference](06-api-quick-reference.md)** - Common patterns
8. **[Embeddings Guide](07-embeddings-guide.md)** - Semantic search & RAG
9. **[HuggingFace Integration](08-huggingface-integration.md)** - Open-source models
