# Data Processing Workflow

This example demonstrates how to build a comprehensive data processing pipeline using GraphBit to analyze, transform, and generate insights from structured data.

## Overview

We'll create a workflow that:
1. Loads and validates input data
2. Performs statistical analysis
3. Identifies patterns and anomalies
4. Generates actionable insights
5. Creates formatted reports

## Complete Example

```python
import graphbit
import json
import os

def create_data_processing_pipeline():
    """Creates a comprehensive data processing workflow."""
    
    # Initialize GraphBit
    graphbit.init()
    
    # Configure LLM
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Create workflow builder
    builder = graphbit.PyWorkflowBuilder("Data Processing Pipeline")
    builder.description("Comprehensive data analysis and insight generation")
    
    # 1. Data Validator
    validator = graphbit.PyWorkflowNode.agent_node(
        name="Data Validator",
        description="Validates and cleans input data",
        agent_id="data_validator",
        prompt="""
        Validate this dataset and check for:
        - Data completeness
        - Format consistency  
        - Obvious errors or outliers
        - Missing values
        
        Dataset: {input}
        
        Provide:
        - Validation status (VALID/INVALID)
        - Issues found
        - Recommended fixes
        - Cleaned data if possible
        """
    )
    
    # 2. Statistical Analyzer
    stats_analyzer = graphbit.PyWorkflowNode.agent_node_with_config(
        name="Statistical Analyzer",
        description="Performs statistical analysis of the data",
        agent_id="stats_analyzer",
        prompt="""
        Perform comprehensive statistical analysis on this dataset:
        
        {validated_data}
        
        Calculate and provide:
        - Descriptive statistics (mean, median, mode, std dev)
        - Distribution analysis
        - Correlation analysis
        - Trend identification
        - Statistical significance tests where applicable
        
        Format as JSON with clear structure.
        """,
        max_tokens=2000,
        temperature=0.2
    )
    
    # 3. Pattern Detector  
    pattern_detector = graphbit.PyWorkflowNode.agent_node(
        name="Pattern Detector",
        description="Identifies patterns and anomalies in data",
        agent_id="pattern_detector",
        prompt="""
        Analyze this data for patterns and anomalies:
        
        Statistical Analysis: {stats_results}
        Original Data: {validated_data}
        
        Identify:
        - Recurring patterns
        - Seasonal trends
        - Anomalies and outliers
        - Clustering or groupings
        - Predictive indicators
        
        Explain the significance of each finding.
        """
    )
    
    # Build workflow
    validator_id = builder.add_node(validator)
    stats_id = builder.add_node(stats_analyzer)
    pattern_id = builder.add_node(pattern_detector)
    
    # Connect processing path
    builder.connect(validator_id, stats_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(stats_id, pattern_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def main():
    """Execute the data processing pipeline."""
    
    # Sample dataset
    sample_data = {
        "sales_data": [
            {"month": "Jan", "sales": 15000, "region": "North", "product": "A"},
            {"month": "Feb", "sales": 18000, "region": "North", "product": "A"},
            {"month": "Mar", "sales": 12000, "region": "South", "product": "B"},
            {"month": "Apr", "sales": 22000, "region": "North", "product": "A"},
            {"month": "May", "sales": 19000, "region": "South", "product": "B"},
            {"month": "Jun", "sales": 25000, "region": "North", "product": "A"},
        ]
    }
    
    # Create workflow
    workflow = create_data_processing_pipeline()
    
    # Configure executor
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = graphbit.PyWorkflowExecutor(config)
    
    # Execute workflow
    print("🚀 Starting data processing pipeline...")
    
    context = graphbit.PyExecutionContext()
    context.set_variable("input", json.dumps(sample_data))
    
    result = executor.execute_with_context(workflow, context)
    
    if result.is_completed():
        print("✅ Data processing completed successfully!")
        print("📊 Analysis Report:")
        print("=" * 50)
        print(result.get_variable("output"))
    else:
        print("❌ Data processing failed:")
        print(result.get_error())

if __name__ == "__main__":
    main()
```

## Key Features

### Data Validation
- Comprehensive input validation
- Data quality assessment
- Error detection and correction
- Format standardization

### Statistical Analysis
- Descriptive statistics
- Distribution analysis
- Correlation detection
- Trend identification

### Pattern Recognition
- Anomaly detection
- Seasonal pattern identification
- Clustering analysis
- Predictive indicators

This example shows how GraphBit can handle complex data processing tasks with reliability and scalability.

## Advanced Features

### Batch Data Processing

