# Dynamic Workflow Creation

GraphBit supports dynamic workflow creation, allowing you to build and modify workflows at runtime based on data, conditions, and business logic. This powerful feature enables adaptive workflows that can respond to changing requirements.

## Overview

Dynamic workflow creation allows you to:
- Create workflows that adapt to input data
- Generate nodes and connections programmatically
- Modify workflow structure based on runtime conditions
- Build self-organizing processing pipelines
- Implement conditional workflow branches

## Basic Dynamic Workflow Creation

### Simple Dynamic Workflow

```python
from graphbit import Workflow, Node

def create_dynamic_workflow(input_data):
    """Creates a workflow dynamically based on input data."""
    
    # Analyze input to determine workflow structure
    data_type = detect_data_type(input_data)
    
    if data_type == "text":
        return create_text_processing_workflow()
    elif data_type == "numerical":
        return create_numerical_analysis_workflow()
    elif data_type == "mixed":
        return create_mixed_data_workflow()
    else:
        return create_generic_workflow()

def detect_data_type(data):
    """Detect the type of input data."""
    if isinstance(data, str):
        return "text"
    elif isinstance(data, (int, float)):
        return "numerical"
    elif isinstance(data, dict) or isinstance(data, list):
        return "mixed"
    else:
        return "unknown"

def create_text_processing_workflow():
    """Create workflow optimized for text processing."""
    
    workflow = Workflow("Text Processing Workflow")
    
    # Text analyzer
    analyzer = Node.agent(
        name="Text Analyzer",
        prompt=f"Analyze this text: {input}",
        agent_id="text_analyzer"
    )
    
    # Sentiment detector
    sentiment = Node.agent(
        name="Sentiment Detector",
        prompt="Determine sentiment of the analyzed data.",
        agent_id="sentiment_detector"
    )
    
    # Build text processing chain
    analyzer_id = workflow.add_node(analyzer)
    sentiment_id = workflow.add_node(sentiment)
    
    workflow.connect(analyzer_id, sentiment_id)
    
    return workflow

def create_numerical_analysis_workflow():
    """Create workflow optimized for numerical analysis."""
    
    workflow = Workflow("Numerical Analysis Workflow")
    
    # Statistical analyzer
    stats = Node.agent(
        name="Statistical Analyzer",
        prompt=f"Perform statistical analysis on: {input}",
        agent_id="stats_analyzer"
    )
    
    # Trend detector
    trends = Node.agent(
        name="Trend Detector",
        prompt="Identify trends in stats analyzed.",
        agent_id="trend_detector"
    )
    
    # Build numerical analysis chain
    stats_id = workflow.add_node(stats)
    trends_id = workflow.add_node(trends)
    
    workflow.connect(stats_id, trends_id)
    
    return workflow

def create_mixed_data_workflow():
    """Create workflow for mixed data types."""
    
    workflow = Workflow("Mixed Data Workflow")
    
    # Data classifier
    classifier = Node.agent(
        name="Data Classifier",
        prompt=f"Classify this mixed data: {input}",
        agent_id="classifier"
    )
    
    # Multi-modal processor
    processor = Node.agent(
        name="Multi-Modal Processor",
        prompt="Process classified data.",
        agent_id="multimodal_processor"
    )
    
    # Build mixed data chain
    classifier_id = workflow.add_node(classifier)
    processor_id = workflow.add_node(processor)
    
    workflow.connect(classifier_id, processor_id)
    
    return workflow

def create_generic_workflow():
    """Create generic workflow for unknown data types."""
    
    workflow = Workflow("Generic Workflow")
    
    # Generic processor
    processor = Node.agent(
        name="Generic Processor",
        prompt=f"Process this input: {input}",
        agent_id="generic_processor"
    )
    
    workflow.add_node(processor)
    
    return workflow
```

## Advanced Dynamic Generation

### Data-Driven Node Creation

