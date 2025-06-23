# GraphBit Python API

Python bindings for GraphBit - a declarative agentic workflow automation framework.

## Installation

### From PyPI (Recommended)

```bash
pip install graphbit
```

### From Source

```bash
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/
cargo build --release
cd python/
cargo build --release
maturin develop --release
```

## Quick Start

```python
import graphbit
import os

graphbit.init()

api_key = os.getenv("OPENAI_API_KEY")
config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")

builder = graphbit.PyWorkflowBuilder("Hello World")
node = graphbit.PyWorkflowNode.agent_node(
    "Greeter",
    "Generates greetings",
    "greeter",
    "Say hello to {name}"
)

node_id = builder.add_node(node)
workflow = builder.build()

executor = graphbit.PyWorkflowExecutor(config)
context = executor.execute(workflow)
print(f"Result: {context.get_variable('output')}")
```

## Core Classes

### PyLlmConfig

Configuration for LLM providers.

```python
openai_config = graphbit.PyLlmConfig.openai(api_key, "gpt-4")

anthropic_config = graphbit.PyLlmConfig.anthropic(api_key, "claude-3-sonnet-20240229")

hf_config = graphbit.PyLlmConfig.huggingface(api_key, "microsoft/DialoGPT-medium")

print(f"Provider: {config.provider_name()}")
print(f"Model: {config.model_name()}")
```

### PyWorkflowBuilder

Builder pattern for creating workflows.

```python
builder = graphbit.PyWorkflowBuilder("My Workflow")
builder.description("A sample workflow for demonstration")

agent_node = graphbit.PyWorkflowNode.agent_node(
    "Analyzer", 
    "Analyzes input data",
    "analyzer",
    "Analyze: {input}"
)

node_id = builder.add_node(agent_node)

transform_node = graphbit.PyWorkflowNode.transform_node(
    "Formatter",
    "Formats the output", 
    "uppercase"
)

transform_id = builder.add_node(transform_node)

edge = graphbit.PyWorkflowEdge.data_flow()
builder.connect(node_id, transform_id, edge)

final_workflow = builder.build()
```

### PyEmbeddingService

Service for generating text embeddings.

```python
config = graphbit.PyEmbeddingConfig.openai(api_key, "text-embedding-ada-002")

config = graphbit.PyEmbeddingConfig.huggingface(api_key, "sentence-transformers/all-MiniLM-L6-v2")

config = config.with_timeout(30).with_max_batch_size(100)

embedding_service = graphbit.PyEmbeddingService(config)

import asyncio

async def generate_embeddings():
    """ Generating embeddings with embeddings model """
    embedding = await embedding_service.embed_text("Hello world")
    print(f"Embedding dimensions: {len(embedding)}")
    
    texts = ["First text", "Second text", "Third text"]
    embeddings = await embedding_service.embed_texts(texts)
    print(f"Generated {len(embeddings)} embeddings")

    similarity = graphbit.PyEmbeddingService.cosine_similarity(embeddings[0], embeddings[1])
    print(f"Similarity: {similarity}")

asyncio.run(generate_embeddings())
```

### PyWorkflowNode

Factory for creating different types of workflow nodes.

#### Agent Nodes

```python
# Basic agent node
agent = graphbit.PyWorkflowNode.agent_node(
    name="Content Writer",
    description="Writes articles",
    agent_id="writer",
    prompt="Write an article about {topic}"
)

agent = graphbit.PyWorkflowNode.agent_node_with_config(
    name="Technical Writer",
    description="Writes technical documentation",
    agent_id="tech_writer", 
    prompt="Write technical docs for {feature}",
    max_tokens=2000,
    temperature=0.3
)
```

#### Condition Nodes

```python

condition = graphbit.PyWorkflowNode.condition_node(
    name="Quality Check",
    description="Validates content quality",
    expression="quality_score > 0.8"
)

condition = graphbit.PyWorkflowNode.condition_node(
    name="Content Filter",
    description="Filters inappropriate content",
    expression="toxicity_score < 0.1 && sentiment != 'negative'"
)
```

