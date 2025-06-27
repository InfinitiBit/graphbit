# Agents

Agents are AI-powered components that execute tasks within GraphBit workflows. This guide covers how to create, configure, and optimize agents for different use cases.

## Overview

In GraphBit, agents are implemented as specialized workflow nodes that:
- Execute AI tasks using configured LLM providers
- Process inputs through prompt templates with variable substitution
- Generate outputs that flow to connected nodes
- Support different execution contexts and requirements

## Creating Agents

### Basic Agent Creation

```python
import graphbit

# Initialize GraphBit
graphbit.init()

# Create a basic agent node
analyzer = graphbit.Node.agent(
    name="Data Analyzer",
    prompt="Analyze the following data and identify key patterns: {input}",
    agent_id="analyzer"  # Optional - auto-generated if not provided
)

# Access agent properties
print(f"Agent ID: {analyzer.id()}")
print(f"Agent Name: {analyzer.name()}")
```

### Agent with Explicit Configuration

```python
# Agent with explicit ID for referencing
content_creator = graphbit.Node.agent(
    name="Content Creator",
    prompt="Create engaging content about: {topic}",
    agent_id="content_creator_v1"
)

# Agent for specific domain
technical_writer = graphbit.Node.agent(
    name="Technical Documentation Writer",
    prompt="""
    Write comprehensive technical documentation for: {feature}
    
    Include:
    - Overview and purpose
    - Implementation details
    - Usage examples
    - Best practices
    
    Feature details: {input}
    """,
    agent_id="tech_doc_writer"
)
```

## Agent Configuration in Workflows

### Single Agent Workflow

```python
# Create workflow with single agent
workflow = graphbit.Workflow("Content Analysis")

# Create and add agent
analyzer = graphbit.Node.agent(
    name="Content Analyzer",
    prompt="Analyze this content for sentiment, key themes, and quality: {input}",
    agent_id="content_analyzer"
)

analyzer_id = workflow.add_node(analyzer)
workflow.validate()

# Execute with LLM configuration
llm_config = graphbit.LlmConfig.openai(
    api_key="your-openai-key",
    model="gpt-4o-mini"
)

executor = graphbit.Executor(llm_config, timeout_seconds=60)
result = executor.execute(workflow)
```

### Multi-Agent Workflow

```python
# Create workflow with multiple specialized agents
workflow = graphbit.Workflow("Multi-Agent Analysis Pipeline")

# Create specialized agents
sentiment_agent = graphbit.Node.agent(
    name="Sentiment Analyzer",
    prompt="Analyze the sentiment of this text (positive/negative/neutral): {input}",
    agent_id="sentiment_analyzer"
)

topic_agent = graphbit.Node.agent(
    name="Topic Extractor", 
    prompt="Extract the main topics and themes from: {input}",
    agent_id="topic_extractor"
)

summary_agent = graphbit.Node.agent(
    name="Content Summarizer",
    prompt="Create a concise summary of: {input}",
    agent_id="summarizer"
)

# Aggregation agent
aggregator = graphbit.Node.agent(
    name="Analysis Aggregator",
    prompt="""
    Combine the following analysis results into a comprehensive report:
    
    Sentiment Analysis: {sentiment_output}
    Topic Analysis: {topic_output}  
    Summary: {summary_output}
    
    Provide an integrated analysis with key insights.
    """,
    agent_id="aggregator"
)

# Build workflow
sentiment_id = workflow.add_node(sentiment_agent)
topic_id = workflow.add_node(topic_agent)
summary_id = workflow.add_node(summary_agent)
agg_id = workflow.add_node(aggregator)

# Connect nodes for parallel processing then aggregation
workflow.connect(sentiment_id, agg_id)
workflow.connect(topic_id, agg_id)
workflow.connect(summary_id, agg_id)

workflow.validate()
```

## Prompt Engineering

### Basic Prompt Structure

Design effective prompts for your agents:

```python
# Simple, direct prompt
simple_agent = graphbit.Node.agent(
    name="Simple Translator",
    prompt="Translate this text to French: {input}",
    agent_id="translator"
)

# Structured prompt with clear instructions
structured_agent = graphbit.Node.agent(
    name="Structured Analyzer",
    prompt="""
    Task: Analyze the provided text for business insights
    
    Text to analyze: {input}
    
    Please provide:
    1. Key business themes identified
    2. Market opportunities mentioned
    3. Risk factors highlighted  
    4. Recommended actions
    
    Format your response as a structured analysis.
    """,
    agent_id="business_analyzer"
)
```