```python
from graphbit import Workflow, Node

def create_data_driven_workflow(schema):
    """Create workflow based on data schema."""
    
    workflow = Workflow("Data-Driven Workflow")
    
    node_ids = []
    
    # Create nodes based on schema fields
    for field in schema.get("fields", []):
        field_type = field.get("type")
        field_name = field.get("name")
        
        if field_type == "string":
            node = create_text_processing_node(field_name)
        elif field_type == "number":
            node = create_numerical_processing_node(field_name)
        elif field_type == "date":
            node = create_date_processing_node(field_name)
        else:
            node = create_generic_processing_node(field_name)
        
        node_id = workflow.add_node(node)
        node_ids.append((field_name, node_id))
    
    # Create aggregator node
    aggregator = Node.agent(
        name="Data Aggregator",
        prompt="Combine and analyze these processed fields",
        agent_id="aggregator"
    )
    
    agg_id = workflow.add_node(aggregator)
    
    # Connect all field processors to aggregator
    for field_name, node_id in node_ids:
        workflow.connect(node_id, agg_id)
    
    return workflow

def create_text_processing_node(field_name):
    """Create node for text field processing."""
    return Node.agent(
        name=f"{field_name} Text Processor",
        prompt=f"Process {field_name} text field: {{{field_name}_input}}",
        agent_id=f"{field_name}_text_processor"
    )

def create_numerical_processing_node(field_name):
    """Create node for numerical field processing."""
    return Node.agent(
        name=f"{field_name} Numerical Processor",
        prompt=f"Analyze {field_name} numerical data: {{{field_name}_input}}",
        agent_id=f"{field_name}_num_processor"
    )

def create_date_processing_node(field_name):
    """Create node for date field processing."""
    return Node.agent(
        name=f"{field_name} Date Processor",
        prompt=f"Analyze {field_name} date patterns: {{{field_name}_input}}",
        agent_id=f"{field_name}_date_processor"
    )

def create_generic_processing_node(field_name):
    """Create generic processing node."""
    return Node.agent(
        name=f"{field_name} Generic Processor",
        prompt=f"Process {field_name} field: {{{field_name}_input}}",
        agent_id=f"{field_name}_generic_processor"
    )
```

## Adaptive Workflow Patterns

### Self-Optimizing Workflows

```python
from graphbit import Workflow, Node

class AdaptiveWorkflow:
    """Workflow that adapts based on execution history."""
    
    def __init__(self, name):
        self.name = name
        self.workflow = Workflow(name)
        self.execution_history = []
        self.performance_metrics = {}
        self.optimization_rules = []
    
    def add_optimization_rule(self, condition, action):
        """Add optimization rule."""
        self.optimization_rules.append({
            "condition": condition,
            "action": action
        })
    
    def execute_and_adapt(self, executor, input_data):
        """Execute workflow and adapt based on results."""
        
        import time
        
        # Record execution start
        start_time = time.time()
        
        # Execute workflow
        result = executor.execute(self.workflow)
        
        # Record execution metrics
        execution_time = (time.time() - start_time) * 1000
        
        execution_record = {
            "timestamp": time.time(),
            "execution_time_ms": execution_time,
            "success": result.is_completed(),
            "input_size": len(str(input_data)) if input_data else 0,
            "output_size": len(result.output()) if result.is_completed() else 0
        }
        
        self.execution_history.append(execution_record)
        
        # Update performance metrics
        self._update_performance_metrics()
        
        # Apply optimization rules
        self._apply_optimizations()
        
        return result
    
    def _update_performance_metrics(self):
        """Update performance metrics based on execution history."""
        
        if not self.execution_history:
            return
        
        recent_executions = self.execution_history[-10:]  # Last 10 executions
        
        self.performance_metrics = {
            "average_execution_time": sum(e["execution_time_ms"] for e in recent_executions) / len(recent_executions),
            "success_rate": sum(1 for e in recent_executions if e["success"]) / len(recent_executions),
            "total_executions": len(self.execution_history),
            "throughput": len(recent_executions) / (recent_executions[-1]["timestamp"] - recent_executions[0]["timestamp"]) if len(recent_executions) > 1 else 0
        }
    
    def _apply_optimizations(self):
        """Apply optimization rules based on current metrics."""
        
        for rule in self.optimization_rules:
            if self._evaluate_condition(rule["condition"]):
                self._execute_action(rule["action"])
    
    def _evaluate_condition(self, condition):
        """Evaluate optimization condition."""
        
        metrics = self.performance_metrics
        
        if condition["type"] == "performance_threshold":
            metric_value = metrics.get(condition["metric"], 0)
            return self._compare_values(metric_value, condition["operator"], condition["threshold"])
        
        elif condition["type"] == "execution_count":
            return metrics.get("total_executions", 0) >= condition["count"]
        
        return False
    
    def _compare_values(self, value, operator, threshold):
        """Compare values based on operator."""
        
        if operator == ">":
            return value > threshold
        elif operator == "<":
            return value < threshold
        elif operator == ">=":
            return value >= threshold
        elif operator == "<=":
            return value <= threshold
        elif operator == "==":
            return value == threshold
        
        return False
    
    def _execute_action(self, action):
        """Execute optimization action."""
        
        if action["type"] == "add_caching_layer":
            self._add_caching_layer()
        elif action["type"] == "add_parallel_processing":
            self._add_parallel_processing()
        elif action["type"] == "optimize_prompts":
            self._optimize_prompts()
    
    def _add_caching_layer(self):
        """Add caching layer to workflow."""
        
        cache_node = Node.agent(
            name="Cache Manager",
            prompt=f"Check cache for input: {input}. If found, return cached result, otherwise process normally.",
            agent_id="cache_manager"
        )
        
        # Insert cache node at the beginning
        cache_id = self.workflow.add_node(cache_node)
        print(f"Added caching layer to workflow {self.name}")
    
    def _add_parallel_processing(self):
        """Add parallel processing capability."""
        
        # Create parallel branch
        parallel_processor = Node.agent(
            name="Parallel Processor",
            prompt=f"Process input in parallel: {input}",
            agent_id="parallel_proc"
        )
        
        parallel_id = self.workflow.add_node(parallel_processor)
        print(f"Added parallel processing to workflow {self.name}")
    
    def _optimize_prompts(self):
        """Optimize prompts for better performance."""
        
        # This would involve modifying existing nodes with optimized prompts
        print(f"Optimized prompts for workflow {self.name}")

def create_adaptive_text_processor():
    """Create an adaptive text processing workflow."""
    
    adaptive_workflow = AdaptiveWorkflow("Adaptive Text Processor")
    
    # Build initial workflow
    processor = Node.agent(
        name="Text Processor",
        prompt=f"Process and analyze this text: {input}",
        agent_id="text_proc"
    )
    
    adaptive_workflow.workflow.add_node(processor)
    
    # Add optimization rules
    adaptive_workflow.add_optimization_rule(
        condition={
            "type": "performance_threshold",
            "metric": "average_execution_time",
            "operator": ">",
            "threshold": 5000  # 5 seconds
        },
        action={
            "type": "add_caching_layer"
        }
    )
    
    adaptive_workflow.add_optimization_rule(
        condition={
            "type": "execution_count",
            "count": 10
        },
        action={
            "type": "optimize_prompts"
        }
    )
    
    return adaptive_workflow
```

