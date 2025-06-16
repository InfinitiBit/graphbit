# Workflow Builder

The Workflow Builder is GraphBit's primary tool for creating complex AI agent workflows using a simple, intuitive builder pattern.

## Overview

The `PyWorkflowBuilder` class provides a fluent interface for constructing workflows by:
- Adding nodes (agents, conditions, transforms)
- Connecting nodes with edges
- Configuring workflow properties
- Validating workflow structure

## Basic Usage

### Creating a Builder

```python
import graphbit

# Create a new workflow builder
builder = graphbit.PyWorkflowBuilder("My Workflow")
builder.description("A workflow for processing data")
```

### Adding Nodes

```python
# Add an agent node
analyzer = graphbit.PyWorkflowNode.agent_node(
    name="Data Analyzer",
    description="Analyzes input data",
    agent_id="analyzer",
    prompt="Analyze this data: {input}"
)

node_id = builder.add_node(analyzer)
```

### Connecting Nodes

```python
# Connect nodes with data flow
builder.connect(node1_id, node2_id, graphbit.PyWorkflowEdge.data_flow())
```

### Building the Workflow

```python
# Build the final workflow
workflow = builder.build()
```

## Advanced Workflow Patterns

### Sequential Processing

Create linear processing pipelines:

```python
builder = graphbit.PyWorkflowBuilder("Sequential Pipeline")

# Create nodes
step1 = graphbit.PyWorkflowNode.agent_node("Step 1", "First step", "step1", "Process: {input}")
step2 = graphbit.PyWorkflowNode.agent_node("Step 2", "Second step", "step2", "Refine: {step1_output}")
step3 = graphbit.PyWorkflowNode.agent_node("Step 3", "Final step", "step3", "Finalize: {step2_output}")

# Add nodes
id1 = builder.add_node(step1)
id2 = builder.add_node(step2)
id3 = builder.add_node(step3)

# Connect sequentially
builder.connect(id1, id2, graphbit.PyWorkflowEdge.data_flow())
builder.connect(id2, id3, graphbit.PyWorkflowEdge.data_flow())

workflow = builder.build()
```

### Parallel Processing

Create workflows that process multiple branches simultaneously:

```python
builder = graphbit.PyWorkflowBuilder("Parallel Processing")

# Input node
input_node = graphbit.PyWorkflowNode.agent_node("Input", "Processes input", "input", "Prepare: {input}")

# Parallel processing branches
branch1 = graphbit.PyWorkflowNode.agent_node("Branch 1", "First analysis", "branch1", "Analyze A: {input_data}")
branch2 = graphbit.PyWorkflowNode.agent_node("Branch 2", "Second analysis", "branch2", "Analyze B: {input_data}")
branch3 = graphbit.PyWorkflowNode.agent_node("Branch 3", "Third analysis", "branch3", "Analyze C: {input_data}")

# Aggregator node
aggregator = graphbit.PyWorkflowNode.agent_node(
    "Aggregator", 
    "Combines results", 
    "aggregator", 
    "Combine results: {branch1_output}, {branch2_output}, {branch3_output}"
)

# Build graph
input_id = builder.add_node(input_node)
b1_id = builder.add_node(branch1)
b2_id = builder.add_node(branch2)
b3_id = builder.add_node(branch3)
agg_id = builder.add_node(aggregator)

# Connect parallel branches
builder.connect(input_id, b1_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(input_id, b2_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(input_id, b3_id, graphbit.PyWorkflowEdge.data_flow())

# Aggregate results
builder.connect(b1_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(b2_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(b3_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
```

### Conditional Workflows

Use condition nodes for dynamic routing:

```python
builder = graphbit.PyWorkflowBuilder("Conditional Workflow")

# Analyzer
analyzer = graphbit.PyWorkflowNode.agent_node(
    "Analyzer", 
    "Analyzes content quality", 
    "analyzer", 
    "Rate quality 1-10: {input}"
)

# Quality gate
quality_gate = graphbit.PyWorkflowNode.condition_node(
    "Quality Gate",
    "Checks if quality is acceptable",
    "quality_score >= 7"
)

# High quality path
approve = graphbit.PyWorkflowNode.agent_node(
    "Approver", 
    "Approves high quality content", 
    "approver", 
    "Approve: {input}"
)

# Low quality path
reject = graphbit.PyWorkflowNode.agent_node(
    "Rejector", 
    "Handles low quality content", 
    "rejector", 
    "Suggest improvements: {input}"
)

# Build conditional flow
analyzer_id = builder.add_node(analyzer)
gate_id = builder.add_node(quality_gate)
approve_id = builder.add_node(approve)
reject_id = builder.add_node(reject)

builder.connect(analyzer_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
builder.connect(gate_id, approve_id, graphbit.PyWorkflowEdge.conditional("quality_score >= 7"))
builder.connect(gate_id, reject_id, graphbit.PyWorkflowEdge.conditional("quality_score < 7"))
```

## Data Transformation

### Using Transform Nodes

```python
# JSON extraction
json_extractor = graphbit.PyWorkflowNode.transform_node(
    "JSON Extractor",
    "Extracts JSON from text",
    "json_extract"
)

# Text transformation
uppercaser = graphbit.PyWorkflowNode.transform_node(
    "Uppercaser",
    "Converts to uppercase",
    "uppercase"
)

# Add to workflow
builder.add_node(json_extractor)
builder.add_node(uppercaser)
```

### Available Transformations

- `uppercase` - Convert text to uppercase
- `lowercase` - Convert text to lowercase  
- `json_extract` - Extract JSON objects from text
- `split` - Split text by delimiter
- `join` - Join text with delimiter

## Error Handling in Workflows

### Adding Delays

