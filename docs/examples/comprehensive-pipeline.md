# Comprehensive AI Pipeline

This example demonstrates a complete AI pipeline combining GraphBit's workflow system, embedding capabilities, and LLM integration for building sophisticated AI applications.

## Overview

We'll build an intelligent document analysis and recommendation system that:
1. **Processes** documents with semantic understanding
2. **Analyzes** content using LLM workflows
3. **Embeds** documents for similarity search
4. **Generates** intelligent recommendations
5. **Monitors** system performance and health

## Complete System Implementation

```python
import graphbit
import os
import json
import asyncio
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
import time

@dataclass
class Document:
    """Document data structure."""
    id: str
    title: str
    content: str
    category: str
    metadata: Dict[str, any]

@dataclass
class AnalysisResult:
    """Analysis result data structure."""
    document_id: str
    summary: str
    key_topics: List[str]
    sentiment: str
    quality_score: float
    recommendations: List[str]

class IntelligentDocumentPipeline:
    """Comprehensive document analysis and recommendation system."""
    
    def __init__(self, openai_api_key: str, anthropic_api_key: Optional[str] = None):
        """Initialize the intelligent document pipeline."""
        
        # Initialize GraphBit
        graphbit.init(log_level="info", enable_tracing=True)
        
        # Configure multiple LLM providers
        self.llm_configs = {
            'openai': graphbit.LlmConfig.openai(openai_api_key, "gpt-4o-mini"),
            'openai_fast': graphbit.LlmConfig.openai(openai_api_key, "gpt-4o-mini")
        }
        
        if anthropic_api_key:
            self.llm_configs['anthropic'] = graphbit.LlmConfig.anthropic(
                anthropic_api_key, 
                "claude-3-5-sonnet-20241022"
            )
        
        # Configure embeddings
        self.embedding_config = graphbit.EmbeddingConfig.openai(
            openai_api_key,
            "text-embedding-3-small"
        )
        self.embedding_client = graphbit.EmbeddingClient(self.embedding_config)
        
        # Create executors for different use cases
        self.executors = {
            'analysis': graphbit.Executor(
                self.llm_configs['openai'],
                timeout_seconds=180,
                debug=True
            ),
            'batch': graphbit.Executor.new_high_throughput(
                self.llm_configs['openai_fast'],
                timeout_seconds=120,
                debug=False
            ),
            'fast': graphbit.Executor.new_low_latency(
                self.llm_configs['openai_fast'],
                timeout_seconds=60,
                debug=False
            )
        }
        
        # Document storage
        self.documents: List[Document] = []
        self.embeddings: List[List[float]] = []
        self.analysis_results: Dict[str, AnalysisResult] = {}
        
        # Create workflows
        self.workflows = self.create_workflows()
    
    def create_workflows(self) -> Dict[str, graphbit.Workflow]:
        """Create all workflow pipelines."""
        workflows = {}
        
        # 1. Document Analysis Workflow
        workflows['analysis'] = self.create_document_analysis_workflow()
        
        # 2. Content Enhancement Workflow  
        workflows['enhancement'] = self.create_content_enhancement_workflow()
        
        # 3. Quality Assessment Workflow
        workflows['quality'] = self.create_quality_assessment_workflow()
        
        # 4. Recommendation Generation Workflow
        workflows['recommendation'] = self.create_recommendation_workflow()
        
        return workflows
    
    def create_document_analysis_workflow(self) -> graphbit.Workflow:
        """Create comprehensive document analysis workflow."""
        
        workflow = graphbit.Workflow("Document Analysis Pipeline")
        
        # Content Preprocessor
        preprocessor = graphbit.Node.agent(
            name="Content Preprocessor",
            prompt="""Preprocess this document for analysis:

Title: {title}
Content: {content}
Category: {category}

Tasks:
1. Extract key information and structure
2. Identify main topics and themes  
3. Note any special content (data, quotes, references)
4. Assess content complexity and readability
5. Identify potential quality issues

Provide structured preprocessing results.
""",
            agent_id="preprocessor"
        )
        
        # Content Analyzer
        analyzer = graphbit.Node.agent(
            name="Content Analyzer",
            prompt="""Analyze this preprocessed document content:

{preprocessed_content}

Perform comprehensive analysis:

1. **Topic Analysis**: Identify and rank key topics (max 5)
2. **Sentiment Analysis**: Determine overall sentiment (positive/negative/neutral)
3. **Content Quality**: Rate quality 1-10 based on clarity, depth, accuracy
4. **Key Insights**: Extract 3-5 main insights or findings
5. **Content Type**: Classify the content type and purpose

Format response as JSON with clear sections for each analysis type.
""",
            agent_id="analyzer"
        )
        
        # Summary Generator
        summarizer = graphbit.Node.agent(
            name="Summary Generator",
            prompt="""Generate a comprehensive summary based on this analysis:

Content Analysis: {analysis_results}
Original Content: {preprocessed_content}

Create:
1. **Executive Summary**: 2-3 sentence overview
2. **Key Points**: Bullet points of main findings
3. **Topics**: List of main topics with brief descriptions
4. **Insights**: Notable insights or conclusions
5. **Context**: How this content fits into its category

Keep summary informative yet concise.
""",
            agent_id="summarizer"
        )
        
        # Connect the pipeline
        prep_id = workflow.add_node(preprocessor)
        analyze_id = workflow.add_node(analyzer)
        summary_id = workflow.add_node(summarizer)
        
        workflow.connect(prep_id, analyze_id)
        workflow.connect(analyze_id, summary_id)
        
        workflow.validate()
        return workflow
    
    def create_content_enhancement_workflow(self) -> graphbit.Workflow:
        """Create content enhancement and optimization workflow."""
        
        workflow = graphbit.Workflow("Content Enhancement Pipeline")
        
        # Content Reviewer
        reviewer = graphbit.Node.agent(
            name="Content Reviewer",
            prompt="""Review this content for enhancement opportunities:

{content}

Evaluate:
1. **Clarity**: Is the content clear and well-structured?
2. **Completeness**: Are there missing elements or gaps?
3. **Engagement**: How engaging is the content?
4. **Accuracy**: Are there any factual concerns?
5. **Optimization**: What improvements could be made?

Provide specific, actionable recommendations.
""",
            agent_id="reviewer"
        )
        
        # Enhancement Suggester
        enhancer = graphbit.Node.agent(
            name="Enhancement Suggester",
            prompt="""Based on this review, suggest specific enhancements:

Review Results: {review_results}
Original Content: {content}

Suggest improvements for:
1. **Structure**: Better organization or formatting
2. **Content**: Additional information or clarifications
3. **Engagement**: Ways to make content more engaging
4. **SEO**: Search optimization opportunities
5. **Accessibility**: Improvements for better accessibility

Prioritize suggestions by impact and feasibility.
""",
            agent_id="enhancer"
        )
        
        # Connect pipeline
        review_id = workflow.add_node(reviewer)
        enhance_id = workflow.add_node(enhancer)
        
        workflow.connect(review_id, enhance_id)
        
        workflow.validate()
        return workflow
    
    def create_quality_assessment_workflow(self) -> graphbit.Workflow:
        """Create quality assessment workflow."""
        
        workflow = graphbit.Workflow("Quality Assessment Pipeline")
        
        # Quality Assessor
        assessor = graphbit.Node.agent(
            name="Quality Assessor",
            prompt="""Assess the quality of this content comprehensively:

{content}

Rate (1-10) and explain:
1. **Accuracy**: Factual correctness and reliability
2. **Clarity**: How clear and understandable the content is
3. **Depth**: Level of detail and thoroughness
4. **Structure**: Organization and logical flow
5. **Relevance**: How relevant to its intended purpose
6. **Originality**: Uniqueness and fresh insights

Provide overall quality score and detailed feedback.
""",
            agent_id="assessor"
        )
        
        # Quality Gate
        quality_gate = graphbit.Node.condition(
            name="Quality Gate",
            expression="overall_quality >= 7"
        )
        
        # Improvement Recommender
        improver = graphbit.Node.agent(
            name="Improvement Recommender",
            prompt="""Based on this quality assessment, recommend improvements:

Quality Assessment: {quality_results}

For content that scored below 7, provide:
1. **Priority Issues**: Most critical problems to address
2. **Quick Wins**: Easy improvements with high impact
3. **Long-term Improvements**: Comprehensive enhancement strategies
4. **Resources**: Suggest tools or resources for improvement

Focus on actionable, specific recommendations.
""",
            agent_id="improver"
        )
        
        # Connect pipeline
        assess_id = workflow.add_node(assessor)
        gate_id = workflow.add_node(quality_gate)
        improve_id = workflow.add_node(improver)
        
        workflow.connect(assess_id, gate_id)
        workflow.connect(gate_id, improve_id)
        
        workflow.validate()
        return workflow
    
    def create_recommendation_workflow(self) -> graphbit.Workflow:
        """Create intelligent recommendation workflow."""
        
        workflow = graphbit.Workflow("Recommendation Engine")
        
        # Context Analyzer
        context_analyzer = graphbit.Node.agent(
            name="Context Analyzer",
            prompt="""Analyze the context for generating recommendations:

Current Document: {current_document}
Similar Documents: {similar_documents}
User Preferences: {user_preferences}
Document Category: {category}

Analyze:
1. **Content Themes**: Common themes across documents
2. **User Patterns**: What the user seems interested in
3. **Content Gaps**: Missing information or topics
4. **Complementary Content**: What would complement this content
5. **Trending Topics**: Relevant trending or popular topics

Provide context analysis for recommendation generation.
""",
            agent_id="context_analyzer"
        )
        
        # Recommendation Generator
        recommender = graphbit.Node.agent(
            name="Recommendation Generator",
            prompt="""Generate intelligent recommendations based on this context:

Context Analysis: {context_analysis}

Generate recommendations for:
1. **Related Content**: Documents or topics to explore next
2. **Deep Dive**: Areas for more detailed investigation
3. **Broad Exploration**: Related but different topics
4. **Practical Applications**: How to apply this knowledge
5. **Learning Path**: Suggested sequence for learning more

Rank recommendations by relevance and provide reasoning.
""",
            agent_id="recommender"
        )
        
        # Connect pipeline
        context_id = workflow.add_node(context_analyzer)
        rec_id = workflow.add_node(recommender)
        
        workflow.connect(context_id, rec_id)
        
        workflow.validate()
        return workflow
    
    async def add_document(self, document: Document) -> str:
        """Add a document to the system with full analysis."""
        
        print(f"🔄 Processing document: {document.title}")
        
        # Store document
        self.documents.append(document)
        
        # Generate embedding
        embedding = self.embedding_client.embed(
            f"{document.title}\n\n{document.content}"
        )
        self.embeddings.append(embedding)
        
        # Run analysis workflow
        analysis_result = await self.analyze_document(document)
        self.analysis_results[document.id] = analysis_result
        
        print(f"Document processed: {document.id}")
        return document.id
    
    async def analyze_document(self, document: Document) -> AnalysisResult:
        """Analyze document using the analysis workflow."""
        
        # Execute analysis workflow
        workflow = self.workflows['analysis']
        executor = self.executors['analysis']
        
        try:
            result = await executor.run_async(workflow)
            
            if result.is_success():
                # Parse results (simplified - in practice you'd parse JSON)
                output = result.get_output()
                
                return AnalysisResult(
                    document_id=document.id,
                    summary=output[:200] + "...",  # Simplified
                    key_topics=["topic1", "topic2"],  # Would parse from output
                    sentiment="neutral",  # Would parse from output
                    quality_score=8.0,  # Would parse from output
                    recommendations=["rec1", "rec2"]  # Would parse from output
                )
            else:
                raise Exception(f"Analysis failed: {result.get_error()}")
                
        except Exception as e:
            print(f"Analysis failed for {document.id}: {e}")
            # Return default result
            return AnalysisResult(
                document_id=document.id,
                summary="Analysis failed",
                key_topics=[],
                sentiment="unknown",
                quality_score=0.0,
                recommendations=[]
            )
    
    def find_similar_documents(self, document_id: str, top_k: int = 5) -> List[Dict]:
        """Find similar documents using embeddings."""
        
        # Find document index
        doc_index = None
        for i, doc in enumerate(self.documents):
            if doc.id == document_id:
                doc_index = i
                break
        
        if doc_index is None:
            return []
        
        query_embedding = self.embeddings[doc_index]
        similarities = []
        
        for i, (doc, embedding) in enumerate(zip(self.documents, self.embeddings)):
            if i == doc_index:
                continue
                
            similarity = graphbit.EmbeddingClient.similarity(query_embedding, embedding)
            similarities.append({
                'document_id': doc.id,
                'title': doc.title,
                'similarity': similarity,
                'category': doc.category
            })
        
        # Sort by similarity
        similarities.sort(key=lambda x: x['similarity'], reverse=True)
        return similarities[:top_k]
    
    async def generate_recommendations(self, document_id: str, user_preferences: Dict = None) -> List[str]:
        """Generate intelligent recommendations for a document."""
        
        # Find similar documents
        similar_docs = self.find_similar_documents(document_id, top_k=3)
        
        # Get current document
        current_doc = None
        for doc in self.documents:
            if doc.id == document_id:
                current_doc = doc
                break
        
        if not current_doc:
            return []
        
        # Execute recommendation workflow
        workflow = self.workflows['recommendation']
        executor = self.executors['fast']
        
        try:
            result = await executor.run_async(workflow)
            
            if result.is_success():
                # Parse recommendations from output
                output = result.get_output()
                # In practice, you'd parse structured JSON output
                return ["Recommendation 1", "Recommendation 2", "Recommendation 3"]
            else:
                return []
                
        except Exception as e:
            print(f"Recommendation generation failed: {e}")
            return []
    
    async def batch_process_documents(self, documents: List[Document]) -> Dict[str, AnalysisResult]:
        """Process multiple documents in batch."""
        
        print(f"Batch processing {len(documents)} documents...")
        
        # Add all documents first
        for doc in documents:
            self.documents.append(doc)
            
            # Generate embeddings in batch
            texts = [f"{doc.title}\n\n{doc.content}" for doc in documents]
            embeddings = self.embedding_client.embed_many(texts)
            self.embeddings.extend(embeddings)
        
        # Process analysis in batches
        batch_results = {}
        batch_size = 5
        
        for i in range(0, len(documents), batch_size):
            batch = documents[i:i + batch_size]
            print(f"Processing batch {i//batch_size + 1}/{(len(documents) + batch_size - 1)//batch_size}")
            
            # Process batch concurrently
            tasks = [self.analyze_document(doc) for doc in batch]
            results = await asyncio.gather(*tasks, return_exceptions=True)
            
            for doc, result in zip(batch, results):
                if isinstance(result, Exception):
                    print(f"Failed to process {doc.id}: {result}")
                else:
                    batch_results[doc.id] = result
                    self.analysis_results[doc.id] = result
        
        print(f"Batch processing completed: {len(batch_results)}/{len(documents)} successful")
        return batch_results
    
    def get_system_statistics(self) -> Dict:
        """Get comprehensive system statistics."""
        
        stats = {
            'documents': {
                'total': len(self.documents),
                'categories': {},
                'average_length': 0
            },
            'analysis': {
                'completed': len(self.analysis_results),
                'average_quality': 0,
                'sentiment_distribution': {'positive': 0, 'negative': 0, 'neutral': 0}
            },
            'embeddings': {
                'total': len(self.embeddings),
                'dimension': len(self.embeddings[0]) if self.embeddings else 0
            },
            'system': graphbit.get_system_info(),
            'health': graphbit.health_check()
        }
        
        # Calculate document statistics
        if self.documents:
            category_counts = {}
            total_length = 0
            
            for doc in self.documents:
                category_counts[doc.category] = category_counts.get(doc.category, 0) + 1
                total_length += len(doc.content)
            
            stats['documents']['categories'] = category_counts
            stats['documents']['average_length'] = total_length / len(self.documents)
        
        # Calculate analysis statistics
        if self.analysis_results:
            quality_scores = [r.quality_score for r in self.analysis_results.values()]
            stats['analysis']['average_quality'] = sum(quality_scores) / len(quality_scores)
            
            sentiment_counts = {'positive': 0, 'negative': 0, 'neutral': 0}
            for result in self.analysis_results.values():
                sentiment_counts[result.sentiment] = sentiment_counts.get(result.sentiment, 0) + 1
            stats['analysis']['sentiment_distribution'] = sentiment_counts
        
        return stats
    
    def export_analysis_results(self, filepath: str):
        """Export analysis results to JSON file."""
        
        export_data = {
            'documents': [
                {
                    'id': doc.id,
                    'title': doc.title,
                    'category': doc.category,
                    'metadata': doc.metadata
                }
                for doc in self.documents
            ],
            'analysis_results': {
                doc_id: {
                    'summary': result.summary,
                    'key_topics': result.key_topics,
                    'sentiment': result.sentiment,
                    'quality_score': result.quality_score,
                    'recommendations': result.recommendations
                }
                for doc_id, result in self.analysis_results.items()
            },
            'statistics': self.get_system_statistics(),
            'export_timestamp': time.time()
        }
        
        with open(filepath, 'w') as f:
            json.dump(export_data, f, indent=2)
        
        print(f"Analysis results exported to {filepath}")

# Example usage and demonstration
async def main():
    """Demonstrate the comprehensive AI pipeline."""
    
    # Setup
    openai_key = os.getenv("OPENAI_API_KEY")
    anthropic_key = os.getenv("ANTHROPIC_API_KEY")
    
    if not openai_key:
        print("OPENAI_API_KEY required")
        return
    
    print("Initializing Comprehensive AI Pipeline")
    print("=" * 60)
    
    # Create pipeline
    pipeline = IntelligentDocumentPipeline(openai_key, anthropic_key)
    
    # Sample documents
    sample_documents = [
        Document(
            id="doc1",
            title="Machine Learning Fundamentals",
            content="""Machine learning is a subset of artificial intelligence that enables computers to learn and improve from experience without being explicitly programmed. It encompasses various algorithms and statistical models that allow systems to automatically learn patterns from data. The field includes supervised learning, where models learn from labeled training data, unsupervised learning, which finds hidden patterns in unlabeled data, and reinforcement learning, where agents learn through interaction with an environment. Key applications include image recognition, natural language processing, recommendation systems, and predictive analytics.""",
            category="technology",
            metadata={"author": "AI Research Team", "difficulty": "beginner", "tags": ["AI", "ML", "fundamentals"]}
        ),
        Document(
            id="doc2", 
            title="Sustainable Energy Solutions",
            content="""Renewable energy sources such as solar, wind, hydroelectric, and geothermal power are becoming increasingly crucial for addressing climate change and reducing dependence on fossil fuels. Solar technology has seen dramatic cost reductions, with photovoltaic panels becoming more efficient and affordable. Wind energy has also expanded rapidly, with offshore wind farms providing significant power generation capacity. Energy storage technologies, including advanced battery systems and pumped hydro storage, are essential for managing the intermittent nature of renewable sources. Smart grid technologies enable better integration and distribution of renewable energy across power networks.""",
            category="environment",
            metadata={"author": "Green Energy Council", "difficulty": "intermediate", "tags": ["renewable", "solar", "wind"]}
        ),
        Document(
            id="doc3",
            title="Digital Marketing Strategies", 
            content="""Modern digital marketing encompasses a wide range of strategies and channels designed to reach and engage target audiences online. Social media marketing leverages platforms like Facebook, Instagram, LinkedIn, and Twitter to build brand awareness and drive engagement. Search engine optimization (SEO) improves website visibility in search results, while pay-per-click (PPC) advertising provides immediate visibility. Content marketing focuses on creating valuable, relevant content to attract and retain customers. Email marketing remains highly effective for nurturing leads and maintaining customer relationships. Data analytics tools enable marketers to measure campaign performance and optimize strategies for better results.""",
            category="business",
            metadata={"author": "Marketing Institute", "difficulty": "intermediate", "tags": ["marketing", "digital", "SEO"]}
        )
    ]
    
    # Process documents individually
    print("\nProcessing Documents Individually")
    print("-" * 40)
    
    for doc in sample_documents[:2]:  # Process first 2 individually
        doc_id = await pipeline.add_document(doc)
        print(f"Processed: {doc.title} (ID: {doc_id})")
    
    # Batch process remaining documents
    print("\nBatch Processing Remaining Documents")
    print("-" * 40)
    
    remaining_docs = sample_documents[2:]
    if remaining_docs:
        batch_results = await pipeline.batch_process_documents(remaining_docs)
        print(f"Batch processing completed: {len(batch_results)} documents")
    
    # Find similar documents
    print("\nFinding Similar Documents")
    print("-" * 40)
    
    similar_docs = pipeline.find_similar_documents("doc1", top_k=2)
    print(f"Documents similar to '{pipeline.documents[0].title}':")
    for sim_doc in similar_docs:
        print(f"  - {sim_doc['title']} (similarity: {sim_doc['similarity']:.3f})")
    
    # Generate recommendations
    print("\nGenerating Recommendations")
    print("-" * 40)
    
    recommendations = await pipeline.generate_recommendations("doc1")
    print(f"Recommendations for '{pipeline.documents[0].title}':")
    for i, rec in enumerate(recommendations, 1):
        print(f"  {i}. {rec}")
    
    # Display system statistics
    print("\nSystem Statistics")
    print("-" * 40)
    
    stats = pipeline.get_system_statistics()
    
    print(f"Documents: {stats['documents']['total']}")
    print(f"Categories: {list(stats['documents']['categories'].keys())}")
    print(f"Analysis completed: {stats['analysis']['completed']}")
    print(f"Average quality score: {stats['analysis']['average_quality']:.2f}")
    print(f"Embeddings generated: {stats['embeddings']['total']}")
    print(f"System health: {'✅' if stats['health']['overall_healthy'] else '❌'}")
    
    # Export results
    print("\nExporting Results")
    print("-" * 40)
    
    pipeline.export_analysis_results("analysis_results.json")
    
    print("\nComprehensive AI Pipeline Demo Completed!")

if __name__ == "__main__":
    asyncio.run(main())
```

## Key System Features

### Comprehensive Integration
- **Multi-Workflow System**: Analysis, enhancement, quality assessment, recommendations
- **Multiple LLM Providers**: OpenAI, Anthropic support with fallbacks
- **Embedding Integration**: Semantic search and similarity analysis
- **Performance Optimization**: Different executors for different use cases

### Advanced Capabilities
- **Batch Processing**: Efficient handling of multiple documents
- **Async Operations**: Non-blocking operations for better performance
- **Quality Gates**: Conditional workflow execution based on quality scores
- **Intelligent Recommendations**: Context-aware recommendation generation

### Production Features
- **Error Handling**: Comprehensive error management and fallbacks
- **Monitoring**: System health checks and performance statistics
- **Export Capabilities**: JSON export of analysis results
- **Flexible Configuration**: Multiple execution modes and provider support

### Real-World Applications
- **Document Management**: Intelligent document analysis and organization
- **Content Platforms**: Automated content quality assessment
- **Knowledge Management**: Semantic search and recommendation systems
- **Research Tools**: Comprehensive analysis and insight generation

This comprehensive example demonstrates how GraphBit's various components work together to create sophisticated, production-ready AI applications that can handle complex workflows with reliability and performance. 