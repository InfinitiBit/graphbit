# Node Types Reference - JavaScript

GraphBit workflows are built using different types of nodes, each serving a specific purpose. This reference covers all available node types in the JavaScript/TypeScript bindings and their usage patterns.

## Node Type Categories

1. **Agent Nodes** - AI-powered processing nodes

## Agent Nodes

Agent nodes are the core AI-powered components that interact with LLM providers.

### Basic Agent Node

```typescript
import { Node } from '@infinitibit_gmbh/graphbit';

const agent = Node.agent(
  'Content Analyzer',
  'Analyze the following content and provide insights',
  'analyzer'  // Optional, auto-generated if not provided
);
```

**Parameters:**
- `name` (string): Human-readable node name
- `prompt` (string): LLM prompt template with variable placeholders
- `agentId` (string, optional): Unique identifier for the agent. Auto-generated if not provided

### Agent Node with Tools

```typescript
import { Node, ToolRegistry } from '@infinitibit_gmbh/graphbit';

const toolRegistry = new ToolRegistry();

// Register a tool
await toolRegistry.register({
  name: 'get_weather',
  description: 'Get weather forecast for a location',
  inputSchema: {
    type: 'object',
    properties: {
      location: { type: 'string', description: 'City name' }
    },
    required: ['location']
  },
  handler: async (params: any) => {
    return { temperature: 72, condition: 'sunny' };
  }
});

const agent = Node.agent(
  'Weather Agent',
  'Using the get_weather tool, get the weather forecast for the specified location',
  'weather_agent'
);
```

### Agent Node with System Prompt

GraphBit's system prompts are defined through the LLM configuration rather than per-node in the JavaScript bindings. For behavior control, use detailed prompts:

```typescript
import { Node } from '@infinitibit_gmbh/graphbit';

// Agent with detailed prompt for behavior control
const analyzer = Node.agent(
  'Code Reviewer',
  `You are an experienced software engineer and code reviewer.

Review this code for issues:

Focus on:
- Security vulnerabilities
- Performance issues
- Code quality and best practices
- Potential bugs

Provide specific, actionable feedback with examples.`,
  'code_reviewer'
);

// Agent with structured output format
const jsonAgent = Node.agent(
  'Sentiment Analyzer',
  `Analyze sentiment and respond only in valid JSON format:
{
  "sentiment": "positive|negative|neutral",
  "confidence": 0.0-1.0,
  "reasoning": "brief explanation"
}`,
  'sentiment_analyzer'
);
```

### Agent Node with Custom LLM Config

```typescript
import { LlmConfig, Node, Workflow } from '@infinitibit_gmbh/graphbit';

// Configure LLM providers
const openaiConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini'
});

const anthropicConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || '',
  model: 'claude-sonnet-4-20250514'
});

// Create agents (LLM config is set at executor level in JS bindings)
const codeReviewer = Node.agent(
  'Code Reviewer',
  'Review this code for issues',
  'code_reviewer'
);

const sentimentAnalyzer = Node.agent(
  'Sentiment Analyzer',
  'Analyze sentiment of the provided text',
  'sentiment_analyzer'
);
```

### Agent Node Examples

#### Text Analysis Agent

```typescript
const sentimentAnalyzer = Node.agent(
  'Sentiment Analyzer',
  `Analyze the sentiment of this text: "{text}"

Provide:
- Overall sentiment (positive/negative/neutral)
- Confidence score (0-1)
- Key emotional indicators`,
  'sentiment_analyzer'
);
```

#### Code Review Agent

```typescript
const codeReviewer = Node.agent(
  'Code Reviewer',
  `Review this code for quality and security issues:

{code}

Check for:
- Security vulnerabilities
- Performance issues
- Code style problems
- Best practices violations`,
  'code_reviewer'
);
```

#### Data Processing Agent

```typescript
const dataProcessor = Node.agent(
  'Data Processor',
  `Process this dataset and provide insights:

Data: {dataset}

Include:
1. Statistical summary
2. Key trends
3. Anomalies
4. Recommendations`,
  'data_processor'
);
```

#### Content Generation Agent

```typescript
const contentWriter = Node.agent(
  'Content Writer',
  `Write engaging content about: {topic}

Requirements:
- Target audience: {audience}
- Tone: {tone}
- Length: {wordCount} words
- Include call-to-action`,
  'content_writer'
);
```

#### Research Assistant Agent

```typescript
const researchAssistant = Node.agent(
  'Research Assistant',
  `Research the following topic: {researchTopic}

Provide:
- Key findings (3-5 points)
- Supporting evidence
- Potential implications
- Areas for further investigation

Focus on: {focusArea}`,
  'research_assistant'
);
```

## Node Connection Patterns

### Sequential Connections

Connect nodes for sequential processing:

```typescript
import { Workflow, Node } from '@infinitibit_gmbh/graphbit';

async function createSequentialWorkflow() {
  const workflow = new Workflow('Sequential Pipeline');

  // Add nodes
  const inputProcessor = Node.agent('Input Processor', 'Process input', 'input');
  const analyzer = Node.agent('Analyzer', 'Analyze processed data', 'analyzer');
  const outputFormatter = Node.agent('Output Formatter', 'Format results', 'formatter');

  await workflow.addNode(inputProcessor);
  await workflow.addNode(analyzer);
  await workflow.addNode(outputFormatter);

  // Connect sequentially
  await workflow.connect('input', 'analyzer');
  await workflow.connect('analyzer', 'formatter');

  return workflow;
}
```

### Parallel Processing

Process multiple branches simultaneously:

```typescript
async function createParallelWorkflow() {
  const workflow = new Workflow('Parallel Processing');

  // Add input and processors
  const inputProcessor = Node.agent('Input Processor', 'Process input', 'input');
  const sentimentAnalyzer = Node.agent('Sentiment Analyzer', 'Analyze sentiment', 'sentiment');
  const topicExtractor = Node.agent('Topic Extractor', 'Extract topics', 'topics');
  const summaryGenerator = Node.agent('Summary Generator', 'Generate summary', 'summary');
  const resultAggregator = Node.agent('Result Aggregator', 'Combine results', 'aggregator');

  await workflow.addNode(inputProcessor);
  await workflow.addNode(sentimentAnalyzer);
  await workflow.addNode(topicExtractor);
  await workflow.addNode(summaryGenerator);
  await workflow.addNode(resultAggregator);

  // Fan-out to parallel processors
  await workflow.connect('input', 'sentiment');
  await workflow.connect('input', 'topics');
  await workflow.connect('input', 'summary');

  // Fan-in to aggregator
  await workflow.connect('sentiment', 'aggregator');
  await workflow.connect('topics', 'aggregator');
  await workflow.connect('summary', 'aggregator');

  return workflow;
}
```

## Advanced Node Patterns

### Error Handling Pattern

```typescript
import { Node, Workflow } from '@infinitibit_gmbh/graphbit';

async function createErrorHandlingWorkflow() {
  const workflow = new Workflow('Error Handling');

  // Main processor
  const mainProcessor = Node.agent(
    'Main Processor',
    'Process the input data',
    'main'
  );

  // Error handler
  const errorHandler = Node.agent(
    'Error Handler',
    'Handle any errors that occurred during processing',
    'error_handler'
  );

  // Success handler
  const successHandler = Node.agent(
    'Success Handler',
    'Finalize successful result',
    'success_handler'
  );

  // Build error handling flow
  await workflow.addNode(mainProcessor);
  await workflow.addNode(errorHandler);
  await workflow.addNode(successHandler);

  await workflow.connect('main', 'error_handler');     // Error path
  await workflow.connect('error_handler', 'success_handler'); // Success path

  return workflow;
}
```

### Multi-Step Analysis Pipeline

```typescript
import { Node, Workflow } from '@infinitibit_gmbh/graphbit';

async function createAnalysisPipeline() {
  const workflow = new Workflow('Multi-Step Analysis');

  // Step 1: Initial analysis
  const initialAnalyzer = Node.agent(
    'Initial Analyzer',
    'Perform initial analysis of the content',
    'initial_analyzer'
  );

  // Step 2: Deep analysis
  const deepAnalyzer = Node.agent(
    'Deep Analyzer',
    'Perform deep analysis based on initial findings',
    'deep_analyzer'
  );

  // Step 3: Final report
  const reportGenerator = Node.agent(
    'Report Generator',
    'Generate comprehensive analysis report',
    'report_generator'
  );

  // Connect the pipeline
  await workflow.addNode(initialAnalyzer);
  await workflow.addNode(deepAnalyzer);
  await workflow.addNode(reportGenerator);

  await workflow.connect('initial_analyzer', 'deep_analyzer');
  await workflow.connect('deep_analyzer', 'report_generator');

  return workflow;
}
```

## TypeScript Type Definitions

### Node Interface

```typescript
interface NodeOptions {
  name: string;
  prompt: string;
  agentId?: string;
}

// Node creation
const node = Node.agent(
  options.name,
  options.prompt,
  options.agentId
);
```

### Workflow with Typed Nodes

```typescript
interface WorkflowNodes {
  input: Node;
  processor: Node;
  output: Node;
}

async function createTypedWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('Typed Workflow');

  const nodes: WorkflowNodes = {
    input: Node.agent('Input Handler', 'Process input', 'input'),
    processor: Node.agent('Data Processor', 'Process data', 'processor'),
    output: Node.agent('Output Formatter', 'Format output', 'output')
  };

  // Add all nodes
  for (const node of Object.values(nodes)) {
    await workflow.addNode(node);
  }

  // Connect nodes
  await workflow.connect('input', 'processor');
  await workflow.connect('processor', 'output');

  return workflow;
}
```

## Best Practices

### 1. Descriptive Names

