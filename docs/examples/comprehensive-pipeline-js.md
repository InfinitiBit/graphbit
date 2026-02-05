# Comprehensive AI Pipeline - JavaScript

This example demonstrates a complete AI pipeline using GraphBit's JavaScript bindings, combining workflow orchestration, LLM integration, and tool management for building sophisticated AI applications.

## Overview

We'll build an intelligent document analysis and recommendation system that:
1. **Processes** documents with semantic understanding
2. **Analyzes** content using LLM workflows
3. **Generates** intelligent recommendations
4. **Monitors** system performance and health
5. **Handles** errors gracefully with fallback strategies

## Complete System Implementation

```typescript
import {
  init,
  LlmConfig,
  Executor,
  Workflow,
  Node,
  ToolRegistry,
  getSystemInfo,
  healthCheck
} from '@infinitibit_gmbh/graphbit';

interface Document {
  id: string;
  title: string;
  content: string;
  category: string;
  metadata: Record<string, any>;
}

interface AnalysisResult {
  documentId: string;
  summary: string;
  keyTopics: string[];
  sentiment: string;
  qualityScore: number;
  recommendations: string[];
}

class IntelligentDocumentPipeline {
  private executors: Record<string, Executor>;
  private workflows: Record<string, Workflow>;
  private toolRegistry: ToolRegistry;
  private documents: Document[] = [];
  private analysisResults: Map<string, AnalysisResult> = new Map();

  constructor(
    private openaiApiKey: string,
    private anthropicApiKey?: string
  ) {
    init();

    // Initialize LLM configurations
    const configs = {
      openai: LlmConfig.openai({
        apiKey: openaiApiKey,
        model: 'gpt-4o-mini'
      }),
      openai_fast: LlmConfig.openai({
        apiKey: openaiApiKey,
        model: 'gpt-4o-mini'
      })
    };

    if (anthropicApiKey) {
      const anthropicConfig = LlmConfig.anthropic({
        apiKey: anthropicApiKey,
        model: 'claude-3-5-sonnet-20241022'
      });
    }

    // Create executors for different use cases
    this.executors = {
      analysis: new Executor(configs.openai),
      batch: new Executor(configs.openai_fast),
      fast: Executor.newLowLatency(configs.openai_fast)
    };

    // Initialize tool registry
    this.toolRegistry = new ToolRegistry();

    // Create workflows
    this.workflows = {};
  }

  async initialize(): Promise<void> {
    await this.registerTools();
    await this.createWorkflows();
    console.log('✅ Pipeline initialized successfully');
  }

  private async registerTools(): Promise<void> {
    // Register document retrieval tool
    await this.toolRegistry.register({
      name: 'get_document',
      description: 'Retrieve a document by ID',
      inputSchema: {
        type: 'object',
        properties: {
          documentId: { type: 'string', description: 'Document ID' }
        },
        required: ['documentId']
      },
      handler: async (params: any) => {
        const doc = this.documents.find(d => d.id === params.documentId);
        return doc || { error: 'Document not found' };
      }
    });

    // Register document search tool
    await this.toolRegistry.register({
      name: 'search_documents',
      description: 'Search documents by keyword or category',
      inputSchema: {
        type: 'object',
        properties: {
          query: { type: 'string', description: 'Search query' },
          category: { type: 'string', description: 'Filter by category' }
        }
      },
      handler: async (params: any) => {
        let results = this.documents;

        if (params.category) {
          results = results.filter(d => 
            d.category.toLowerCase() === params.category.toLowerCase()
          );
        }

        if (params.query) {
          const query = params.query.toLowerCase();
          results = results.filter(d =>
            d.title.toLowerCase().includes(query) ||
            d.content.toLowerCase().includes(query)
          );
        }

        return results.map(d => ({
          id: d.id,
          title: d.title,
          category: d.category
        }));
      }
    });

    // Register quality scoring tool
    await this.toolRegistry.register({
      name: 'calculate_quality_score',
      description: 'Calculate quality score for document analysis',
      inputSchema: {
        type: 'object',
        properties: {
          factorsJson: { type: 'string', description: 'JSON string of quality factors' }
        },
        required: ['factorsJson']
      },
      handler: async (params: any) => {
        try {
          const factors = JSON.parse(params.factorsJson);
          const weights = {
            clarity: 0.25,
            depth: 0.25,
            accuracy: 0.30,
            relevance: 0.20
          };

          let score = 0;
          for (const [key, value] of Object.entries(factors)) {
            if (key in weights) {
              score += (value as number) * weights[key as keyof typeof weights];
            }
          }

          return {
            overallScore: Math.round(score * 10) / 10,
            breakdown: factors,
            weights
          };
        } catch (error) {
          return { error: 'Invalid factors JSON' };
        }
      }
    });

    console.log('✅ Tools registered');
  }

  private async createWorkflows(): Promise<void> {
    // 1. Document Analysis Workflow
    this.workflows.analysis = await this.createDocumentAnalysisWorkflow();

    // 2. Content Enhancement Workflow
    this.workflows.enhancement = await this.createContentEnhancementWorkflow();

    // 3. Recommendation Generation Workflow
    this.workflows.recommendation = await this.createRecommendationWorkflow();

    console.log('✅ Workflows created');
  }

  private async createDocumentAnalysisWorkflow(): Promise<Workflow> {
    const workflow = new Workflow('Document Analysis Pipeline');

    // Content Preprocessor
    const preprocessor = Node.agent(
      'Content Preprocessor',
      `Preprocess this document for analysis:

