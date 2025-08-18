# Semantic Search and Analysis

This example demonstrates how to build a semantic search and analysis system using GraphBit's embedding capabilities and LLM integration.

## Overview

We'll create a system that:
1. **Embeds** text documents for semantic search
2. **Searches** for semantically similar content  
3. **Analyzes** results with LLM insights
4. **Compares** multiple documents for similarity
5. **Generates** intelligent summaries

## Complete Example

```python
from graphbit import init, EmbeddingConfig, EmbeddingClient, LlmConfig, LlmClient
import os
from typing import List, Dict

class SemanticSearchSystem:
    def __init__(self, openai_api_key: str):
        """Initialize the semantic search system."""
        # Initialize GraphBit
        init(enable_tracing=True)
        
        # Configure embeddings
        self.embedding_config = EmbeddingConfig.openai(
            api_key=openai_api_key,
            model="text-embedding-3-small"
        )
        self.embedding_client = EmbeddingClient(self.embedding_config)
        
        # Configure LLM for analysis
        self.llm_config = LlmConfig.openai(
            api_key=openai_api_key,
            model="gpt-4o-mini"
        )
        self.llm_client = LlmClient(self.llm_config)
        
        # Document storage
        self.documents = []
        self.embeddings = []
        self.document_index = {}
    
    def add_documents(self, documents: List[Dict[str, str]]):
        """Add documents to the search index."""
        print(f"Adding {len(documents)} documents to index...")
        
        # Extract text for embedding
        texts = [doc['content'] for doc in documents]
        
        # Generate embeddings in batch
        embeddings = self.embedding_client.embed_many(texts)
        
        # Store documents and embeddings
        start_idx = len(self.documents)
        for i, (doc, embedding) in enumerate(zip(documents, embeddings)):
            doc_id = start_idx + i
            self.documents.append(doc)
            self.embeddings.append(embedding)
            self.document_index[doc_id] = {
                'title': doc.get('title', f'Document {doc_id}'),
                'content_preview': doc['content'][:200] + '...',
                'metadata': doc.get('metadata', {})
            }
        
        print(f"Added {len(documents)} documents. Total: {len(self.documents)}")
    
    def search(self, query: str, top_k: int = 5) -> List[Dict]:
        """Search for semantically similar documents."""
        if not self.documents:
            print("No documents in index")
            return []
        
        print(f"🔍 Searching for: '{query}'")
        
        # Embed the query
        query_embedding = self.embedding_client.embed(query)
        
        # Calculate similarities
        similarities = []
        for i, doc_embedding in enumerate(self.embeddings):
            similarity = EmbeddingClient.similarity(
                query_embedding, 
                doc_embedding
            )
            similarities.append({
                'doc_id': i,
                'similarity': similarity,
                'title': self.document_index[i]['title'],
                'content_preview': self.document_index[i]['content_preview'],
                'metadata': self.document_index[i]['metadata']
            })
        
        # Sort by similarity and return top-k
        similarities.sort(key=lambda x: x['similarity'], reverse=True)
        return similarities[:top_k]
    
    def analyze_search_results(self, query: str, results: List[Dict]) -> str:
        """Analyze search results with LLM insights."""
        if not results:
            return "No results to analyze."
        
        print("🤖 Analyzing search results with LLM...")
        
        # Prepare context for LLM
        results_text = "\n\n".join([
            f"Document {i+1}: {result['title']}\n"
            f"Similarity: {result['similarity']:.3f}\n"
            f"Preview: {result['content_preview']}"
            for i, result in enumerate(results)
        ])
        
        prompt = f"""Analyze these search results for the query: "{query}"

Search Results:
{results_text}

Provide:
1. Summary of what the results reveal about the query
2. Key themes and patterns across the documents
3. Quality assessment of the search matches
4. Recommendations for refining the search or exploring related topics
5. Most relevant documents and why

Be insightful and practical in your analysis.
"""
        
        try:
            analysis = self.llm_client.complete(prompt)
            return analysis
        except Exception as e:
            return f"Analysis failed: {str(e)}"
    
    def compare_documents(self, doc_ids: List[int]) -> Dict:
        """Compare multiple documents for similarity."""
        if len(doc_ids) < 2:
            return {"error": "Need at least 2 documents to compare"}
        
        print(f"Comparing {len(doc_ids)} documents...")
        
        # Get embeddings for specified documents
        selected_embeddings = [self.embeddings[doc_id] for doc_id in doc_ids]
        selected_docs = [self.documents[doc_id] for doc_id in doc_ids]
        
        # Calculate pairwise similarities
        comparisons = []
        for i in range(len(doc_ids)):
            for j in range(i + 1, len(doc_ids)):
                similarity = EmbeddingClient.similarity(
                    selected_embeddings[i],
                    selected_embeddings[j]
                )
                comparisons.append({
                    'doc1_id': doc_ids[i],
                    'doc2_id': doc_ids[j],
                    'doc1_title': self.document_index[doc_ids[i]]['title'],
                    'doc2_title': self.document_index[doc_ids[j]]['title'],
                    'similarity': similarity
                })
        
        # Sort by similarity
        comparisons.sort(key=lambda x: x['similarity'], reverse=True)
        
        return {
            'comparisons': comparisons,
            'most_similar': comparisons[0] if comparisons else None,
            'least_similar': comparisons[-1] if comparisons else None,
            'average_similarity': sum(c['similarity'] for c in comparisons) / len(comparisons) if comparisons else 0
        }
    
    def generate_document_summary(self, doc_id: int) -> str:
        """Generate an intelligent summary of a document."""
        if doc_id >= len(self.documents):
            return "Document not found"
        
        document = self.documents[doc_id]
        print(f"📝 Generating summary for: {self.document_index[doc_id]['title']}")
        
        prompt = f"""Summarize this document concisely:

Title: {document.get('title', 'Untitled')}
Content: {document['content']}

Provide:
1. Main topics and themes
2. Key insights or findings
3. Important details or data points
4. Practical implications or takeaways

Keep the summary informative but concise (2-3 paragraphs).
"""
        
        try:
            summary = self.llm_client.complete(prompt, max_tokens=500)
            return summary
        except Exception as e:
            return f"Summary generation failed: {str(e)}"
    
    def get_statistics(self) -> Dict:
        """Get system statistics."""
        if not self.documents:
            return {"documents": 0, "embeddings": 0}
        
        # Calculate average similarity across all documents
        all_similarities = []
        for i in range(len(self.embeddings)):
            for j in range(i + 1, len(self.embeddings)):
                similarity = EmbeddingClient.similarity(
                    self.embeddings[i],
                    self.embeddings[j]
                )
                all_similarities.append(similarity)
        
        return {
            "total_documents": len(self.documents),
            "total_embeddings": len(self.embeddings),
            "average_document_similarity": sum(all_similarities) / len(all_similarities) if all_similarities else 0,
            "max_similarity": max(all_similarities) if all_similarities else 0,
            "min_similarity": min(all_similarities) if all_similarities else 0
        }

# Example usage
def main():
    """Demonstrate the semantic search system."""
    
    # Set up API key
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("Please set OPENAI_API_KEY environment variable")
        return
    
    # Create search system
    search_system = SemanticSearchSystem(api_key)
    
    # Sample documents
    sample_documents = [
        {
            "title": "Introduction to Machine Learning",
            "content": """Machine learning is a subset of artificial intelligence that enables computers to learn and improve from experience without being explicitly programmed. It focuses on developing algorithms that can automatically learn patterns from data and make predictions or decisions. The field encompasses supervised learning, unsupervised learning, and reinforcement learning approaches.""",
            "metadata": {"category": "technology", "difficulty": "beginner"}
        },
        {
            "title": "Deep Learning Neural Networks",
            "content": """Deep learning uses artificial neural networks with multiple layers to model and understand complex patterns in data. These networks can automatically learn hierarchical representations of data, making them particularly effective for tasks like image recognition, natural language processing, and speech recognition. Popular architectures include convolutional neural networks and recurrent neural networks.""",
            "metadata": {"category": "technology", "difficulty": "advanced"}
        },
        {
            "title": "Sustainable Energy Solutions",
            "content": """Renewable energy sources like solar, wind, and hydroelectric power are becoming increasingly important for environmental sustainability. These technologies offer clean alternatives to fossil fuels and can help reduce carbon emissions. Energy storage systems and smart grid technologies are crucial for integrating renewable energy into existing power infrastructure.""",
            "metadata": {"category": "environment", "difficulty": "intermediate"}
        },
        {
            "title": "Climate Change Impacts",
            "content": """Climate change is causing significant environmental disruptions including rising sea levels, extreme weather events, and ecosystem changes. The scientific consensus indicates human activities, particularly greenhouse gas emissions, are the primary drivers. Mitigation strategies include transitioning to renewable energy, improving energy efficiency, and implementing carbon capture technologies.""",
            "metadata": {"category": "environment", "difficulty": "intermediate"}
        },
        {
            "title": "Digital Marketing Strategies",
            "content": """Modern digital marketing encompasses social media marketing, search engine optimization, content marketing, and data analytics. Successful campaigns require understanding customer behavior, creating engaging content, and leveraging multiple digital channels. Personalization and automation tools are increasingly important for reaching target audiences effectively.""",
            "metadata": {"category": "business", "difficulty": "beginner"}
        }
    ]
    
    # Add documents to index
    search_system.add_documents(sample_documents)
    
    # Perform searches
    queries = [
        "artificial intelligence and neural networks",
        "renewable energy and sustainability",
        "online marketing and social media"
    ]
    
    for query in queries:
        print(f"\n{'='*60}")
        print(f"Query: {query}")
        print('='*60)
        
        # Search for similar documents
        results = search_system.search(query, top_k=3)
        
        # Display results
        print("\n🔍 Search Results:")
        for i, result in enumerate(results, 1):
            print(f"{i}. {result['title']} (Similarity: {result['similarity']:.3f})")
            print(f"   {result['content_preview']}")
            print()
        
        # Analyze results with LLM
        analysis = search_system.analyze_search_results(query, results)
        print("LLM Analysis:")
        print(analysis)
        print()
    
    # Compare documents
    print(f"\n{'='*60}")
    print("Document Comparison")
    print('='*60)
    comparison = search_system.compare_documents([0, 1, 2])
    
    print("Document Similarities:")
    for comp in comparison['comparisons'][:3]:
        print(f"{comp['doc1_title']} <-> {comp['doc2_title']}: {comp['similarity']:.3f}")
    
    # Generate summaries
    print(f"\n{'='*60}")
    print("Document Summaries")
    print('='*60)
    
    for i in range(min(2, len(sample_documents))):
        summary = search_system.generate_document_summary(i)
        print(f"\n📝 Summary of '{search_system.document_index[i]['title']}':")
        print(summary)
    
    # Show statistics
    print(f"\n{'='*60}")
    print("System Statistics")
    print('='*60)
    stats = search_system.get_statistics()
    for key, value in stats.items():
        if isinstance(value, float):
            print(f"{key}: {value:.3f}")
        else:
            print(f"{key}: {value}")

if __name__ == "__main__":
    main()
```

