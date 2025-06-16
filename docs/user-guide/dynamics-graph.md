# Dynamic Graph Generation

GraphBit supports dynamic graph generation, allowing workflows to create and modify their structure at runtime based on data, conditions, and business logic. This powerful feature enables adaptive workflows that can respond to changing requirements.

## Overview

Dynamic graph generation allows you to:
- Create workflows that adapt to input data
- Generate nodes and connections programmatically
- Modify workflow structure based on runtime conditions
- Build self-organizing processing pipelines
- Implement conditional workflow branches

## Basic Dynamic Graph Creation

### Simple Dynamic Workflow

```python
import graphbit
import json

def create_dynamic_workflow(input_data):
    """Creates a workflow dynamically based on input data."""
    
    graphbit.init()
    builder = graphbit.PyWorkflowBuilder("Dynamic Workflow")
    
    # Analyze input to determine workflow structure
    data_type = detect_data_type(input_data)
    
    if data_type == "text":
        return create_text_processing_workflow(builder)
    elif data_type == "numerical":
        return create_numerical_analysis_workflow(builder)
    elif data_type == "mixed":
        return create_mixed_data_workflow(builder)
    else:
        return create_generic_workflow(builder)

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

def create_text_processing_workflow(builder):
    """Create workflow optimized for text processing."""
    
    # Text analyzer
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Text Analyzer",
        description="Analyzes text content",
        agent_id="text_analyzer",
        prompt="Analyze this text: {input}"
    )
    
    # Sentiment detector
    sentiment = graphbit.PyWorkflowNode.agent_node(
        name="Sentiment Detector",
        description="Detects text sentiment",
        agent_id="sentiment_detector",
        prompt="Determine sentiment of: {analyzed_text}"
    )
    
    # Build text processing chain
    analyzer_id = builder.add_node(analyzer)
    sentiment_id = builder.add_node(sentiment)
    
    builder.connect(analyzer_id, sentiment_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def create_numerical_analysis_workflow(builder):
    """Create workflow optimized for numerical analysis."""
    
    # Statistical analyzer
    stats = graphbit.PyWorkflowNode.agent_node(
        name="Statistical Analyzer",
        description="Performs statistical analysis",
        agent_id="stats_analyzer",
        prompt="Perform statistical analysis on: {input}"
    )
    
    # Trend detector
    trends = graphbit.PyWorkflowNode.agent_node(
        name="Trend Detector",
        description="Detects trends in data",
        agent_id="trend_detector",
        prompt="Identify trends in: {stats_results}"
    )
    
    # Build numerical analysis chain
    stats_id = builder.add_node(stats)
    trends_id = builder.add_node(trends)
    
    builder.connect(stats_id, trends_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Advanced Dynamic Generation

### Data-Driven Node Creation

```python
def create_data_driven_workflow(schema):
    """Create workflow based on data schema."""
    
    builder = graphbit.PyWorkflowBuilder("Data-Driven Workflow")
    
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
        
        node_id = builder.add_node(node)
        node_ids.append((field_name, node_id))
    
    # Create aggregator node
    aggregator = graphbit.PyWorkflowNode.agent_node(
        name="Data Aggregator",
        description="Aggregates processed field data",
        agent_id="aggregator",
        prompt="Combine and analyze these processed fields: {all_results}"
    )
    
    agg_id = builder.add_node(aggregator)
    
    # Connect all field processors to aggregator
    for field_name, node_id in node_ids:
        builder.connect(node_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def create_text_processing_node(field_name):
    """Create node for text field processing."""
    return graphbit.PyWorkflowNode.agent_node(
        name=f"{field_name} Text Processor",
        description=f"Processes {field_name} text field",
        agent_id=f"{field_name}_text_processor",
        prompt=f"Process {field_name} text field: {{{field_name}_input}}"
    )

def create_numerical_processing_node(field_name):
    """Create node for numerical field processing."""
    return graphbit.PyWorkflowNode.agent_node(
        name=f"{field_name} Numerical Processor",
        description=f"Processes {field_name} numerical field",
        agent_id=f"{field_name}_num_processor",
        prompt=f"Analyze {field_name} numerical data: {{{field_name}_input}}"
    )

def create_date_processing_node(field_name):
    """Create node for date field processing."""
    return graphbit.PyWorkflowNode.agent_node(
        name=f"{field_name} Date Processor",
        description=f"Processes {field_name} date field",
        agent_id=f"{field_name}_date_processor",
        prompt=f"Analyze {field_name} date patterns: {{{field_name}_input}}"
    )

def create_generic_processing_node(field_name):
    """Create generic processing node."""
    return graphbit.PyWorkflowNode.agent_node(
        name=f"{field_name} Generic Processor",
        description=f"Processes {field_name} field generically",
        agent_id=f"{field_name}_generic_processor",
        prompt=f"Process {field_name} field: {{{field_name}_input}}"
    )
```

### Conditional Graph Generation

```python
def create_conditional_workflow(requirements):
    """Create workflow with conditional branches."""
    
    builder = graphbit.PyWorkflowBuilder("Conditional Workflow")
    
    # Input processor
    input_processor = graphbit.PyWorkflowNode.agent_node(
        name="Input Processor",
        description="Processes initial input",
        agent_id="input_processor",
        prompt="Process and categorize this input: {input}"
    )
    
    input_id = builder.add_node(input_processor)
    
    # Create conditional branches based on requirements
    branches = []
    
    if requirements.get("needs_validation"):
        validation_branch = create_validation_branch(builder)
        branches.append(("validation", validation_branch))
    
    if requirements.get("needs_analysis"):
        analysis_branch = create_analysis_branch(builder)
        branches.append(("analysis", analysis_branch))
    
    if requirements.get("needs_reporting"):
        reporting_branch = create_reporting_branch(builder)
        branches.append(("reporting", reporting_branch))
    
    # Connect conditional branches
    for branch_name, branch_nodes in branches:
        condition = graphbit.PyWorkflowNode.condition_node(
            name=f"{branch_name.title()} Condition",
            description=f"Determines if {branch_name} is needed",
            expression=f"needs_{branch_name} == true"
        )
        
        condition_id = builder.add_node(condition)
        builder.connect(input_id, condition_id, graphbit.PyWorkflowEdge.data_flow())
        
        # Connect to branch nodes
        for node_id in branch_nodes:
            builder.connect(condition_id, node_id, 
                          graphbit.PyWorkflowEdge.conditional(f"needs_{branch_name} == true"))
    
    return builder.build()

def create_validation_branch(builder):
    """Create validation processing branch."""
    
    validator = graphbit.PyWorkflowNode.agent_node(
        name="Data Validator",
        description="Validates data quality",
        agent_id="validator",
        prompt="Validate data quality: {input_data}"
    )
    
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        name="Quality Gate",
        description="Checks validation results",
        expression="validation_passed == true"
    )
    
    validator_id = builder.add_node(validator)
    gate_id = builder.add_node(quality_gate)
    
    builder.connect(validator_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    
    return [validator_id, gate_id]

def create_analysis_branch(builder):
    """Create analysis processing branch."""
    
    analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Data Analyzer",
        description="Performs data analysis",
        agent_id="analyzer",
        prompt="Analyze this data: {validated_data}"
    )
    
    pattern_detector = graphbit.PyWorkflowNode.agent_node(
        name="Pattern Detector",
        description="Detects patterns in analysis",
        agent_id="pattern_detector",
        prompt="Detect patterns in: {analysis_results}"
    )
    
    analyzer_id = builder.add_node(analyzer)
    pattern_id = builder.add_node(pattern_detector)
    
    builder.connect(analyzer_id, pattern_id, graphbit.PyWorkflowEdge.data_flow())
    
    return [analyzer_id, pattern_id]