Tasks:
1. Extract key information and structure
2. Identify main topics and themes
3. Note any special content (data, quotes, references)
4. Assess content complexity and readability
5. Identify potential quality issues

Provide structured preprocessing results in JSON format.`,
      'preprocessor'
    );

    // Content Analyzer
    const analyzer = Node.agent(
      'Content Analyzer',
      `Analyze this preprocessed document content:

Perform comprehensive analysis:
1. **Topic Analysis**: Identify and rank key topics (max 5)
2. **Sentiment Analysis**: Determine overall sentiment
3. **Content Quality**: Rate quality 1-10
4. **Key Insights**: Extract 3-5 main insights
5. **Content Type**: Classify the content type

Format response as JSON with clear sections.`,
      'analyzer'
    );

    // Summary Generator
    const summarizer = Node.agent(
      'Summary Generator',
      `Generate a comprehensive summary:

Create:
- Executive summary (2-3 sentences)
- Key points (bullet list)
- Main takeaways (3-5 items)
- Action items if applicable

Keep it concise and actionable.`,
      'summarizer'
    );

    // Quality Assessor
    const assessor = Node.agent(
      'Quality Assessor',
      `Assess the quality of this analysis:

Use the calculate_quality_score tool to compute final score based on:
- Clarity (1-10)
- Depth (1-10)
- Accuracy (1-10)
- Relevance (1-10)

Provide detailed quality report with recommendations.`,
      'assessor'
    );

    await workflow.addNode(preprocessor);
    await workflow.addNode(analyzer);
    await workflow.addNode(summarizer);
    await workflow.addNode(assessor);

    await workflow.connect('preprocessor', 'analyzer');
    await workflow.connect('analyzer', 'summarizer');
    await workflow.connect('summarizer', 'assessor');

    await workflow.validate();

    return workflow;
  }

  private async createContentEnhancementWorkflow(): Promise<Workflow> {
    const workflow = new Workflow('Content Enhancement Pipeline');

    const enhancer = Node.agent(
      'Content Enhancer',
      `Enhance this content by:
1. Improving clarity and readability
2. Adding relevant context
3. Strengthening key arguments
4. Fixing any inconsistencies
5. Optimizing structure

Maintain original meaning while improving quality.`,
      'enhancer'
    );

    const validator = Node.agent(
      'Enhancement Validator',
      `Validate the enhanced content:
- Verify improvements
- Check for accuracy
- Confirm readability improvements
- Assess overall enhancement quality

Provide validation report.`,
      'validator'
    );

    await workflow.addNode(enhancer);
    await workflow.addNode(validator);
    await workflow.connect('enhancer', 'validator');
    await workflow.validate();

    return workflow;
  }

  private async createRecommendationWorkflow(): Promise<Workflow> {
    const workflow = new Workflow('Recommendation Generation');

    const analyzer = Node.agent(
      'Recommendation Analyzer',
      `Analyze documents to generate recommendations:

Use search_documents tool to find related documents.
Based on analysis, identify:
1. Similar content themes
2. Complementary topics
3. Related categories
4. Knowledge gaps

Provide structured recommendation data.`,
      'rec_analyzer'
    );

    const generator = Node.agent(
      'Recommendation Generator',
      `Generate personalized recommendations:

Create:
- Top 5 recommended documents with reasons
- Related topics to explore
- Suggested reading order
- Expected learning outcomes