#### Transform Nodes

```python

uppercase_transform = graphbit.PyWorkflowNode.transform_node(
    name="Uppercase",
    description="Converts text to uppercase",
    transformation="uppercase"
)

json_extract = graphbit.PyWorkflowNode.transform_node(
    name="Extract JSON",
    description="Extracts JSON from response",
    transformation="json_extract",
    field="content"
)

# Available transformations: "uppercase", "lowercase", "json_extract", "split", "join"
```

#### Delay Nodes

```python
delay = graphbit.PyWorkflowNode.delay_node(
    name="Wait Period",
    description="Waits before next step",
    duration_seconds=30
)
```

#### Document Loader Nodes

```python


pdf_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="PDF Loader",
    description="Loads PDF documents",
    document_type="pdf",
    source_path="/path/to/document.pdf"
)

text_loader = graphbit.PyWorkflowNode.document_loader_node(
    name="Text Loader", 
    description="Loads text files",
    document_type="text",
    source_path="/path/to/document.txt"
)

# Supported document types: "pdf", "text", "markdown", "html"
```

#### Agent Capabilities

```python

text_capability = graphbit.PyAgentCapability.text_processing()
data_capability = graphbit.PyAgentCapability.data_analysis()
tool_capability = graphbit.PyAgentCapability.tool_execution()
decision_capability = graphbit.PyAgentCapability.decision_making()
custom_capability = graphbit.PyAgentCapability.custom("custom_skill")

print(f"Capability: {text_capability}")
```

#### Workflow Edges

```python

data_edge = graphbit.PyWorkflowEdge.data_flow()
control_edge = graphbit.PyWorkflowEdge.control_flow()
conditional_edge = graphbit.PyWorkflowEdge.conditional("status == 'success'")
```

### PyWorkflowExecutor

Executes workflows with the configured LLM.

```python

executor = graphbit.PyWorkflowExecutor(config)


high_throughput_executor = graphbit.PyWorkflowExecutor.new_high_throughput(config)
low_latency_executor = graphbit.PyWorkflowExecutor.new_low_latency(config)
memory_optimized_executor = graphbit.PyWorkflowExecutor.new_memory_optimized(config)

retry_config = graphbit.PyRetryConfig(max_attempts=3)
retry_config = retry_config.with_exponential_backoff(
    initial_delay_ms=1000,
    multiplier=2.0,
    max_delay_ms=30000
).with_jitter(0.1)


circuit_breaker = graphbit.PyCircuitBreakerConfig(
    failure_threshold=5,
    recovery_timeout_ms=60000
)

advanced_executor = executor.with_retry_config(retry_config) \
                           .with_circuit_breaker_config(circuit_breaker) \
                           .with_max_node_execution_time(30000) \
                           .with_fail_fast(True)


context = executor.execute(workflow)
print(f"Workflow ID: {context.workflow_id()}")
print(f"Status: {context.state()}")
print(f"Completed: {context.is_completed()}")
print(f"Execution time: {context.execution_time_ms()}ms")


import asyncio

async def run_workflow():
    context = await executor.execute_async(workflow)
    return context

context = asyncio.run(run_workflow())

workflows = [workflow1, workflow2, workflow3]
contexts = executor.execute_concurrent(workflows)



```

### PyWorkflow

Represents a complete workflow definition.

```python

print(f"Name: {workflow.name()}")
print(f"Description: {workflow.description()}")
print(f"Node count: {workflow.node_count()}")
print(f"Edge count: {workflow.edge_count()}")


validation_result = workflow.validate()
if validation_result.is_valid():
    print("Workflow is valid!")
else:
    print(f"Validation errors: {validation_result.errors()}")

json_str = workflow.to_json()
workflow_from_json = graphbit.PyWorkflow.from_json(json_str)
```

