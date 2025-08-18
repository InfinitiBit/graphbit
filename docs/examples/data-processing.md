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
from graphbit import init, LlmConfig, Executor, Workflow, Node
import os

def create_data_processing_pipeline():
    """Creates a comprehensive data processing workflow."""
    
    # Initialize GraphBit
    init(enable_tracing=True)
    
    # Configure LLM
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Create executor
    executor = Executor(config, timeout_seconds=180, debug=True)
    
    # Create workflow
    workflow = Workflow("Data Processing Pipeline")
    
    # 1. Data Validator
    validator = Node.agent(
        name="Data Validator",
        prompt=f"""Validate this dataset and check for:
- Data completeness
- Format consistency  
- Obvious errors or outliers
- Missing values

Dataset: {input_data}

Provide:
- Validation status (VALID/INVALID)
- Issues found
- Recommended fixes
- Cleaned data if possible

Format response as JSON with validation_status, issues, and cleaned_data fields.
""",
        agent_id="data_validator"
    )
    
    # 2. Statistical Analyzer
    stats_analyzer = Node.agent(
        name="Statistical Analyzer",
        prompt="""Perform comprehensive statistical analysis on this dataset:

Calculate and provide:
- Descriptive statistics (mean, median, mode, std dev)
- Distribution analysis
- Correlation analysis
- Trend identification
- Statistical significance tests where applicable

Format as JSON with clear structure including summary_stats, correlations, and trends.
""",
        agent_id="stats_analyzer"
    )
    
    # 3. Pattern Detector  
    pattern_detector = Node.agent(
        name="Pattern Detector",
        prompt="""Analyze this data for patterns and anomalies:

Identify:
- Recurring patterns
- Seasonal trends
- Anomalies and outliers
- Clustering or groupings
- Predictive indicators

Explain the significance of each finding.
Format as JSON with patterns, anomalies, and insights fields.
""",
        agent_id="pattern_detector"
    )
    
    # 4. Insight Generator
    insight_generator = Node.agent(
        name="Insight Generator",
        prompt="""Generate actionable insights based on this analysis:

Create:
- Key business insights
- Actionable recommendations
- Risk assessments
- Opportunities identified
- Next steps

Focus on practical, implementable insights.
""",
        agent_id="insight_generator"
    )
    
    # 5. Report Generator
    report_generator = Node.agent(
        name="Report Generator",
        prompt="""Create a comprehensive data analysis report:

Format as a professional report with:
- Executive summary
- Data quality assessment
- Key findings
- Statistical highlights
- Actionable recommendations
- Appendices with detailed analysis

Use clear, business-friendly language.
""",
        agent_id="report_generator"
    )
    
    # Add nodes to workflow
    validator_id = workflow.add_node(validator)
    stats_id = workflow.add_node(stats_analyzer)
    pattern_id = workflow.add_node(pattern_detector)
    insight_id = workflow.add_node(insight_generator)
    report_id = workflow.add_node(report_generator)
    
    # Connect processing pipeline
    workflow.connect(validator_id, stats_id)
    workflow.connect(stats_id, pattern_id)
    workflow.connect(validator_id, pattern_id)
    workflow.connect(pattern_id, insight_id)
    workflow.connect(stats_id, insight_id)
    workflow.connect(validator_id, insight_id)
    workflow.connect(insight_id, report_id)
    workflow.connect(pattern_id, report_id)
    workflow.connect(stats_id, report_id)
    workflow.connect(validator_id, report_id)
    
    # Validate workflow
    workflow.validate()
    
    return executor, workflow

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
            {"month": "Jul", "sales": 16000, "region": "East", "product": "C"},
            {"month": "Aug", "sales": 21000, "region": "North", "product": "A"},
            {"month": "Sep", "sales": 14000, "region": "South", "product": "B"},
            {"month": "Oct", "sales": 26000, "region": "North", "product": "A"},
            {"month": "Nov", "sales": 20000, "region": "East", "product": "C"},
            {"month": "Dec", "sales": 28000, "region": "North", "product": "A"}
        ],
        "metadata": {
            "source": "CRM System",
            "period": "2024",
            "currency": "USD"
        }
    }
    
    # Create workflow
    executor, workflow = create_data_processing_pipeline()
    
    # Execute workflow
    print("🚀 Starting data processing pipeline...")
    
    result = executor.execute(workflow)
    
    if result.is_success():
        print("✅ Data processing completed successfully!")
        print("📊 Analysis Report:")
        print("=" * 60)
        print(result.get_all_node_outputs())
        
        # Get execution statistics
        stats = executor.get_stats()
        print(f"\nExecution Stats:")
        print(f"Total executions: {stats.get('total_executions', 0)}")
        print(f"Success rate: {stats.get('successful_executions', 0)}/{stats.get('total_executions', 0)}")
        print(f"Average duration: {stats.get('average_duration_ms', 0):.2f}ms")
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

