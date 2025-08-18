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
from graphbit import init, LlmConfig, Executor, Workflow, Node, version, get_system_info, health_check
import os
from typing import Optional

class ContentGenerationPipeline:
    def __init__(self, api_key: str, model: str = "gpt-4o-mini"):
        """Initialize the content generation pipeline."""
        # Initialize GraphBit
        init(log_level="info", enable_tracing=True)
        
        # Create LLM configuration
        self.llm_config = LlmConfig.openai(api_key, model)
        
        # Create executor with custom settings
        self.executor = Executor(
            self.llm_config,
            timeout_seconds=300,  # 5 minutes
            debug=True
        )
    
    def create_workflow(self, content_type: str = "article") -> Workflow:
        """Create the content generation workflow."""
        # Create workflow
        workflow = Workflow("Content Generation Pipeline")
        
        # Stage 1: Research Agent
        researcher = Node.agent(
            name="Research Specialist",
            prompt="""Research the topic: {topic}

Please provide:
1. Key facts and statistics
2. Current trends and developments
3. Expert opinions or quotes
4. Relevant examples or case studies
5. Important considerations or nuances

Focus on accuracy and credibility. Cite sources where possible.
Format as structured research notes.
""",
            agent_id="researcher"
        )
        
        # Stage 2: Content Writer
        writer = Node.agent(
            name="Content Writer",
            prompt="""Write a comprehensive {content_type} about: {topic}

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
            agent_id="writer"
        )
        
        # Stage 3: Editor
        editor = Node.agent(
            name="Content Editor",
            prompt="""Edit and improve the following {content_type}:

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
            agent_id="editor"
        )
        
        # Stage 4: Quality Reviewer
        reviewer = Node.agent(
            name="Quality Reviewer",
            prompt="""Review this {content_type} for quality and accuracy:

{edited_content}

Provide feedback on:
1. Factual accuracy
2. Completeness of coverage
3. Logical flow
4. Audience appropriateness
5. Areas for improvement

Rate overall quality (1-10) and provide specific suggestions.
If quality is 7 or above, mark as APPROVED, otherwise mark as NEEDS_REVISION.
""",
            agent_id="reviewer"
        )
        
        # Stage 5: Final Formatter
        formatter = Node.agent(
            name="Content Formatter",
            prompt="""Format this content for publication:

{approved_content}

Apply:
- Professional formatting
- Proper headings hierarchy
- Bullet points where appropriate
- Clear paragraph breaks
- Call-to-action if needed

Output clean, publication-ready content.
""",
            agent_id="formatter"
        )
        
        # Add nodes to workflow
        research_id = workflow.add_node(researcher)
        writer_id = workflow.add_node(writer)
        editor_id = workflow.add_node(editor)
        reviewer_id = workflow.add_node(reviewer)
        formatter_id = workflow.add_node(formatter)
        
        # Connect the workflow: Research → Write → Edit → Review → Format
        workflow.connect(research_id, writer_id)
        workflow.connect(writer_id, editor_id)
        workflow.connect(editor_id, reviewer_id)
        workflow.connect(reviewer_id, formatter_id)
        
        # Validate workflow
        workflow.validate()
        
        return workflow
    
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
        
        # Execute workflow with input context
        result = self.executor.execute(workflow)
        
        if result.is_success():
            execution_time = result.execution_time_ms()
            print(f"Content generation completed in {execution_time}ms")
            
            return {
                "status": "success",
                "content": result.get_all_node_outputs(),
                "execution_time_ms": execution_time,
                "workflow_stats": self.executor.get_stats()
            }
        else:
            error_msg = result.get_error()
            print(f"Content generation failed: {error_msg}")
            
            return {
                "status": "error",
                "error": error_msg,
                "workflow_stats": self.executor.get_stats()
            }

# Example usage
def main():
    """Run the content generation pipeline."""
    
    # Set up API key (you can also set OPENAI_API_KEY environment variable)
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("❌ Please set OPENAI_API_KEY environment variable")
        return
    
    # Create pipeline
    pipeline = ContentGenerationPipeline(api_key, "gpt-4o-mini")
    
    # Generate content
    result = pipeline.generate_content(
        topic="Sustainable Energy Solutions",
        content_type="blog post",
        target_length=1200,
        tone="informative yet engaging",
        audience="technology enthusiasts"
    )
    
    # Display results
    if result["status"] == "success":
        print("\nGenerated Content:")
        print("=" * 60)
        print(result["content"])
        print("\nPerformance Stats:")
        print(f"Execution time: {result['execution_time_ms']}ms")
    else:
        print(f"\nGeneration failed: {result['error']}")

if __name__ == "__main__":
    main()
```

## Alternative: Using Different LLM Providers

### Using Anthropic Claude

```python
from graphbit import init, LlmConfig, Executor, Workflow, Node
import os