## Dynamic Workflow Templates

### Template-Based Generation

```python
from graphbit import Workflow, Node

class WorkflowTemplate:
    """Template for generating similar workflows."""
    
    def __init__(self, template_name):
        self.template_name = template_name
        self.template_structure = {}
        self.parameter_mappings = {}
    
    def define_template(self, structure, parameter_mappings):
        """Define workflow template structure."""
        self.template_structure = structure
        self.parameter_mappings = parameter_mappings
    
    def instantiate(self, parameters):
        """Create workflow instance from template."""
        
        workflow = Workflow(f"{self.template_name}_{parameters.get('instance_id', 'default')}")
        
        node_map = {}
        
        # Create nodes from template
        for node_config in self.template_structure.get("nodes", []):
            node = self._create_node_from_template(node_config, parameters)
            node_id = workflow.add_node(node)
            node_map[node_config["id"]] = node_id
        
        # Create connections from template
        for connection in self.template_structure.get("connections", []):
            source_id = node_map.get(connection["source"])
            target_id = node_map.get(connection["target"])
            
            if source_id and target_id:
                workflow.connect(source_id, target_id)
        
        return workflow
    
    def _create_node_from_template(self, node_config, parameters):
        """Create node from template configuration."""
        
        node_type = node_config.get("type")
        
        if node_type == "agent":
            # Replace template parameters in prompt
            prompt = node_config.get("prompt", "")
            for param, value in parameters.items():
                prompt = prompt.replace(f"${{{param}}}", str(value))
            
            return Node.agent(
                name=node_config.get("name", "Agent"),
                prompt=prompt,
                agent_id=node_config.get("agent_id", "agent")
            )

def create_data_processing_template():
    """Create a template for data processing workflows."""
    
    template = WorkflowTemplate("Data Processing Template")
    
    template_structure = {
        "nodes": [
            {
                "id": "validator",
                "type": "agent",
                "name": f"{domain} Data Validator",
                "prompt": f"Validate {domain} data according to {validation_rules}: {input}",
                "agent_id": "validator"
            },
            {
                "id": "processor",
                "type": "agent", 
                "name": f"{domain} Processor",
                "prompt": f"Process validated data using {processing_method}.",
                "agent_id": "processor"
            },
                        {
                "id": "quality_check",
                "type": "agent",
                "name": "Quality Gate",
                "prompt": f"Verify and return the quality of the data using quality_score >= {quality_threshold}."
            },
            {
                "id": "formatter",
                "type": "agent",
                "name": "Output Formatter",
                "prompt": f"Generate the output according to the output format: {output_format}"
            }
        ],
        "connections": [
            {"source": "validator", "target": "processor"},
            {"source": "processor", "target": "quality_check"},
            {"source": "quality_check", "target": "formatter"}
        ]
    }
    
    parameter_mappings = {
        "domain": "Application domain (e.g., financial, medical, scientific)",
        "validation_rules": "Specific validation rules for the domain",
        "processing_method": "Method used for processing data",
        "quality_threshold": "Minimum quality score threshold",
        "output_format": "Format for output transformation"
    }
    
    template.define_template(template_structure, parameter_mappings)
    
    return template

def create_workflows_from_template():
    """Create multiple workflows from template."""
    
    template = create_data_processing_template()
    
    # Financial data processing workflow
    financial_workflow = template.instantiate({
        "instance_id": "financial",
        "domain": "financial",
        "validation_rules": "GAAP compliance and data integrity checks",
        "processing_method": "financial analysis algorithms",
        "quality_threshold": "0.95",
        "output_format": "uppercase"
    })
    
    # Medical data processing workflow
    medical_workflow = template.instantiate({
        "instance_id": "medical",
        "domain": "medical",
        "validation_rules": "HIPAA compliance and medical data standards",
        "processing_method": "clinical analysis procedures",
        "quality_threshold": "0.98",
        "output_format": "lowercase"
    })
    
    return {
        "financial": financial_workflow,
        "medical": medical_workflow
    }
```

