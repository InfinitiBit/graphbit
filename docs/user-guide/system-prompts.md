# System Prompts

System prompts are foundational instructions that define the behavior, personality, and capabilities of AI agents in GraphBit workflows. They provide context and constraints that guide how agents interpret and respond to user prompts, making them essential for creating reliable and consistent AI-powered applications.

---

## How System Prompts Work

In GraphBit, system prompts are processed at the LLM provider level and converted into the appropriate format for each provider:

1. **Storage**: System prompts are stored in the node configuration when creating agent nodes
2. **Retrieval**: During workflow execution, the system prompt is extracted from the node metadata
3. **Message Construction**: The system prompt is converted to a system message in the LLM request
4. **Provider Handling**: Each LLM provider handles system messages according to their API specifications

### Technical Flow

```
Node.agent(system_prompt="...") 
    ↓
Stored in node.config["system_prompt"]
    ↓
Retrieved during workflow execution
    ↓
Added as LlmMessage::system() to request
    ↓
Converted to provider-specific format
    ↓
Sent to LLM with user prompt
```

---

## Basic Usage

### Simple System Prompt

```python
from graphbit import Node, Workflow, LlmConfig, Executor
import os

# Create an agent with a basic system prompt
agent = Node.agent(
    name="Helpful Assistant",
    prompt="What is the capital of France?",
    system_prompt="You are a helpful assistant. Be concise and accurate."
)

# Create and execute workflow
workflow = Workflow("Basic System Prompt Example")
workflow.add_node(agent)

config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
executor = Executor(config)
result = executor.execute(workflow)

print(result.get_node_output("Helpful Assistant"))
# Output: "The capital of France is Paris."
```

### Without System Prompt (Optional Parameter)

```python
# System prompt is optional - agent will use default behavior
agent = Node.agent(
    name="Default Agent",
    prompt="Explain quantum computing in simple terms."
    # No system_prompt parameter
)
```

### Role-Based System Prompt

```python
agent = Node.agent(
    name="Code Reviewer",
    prompt=f"Review this code for bugs and improvements: {code_input}",
    system_prompt="""You are an experienced software engineer and code reviewer.
    
    When reviewing code:
    - Focus on bugs, security issues, and performance problems
    - Suggest specific improvements with examples
    - Be constructive and educational in your feedback
    - Rate the code quality from 1-10
    - Provide a summary of key issues found"""
)
```

---

## System Prompt Patterns

### 1. Role Definition Pattern

Define a clear role and expertise area for the agent:

```python
# Technical Writer Agent
technical_writer = Node.agent(
    name="Technical Writer",
    prompt=f"Write documentation for: {feature_description}",
    system_prompt="""You are a senior technical writer with expertise in software documentation.
    
    Your writing style:
    - Clear, concise, and user-focused
    - Uses examples and code snippets when helpful
    - Follows standard documentation patterns
    - Includes troubleshooting sections when relevant
    
    Always structure documentation with:
    1. Overview
    2. Prerequisites  
    3. Step-by-step instructions
    4. Examples
    5. Common issues and solutions"""
)

# Data Analyst Agent
data_analyst = Node.agent(
    name="Data Analyst",
    prompt=f"Analyze this dataset and provide insights: {data}",
    system_prompt="""You are a data analyst with expertise in statistical analysis and business intelligence.
    
    For every analysis:
    - Start with data quality assessment
    - Identify key patterns and trends
    - Provide statistical significance where applicable
    - Suggest actionable business recommendations
    - Highlight any limitations or caveats
    
    Present findings in a structured format with visualizations when possible."""
)
```

### 2. Output Format Pattern

Control the structure and format of responses:

