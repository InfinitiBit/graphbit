# Content Generation Pipeline

This example demonstrates how to build a sophisticated content generation workflow with GraphBit, featuring research, writing, editing, and quality assurance stages.

## Overview

We'll create a multi-agent pipeline that:
1. **Researches** a given topic
2. **Writes** initial content based on research
3. **Edits** for clarity and engagement
4. **Reviews** for quality and accuracy
5. **Formats** the final output

## Complete Example

```python
import graphbit
import os
from typing import Optional

class ContentGenerationPipeline:
    def __init__(self, api_key: str, model: str = "gpt-4o-mini"):
        """Initialize the content generation pipeline."""
        graphbit.init()
        self.config = graphbit.PyLlmConfig.openai(api_key, model)
        
        # Create executor with reliability features
        self.executor = graphbit.PyWorkflowExecutor(self.config) \
            .with_retry_config(graphbit.PyRetryConfig.default()) \
            .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig.default())
    
    def create_workflow(self, content_type: str = "article") -> graphbit.PyWorkflow:
        """Create the content generation workflow."""
        builder = graphbit.PyWorkflowBuilder("Content Generation Pipeline")
        builder.description(f"Complete {content_type} generation with research, writing, and editing")
        
        # Stage 1: Research Agent
        researcher = graphbit.PyWorkflowNode.agent_node(
            name="Research Specialist",
            description="Conducts comprehensive research on the topic",
            agent_id="researcher",
            prompt="""
            Research the topic: {topic}
            
            Please provide:
            1. Key facts and statistics
            2. Current trends and developments
            3. Expert opinions or quotes
            4. Relevant examples or case studies
            5. Important considerations or nuances
            
            Focus on accuracy and credibility. Cite sources where possible.
            Format as structured research notes.
            """
        )
        
        # Stage 2: Content Writer
        writer = graphbit.PyWorkflowNode.agent_node_with_config(
            name="Content Writer",
            description="Creates engaging content based on research",
            agent_id="writer",
            prompt="""
            Write a comprehensive {content_type} about: {topic}
            
            Based on this research: {research_data}
            
            Requirements:
            - Target length: {target_length} words
            - Tone: {tone}
            - Audience: {audience}
            - Include relevant examples and data from research
            - Use engaging headlines and subheadings
            - Ensure logical flow and structure
            
            Create compelling, informative content that captures reader attention.
            """,
            max_tokens=2000,
            temperature=0.7
        )
        
        # Stage 3: Editor
        editor = graphbit.PyWorkflowNode.agent_node_with_config(
            name="Content Editor",
            description="Edits and improves content for clarity and engagement",
            agent_id="editor",
            prompt="""
            Edit and improve the following {content_type}:
            
            {draft_content}
            
            Focus on:
            - Clarity and readability
            - Flow and structure
            - Engaging language
            - Grammar and style
            - Consistency in tone
            - Compelling headlines
            
            Maintain the core message while making it more engaging and polished.
            """,
            max_tokens=2000,
            temperature=0.3
        )
        
        # Stage 4: Quality Reviewer
        reviewer = graphbit.PyWorkflowNode.agent_node(
            name="Quality Reviewer",
            description="Reviews content for accuracy and completeness",
            agent_id="reviewer",
            prompt="""
            Review this {content_type} for quality and accuracy:
            
            {edited_content}
            
            Provide feedback on:
            1. Factual accuracy
            2. Completeness of coverage
            3. Logical flow
            4. Audience appropriateness
            5. Areas for improvement
            
            Rate overall quality (1-10) and provide specific suggestions.
            """
        )
        
        # Stage 5: Quality Gate (Condition Node)
        quality_gate = graphbit.PyWorkflowNode.condition_node(
            name="Quality Gate",
            description="Checks if content meets quality standards",
            expression="quality_rating >= 7"
        )
        
        # Stage 6: Final Formatter
        formatter = graphbit.PyWorkflowNode.agent_node(
            name="Content Formatter",
            description="Formats content for final publication",
            agent_id="formatter",
            prompt="""
            Format this content for publication:
            
            {final_content}
            
            Apply:
            - Professional formatting
            - Proper headings hierarchy
            - Bullet points where appropriate
            - Clear paragraph breaks
            - Call-to-action if needed
            
            Output clean, publication-ready content.
            """
        )
        
        # Build the workflow graph
        research_id = builder.add_node(researcher)
        writer_id = builder.add_node(writer)
        editor_id = builder.add_node(editor)
        reviewer_id = builder.add_node(reviewer)
        quality_id = builder.add_node(quality_gate)
        formatter_id = builder.add_node(formatter)
        
        # Connect the workflow: Research → Write → Edit → Review → Quality Check → Format
        builder.connect(research_id, writer_id, graphbit.PyWorkflowEdge.data_flow())
        builder.connect(writer_id, editor_id, graphbit.PyWorkflowEdge.data_flow())
        builder.connect(editor_id, reviewer_id, graphbit.PyWorkflowEdge.data_flow())
        builder.connect(reviewer_id, quality_id, graphbit.PyWorkflowEdge.data_flow())
        builder.connect(quality_id, formatter_id, graphbit.PyWorkflowEdge.conditional("quality_rating >= 7"))
        
        return builder.build()
    
    def generate_content(
        self,
        topic: str,
        content_type: str = "article",
        target_length: int = 800,
        tone: str = "professional",
        audience: str = "general"
    ) -> dict:
        """Generate content using the pipeline."""
        
        print(f"🚀 Starting content generation for: {topic}")
        
        # Create workflow
        workflow = self.create_workflow(content_type)
        
        # Set input variables
        input_vars = {
            "topic": topic,
            "content_type": content_type,
            "target_length": target_length,
            "tone": tone,
            "audience": audience
        }
        
        try:
            # Execute workflow
            result = self.executor.execute(workflow)
            
            if result.is_completed():
                print(f"✅ Content generation completed in {result.execution_time_ms()}ms")
                
                return {
                    "success": True,
                    "content": result.get_variable("output"),
                    "research": result.get_variable("research_data"),
                    "quality_score": result.get_variable("quality_rating"),
                    "execution_time": result.execution_time_ms()
                }
            else:
                print(f"❌ Content generation failed")
                return {
                    "success": False,
                    "error": "Workflow execution failed",
                    "execution_time": result.execution_time_ms()
                }
                
        except Exception as e:
            print(f"❌ Error during content generation: {e}")
            return {"success": False, "error": str(e)}

# Usage Example
def main():
    # Initialize pipeline
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("❌ Please set OPENAI_API_KEY environment variable")
        return
    
    pipeline = ContentGenerationPipeline(api_key)
    
    # Generate different types of content
    topics = [
        {
            "topic": "The Future of Artificial Intelligence in Healthcare",
            "content_type": "article",
            "target_length": 1200,
            "tone": "informative",
            "audience": "healthcare professionals"
        },
        {
            "topic": "10 Tips for Remote Work Productivity",
            "content_type": "blog post",
            "target_length": 800,
            "tone": "friendly",
            "audience": "remote workers"
        },
        {
            "topic": "Sustainable Energy Solutions for Small Businesses",
            "content_type": "guide",
            "target_length": 1500,
            "tone": "professional",
            "audience": "business owners"
        }
    ]
    
    for content_request in topics:
        print(f"\n{'='*50}")
        print(f"Generating: {content_request['topic']}")
        print(f"{'='*50}")
        
        result = pipeline.generate_content(**content_request)
        
        if result["success"]:
            print(f"\n📄 Generated Content:")
            print("-" * 30)
            print(result["content"][:500] + "..." if len(result["content"]) > 500 else result["content"])
            print(f"\n📊 Quality Score: {result.get('quality_score', 'N/A')}")
            print(f"⏱️  Execution Time: {result['execution_time']}ms")
        else:
            print(f"\n❌ Failed: {result['error']}")

if __name__ == "__main__":
    main()
```

