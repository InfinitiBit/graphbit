# GraphBit Examples

This directory contains example workflows and configurations to help you get started with GraphBit.

## Quick Start

1. **Set up your environment**:
   ```bash
   # For OpenAI (if using cloud models)
   export OPENAI_API_KEY="your-api-key"
   
   # For Ollama (if using local models)
   ollama serve
   ollama pull llama3.1
   ```

2. **Run a basic example**:
   ```bash
   # Validate the workflow
   graphbit validate examples/workflows/simple_analysis.json
   
   # Execute the workflow
   graphbit run examples/workflows/simple_analysis.json
   ```

## Available Examples

### Basic Workflows

#### Simple Analysis (`workflows/simple_analysis.json`)
A single-node workflow that demonstrates basic agent usage.

**Use case**: Text analysis and summarization
**LLM provider**: Any (OpenAI, Anthropic, Ollama)
**Run time**: ~30 seconds

```bash
graphbit run examples/workflows/simple_analysis.json
```

#### Data Pipeline (`workflows/data_pipeline.json`)
Multi-stage data processing workflow with sequential execution.

**Use case**: Extract → Transform → Load pattern
**Features**: Data flow between nodes, error handling
**Run time**: ~1-2 minutes

```bash
graphbit run examples/workflows/data_pipeline.json
```

### Advanced Workflows

#### Parallel Processing (`workflows/parallel_processing.json`)
Demonstrates concurrent execution of independent tasks.

**Use case**: Batch processing, parallel analysis
**Features**: Concurrent execution, result aggregation
**Run time**: ~45 seconds (vs 2+ minutes sequential)

```bash
graphbit run examples/workflows/parallel_processing.json
```

#### Conditional Workflow (`workflows/conditional_flow.json`)
Shows conditional branching based on analysis results.

**Use case**: Dynamic workflow routing, quality control
**Features**: Conditional edges, validation nodes
**Run time**: ~1 minute

```bash
graphbit run examples/workflows/conditional_flow.json
```

#### Content Generation (`workflows/content_generation.json`)
Multi-agent content creation pipeline.

**Use case**: Research → Write → Edit workflow
**Features**: Specialized agents, content refinement
**Run time**: ~2-3 minutes

```bash
graphbit run examples/workflows/content_generation.json
```

### Domain-Specific Examples

#### Code Review (`workflows/code_review.json`)
Automated code review using specialized models.

**Use case**: Code quality analysis, security scanning
**Recommended models**: CodeLlama, GPT-4, Claude
**Run time**: ~1-2 minutes

```bash
graphbit run examples/workflows/code_review.json
```

#### Research Assistant (`workflows/research_assistant.json`)
Academic research and citation workflow.

**Use case**: Literature review, fact-checking
**Features**: Web search integration, citation formatting
**Run time**: ~3-5 minutes

```bash
graphbit run examples/workflows/research_assistant.json
```

#### Customer Support (`workflows/customer_support.json`)
Multi-stage customer inquiry processing.

**Use case**: Ticket classification, response generation
**Features**: Sentiment analysis, priority routing
**Run time**: ~1 minute

```bash
graphbit run examples/workflows/customer_support.json
```

## Configurations

### Provider Configurations

#### OpenAI Configuration (`configs/openai.json`)
```json
{
  "llm": {
    "provider": "openai",
    "model": "gpt-4",
    "api_key": "${OPENAI_API_KEY}"
  },
  "agents": {
    "default": {
      "max_tokens": 1000,
      "temperature": 0.7
    }
  }
}
```

#### Ollama Configuration (`configs/ollama.json`)
```json
{
  "llm": {
    "provider": "ollama",
    "model": "llama3.1",
    "base_url": "http://localhost:11434"
  },
  "agents": {
    "default": {
      "max_tokens": 1000,
      "temperature": 0.7
    }
  }
}
```

#### Multi-Provider Configuration (`configs/multi_provider.json`)
```json
{
  "llm": {
    "provider": "openai",
    "base_url": "https://api.openai.com/v1"
  },
  "agents": {
    "fast_agent": {
      "provider": "ollama",
      "model": "phi3",
      "temperature": 0.8
    },
    "quality_agent": {
      "provider": "openai", 
      "model": "gpt-4",
      "temperature": 0.3
    }
  }
}
```

### Specialized Configurations

#### Development Configuration (`configs/development.json`)
Optimized for fast iteration and debugging.

- Lower token limits for faster responses
- Higher temperature for creative outputs
- Verbose logging enabled

#### Production Configuration (`configs/production.json`)
Optimized for reliability and quality.

- Conservative retry settings
- Lower temperature for consistent outputs
- Comprehensive error handling

## Python Examples

### Basic Usage (`example.py`)

