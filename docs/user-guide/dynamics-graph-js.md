# Dynamic Workflow Creation - JavaScript

GraphBit's JavaScript bindings support dynamic workflow creation, allowing you to build and modify workflows at runtime based on data, conditions, and business logic. This powerful feature enables adaptive workflows that can respond to changing requirements.

## Overview

Dynamic workflow creation allows you to:
- Create workflows that adapt to input data
- Generate nodes and connections programmatically
- Modify workflow structure based on runtime conditions
- Build self-organizing processing pipelines
- Implement conditional workflow branches

## Basic Dynamic Workflow Creation

### Simple Dynamic Workflow

```typescript
import { init, Workflow, Node } from '@infinitibit_gmbh/graphbit';

interface InputData {
  type: string;
  content: string;
  [key: string]: any;
}

function detectDataType(data: InputData): string {
  if (typeof data.content === 'string' && data.content.length > 0) {
    return 'text';
  } else if (typeof data.content === 'number') {
    return 'numerical';
  } else if (data.type === 'mixed' || Array.isArray(data)) {
    return 'mixed';
  }
  return 'unknown';
}

async function createDynamicWorkflow(inputData: InputData): Promise<Workflow> {
  init();

  const dataType = detectDataType(inputData);

  switch (dataType) {
    case 'text':
      return await createTextProcessingWorkflow();
    case 'numerical':
      return await createNumericalAnalysisWorkflow();
    case 'mixed':
      return await createMixedDataWorkflow();
    default:
      return await createGenericWorkflow();
  }
}

async function createTextProcessingWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('Text Processing Workflow');

  // Text analyzer
  const analyzer = Node.agent(
    'Text Analyzer',
    'Analyze this text and provide key insights',
    'text_analyzer'
  );

  // Sentiment detector
  const sentiment = Node.agent(
    'Sentiment Detector',
    'Determine sentiment of the analyzed text',
    'sentiment_detector'
  );

  // Build text processing chain
  await workflow.addNode(analyzer);
  await workflow.addNode(sentiment);
  await workflow.connect('text_analyzer', 'sentiment_detector');

  return workflow;
}

async function createNumericalAnalysisWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('Numerical Analysis Workflow');

  // Statistical analyzer
  const stats = Node.agent(
    'Statistical Analyzer',
    'Perform statistical analysis on numerical data',
    'stats_analyzer'
  );

  // Trend detector
  const trends = Node.agent(
    'Trend Detector',
    'Identify trends in the statistical analysis',
    'trend_detector'
  );

  // Build numerical analysis chain
  await workflow.addNode(stats);
  await workflow.addNode(trends);
  await workflow.connect('stats_analyzer', 'trend_detector');

  return workflow;
}

async function createMixedDataWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('Mixed Data Workflow');

  // Data classifier
  const classifier = Node.agent(
    'Data Classifier',
    'Classify this mixed data into categories',
    'classifier'
  );

  // Multi-modal processor
  const processor = Node.agent(
    'Multi-Modal Processor',
    'Process classified data appropriately',
    'multimodal_processor'
  );

  // Build mixed data chain
  await workflow.addNode(classifier);
  await workflow.addNode(processor);
  await workflow.connect('classifier', 'multimodal_processor');

  return workflow;
}

async function createGenericWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('Generic Workflow');

  // Generic processor
  const processor = Node.agent(
    'Generic Processor',
    'Process input data with general-purpose logic',
    'generic_processor'
  );

  await workflow.addNode(processor);

  return workflow;
}

// Usage example
async function exampleBasicDynamic() {
  const inputData: InputData = {
    type: 'text',
    content: 'Sample text for processing'
  };

  const workflow = await createDynamicWorkflow(inputData);
  await workflow.validate();

  console.log(`✅ Created dynamic workflow: ${workflow}`);
}
```

## Advanced Dynamic Generation

### Data-Driven Node Creation

