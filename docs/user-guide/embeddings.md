# Embeddings and Vector Search

GraphBit provides powerful embedding capabilities for semantic search, similarity matching, and intelligent document processing. This guide covers embedding generation, vector storage, and search operations within GraphBit workflows.

## Overview

GraphBit embeddings enable:
- **Semantic Search**: Find documents based on meaning, not just keywords
- **Similarity Matching**: Compare documents, text chunks, or data objects
- **Content Classification**: Categorize content based on semantic features
- **Recommendation Systems**: Build content and product recommendations
- **Document Processing**: Intelligent text analysis and extraction

## Basic Embedding Operations

### Generating Embeddings

```python
import graphbit
import numpy as np
import time
import os
from typing import List, Dict, Any

def create_embedding_workflow():
    """Create workflow for generating embeddings from text."""
    
    builder = graphbit.PyWorkflowBuilder("Embedding Generation Workflow")
    
    # Text preprocessor
    preprocessor = graphbit.PyWorkflowNode.agent_node(
        name="Text Preprocessor",
        description="Preprocesses text for embedding generation",
        agent_id="text_preprocessor",
        prompt="""
        Preprocess this text for embedding generation:
        
        Text: {input}
        
        Tasks:
        1. Clean and normalize the text
        2. Remove noise and irrelevant content
        3. Extract key semantic content
        4. Prepare for optimal embedding quality
        
        Return the cleaned text ready for embedding.
        """
    )
    
    # Embedding generator
    embedding_generator = graphbit.PyWorkflowNode.agent_node(
        name="Embedding Generator",
        description="Generates semantic embeddings from text",
        agent_id="embedding_generator",
        prompt="""
        Generate semantic embeddings for this text:
        
        Processed Text: {preprocessed_text}
        
        Create high-quality embeddings that capture:
        1. Semantic meaning and context
        2. Key concepts and relationships
        3. Domain-specific terminology
        4. Emotional tone and intent
        
        Return embedding vector and metadata.
        """
    )
    
    # Build embedding workflow
    prep_id = builder.add_node(preprocessor)
    gen_id = builder.add_node(embedding_generator)
    
    builder.connect(prep_id, gen_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def create_batch_embedding_workflow():
    """Create workflow for batch embedding generation."""
    
    builder = graphbit.PyWorkflowBuilder("Batch Embedding Workflow")
    
    # Batch splitter
    splitter = graphbit.PyWorkflowNode.transform_node(
        name="Text Batch Splitter",
        description="Splits input into optimal batch sizes",
        transformation="split"
    )
    
    # Parallel embedding processors
    embedding_processors = []
    for i in range(4):  # Process 4 batches in parallel
        processor = graphbit.PyWorkflowNode.agent_node(
            name=f"Batch Processor {i+1}",
            description=f"Processes embedding batch {i+1}",
            agent_id=f"batch_processor_{i+1}",
            prompt=f"""
            Generate embeddings for this batch of texts:
            
            Text Batch: {{batch_{i+1}_texts}}
            
            For each text in the batch:
            1. Generate high-quality embeddings
            2. Maintain consistency across the batch
            3. Include metadata and quality scores
            4. Optimize for downstream processing
            
            Return batch embeddings with metadata.
            """
        )
        embedding_processors.append(processor)
    
    # Batch aggregator
    aggregator = graphbit.PyWorkflowNode.agent_node(
        name="Embedding Aggregator",
        description="Aggregates batch embedding results",
        agent_id="embedding_aggregator",
        prompt="""
        Aggregate embeddings from all batches:
        
        Batch Results: {all_batch_results}
        
        Combine and organize:
        1. All embedding vectors
        2. Metadata and quality scores
        3. Index mappings
        4. Summary statistics
        
        Return consolidated embedding collection.
        """
    )
    
    # Build batch embedding workflow
    split_id = builder.add_node(splitter)
    proc_ids = [builder.add_node(proc) for proc in embedding_processors]
    agg_id = builder.add_node(aggregator)
    
    # Connect splitter to all processors
    for proc_id in proc_ids:
        builder.connect(split_id, proc_id, graphbit.PyWorkflowEdge.data_flow())
    
    # Connect all processors to aggregator
    for proc_id in proc_ids:
        builder.connect(proc_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Vector Storage and Indexing

### Vector Database Integration

```python
class VectorStore:
    """Simple vector storage implementation for embeddings."""
    
    def __init__(self, dimension=1536):  # OpenAI embedding dimension
        self.dimension = dimension
        self.vectors: List[np.ndarray] = []
        self.metadata: List[Dict[str, Any]] = []
        self.index_map: Dict[str, int] = {}
    
    def add_embedding(self, vector, metadata=None, doc_id=None):
        """Add an embedding vector to the store."""
        
        if isinstance(vector, list):
            vector = np.array(vector)
        
        if vector.shape[0] != self.dimension:
            raise ValueError(f"Vector dimension {vector.shape[0]} doesn't match store dimension {self.dimension}")
        
        # Normalize vector for cosine similarity
        normalized_vector = vector / np.linalg.norm(vector)
        
        index = len(self.vectors)
        self.vectors.append(normalized_vector)
        self.metadata.append(metadata or {})
        
        if doc_id:
            self.index_map[doc_id] = index
        
        return index
    
    def search_similar(self, query_vector, top_k=5, threshold=0.7):
        """Search for similar vectors using cosine similarity."""
        
        if not self.vectors:
            return []
        
        if isinstance(query_vector, list):
            query_vector = np.array(query_vector)
        
        # Normalize query vector
        query_normalized = query_vector / np.linalg.norm(query_vector)
        
        # Calculate cosine similarities
        similarities = []
        for i, stored_vector in enumerate(self.vectors):
            similarity = np.dot(query_normalized, stored_vector)
            if similarity >= threshold:
                similarities.append((i, similarity))
        
        # Sort by similarity score (descending)
        similarities.sort(key=lambda x: x[1], reverse=True)
        
        # Return top_k results
        results = []
        for i, (index, score) in enumerate(similarities[:top_k]):
            results.append({
                "index": index,
                "similarity": float(score),
                "metadata": self.metadata[index],
                "vector": self.vectors[index]
            })
        
        return results
    
    def get_stats(self):
        """Get vector store statistics."""
        
        return {
            "total_vectors": len(self.vectors),
            "dimension": self.dimension,
            "memory_usage_mb": len(self.vectors) * self.dimension * 8 / (1024 * 1024),  # Approximate
            "indexed_documents": len(self.index_map)
        }