## Advanced Features

### Batch Processing with Async Operations

```python
import asyncio
from graphbit import init, LlmConfig, LlmClient
import os

async def process_large_document_collection():
    """Process large document collections asynchronously."""
    
    init()
    
    # Configure for high-throughput processing
    llm_config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    llm_client = LlmClient(llm_config, debug=False)
    
    # Large collection of documents (simulated)
    documents = [f"Document {i} content about various topics..." for i in range(100)]
    
    # Process in batches
    batch_size = 10
    results = []
    
    for i in range(0, len(documents), batch_size):
        batch = documents[i:i + batch_size]
        print(f"Processing batch {i//batch_size + 1}...")
        
        # Use batch completion for efficiency
        prompts = [f"Summarize this document: {doc}" for doc in batch]
        
        try:
            batch_results = await llm_client.complete_batch(
                prompts,
                max_tokens=200,
                temperature=0.3,
                max_concurrency=5
            )
            results.extend(batch_results)
            print(f"Completed batch {i//batch_size + 1}")
        except Exception as e:
            print(f"Batch {i//batch_size + 1} failed: {e}")
    
    return results

# Usage
results = asyncio.run(process_large_document_collection())
```

### Multi-Provider Search System

```python
from graphbit import init, EmbeddingConfig, EmbeddingClient, LlmConfig, LlmClient
import os
from typing import List, Dict

def create_multi_provider_system():
    """Create search system with multiple LLM providers."""
    
    init()
    
    # OpenAI for embeddings
    embedding_config = EmbeddingConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="text-embedding-3-small"
    )
    embedding_client = EmbeddingClient(embedding_config)
    
    # Multiple LLM providers for analysis
    providers = {
        'openai': LlmClient(
            LlmConfig.openai(
                api_key=os.getenv("OPENAI_API_KEY"),
                model="gpt-4o-mini"
            )
        ),
        'anthropic': LlmClient(
            LlmConfig.anthropic(
                api_key=os.getenv("ANTHROPIC_API_KEY"),
                model="claude-3-5-haiku-20241022"
            )
        ) if os.getenv("ANTHROPIC_API_KEY") else None,
        'ollama': LlmClient(
            LlmConfig.ollama("llama3.2")
        )
    }
    
    # Filter available providers
    available_providers = {k: v for k, v in providers.items() if v is not None}
    
    def analyze_with_multiple_providers(query: str, results: List[Dict]) -> Dict[str, str]:
        """Get analysis from multiple LLM providers."""
        analyses = {}
        
        prompt = f"Analyze these search results for '{query}': {results}"
        
        for provider_name, client in available_providers.items():
            try:
                print(f"🤖 Getting analysis from {provider_name}...")
                analysis = client.complete(prompt, max_tokens=300)
                analyses[provider_name] = analysis
            except Exception as e:
                analyses[provider_name] = f"Error: {str(e)}"
        
        return analyses
    
    return embedding_client, analyze_with_multiple_providers

# Usage
embedding_client, analyzer = create_multi_provider_system()
```

