# Workflow Validation

GraphBit provides comprehensive validation capabilities to ensure workflow integrity, data quality, and execution reliability. This guide covers all aspects of validation in GraphBit workflows.

## Overview

GraphBit validation operates at multiple levels:
- **Structural Validation**: Workflow graph structure and connectivity
- **Data Validation**: Input/output data format and content validation
- **Business Rules**: Custom validation logic and constraints
- **Runtime Validation**: Real-time validation during execution
- **Performance Validation**: Resource usage and performance constraints

## Structural Validation

### Workflow Structure Validation

```python
import graphbit

def validate_workflow_structure(workflow):
    """Validate basic workflow structure."""
    
    try:
        # Built-in validation
        workflow.validate()
        print("✅ Workflow structure is valid")
        return True
    except Exception as e:
        print(f"❌ Workflow validation failed: {e}")
        return False

def detailed_structure_validation(workflow):
    """Perform detailed structural validation."""
    
    validation_results = {
        "has_nodes": False,
        "has_connections": False,
        "no_cycles": False,
        "reachable_nodes": False,
        "valid_edges": False
    }
    
    # Check if workflow has nodes
    node_count = workflow.node_count()
    validation_results["has_nodes"] = node_count > 0
    
    # Check if workflow has edges (for multi-node workflows)
    edge_count = workflow.edge_count()
    validation_results["has_connections"] = edge_count > 0 or node_count <= 1
    
    # Check for cycles (GraphBit prevents cycles automatically)
    try:
        workflow.validate()
        validation_results["no_cycles"] = True
    except Exception:
        validation_results["no_cycles"] = False
    
    # Print validation summary
    print("Structural Validation Results:")
    for check, result in validation_results.items():
        status = "✅" if result else "❌"
        print(f"  {status} {check.replace('_', ' ').title()}")
    
    return all(validation_results.values())
```

### Node Validation

```python
def validate_nodes(workflow):
    """Validate individual nodes in workflow."""
    
    validation_issues = []
    
    # Get workflow nodes (this would depend on GraphBit's API)
    nodes = workflow.get_nodes()  # Hypothetical method
    
    for node in nodes:
        issues = validate_single_node(node)
        if issues:
            validation_issues.extend(issues)
    
    if validation_issues:
        print("❌ Node validation issues found:")
        for issue in validation_issues:
            print(f"  - {issue}")
        return False
    else:
        print("✅ All nodes are valid")
        return True

def validate_single_node(node):
    """Validate a single node."""
    
    issues = []
    
    # Check node name
    if not node.name() or len(node.name().strip()) == 0:
        issues.append(f"Node {node.id()} has empty name")
    
    # Check node description
    if not node.description() or len(node.description().strip()) == 0:
        issues.append(f"Node {node.name()} has empty description")
    
    # Node-specific validation
    node_type = node.node_type()  # Hypothetical method
    
    if node_type == "agent":
        issues.extend(validate_agent_node(node))
    elif node_type == "condition":
        issues.extend(validate_condition_node(node))
    elif node_type == "transform":
        issues.extend(validate_transform_node(node))
    
    return issues

def validate_agent_node(node):
    """Validate agent node specifics."""
    
    issues = []
    
    # Check if agent has prompt
    prompt = node.prompt()  # Hypothetical method
    if not prompt or len(prompt.strip()) == 0:
        issues.append(f"Agent node {node.name()} has empty prompt")
    
    # Check if prompt has valid variables
    if prompt and "{" in prompt and "}" in prompt:
        # Validate prompt variables
        import re
        variables = re.findall(r'\{(\w+)\}', prompt)
        if not variables:
            issues.append(f"Agent node {node.name()} prompt appears to have malformed variables")
    
    return issues

def validate_condition_node(node):
    """Validate condition node specifics."""
    
    issues = []
    
    # Check if condition has expression
    expression = node.expression()  # Hypothetical method
    if not expression or len(expression.strip()) == 0:
        issues.append(f"Condition node {node.name()} has empty expression")
    
    # Validate expression syntax (basic check)
    if expression:
        try:
            # Simple syntax validation
            validate_condition_expression(expression)
        except Exception as e:
            issues.append(f"Condition node {node.name()} has invalid expression: {e}")
    
    return issues

def validate_transform_node(node):
    """Validate transform node specifics."""
    
    issues = []
    
    # Check if transform has valid transformation type
    transformation = node.transformation()  # Hypothetical method
    valid_transformations = ["uppercase", "lowercase", "json_extract", "split", "join"]
    
    if transformation not in valid_transformations:
        issues.append(f"Transform node {node.name()} has invalid transformation: {transformation}")
    
    return issues

def validate_condition_expression(expression):
    """Validate condition expression syntax."""
    
    # Basic validation - could be expanded
    forbidden_keywords = ["import", "exec", "eval", "__"]
    
    for keyword in forbidden_keywords:
        if keyword in expression.lower():
            raise ValueError(f"Forbidden keyword '{keyword}' in expression")
    
    # Check for balanced parentheses
    if expression.count("(") != expression.count(")"):
        raise ValueError("Unbalanced parentheses in expression")
    
    return True
```