## Configuration-Driven Workflows

### JSON-Based Workflow Definition

```python
import json

from graphbit import Workflow, Node

def create_workflow_from_json(json_config):
    """Create workflow from JSON configuration."""
    
    if isinstance(json_config, str):
        config = json.loads(json_config)
    else:
        config = json_config
    
    workflow = Workflow(config.get("name", "JSON Workflow"))
    
    node_map = {}
    
    # Create nodes from configuration
    for node_config in config.get("nodes", []):
        node = _create_node_from_json(node_config)
        node_id = workflow.add_node(node)
        node_map[node_config["id"]] = node_id
    
    # Create connections from configuration
    for connection in config.get("connections", []):
        source_id = node_map.get(connection["source"])
        target_id = node_map.get(connection["target"])
        
        if source_id and target_id:
            workflow.connect(source_id, target_id)
    
    return workflow

def _create_node_from_json(node_config):
    """Create node from JSON configuration."""
    
    node_type = node_config.get("type")
    
    if node_type == "agent":
        return Node.agent(
            name=node_config.get("name", "Agent"),
            prompt=node_config.get("prompt", f"Process input: {input}"),
            agent_id=node_config.get("agent_id", "agent")
        )

# Example JSON configurations
def get_example_workflow_configs():
    """Get example workflow configurations."""
    
    simple_config = {
        "name": "Simple Analysis Workflow",
        "nodes": [
            {
                "id": "analyzer",
                "type": "agent",
                "name": "Data Analyzer",
                "prompt": "Analyze this data: {input}",
                "agent_id": "analyzer"
            },
            {
                "id": "formatter",
                "type": "agent",
                "name": "Output Formatter",
                "prompt": "Turn the analysis in uppercase."
            }
        ],
        "connections": [
            {"source": "analyzer", "target": "formatter"}
        ]
    }
    
    complex_config = {
        "name": "Complex Processing Workflow",
        "nodes": [
            {
                "id": "input_processor",
                "type": "agent",
                "name": "Input Processor",
                "prompt": "Process and prepare input: {input}",
                "agent_id": "input_proc"
            },
            {
                "id": "quality_check",
                "type": "agent",
                "name": "Quality Gate",
                "prompt": "Verify and return the quality of the data using quality_score > 0.8."
            },
            {
                "id": "high_quality_processor",
                "type": "agent",
                "name": "High Quality Processor",
                "prompt": "Process high-quality data.",
                "agent_id": "hq_proc"
            },
            {
                "id": "enhancement_processor",
                "type": "agent",
                "name": "Enhancement Processor",
                "prompt": "Enhance and process lower-quality data.",
                "agent_id": "enhancement_proc"
            },
            {
                "id": "aggregator",
                "type": "agent",
                "name": "Result Aggregator",
                "prompt": "Combine processing results.",
                "agent_id": "aggregator"
            }
        ],
        "connections": [
            {"source": "input_processor", "target": "quality_check"},
            {"source": "quality_check", "target": "high_quality_processor"},
            {"source": "quality_check", "target": "enhancement_processor"},
            {"source": "high_quality_processor", "target": "aggregator"},
            {"source": "enhancement_processor", "target": "aggregator"}
        ]
    }
    
    return {
        "simple": simple_config,
        "complex": complex_config
    }
```

