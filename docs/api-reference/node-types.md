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
agent = graphbit.PyWorkflowNode.agent_node(
    name="Content Analyzer",
    description="Analyzes content for insights",
    agent_id="analyzer",
    prompt="Analyze the following content: {input}"
)
```

**Parameters:**
- `name` (str): Human-readable node name
- `description` (str): Purpose and functionality description
- `agent_id` (str): Unique identifier for the agent
- `prompt` (str): LLM prompt template with variable placeholders

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
sentiment_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Sentiment Analyzer",
    description="Analyzes text sentiment",
    agent_id="sentiment_analyzer",
    prompt="""
    Analyze the sentiment of this text: "{text}"
    
    Provide:
    - Overall sentiment (positive/negative/neutral)
    - Confidence score (0-1)
    - Key emotional indicators
    """
)
```

#### Code Review Agent
```python
code_reviewer = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Code Reviewer",
    description="Reviews code for quality and security",
    agent_id="code_reviewer",
    prompt="""
    Review this code for quality and security issues:
    
    {code}
    
    Check for:
    - Security vulnerabilities
    - Performance issues
    - Code style problems
    - Best practices violations
    """,
    max_tokens=1500,
    temperature=0.2
)
```

#### Data Processing Agent
```python
data_processor = graphbit.PyWorkflowNode.agent_node(
    name="Data Processor",
    description="Processes and summarizes data",
    agent_id="data_processor", 
    prompt="""
    Process this dataset and provide insights:
    
    Data: {dataset}
    
    Include:
    1. Statistical summary
    2. Key trends
    3. Anomalies
    4. Recommendations
    """
)
```

## Condition Nodes

Condition nodes enable branching logic and decision-making in workflows.

### Basic Condition Node

```python
condition = graphbit.PyWorkflowNode.condition_node(
    name="Quality Gate",
    description="Checks if quality meets threshold",
    expression="quality_score > 0.8"
)
```

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description  
- `expression` (str): Boolean expression to evaluate

### Condition Expressions

Condition nodes support various comparison operators:

#### Numeric Comparisons
```python
# Greater than
high_score = graphbit.PyWorkflowNode.condition_node(
    "High Score Check", "Checks for high scores", "score > 80"
)

# Range checks
valid_range = graphbit.PyWorkflowNode.condition_node(
    "Range Validator", "Validates value range", "value >= 10 && value <= 100"
)

# Multiple conditions
complex_check = graphbit.PyWorkflowNode.condition_node(
    "Complex Check", "Complex validation", 
    "(score > 75 && confidence > 0.8) || priority == 'high'"
)
```

#### String Comparisons
```python
# Equality
status_check = graphbit.PyWorkflowNode.condition_node(
    "Status Check", "Checks status", "status == 'approved'"
)

# Contains
content_check = graphbit.PyWorkflowNode.condition_node(
    "Content Check", "Checks for keywords", "content.contains('urgent')"
)
```

#### Boolean Logic
```python
# AND conditions
approval_gate = graphbit.PyWorkflowNode.condition_node(
    "Approval Gate", "Multi-factor approval", 
    "technical_approved == true && business_approved == true"
)

# OR conditions  
priority_check = graphbit.PyWorkflowNode.condition_node(
    "Priority Check", "Checks priority", 
    "priority == 'high' || severity == 'critical'"
)
```

### Condition Node Examples

#### Quality Assurance
```python
qa_gate = graphbit.PyWorkflowNode.condition_node(
    name="QA Gate",
    description="Quality assurance checkpoint",
    expression="quality_rating >= 8 && error_count == 0"
)
```

#### Content Moderation
```python
content_filter = graphbit.PyWorkflowNode.condition_node(
    name="Content Filter",
    description="Filters inappropriate content", 
    expression="toxicity_score < 0.1 && sentiment != 'very_negative'"
)
```

#### Business Rules
```python
business_rule = graphbit.PyWorkflowNode.condition_node(
    name="Business Rule",
    description="Applies business validation",
    expression="budget_remaining > cost && approval_level >= required_level"
)
```

## Transform Nodes

Transform nodes perform data processing and format conversions.

### Basic Transform Node

```python
transformer = graphbit.PyWorkflowNode.transform_node(
    name="Text Transformer",
    description="Transforms text format",
    transformation="uppercase"
)
```

**Parameters:**
- `name` (str): Node name
- `description` (str): Node description
- `transformation` (str): Transformation type

### Available Transformations

#### Text Transformations
```python
# Convert to uppercase
upper = graphbit.PyWorkflowNode.transform_node(
    "Uppercase", "Converts to uppercase", "uppercase"
)

# Convert to lowercase  
lower = graphbit.PyWorkflowNode.transform_node(
    "Lowercase", "Converts to lowercase", "lowercase"
)
```

#### Data Extraction
```python
# Extract JSON from text
json_extractor = graphbit.PyWorkflowNode.transform_node(
    "JSON Extractor", "Extracts JSON objects", "json_extract"
)
```

#### Text Processing
```python
# Split text
text_splitter = graphbit.PyWorkflowNode.transform_node(
    "Text Splitter", "Splits text by delimiter", "split"
)

# Join text
text_joiner = graphbit.PyWorkflowNode.transform_node(
    "Text Joiner", "Joins text with delimiter", "join"
)
```

### Transform Node Examples

#### Data Cleaning Pipeline
```python
# Extract JSON from LLM response
json_extractor = graphbit.PyWorkflowNode.transform_node(
    name="JSON Extractor",
    description="Extracts structured data from response",
    transformation="json_extract"
)

# Clean and format text
text_cleaner = graphbit.PyWorkflowNode.transform_node(
    name="Text Cleaner", 
    description="Cleans and normalizes text",
    transformation="lowercase"
)
```

