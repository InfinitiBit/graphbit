# Data Processing Workflow - JavaScript

This example demonstrates how to build a comprehensive data processing pipeline using GraphBit's JavaScript bindings to analyze, transform, and generate insights from structured data.

## Overview

We'll create a workflow that:
1. Loads and validates input data
2. Performs statistical analysis
3. Identifies patterns and anomalies
4. Generates actionable insights
5. Creates formatted reports

## Complete Implementation

```typescript
import {
  init,
  LlmConfig,
  Executor,
  Workflow,
  Node,
  ToolRegistry,
  registerAsync
} from '@infinitibit_gmbh/graphbit';

interface DataPoint {
  [key: string]: string | number | boolean;
}

interface ValidationResult {
  status: 'VALID' | 'INVALID';
  issues: string[];
  cleanedData?: DataPoint[];
}

interface AnalysisReport {
  validation: ValidationResult;
  statistics?: any;
  patterns?: any;
  insights?: string[];
  report?: string;
}

class DataProcessingPipeline {
  private executor: Executor;
  private toolRegistry: ToolRegistry;

  constructor(apiKey: string) {
    init();

    const config = LlmConfig.openai({
      apiKey,
      model: 'gpt-4o-mini'
    });

    this.executor = new Executor(config);
    this.toolRegistry = new ToolRegistry();
  }

  async initialize(): Promise<void> {
    await this.registerDataTools();
    console.log('✅ Data processing pipeline initialized');
  }

  private async registerDataTools(): Promise<void> {
    // Register data processing tools
    
    // Statistical calculation tool
    // Statistical calculation tool
    registerAsync(
      this.toolRegistry,
      'calculate_statistics',
      'Calculate basic statistics for numerical data',
      {
        type: 'object',
        properties: {
          data: { type: 'array', description: 'Array of numbers' }
        },
        required: ['data']
      },
      async (params: any) => {
        const data = params.data as number[];
        
        if (!Array.isArray(data) || data.length === 0) {
          return { error: 'Invalid data' };
        }

        const sorted = [...data].sort((a, b) => a - b);
        const sum = data.reduce((a, b) => a + b, 0);
        const mean = sum / data.length;
        
        const variance = data.reduce((acc, val) => 
          acc + Math.pow(val - mean, 2), 0
        ) / data.length;
        
        const stdDev = Math.sqrt(variance);
        
        const median = data.length % 2 === 0
          ? (sorted[data.length / 2 - 1] + sorted[data.length / 2]) / 2
          : sorted[Math.floor(data.length / 2)];

        return {
          count: data.length,
          sum,
          mean: Math.round(mean * 100) / 100,
          median,
          stdDev: Math.round(stdDev * 100) / 100,
          min: sorted[0],
          max: sorted[sorted.length - 1],
          range: sorted[sorted.length - 1] - sorted[0]
        };
      }
    );

    // Outlier detection tool
    await this.toolRegistry.register({
      name: 'detect_outliers',
      description: 'Detect outliers in numerical data using IQR method',
      inputSchema: {
        type: 'object',
        properties: {
          data: { type: 'array', description: 'Array of numbers' }
        },
        required: ['data']
      },
      handler: async (params: any) => {
        const data = params.data as number[];
        
        if (!Array.isArray(data) || data.length < 4) {
          return { error: 'Insufficient data for outlier detection' };
        }

        const sorted = [...data].sort((a, b) => a - b);
        
        const q1Index = Math.floor(sorted.length * 0.25);
        const q3Index = Math.floor(sorted.length * 0.75);
        const q1 = sorted[q1Index];
        const q3 = sorted[q3Index];
        const iqr = q3 - q1;
        
        const lowerBound = q1 - 1.5 * iqr;
        const upperBound = q3 + 1.5 * iqr;
        
        const outliers = data.filter(
          val => val < lowerBound || val > upperBound
        );

        return {
          q1,
          q3,
          iqr,
          lowerBound,
          upperBound,
          outliers,
          outlierCount: outliers.length,
          outlierPercentage: Math.round((outliers.length / data.length) * 100)
        };
      }
    });

    // Data validation tool
    await this.toolRegistry.register({
      name: 'validate_data',
      description: 'Validate data completeness and format',
      inputSchema: {
        type: 'object',
        properties: {
          dataJson: { type: 'string', description: 'JSON string of data array' },
          requiredFields: { type: 'array', description: 'Required field names' }
        },
        required: ['dataJson', 'requiredFields']
      },
      handler: async (params: any) => {
        try {
          const data = JSON.parse(params.dataJson);
          const required = params.requiredFields as string[];
          
          const issues: string[] = [];
          
          if (!Array.isArray(data)) {
            issues.push('Data is not an array');
            return { valid: false, issues };
          }

          // Check required fields
          data.forEach((row, index) => {
            required.forEach(field => {
              if (!(field in row)) {
                issues.push(`Row ${index}: Missing required field '${field}'`);
              }
            });
          });

          return {
            valid: issues.length === 0,
            issues,
            recordCount: data.length,
            fieldCount: Object.keys(data[0] || {}).length
          };
        } catch (error) {
          return { valid: false, issues: ['Invalid JSON data'] };
        }
      }
    });

    console.log('✅ Data processing tools registered');
  }

  async createDataProcessingWorkflow(): Promise<Workflow> {
    const workflow = new Workflow('Data Processing Pipeline');

    // 1. Data Validator
    const validator = Node.agent(
      'Data Validator',
      `Validate this dataset using the validate_data tool:

Check for:
- Data completeness
- Format consistency
- Obvious errors or outliers
- Missing values

Provide validation status and issues found.
If data is invalid, suggest fixes.
Format response as JSON.`,
      'data_validator'
    );

    // 2. Statistical Analyzer
    const statsAnalyzer = Node.agent(
      'Statistical Analyzer',
      `Perform statistical analysis using calculate_statistics tool:

Calculate and provide:
- Descriptive statistics (mean, median, std dev)
- Distribution analysis
- Range and variance
- Statistical significance where applicable

Format as JSON with clear structure.`,
      'stats_analyzer'
    );

    // 3. Pattern Detector
    const patternDetector = Node.agent(
      'Pattern Detector',
      `Analyze data for patterns and anomalies using detect_outliers tool:

Identify:
- Recurring patterns
- Trends
- Anomalies and outliers (use detect_outliers tool)
- Clustering or groupings
- Correlations

Explain significance of each finding.
Format as JSON.`,
      'pattern_detector'
    );

    // 4. Insight Generator
    const insightGenerator = Node.agent(
      'Insight Generator',
      `Generate actionable insights based on analysis:

Create:
- Key business insights
- Actionable recommendations
- Risk assessments
- Opportunities identified
- Next steps

Focus on practical, implementable insights.`,
      'insight_generator'
    );

    // 5. Report Generator
    const reportGenerator = Node.agent(
      'Report Generator',
      `Create a comprehensive data analysis report:

Format as a professional report with:
- Executive summary
- Data quality assessment
- Key findings
- Statistical highlights
- Actionable recommendations
- Detailed analysis appendix