```python
def create_batch_processor():
    """Process multiple datasets in parallel."""
    
    builder = graphbit.PyWorkflowBuilder("Batch Data Processor")
    
    # Create multiple processing branches
    processors = []
    for i in range(3):
        processor = graphbit.PyWorkflowNode.agent_node(
            name=f"Data Processor {i+1}",
            description=f"Processes batch {i+1}",
            agent_id=f"processor_{i+1}",
            prompt=f"Process dataset batch {i+1}: {{batch_{i+1}_data}}"
        )
        processors.append(processor)
    
    # Aggregator
    aggregator = graphbit.PyWorkflowNode.agent_node(
        name="Results Aggregator",
        description="Combines batch processing results",
        agent_id="aggregator",
        prompt="""
        Combine these batch processing results:
        
        Batch 1: {processor_1_output}
        Batch 2: {processor_2_output}  
        Batch 3: {processor_3_output}
        
        Provide:
        - Combined statistics
        - Cross-batch patterns
        - Consolidated insights
        """
    )
    
    # Build parallel processing
    processor_ids = [builder.add_node(p) for p in processors]
    agg_id = builder.add_node(aggregator)
    
    # Connect all processors to aggregator
    for proc_id in processor_ids:
        builder.connect(proc_id, agg_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Real-time Data Stream Processing

```python
def create_streaming_processor():
    """Process streaming data in real-time."""
    
    builder = graphbit.PyWorkflowBuilder("Streaming Data Processor")
    
    # Stream validator
    validator = graphbit.PyWorkflowNode.agent_node(
        name="Stream Validator",
        description="Validates streaming data",
        agent_id="stream_validator",
        prompt="""
        Validate this streaming data point:
        
        {stream_data}
        
        Check for:
        - Data completeness
        - Format validity
        - Value ranges
        - Timestamp accuracy
        """
    )
    
    # Anomaly detector
    anomaly_detector = graphbit.PyWorkflowNode.agent_node(
        name="Anomaly Detector",
        description="Detects anomalies in real-time",
        agent_id="anomaly_detector",
        prompt="""
        Analyze this data point for anomalies:
        
        Current Data: {validated_stream}
        Historical Context: {historical_data}
        
        Detect:
        - Statistical anomalies
        - Pattern deviations  
        - Trend breaks
        - Threshold violations
        """
    )
    
    # Alert generator
    alert_generator = graphbit.PyWorkflowNode.condition_node(
        name="Alert Generator",
        description="Generates alerts for significant anomalies",
        expression="anomaly_score > 0.8 || critical_threshold_exceeded == true"
    )
    
    # Rate limiter
    rate_limiter = graphbit.PyWorkflowNode.delay_node(
        name="Rate Limiter",
        description="Prevents excessive processing",
        duration_seconds=1
    )
    
    # Build streaming pipeline
    val_id = builder.add_node(validator)
    anom_id = builder.add_node(anomaly_detector)
    alert_id = builder.add_node(alert_generator)
    rate_id = builder.add_node(rate_limiter)
    
    builder.connect(val_id, rate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(rate_id, anom_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(anom_id, alert_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

### Data Quality Assessment

```python
def create_quality_assessor():
    """Comprehensive data quality assessment."""
    
    builder = graphbit.PyWorkflowBuilder("Data Quality Assessor")
    
    # Completeness checker
    completeness_checker = graphbit.PyWorkflowNode.agent_node(
        name="Completeness Checker",
        description="Checks data completeness",
        agent_id="completeness_checker",
        prompt="""
        Assess data completeness:
        
        {dataset}
        
        Analyze:
        - Missing value percentage
        - Null patterns
        - Required field coverage
        - Data density by segment
        
        Score: 0-10
        """
    )
    
    # Accuracy validator
    accuracy_validator = graphbit.PyWorkflowNode.agent_node(
        name="Accuracy Validator",
        description="Validates data accuracy",
        agent_id="accuracy_validator",
        prompt="""
        Validate data accuracy:
        
        {dataset}
        
        Check:
        - Value ranges
        - Format consistency
        - Business rule compliance
        - Cross-field validation
        
        Score: 0-10
        """
    )
    
    # Consistency analyzer
    consistency_analyzer = graphbit.PyWorkflowNode.agent_node(
        name="Consistency Analyzer",
        description="Analyzes data consistency",
        agent_id="consistency_analyzer",
        prompt="""
        Analyze data consistency:
        
        {dataset}
        
        Evaluate:
        - Format standardization
        - Naming conventions
        - Unit consistency
        - Referential integrity
        
        Score: 0-10
        """
    )
    
    # Quality synthesizer
    synthesizer = graphbit.PyWorkflowNode.agent_node(
        name="Quality Synthesizer",
        description="Combines quality assessments",
        agent_id="quality_synthesizer",
        prompt="""
        Synthesize data quality assessment:
        
        Completeness: {completeness_output}
        Accuracy: {accuracy_output}
        Consistency: {consistency_output}
        
        Provide:
        - Overall quality score
        - Key issues summary
        - Improvement recommendations
        - Quality trend analysis
        """
    )
    
    # Build quality assessment pipeline
    comp_id = builder.add_node(completeness_checker)
    acc_id = builder.add_node(accuracy_validator)
    cons_id = builder.add_node(consistency_analyzer)
    synth_id = builder.add_node(synthesizer)
    
    # Parallel quality checks
    builder.connect(comp_id, synth_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(acc_id, synth_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(cons_id, synth_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Integration Examples

### Database Integration

```python
import sqlite3
import pandas as pd

def process_database_data():
    """Process data directly from database."""
    
    # Connect to database
    conn = sqlite3.connect('analytics.db')
    
    # Query data
    query = """
    SELECT 
        date, 
        sales_amount, 
        region, 
        product_category,
        customer_segment
    FROM sales_data 
    WHERE date >= date('now', '-30 days')
    """
    
    df = pd.read_sql_query(query, conn)
    
    # Convert to JSON for processing
    data_json = df.to_json(orient='records')
    
    # Create and execute workflow
    workflow = create_data_processing_pipeline()
    
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = graphbit.PyWorkflowExecutor(config)
    
    context = graphbit.PyExecutionContext()
    context.set_variable("input", data_json)
    
    result = executor.execute_with_context(workflow, context)
    
    conn.close()
    return result
```

### CSV File Processing

```python
def process_csv_files(file_paths):
    """Process multiple CSV files."""
    
    results = []
    
    for file_path in file_paths:
        # Read CSV
        df = pd.read_csv(file_path)
        data_json = df.to_json(orient='records')
        
        # Process with GraphBit
        workflow = create_data_processing_pipeline()
        
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
        
        executor = graphbit.PyWorkflowExecutor(config)
        
        context = graphbit.PyExecutionContext()
        context.set_variable("input", data_json)
        context.set_variable("file_name", file_path)
        
        result = executor.execute_with_context(workflow, context)
        results.append({
            'file': file_path,
            'result': result
        })
    
    return results
```

## Performance Optimization

### Caching Results

```python
def create_cached_processor():
    """Data processor with result caching."""
    
    import hashlib
    import pickle
    
    cache = {}
    
    def get_cache_key(data):
        return hashlib.md5(str(data).encode()).hexdigest()
    
    def cached_analysis(data):
        cache_key = get_cache_key(data)
        
        if cache_key in cache:
            print("📋 Using cached result")
            return cache[cache_key]
        
        # Process with GraphBit
        workflow = create_data_processing_pipeline()
        
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
        
        executor = graphbit.PyWorkflowExecutor(config)
        
        context = graphbit.PyExecutionContext()
        context.set_variable("input", json.dumps(data))
        
        result = executor.execute_with_context(workflow, context)
        
        # Cache result
        cache[cache_key] = result
        
        return result
    
    return cached_analysis
```

### Parallel Processing

```python
import concurrent.futures

def process_datasets_parallel(datasets):
    """Process multiple datasets in parallel."""
    
    def process_single_dataset(data):
        workflow = create_data_processing_pipeline()
        
        config = graphbit.PyLlmConfig.openai(
            api_key=os.getenv("OPENAI_API_KEY"),
            model="gpt-4o-mini"
        )
        
        executor = graphbit.PyWorkflowExecutor(config)
        
        context = graphbit.PyExecutionContext()
        context.set_variable("input", json.dumps(data))
        
        return executor.execute_with_context(workflow, context)
    
    # Process in parallel
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        futures = [executor.submit(process_single_dataset, data) for data in datasets]
        results = [future.result() for future in concurrent.futures.as_completed(futures)]
    
    return results
```

## Best Practices

1. **Data Validation**: Always validate input data before processing
2. **Quality Gates**: Use condition nodes to ensure quality standards
3. **Error Handling**: Include alternative paths for edge cases
4. **Performance**: Cache results and use parallel processing when possible
5. **Monitoring**: Track processing times and success rates
6. **Documentation**: Document data formats and processing steps

This data processing workflow demonstrates how GraphBit can handle complex analytical tasks with reliability and scalability, making it ideal for business intelligence and data science applications. 