```typescript
import { init, Workflow, Node } from '@infinitibit_gmbh/graphbit';

interface DataSchema {
  fields: Array<{
    name: string;
    type: string;
  }>;
}

async function createDataDrivenWorkflow(schema: DataSchema): Promise<Workflow> {
  init();

  const workflow = new Workflow('Data-Driven Workflow');
  const nodeIds: Array<[string, string]> = [];

  // Create nodes based on schema fields
  for (const field of schema.fields) {
    const node = createNodeForFieldType(field.name, field.type);
    await workflow.addNode(node);
    nodeIds.push([field.name, node.agentId || field.name]);
  }

  // Create aggregator node
  const aggregator = Node.agent(
    'Data Aggregator',
    'Combine and analyze all processed fields',
    'aggregator'
  );

  await workflow.addNode(aggregator);

  // Connect all field processors to aggregator
  for (const [fieldName, nodeId] of nodeIds) {
    await workflow.connect(nodeId, 'aggregator');
  }

  return workflow;
}

function createNodeForFieldType(fieldName: string, fieldType: string): Node {
  switch (fieldType) {
    case 'string':
      return createTextProcessingNode(fieldName);
    case 'number':
      return createNumericalProcessingNode(fieldName);
    case 'date':
      return createDateProcessingNode(fieldName);
    default:
      return createGenericProcessingNode(fieldName);
  }
}

function createTextProcessingNode(fieldName: string): Node {
  return Node.agent(
    `${fieldName} Text Processor`,
    `Process ${fieldName} text field and extract insights`,
    `${fieldName}_text_processor`
  );
}

function createNumericalProcessingNode(fieldName: string): Node {
  return Node.agent(
    `${fieldName} Numerical Processor`,
    `Analyze ${fieldName} numerical data and identify patterns`,
    `${fieldName}_num_processor`
  );
}

function createDateProcessingNode(fieldName: string): Node {
  return Node.agent(
    `${fieldName} Date Processor`,
    `Analyze ${fieldName} date patterns and trends`,
    `${fieldName}_date_processor`
  );
}

function createGenericProcessingNode(fieldName: string): Node {
  return Node.agent(
    `${fieldName} Generic Processor`,
    `Process ${fieldName} field with general logic`,
    `${fieldName}_generic_processor`
  );
}

// Usage example
async function exampleDataDriven() {
  const schema: DataSchema = {
    fields: [
      { name: 'customer_name', type: 'string' },
      { name: 'order_amount', type: 'number' },
      { name: 'order_date', type: 'date' }
    ]
  };

  const workflow = await createDataDrivenWorkflow(schema);
  await workflow.validate();

  console.log('✅ Created data-driven workflow with', schema.fields.length, 'field processors');
}
```

## Adaptive Workflow Patterns

### Self-Optimizing Workflows

