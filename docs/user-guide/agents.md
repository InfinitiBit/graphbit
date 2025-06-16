# Agent Configuration

Agents are the core AI-powered components in GraphBit workflows. This guide covers how to configure, customize, and optimize agents for different use cases.

## Agent Basics

### Creating Basic Agents

```python
import graphbit

# Simple agent
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Data Analyzer",
    description="Analyzes input data for patterns",
    agent_id="analyzer",
    prompt="Analyze the following data and identify key patterns: {input}"
)
```

### Agent with Configuration

```python
# Agent with custom configuration
writer = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Creative Writer",
    description="Writes creative content",
    agent_id="creative_writer",
    prompt="Write a creative story about: {topic}",
    max_tokens=1500,
    temperature=0.8
)
```

## Agent Properties

### Core Properties

Every agent has these essential properties:

- **Name**: Human-readable identifier
- **Description**: Purpose and functionality
- **Agent ID**: Unique identifier for the agent
- **Prompt**: Template for LLM interaction

### Configuration Parameters

When using `agent_node_with_config()`:

- **max_tokens**: Maximum tokens to generate (default: 1000)
- **temperature**: Creativity level 0.0-1.0 (default: 0.7)

```python
# Conservative agent (factual, deterministic)
fact_checker = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Fact Checker",
    description="Verifies factual accuracy",
    agent_id="fact_checker",
    prompt="Verify the accuracy of: {content}",
    max_tokens=800,
    temperature=0.1  # Low temperature for consistency
)

# Creative agent (imaginative, varied)
storyteller = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Storyteller",
    description="Creates engaging stories",
    agent_id="storyteller",
    prompt="Tell an engaging story about: {theme}",
    max_tokens=2000,
    temperature=0.9  # High temperature for creativity
)
```

## Agent Capabilities

GraphBit supports different agent capability types:

### Text Processing

```python
text_capability = graphbit.PyAgentCapability.text_processing()

# Agent specialized in text processing
text_processor = graphbit.PyWorkflowNode.agent_node(
    name="Text Processor",
    description="Processes and transforms text",
    agent_id="text_processor",
    prompt="Process this text: {input}"
)
```

### Data Analysis

```python
data_capability = graphbit.PyAgentCapability.data_analysis()

# Agent for data analysis tasks
data_analyst = graphbit.PyWorkflowNode.agent_node(
    name="Data Analyst",
    description="Analyzes data patterns",
    agent_id="data_analyst",
    prompt="Analyze this dataset: {data}"
)
```

### Decision Making

```python
decision_capability = graphbit.PyAgentCapability.decision_making()

# Agent for making decisions
decision_maker = graphbit.PyWorkflowNode.agent_node(
    name="Decision Maker",
    description="Makes strategic decisions",
    agent_id="decision_maker",
    prompt="Make a decision based on: {criteria}"
)
```

### Tool Execution

```python
tool_capability = graphbit.PyAgentCapability.tool_execution()

# Agent for executing tools
tool_executor = graphbit.PyWorkflowNode.agent_node(
    name="Tool Executor",
    description="Executes various tools",
    agent_id="tool_executor",
    prompt="Execute tool for: {task}"
)
```

### Custom Capabilities

```python
# Create custom capability
domain_expert = graphbit.PyAgentCapability.custom("medical_expert")

# Agent with custom capability
medical_advisor = graphbit.PyWorkflowNode.agent_node(
    name="Medical Advisor",
    description="Provides medical insights",
    agent_id="medical_advisor",
    prompt="Provide medical analysis of: {symptoms}"
)
```

## Prompt Engineering

### Basic Prompt Structure

```python
prompt = """
Task: {task_description}
Context: {context}
Input: {input_data}

Please provide a detailed analysis focusing on:
1. Key findings
2. Recommendations
3. Next steps

Format your response as structured text.
"""

agent = graphbit.PyWorkflowNode.agent_node(
    name="Structured Analyzer",
    description="Provides structured analysis",
    agent_id="structured_analyzer",
    prompt=prompt
)
```

### Variable Substitution

Prompts support variable substitution using `{variable_name}`:

```python
# Multi-variable prompt
analysis_prompt = """
Analyze the {data_type} data about {subject}.

Context: {background_info}
Requirements:
- Focus on {focus_area}
- Provide {num_insights} key insights
- Use {tone} tone
- Format as {output_format}

Data: {input}
"""

analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Flexible Analyzer",
    description="Analyzes various data types",
    agent_id="flex_analyzer",
    prompt=analysis_prompt
)
```

### Prompt Best Practices

#### 1. Be Specific and Clear

```python
# Good - specific and detailed
good_prompt = """
Review this Python code for security vulnerabilities.

Focus on:
1. SQL injection risks
2. XSS vulnerabilities  
3. Authentication issues
4. Input validation problems

Code: {code}

Provide specific line numbers and remediation steps.
"""

# Avoid - vague and unclear
bad_prompt = "Look at this code: {code}"
```

