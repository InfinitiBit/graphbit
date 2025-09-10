# Node Types Reference

GraphBit workflows are built using different types of nodes, each serving a specific purpose. This reference covers all available node types and their usage patterns.

## Node Type Categories

1. **Agent Nodes** - AI-powered processing nodes

## Agent Nodes

Agent nodes are the core AI-powered components that interact with LLM providers.

### Basic Agent Node

```python
from graphbit import Node

agent = Node.agent(
    name="Content Analyzer",
    prompt=f"Analyze the following content: {input}",
    agent_id="analyzer"  # Optional, auto-generated if not provided
)
```

**Parameters:**
- `name` (str): Human-readable node name
- `prompt` (str): LLM prompt template with variable placeholders
- `agent_id` (str, optional): Unique identifier for the agent. Auto-generated if not provided
- `llm_config` (obj, optional): Custom LLM configuration for the node
- `output_name` (str, optional): Custom name for the node's output
- `tools` (List, optional): List of tools available to the agent
- `system_prompt` (str, optional): System prompt that defines agent behavior and constraints

### Agent Node with Tool calling

```python
from graphbit import Node

agent = Node.agent(
    name="Weather Agent",
    prompt=f"Using the tool, get the weather forecast for: {input}",
    agent_id="weather_agent",  # Optional, auto-generated if not provided
    tools=[get_weather]         
)
```

### Agent Node with System Prompt

```python
from graphbit import Node

# Agent with system prompt for behavior control
analyzer = Node.agent(
    name="Code Reviewer",
    prompt=f"Review this code for issues: {code}",
    agent_id="code_reviewer",
    system_prompt="""You are an experienced software engineer and code reviewer.

    Focus on:
    - Security vulnerabilities
    - Performance issues
    - Code quality and best practices
    - Potential bugs

    Provide specific, actionable feedback with examples."""
)

# Agent with structured output format
json_agent = Node.agent(
    name="Sentiment Analyzer",
    prompt=f"Analyze sentiment: {text}",
    system_prompt="""Respond only in valid JSON format:
    {
        "sentiment": "positive|negative|neutral",
        "confidence": 0.0-1.0,
        "reasoning": "brief explanation"
    }"""
)
```

### Agent Node with llm_config

```python
from graphbit import LlmConfig, Node

# Configure LLM providers
openai_config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
anthropic_config = LlmConfig.anthropic(os.getenv("ANTHROPIC_API_KEY"), "claude-sonnet-4-20250514")

# Agent with system prompt for behavior control
analyzer = Node.agent(
    name="Code Reviewer",
    prompt=f"Review this code for issues: {code}",
    agent_id="code_reviewer",
    llm_config=anthropic_config  # Use Anthropic
)

# Agent with structured output format
json_agent = Node.agent(
    name="Sentiment Analyzer",
    prompt=f"Analyze sentiment: {text}",
    llm_config=openai_config  # Use Openai
)
```

### Agent Node Examples

#### Text Analysis Agent
```python
sentiment_analyzer = Node.agent(
    name="Sentiment Analyzer",
    prompt=f"""
    Analyze the sentiment of this text: "{text}"
    
    Provide:
    - Overall sentiment (positive/negative/neutral)
    - Confidence score (0-1)
    - Key emotional indicators
    """,
    agent_id="sentiment_analyzer"
)
```

#### Code Review Agent
```python
code_reviewer = Node.agent(
    name="Code Reviewer",
    prompt=f"""
    Review this code for quality and security issues:
    
    {code}
    
    Check for:
    - Security vulnerabilities
    - Performance issues
    - Code style problems
    - Best practices violations
    """,
    agent_id="code_reviewer"
)
```

#### Data Processing Agent
```python
data_processor = Node.agent(
    name="Data Processor",
    prompt=f"""
    Process this dataset and provide insights:
    
    Data: {dataset}
    
    Include:
    1. Statistical summary
    2. Key trends
    3. Anomalies
    4. Recommendations
    """,
    agent_id="data_processor"
)
```

#### Content Generation Agent
```python
content_writer = Node.agent(
    name="Content Writer",
    prompt=f"""
    Write engaging content about: {topic}
    
    Requirements:
    - Target audience: {audience}
    - Tone: {tone}
    - Length: {word_count} words
    - Include call-to-action
    """,
    agent_id="content_writer"
)
```

#### Research Assistant Agent
```python
research_assistant = Node.agent(
    name="Research Assistant",
    prompt=f"""
    Research the following topic: {research_topic}
    
    Provide:
    - Key findings (3-5 points)
    - Supporting evidence
    - Potential implications
    - Areas for further investigation
    
    Focus on: {focus_area}
    """,
    agent_id="research_assistant"
)
```

## Node Connection Patterns

### Sequential Connections
Connect nodes for sequential processing:

```python
from graphbit import Workflow

workflow = Workflow("Sequential Pipeline")

# Add nodes
node1_id = workflow.add_node(input_processor)
node2_id = workflow.add_node(analyzer)
node3_id = workflow.add_node(output_formatter)

# Connect sequentially
workflow.connect(node1_id, node2_id)
workflow.connect(node2_id, node3_id)
```