### PyDynamicGraphManager

Advanced dynamic workflow generation capabilities.

```python

dynamic_config = graphbit.PyDynamicGraphConfig() \
    .with_max_auto_nodes(10) \
    .with_confidence_threshold(0.8) \
    .with_generation_temperature(0.3) \
    .with_max_generation_depth(5) \
    .with_completion_objectives(["efficiency", "accuracy"]) \
    .enable_validation()

# Create dynamic graph manager
manager = graphbit.PyDynamicGraphManager(llm_config, dynamic_config)
manager.initialize(llm_config, dynamic_config)

context = executor.execute(workflow)  # Get current context
node_response = manager.generate_node(
    workflow_id="workflow_123",
    context=context,
    objective="Create a data validation step",
    allowed_node_types=["condition", "transform"]
)

print(f"Generated node with confidence: {node_response.confidence()}")
print(f"Reasoning: {node_response.reasoning()}")
generated_node = node_response.get_node()

auto_completion = manager.auto_complete_workflow(
    workflow=workflow,
    context=context,
    objectives=["validate inputs", "generate summary"]
)

if auto_completion.success():
    print(f"Generated {auto_completion.node_count()} new nodes")
    print(f"Completion time: {auto_completion.completion_time_ms()}ms")
    
analytics = manager.get_analytics()
print(f"Total nodes generated: {analytics.total_nodes_generated()}")
print(f"Success rate: {analytics.success_rate():.2%}")
print(f"Average confidence: {analytics.avg_confidence():.2f}")
print(f"Cache hit rate: {analytics.cache_hit_rate():.2%}")
```

### PyWorkflowAutoCompletion

Specialized workflow auto-completion engine.

```python

auto_completion_engine = graphbit.PyWorkflowAutoCompletion(manager)
auto_completion_engine.initialize(manager)


configured_engine = auto_completion_engine \
    .with_max_iterations(5) \
    .with_timeout_ms(30000)

result = configured_engine.complete_workflow(
    workflow=workflow,
    context=context,
    objectives=["error handling", "logging", "metrics"]
)

print(f"Auto-completion result: {result}")
```

## Advanced Usage

### Multi-Agent Workflows

```python

import asyncio
import graphbit

async def research_workflow():
    builder = graphbit.PyWorkflowBuilder("Research Pipeline")
    
    researcher = graphbit.PyWorkflowNode.agent_node(
        "Researcher",
        "Conducts initial research",
        "researcher",
        "Research the topic: {topic}. Provide key facts and sources."
    )
    
    # Analysis agent
    analyst = graphbit.PyWorkflowNode.agent_node(
        "Analyst",
        "Analyzes research findings", 
        "analyst",
        "Analyze this research data: {research_data}. Identify patterns and insights."
    )
    
    quality_check = graphbit.PyWorkflowNode.condition_node(
        "Quality Gate",
        "Ensures research quality",
        "word_count > 100 && source_count > 2"
    )
    
    summarizer = graphbit.PyWorkflowNode.agent_node(
        "Summarizer",
        "Creates executive summary",
        "summarizer", 
        "Create a concise summary of: {analysis}"
    )
    
    researcher_id = builder.add_node(researcher)
    quality_id = builder.add_node(quality_check)
    analyst_id = builder.add_node(analyst)
    summary_id = builder.add_node(summarizer)
    
    data_edge = graphbit.PyWorkflowEdge.data_flow()
    conditional_edge = graphbit.PyWorkflowEdge.conditional("quality_score > 0.8")
    
    builder.connect(researcher_id, quality_id, data_edge)
    builder.connect(quality_id, analyst_id, conditional_edge)
    builder.connect(analyst_id, summary_id, data_edge)
    
    workflow = builder.build()
    
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
    executor = graphbit.PyWorkflowExecutor(config)
    
    context = await executor.execute_async(workflow)
    
    return context

context = asyncio.run(research_workflow())
print(f"Research complete: {context.get_variable('output')}")
```