### Workflow-Based Semantic Analysis

```python
from graphbit import init, LlmConfig, Executor, Workflow, Node
import os

def create_semantic_analysis_workflow():
    """Create comprehensive semantic analysis workflow."""
    
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = Executor(config, debug=True)
    workflow = Workflow("Semantic Analysis Pipeline")
    
    # Document Preprocessor
    preprocessor = Node.agent(
        name="Document Preprocessor",
        prompt="""Preprocess this document for semantic analysis:

{document}

Tasks:
- Extract key topics and themes
- Identify important entities (people, places, concepts)
- Determine document type and structure
- Note any special formatting or data

Provide structured output for further analysis.
""",
        agent_id="preprocessor"
    )
    
    # Semantic Analyzer
    analyzer = Node.agent(
        name="Semantic Analyzer",
        prompt="""Perform semantic analysis on this preprocessed document:

{preprocessed_document}

Analyze:
- Semantic relationships between concepts
- Document sentiment and tone
- Key insights and findings
- Conceptual density and complexity
- Domain-specific terminology

Provide detailed semantic breakdown.
""",
        agent_id="semantic_analyzer"
    )
    
    # Insight Generator
    insight_generator = Node.agent(
        name="Insight Generator",
        prompt="""Generate actionable insights from this semantic analysis:

{semantic_analysis}

Create:
- Summary of key findings
- Practical implications
- Related topics for exploration
- Recommendations for further analysis
- Quality assessment of the content

Focus on useful, actionable insights.
""",
        agent_id="insight_generator"
    )
    
    # Add nodes and connect
    prep_id = workflow.add_node(preprocessor)
    analyze_id = workflow.add_node(analyzer)
    insight_id = workflow.add_node(insight_generator)
    
    workflow.connect(prep_id, analyze_id)
    workflow.connect(analyze_id, insight_id)
    
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_semantic_analysis_workflow()
result = executor.execute(workflow)
```