def create_reporting_branch(builder):
    """Create reporting processing branch."""
    
    reporter = graphbit.PyWorkflowNode.agent_node(
        name="Report Generator",
        description="Generates reports",
        agent_id="reporter",
        prompt="Generate report from: {processed_data}"
    )
    
    formatter = graphbit.PyWorkflowNode.transform_node(
        name="Report Formatter",
        description="Formats reports",
        transformation="uppercase"
    )
    
    reporter_id = builder.add_node(reporter)
    formatter_id = builder.add_node(formatter)
    
    builder.connect(reporter_id, formatter_id, graphbit.PyWorkflowEdge.data_flow())
    
    return [reporter_id, formatter_id]
```

## Runtime Graph Modification

### Dynamic Node Addition

```python
class DynamicWorkflowManager:
    """Manages dynamic workflow modifications."""
    
    def __init__(self):
        self.base_workflow = None
        self.modifications = []
    
    def create_base_workflow(self):
        """Create the base workflow structure."""
        builder = graphbit.PyWorkflowBuilder("Dynamic Base Workflow")
        
        # Core processing node
        core_processor = graphbit.PyWorkflowNode.agent_node(
            name="Core Processor",
            description="Core data processing",
            agent_id="core_processor",
            prompt="Process core data: {input}"
        )
        
        builder.add_node(core_processor)
        self.base_workflow = builder.build()
        
        return self.base_workflow
    
    def add_processing_stage(self, stage_type, stage_config):
        """Add a new processing stage dynamically."""
        
        if not self.base_workflow:
            raise ValueError("Base workflow must be created first")
        
        # Create new builder with existing workflow
        builder = self.rebuild_workflow()
        
        if stage_type == "validation":
            self.add_validation_stage(builder, stage_config)
        elif stage_type == "enrichment":
            self.add_enrichment_stage(builder, stage_config)
        elif stage_type == "filtering":
            self.add_filtering_stage(builder, stage_config)
        
        self.base_workflow = builder.build()
        return self.base_workflow
    
    def rebuild_workflow(self):
        """Rebuild workflow from current state."""
        # This would typically involve serializing/deserializing
        # the current workflow state
        builder = graphbit.PyWorkflowBuilder("Rebuilt Dynamic Workflow")
        
        # Rebuild existing nodes and connections
        # (Implementation would depend on GraphBit's serialization capabilities)
        
        return builder
    
    def add_validation_stage(self, builder, config):
        """Add validation stage to workflow."""
        
        validator = graphbit.PyWorkflowNode.agent_node(
            name=f"Validator {len(self.modifications)}",
            description="Dynamic validation stage",
            agent_id=f"validator_{len(self.modifications)}",
            prompt=f"Validate according to rules: {config.get('rules', 'default')}"
        )
        
        validator_id = builder.add_node(validator)
        self.modifications.append(("validation", validator_id))
        
        return validator_id
    
    def add_enrichment_stage(self, builder, config):
        """Add enrichment stage to workflow."""
        
        enricher = graphbit.PyWorkflowNode.agent_node(
            name=f"Enricher {len(self.modifications)}",
            description="Dynamic enrichment stage",
            agent_id=f"enricher_{len(self.modifications)}",
            prompt=f"Enrich data with: {config.get('sources', 'external data')}"
        )
        
        enricher_id = builder.add_node(enricher)
        self.modifications.append(("enrichment", enricher_id))
        
        return enricher_id
    
    def add_filtering_stage(self, builder, config):
        """Add filtering stage to workflow."""
        
        filter_condition = graphbit.PyWorkflowNode.condition_node(
            name=f"Filter {len(self.modifications)}",
            description="Dynamic filtering stage",
            expression=config.get('condition', 'value > 0')
        )
        
        filter_id = builder.add_node(filter_condition)
        self.modifications.append(("filtering", filter_id))
        
        return filter_id