## Data Validation

### Input Data Validation

```python
def create_input_validation_workflow():
    """Create workflow with comprehensive input validation."""
    
    builder = graphbit.PyWorkflowBuilder("Input Validation Workflow")
    
    # Input validator
    input_validator = graphbit.PyWorkflowNode.agent_node(
        name="Input Validator",
        description="Validates all input data",
        agent_id="input_validator",
        prompt="""
        Validate this input data comprehensively:
        
        Data: {input}
        
        Check for:
        1. Data completeness (no missing required fields)
        2. Data format (correct types and structures)
        3. Data ranges (values within acceptable limits)
        4. Data consistency (logical relationships)
        5. Security issues (potential malicious content)
        
        Respond with:
        - validation_status: VALID or INVALID
        - issues_found: list of specific issues
        - severity: HIGH, MEDIUM, or LOW
        - recommendations: suggested fixes
        """
    )
    
    # Validation gate
    validation_gate = graphbit.PyWorkflowNode.condition_node(
        name="Validation Gate",
        description="Gates processing based on validation",
        expression="validation_status == 'VALID' && severity != 'HIGH'"
    )
    
    # Build validation workflow
    validator_id = builder.add_node(input_validator)
    gate_id = builder.add_node(validation_gate)
    
    builder.connect(validator_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def validate_input_schema(data, schema):
    """Validate input data against schema."""
    
    validation_results = {
        "valid": True,
        "errors": [],
        "warnings": []
    }
    
    # Check required fields
    required_fields = schema.get("required", [])
    for field in required_fields:
        if field not in data:
            validation_results["valid"] = False
            validation_results["errors"].append(f"Missing required field: {field}")
    
    # Check field types
    field_types = schema.get("types", {})
    for field, expected_type in field_types.items():
        if field in data:
            actual_type = type(data[field]).__name__
            if actual_type != expected_type:
                validation_results["warnings"].append(
                    f"Field {field} type mismatch: expected {expected_type}, got {actual_type}"
                )
    
    return validation_results
```

### Output Validation