### HuggingFace Integration

```python
from graphbit import init, EmbeddingConfig, EmbeddingClient
import os

def create_huggingface_search_system():
    """Create search system using HuggingFace embeddings."""
    
    init()
    
    # Configure HuggingFace embeddings
    embedding_config = EmbeddingConfig.huggingface(
        api_key=os.getenv("HUGGINGFACE_API_KEY"),
        model="intfloat/multilingual-e5-large"
    )
    
    embedding_client = EmbeddingClient(embedding_config)
    
    # Test embedding generation
    test_texts = [
        "Machine learning algorithms",
        "Natural language processing",
        "Computer vision applications"
    ]
    
    try:
        embeddings = embedding_client.embed_many(test_texts)
        print(f"Generated {len(embeddings)} embeddings with HuggingFace")
        
        # Calculate similarities
        for i in range(len(embeddings)):
            for j in range(i + 1, len(embeddings)):
                similarity = EmbeddingClient.similarity(
                    embeddings[i], 
                    embeddings[j]
                )
                print(f"'{test_texts[i]}' <-> '{test_texts[j]}': {similarity:.3f}")
    
    except Exception as e:
        print(f"HuggingFace embedding failed: {e}")
    
    return embedding_client

# Usage (requires HUGGINGFACE_API_KEY)
# client = create_huggingface_search_system()
```