```python
import graphbit
import os

# Set up configuration
graphbit.init()

# Create LLM configuration
api_key = os.getenv("OPENAI_API_KEY")
llm_config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")

# Create and run workflow
builder = graphbit.PyWorkflowBuilder("Example Workflow")
node = graphbit.PyWorkflowNode.agent_node(
    "Analyzer",
    "Analyzes input text",
    "analyzer",
    "Analyze this text: {input}"
)

workflow, _ = builder.add_node(node)
workflow = builder.build()

# Execute
executor = graphbit.PyWorkflowExecutor(llm_config)
result = executor.execute(workflow, {"input": "Hello, world!"})
print(f"Result: {result}")
```

### Advanced Python Example

```python
import asyncio
import graphbit

async def advanced_workflow():
    # Create multi-agent workflow
    builder = graphbit.PyWorkflowBuilder("Advanced Analysis")
    
    # Research node
    research_node = graphbit.PyWorkflowNode.agent_node(
        "Researcher",
        "Conducts research on the topic",
        "researcher",
        "Research the topic: {topic}"
    )
    
    # Analysis node
    analysis_node = graphbit.PyWorkflowNode.agent_node(
        "Analyst", 
        "Analyzes research findings",
        "analyst",
        "Analyze these findings: {research_data}"
    )
    
    # Build workflow
    workflow, research_id = builder.add_node(research_node)
    workflow, analysis_id = builder.add_node(analysis_node)
    
    # Connect nodes
    workflow = builder.add_edge(
        research_id, 
        analysis_id, 
        graphbit.EdgeType.DataFlow
    )
    
    workflow = builder.build()
    
    # Execute with input
    executor = graphbit.PyWorkflowExecutor(config)
    result = await executor.execute_async(workflow, {
        "topic": "Artificial Intelligence trends"
    })
    
    return result

# Run the workflow
result = asyncio.run(advanced_workflow())
print(f"Analysis complete: {result}")
```

## Creating Your Own Examples

### Workflow Structure

All workflows follow this basic structure:

```json
{
  "name": "Your Workflow Name",
  "description": "Brief description of what this workflow does",
  "nodes": [
    {
      "id": "unique-node-id",
      "type": "agent|condition|transform|delay",
      "name": "Human-readable name", 
      "description": "What this node does",
      "config": {
        // Node-specific configuration
      }
    }
  ],
  "edges": [
    {
      "from": "source-node-id",
      "to": "target-node-id", 
      "type": "data_flow|control_flow|conditional"
    }
  ]
}
```

### Node Types and Configurations

#### Agent Node
```json
{
  "id": "analyzer",
  "type": "agent",
  "name": "Data Analyzer",
  "config": {
    "agent_id": "default",
    "prompt": "Analyze this data: {input}",
    "max_tokens": 1000,
    "temperature": 0.7
  }
}
```

#### Condition Node
```json
{
  "id": "quality_check",
  "type": "condition", 
  "name": "Quality Gate",
  "config": {
    "expression": "confidence > 0.8 && length > 100"
  }
}
```

#### Transform Node
```json
{
  "id": "formatter",
  "type": "transform",
  "name": "Format Output",
  "config": {
    "transformation": "uppercase|lowercase|json_extract",
    "field": "content"
  }
}
```

#### Delay Node
```json
{
  "id": "wait",
  "type": "delay",
  "name": "Wait Period", 
  "config": {
    "duration_seconds": 30
  }
}
```

### Edge Types

#### Data Flow
Passes output from source node as input to target node.
```json
{"from": "analyzer", "to": "processor", "type": "data_flow"}
```

#### Control Flow  
Ensures execution order without passing data.
```json
{"from": "setup", "to": "main_task", "type": "control_flow"}
```

#### Conditional
Only executes if condition is met.
```json
{
  "from": "validator", 
  "to": "publisher", 
  "type": "conditional",
  "condition": "validation_passed == true"
}
```

### Best Practices

#### Prompt Design
- Use clear, specific instructions
- Include examples for complex tasks
- Use template variables for dynamic content
- Test prompts independently before workflow integration

```json
{
  "prompt": "Analyze the following text for sentiment and key themes:\n\nText: {input_text}\n\nProvide your analysis in JSON format:\n{\n  \"sentiment\": \"positive|negative|neutral\",\n  \"confidence\": 0.95,\n  \"key_themes\": [\"theme1\", \"theme2\"]\n}"
}
```

#### Error Handling
- Set appropriate timeout values
- Configure retry logic for unreliable operations
- Use validation nodes to catch errors early
- Provide fallback paths for critical workflows

```json
{
  "config": {
    "retry": {
      "max_attempts": 3,
      "delay_seconds": 5,
      "exponential_backoff": true
    },
    "timeout_seconds": 120
  }
}
```

#### Performance Optimization
- Use parallel execution for independent tasks
- Choose appropriate model sizes for each task
- Cache results when possible
- Monitor token usage and costs

### Testing Your Examples

1. **Validate syntax**:
   ```bash
   graphbit validate examples/workflows/your_workflow.json
   ```

2. **Test with mock data**:
   ```bash
   echo '{"input": "test data"}' | graphbit run examples/workflows/your_workflow.json
   ```

3. **Debug mode**:
   ```bash
   RUST_LOG=debug graphbit run examples/workflows/your_workflow.json --verbose
   ```