```typescript
import { init, Workflow, Node, Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

interface ExecutionRecord {
  timestamp: number;
  executionTimeMs: number;
  success: boolean;
  inputSize: number;
  outputSize: number;
}

interface PerformanceMetrics {
  averageExecutionTime: number;
  successRate: number;
  totalExecutions: number;
  throughput: number;
}

interface OptimizationRule {
  condition: {
    type: string;
    metric?: string;
    operator?: string;
    threshold?: number;
    count?: number;
  };
  action: {
    type: string;
  };
}

class AdaptiveWorkflow {
  private name: string;
  private workflow: Workflow;
  private executionHistory: ExecutionRecord[] = [];
  private performanceMetrics: Partial<PerformanceMetrics> = {};
  private optimizationRules: OptimizationRule[] = [];

  constructor(name: string) {
    init();
    this.name = name;
    this.workflow = new Workflow(name);
  }

  getWorkflow(): Workflow {
    return this.workflow;
  }

  addOptimizationRule(rule: OptimizationRule): void {
    this.optimizationRules.push(rule);
  }

  async executeAndAdapt(executor: Executor, inputData: any): Promise<any> {
    // Record execution start
    const startTime = Date.now();

    // Execute workflow
    const result = await executor.execute(this.workflow);

    // Record execution metrics
    const executionTime = Date.now() - startTime;

    const executionRecord: ExecutionRecord = {
      timestamp: Date.now(),
      executionTimeMs: executionTime,
      success: result.isSuccess(),
      inputSize: JSON.stringify(inputData).length,
      outputSize: result.isSuccess() ? JSON.stringify(result.variables()).length : 0
    };

    this.executionHistory.push(executionRecord);

    // Update performance metrics
    this.updatePerformanceMetrics();

    // Apply optimization rules
    this.applyOptimizations();

    return result;
  }

  private updatePerformanceMetrics(): void {
    if (this.executionHistory.length === 0) {
      return;
    }

    const recentExecutions = this.executionHistory.slice(-10); // Last 10 executions

    const totalTime = recentExecutions.reduce((sum, e) => sum + e.executionTimeMs, 0);
    const successCount = recentExecutions.filter(e => e.success).length;

    this.performanceMetrics = {
      averageExecutionTime: totalTime / recentExecutions.length,
      successRate: successCount / recentExecutions.length,
      totalExecutions: this.executionHistory.length,
      throughput: recentExecutions.length > 1
        ? recentExecutions.length / ((recentExecutions[recentExecutions.length - 1].timestamp - recentExecutions[0].timestamp) / 1000)
        : 0
    };
  }

  private applyOptimizations(): void {
    for (const rule of this.optimizationRules) {
      if (this.evaluateCondition(rule.condition)) {
        this.executeAction(rule.action);
      }
    }
  }

  private evaluateCondition(condition: OptimizationRule['condition']): boolean {
    const metrics = this.performanceMetrics;

    if (condition.type === 'performance_threshold' && condition.metric && condition.operator && condition.threshold !== undefined) {
      const metricValue = (metrics as any)[condition.metric] || 0;
      return this.compareValues(metricValue, condition.operator, condition.threshold);
    } else if (condition.type === 'execution_count' && condition.count !== undefined) {
      return (metrics.totalExecutions || 0) >= condition.count;
    }

    return false;
  }

  private compareValues(value: number, operator: string, threshold: number): boolean {
    switch (operator) {
      case '>': return value > threshold;
      case '<': return value < threshold;
      case '>=': return value >= threshold;
      case '<=': return value <= threshold;
      case '==': return value === threshold;
      default: return false;
    }
  }

  private executeAction(action: OptimizationRule['action']): void {
    switch (action.type) {
      case 'add_caching_layer':
        this.addCachingLayer();
        break;
      case 'add_parallel_processing':
        this.addParallelProcessing();
        break;
      case 'optimize_prompts':
        this.optimizePrompts();
        break;
    }
  }

  private async addCachingLayer(): Promise<void> {
    const cacheNode = Node.agent(
      'Cache Manager',
      'Check cache for input. If found, return cached result, otherwise process normally.',
      'cache_manager'
    );

    await this.workflow.addNode(cacheNode);
    console.log(`✅ Added caching layer to workflow ${this.name}`);
  }

  private async addParallelProcessing(): Promise<void> {
    const parallelProcessor = Node.agent(
      'Parallel Processor',
      'Process input in parallel for improved performance',
      'parallel_proc'
    );

    await this.workflow.addNode(parallelProcessor);
    console.log(`✅ Added parallel processing to workflow ${this.name}`);
  }

  private optimizePrompts(): void {
    console.log(`✅ Optimized prompts for workflow ${this.name}`);
  }

  getMetrics(): Partial<PerformanceMetrics> {
    return this.performanceMetrics;
  }
}

// Usage example
async function createAdaptiveTextProcessor(): Promise<AdaptiveWorkflow> {
  const adaptiveWorkflow = new AdaptiveWorkflow('Adaptive Text Processor');

  // Build initial workflow
  const processor = Node.agent(
    'Text Processor',
    'Process and analyze this text input',
    'text_proc'
  );

  await adaptiveWorkflow.getWorkflow().addNode(processor);

  // Add optimization rules
  adaptiveWorkflow.addOptimizationRule({
    condition: {
      type: 'performance_threshold',
      metric: 'averageExecutionTime',
      operator: '>',
      threshold: 5000 // 5 seconds
    },
    action: {
      type: 'add_caching_layer'
    }
  });

  adaptiveWorkflow.addOptimizationRule({
    condition: {
      type: 'execution_count',
      count: 10
    },
    action: {
      type: 'optimize_prompts'
    }
  });

  return adaptiveWorkflow;
}
```