### Variable Substitution

Use variables in prompts for dynamic content:

```python
# Multi-variable prompt
flexible_prompt = """
Context: You are a {role} expert analyzing {content_type} content.

Task: {task_description}

Content to analyze: {input}

Analysis requirements:
- Focus on {focus_area}
- Provide {detail_level} analysis
- Use {tone} tone
- Consider {constraints}

Please provide your analysis following these requirements.
"""

flexible_agent = graphbit.Node.agent(
    name="Flexible Content Analyzer",
    prompt=flexible_prompt,
    agent_id="flexible_analyzer"
)
```

### Domain-Specific Prompts

Create agents for specific domains:

```python
# Financial analysis agent
financial_agent = graphbit.Node.agent(
    name="Financial Analyst",
    prompt="""
    As a financial expert, analyze the following financial data:
    
    {input}
    
    Provide analysis covering:
    - Revenue trends and patterns
    - Cost structure analysis
    - Profitability insights
    - Risk assessment
    - Strategic recommendations
    
    Use standard financial analysis frameworks in your assessment.
    """,
    agent_id="financial_analyst"
)

# Marketing content agent
marketing_agent = graphbit.Node.agent(
    name="Marketing Content Creator",
    prompt="""
    Create compelling marketing content for: {product}
    
    Target audience: {audience}
    Key features: {features}
    Brand tone: {brand_tone}
    
    Create:
    1. Attention-grabbing headline
    2. Benefit-focused description
    3. Clear call-to-action
    4. Key selling points
    
    Content: {input}
    """,
    agent_id="marketing_creator"
)

# Technical documentation agent
technical_agent = graphbit.Node.agent(
    name="Technical Documentation Writer",
    prompt="""
    Write clear, comprehensive technical documentation for developers.
    
    Topic: {input}
    
    Include:
    - Clear overview and purpose
    - Step-by-step implementation guide
    - Code examples with explanations
    - Common pitfalls and solutions
    - Best practices and recommendations
    
    Use clear, professional technical writing style.
    """,
    agent_id="tech_writer"
)
```

## Agent Specialization Patterns

### Sequential Processing Agents

Create agents that build on each other's work:

```python
workflow = graphbit.Workflow("Sequential Content Processing")

# Stage 1: Content preparation
prep_agent = graphbit.Node.agent(
    name="Content Preparation Agent",
    prompt="Clean and structure this raw content for further processing: {input}",
    agent_id="content_prep"
)

# Stage 2: Content analysis
analysis_agent = graphbit.Node.agent(
    name="Content Analysis Agent", 
    prompt="Analyze the prepared content for key insights: {prepared_content}",
    agent_id="content_analysis"
)

# Stage 3: Content enhancement
enhancement_agent = graphbit.Node.agent(
    name="Content Enhancement Agent",
    prompt="Enhance the analyzed content with additional details: {analyzed_content}",
    agent_id="content_enhancement"
)

# Connect sequentially
prep_id = workflow.add_node(prep_agent)
analysis_id = workflow.add_node(analysis_agent)
enhance_id = workflow.add_node(enhancement_agent)

workflow.connect(prep_id, analysis_id)
workflow.connect(analysis_id, enhance_id)
```

### Parallel Specialist Agents

Create specialized agents that work in parallel:

```python
workflow = graphbit.Workflow("Parallel Content Analysis")

# Input preparation
input_agent = graphbit.Node.agent(
    name="Input Processor",
    prompt="Prepare content for specialized analysis: {input}",
    agent_id="input_processor"
)

# Parallel specialists
seo_agent = graphbit.Node.agent(
    name="SEO Specialist",
    prompt="Analyze SEO aspects of: {processed_content}",
    agent_id="seo_specialist"
)

readability_agent = graphbit.Node.agent(
    name="Readability Specialist",
    prompt="Analyze readability and clarity of: {processed_content}",
    agent_id="readability_specialist"
)

compliance_agent = graphbit.Node.agent(
    name="Compliance Specialist", 
    prompt="Check compliance and accuracy of: {processed_content}",
    agent_id="compliance_specialist"
)

# Results integrator
integrator = graphbit.Node.agent(
    name="Results Integrator",
    prompt="""
    Integrate specialized analysis results:
    
    SEO Analysis: {seo_output}
    Readability Analysis: {readability_output}
    Compliance Analysis: {compliance_output}
    
    Provide comprehensive recommendations.
    """,
    agent_id="results_integrator"
)

# Build parallel structure
input_id = workflow.add_node(input_agent)
seo_id = workflow.add_node(seo_agent)
read_id = workflow.add_node(readability_agent)
comp_id = workflow.add_node(compliance_agent)
int_id = workflow.add_node(integrator)

# Connect input to all specialists
workflow.connect(input_id, seo_id)
workflow.connect(input_id, read_id)
workflow.connect(input_id, comp_id)

# Connect specialists to integrator
workflow.connect(seo_id, int_id)
workflow.connect(read_id, int_id)
workflow.connect(comp_id, int_id)
```