### Workflow Context and Variables

```python

context = executor.execute(workflow)

print(f"Workflow ID: {context.workflow_id()}")
print(f"Current state: {context.state()}")
print(f"Is completed: {context.is_completed()}")
print(f"Is failed: {context.is_failed()}")
print(f"Execution time: {context.execution_time_ms()}ms")

output_value = context.get_variable("output")
if output_value:
    print(f"Output: {output_value}")

all_variables = context.variables()
for key, value in all_variables:
    print(f"{key}: {value}")
```

### Error Handling and Retry Logic

```python

retry_config = graphbit.PyRetryConfig(max_attempts=3)
retry_config = retry_config.with_exponential_backoff(
    initial_delay_ms=1000,
    multiplier=2.0,
    max_delay_ms=30000
).with_jitter(0.1)

circuit_breaker = graphbit.PyCircuitBreakerConfig(
    failure_threshold=5,
    recovery_timeout_ms=60000
)


robust_executor = executor.with_retry_config(retry_config) \
                         .with_circuit_breaker_config(circuit_breaker)

try:
    context = robust_executor.execute(workflow)
    if context.is_failed():
        print("Workflow execution failed")
    else:
        print("Workflow executed successfully")
except Exception as e:
    print(f"Execution error: {e}")
```

### Workflow Templates

```python
class WorkflowTemplates:
    @staticmethod
    def content_pipeline():
        """Creates a content generation pipeline."""
        builder = graphbit.PyWorkflowBuilder("Content Pipeline")
        
        researcher = graphbit.PyWorkflowNode.agent_node(
            "Content Researcher", 
            "Researches the topic",
            "researcher",
            "Research {topic} and gather relevant information"
        )
        
        writer = graphbit.PyWorkflowNode.agent_node(
            "Content Writer",
            "Writes the initial draft", 
            "writer",
            "Write a {content_type} about {topic} using this research: {research}"
        )
        
        editor = graphbit.PyWorkflowNode.agent_node(
            "Content Editor",
            "Edits and improves the content",
            "editor", 
            "Edit and improve this content: {content}"
        )
        
        r_id = builder.add_node(researcher)
        w_id = builder.add_node(writer)
        e_id = builder.add_node(editor)
        
        data_edge = graphbit.PyWorkflowEdge.data_flow()
        builder.connect(r_id, w_id, data_edge)
        builder.connect(w_id, e_id, data_edge)
        
        return builder.build()
    
    @staticmethod
    def data_analysis_pipeline():
        """Creates a data analysis pipeline."""
        builder = graphbit.PyWorkflowBuilder("Data Analysis")
        
        # Data extraction
        extractor = graphbit.PyWorkflowNode.agent_node(
            "Data Extractor",
            "Extracts data from input",
            "extractor",
            "Extract key data points from: {input}"
        )
        
        validator = graphbit.PyWorkflowNode.condition_node(
            "Data Validator",
            "Validates extracted data",
            "data_points > 5 && confidence > 0.8"
        )
        
        analyzer = graphbit.PyWorkflowNode.agent_node(
            "Data Analyzer", 
            "Analyzes the data",
            "analyzer",
            "Analyze this data and identify trends: {data}"
        )
        
        reporter = graphbit.PyWorkflowNode.agent_node(
            "Report Generator",
            "Generates analysis report",
            "reporter",
            "Generate a comprehensive report from this analysis: {analysis}"
        )
        
        ext_id = builder.add_node(extractor)
        val_id = builder.add_node(validator)
        ana_id = builder.add_node(analyzer)
        rep_id = builder.add_node(reporter)
        
        data_edge = graphbit.PyWorkflowEdge.data_flow()
        conditional_edge = graphbit.PyWorkflowEdge.conditional("data_points > 5 && confidence > 0.8")
        
        builder.connect(ext_id, val_id, data_edge)
        builder.connect(val_id, ana_id, conditional_edge)
        builder.connect(ana_id, rep_id, data_edge)
        
        return builder.build()

# Use templates
content_workflow = WorkflowTemplates.content_pipeline()
analysis_workflow = WorkflowTemplates.data_analysis_pipeline()
```