## Dynamic Workflow Templates

### Template-Based Generation

```typescript
import { init, Workflow, Node } from '@infinitibit_gmbh/graphbit';

interface NodeConfig {
  id: string;
  type: string;
  name: string;
  prompt: string;
  agentId: string;
}

interface ConnectionConfig {
  source: string;
  target: string;
}

interface TemplateStructure {
  nodes: NodeConfig[];
  connections: ConnectionConfig[];
}

class WorkflowTemplate {
  private templateName: string;
  private templateStructure: Partial<TemplateStructure> = {};
  private parameterMappings: Record<string, string> = {};

  constructor(templateName: string) {
    init();
    this.templateName = templateName;
  }

  defineTemplate(structure: TemplateStructure, parameterMappings: Record<string, string>): void {
    this.templateStructure = structure;
    this.parameterMappings = parameterMappings;
  }

  async instantiate(parameters: Record<string, string>): Promise<Workflow> {
    const instanceId = parameters.instance_id || 'default';
    const workflow = new Workflow(`${this.templateName}_${instanceId}`);

    const nodeMap: Record<string, string> = {};

    // Create nodes from template
    for (const nodeConfig of this.templateStructure.nodes || []) {
      const node = this.createNodeFromTemplate(nodeConfig, parameters);
      await workflow.addNode(node);
      nodeMap[nodeConfig.id] = nodeConfig.agentId;
    }

    // Create connections from template
    for (const connection of this.templateStructure.connections || []) {
      const sourceId = nodeMap[connection.source];
      const targetId = nodeMap[connection.target];

      if (sourceId && targetId) {
        await workflow.connect(sourceId, targetId);
      }
    }

    return workflow;
  }

  private createNodeFromTemplate(nodeConfig: NodeConfig, parameters: Record<string, string>): Node {
    // Replace template parameters in prompt
    let prompt = nodeConfig.prompt;
    for (const [param, value] of Object.entries(parameters)) {
      prompt = prompt.replace(new RegExp(`\\$\\{${param}\\}`, 'g'), value);
    }

    if (nodeConfig.type === 'agent') {
      return Node.agent(
        nodeConfig.name,
        prompt,
        nodeConfig.agentId
      );
    }

    throw new Error(`Unknown node type: ${nodeConfig.type}`);
  }
}

function createDataProcessingTemplate(): WorkflowTemplate {
  const template = new WorkflowTemplate('Data Processing Template');

  const templateStructure: TemplateStructure = {
    nodes: [
      {
        id: 'validator',
        type: 'agent',
        name: '${domain} Data Validator',
        prompt: 'Validate ${domain} data according to ${validation_rules}',
        agentId: 'validator'
      },
      {
        id: 'processor',
        type: 'agent',
        name: '${domain} Processor',
        prompt: 'Process validated data using ${processing_method}',
        agentId: 'processor'
      },
      {
        id: 'quality_check',
        type: 'agent',
        name: 'Quality Gate',
        prompt: 'Verify data quality meets threshold: ${quality_threshold}',
        agentId: 'quality_check'
      },
      {
        id: 'formatter',
        type: 'agent',
        name: 'Output Formatter',
        prompt: 'Format output according to: ${output_format}',
        agentId: 'formatter'
      }
    ],
    connections: [
      { source: 'validator', target: 'processor' },
      { source: 'processor', target: 'quality_check' },
      { source: 'quality_check', target: 'formatter' }
    ]
  };

  const parameterMappings = {
    domain: 'Application domain (e.g., financial, medical, scientific)',
    validation_rules: 'Specific validation rules for the domain',
    processing_method: 'Method used for processing data',
    quality_threshold: 'Minimum quality score threshold',
    output_format: 'Format for output transformation'
  };

  template.defineTemplate(templateStructure, parameterMappings);

  return template;
}

// Usage example
async function createWorkflowsFromTemplate(): Promise<Record<string, Workflow>> {
  const template = createDataProcessingTemplate();

  // Financial data processing workflow
  const financialWorkflow = await template.instantiate({
    instance_id: 'financial',
    domain: 'financial',
    validation_rules: 'GAAP compliance and data integrity checks',
    processing_method: 'financial analysis algorithms',
    quality_threshold: '0.95',
    output_format: 'JSON with financial metrics'
  });

  // Medical data processing workflow
  const medicalWorkflow = await template.instantiate({
    instance_id: 'medical',
    domain: 'medical',
    validation_rules: 'HIPAA compliance and medical data standards',
    processing_method: 'clinical analysis procedures',
    quality_threshold: '0.98',
    output_format: 'HL7 FHIR format'
  });

  return {
    financial: financialWorkflow,
    medical: medicalWorkflow
  };
}
```