```python
def create_output_validation_workflow():
    """Create workflow with output validation."""
    
    builder = graphbit.PyWorkflowBuilder("Output Validation Workflow")
    
    # Main processor
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Main Processor",
        description="Main data processing",
        agent_id="processor",
        prompt="Process this data: {input}"
    )
    
    # Output validator
    output_validator = graphbit.PyWorkflowNode.agent_node(
        name="Output Validator",
        description="Validates processing output",
        agent_id="output_validator",
        prompt="""
        Validate this processing output:
        
        Output: {processor_output}
        Expected Format: {expected_format}
        
        Check:
        1. Format compliance
        2. Completeness
        3. Logical consistency
        4. Quality metrics
        
        Provide validation score (0-10) and issues found.
        """
    )
    
    # Quality gate
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        name="Quality Gate",
        description="Ensures output quality",
        expression="validation_score >= 7 && critical_issues == 0"
    )
    
    # Output formatter
    formatter = graphbit.PyWorkflowNode.agent_node(
        name="Output Formatter",
        description="Formats validated output",
        agent_id="formatter",
        prompt="Format this validated output for final delivery: {validated_output}"
    )
    
    # Error handler
    error_handler = graphbit.PyWorkflowNode.agent_node(
        name="Error Handler",
        description="Handles validation failures",
        agent_id="error_handler",
        prompt="Handle validation failure and provide alternative output: {validation_errors}"
    )
    
    # Build output validation workflow
    proc_id = builder.add_node(processor)
    val_id = builder.add_node(output_validator)
    gate_id = builder.add_node(quality_gate)
    fmt_id = builder.add_node(formatter)
    err_id = builder.add_node(error_handler)
    
    builder.connect(proc_id, val_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(val_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(gate_id, fmt_id, graphbit.PyWorkflowEdge.conditional("validation_score >= 7"))
    builder.connect(gate_id, err_id, graphbit.PyWorkflowEdge.conditional("validation_score < 7"))
    
    return builder.build()
```

## Business Rules Validation

### Custom Validation Rules

```python
class ValidationRule:
    """Base class for validation rules."""
    
    def __init__(self, name, description):
        self.name = name
        self.description = description
    
    def validate(self, data):
        """Validate data against this rule."""
        raise NotImplementedError

class RangeValidationRule(ValidationRule):
    """Validates numeric ranges."""
    
    def __init__(self, field, min_value, max_value):
        super().__init__(f"Range validation for {field}", 
                        f"{field} must be between {min_value} and {max_value}")
        self.field = field
        self.min_value = min_value
        self.max_value = max_value
    
    def validate(self, data):
        if self.field not in data:
            return {"valid": False, "error": f"Missing field: {self.field}"}
        
        value = data[self.field]
        if not isinstance(value, (int, float)):
            return {"valid": False, "error": f"{self.field} must be numeric"}
        
        if not (self.min_value <= value <= self.max_value):
            return {"valid": False, "error": f"{self.field} must be between {self.min_value} and {self.max_value}"}
        
        return {"valid": True}

def create_business_validation_workflow(rules):
    """Create workflow with custom business rules."""
    
    builder = graphbit.PyWorkflowBuilder("Business Validation Workflow")
    
    # Rule validator
    rule_validator = graphbit.PyWorkflowNode.agent_node(
        name="Business Rule Validator",
        description="Validates against business rules",
        agent_id="rule_validator",
        prompt=f"""
        Validate this data against business rules:
        
        Data: {{input}}
        Rules: {[rule.description for rule in rules]}
        
        Check each rule and report:
        1. Which rules passed/failed
        2. Specific violations
        3. Severity of violations
        4. Recommended actions
        """
    )
    
    builder.add_node(rule_validator)
    
    return builder.build()
```

## Runtime Validation

### Real-time Validation During Execution