Make recommendations actionable and relevant.`,
      'rec_generator'
    );

    await workflow.addNode(analyzer);
    await workflow.addNode(generator);
    await workflow.connect('rec_analyzer', 'rec_generator');
    await workflow.validate();

    return workflow;
  }

  async addDocument(doc: Document): Promise<void> {
    this.documents.push(doc);
    console.log(`✅ Added document: ${doc.title}`);
  }

  async analyzeDocument(documentId: string): Promise<AnalysisResult | null> {
    const doc = this.documents.find(d => d.id === documentId);
    if (!doc) {
      console.error(`❌ Document not found: ${documentId}`);
      return null;
    }

    console.log(`🔍 Analyzing document: ${doc.title}`);

    try {
      const result = await this.executors.analysis.execute(
        this.workflows.analysis
      );

      if (result.isSuccess()) {
        const vars = result.variables();
        
        const analysisResult: AnalysisResult = {
          documentId: doc.id,
          summary: vars.summary || 'No summary available',
          keyTopics: vars.keyTopics || [],
          sentiment: vars.sentiment || 'neutral',
          qualityScore: vars.qualityScore || 0,
          recommendations: vars.recommendations || []
        };

        this.analysisResults.set(documentId, analysisResult);
        console.log('✅ Analysis complete');
        return analysisResult;
      } else {
        console.error('❌ Analysis failed:', result.error());
        return null;
      }
    } catch (error) {
      console.error('❌ Analysis error:', error);
      return null;
    }
  }

  async enhanceContent(documentId: string): Promise<string | null> {
    const doc = this.documents.find(d => d.id === documentId);
    if (!doc) {
      console.error(`❌ Document not found: ${documentId}`);
      return null;
    }

    console.log(`✨ Enhancing document: ${doc.title}`);

    try {
      const result = await this.executors.fast.execute(
        this.workflows.enhancement
      );

      if (result.isSuccess()) {
        const vars = result.variables();
        console.log('✅ Enhancement complete');
        return vars.enhancedContent || doc.content;
      } else {
        console.error('❌ Enhancement failed:', result.error());
        return null;
      }
    } catch (error) {
      console.error('❌ Enhancement error:', error);
      return null;
    }
  }

  async generateRecommendations(
    documentId: string
  ): Promise<string[] | null> {
    const doc = this.documents.find(d => d.id === documentId);
    if (!doc) {
      console.error(`❌ Document not found: ${documentId}`);
      return null;
    }

    console.log(`🎯 Generating recommendations for: ${doc.title}`);

    try {
      const result = await this.executors.batch.execute(
        this.workflows.recommendation
      );

      if (result.isSuccess()) {
        const vars = result.variables();
        console.log('✅ Recommendations generated');
        return vars.recommendations || [];
      } else {
        console.error('❌ Recommendation generation failed:', result.error());
        return null;
      }
    } catch (error) {
      console.error('❌ Recommendation error:', error);
      return null;
    }
  }

  getSystemHealth(): any {
    return {
      health: healthCheck(),
      systemInfo: getSystemInfo(),
      documents: this.documents.length,
      analysisResults: this.analysisResults.size
    };
  }
}

// Usage Example
async function main() {
  const pipeline = new IntelligentDocumentPipeline(
    process.env.OPENAI_API_KEY || '',
    process.env.ANTHROPIC_API_KEY
  );

  await pipeline.initialize();

  // Add sample documents
  await pipeline.addDocument({
    id: 'doc1',
    title: 'Introduction to Machine Learning',
    content: 'Machine learning is a subset of artificial intelligence...',
    category: 'technology',
    metadata: { author: 'John Doe', date: '2025-01-01' }
  });

  await pipeline.addDocument({
    id: 'doc2',
    title: 'Advanced Neural Networks',
    content: 'Neural networks are computing systems inspired by...',
    category: 'technology',
    metadata: { author: 'Jane Smith', date: '2025-01-15' }
  });

  // Analyze document
  const analysis = await pipeline.analyzeDocument('doc1');
  if (analysis) {
    console.log('\n📊 Analysis Results:');
    console.log(`Summary: ${analysis.summary}`);
    console.log(`Key Topics: ${analysis.keyTopics.join(', ')}`);
    console.log(`Sentiment: ${analysis.sentiment}`);
    console.log(`Quality Score: ${analysis.qualityScore}/10`);
  }

  // Enhance content
  const enhanced = await pipeline.enhanceContent('doc1');
  if (enhanced) {
    console.log('\n✨ Content enhanced successfully');
  }

  // Generate recommendations
  const recommendations = await pipeline.generateRecommendations('doc1');
  if (recommendations) {
    console.log('\n🎯 Recommendations:');
    recommendations.forEach((rec, i) => {
      console.log(`${i + 1}. ${rec}`);
    });
  }

  // Check system health
  const health = pipeline.getSystemHealth();
  console.log('\n💚 System Health:', health);
}

main().catch(console.error);
```

## Key Features Demonstrated

1. **Multi-Executor Strategy**: Different executors for different performance needs
2. **Tool Integration**: Custom tools for document operations
3. **Workflow Orchestration**: Multiple specialized workflows
4. **Error Handling**: Graceful error handling throughout
5. **Health Monitoring**: System health checks and diagnostics
6. **Async/Await Pattern**: Proper async handling for all operations
7. **Type Safety**: Full TypeScript types for reliability

## Best Practices

1. **Initialize once**: Call `init()` at application start
2. **Validate workflows**: Always validate before execution
3. **Handle errors**: Check `result.isSuccess()` before accessing variables
4. **Use appropriate executors**: Choose executor type based on workload
5. **Register tools early**: Set up tool registry during initialization
6. **Monitor health**: Regular health checks for production systems

## See Also

- [Workflow Builder Guide](../user-guide/workflow-builder-js.md)
- [Tool Calling Guide](../user-guide/tool-calling-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
- [Monitoring Guide](../user-guide/monitoring-js.md)