Use clear, descriptive names for all nodes:

```typescript
// Good
const emailSentimentAnalyzer = Node.agent(
  'Email Sentiment Analyzer',
  'Analyze sentiment of customer emails',
  'email_sentiment'
);

// Avoid
const node1 = Node.agent(
  'Node1',
  'Do something',
  'n1'
);
```

### 2. Single Responsibility

Each node should have one clear purpose:

```typescript
// Good - focused on one task
const spamDetector = Node.agent(
  'Spam Detector',
  'Determine if this email is spam',
  'spam_detector'
);

// Avoid - too many responsibilities
const everythingProcessor = Node.agent(
  'Everything Processor',
  'Do everything with the input',
  'everything'
);
```

### 3. Node Types

- **Agent Nodes**: AI/LLM processing tasks

### 4. Error Handling

Include appropriate error handling and validation:

```typescript
async function safeWorkflowExecution(executor: Executor, workflow: Workflow) {
  try {
    // Validate workflow before execution
    await workflow.validate();

    // Execute with timeout
    const result = await executor.execute(workflow);

    if (result.isSuccess()) {
      return result.variables();
    } else {
      console.error('Workflow failed:', result.error());
      return null;
    }
  } catch (error) {
    console.error('Execution error:', error);
    return null;
  }
}
```

### 5. Clear Prompt Design

Write clear, specific prompts for agent nodes:

```typescript
// Good - specific and clear
const summarizer = Node.agent(
  'Document Summarizer',
  `Summarize this document in exactly 3 paragraphs:

Document: {documentContent}

Requirements:
- Paragraph 1: Main topic and purpose
- Paragraph 2: Key findings or arguments
- Paragraph 3: Conclusions and implications`,
  'summarizer'
);

// Avoid - vague and unclear
const badSummarizer = Node.agent(
  'Summarizer',
  'Summarize this',
  'summarizer'
);
```

## Node Reusability

### Creating Node Factories

```typescript
interface NodeConfig {
  name: string;
  prompt: string;
  agentId: string;
}

function createAnalyzerNode(config: NodeConfig): Node {
  return Node.agent(
    config.name,
    config.prompt,
    config.agentId
  );
}

// Usage
const sentimentNode = createAnalyzerNode({
  name: 'Sentiment Analyzer',
  prompt: 'Analyze sentiment of: {text}',
  agentId: 'sentiment'
});

const topicNode = createAnalyzerNode({
  name: 'Topic Analyzer',
  prompt: 'Extract topics from: {text}',
  agentId: 'topics'
});
```

### Template-Based Nodes

```typescript
function createProcessorNode(
  taskName: string,
  taskDescription: string
): Node {
  return Node.agent(
    `${taskName} Processor`,
    `Process ${taskDescription}

Provide:
- Analysis
- Key findings
- Recommendations`,
    taskName.toLowerCase().replace(/\s+/g, '_')
  );
}

// Create multiple similar nodes
const emailProcessor = createProcessorNode('Email', 'customer emails');
const reviewProcessor = createProcessorNode('Review', 'product reviews');
const feedbackProcessor = createProcessorNode('Feedback', 'user feedback');
```

## Complete Workflow Example

```typescript
import { init, LlmConfig, Executor, Workflow, Node } from '@infinitibit_gmbh/graphbit';

async function completeExample() {
  // Initialize
  init();

  // Configure LLM
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Create executor
  const executor = new Executor(config);

  // Create workflow
  const workflow = new Workflow('Content Analysis Pipeline');

  // Define nodes
  const classifier = Node.agent(
    'Content Classifier',
    'Classify this content by type and sentiment',
    'classifier'
  );

  const analyzer = Node.agent(
    'Content Analyzer',
    'Analyze the classified content in detail',
    'analyzer'
  );

  const reporter = Node.agent(
    'Report Generator',
    'Generate a comprehensive report from the analysis',
    'reporter'
  );

  // Build workflow
  await workflow.addNode(classifier);
  await workflow.addNode(analyzer);
  await workflow.addNode(reporter);

  await workflow.connect('classifier', 'analyzer');
  await workflow.connect('analyzer', 'reporter');

  // Validate
  await workflow.validate();

  // Execute
  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    const vars = result.variables();
    console.log('Classification:', vars.classifier);
    console.log('Analysis:', vars.analyzer);
    console.log('Report:', vars.reporter);
  } else {
    console.error('Failed:', result.error());
  }
}
```

Understanding these node types and their usage patterns in JavaScript/TypeScript enables you to build sophisticated, reliable workflows that handle complex AI processing tasks effectively. Choose appropriate node types for each step in your workflow, and connect them in logical patterns to achieve your processing goals.

## See Also

- [Configuration Options](configuration-js.md)
- [JavaScript API Reference](javascript-api.md)
- [Workflow Builder Guide](../user-guide/workflow-builder-js.md)
- [Tool Calling Guide](../user-guide/tool-calling-js.md)