```python
def create_runtime_validation_workflow():
    """Create workflow with runtime validation."""
    
    builder = graphbit.PyWorkflowBuilder("Runtime Validation Workflow")
    
    # Step 1: Process with validation
    step1 = graphbit.PyWorkflowNode.agent_node(
        name="Step 1 Processor",
        description="First processing step with validation",
        agent_id="step1",
        prompt="""
        Process this data and validate the result:
        
        Input: {input}
        
        Process the data and then validate:
        1. Processing completed successfully
        2. Output format is correct
        3. No data corruption occurred
        4. Performance metrics acceptable
        
        Include validation_status in response.
        """
    )
    
    # Runtime validator
    runtime_validator = graphbit.PyWorkflowNode.condition_node(
        name="Runtime Validator",
        description="Validates runtime execution",
        expression="validation_status == 'VALID' && processing_time < 30000"
    )
    
    # Build runtime validation workflow
    step1_id = builder.add_node(step1)
    validator_id = builder.add_node(runtime_validator)
    
    builder.connect(step1_id, validator_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Performance Validation

### Resource Usage Validation

```python
def create_performance_validation_workflow():
    """Create workflow with performance validation."""
    
    builder = graphbit.PyWorkflowBuilder("Performance Validation Workflow")
    
    # Performance monitor
    perf_monitor = graphbit.PyWorkflowNode.agent_node(
        name="Performance Monitor",
        description="Monitors processing performance",
        agent_id="perf_monitor",
        prompt="""
        Monitor and validate performance metrics:
        
        Processing Data: {input}
        Start Time: {start_time}
        
        Track:
        1. Processing duration
        2. Memory usage
        3. API call counts
        4. Error rates
        
        Validate against thresholds:
        - Duration < 60 seconds
        - Memory < 500MB
        - API calls < 100
        - Error rate < 5%
        """
    )
    
    # Performance gate
    perf_gate = graphbit.PyWorkflowNode.condition_node(
        name="Performance Gate",
        description="Validates performance thresholds",
        expression="duration < 60000 && memory_mb < 500 && api_calls < 100 && error_rate < 0.05"
    )
    
    # Performance optimizer
    optimizer = graphbit.PyWorkflowNode.agent_node(
        name="Performance Optimizer",
        description="Optimizes based on performance data",
        agent_id="optimizer",
        prompt="Optimize processing based on performance metrics: {performance_data}"
    )
    
    # Build performance validation workflow
    monitor_id = builder.add_node(perf_monitor)
    gate_id = builder.add_node(perf_gate)
    opt_id = builder.add_node(optimizer)
    
    builder.connect(monitor_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(gate_id, opt_id, graphbit.PyWorkflowEdge.conditional("duration >= 30000"))
    
    return builder.build()
```

## Best Practices

### 1. Layered Validation Strategy

```python
def create_comprehensive_validation_workflow():
    """Create workflow with layered validation."""
    
    builder = graphbit.PyWorkflowBuilder("Comprehensive Validation")
    
    # Layer 1: Input validation
    input_val = graphbit.PyWorkflowNode.agent_node(
        name="Input Validation", "Validates input", "input_val",
        "Validate input format and content: {input}"
    )
    
    # Layer 2: Business rules validation
    business_val = graphbit.PyWorkflowNode.agent_node(
        name="Business Validation", "Validates business rules", "business_val",
        "Validate against business rules: {validated_input}"
    )
    
    # Layer 3: Processing with validation
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Validated Processor", "Processes with validation", "processor",
        "Process data with continuous validation: {business_validated_data}"
    )
    
    # Build layered validation
    input_id = builder.add_node(input_val)
    business_id = builder.add_node(business_val)
    proc_id = builder.add_node(processor)
    
    builder.connect(input_id, business_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(business_id, proc_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### 2. Validation Error Handling

```python
def create_validation_error_handling():
    """Create robust validation error handling."""
    
    builder = graphbit.PyWorkflowBuilder("Validation Error Handling")
    
    # Validator with error details
    validator = graphbit.PyWorkflowNode.agent_node(
        name="Detailed Validator",
        description="Provides detailed validation results",
        agent_id="detailed_validator",
        prompt="""
        Validate data with detailed error reporting:
        
        Data: {input}
        
        Provide:
        - validation_passed: true/false
        - error_count: number of errors
        - warning_count: number of warnings
        - critical_errors: list of critical issues
        - suggestions: recommended fixes
        - retry_possible: whether retry is recommended
        """
    )
    
    # Error classifier
    error_classifier = graphbit.PyWorkflowNode.condition_node(
        name="Error Classifier",
        description="Classifies validation errors",
        expression="validation_passed == true || (error_count <= 3 && critical_errors == 0)"
    )
    
    # Build error handling workflow
    val_id = builder.add_node(validator)
    class_id = builder.add_node(error_classifier)
    
    builder.connect(val_id, class_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

Comprehensive validation in GraphBit ensures reliable, high-quality workflow execution. Use these patterns to build robust validation into your workflows and maintain data integrity throughout processing. 