## Integration Examples

### FastAPI Integration

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import graphbit
import asyncio

app = FastAPI()

graphbit.init()

config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
executor = graphbit.PyWorkflowExecutor(config)

class WorkflowRequest(BaseModel):
    workflow_json: str
    input_data: dict

class WorkflowResponse(BaseModel):
    success: bool
    result: dict
    error: str = None

@app.post("/execute", response_model=WorkflowResponse)
async def execute_workflow(request: WorkflowRequest):
    try:
        workflow = graphbit.PyWorkflow.from_json(request.workflow_json)
        
        validation_result = workflow.validate()
        if not validation_result.is_valid():
            return WorkflowResponse(
                success=False, 
                result={}, 
                error=f"Validation errors: {validation_result.errors()}"
            )
        
        context = await executor.execute_async(workflow)
        
        if context.is_completed():
            return WorkflowResponse(success=True, result={"context": context.variables()})
        else:
            return WorkflowResponse(success=False, result={}, error="Workflow execution failed")
    
    except Exception as e:
        return WorkflowResponse(success=False, result={}, error=str(e))

@app.get("/health")
async def health_check():
    return {"status": "healthy"}
```

### Jupyter Notebook Integration

```python

import graphbit
import json
from IPython.display import display, JSON

def create_interactive_workflow():
    builder = graphbit.PyWorkflowBuilder("Interactive Analysis")
    
    topic = input("Enter topic to analyze: ")
    analysis_type = input("Analysis type (sentiment/summary/keywords): ")
    
    if analysis_type == "sentiment":
        prompt = f"Analyze the sentiment of: {{input}}"
    elif analysis_type == "summary":
        prompt = f"Summarize: {{input}}"
    else:
        prompt = f"Extract keywords from: {{input}}"
    
    agent = graphbit.PyWorkflowNode.agent_node(
        f"{analysis_type.title()} Analyzer",
        f"Performs {analysis_type} analysis",
        "analyzer",
        prompt
    )
    
    node_id = builder.add_node(agent)
    return builder.build(), topic

workflow, topic = create_interactive_workflow()

config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
executor = graphbit.PyWorkflowExecutor(config)

context = executor.execute(workflow)

result_data = {
    "workflow_id": context.workflow_id(),
    "status": context.state(),
    "execution_time_ms": context.execution_time_ms(),
    "variables": dict(context.variables())
}
display(JSON(result_data))
```

### Django Integration

```python

from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods
import json
import graphbit

graphbit.init()

@csrf_exempt
@require_http_methods(["POST"])
def execute_workflow_view(request):
    try:
        data = json.loads(request.body)
        
        workflow_type = data.get('workflow_type')
        input_data = data.get('input_data', {})
        
        if workflow_type == "content_generation":
            workflow = create_content_workflow()
        elif workflow_type == "data_analysis":
            workflow = create_analysis_workflow()
        else:
            return JsonResponse({"error": "Unknown workflow type"}, status=400)
        
        config = graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), 
            "gpt-4o-mini"
        )
        executor = graphbit.PyWorkflowExecutor(config)
        context = executor.execute(workflow)
        
        if context.is_completed():
            return JsonResponse({
                "success": True, 
                "result": dict(context.variables()),
                "execution_time_ms": context.execution_time_ms()
            })
        else:
            return JsonResponse({"success": False, "error": "Workflow execution failed"})
        
    except Exception as e:
        return JsonResponse({"error": str(e)}, status=500)

def create_content_workflow():
    # Implementation here
    pass