## Agent Configuration with Different LLM Providers

### Provider-Optimized Agents

Configure agents for different LLM providers:

```python
def create_provider_optimized_agents():
    """Create agents optimized for different providers"""
    
    # OpenAI-optimized agent (structured prompts work well)
    openai_agent = graphbit.Node.agent(
        name="OpenAI Structured Analyzer",
        prompt="""
        Task: Comprehensive content analysis
        
        Content: {input}
        
        Analysis Framework:
        1. Content Structure Analysis
        2. Quality Assessment
        3. Improvement Recommendations
        4. Risk Evaluation
        
        Provide detailed analysis for each framework component.
        """,
        agent_id="openai_analyzer"
    )
    
    # Anthropic-optimized agent (conversational style)
    anthropic_agent = graphbit.Node.agent(
        name="Claude Conversational Analyzer",
        prompt="""
        I'd like you to analyze this content from multiple perspectives.
        
        Content: {input}
        
        Please help me understand:
        - What are the main themes and messages?
        - How effective is the communication style?
        - What improvements would you suggest?
        - Are there any potential issues or concerns?
        
        Please be thorough in your analysis and explain your reasoning.
        """,
        agent_id="claude_analyzer"
    )
    
    # Ollama-optimized agent (concise prompts for local models)
    ollama_agent = graphbit.Node.agent(
        name="Local Model Analyzer",
        prompt="Analyze this content briefly: {input}",
        agent_id="local_analyzer"
    )
    
    return {
        "openai": openai_agent,
        "anthropic": anthropic_agent,
        "ollama": ollama_agent
    }
```

### Execution with Different Providers

```python
def execute_with_different_providers(agents, workflow_factory):
    """Execute same workflow with different providers"""
    
    # OpenAI execution
    openai_config = graphbit.LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    openai_executor = graphbit.Executor(openai_config, timeout_seconds=60)
    
    # Anthropic execution
    anthropic_config = graphbit.LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-3-5-sonnet-20241022"
    )
    anthropic_executor = graphbit.Executor(anthropic_config, timeout_seconds=120)
    
    # Ollama execution
    ollama_config = graphbit.LlmConfig.ollama(model="llama3.2")
    ollama_executor = graphbit.Executor(ollama_config, timeout_seconds=180)
    
    return {
        "openai": openai_executor,
        "anthropic": anthropic_executor,
        "ollama": ollama_executor
    }
```

## Error Handling and Resilience

### Robust Agent Design

Design agents that handle edge cases:

```python
# Agent with error handling instructions
robust_agent = graphbit.Node.agent(
    name="Robust Content Processor",
    prompt="""
    Process the following content. If the content is unclear, incomplete, 
    or problematic, please:
    
    1. Identify specific issues
    2. Provide what analysis is possible
    3. Suggest what additional information would be helpful
    4. Indicate confidence level in your analysis
    
    Content: {input}
    
    If you cannot process the content, explain why and suggest alternatives.
    """,
    agent_id="robust_processor"
)

# Agent with fallback behavior
fallback_agent = graphbit.Node.agent(
    name="Fallback Handler",
    prompt="""
    This content may have been processed unsuccessfully by a previous agent.
    
    Previous result: {previous_output}
    Original content: {original_input}
    
    Please provide a basic analysis of the original content, noting any
    issues or limitations in your analysis.
    """,
    agent_id="fallback_handler"
)
```

## Advanced Agent Patterns

### Agent with Context Memory

Create agents that maintain context across processing steps:

```python
# Context-aware agent
context_agent = graphbit.Node.agent(
    name="Context Aware Processor",
    prompt="""
    Previous context: {context_history}
    Current input: {input}
    Processing step: {step_number}
    
    Process the current input while maintaining awareness of the previous context.
    Update the context for the next processing step.
    
    Provide:
    1. Analysis of current input
    2. Relationship to previous context
    3. Updated context summary for next steps
    """,
    agent_id="context_processor"
)
```