def create_vector_storage_workflow():
    """Create workflow for storing embeddings in vector database."""
    
    builder = graphbit.PyWorkflowBuilder("Vector Storage Workflow")
    
    # Vector processor
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Vector Processor",
        description="Processes vectors for storage",
        agent_id="vector_processor",
        prompt="""
        Process embeddings for vector storage:
        
        Embeddings: {embeddings}
        Metadata: {metadata}
        
        Prepare for storage:
        1. Validate vector dimensions
        2. Normalize vectors if needed
        3. Enrich metadata
        4. Generate unique identifiers
        5. Optimize for search performance
        
        Return processed vectors ready for storage.
        """
    )
    
    builder.add_node(processor)
    
    return builder.build()
```

## Semantic Search

### Search and Retrieval Workflows

```python
def create_semantic_search_workflow():
    """Create workflow for semantic search operations."""
    
    builder = graphbit.PyWorkflowBuilder("Semantic Search Workflow")
    
    # Query processor
    query_processor = graphbit.PyWorkflowNode.agent_node(
        name="Query Processor",
        description="Processes search queries for optimal matching",
        agent_id="query_processor",
        prompt="""
        Process this search query for semantic matching:
        
        Query: {query}
        Search Context: {search_context}
        
        Process query:
        1. Extract key concepts and intent
        2. Expand with synonyms and related terms
        3. Identify domain-specific terminology
        4. Optimize for embedding generation
        5. Handle multi-language queries if needed
        
        Return processed query ready for embedding.
        """
    )
    
    # Search engine
    search_engine = graphbit.PyWorkflowNode.agent_node(
        name="Semantic Search Engine",
        description="Performs semantic search using embeddings",
        agent_id="search_engine",
        prompt="""
        Perform semantic search using query embeddings:
        
        Query Embeddings: {query_embeddings}
        Vector Database: {vector_store}
        Search Parameters: {search_params}
        
        Execute search:
        1. Find semantically similar documents
        2. Rank by relevance and quality
        3. Apply filters and constraints
        4. Diversify results if needed
        5. Generate relevance scores
        
        Return ranked search results with explanations.
        """
    )
    
    # Build semantic search workflow
    query_id = builder.add_node(query_processor)
    search_id = builder.add_node(search_engine)
    
    builder.connect(query_id, search_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

class SemanticSearchEngine:
    """High-level semantic search engine using GraphBit workflows."""
    
    def __init__(self, vector_store: VectorStore):
        self.vector_store = vector_store
        self.search_workflow = create_semantic_search_workflow()
        self.embedding_workflow = create_embedding_workflow()
        
        # Configure executor
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-3.5-turbo"
        )
        self.executor = graphbit.PyWorkflowExecutor(config)
    
    def add_document(self, text, metadata=None, doc_id=None):
        """Add a document to the search index."""
        
        # Generate embeddings for the document
        embedding_result = self.executor.execute_with_input(
            self.embedding_workflow, 
            {"input": text, "metadata": metadata}
        )
        
        # Extract embedding vector (mock implementation)
        embedding_vector = self._extract_embedding_from_result(embedding_result)
        
        # Store in vector database
        doc_metadata = metadata or {}
        doc_metadata.update({
            "text": text,
            "doc_id": doc_id,
            "indexed_at": time.time()
        })
        
        index = self.vector_store.add_embedding(
            vector=embedding_vector,
            metadata=doc_metadata,
            doc_id=doc_id
        )
        
        return index
    
    def search(self, query, top_k=5, threshold=0.7):
        """Search for semantically similar documents."""
        
        # Generate query embeddings
        query_result = self.executor.execute_with_input(
            self.embedding_workflow,
            {"input": query}
        )
        
        query_embedding = self._extract_embedding_from_result(query_result)
        
        # Perform vector search
        results = self.vector_store.search_similar(
            query_vector=query_embedding,
            top_k=top_k,
            threshold=threshold
        )
        
        return results
    
    def _extract_embedding_from_result(self, result):
        """Extract embedding vector from workflow result."""
        # This would parse the actual workflow output format
        # For demonstration, returning a mock embedding
        return np.random.rand(1536)  # OpenAI embedding dimension