## Advanced Examples

### Batch Data Processing

```python
from graphbit import init, LlmConfig, Executor, Workflow, Node
import os
import asyncio

async def process_multiple_datasets_async():
    """Process multiple datasets asynchronously."""
    
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    # Use high-throughput executor for batch processing
    Executor(config, timeout_seconds=120, debug=False)
    
    # Create simple analysis workflow
    workflow = Workflow("Batch Data Analyzer")
    
    analyzer = Node.agent(
        name="Batch Analyzer",
        prompt=f"""Analyze this dataset quickly:

Data: {dataset}

Provide:
- Quick summary statistics
- Key trends
- Notable patterns
- Recommendations

Keep analysis concise but actionable.
""",
        agent_id="batch_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    # Execute asynchronously
    result = await executor.run_async(workflow)
    
    if result.is_success():
        return result.get_all_node_outputs()
    else:
        return f"Error: {result.get_error()}"

# Usage
# result = asyncio.run(process_multiple_datasets_async())
```

### Time Series Analysis

```python
def create_time_series_pipeline():
    """Create specialized pipeline for time series data."""
    
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = Executor(config, debug=True)
    workflow = Workflow("Time Series Analysis")
    
    # Trend Analyzer
    trend_analyzer = Node.agent(
        name="Trend Analyzer",
        prompt=f"""Analyze trends in this time series data:

{time_series_data}

Identify:
- Overall trend direction (up/down/stable)
- Seasonality patterns
- Cyclical behavior
- Growth rates
- Trend changes or breakpoints

Provide quantitative analysis where possible.
""",
        agent_id="trend_analyzer"
    )
    
    # Forecast Generator
    forecaster = Node.agent(
        name="Forecaster",
        prompt=f"""Based on this trend analysis, generate forecasts:
Historical Data: {time_series_data}

Create:
- Short-term forecast (next 3 periods)
- Medium-term forecast (next 6-12 periods)
- Confidence intervals
- Key assumptions
- Risk factors

Explain methodology and limitations.
""",
        agent_id="forecaster"
    )
    
    trend_id = workflow.add_node(trend_analyzer)
    forecast_id = workflow.add_node(forecaster)
    
    workflow.connect(trend_id, forecast_id)
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_time_series_pipeline()
result = executor.execute(workflow)
```

### Data Quality Assessment

```python
def create_data_quality_pipeline():
    """Create pipeline focused on data quality assessment."""
    
    init()
    
    config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    executor = Executor(config)
    workflow = Workflow("Data Quality Assessment")
    
    # Completeness Checker
    completeness_checker = Node.agent(
        name="Completeness Checker",
        prompt=f"""Assess data completeness:

Dataset: {input_data}

Check for:
- Missing values per field
- Data coverage gaps
- Incomplete records
- Empty or null fields

Rate completeness (1-10) and provide recommendations.
""",
        agent_id="completeness_checker"
    )
    
    # Consistency Checker
    consistency_checker = Node.agent(
        name="Consistency Checker",
        prompt=f"""Check data consistency:

Dataset: {input_data}

Examine:
- Format consistency
- Value range consistency
- Cross-field validation
- Data type consistency
- Naming convention adherence

Rate consistency (1-10) and identify issues.
""",
        agent_id="consistency_checker"
    )
    
    # Accuracy Assessor
    accuracy_assessor = Node.agent(
        name="Accuracy Assessor",
        prompt=f"""Assess data accuracy:

Dataset: {input_data}

Evaluate:
- Logical value ranges
- Cross-validation checks
- Outlier detection
- Business rule compliance

Provide accuracy score and recommendations.
""",
        agent_id="accuracy_assessor"
    )
    
    # Quality Score Calculator
    quality_calculator = Node.agent(
        name="Quality Calculator",
        prompt="""Calculate overall data quality score:

Provide:
- Overall quality score (1-10)
- Quality grade (A-F)
- Priority improvement areas
- Action plan for quality improvement

Create executive summary of data quality.
""",
        agent_id="quality_calculator"
    )
    
    complete_id = workflow.add_node(completeness_checker)
    consistent_id = workflow.add_node(consistency_checker)
    accurate_id = workflow.add_node(accuracy_assessor)
    quality_id = workflow.add_node(quality_calculator)
    
    # Run completeness and consistency in parallel, then accuracy, then quality
    workflow.connect(complete_id, accurate_id)
    workflow.connect(consistent_id, accurate_id)
    workflow.connect(complete_id, quality_id)
    workflow.connect(consistent_id, quality_id)
    workflow.connect(accurate_id, quality_id)
    
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_data_quality_pipeline()
result = executor.execute(workflow)
```