#### 2. Provide Examples

```python
prompt_with_examples = """
Classify the sentiment of customer feedback.

Examples:
- "Great product, love it!" → Positive
- "Terrible experience, waste of money" → Negative  
- "It's okay, nothing special" → Neutral

Customer feedback: {feedback}
Classification: 
"""
```

#### 3. Define Output Format

```python
structured_output_prompt = """
Analyze the business proposal and provide feedback.

Proposal: {proposal}

Respond in JSON format:
{
    "overall_score": 1-10,
    "strengths": ["strength1", "strength2"],
    "weaknesses": ["weakness1", "weakness2"],
    "recommendation": "approve/reject/revise",
    "comments": "detailed feedback"
}
"""
```

## Specialized Agent Types

### Content Agents

```python
# Content writer
writer = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Content Writer",
    description="Creates marketing content",
    agent_id="content_writer",
    prompt="""
    Write engaging {content_type} about {topic}.
    
    Target audience: {audience}
    Tone: {tone}
    Length: {length} words
    
    Include a compelling headline and call-to-action.
    """,
    max_tokens=1500,
    temperature=0.7
)

# Content editor
editor = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Content Editor",
    description="Edits and improves content",
    agent_id="content_editor",
    prompt="""
    Edit this content for clarity and engagement:
    
    {content}
    
    Improve:
    - Readability and flow
    - Grammar and style
    - Engagement level
    - Structure and organization
    """,
    max_tokens=1200,
    temperature=0.3
)
```

### Analysis Agents

```python
# Data analyzer
data_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Data Analyzer",
    description="Analyzes numerical data",
    agent_id="data_analyzer",
    prompt="""
    Analyze this dataset and provide insights:
    
    {dataset}
    
    Include:
    1. Statistical summary
    2. Key trends and patterns
    3. Anomalies or outliers
    4. Recommendations
    """
)

# Sentiment analyzer
sentiment_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Sentiment Analyzer", 
    description="Analyzes text sentiment",
    agent_id="sentiment_analyzer",
    prompt="""
    Analyze the sentiment of this text:
    
    "{text}"
    
    Provide:
    - Overall sentiment (positive/negative/neutral)
    - Confidence score (0-1)
    - Key emotional indicators
    - Reasoning for classification
    """
)
```

### Domain Expert Agents

```python
# Technical expert
tech_expert = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Technical Expert",
    description="Provides technical insights",
    agent_id="tech_expert",
    prompt="""
    As a technical expert, analyze this {technology} implementation:
    
    {technical_details}
    
    Evaluate:
    - Architecture quality
    - Performance implications
    - Security considerations
    - Best practices compliance
    - Improvement recommendations
    """,
    max_tokens=2000,
    temperature=0.2
)

# Business expert
business_expert = graphbit.PyWorkflowNode.agent_node(
    name="Business Expert",
    description="Provides business strategy insights",
    agent_id="business_expert",
    prompt="""
    From a business perspective, evaluate this proposal:
    
    {business_proposal}
    
    Consider:
    - Market opportunity
    - Financial viability
    - Risk assessment
    - Competitive advantage
    - Implementation timeline
    """
)
```

## Agent Orchestration

### Sequential Agent Chain