## System Monitoring and Health

```python
from graphbit import init, EmbeddingConfig, EmbeddingClient, health_check, get_system_info
import os

def monitor_semantic_search_system():
    """Monitor system health and performance."""
    
    init()
    
    # Check system health
    health = health_check()
    print("System Health:")
    for key, value in health.items():
        status = "Ok!" if value else "Not Ok!"
        print(f"  {status} {key}: {value}")
    
    # Get system information
    info = get_system_info()
    print("\nSystem Information:")
    for key, value in info.items():
        print(f"  {key}: {value}")
    
    # Test embedding client performance
    embedding_config = EmbeddingConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="text-embedding-3-small"
    )
    
    try:
        client = EmbeddingClient(embedding_config)
        
        # Performance test
        import time
        start_time = time.time()
        
        test_embedding = client.embed("Performance test text")
        
        end_time = time.time()
        duration = (end_time - start_time) * 1000
        
        print(f"\n⚡ Performance Test:")
        print(f"  Embedding generation: {duration:.2f}ms")
        print(f"  Embedding dimension: {len(test_embedding)}")
        
        return True
    
    except Exception as e:
        print(f"\nPerformance test failed: {e}")
        return False

# Usage
system_healthy = monitor_semantic_search_system()
```

## Key Benefits

### Semantic Understanding
- **Deep Search**: Beyond keyword matching to semantic similarity
- **Context Awareness**: Understanding document relationships and themes
- **Intelligent Analysis**: LLM-powered insights and recommendations

### Scalability
- **Batch Processing**: Efficient handling of large document collections
- **Async Operations**: Non-blocking processing for better performance
- **Multiple Providers**: Flexibility to use different LLM providers

### Flexibility
- **Multi-Provider Support**: OpenAI, Anthropic, Ollama, HuggingFace
- **Workflow Integration**: Combine with GraphBit's workflow system
- **Custom Analysis**: Tailored semantic analysis pipelines

This example demonstrates how GraphBit's embedding capabilities can be combined with LLM analysis to create powerful semantic search and analysis systems. 
