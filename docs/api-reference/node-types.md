# Node Types Reference

GraphBit workflows are built using different types of nodes, each serving a specific purpose. This reference covers all available node types and their usage patterns.

## Node Type Categories

1. **Agent Nodes** - AI-powered processing nodes
2. **Condition Nodes** - Decision and branching logic
3. **Transform Nodes** - Data transformation and processing
4. **Delay Nodes** - Timing and rate limiting
5. **Document Loader Nodes** - Document processing

## Agent Nodes

Agent nodes are the core AI-powered components that interact with LLM providers.

### Basic Agent Node

```python
agent = graphbit.Node.agent(
    name="Content Analyzer",
    prompt="Analyze the following content: {input}",
    agent_id="analyzer"  # Optional, auto-generated if not provided
)
```

**Parameters:**
- `name` (str): Human-readable node name
- `prompt` (str): LLM prompt template with variable placeholders
- `agent_id` (str, optional): Unique identifier for the agent. Auto-generated if not provided

### Advanced Agent Node

```python
agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Creative Writer", 
    description="Writes creative content",
    agent_id="writer",
    prompt="Write a story about: {topic}",
    max_tokens=2000,
    temperature=0.8
)
```

**Additional Parameters:**
- `max_tokens` (int): Maximum tokens to generate
- `temperature` (float): Creativity/randomness level (0.0-1.0)

### Agent Node Examples

#### Text Analysis Agent
```python
sentiment_analyzer = graphbit.Node.agent(
    name="Sentiment Analyzer",
    prompt="""
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
code_reviewer = graphbit.Node.agent(
    name="Code Reviewer",
    prompt="""
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
data_processor = graphbit.Node.agent(
    name="Data Processor",
    prompt="""
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
content_writer = graphbit.Node.agent(
    name="Content Writer",
    prompt="""
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
research_assistant = graphbit.Node.agent(
    name="Research Assistant",
    prompt="""
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

## Condition Nodes

Condition nodes enable branching logic and decision-making in workflows.

### Basic Condition Node

```python
condition = graphbit.Node.condition(
    name="Quality Gate",
    expression="quality_score > 0.8"
)
```

**Parameters:**
- `name` (str): Node name
- `expression` (str): Boolean expression to evaluate

### Condition Expressions

Condition nodes support various comparison operators:

#### Numeric Comparisons
```python
# Greater than
high_score = graphbit.Node.condition(
    name="High Score Check", 
    expression="score > 80"
)

# Range checks
valid_range = graphbit.Node.condition(
    name="Range Validator", 
    expression="value >= 10 && value <= 100"
)

# Multiple conditions
complex_check = graphbit.Node.condition(
    name="Complex Check", 
    expression="(score > 75 && confidence > 0.8) || priority == 'high'"
)
```

#### String Comparisons
```python
# Equality
status_check = graphbit.Node.condition(
    name="Status Check", 
    expression="status == 'approved'"
)

# Contains check
content_check = graphbit.Node.condition(
    name="Content Check", 
    expression="content.contains('urgent')"
)
```

#### Boolean Logic
```python
# AND conditions
approval_gate = graphbit.Node.condition(
    name="Approval Gate", 
    expression="technical_approved == true && business_approved == true"
)

# OR conditions  
priority_check = graphbit.Node.condition(
    name="Priority Check", 
    expression="priority == 'high' || severity == 'critical'"
)
```

### Condition Node Examples

#### Quality Assurance
```python
qa_gate = graphbit.Node.condition(
    name="QA Gate",
    expression="quality_rating >= 8 && error_count == 0"
)
```

#### Content Moderation
```python
content_filter = graphbit.Node.condition(
    name="Content Filter", 
    expression="toxicity_score < 0.1 && sentiment != 'very_negative'"
)
```

#### Business Rules
```python
business_rule = graphbit.Node.condition(
    name="Business Rule",
    expression="budget_remaining > cost && approval_level >= required_level"
)
```

#### Threshold Checking
```python
performance_threshold = graphbit.Node.condition(
    name="Performance Threshold",
    expression="response_time < 1000 && error_rate < 0.01"
)
```

#### Data Validation
```python
data_validator = graphbit.Node.condition(
    name="Data Validator",
    expression="data_completeness > 0.95 && data_accuracy > 0.9"
)
```

## Transform Nodes

Transform nodes perform data processing and format conversions.

### Basic Transform Node

```python
transformer = graphbit.Node.transform(
    name="Text Transformer",
    transformation="uppercase"
)
```

**Parameters:**
- `name` (str): Node name
- `transformation` (str): Transformation type

### Available Transformations

#### Text Transformations
```python
# Convert to uppercase
upper = graphbit.Node.transform(
    name="Uppercase Converter", 
    transformation="uppercase"
)