```python
def create_analysis_chain():
    builder = graphbit.PyWorkflowBuilder("Analysis Chain")
    
    # Data collector
    collector = graphbit.PyWorkflowNode.agent_node(
        "Data Collector", "Collects relevant data", "collector",
        "Collect data about: {topic}"
    )
    
    # Data analyzer
    analyzer = graphbit.PyWorkflowNode.agent_node(
        "Data Analyzer", "Analyzes collected data", "analyzer", 
        "Analyze this data: {collected_data}"
    )
    
    # Report generator
    reporter = graphbit.PyWorkflowNode.agent_node(
        "Report Generator", "Creates final report", "reporter",
        "Create a comprehensive report based on: {analysis_results}"
    )
    
    # Chain agents
    c_id = builder.add_node(collector)
    a_id = builder.add_node(analyzer)
    r_id = builder.add_node(reporter)
    
    builder.connect(c_id, a_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(a_id, r_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Collaborative Agents

```python
def create_collaborative_workflow():
    builder = graphbit.PyWorkflowBuilder("Collaborative Review")
    
    # Multiple expert agents
    expert1 = graphbit.PyWorkflowNode.agent_node(
        "Technical Expert", "Technical review", "tech_expert",
        "Technical review: {proposal}"
    )
    
    expert2 = graphbit.PyWorkflowNode.agent_node(
        "Business Expert", "Business review", "biz_expert", 
        "Business review: {proposal}"
    )
    
    expert3 = graphbit.PyWorkflowNode.agent_node(
        "Legal Expert", "Legal review", "legal_expert",
        "Legal review: {proposal}"
    )
    
    # Synthesizer agent
    synthesizer = graphbit.PyWorkflowNode.agent_node(
        "Synthesizer", "Combines expert opinions", "synthesizer",
        """
        Synthesize these expert reviews:
        Technical: {tech_review}
        Business: {business_review}  
        Legal: {legal_review}
        
        Provide consolidated recommendation.
        """
    )
    
    # Build collaborative structure
    e1_id = builder.add_node(expert1)
    e2_id = builder.add_node(expert2)
    e3_id = builder.add_node(expert3)
    s_id = builder.add_node(synthesizer)
    
    # All experts feed into synthesizer
    builder.connect(e1_id, s_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(e2_id, s_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(e3_id, s_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Agent Performance Optimization

### Configuration for Different Use Cases

```python
# High-speed processing (minimal tokens, low temperature)
fast_agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Fast Processor",
    description="Quick processing agent",
    agent_id="fast_processor",
    prompt="Quick summary: {input}",
    max_tokens=200,
    temperature=0.1
)

# High-quality output (more tokens, moderate temperature)
quality_agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Quality Processor",
    description="High-quality processing agent", 
    agent_id="quality_processor",
    prompt="Detailed analysis: {input}",
    max_tokens=1500,
    temperature=0.4
)

# Creative output (high tokens, high temperature)
creative_agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Creative Processor",
    description="Creative processing agent",
    agent_id="creative_processor", 
    prompt="Creative interpretation: {input}",
    max_tokens=2000,
    temperature=0.8
)
```

### Batch Processing with Agents

```python
# Create multiple instances for batch processing
def create_batch_agents(num_agents=3):
    agents = []
    for i in range(num_agents):
        agent = graphbit.PyWorkflowNode.agent_node(
            name=f"Batch Agent {i+1}",
            description=f"Batch processing agent {i+1}",
            agent_id=f"batch_agent_{i+1}",
            prompt="Process batch item: {item}"
        )
        agents.append(agent)
    return agents
```

## Error Handling for Agents

### Robust Agent Configuration

```python
# Agent with error handling context
robust_agent = graphbit.PyWorkflowNode.agent_node(
    name="Robust Agent",
    description="Agent with error handling",
    agent_id="robust_agent",
    prompt="""
    Process this input safely: {input}
    
    If the input is invalid or unclear:
    1. Explain what's wrong
    2. Suggest corrections
    3. Provide a safe default response
    
    Always provide a response, even for edge cases.
    """
)
```

### Validation Agents

```python
# Input validator agent
validator = graphbit.PyWorkflowNode.agent_node(
    name="Input Validator",
    description="Validates input data",
    agent_id="validator",
    prompt="""
    Validate this input: {input}
    
    Check for:
    - Data completeness
    - Format correctness
    - Reasonable values
    - Security issues
    
    Respond with: VALID or INVALID with explanation
    """
)
```

## Agent Best Practices

### 1. Single Responsibility
Each agent should have one clear purpose:

```python
# Good - focused responsibility
email_classifier = graphbit.PyWorkflowNode.agent_node(
    name="Email Classifier",
    description="Classifies emails by category",
    agent_id="email_classifier", 
    prompt="Classify this email as: spam/important/newsletter/personal\n\nEmail: {email}"
)

# Avoid - multiple responsibilities
everything_agent = graphbit.PyWorkflowNode.agent_node(
    name="Everything Agent",
    description="Does everything",
    agent_id="everything",
    prompt="Do whatever is needed with: {input}"
)
```

### 2. Clear Interfaces
Define clear input/output expectations:

```python
# Well-defined interface
code_reviewer = graphbit.PyWorkflowNode.agent_node(
    name="Code Reviewer",
    description="Reviews code for quality and security",
    agent_id="code_reviewer",
    prompt="""
    Review this code for quality and security:
    
    {code}
    
    Provide structured feedback:
    RATING: 1-10
    ISSUES: List of problems found
    SUGGESTIONS: Recommended improvements
    SECURITY: Security concerns if any
    """
)
```

### 3. Context Awareness
Include relevant context in prompts:

```python
context_aware_agent = graphbit.PyWorkflowNode.agent_node(
    name="Context-Aware Agent",
    description="Makes decisions with full context",
    agent_id="context_agent",
    prompt="""
    Context: You are a {role} working on {project_type}.
    Current phase: {project_phase}
    Constraints: {constraints}
    
    Task: {task}
    
    Consider the context when providing your response.
    """
)
```

Agents are the powerhouse of GraphBit workflows. By configuring them properly and following best practices, you can create sophisticated AI-powered processing pipelines that are both reliable and effective. 