#### Format Conversion
```python
# Convert response to structured format
formatter = graphbit.PyWorkflowNode.transform_node(
    name="Response Formatter",
    description="Formats response for next stage",
    transformation="json_extract"
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
builder = graphbit.PyWorkflowBuilder("Sequential Pipeline")

# Add nodes
node1_id = builder.add_node(input_processor)
node2_id = builder.add_node(analyzer)
node3_id = builder.add_node(output_formatter)

# Connect sequentially
builder.connect(node1_id, node2_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(node2_id, node3_id, graphbit.PyWorkflowEdge.data_flow())
```

### Conditional Connections
Use condition nodes for branching:

```python
# Condition-based routing
builder.connect(analyzer_id, condition_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(condition_id, success_path_id, graphbit.PyWorkflowEdge.conditional("score > 0.8"))
builder.connect(condition_id, failure_path_id, graphbit.PyWorkflowEdge.conditional("score <= 0.8"))
```

### Parallel Processing
Process multiple branches simultaneously:

```python
# Fan-out to parallel processors
builder.connect(input_id, processor1_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(input_id, processor2_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(input_id, processor3_id, graphbit.PyWorkflowEdge.data_flow())

# Fan-in to aggregator
builder.connect(processor1_id, aggregator_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(processor2_id, aggregator_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(processor3_id, aggregator_id, graphbit.PyWorkflowEdge.data_flow())
```

## Advanced Node Patterns

### Validation Chain
```python
def create_validation_chain():
    builder = graphbit.PyWorkflowBuilder("Validation Chain")
    
    # Input validator
    input_validator = graphbit.PyWorkflowNode.condition_node(
        "Input Validator", "Validates input format", "input_valid == true"
    )
    
    # Content processor
    processor = graphbit.PyWorkflowNode.agent_node(
        "Content Processor", "Processes valid content", "processor",
        "Process this validated content: {input}"
    )
    
    # Output validator
    output_validator = graphbit.PyWorkflowNode.condition_node(
        "Output Validator", "Validates output quality", "output_quality > 0.7"
    )
    
    # Connect validation chain
    input_id = builder.add_node(input_validator)
    proc_id = builder.add_node(processor)
    output_id = builder.add_node(output_validator)
    
    builder.connect(input_id, proc_id, graphbit.PyWorkflowEdge.conditional("input_valid == true"))
    builder.connect(proc_id, output_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Error Handling Pattern
```python
def create_error_handling_workflow():
    builder = graphbit.PyWorkflowBuilder("Error Handling")
    
    # Main processor
    main_processor = graphbit.PyWorkflowNode.agent_node(
        "Main Processor", "Primary processing", "main", "Process: {input}"
    )
    
    # Error detector
    error_detector = graphbit.PyWorkflowNode.condition_node(
        "Error Detector", "Detects processing errors", "error_occurred == false"
    )
    
    # Error handler
    error_handler = graphbit.PyWorkflowNode.agent_node(
        "Error Handler", "Handles errors", "error_handler", 
        "Handle this error: {error_message}"
    )
    
    # Success handler
    success_handler = graphbit.PyWorkflowNode.agent_node(
        "Success Handler", "Handles success", "success_handler",
        "Finalize successful result: {result}"
    )
    
    # Build error handling flow
    main_id = builder.add_node(main_processor)
    detector_id = builder.add_node(error_detector)
    error_id = builder.add_node(error_handler)
    success_id = builder.add_node(success_handler)
    
    builder.connect(main_id, detector_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(detector_id, error_id, graphbit.PyWorkflowEdge.conditional("error_occurred == true"))
    builder.connect(detector_id, success_id, graphbit.PyWorkflowEdge.conditional("error_occurred == false"))
    
    return builder.build()
```

## Node Properties and Methods

All nodes share common properties:

```python
# Access node properties
node_id = node.id()           # Unique identifier
node_name = node.name()       # Human-readable name  
node_desc = node.description() # Description text
```

## Best Practices

### 1. Descriptive Names
Use clear, descriptive names for all nodes:

```python
# Good
email_sentiment_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Email Sentiment Analyzer",
    description="Analyzes sentiment of customer emails",
    agent_id="email_sentiment", 
    prompt="Analyze email sentiment: {email_content}"
)

# Avoid
node1 = graphbit.PyWorkflowNode.agent_node(
    name="Node1", description="Does stuff", agent_id="n1", prompt="Do: {input}"
)
```

### 2. Single Responsibility
Each node should have one clear purpose:

```python
# Good - focused on one task
spam_detector = graphbit.PyWorkflowNode.agent_node(
    name="Spam Detector",
    description="Detects spam emails",
    agent_id="spam_detector",
    prompt="Is this email spam? {email}"
)

# Avoid - too many responsibilities  
everything_processor = graphbit.PyWorkflowNode.agent_node(
    name="Everything Processor",
    description="Processes everything",
    agent_id="everything",
    prompt="Do everything with: {input}"
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
validator = graphbit.PyWorkflowNode.condition_node(
    "Input Validator", "Validates input data", "data_valid == true"
)

# Error recovery
error_handler = graphbit.PyWorkflowNode.agent_node(
    "Error Handler", "Handles processing errors", "error_handler",
    "Safely handle this error: {error}"
)
```

Understanding these node types and their usage patterns enables you to build sophisticated, reliable workflows that handle complex AI processing tasks effectively. 