```python
# JSON Response Agent
json_agent = Node.agent(
    name="Structured Analyzer",
    prompt=f"Analyze the sentiment of: {text}",
    system_prompt="""You are a sentiment analysis expert. Always respond in valid JSON format.

    Response structure:
    {
        "sentiment": "positive|negative|neutral",
        "confidence": 0.0-1.0,
        "key_phrases": ["phrase1", "phrase2"],
        "reasoning": "brief explanation"
    }
    
    Never include any text outside the JSON structure."""
)

# Markdown Report Agent  
report_agent = Node.agent(
    name="Report Generator",
    prompt=f"Create a report on: {topic}",
    system_prompt="""You are a business analyst who creates professional reports in Markdown format.
    
    Report structure:
    # Executive Summary
    ## Key Findings
    ## Methodology
    ## Detailed Analysis
    ## Recommendations
    ## Appendix
    
    Use tables, bullet points, and headers appropriately. Include relevant metrics and data."""
)
```

### 3. Constraint and Safety Pattern

Set boundaries and safety guidelines:

```python
# Safe Content Agent
safe_agent = Node.agent(
    name="Content Moderator",
    prompt=f"Review this content: {content}",
    system_prompt="""You are a content moderator focused on safety and appropriateness.
    
    Guidelines:
    - Never generate harmful, illegal, or inappropriate content
    - Flag potential issues with content safety
    - Suggest improvements for problematic content
    - Be objective and professional in assessments
    
    If content violates guidelines, explain why and suggest alternatives."""
)

# Domain-Specific Constraint Agent
financial_agent = Node.agent(
    name="Financial Advisor",
    prompt=f"Provide financial guidance on: {question}",
    system_prompt="""You are a financial education specialist. 
    
    Important constraints:
    - Provide educational information only, not personalized financial advice
    - Always recommend consulting with qualified financial professionals
    - Clearly state when information is general vs. specific
    - Include appropriate disclaimers about financial risks
    - Focus on widely accepted financial principles"""
)
```

### 4. Context and Memory Pattern

Provide context and maintain consistency across interactions:

```python
# Context-Aware Agent
context_agent = Node.agent(
    name="Customer Support",
    prompt=f"Help the customer with: {customer_query}",
    system_prompt="""You are a customer support specialist for a software company.

    Context about our product:
    - GraphBit: AI workflow automation framework
    - Supports Python, Node.js, and Rust
    - Key features: LLM integration, workflow builder, agent management

    Support guidelines:
    - Be empathetic and patient
    - Ask clarifying questions when needed
    - Provide step-by-step solutions
    - Escalate complex technical issues appropriately
    - Always end with asking if there's anything else you can help with"""
)

# Conversation Memory Agent
memory_agent = Node.agent(
    name="Conversational AI",
    prompt=f"Continue the conversation: {user_input}",
    system_prompt="""You are a conversational AI that maintains context throughout interactions.

    Conversation principles:
    - Remember previous topics and references
    - Build on earlier parts of the conversation
    - Maintain consistent personality and tone
    - Ask follow-up questions to deepen understanding
    - Acknowledge when you don't remember something from earlier"""
)
```

---

## API Reference

### Node.agent() Parameters

```python
Node.agent(
    name: str,                    # Required: Human-readable node name
    prompt: str,                  # Required: User prompt template
    agent_id: Optional[str],      # Optional: Unique agent identifier
    output_name: Optional[str],   # Optional: Custom output name
    tools: Optional[List],        # Optional: Available tools
    system_prompt: Optional[str]  # Optional: System prompt for behavior control
)
```

### System Prompt Parameter Details

- **Type**: `Optional[str]`
- **Default**: `None` (no system prompt)
- **Storage**: Stored in `node.config["system_prompt"]`
- **Processing**: Converted to `LlmMessage::system()` during execution
- **Provider Support**: All supported providers handle system messages appropriately

---

By following the patterns outlined in this guide, you can create more reliable, predictable, and effective AI workflows that meet your specific requirements.

For more information, see:
- [Agent Configuration Guide](agents.md)
- [Workflow Builder](workflow-builder.md)
- [LLM Providers](llm-providers.md)