```python
# Rate limiting delay
rate_limit = graphbit.PyWorkflowNode.delay_node(
    "Rate Limiter",
    "Prevents API rate limiting",
    5  # 5 seconds
)

builder.add_node(rate_limit)
```

### Validation Points

```python
# Add validation at key points
validator = graphbit.PyWorkflowNode.condition_node(
    "Validator",
    "Validates intermediate results",
    "result_valid == true"
)
```

## Best Practices

### 1. Descriptive Names

```python
# Good
content_analyzer = graphbit.PyWorkflowNode.agent_node(
    name="SEO Content Analyzer",
    description="Analyzes content for SEO optimization",
    agent_id="seo_analyzer",
    prompt="Analyze for SEO: {content}"
)

# Avoid
node1 = graphbit.PyWorkflowNode.agent_node("Node1", "Does stuff", "n1", "Do: {input}")
```

### 2. Logical Grouping

```python
# Group related functionality
def create_content_analysis_section(builder):
    analyzer = graphbit.PyWorkflowNode.agent_node(...)
    validator = graphbit.PyWorkflowNode.condition_node(...)
    formatter = graphbit.PyWorkflowNode.transform_node(...)
    
    # Connect and return IDs
    return analyzer_id, validator_id, formatter_id
```

### 3. Error Recovery

```python
# Add error handling nodes
error_handler = graphbit.PyWorkflowNode.agent_node(
    "Error Handler",
    "Handles processing errors",
    "error_handler",
    "Handle error: {error_message}"
)
```

### 4. Workflow Validation

```python
# Always validate before execution
try:
    workflow = builder.build()
    workflow.validate()
    print("✅ Workflow is valid")
except Exception as e:
    print(f"❌ Workflow validation failed: {e}")
```

## Complex Example: Content Creation Pipeline

```python
def create_content_creation_workflow():
    builder = graphbit.PyWorkflowBuilder("Content Creation Pipeline")
    builder.description("Research → Write → Edit → Review → Publish")
    
    # Research phase
    researcher = graphbit.PyWorkflowNode.agent_node(
        "Researcher", "Researches topic", "researcher",
        "Research key points about: {topic}"
    )
    
    # Writing phase
    writer = graphbit.PyWorkflowNode.agent_node(
        "Writer", "Writes initial content", "writer",
        "Write article about {topic} using: {research_data}"
    )
    
    # Editing phase
    editor = graphbit.PyWorkflowNode.agent_node(
        "Editor", "Edits for clarity", "editor",
        "Edit and improve: {draft_content}"
    )
    
    # Quality review
    reviewer = graphbit.PyWorkflowNode.agent_node(
        "Reviewer", "Reviews quality", "reviewer",
        "Review quality and rate 1-10: {edited_content}"
    )
    
    # Quality gate
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        "Quality Gate", "Checks quality threshold", "quality_rating >= 8"
    )
    
    # Publication formatter
    formatter = graphbit.PyWorkflowNode.transform_node(
        "Formatter", "Formats for publication", "uppercase"
    )
    
    # Build workflow
    ids = {}
    ids['research'] = builder.add_node(researcher)
    ids['write'] = builder.add_node(writer)
    ids['edit'] = builder.add_node(editor)
    ids['review'] = builder.add_node(reviewer)
    ids['gate'] = builder.add_node(quality_gate)
    ids['format'] = builder.add_node(formatter)
    
    # Connect workflow
    builder.connect(ids['research'], ids['write'], graphbit.PyWorkflowEdge.data_flow())
    builder.connect(ids['write'], ids['edit'], graphbit.PyWorkflowEdge.data_flow())
    builder.connect(ids['edit'], ids['review'], graphbit.PyWorkflowEdge.data_flow())
    builder.connect(ids['review'], ids['gate'], graphbit.PyWorkflowEdge.data_flow())
    builder.connect(ids['gate'], ids['format'], graphbit.PyWorkflowEdge.conditional("quality_rating >= 8"))
    
    return builder.build()

# Usage
workflow = create_content_creation_workflow()
```

## Workflow Templates

Create reusable workflow templates:

```python
class WorkflowTemplates:
    @staticmethod
    def simple_analysis():
        builder = graphbit.PyWorkflowBuilder("Simple Analysis")
        analyzer = graphbit.PyWorkflowNode.agent_node(
            "Analyzer", "Analyzes input", "analyzer", "Analyze: {input}"
        )
        builder.add_node(analyzer)
        return builder.build()
    
    @staticmethod
    def two_step_process():
        builder = graphbit.PyWorkflowBuilder("Two Step Process")
        step1 = graphbit.PyWorkflowNode.agent_node("Step 1", "First step", "step1", "Process: {input}")
        step2 = graphbit.PyWorkflowNode.agent_node("Step 2", "Second step", "step2", "Refine: {step1_output}")
        
        id1 = builder.add_node(step1)
        id2 = builder.add_node(step2)
        builder.connect(id1, id2, graphbit.PyWorkflowEdge.data_flow())
        
        return builder.build()

# Usage
workflow = WorkflowTemplates.simple_analysis()
```

## Troubleshooting

### Common Issues

1. **Missing Connections**: Ensure all nodes are properly connected
2. **Circular Dependencies**: GraphBit prevents cycles automatically
3. **Invalid Conditions**: Check condition expressions for syntax errors
4. **Node ID Conflicts**: Each node gets a unique ID automatically

### Debugging Tips

```python
# Check workflow structure
print(f"Nodes: {workflow.node_count()}")
print(f"Edges: {workflow.edge_count()}")

# Validate before execution
workflow.validate()

# Export to JSON for inspection
json_str = workflow.to_json()
print(json_str)
```

The Workflow Builder provides a powerful, flexible way to create sophisticated AI agent workflows. Start with simple patterns and gradually build more complex orchestrations as needed. 