## Configuration-Driven Workflows

### JSON-Based Workflow Definition

```typescript
import { init, Workflow, Node } from '@infinitibit_gmbh/graphbit';

interface WorkflowConfig {
  name: string;
  nodes: NodeConfig[];
  connections: ConnectionConfig[];
}

async function createWorkflowFromJSON(jsonConfig: string | WorkflowConfig): Promise<Workflow> {
  init();

  const config: WorkflowConfig = typeof jsonConfig === 'string'
    ? JSON.parse(jsonConfig)
    : jsonConfig;

  const workflow = new Workflow(config.name || 'JSON Workflow');
  const nodeMap: Record<string, string> = {};

  // Create nodes from configuration
  for (const nodeConfig of config.nodes) {
    const node = createNodeFromJSON(nodeConfig);
    await workflow.addNode(node);
    nodeMap[nodeConfig.id] = nodeConfig.agentId;
  }

  // Create connections from configuration
  for (const connection of config.connections) {
    const sourceId = nodeMap[connection.source];
    const targetId = nodeMap[connection.target];

    if (sourceId && targetId) {
      await workflow.connect(sourceId, targetId);
    }
  }

  return workflow;
}

function createNodeFromJSON(nodeConfig: NodeConfig): Node {
  if (nodeConfig.type === 'agent') {
    return Node.agent(
      nodeConfig.name || 'Agent',
      nodeConfig.prompt || 'Process input data',
      nodeConfig.agentId || 'agent'
    );
  }

  throw new Error(`Unknown node type: ${nodeConfig.type}`);
}

// Example JSON configurations
function getExampleWorkflowConfigs(): Record<string, WorkflowConfig> {
  const simpleConfig: WorkflowConfig = {
    name: 'Simple Analysis Workflow',
    nodes: [
      {
        id: 'analyzer',
        type: 'agent',
        name: 'Data Analyzer',
        prompt: 'Analyze this data and provide insights',
        agentId: 'analyzer'
      },
      {
        id: 'formatter',
        type: 'agent',
        name: 'Output Formatter',
        prompt: 'Format the analysis in a clear structure',
        agentId: 'formatter'
      }
    ],
    connections: [
      { source: 'analyzer', target: 'formatter' }
    ]
  };

  const complexConfig: WorkflowConfig = {
    name: 'Complex Processing Workflow',
    nodes: [
      {
        id: 'input_processor',
        type: 'agent',
        name: 'Input Processor',
        prompt: 'Process and prepare input data',
        agentId: 'input_proc'
      },
      {
        id: 'quality_check',
        type: 'agent',
        name: 'Quality Gate',
        prompt: 'Verify data quality score > 0.8',
        agentId: 'quality_check'
      },
      {
        id: 'high_quality_processor',
        type: 'agent',
        name: 'High Quality Processor',
        prompt: 'Process high-quality data with advanced algorithms',
        agentId: 'hq_proc'
      },
      {
        id: 'enhancement_processor',
        type: 'agent',
        name: 'Enhancement Processor',
        prompt: 'Enhance and improve lower-quality data',
        agentId: 'enhancement_proc'
      },
      {
        id: 'aggregator',
        type: 'agent',
        name: 'Result Aggregator',
        prompt: 'Combine processing results from both paths',
        agentId: 'aggregator'
      }
    ],
    connections: [
      { source: 'input_processor', target: 'quality_check' },
      { source: 'quality_check', target: 'high_quality_processor' },
      { source: 'quality_check', target: 'enhancement_processor' },
      { source: 'high_quality_processor', target: 'aggregator' },
      { source: 'enhancement_processor', target: 'aggregator' }
    ]
  };

  return {
    simple: simpleConfig,
    complex: complexConfig
  };
}
```