### Parallel Processing
Process multiple branches simultaneously:

```python
workflow = Workflow("Parallel Processing")

# Add input and processors
input_id = workflow.add_node(input_processor)
processor1_id = workflow.add_node(sentiment_analyzer)
processor2_id = workflow.add_node(topic_extractor)
processor3_id = workflow.add_node(summary_generator)
aggregator_id = workflow.add_node(result_aggregator)

# Fan-out to parallel processors
workflow.connect(input_id, processor1_id)
workflow.connect(input_id, processor2_id)
workflow.connect(input_id, processor3_id)

# Fan-in to aggregator
workflow.connect(processor1_id, aggregator_id)
workflow.connect(processor2_id, aggregator_id)
workflow.connect(processor3_id, aggregator_id)
```

## Advanced Node Patterns

### Error Handling Pattern
```python
from graphbit import Node, Workflow

def create_error_handling_workflow():
    workflow = Workflow("Error Handling")
    
    # Main processor
    main_processor = Node.agent(
        name="Main Processor",
        prompt=f"Process: {input}",
        agent_id="main"
    )
    
    # Error handler
    error_handler = Node.agent(
        name="Error Handler",
        prompt=f"Handle this error: {error_message}",
        agent_id="error_handler"
    )
    
    # Success handler
    success_handler = Node.agent(
        name="Success Handler",
        prompt=f"Finalize successful result: {result}",
        agent_id="success_handler"
    )
    
    # Build error handling flow
    main_id = workflow.add_node(main_processor)
    error_id = workflow.add_node(error_handler)
    success_id = workflow.add_node(success_handler)
    
    workflow.connect(main_id, error_id)     # Error path
    workflow.connect(error_id, success_id) # Success path
    
    return workflow
```

### Multi-Step Analysis Pipeline
```python
from graphbit import Node, Workflow

def create_analysis_pipeline():
    workflow = Workflow("Multi-Step Analysis")
    
    # Step 1: Initial analysis
    initial_analyzer = Node.agent(
        name="Initial Analyzer",
        prompt=f"Perform initial analysis of: {input}",
        agent_id="initial_analyzer"
    )
    
    # Step 2: Deep analysis (if quality is good)
    deep_analyzer = Node.agent(
        name="Deep Analyzer",
        prompt=f"Perform deep analysis of: {analyzed_content}",
        agent_id="deep_analyzer"
    )
    
    # Connect the pipeline
    initial_id = workflow.add_node(initial_analyzer)
    deep_id = workflow.add_node(deep_analyzer)
    
    workflow.connect(initial_id, deep_id)
    
    return workflow
```

## Node Properties and Methods

All nodes share common properties:

```python
# Access node properties
node_id = node.id()     # Unique identifier
node_name = node.name() # Human-readable name
```

## Best Practices

### 1. Descriptive Names
Use clear, descriptive names for all nodes:

```python
# Good
email_sentiment_analyzer = Node.agent(
    name="Email Sentiment Analyzer",
    prompt=f"Analyze sentiment of customer emails: {email_content}",
    agent_id="email_sentiment"
)

# Avoid
node1 = Node.agent(
    name="Node1", 
    prompt=f"Do: {input}",
    agent_id="n1"
)
```

### 2. Single Responsibility
Each node should have one clear purpose:

```python
# Good - focused on one task
spam_detector = Node.agent(
    name="Spam Detector",
    prompt=f"Is this email spam? {email}",
    agent_id="spam_detector"
)

# Avoid - too many responsibilities  
everything_processor = Node.agent(
    name="Everything Processor",
    prompt=f"Do everything with: {input}",
    agent_id="everything"
)
```

### 3. Node Types

- **Agent Nodes**: AI/LLM processing tasks

### 4. Error Handling
Include appropriate error handling and validation:

```python

# Error recovery
error_handler = Node.agent(
    name="Error Handler",
    prompt=f"Safely handle this error: {error}",
    agent_id="error_handler"
)
```

### 5. Clear Prompt Design
Write clear, specific prompts for agent nodes:

```python
# Good - specific and clear
summarizer = Node.agent(
    name="Document Summarizer",
    prompt=f"""
    Summarize this document in exactly 3 paragraphs:
    
    Document: {document_content}
    
    Requirements:
    - Paragraph 1: Main topic and purpose
    - Paragraph 2: Key findings or arguments
    - Paragraph 3: Conclusions and implications
    """,
    agent_id="summarizer"
)

# Avoid - vague and unclear
bad_summarizer = Node.agent(
    name="Summarizer",
    prompt=f"Summarize: {input}",
    agent_id="summarizer"
)
```

Understanding these node types and their usage patterns enables you to build sophisticated, reliable workflows that handle complex AI processing tasks effectively. Choose the appropriate node type for each step in your workflow, and connect them in logical patterns to achieve your processing goals.