## Best Practices

### 1. Dynamic Workflow Design Principles

```python
def get_dynamic_workflow_best_practices():
    """Get best practices for dynamic workflow creation."""
    
    best_practices = {
        "modularity": "Design workflows with modular, reusable components",
        "parameterization": "Use parameters and templates for flexibility",
        "validation": "Always validate dynamically created workflows",
        "performance": "Monitor and optimize dynamic workflow performance",
        "maintainability": "Keep dynamic generation logic simple and readable",
        "error_handling": "Implement robust error handling for dynamic creation",
        "testing": "Thoroughly test dynamic workflows with various inputs"
    }
    
    for practice, description in best_practices.items():
        print(f"✅ {practice.title()}: {description}")
    
    return best_practices
```

### 2. Error Handling and Validation

```python
def validate_dynamic_workflow(workflow):
    """Validate dynamically created workflow."""
    
    try:
        # Basic validation
        workflow.validate()
        print("✅ Dynamic workflow validation passed")
        return True
        
    except Exception as e:
        print(f"❌ Dynamic workflow validation failed: {e}")
        return False

def safe_dynamic_workflow_creation(creation_func, *args, **kwargs):
    """Safely create dynamic workflow with error handling."""
    
    try:
        workflow = creation_func(*args, **kwargs)
        
        if validate_dynamic_workflow(workflow):
            return workflow
        else:
            raise ValueError("Dynamic workflow validation failed")
            
    except Exception as e:
        print(f"Error creating dynamic workflow: {e}")
        
        # Return a simple fallback workflow
        fallback_workflow = Workflow("Fallback Workflow")
        fallback_node = Node.agent(
            name="Fallback Processor",
            prompt=f"Process input safely: {input}",
            agent_id="fallback"
        )
        fallback_workflow.add_node(fallback_node)
        
        return fallback_workflow
```

## Usage Examples

### Complete Dynamic Workflow Example

```python
from graphbit import LlmConfig, Executor

def example_complete_dynamic_workflow():
    """Complete example of dynamic workflow creation and execution."""
    
    # Create dynamic workflow based on input
    input_data = {
        "type": "mixed",
        "content": "Sample text with numerical data: 123, 456",
        "requirements": ["quality_check", "fast_processing"]
    }
    
    # Create workflow dynamically
    workflow = create_dynamic_workflow(input_data)
    
    # Validate the workflow
    if validate_dynamic_workflow(workflow):
        print("✅ Dynamic workflow created and validated successfully")
        
        # Create executor
        llm_config = LlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
        executor = Executor(llm_config)
        
        # Execute workflow
        result = executor.execute(workflow)
        
        if result.is_completed():
            print(f"✅ Dynamic workflow executed successfully")
            print(f"Output: {result.output()}")
        else:
            print(f"❌ Dynamic workflow execution failed: {result.error()}")
    
    else:
        print("❌ Dynamic workflow validation failed")

if __name__ == "__main__":
    example_complete_dynamic_workflow()
```

## What's Next

- Learn about [Performance](performance.md) optimization for dynamic workflows
- Explore [Monitoring](monitoring.md) for tracking dynamic workflow execution
- Check [Validation](validation.md) for comprehensive dynamic workflow testing
- See [Workflow Builder](workflow-builder.md) for static workflow patterns 