# Convert to lowercase  
lower = graphbit.Node.transform(
    name="Lowercase Converter", 
    transformation="lowercase"
)
```

#### Data Extraction
```python
# Extract JSON from text
json_extractor = graphbit.Node.transform(
    name="JSON Extractor", 
    transformation="json_extract"
)
```

#### Text Processing
```python
# Split text
text_splitter = graphbit.Node.transform(
    name="Text Splitter", 
    transformation="split"
)

# Join text
text_joiner = graphbit.Node.transform(
    name="Text Joiner", 
    transformation="join"
)
```

### Transform Node Examples

#### Data Cleaning Pipeline
```python
# Clean and format text
text_cleaner = graphbit.Node.transform(
    name="Text Cleaner", 
    transformation="lowercase"
)

# Normalize data format
data_normalizer = graphbit.Node.transform(
    name="Data Normalizer",
    transformation="uppercase"
)
```

#### Format Conversion
```python
# Convert response to structured format
formatter = graphbit.Node.transform(
    name="Response Formatter",
    transformation="lowercase"
)
```

## Delay Nodes

Delay nodes add timing controls and rate limiting to workflows.

### Basic Delay Node

```python
delay = graphbit.PyWorkflowNode.delay_node(
    name="Rate Limiter",
    description="Prevents API rate limiting", 
    duration_seconds=5
)
```

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `duration_seconds` (int): Delay duration in seconds

### Delay Node Examples

#### Rate Limiting
```python
# API rate limiting
api_delay = graphbit.PyWorkflowNode.delay_node(
    name="API Rate Limit",
    description="Waits to respect API limits",
    duration_seconds=2
)

# Batch processing delay
batch_delay = graphbit.PyWorkflowNode.delay_node(
    name="Batch Delay", 
    description="Delays between batch items",
    duration_seconds=1
)
```

#### System Cooldown
```python
# Cool-down period
cooldown = graphbit.PyWorkflowNode.delay_node(
    name="System Cooldown",
    description="Allows system recovery time",
    duration_seconds=30
)
```

#### Scheduled Processing
```python
# Wait for scheduled time
scheduler_delay = graphbit.PyWorkflowNode.delay_node(
    name="Schedule Delay",
    description="Waits for next processing window", 
    duration_seconds=300  # 5 minutes
)
```

## Document Loader Nodes

Document loader nodes process and load various document types.

### Basic Document Loader

```python
loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Loader",
    description="Loads PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)
```

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `document_type` (str): Document type ("pdf", "txt", "docx", etc.)
- `source_path` (str): Path to the document

### Supported Document Types

#### PDF Documents
```python
pdf_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Document Loader",
    description="Loads and processes PDF files",
    document_type="pdf",
    source_path="documents/report.pdf"
)
```

#### Text Files
```python
text_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="Text File Loader", 
    description="Loads plain text files",
    document_type="txt",
    source_path="data/content.txt"
)
```

#### Word Documents
```python
docx_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="Word Document Loader",
    description="Loads Microsoft Word documents",
    document_type="docx", 
    source_path="documents/specification.docx"
)
```

## Node Connection Patterns

### Sequential Connections
Connect nodes for sequential processing:

```python
workflow = graphbit.Workflow("Sequential Pipeline")

# Add nodes
node1_id = workflow.add_node(input_processor)
node2_id = workflow.add_node(analyzer)
node3_id = workflow.add_node(output_formatter)

# Connect sequentially
workflow.connect(node1_id, node2_id)
workflow.connect(node2_id, node3_id)
```

### Conditional Connections
Use condition nodes for branching:

```python
workflow = graphbit.Workflow("Conditional Pipeline")

# Add nodes
analyzer_id = workflow.add_node(analyzer)
condition_id = workflow.add_node(quality_condition)
success_path_id = workflow.add_node(success_handler)
failure_path_id = workflow.add_node(failure_handler)

# Connect with conditions
workflow.connect(analyzer_id, condition_id)
workflow.connect(condition_id, success_path_id)  # Connect to success path
workflow.connect(condition_id, failure_path_id)  # Connect to failure path
```

### Parallel Processing
Process multiple branches simultaneously:

```python
workflow = graphbit.Workflow("Parallel Processing")

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

### Validation Chain
```python
def create_validation_chain():
    workflow = graphbit.Workflow("Validation Chain")
    
    # Input validator
    input_validator = graphbit.Node.condition(
        name="Input Validator", 
        expression="input_valid == true"
    )
    
    # Content processor
    processor = graphbit.Node.agent(
        name="Content Processor",
        prompt="Process this validated content: {input}",
        agent_id="processor"
    )
    
    # Output validator
    output_validator = graphbit.Node.condition(
        name="Output Validator", 
        expression="output_quality > 0.7"
    )
    
    # Connect validation chain
    input_id = workflow.add_node(input_validator)
    proc_id = workflow.add_node(processor)
    output_id = workflow.add_node(output_validator)
    
    workflow.connect(input_id, proc_id)
    workflow.connect(proc_id, output_id)
    
    return workflow
```