def create_anthropic_pipeline():
    """Create pipeline using Anthropic Claude."""
    init()
    print("ANTHOPIC_API_KEY",os.getenv("ANTHROPIC_API_KEY"))
    
    # Configure for Anthropic
    config = LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-sonnet-4-20250514"
    )
    
    executor = Executor(config, debug=True)
    
    # Create simple workflow
    workflow = Workflow("Anthropic Content Generator")
    
    writer = Node.agent(
        name="Claude Writer",
        prompt="""Write a comprehensive article about: {topic}

Requirements:
- Length: {word_count} words
- Tone: {tone}
- Include practical examples
- Structure with clear headings

Create engaging, well-researched content.
""",
        agent_id="claude_writer"
    )
    
    workflow.add_node(writer)
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_anthropic_pipeline()
result = executor.execute(workflow)
```

### Using Local Ollama Models

```python
from graphbit import init, LlmConfig, Executor, Workflow, Node

def create_ollama_pipeline():
    """Create pipeline using local Ollama models."""
    init()
    
    # Configure for Ollama (no API key needed)
    config = LlmConfig.ollama("llama3.2")
    
    executor = Executor(
        config,
        timeout_seconds=180,  # Longer timeout for local inference
        debug=True
    )
    
    workflow = Workflow("Local Content Generator")
    
    writer = Node.agent(
        name="Llama Writer",
        prompt="""Write about: {topic}

Keep it concise but informative.
Focus on practical insights.
""",
        agent_id="llama_writer"
    )
    
    workflow.add_node(writer)
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_ollama_pipeline()
result = executor.execute(workflow)
```

## Advanced Features

### High-Performance Content Generation

```python
from graphbit import init, LlmConfig, Executor
import os

def create_high_performance_pipeline():
    """Create optimized pipeline for high-throughput content generation."""
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"  # Faster model for high throughput
    )
    
    # Use high-throughput executor
    executor = Executor.new_high_throughput(
        config,
        timeout_seconds=60,  # Shorter timeout
        debug=False  # Disable debug for performance
    )
    
    return executor

def create_low_latency_pipeline():
    """Create pipeline optimized for low latency."""
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Use low-latency executor
    executor = Executor.new_low_latency(
        config,
        timeout_seconds=30,  # Very short timeout
        debug=False
    )
    
    return executor

def create_memory_optimized_pipeline():
    """Create pipeline optimized for memory usage."""
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Use memory-optimized executor
    executor = Executor.new_memory_optimized(
        config,
        timeout_seconds=120,
        debug=False
    )
    
    return executor
```

### Async Content Generation

```python
from graphbit import init, LlmConfig, Executor, Workflow, Node
import asyncio
import os

async def generate_content_async():
    """Generate content asynchronously."""
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = Executor(config)
    
    # Create workflow
    workflow = Workflow("Async Content Generator")
    
    writer = Node.agent(
        name="Async Writer",
        prompt="Write a brief article about: {topic}",
        agent_id="async_writer"
    )
    
    workflow.add_node(writer)
    workflow.validate()
    
    # Execute asynchronously
    result = await executor.run_async(workflow)
    
    if result.is_success():
        print("Async generation completed")
        return result.get_all_node_outputs()
    else:
        print(f"Async generation failed: {result.get_error()}")
        return None

# Usage
async def main_async():
    content = await generate_content_async()
    if content:
        print(f"Generated: {content}")

# Run async
asyncio.run(main_async())
```

## System Information and Health Checks

```python
from graphbit import init, get_system_info, health_check, version

def check_system_health():
    """Check GraphBit system health and capabilities."""
    init()
    
    # Get system information
    system_info = get_system_info()
    print("System Information:")
    for key, value in system_info.items():
        print(f"  {key}: {value}")
    
    # Perform health check
    health_status = health_check()
    print(f"\nHealth Status:")
    for key, value in health_status.items():
        print(f"  {key}: {value}")
    
    # Check version
    version_info = version()
    print(f"\nGraphBit Version: {version_info}")

# Usage
check_system_health()
```

## Key Features

### Content Pipeline Components
- **Research Agent**: Gathers comprehensive information on topics
- **Content Writer**: Creates initial drafts based on research
- **Editor**: Improves clarity, flow, and engagement
- **Quality Reviewer**: Ensures accuracy and completeness
- **Formatter**: Prepares content for publication

### Reliability Features
- **Multiple LLM Providers**: OpenAI, Anthropic, Ollama support
- **Execution Modes**: High-throughput, low-latency, memory-optimized
- **Error Handling**: Comprehensive error reporting and recovery
- **Performance Monitoring**: Built-in execution statistics
- **Health Checks**: System health and capability monitoring

This example demonstrates GraphBit's capabilities for building production-ready content generation workflows with reliability, performance, and flexibility.