```

## Testing

### Unit Testing

```python
import unittest
import graphbit

class TestGraphBitWorkflows(unittest.TestCase):
    
    def setUp(self):
        graphbit.init()
        self.config = graphbit.PyLlmConfig.openai("test-key", "gpt-4")
        self.executor = graphbit.PyWorkflowExecutor(self.config)
    
    def test_workflow_creation(self):
        builder = graphbit.PyWorkflowBuilder("Test Workflow")
        node = graphbit.PyWorkflowNode.agent_node(
            "Test Agent", 
            "Test description",
            "test",
            "Test prompt: {input}"
        )
        
        node_id = builder.add_node(node)
        workflow = builder.build()
        
        self.assertEqual(workflow.name(), "Test Workflow")
        self.assertEqual(workflow.node_count(), 1)
    
    def test_workflow_validation(self):
        builder = graphbit.PyWorkflowBuilder("Valid Workflow")
        node = graphbit.PyWorkflowNode.agent_node(
            "Agent", "Description", "agent", "Prompt"
        )
        
        node_id = builder.add_node(node)
        workflow = builder.build()
        
        validation_result = workflow.validate()
        self.assertTrue(validation_result.is_valid())
    
    async def test_workflow_execution(self):
        builder = graphbit.PyWorkflowBuilder("Execution Test")
        node = graphbit.PyWorkflowNode.agent_node(
            "Echo Agent", "Echoes input", "echo", "Echo: {input}"
        )
        
        node_id = builder.add_node(node)
        workflow = builder.build()
        
        context = await self.executor.execute_async(workflow)
        self.assertIsNotNone(context)
        self.assertTrue(context.is_completed())

if __name__ == "__main__":
    unittest.main()
```

### Pytest Integration

```python
import pytest
import asyncio
import graphbit

@pytest.fixture
def test_config():
    graphbit.init()
    return graphbit.PyLlmConfig.openai("test-key", "gpt-4")

@pytest.fixture
def executor(test_config):
    return graphbit.PyWorkflowExecutor(test_config)

@pytest.fixture
def simple_workflow():
    builder = graphbit.PyWorkflowBuilder("Test")
    node = graphbit.PyWorkflowNode.agent_node(
        "Test", "Test", "test", "Test: {input}"
    )
    node_id = builder.add_node(node)
    return builder.build()

def test_workflow_creation(simple_workflow):
    assert simple_workflow.name() == "Test"
    assert simple_workflow.node_count() == 1

@pytest.mark.asyncio
async def test_workflow_execution(executor, simple_workflow):
    context = await executor.execute_async(simple_workflow)
    assert context is not None
    assert context.is_completed()

def test_workflow_serialization(simple_workflow):
    json_str = simple_workflow.to_json()
    recovered = graphbit.PyWorkflow.from_json(json_str)
    assert recovered.name() == simple_workflow.name()
```

## Performance Optimization

### Batch Processing

```python
import asyncio
import graphbit

async def batch_process(workflows, input_batches):
    """Process multiple workflows concurrently."""
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
    executor = graphbit.PyWorkflowExecutor(config)
    
    # Create tasks for concurrent execution
    tasks = []
    for workflow in workflows:
        task = executor.execute_async(workflow)
        tasks.append(task)
    
    # Execute all workflows concurrently
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # Process results
    successful_results = []
    errors = []
    
    for i, result in enumerate(results):
        if isinstance(result, Exception):
            errors.append(f"Workflow {i} failed: {result}")
        else:
            successful_results.append(result)
    
    return successful_results, errors

# Usage
workflows = [workflow1, workflow2, workflow3]

results, errors = asyncio.run(batch_process(workflows, []))
```

### Connection Pooling

```python
import graphbit