### Error Handling Pattern
```python
def create_error_handling_workflow():
    workflow = graphbit.Workflow("Error Handling")
    
    # Main processor
    main_processor = graphbit.Node.agent(
        name="Main Processor",
        prompt="Process: {input}",
        agent_id="main"
    )
    
    # Error detector
    error_detector = graphbit.Node.condition(
        name="Error Detector", 
        expression="error_occurred == false"
    )
    
    # Error handler
    error_handler = graphbit.Node.agent(
        name="Error Handler",
        prompt="Handle this error: {error_message}",
        agent_id="error_handler"
    )
    
    # Success handler
    success_handler = graphbit.Node.agent(
        name="Success Handler",
        prompt="Finalize successful result: {result}",
        agent_id="success_handler"
    )
    
    # Build error handling flow
    main_id = workflow.add_node(main_processor)
    detector_id = workflow.add_node(error_detector)
    error_id = workflow.add_node(error_handler)
    success_id = workflow.add_node(success_handler)
    
    workflow.connect(main_id, detector_id)
    workflow.connect(detector_id, error_id)   # Error path
    workflow.connect(detector_id, success_id) # Success path
    
    return workflow
```

### Multi-Step Analysis Pipeline
```python
def create_analysis_pipeline():
    workflow = graphbit.Workflow("Multi-Step Analysis")
    
    # Step 1: Initial analysis
    initial_analyzer = graphbit.Node.agent(
        name="Initial Analyzer",
        prompt="Perform initial analysis of: {input}",
        agent_id="initial_analyzer"
    )
    
    # Step 2: Quality check
    quality_check = graphbit.Node.condition(
        name="Quality Check",
        expression="initial_quality > 0.6"
    )
    
    # Step 3: Deep analysis (if quality is good)
    deep_analyzer = graphbit.Node.agent(
        name="Deep Analyzer",
        prompt="Perform deep analysis of: {analyzed_content}",
        agent_id="deep_analyzer"
    )
    
    # Step 4: Final formatter
    formatter = graphbit.Node.transform(
        name="Result Formatter",
        transformation="uppercase"
    )
    
    # Connect the pipeline
    initial_id = workflow.add_node(initial_analyzer)
    quality_id = workflow.add_node(quality_check)
    deep_id = workflow.add_node(deep_analyzer)
    format_id = workflow.add_node(formatter)
    
    workflow.connect(initial_id, quality_id)
    workflow.connect(quality_id, deep_id)
    workflow.connect(deep_id, format_id)
    
    return workflow
```

## Node Properties and Methods

All nodes share common properties:

```python
# Access node properties
node_id = node.id()    # Unique identifier
node_name = node.name() # Human-readable name
```

## Best Practices

### 1. Descriptive Names
Use clear, descriptive names for all nodes:

```python
# Good
email_sentiment_analyzer = graphbit.Node.agent(
    name="Email Sentiment Analyzer",
    prompt="Analyze sentiment of customer emails: {email_content}",
    agent_id="email_sentiment"
)

# Avoid
node1 = graphbit.Node.agent(
    name="Node1", 
    prompt="Do: {input}",
    agent_id="n1"
)
```

### 2. Single Responsibility
Each node should have one clear purpose:

```python
# Good - focused on one task
spam_detector = graphbit.Node.agent(
    name="Spam Detector",
    prompt="Is this email spam? {email}",
    agent_id="spam_detector"
)

# Avoid - too many responsibilities  
everything_processor = graphbit.Node.agent(
    name="Everything Processor",
    prompt="Do everything with: {input}",
    agent_id="everything"
)
```

### 3. Appropriate Node Types
Choose the right node type for each task:

- **Agent Nodes**: AI/LLM processing tasks
- **Condition Nodes**: Decision making and branching
- **Transform Nodes**: Data format conversion
- **Delay Nodes**: Timing and rate control
- **Document Loaders**: File processing

### 4. Error Handling
Include appropriate error handling and validation:

```python
# Validation before processing
validator = graphbit.Node.condition(
    name="Input Validator", 
    expression="data_valid == true"
)

# Error recovery
error_handler = graphbit.Node.agent(
    name="Error Handler",
    prompt="Safely handle this error: {error}",
    agent_id="error_handler"
)
```

### 5. Clear Prompt Design
Write clear, specific prompts for agent nodes:

```python
# Good - specific and clear
summarizer = graphbit.Node.agent(
    name="Document Summarizer",
    prompt="""
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
bad_summarizer = graphbit.Node.agent(
    name="Summarizer",
    prompt="Summarize: {input}",
    agent_id="summarizer"
)
```

Understanding these node types and their usage patterns enables you to build sophisticated, reliable workflows that handle complex AI processing tasks effectively. Choose the appropriate node type for each step in your workflow, and connect them in logical patterns to achieve your processing goals.