Use clear, business-friendly language.`,
      'report_generator'
    );

    // Add nodes
    await workflow.addNode(validator);
    await workflow.addNode(statsAnalyzer);
    await workflow.addNode(patternDetector);
    await workflow.addNode(insightGenerator);
    await workflow.addNode(reportGenerator);

    // Connect pipeline with multiple paths
    await workflow.connect('data_validator', 'stats_analyzer');
    await workflow.connect('data_validator', 'pattern_detector');
    await workflow.connect('stats_analyzer', 'pattern_detector');
    await workflow.connect('pattern_detector', 'insight_generator');
    await workflow.connect('stats_analyzer', 'insight_generator');
    await workflow.connect('insight_generator', 'report_generator');

    await workflow.validate();

    return workflow;
  }

  async processData(data: DataPoint[]): Promise<AnalysisReport> {
    console.log(`\n📊 Processing ${data.length} data points...\n`);

    try {
      const workflow = await this.createDataProcessingWorkflow();
      
      const result = await this.executor.execute(workflow);

      if (result.isSuccess()) {
        const vars = result.variables();

        console.log('✅ Data processing completed successfully\n');

        return {
          validation: {
            status: 'VALID',
            issues: []
          },
          statistics: this.parseJSON(vars.stats_analyzer),
          patterns: this.parseJSON(vars.pattern_detector),
          insights: this.extractInsights(vars.insight_generator),
          report: vars.report_generator || 'No report generated'
        };
      } else {
        console.error('❌ Data processing failed:', result.error());
        return {
          validation: {
            status: 'INVALID',
            issues: [result.error()]
          }
        };
      }
    } catch (error) {
      console.error('❌ Processing error:', error);
      return {
        validation: {
          status: 'INVALID',
          issues: [error instanceof Error ? error.message : 'Unknown error']
        }
      };
    }
  }

  private parseJSON(text: string | undefined): any {
    if (!text) return null;
    
    try {
      return JSON.parse(text);
    } catch {
      return { raw: text };
    }
  }

  private extractInsights(text: string | undefined): string[] {
    if (!text) return [];
    
    const lines = text.split('\n')
      .filter(line => line.trim().length > 0)
      .filter(line => /^[-*\d]/.test(line.trim()));
    
    return lines.length > 0 ? lines : [text];
  }
}

// Usage Examples

async function example1_SalesData() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 1: Sales Data Analysis');
  console.log('='.repeat(60));

  const pipeline = new DataProcessingPipeline(
    process.env.OPENAI_API_KEY || ''
  );

  await pipeline.initialize();

  const salesData: DataPoint[] = [
    { month: 'Jan', revenue: 45000, customers: 120, avgOrder: 375 },
    { month: 'Feb', revenue: 52000, customers: 135, avgOrder: 385 },
    { month: 'Mar', revenue: 48000, customers: 125, avgOrder: 384 },
    { month: 'Apr', revenue: 61000, customers: 155, avgOrder: 394 },
    { month: 'May', revenue: 58000, customers: 145, avgOrder: 400 },
    { month: 'Jun', revenue: 67000, customers: 165, avgOrder: 406 },
    { month: 'Jul', revenue: 72000, customers: 175, avgOrder: 411 },
    { month: 'Aug', revenue: 69000, customers: 170, avgOrder: 406 },
    { month: 'Sep', revenue: 63000, customers: 160, avgOrder: 394 },
    { month: 'Oct', revenue: 71000, customers: 180, avgOrder: 394 },
    { month: 'Nov', revenue: 78000, customers: 190, avgOrder: 411 },
    { month: 'Dec', revenue: 85000, customers: 200, avgOrder: 425 }
  ];

  const report = await pipeline.processData(salesData);

  console.log('\n📈 ANALYSIS RESULTS:\n');
  
  if (report.statistics) {
    console.log('Statistics:', JSON.stringify(report.statistics, null, 2));
  }

  if (report.patterns) {
    console.log('\nPatterns:', JSON.stringify(report.patterns, null, 2));
  }

  if (report.insights) {
    console.log('\n💡 Key Insights:');
    report.insights.forEach((insight, i) => {
      console.log(`${i + 1}. ${insight}`);
    });
  }

  if (report.report) {
    console.log('\n📄 Full Report:\n');
    console.log(report.report);
  }
}

async function example2_PerformanceMetrics() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 2: System Performance Metrics');
  console.log('='.repeat(60));

  const pipeline = new DataProcessingPipeline(
    process.env.OPENAI_API_KEY || ''
  );

  await pipeline.initialize();

  const metricsData: DataPoint[] = [
    { timestamp: '2025-01-01T00:00:00Z', responseTime: 45, cpuUsage: 32, memory: 1024 },
    { timestamp: '2025-01-01T01:00:00Z', responseTime: 52, cpuUsage: 38, memory: 1152 },
    { timestamp: '2025-01-01T02:00:00Z', responseTime: 48, cpuUsage: 35, memory: 1088 },
    { timestamp: '2025-01-01T03:00:00Z', responseTime: 180, cpuUsage: 85, memory: 2048 }, // Anomaly
    { timestamp: '2025-01-01T04:00:00Z', responseTime: 50, cpuUsage: 36, memory: 1120 },
    { timestamp: '2025-01-01T05:00:00Z', responseTime: 47, cpuUsage: 34, memory: 1056 }
  ];

  const report = await pipeline.processData(metricsData);

  console.log('\n⚡ PERFORMANCE ANALYSIS:\n');

  if (report.patterns) {
    console.log('Detected Patterns and Anomalies:');
    console.log(JSON.stringify(report.patterns, null, 2));
  }

  if (report.insights) {
    console.log('\n🎯 Recommendations:');
    report.insights.forEach((insight, i) => {
      console.log(`${i + 1}. ${insight}`);
    });
  }
}

async function example3_CustomAnalysis() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 3: Custom Data Analysis');
  console.log('='.repeat(60));

  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || ''
  });

  const executor = new Executor(config);

  // Custom workflow for financial data
  const workflow = new Workflow('Financial Analysis');

  const analyzer = Node.agent(
    'Financial Analyzer',
    `Analyze this financial data:

Revenue: [100000, 120000, 115000, 140000, 135000]
Expenses: [80000, 85000, 90000, 95000, 92000]

Calculate:
- Profit margins
- Growth rate
- Average metrics
- Trends

Provide financial insights and recommendations.`,
    'financial_analyzer'
  );

  const forecaster = Node.agent(
    'Financial Forecaster',
    `Based on the analysis, forecast next quarter:

Provide:
- Revenue projection
- Expected expenses
- Profit forecast
- Risk factors
- Confidence level

Use trend analysis and historical patterns.`,
    'forecaster'
  );

  await workflow.addNode(analyzer);
  await workflow.addNode(forecaster);
  await workflow.connect('financial_analyzer', 'forecaster');
  await workflow.validate();

  console.log('⏳ Running financial analysis...\n');

  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    const vars = result.variables();
    console.log('📊 Analysis:', vars.financial_analyzer);
    console.log('\n📈 Forecast:', vars.forecaster);
  } else {
    console.error('❌ Analysis failed:', result.error());
  }
}

// Main execution
async function main() {
  try {
    await example1_SalesData();
    await example2_PerformanceMetrics();
    await example3_CustomAnalysis();

    console.log('\n✅ All data processing examples completed!\n');
  } catch (error) {
    console.error('❌ Error:', error);
    process.exit(1);
  }
}

// Uncomment to run
// main().catch(console.error);

export { DataProcessingPipeline, DataPoint, AnalysisReport };
```

## Key Features

1. **Validation Pipeline**: Automated data validation and cleaning
2. **Statistical Analysis**: Built-in statistical tools
3. **Pattern Detection**: Outlier detection and trend analysis
4. **Insight Generation**: AI-powered actionable insights
5. **Report Generation**: Professional formatted reports
6. **Tool Integration**: Custom data processing tools

## Best Practices

1. **Validate data first**: Always validate before processing
2. **Use appropriate tools**: Leverage statistical tools for accuracy
3. **Handle anomalies**: Detect and explain outliers
4. **Generate actionable insights**: Focus on business value
5. **Format reports clearly**: Make findings accessible
6. **Monitor processing time**: Balance thoroughness with performance

## Performance Tips

- Batch process multiple datasets
- Cache statistical calculations
- Use streaming for large datasets
- Implement parallel processing for independent analyses
- Monitor memory usage for large data volumes

## See Also

- [Tool Calling Guide](../user-guide/tool-calling-js.md)
- [Workflow Builder Guide](../user-guide/workflow-builder-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