## Advanced Features

### Parallel Content Generation

Generate multiple pieces of content simultaneously:

```python
def batch_generate_content(pipeline, topics_list):
    """Generate multiple pieces of content in parallel."""
    
    workflows = []
    for topic_config in topics_list:
        workflow = pipeline.create_workflow(topic_config["content_type"])
        workflows.append(workflow)
    
    # Execute all workflows concurrently
    results = pipeline.executor.execute_concurrent(workflows)
    
    return [
        {
            "topic": topics_list[i]["topic"],
            "result": result,
            "success": result.is_completed()
        }
        for i, result in enumerate(results)
    ]

# Usage
topics_batch = [
    {"topic": "AI in Education", "content_type": "article"},
    {"topic": "Climate Change Solutions", "content_type": "blog post"},
    {"topic": "Cryptocurrency Basics", "content_type": "guide"}
]

batch_results = batch_generate_content(pipeline, topics_batch)
```

### Content Templates

Create reusable content templates:

```python
class ContentTemplates:
    """Predefined content generation templates."""
    
    @staticmethod
    def blog_post_template():
        return {
            "content_type": "blog post",
            "target_length": 800,
            "tone": "conversational",
            "audience": "general readers"
        }
    
    @staticmethod
    def technical_article_template():
        return {
            "content_type": "technical article",
            "target_length": 1500,
            "tone": "professional",
            "audience": "developers"
        }
    
    @staticmethod
    def marketing_copy_template():
        return {
            "content_type": "marketing copy",
            "target_length": 400,
            "tone": "persuasive",
            "audience": "potential customers"
        }

# Usage with templates
template = ContentTemplates.blog_post_template()
result = pipeline.generate_content("Machine Learning Trends", **template)
```

### Custom Quality Metrics

Implement custom quality assessment:

```python
def create_advanced_quality_workflow():
    """Create workflow with advanced quality metrics."""
    
    # Advanced Quality Reviewer
    advanced_reviewer = graphbit.PyWorkflowNode.agent_node(
        name="Advanced Quality Reviewer",
        description="Comprehensive quality assessment",
        agent_id="advanced_reviewer",
        prompt="""
        Evaluate this content comprehensively:
        
        {content}
        
        Assess and rate (1-10) each criteria:
        1. Accuracy of information
        2. Clarity and readability
        3. Engagement level
        4. Structure and flow
        5. Audience appropriateness
        6. Originality
        7. Completeness
        8. Grammar and style
        
        Provide overall score and detailed feedback.
        Format response as JSON with scores and comments.
        """
    )
    
    return advanced_reviewer
```