## Using Alternative LLM Providers

### Anthropic Claude for Data Analysis

```python
def create_anthropic_data_pipeline():
    """Create data pipeline using Anthropic Claude."""
    
    init()
    
    config = LlmConfig.anthropic(
        api_key=os.getenv("ANTHROPIC_API_KEY"),
        model="claude-sonnet-4-20250514"
    )
    
    executor = Executor(config, debug=True)
    workflow = Workflow("Claude Data Analyzer")
    
    analyzer = Node.agent(
        name="Claude Analyzer",
        prompt=f"""Analyze this dataset with Claude's analytical capabilities:

{dataset}

Provide comprehensive analysis including:
- Statistical summary
- Pattern recognition
- Anomaly detection
- Insights and recommendations

Use Claude's strong reasoning for deep insights.
""",
        agent_id="claude_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_anthropic_data_pipeline()
result = executor.execute(workflow)
```

### Local Ollama for Private Data

```python
def create_ollama_data_pipeline():
    """Create data pipeline using local Ollama for sensitive data."""
    
    init()
    
    # No API key needed for local Ollama
    config = LlmConfig.ollama("llama3.2")
    
    executor = Executor(
        config,
        timeout_seconds=240,  # Longer timeout for local processing
        debug=True
    )
    
    workflow = Workflow("Private Data Analyzer")
    
    analyzer = Node.agent(
        name="Local Analyzer",
        prompt=f"""Analyze this sensitive dataset locally:

{dataset}

Provide:
- Basic statistics
- Key patterns
- Security considerations
- Privacy-preserving insights

Keep analysis secure and local.
""",
        agent_id="local_analyzer"
    )
    
    workflow.add_node(analyzer)
    workflow.validate()
    
    return executor, workflow

# Usage
executor, workflow = create_ollama_data_pipeline()
result = executor.execute(workflow)
```

## Embeddings for Similarity Analysis

```python
def create_embedding_analysis_pipeline():
    """Create pipeline using embeddings for similarity analysis."""
    
    init()
    
    # Configure embeddings
    embedding_config = EmbeddingConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="text-embedding-3-small"
    )
    
    embedding_client = EmbeddingClient(embedding_config)
    
    # Sample text data
    texts = [
        "Revenue increased by 15% this quarter",
        "Sales performance exceeded expectations",
        "Customer satisfaction scores improved",
        "Market share declined in key segments",
        "Operating expenses rose significantly"
    ]
    
    # Generate embeddings
    embeddings = embedding_client.embed_many(texts)
    
    # Calculate similarities
    similarities = []
    for i in range(len(embeddings)):
        for j in range(i + 1, len(embeddings)):
            similarity = EmbeddingClient.similarity(
                embeddings[i], 
                embeddings[j]
            )
            similarities.append({
                "text1": texts[i],
                "text2": texts[j],
                "similarity": similarity
            })
    
    # Find most similar pairs
    similarities.sort(key=lambda x: x["similarity"], reverse=True)
    
    print("🔍 Most Similar Text Pairs:")
    for sim in similarities[:3]:
        print(f"Similarity: {sim['similarity']:.3f}")
        print(f"Text 1: {sim['text1']}")
        print(f"Text 2: {sim['text2']}")
        print("-" * 40)

# Usage
create_embedding_analysis_pipeline()
```

## System Health and Monitoring

```python
def monitor_data_processing_health():
    """Monitor GraphBit health during data processing."""
    
    init()
    
    # Check system health
    health = health_check()
    print("Health Check:")
    for key, value in health.items():
        print(f"  {key}: {value}")
    
    # Get system info
    info = get_system_info()
    print("\nSystem Information:")
    for key, value in info.items():
        print(f"  {key}: {value}")
    
    # Check version
    version_info = version()
    print(f"\nVersion: {version_info}")
    
    return health["overall_healthy"]

# Usage
if monitor_data_processing_health():
    print("System ready for data processing")
else:
    print("System issues detected")
```

## Key Benefits

### Flexibility
- **Multiple Analysis Types**: Statistical, pattern, quality, time series
- **Multiple LLM Providers**: OpenAI, Anthropic, Ollama support
- **Execution Modes**: Sync, async, batch processing

### Reliability
- **Error Handling**: Comprehensive error reporting
- **Health Monitoring**: System health checks
- **Performance Tracking**: Built-in execution statistics

### Security
- **Local Processing**: Ollama for sensitive data
- **Privacy-Preserving**: Keep data processing local when needed
- **Embedding Analysis**: Semantic similarity without exposing content

This example demonstrates GraphBit's capabilities for building robust, flexible data processing workflows that can handle various analysis tasks with reliability and performance optimization.