class WorkflowManager:
    def __init__(self, max_concurrent=5):
        self.semaphore = asyncio.Semaphore(max_concurrent)
        self.config = graphbit.PyLlmConfig.openai(
            os.getenv("OPENAI_API_KEY"), 
            "gpt-4"
        )
        self.executor = graphbit.PyWorkflowExecutor(self.config)
    
    async def execute_with_limit(self, workflow):
        """Execute workflow with concurrency limiting."""
        async with self.semaphore:
            return await self.executor.execute_async(workflow)
    
    async def process_queue(self, workflow_queue):
        """Process a queue of workflows with concurrency control."""
        tasks = []
        for workflow in workflow_queue:
            task = self.execute_with_limit(workflow)
            tasks.append(task)
        
        return await asyncio.gather(*tasks)

# Usage
manager = WorkflowManager(max_concurrent=3)
queue = [workflow1, workflow2, workflow3]
results = await manager.process_queue(queue)
```

## Error Handling

### Custom Exception Handling

```python
import graphbit

def safe_workflow_execution(workflow, max_retries=3):
    """Execute workflow with comprehensive error handling."""
    config = graphbit.PyLlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4")
    executor = graphbit.PyWorkflowExecutor(config)
    
    for attempt in range(max_retries):
        try:
            context = executor.execute(workflow)
            return {"success": True, "result": context, "attempts": attempt + 1}
        
        except graphbit.ValidationError as e:
            # Workflow validation failed - don't retry
            return {"success": False, "error": f"Validation error: {e}", "attempts": attempt + 1}
        
        except graphbit.LlmError as e:
            # LLM-specific error - might be temporary
            if attempt < max_retries - 1:
                print(f"LLM error on attempt {attempt + 1}: {e}. Retrying...")
                continue
            return {"success": False, "error": f"LLM error: {e}", "attempts": attempt + 1}
        
        except graphbit.WorkflowExecutionError as e:
            # Workflow execution error - might be recoverable
            if attempt < max_retries - 1:
                print(f"Execution error on attempt {attempt + 1}: {e}. Retrying...")
                continue
            return {"success": False, "error": f"Execution error: {e}", "attempts": attempt + 1}
        
        except Exception as e:
            # Unexpected error - don't retry
            return {"success": False, "error": f"Unexpected error: {e}", "attempts": attempt + 1}
    
    return {"success": False, "error": "Max retries exceeded", "attempts": max_retries}
```

## Best Practices

### Code Organization

```python

import graphbit
from typing import Dict, Any

class WorkflowLibrary:
    """Centralized workflow definitions."""
    
    @staticmethod
    def get_config(provider="openai"):
        """Get standardized configuration."""
        if provider == "openai":
            return graphbit.PyLlmConfig.openai(
                os.getenv("OPENAI_API_KEY"),
                "gpt-4"
            )
        elif provider == "anthropic":
            return graphbit.PyLlmConfig.anthropic(
                os.getenv("ANTHROPIC_API_KEY"),
                "claude-3-sonnet-20240229"
            )
        elif provider == "huggingface":
            return graphbit.PyLlmConfig.huggingface(
                os.getenv("HUGGINGFACE_API_KEY"),
                "microsoft/DialoGPT-medium"
            )
        else:
            raise ValueError(f"Unsupported provider: {provider}")

    @staticmethod
    def create_basic_workflow(name, description=""):
        """Create a basic workflow template."""
        builder = graphbit.PyWorkflowBuilder(name)
        if description:
            builder.description(description)
        return builder
    
    @staticmethod  
    def create_production_executor(config):
        """Create a production-ready executor with retry and circuit breaker."""
        retry_config = graphbit.PyRetryConfig(max_attempts=3) \
            .with_exponential_backoff(1000, 2.0, 30000) \
            .with_jitter(0.1)
            
        circuit_breaker = graphbit.PyCircuitBreakerConfig(
            failure_threshold=5,
            recovery_timeout_ms=60000
        )
        
        return graphbit.PyWorkflowExecutor(config) \
            .with_retry_config(retry_config) \
            .with_circuit_breaker_config(circuit_breaker) \
            .with_fail_fast(True)