```

## Document Processing with Embeddings

### Intelligent Document Analysis

```python
def create_document_processing_workflow():
    """Create workflow for intelligent document processing using embeddings."""
    
    builder = graphbit.PyWorkflowBuilder("Document Processing Workflow")
    
    # Document analyzer
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Document Analyzer",
        description="Analyzes document structure and content",
        agent_id="doc_analyzer",
        prompt="""
        Analyze this document for intelligent processing:
        
        Document: {document}
        Processing Goals: {goals}
        
        Analyze:
        1. Document structure and sections
        2. Key topics and themes
        3. Entity mentions and relationships
        4. Content quality and relevance
        5. Processing strategy recommendations
        
        Return analysis with processing plan.
        """
    )
    
    # Content chunker
    chunker = graphbit.PyWorkflowNode.agent_node(
        name="Content Chunker",
        description="Intelligently chunks content for optimal embeddings",
        agent_id="chunker",
        prompt="""
        Chunk document content for optimal embedding generation:
        
        Document Analysis: {doc_analysis}
        Content: {content}
        
        Create chunks that:
        1. Preserve semantic coherence
        2. Maintain optimal size for embeddings
        3. Respect document structure
        4. Include necessary context
        5. Enable effective retrieval
        
        Return semantically coherent chunks with metadata.
        """
    )
    
    # Build document processing workflow
    analyze_id = builder.add_node(analyzer)
    chunk_id = builder.add_node(chunker)
    
    builder.connect(analyze_id, chunk_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def create_similarity_clustering_workflow():
    """Create workflow for clustering documents by similarity."""
    
    builder = graphbit.PyWorkflowBuilder("Similarity Clustering Workflow")
    
    # Similarity calculator
    similarity_calc = graphbit.PyWorkflowNode.agent_node(
        name="Similarity Calculator",
        description="Calculates pairwise document similarities",
        agent_id="similarity_calc",
        prompt="""
        Calculate similarities between document embeddings:
        
        Document Embeddings: {embeddings}
        Similarity Method: {method}
        
        Calculate:
        1. Pairwise similarity matrix
        2. Nearest neighbor relationships
        3. Similarity distributions
        4. Outlier detection
        5. Clustering hints
        
        Return comprehensive similarity analysis.
        """
    )
    
    # Cluster analyzer
    cluster_analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Cluster Analyzer",
        description="Analyzes and describes document clusters",
        agent_id="cluster_analyzer",
        prompt="""
        Analyze and describe document clusters:
        
        Document Clusters: {clusters}
        Original Documents: {documents}
        
        For each cluster:
        1. Generate descriptive labels
        2. Identify common themes
        3. Extract representative documents
        4. Calculate cluster quality metrics
        5. Suggest cluster uses
        
        Return cluster analysis and recommendations.
        """
    )
    
    # Build clustering workflow
    sim_id = builder.add_node(similarity_calc)
    analyze_id = builder.add_node(cluster_analyzer)
    
    builder.connect(sim_id, analyze_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Practical Applications

### Content Recommendation System

```python
class ContentRecommendationEngine:
    """Content recommendation system using embeddings."""
    
    def __init__(self):
        self.vector_store = VectorStore()
        self.user_profiles = {}  # User preference embeddings
        self.content_embeddings = {}  # Content ID to embedding mapping
        
        # Workflows
        self.embedding_workflow = create_embedding_workflow()
        
        # Executor
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-3.5-turbo"
        )
        self.executor = graphbit.PyWorkflowExecutor(config)
    
    def add_content(self, content_id, text, metadata=None):
        """Add content to the recommendation system."""
        
        # Generate content embeddings
        embedding_result = self.executor.execute_with_input(
            self.embedding_workflow,
            {"input": text, "metadata": metadata}
        )
        
        # Store content embedding (mock implementation)
        embedding_vector = np.random.rand(1536)  # Would extract from actual result
        
        self.vector_store.add_embedding(
            vector=embedding_vector,
            metadata={"content_id": content_id, "text": text, **(metadata or {})},
            doc_id=content_id
        )
        
        self.content_embeddings[content_id] = embedding_vector
        
        return content_id
    
    def get_recommendations(self, user_id, num_recommendations=5):
        """Get personalized recommendations for a user."""
        
        if user_id not in self.user_profiles:
            return []
        
        user_profile = self.user_profiles[user_id]
        user_embedding = user_profile["embedding"]
        
        # Find similar content
        similar_content = self.vector_store.search_similar(
            query_vector=user_embedding,
            top_k=num_recommendations * 2,  # Get more for filtering
            threshold=0.5
        )
        
        # Generate final recommendations
        recommendations = []
        for content in similar_content[:num_recommendations]:
            recommendations.append({
                "content_id": content["metadata"]["content_id"],
                "similarity": content["similarity"],
                "text": content["metadata"]["text"],
                "explanation": f"Recommended based on {content['similarity']:.2f} similarity to your preferences"
            })
        
        return recommendations

def demo_embedding_system():
    """Demonstrate complete embedding system."""
    
    # Initialize recommendation engine
    engine = ContentRecommendationEngine()
    
    # Add sample content
    contents = [
        ("doc1", "Introduction to machine learning algorithms and their applications", {"category": "AI", "difficulty": "beginner"}),
        ("doc2", "Advanced deep learning techniques for computer vision", {"category": "AI", "difficulty": "advanced"}),
        ("doc3", "Web development with modern JavaScript frameworks", {"category": "Programming", "difficulty": "intermediate"}),
        ("doc4", "Data visualization best practices and tools", {"category": "Data Science", "difficulty": "intermediate"}),
        ("doc5", "Cloud computing fundamentals and deployment strategies", {"category": "Infrastructure", "difficulty": "beginner"})
    ]
    
    for content_id, text, metadata in contents:
        engine.add_content(content_id, text, metadata)
        print(f"Added content: {content_id}")
    
    return engine
```

## Best Practices

### 1. Embedding Optimization

```python
def optimize_embeddings_for_domain(domain_type, content_samples):
    """Optimize embedding generation for specific domains."""
    
    optimization_strategies = {
        "technical": {
            "preprocessing": "preserve_technical_terms",
            "chunking": "semantic_boundaries",
            "model_tuning": "technical_vocabulary"
        },
        "creative": {
            "preprocessing": "preserve_style_elements",
            "chunking": "narrative_structure", 
            "model_tuning": "creative_patterns"
        },
        "scientific": {
            "preprocessing": "preserve_formulas_citations",
            "chunking": "logical_sections",
            "model_tuning": "scientific_terminology"
        }
    }
    
    strategy = optimization_strategies.get(domain_type, optimization_strategies["technical"])
    
    return {
        "domain": domain_type,
        "strategy": strategy,
        "recommended_chunk_size": 512 if domain_type == "technical" else 256,
        "overlap_ratio": 0.1,
        "quality_threshold": 0.8
    }
```

### 2. Performance Optimization

```python
def optimize_vector_search_performance():
    """Optimize vector search performance."""
    
    optimization_tips = {
        "indexing": [
            "Use approximate nearest neighbor (ANN) algorithms for large datasets",
            "Implement hierarchical indexing for multi-level search",
            "Consider dimensionality reduction for very high-dimensional embeddings"
        ],
        "caching": [
            "Cache frequently accessed embeddings",
            "Implement query result caching",
            "Use embedding compression for storage efficiency"
        ],
        "batching": [
            "Process embeddings in batches for better throughput",
            "Use parallel processing for independent operations",
            "Optimize batch sizes based on available memory"
        ]
    }
    
    return optimization_tips
```

GraphBit's embedding capabilities enable powerful semantic search, intelligent document processing, and sophisticated content recommendation systems. Use these patterns and workflows to build embedding-powered applications that understand and process content at a semantic level. 