4. **Benchmark performance**:
   ```bash
   time graphbit run examples/workflows/your_workflow.json
   ```

## Common Patterns

### Fan-Out/Fan-In
Process multiple items in parallel, then aggregate results.

```json
{
  "nodes": [
    {"id": "splitter", "type": "transform", "config": {"transformation": "split"}},
    {"id": "processor1", "type": "agent", "config": {"prompt": "Process: {item}"}},
    {"id": "processor2", "type": "agent", "config": {"prompt": "Process: {item}"}}, 
    {"id": "processor3", "type": "agent", "config": {"prompt": "Process: {item}"}},
    {"id": "aggregator", "type": "agent", "config": {"prompt": "Combine results: {results}"}}
  ],
  "edges": [
    {"from": "splitter", "to": "processor1", "type": "data_flow"},
    {"from": "splitter", "to": "processor2", "type": "data_flow"},
    {"from": "splitter", "to": "processor3", "type": "data_flow"},
    {"from": "processor1", "to": "aggregator", "type": "data_flow"},
    {"from": "processor2", "to": "aggregator", "type": "data_flow"},
    {"from": "processor3", "to": "aggregator", "type": "data_flow"}
  ]
}
```

### Pipeline Pattern
Sequential processing stages with validation.

```json
{
  "nodes": [
    {"id": "extract", "type": "agent", "config": {"prompt": "Extract data from: {input}"}},
    {"id": "validate1", "type": "condition", "config": {"expression": "extracted_items > 0"}},
    {"id": "transform", "type": "agent", "config": {"prompt": "Transform data: {data}"}},
    {"id": "validate2", "type": "condition", "config": {"expression": "quality_score > 0.8"}},
    {"id": "load", "type": "agent", "config": {"prompt": "Format for output: {data}"}}
  ],
  "edges": [
    {"from": "extract", "to": "validate1", "type": "data_flow"},
    {"from": "validate1", "to": "transform", "type": "conditional", "condition": "passed"},
    {"from": "transform", "to": "validate2", "type": "data_flow"},
    {"from": "validate2", "to": "load", "type": "conditional", "condition": "passed"}
  ]
}
```

### Router Pattern
Dynamic routing based on content analysis.

```json
{
  "nodes": [
    {"id": "classifier", "type": "agent", "config": {"prompt": "Classify this request: {input}"}},
    {"id": "route_simple", "type": "agent", "config": {"prompt": "Handle simple request: {input}"}},
    {"id": "route_complex", "type": "agent", "config": {"prompt": "Handle complex request: {input}"}},
    {"id": "route_escalate", "type": "agent", "config": {"prompt": "Escalate request: {input}"}}
  ],
  "edges": [
    {"from": "classifier", "to": "route_simple", "type": "conditional", "condition": "classification == 'simple'"},
    {"from": "classifier", "to": "route_complex", "type": "conditional", "condition": "classification == 'complex'"},
    {"from": "classifier", "to": "route_escalate", "type": "conditional", "condition": "classification == 'escalate'"}
  ]
}
```

## Troubleshooting

### Common Issues

#### API Key Not Set
```bash
Error: Authentication failed
```
**Solution**: Set your API key environment variable
```bash
export OPENAI_API_KEY="your-key-here"
```

#### Ollama Not Running
```bash
Error: Connection refused
```
**Solution**: Start Ollama server
```bash
ollama serve
```

#### Model Not Found
```bash
Error: Model not available
```
**Solution**: Pull the required model
```bash
ollama pull llama3.1
```

#### Timeout Errors
```bash
Error: Request timeout
```
**Solution**: Increase timeout in configuration
```json
{"timeout_seconds": 300}
```

### Debug Tips

1. **Start simple**: Begin with single-node workflows
2. **Use verbose mode**: Add `--verbose` flag for detailed logs
3. **Check node outputs**: Use transform nodes to inspect intermediate results
4. **Validate incrementally**: Add nodes one at a time and test
5. **Monitor resources**: Watch CPU/memory usage for large workflows

## Contributing Examples

We welcome contributions of new examples! Please:

1. **Follow naming conventions**: Use descriptive, kebab-case names
2. **Add documentation**: Include clear descriptions and use cases
3. **Test thoroughly**: Ensure examples work with different providers
4. **Add variety**: Cover different domains and complexity levels
5. **Include configs**: Provide appropriate configuration files

### Example Submission Template

```json
{
  "name": "Your Example Name",
  "description": "Detailed description of what this example demonstrates",
  "use_case": "Real-world scenario this solves",
  "complexity": "basic|intermediate|advanced",
  "estimated_runtime": "30 seconds - 2 minutes",
  "required_models": ["gpt-4", "llama3.1"],
  "dependencies": ["ollama", "internet_access"],
  "nodes": [...],
  "edges": [...]
}
```

---

For more examples and patterns, visit our [GitHub repository](https://github.com/InfinitiBit/graphbit) or join our [Discord community](https://discord.gg/graphbit). 