```

## API Reference Summary

### Core Classes
- `PyLlmConfig`: LLM provider configuration
- `PyWorkflowBuilder`: Builder for creating workflows  
- `PyWorkflowNode`: Factory for workflow nodes
- `PyWorkflowEdge`: Factory for workflow edges
- `PyWorkflow`: Workflow definition and operations
- `PyWorkflowExecutor`: Workflow execution engine
- `PyWorkflowContext`: Execution context and results

### Embedding Classes
- `PyEmbeddingConfig`: Embedding service configuration
- `PyEmbeddingService`: Text embedding generation
- `PyEmbeddingRequest`: Embedding request wrapper
- `PyEmbeddingResponse`: Embedding response wrapper

### Dynamic Graph Classes
- `PyDynamicGraphConfig`: Dynamic graph configuration
- `PyDynamicGraphManager`: Dynamic node generation
- `PyWorkflowAutoCompletion`: Workflow auto-completion
- `PyDynamicNodeResponse`: Generated node response
- `PyAutoCompletionResult`: Auto-completion results

### Configuration Classes  
- `PyRetryConfig`: Retry behavior configuration
- `PyCircuitBreakerConfig`: Circuit breaker configuration
- `PyAgentCapability`: Agent capability definitions
- `PyValidationResult`: Workflow validation results

### Utility Functions
- `graphbit.init()`: Initialize the library
- `graphbit.version()`: Get version information

This comprehensive API provides everything needed to build sophisticated agentic workflows with Python, from simple automation to complex multi-agent systems with dynamic graph generation capabilities.
                "llama3.1",
                "http://localhost:11434"
            )
        else:
            raise ValueError(f"Unknown provider: {provider}")
    
    @staticmethod
    def create_content_workflow() -> graphbit.PyWorkflow:
        """Standard content generation workflow."""
        # Implementation here
        pass
    
    @staticmethod
    def create_analysis_workflow() -> graphbit.PyWorkflow:
        """Standard analysis workflow."""
        # Implementation here
        pass

# workflow_service.py
from .graphbit_workflows import WorkflowLibrary

class WorkflowService:
    """Service layer for workflow execution."""
    
    def __init__(self, provider="openai"):
        graphbit.init()
        self.config = WorkflowLibrary.get_config(provider)
        self.executor = graphbit.PyWorkflowExecutor(self.config)
    
    async def execute_content_generation(self, topic: str, content_type: str):
        """Execute content generation workflow."""
        workflow = WorkflowLibrary.create_content_workflow()
        return await self.executor.execute_async(workflow, {
            "topic": topic,
            "content_type": content_type
        })
```

### Environment Configuration

```python
# config.py
import os
from dataclasses import dataclass

@dataclass
class GraphBitSettings:
    openai_api_key: str = os.getenv("OPENAI_API_KEY")
    anthropic_api_key: str = os.getenv("ANTHROPIC_API_KEY")
    ollama_base_url: str = os.getenv("OLLAMA_BASE_URL", "http://localhost:11434")
    default_provider: str = os.getenv("GRAPHBIT_PROVIDER", "openai")
    default_model: str = os.getenv("GRAPHBIT_MODEL", "gpt-4")
    max_concurrent: int = int(os.getenv("GRAPHBIT_MAX_CONCURRENT", "5"))
    timeout_seconds: int = int(os.getenv("GRAPHBIT_TIMEOUT", "120"))
    
    def validate(self):
        """Validate configuration."""
        if self.default_provider == "openai" and not self.openai_api_key:
            raise ValueError("OPENAI_API_KEY required for OpenAI provider")
        if self.default_provider == "anthropic" and not self.anthropic_api_key:
            raise ValueError("ANTHROPIC_API_KEY required for Anthropic provider")

settings = GraphBitSettings()
settings.validate()
```

---