```

## Template-Based Generation

### Workflow Templates

```python
class WorkflowTemplate:
    """Base class for workflow templates."""
    
    def __init__(self, name, description):
        self.name = name
        self.description = description
    
    def generate(self, parameters):
        """Generate workflow from template."""
        raise NotImplementedError

class DataProcessingTemplate(WorkflowTemplate):
    """Template for data processing workflows."""
    
    def __init__(self):
        super().__init__("Data Processing", "Template for data processing workflows")
    
    def generate(self, parameters):
        """Generate data processing workflow."""
        
        builder = graphbit.PyWorkflowBuilder(f"Generated {self.name}")
        
        # Input stage
        if parameters.get("input_validation", True):
            validator = self._create_validator(parameters)
            builder.add_node(validator)
        
        # Processing stages
        processing_stages = parameters.get("processing_stages", ["analyze"])
        stage_nodes = []
        
        for stage in processing_stages:
            node = self._create_processing_node(stage, parameters)
            node_id = builder.add_node(node)
            stage_nodes.append(node_id)
        
        # Connect stages sequentially
        if len(stage_nodes) > 1:
            for i in range(len(stage_nodes) - 1):
                builder.connect(stage_nodes[i], stage_nodes[i + 1], 
                              graphbit.PyWorkflowEdge.data_flow())
        
        return builder.build()
    
    def _create_validator(self, parameters):
        """Create validator node."""
        return graphbit.PyWorkflowNode.agent_node(
            name="Template Validator",
            description="Validates input data",
            agent_id="template_validator",
            prompt=f"Validate data according to: {parameters.get('validation_rules', 'standard rules')}"
        )