## Best Practices

### Error Handling and Validation

```typescript
import { Workflow } from '@infinitibit_gmbh/graphbit';

async function validateDynamicWorkflow(workflow: Workflow): Promise<boolean> {
  try {
    await workflow.validate();
    console.log('✅ Dynamic workflow validation passed');
    return true;
  } catch (error) {
    console.error('❌ Dynamic workflow validation failed:', error);
    return false;
  }
}

async function safeDynamicWorkflowCreation(
  creationFunc: () => Promise<Workflow>
): Promise<Workflow> {
  try {
    const workflow = await creationFunc();

    if (await validateDynamicWorkflow(workflow)) {
      return workflow;
    } else {
      throw new Error('Dynamic workflow validation failed');
    }
  } catch (error) {
    console.error('Error creating dynamic workflow:', error);

    // Return a simple fallback workflow
    const fallbackWorkflow = new Workflow('Fallback Workflow');
    const fallbackNode = Node.agent(
      'Fallback Processor',
      'Process input safely with fallback logic',
      'fallback'
    );
    await fallbackWorkflow.addNode(fallbackNode);

    return fallbackWorkflow;
  }
}
```

## Complete Example

### Full Dynamic Workflow System

```typescript
import { init, LlmConfig, Executor, Workflow } from '@infinitibit_gmbh/graphbit';

async function completeExampleDynamicWorkflow() {
  init();

  // Create dynamic workflow based on input
  const inputData = {
    type: 'mixed',
    content: 'Sample text with numerical data: 123, 456',
    requirements: ['quality_check', 'fast_processing']
  };

  console.log('🔧 Creating dynamic workflow...\n');

  // Create workflow dynamically
  const workflow = await createDynamicWorkflow(inputData);

  // Validate the workflow
  if (await validateDynamicWorkflow(workflow)) {
    console.log('✅ Dynamic workflow created and validated successfully\n');

    // Create executor
    const llmConfig = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY || '',
      model: 'gpt-4o-mini'
    });

    const executor = new Executor(llmConfig);

    console.log('⏳ Executing dynamic workflow...\n');

    // Execute workflow
    const result = await executor.execute(workflow);

    if (result.isSuccess()) {
      console.log('✅ Dynamic workflow executed successfully');
      console.log(`⏱️  Execution time: ${result.executionTimeMs()}ms`);
      console.log('\n📊 Results:');
      console.log(JSON.stringify(result.variables(), null, 2));
    } else {
      console.error('❌ Dynamic workflow execution failed:', result.error());
    }
  } else {
    console.error('❌ Dynamic workflow validation failed');
  }
}

// Run the complete example
completeExampleDynamicWorkflow().catch(console.error);
```

## Key Takeaways

1. **Flexibility**: Build workflows that adapt to different input types
2. **Modularity**: Create reusable workflow components and templates
3. **Validation**: Always validate dynamically created workflows
4. **Error Handling**: Implement robust error handling and fallbacks
5. **Performance**: Monitor and optimize dynamic workflow execution
6. **Templates**: Use templates for consistent workflow generation

## See Also

- [Workflow Builder Guide](workflow-builder-js.md)
- [Performance Optimization](performance-js.md)
- [Monitoring and Observability](monitoring-js.md)
- [Validation Patterns](validation-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