### SEO Optimization Stage

Add SEO optimization to your content pipeline:

```python
def add_seo_optimization(builder):
    """Add SEO optimization stage to workflow."""
    
    seo_optimizer = graphbit.PyWorkflowNode.agent_node(
        name="SEO Optimizer",
        description="Optimizes content for search engines",
        agent_id="seo_optimizer",
        prompt="""
        Optimize this content for SEO:
        
        {content}
        Target keyword: {target_keyword}
        
        Apply SEO best practices:
        1. Include target keyword naturally
        2. Optimize headings (H1, H2, H3)
        3. Add meta description suggestion
        4. Improve content structure
        5. Suggest internal linking opportunities
        6. Ensure keyword density (1-2%)
        
        Maintain readability while improving SEO.
        """
    )
    
    return seo_optimizer
```

## Error Handling and Recovery

Implement robust error handling:

```python
class RobustContentPipeline(ContentGenerationPipeline):
    """Enhanced pipeline with error recovery."""
    
    def __init__(self, api_key: str, model: str = "gpt-4o-mini"):
        super().__init__(api_key, model)
        
        # Enhanced retry configuration
        self.executor = self.executor.with_retry_config(
            graphbit.PyRetryConfig.default()
            .with_exponential_backoff(2000, 2.0, 60000)  # 2s, 4s, 8s, 16s, 32s, 60s max
            .with_jitter(0.2)  # Add 20% jitter
        )
    
    def generate_with_fallback(self, topic: str, **kwargs):
        """Generate content with fallback options."""
        
        # Try with GPT-4 first
        try:
            return self.generate_content(topic, **kwargs)
        except Exception as e:
            print(f"⚠️  GPT-4 failed, trying GPT-3.5-turbo: {e}")
            
            # Fallback to GPT-3.5-turbo
            self.config = graphbit.PyLlmConfig.openai(
                os.getenv("OPENAI_API_KEY"), 
                "gpt-3.5-turbo"
            )
            self.executor = graphbit.PyWorkflowExecutor(self.config)
            
            try:
                return self.generate_content(topic, **kwargs)
            except Exception as e2:
                print(f"❌ Both models failed: {e2}")
                return {"success": False, "error": f"All models failed: {e2}"}
```

## Performance Monitoring

Track and optimize performance:

```python
import time
from typing import List, Dict

class PerformanceMonitor:
    """Monitor content generation performance."""
    
    def __init__(self):
        self.metrics = []
    
    def track_generation(self, topic: str, result: dict):
        """Track generation metrics."""
        self.metrics.append({
            "topic": topic,
            "success": result["success"],
            "execution_time": result.get("execution_time", 0),
            "quality_score": result.get("quality_score"),
            "timestamp": time.time()
        })
    
    def get_stats(self) -> Dict:
        """Get performance statistics."""
        if not self.metrics:
            return {}
        
        successful = [m for m in self.metrics if m["success"]]
        
        return {
            "total_generations": len(self.metrics),
            "success_rate": len(successful) / len(self.metrics),
            "avg_execution_time": sum(m["execution_time"] for m in successful) / len(successful) if successful else 0,
            "avg_quality_score": sum(m.get("quality_score", 0) for m in successful) / len(successful) if successful else 0
        }

# Usage
monitor = PerformanceMonitor()
pipeline = ContentGenerationPipeline(os.getenv("OPENAI_API_KEY"))

for topic in ["AI Ethics", "Remote Work", "Climate Change"]:
    result = pipeline.generate_content(topic)
    monitor.track_generation(topic, result)

print("Performance Stats:", monitor.get_stats())
```

## Best Practices

### 1. Prompt Engineering
- Use specific, detailed prompts
- Include examples and context
- Define clear output formats
- Test and iterate on prompts

### 2. Quality Control
- Implement multi-stage review
- Use quality gates and conditions
- Set minimum quality thresholds
- Include human review for critical content

### 3. Performance Optimization
- Use appropriate model sizes
- Implement caching for repeated requests
- Batch similar requests
- Monitor and optimize execution times

### 4. Error Handling
- Implement retry logic
- Use circuit breakers
- Provide fallback models
- Log errors for analysis

## Customization Options

The content generation pipeline can be customized for different use cases:

- **Blog Posts**: Conversational tone, engaging hooks
- **Technical Documentation**: Precise language, step-by-step instructions
- **Marketing Copy**: Persuasive language, clear CTAs
- **Academic Papers**: Formal tone, citations, methodology
- **Social Media**: Concise format, engaging visuals

Modify the prompts, add specialized agents, or adjust the workflow structure to match your specific content generation needs.

This example demonstrates how GraphBit's flexible architecture enables building sophisticated, production-ready content generation systems with reliability, quality control, and performance optimization built-in. 