### Quality Control Agents

Create agents that validate and improve outputs:

```python
# Quality validator agent
validator_agent = graphbit.Node.agent(
    name="Quality Validator",
    prompt="""
    Review the following content for quality:
    
    Content: {input}
    
    Evaluate:
    - Accuracy and factual correctness
    - Clarity and readability
    - Completeness of information
    - Logical flow and structure
    
    Provide quality score (1-10) and specific improvement suggestions.
    """,
    agent_id="quality_validator"
)

# Content improver agent
improver_agent = graphbit.Node.agent(
    name="Content Improver",
    prompt="""
    Improve the following content based on quality feedback:
    
    Original content: {original_content}
    Quality feedback: {quality_feedback}
    
    Provide improved version addressing the specific feedback points.
    Maintain the core message while enhancing quality.
    """,
    agent_id="content_improver"
)
```

## Best Practices

### 1. Agent Naming and Organization

```python
# Good: Descriptive, clear names
email_spam_detector = graphbit.Node.agent(
    name="Email Spam Detection Agent",
    prompt="Analyze this email for spam indicators: {email_content}",
    agent_id="email_spam_detector_v1"
)

# Good: Consistent naming convention
financial_risk_analyzer = graphbit.Node.agent(
    name="Financial Risk Analysis Agent",
    prompt="Assess financial risks in: {financial_data}",
    agent_id="financial_risk_analyzer_v1"
)

# Avoid: Vague names
agent1 = graphbit.Node.agent(
    name="Agent 1",
    prompt="Do something with: {input}",
    agent_id="a1"
)
```

### 2. Prompt Design Guidelines

```python
# Good: Clear, specific prompts
content_analyzer = graphbit.Node.agent(
    name="Marketing Content Analyzer",
    prompt="""
    Analyze this marketing content for effectiveness:
    
    Content: {input}
    
    Evaluate:
    1. Target audience alignment
    2. Message clarity and impact
    3. Call-to-action effectiveness
    4. Brand consistency
    5. Competitive differentiation
    
    Provide specific recommendations for improvement.
    """,
    agent_id="marketing_content_analyzer"
)

# Avoid: Vague, unclear prompts
vague_agent = graphbit.Node.agent(
    name="Content Thing",
    prompt="Look at this: {input}",
    agent_id="vague"
)
```

### 3. Agent Composition

```python
def create_modular_agents():
    """Create modular, reusable agents"""
    
    agents = {}
    
    # Base analysis agent
    agents['base_analyzer'] = graphbit.Node.agent(
        name="Base Content Analyzer",
        prompt="Provide basic analysis of: {input}",
        agent_id="base_analyzer"
    )
    
    # Specialized enhancement agents
    agents['seo_enhancer'] = graphbit.Node.agent(
        name="SEO Enhancement Agent",
        prompt="Enhance SEO aspects of: {analyzed_content}",
        agent_id="seo_enhancer"
    )
    
    agents['readability_enhancer'] = graphbit.Node.agent(
        name="Readability Enhancement Agent",
        prompt="Improve readability of: {analyzed_content}",
        agent_id="readability_enhancer"
    )
    
    return agents

# Usage: Compose agents into workflows as needed
def create_seo_workflow(agents):
    workflow = graphbit.Workflow("SEO Content Pipeline")
    
    base_id = workflow.add_node(agents['base_analyzer'])
    seo_id = workflow.add_node(agents['seo_enhancer'])
    
    workflow.connect(base_id, seo_id)
    return workflow
```

### 4. Testing and Validation

```python
def test_agent_configuration():
    """Test agent configuration before production use"""
    
    # Create test agent
    test_agent = graphbit.Node.agent(
        name="Test Agent",
        prompt="Test prompt with {input}",
        agent_id="test_agent"
    )
    
    # Validate agent properties
    assert test_agent.name() == "Test Agent"
    assert test_agent.id() is not None
    
    # Test workflow integration
    workflow = graphbit.Workflow("Test Workflow")
    node_id = workflow.add_node(test_agent)
    
    try:
        workflow.validate()
        print("✅ Agent configuration is valid")
        return True
    except Exception as e:
        print(f"❌ Agent configuration failed: {e}")
        return False
```

## What's Next

- Learn about [Workflow Builder](workflow-builder.md) for complex agent orchestration
- Explore [LLM Providers](llm-providers.md) for provider-specific optimizations
- Check [Performance](performance.md) for agent execution optimization
- See [Validation](validation.md) for agent output validation strategies