```

## Practical Examples

### E-commerce Analysis Workflow

```python
def create_ecommerce_analysis_workflow(product_categories):
    """Create workflow for e-commerce analysis based on categories."""
    
    builder = graphbit.PyWorkflowBuilder("E-commerce Analysis")
    
    # Create category-specific analyzers
    category_analyzers = []
    
    for category in product_categories:
        analyzer = graphbit.PyWorkflowNode.agent_node(
            name=f"{category} Analyzer",
            description=f"Analyzes {category} products",
            agent_id=f"{category.lower()}_analyzer",
            prompt=f"Analyze {category} product data: {{{category.lower()}_data}}"
        )
        
        analyzer_id = builder.add_node(analyzer)
        category_analyzers.append((category, analyzer_id))
    
    # Market trends analyzer
    trends_analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Market Trends Analyzer",
        description="Analyzes overall market trends",
        agent_id="trends_analyzer",
        prompt="Analyze market trends across categories: {all_category_results}"
    )
    
    trends_id = builder.add_node(trends_analyzer)
    
    # Connect category analyzers to trends analyzer
    for category, analyzer_id in category_analyzers:
        builder.connect(analyzer_id, trends_id, graphbit.PyWorkflowEdge.data_flow())
    
    # Recommendation engine (if sufficient data)
    if len(product_categories) >= 3:
        recommender = graphbit.PyWorkflowNode.agent_node(
            name="Recommendation Engine",
            description="Generates product recommendations",
            agent_id="recommender",
            prompt="Generate recommendations based on: {trends_analysis}"
        )
        
        recommender_id = builder.add_node(recommender)
        builder.connect(trends_id, recommender_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

# Usage
categories = ["Electronics", "Clothing", "Home & Garden", "Books"]
workflow = create_ecommerce_analysis_workflow(categories)
```

### Document Processing Pipeline

```python
def create_document_processing_pipeline(document_types):
    """Create document processing pipeline based on document types."""
    
    builder = graphbit.PyWorkflowBuilder("Document Processing Pipeline")
    
    # Document classifier
    classifier = graphbit.PyWorkflowNode.agent_node(
        name="Document Classifier",
        description="Classifies incoming documents",
        agent_id="doc_classifier",
        prompt="Classify this document type: {input}"
    )
    
    classifier_id = builder.add_node(classifier)
    
    # Create type-specific processors
    for doc_type in document_types:
        # Type-specific processor
        processor = graphbit.PyWorkflowNode.agent_node(
            name=f"{doc_type} Processor",
            description=f"Processes {doc_type} documents",
            agent_id=f"{doc_type.lower()}_processor",
            prompt=f"Process {doc_type} document: {{classified_document}}"
        )
        
        # Routing condition
        condition = graphbit.PyWorkflowNode.condition_node(
            name=f"{doc_type} Route",
            description=f"Routes {doc_type} documents",
            expression=f"document_type == '{doc_type}'"
        )
        
        processor_id = builder.add_node(processor)
        condition_id = builder.add_node(condition)
        
        # Connect classifier -> condition -> processor
        builder.connect(classifier_id, condition_id, graphbit.PyWorkflowEdge.data_flow())
        builder.connect(condition_id, processor_id, 
                       graphbit.PyWorkflowEdge.conditional(f"document_type == '{doc_type}'"))
    
    return builder.build()

# Usage
doc_types = ["Invoice", "Contract", "Report", "Email"]
workflow = create_document_processing_pipeline(doc_types)
```

## Best Practices

### 1. Design for Flexibility

```python
def create_flexible_workflow(config):
    """Create workflow with flexible configuration."""
    
    builder = graphbit.PyWorkflowBuilder("Flexible Workflow")
    
    # Use configuration to drive structure
    stages = config.get("stages", [])
    parallel_processing = config.get("parallel", False)
    
    stage_nodes = []
    
    for stage_config in stages:
        node = create_stage_node(stage_config)
        node_id = builder.add_node(node)
        stage_nodes.append(node_id)
    
    # Connect based on configuration
    if not parallel_processing and len(stage_nodes) > 1:
        for i in range(len(stage_nodes) - 1):
            builder.connect(stage_nodes[i], stage_nodes[i + 1], 
                          graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### 2. Validate Generated Workflows

```python
def validate_dynamic_workflow(workflow):
    """Validate dynamically generated workflow."""
    
    try:
        workflow.validate()
        node_count = workflow.node_count()
        edge_count = workflow.edge_count()
        
        if node_count == 0:
            raise ValueError("Workflow has no nodes")
        
        print(f"✅ Dynamic workflow valid: {node_count} nodes, {edge_count} edges")
        return True
        
    except Exception as e:
        print(f"❌ Dynamic workflow invalid: {e}")
        return False
```

Dynamic graph generation in GraphBit enables powerful, adaptive workflows that can respond to changing data and requirements. 