# Content Generation Pipeline - JavaScript

This example demonstrates how to build a sophisticated content generation workflow with GraphBit's JavaScript bindings, featuring research, writing, editing, and quality assurance stages.

## Overview

We'll create a multi-agent pipeline that:
1. **Researches** a given topic
2. **Writes** initial content based on research
3. **Edits** for clarity and engagement
4. **Reviews** for quality and accuracy
5. **Formats** the final output

## Complete Implementation

```typescript
import {
  init,
  LlmConfig,
  Executor,
  Workflow,
  Node,
  version,
  getSystemInfo,
  healthCheck
} from '@infinitibit_gmbh/graphbit';

interface ContentRequest {
  topic: string;
  contentType: 'article' | 'blog' | 'report' | 'tutorial';
  targetLength: number; // words
  tone: 'professional' | 'casual' | 'technical' | 'friendly';
  audience: string;
}

interface ContentResult {
  success: boolean;
  content?: string;
  qualityScore?: number;
  metadata?: {
    wordCount: number;
    processingTime: number;
    revisionsNeeded: number;
  };
  error?: string;
}

class ContentGenerationPipeline {
  private executor: Executor;
  private llmConfig: any;

  constructor(apiKey: string, model: string = 'gpt-4o-mini') {
    init();

    console.log(`📝 Initializing Content Generation Pipeline`);
    console.log(`GraphBit version: ${version()}`);

    this.llmConfig = LlmConfig.openai({
      apiKey,
      model
    });

    this.executor = new Executor(this.llmConfig);

    console.log('✅ Pipeline initialized\n');
  }

  async createWorkflow(request: ContentRequest): Promise<Workflow> {
    const workflow = new Workflow('Content Generation Pipeline');

    // Stage 1: Research Agent
    const researcher = Node.agent(
      'Research Specialist',
      `Research the topic: "${request.topic}"

Please provide:
1. Key facts and statistics
2. Current trends and developments
3. Expert opinions or insights
4. Relevant examples or case studies
5. Important considerations or nuances

Focus on accuracy and credibility.
Format as structured research notes.`,
      'researcher'
    );

    // Stage 2: Content Writer
    const writer = Node.agent(
      'Content Writer',
      `Write a comprehensive ${request.contentType} about: "${request.topic}"

Requirements:
- Target length: ${request.targetLength} words
- Tone: ${request.tone}
- Audience: ${request.audience}
- Include relevant examples and data from research
- Use engaging headlines and subheadings
- Ensure logical flow and structure

Create compelling, informative content that captures reader attention.`,
      'writer'
    );

    // Stage 3: Editor
    const editor = Node.agent(
      'Content Editor',
      `Edit and improve the following ${request.contentType}:

Focus on:
- Clarity and readability
- Flow and structure
- Engaging language
- Grammar and style
- Consistency in tone
- Compelling headlines

Maintain the core message while making it more engaging and polished.`,
      'editor'
    );

    // Stage 4: Quality Reviewer
    const reviewer = Node.agent(
      'Quality Reviewer',
      `Review this ${request.contentType} for quality and accuracy:

Provide feedback on:
1. Factual accuracy (1-10)
2. Completeness of coverage (1-10)
3. Logical flow (1-10)
4. Audience appropriateness (1-10)
5. Overall quality (1-10)

Rate overall quality and provide specific suggestions.
If quality is 7 or above, mark as APPROVED.
If below 7, mark as NEEDS_REVISION with improvement suggestions.

Format response as JSON with ratings and status.`,
      'reviewer'
    );

    // Stage 5: Final Formatter
    const formatter = Node.agent(
      'Content Formatter',
      `Format this content for publication:

Apply:
- Professional formatting
- Proper heading hierarchy (H1, H2, H3)
- Bullet points where appropriate
- Clear paragraph breaks
- Call-to-action if needed
- Markdown formatting

Output clean, publication-ready content.`,
      'formatter'
    );

    // Add nodes to workflow
    await workflow.addNode(researcher);
    await workflow.addNode(writer);
    await workflow.addNode(editor);
    await workflow.addNode(reviewer);
    await workflow.addNode(formatter);

    // Connect the pipeline: Research → Write → Edit → Review → Format
    await workflow.connect('researcher', 'writer');
    await workflow.connect('writer', 'editor');
    await workflow.connect('editor', 'reviewer');
    await workflow.connect('reviewer', 'formatter');

    // Validate workflow
    await workflow.validate();

    return workflow;
  }

  async generateContent(request: ContentRequest): Promise<ContentResult> {
    console.log(`\n🚀 Starting content generation for: "${request.topic}"`);
    console.log(`Type: ${request.contentType}`);
    console.log(`Target length: ${request.targetLength} words`);
    console.log(`Tone: ${request.tone}`);
    console.log(`Audience: ${request.audience}\n`);

    const startTime = Date.now();

    try {
      // Create workflow
      const workflow = await this.createWorkflow(request);

      // Execute workflow
      console.log('⏳ Executing content generation workflow...\n');
      const result = await this.executor.execute(workflow);

      const processingTime = Date.now() - startTime;

      if (result.isSuccess()) {
        const vars = result.variables();

        // Extract quality score from reviewer
        let qualityScore = 7; // Default
        try {
          if (vars.reviewer && typeof vars.reviewer === 'string') {
            const reviewData = JSON.parse(vars.reviewer);
            qualityScore = reviewData.overallQuality || reviewData.quality_score || 7;
          }
        } catch {
          // Use default if parsing fails
        }

        // Get final formatted content
        const content = vars.formatter || vars.editor || vars.writer || 'No content generated';

        // Count words
        const wordCount = content.split(/\s+/).length;

        console.log('✅ Content generation completed successfully!');
        console.log(`Processing time: ${Math.round(processingTime / 1000)}s`);
        console.log(`Word count: ${wordCount}`);
        console.log(`Quality score: ${qualityScore}/10\n`);

        return {
          success: true,
          content,
          qualityScore,
          metadata: {
            wordCount,
            processingTime,
            revisionsNeeded: qualityScore < 7 ? 1 : 0
          }
        };
      } else {
        console.error('❌ Content generation failed');
        return {
          success: false,
          error: result.error()
        };
      }
    } catch (error) {
      console.error('❌ Error during content generation:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  async generateMultiple(requests: ContentRequest[]): Promise<ContentResult[]> {
    console.log(`\n📚 Generating ${requests.length} pieces of content...\n`);

    const results: ContentResult[] = [];

    for (let i = 0; i < requests.length; i++) {
      console.log(`\n[${ i + 1}/${requests.length}] Processing: "${requests[i].topic}"`);
      const result = await this.generateContent(requests[i]);
      results.push(result);
      
      // Small delay between requests
      if (i < requests.length - 1) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }

    return results;
  }

  async generateWithIterativeImprovement(
    request: ContentRequest,
    maxIterations: number = 2
  ): Promise<ContentResult> {
    console.log(`\n🔄 Generating content with iterative improvement`);
    console.log(`Max iterations: ${maxIterations}\n`);

    let bestResult: ContentResult = { success: false };
    let iteration = 0;

    while (iteration < maxIterations) {
      console.log(`\n--- Iteration ${iteration + 1} ---`);
      
      const result = await this.generateContent(request);

      if (!result.success) {
        console.error('Generation failed, stopping iterations');
        return result;
      }

      if (!bestResult.success || 
          (result.qualityScore || 0) > (bestResult.qualityScore || 0)) {
        bestResult = result;
      }

      // If quality is excellent, stop early
      if ((result.qualityScore || 0) >= 9) {
        console.log('\n✨ Excellent quality achieved, stopping iterations');
        break;
      }

      iteration++;

      if (iteration < maxIterations) {
        console.log('\n⏳ Waiting before next iteration...');
        await new Promise(resolve => setTimeout(resolve, 2000));
      }
    }

    console.log(`\n✅ Best result: Quality score ${bestResult.qualityScore}/10`);
    return bestResult;
  }

  getSystemStatus(): any {
    return {
      version: version(),
      health: healthCheck(),
      systemInfo: getSystemInfo()
    };
  }
}

// Usage Examples

async function example1_BasicArticle() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 1: Basic Article Generation');
  console.log('='.repeat(60));

  const pipeline = new ContentGenerationPipeline(
    process.env.OPENAI_API_KEY || ''
  );

  const result = await pipeline.generateContent({
    topic: 'The Future of Artificial Intelligence',
    contentType: 'article',
    targetLength: 800,
    tone: 'professional',
    audience: 'Technology professionals and enthusiasts'
  });

  if (result.success && result.content) {
    console.log('\n📄 GENERATED CONTENT:\n');
    console.log(result.content);
    console.log('\n📊 METADATA:');
    console.log(`Word count: ${result.metadata?.wordCount}`);
    console.log(`Quality score: ${result.qualityScore}/10`);
    console.log(`Processing time: ${Math.round((result.metadata?.processingTime || 0) / 1000)}s`);
  } else {
    console.error('Failed to generate content:', result.error);
  }
}

async function example2_MultiplePieces() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 2: Multiple Content Generation');
  console.log('='.repeat(60));

  const pipeline = new ContentGenerationPipeline(
    process.env.OPENAI_API_KEY || ''
  );

  const requests: ContentRequest[] = [
    {
      topic: 'Getting Started with Machine Learning',
      contentType: 'tutorial',
      targetLength: 600,
      tone: 'friendly',
      audience: 'Beginners'
    },
    {
      topic: 'Cloud Computing Best Practices',
      contentType: 'blog',
      targetLength: 500,
      tone: 'professional',
      audience: 'IT professionals'
    },
    {
      topic: 'Introduction to GraphBit',
      contentType: 'article',
      targetLength: 700,
      tone: 'technical',
      audience: 'Developers'
    }
  ];

  const results = await pipeline.generateMultiple(requests);

  console.log('\n📊 SUMMARY:');
  results.forEach((result, i) => {
    console.log(`\n${i + 1}. ${requests[i].topic}`);
    console.log(`   Success: ${result.success ? '✅' : '❌'}`);
    if (result.success) {
      console.log(`   Words: ${result.metadata?.wordCount}`);
      console.log(`   Quality: ${result.qualityScore}/10`);
    } else {
      console.log(`   Error: ${result.error}`);
    }
  });
}

async function example3_IterativeImprovement() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 3: Iterative Content Improvement');
  console.log('='.repeat(60));

  const pipeline = new ContentGenerationPipeline(
    process.env.OPENAI_API_KEY || ''
  );

  const result = await pipeline.generateWithIterativeImprovement({
    topic: 'Sustainable Energy Solutions',
    contentType: 'report',
    targetLength: 1000,
    tone: 'professional',
    audience: 'Policy makers and industry leaders'
  }, 2);

  if (result.success && result.content) {
    console.log('\n📄 FINAL CONTENT:\n');
    console.log(result.content.substring(0, 500) + '...\n');
    console.log('📊 FINAL METRICS:');
    console.log(`Quality score: ${result.qualityScore}/10`);
    console.log(`Word count: ${result.metadata?.wordCount}`);
  }
}

async function example4_CustomWorkflow() {
  console.log('\n' + '='.repeat(60));
  console.log('EXAMPLE 4: Custom Content Workflow');
  console.log('='.repeat(60));

  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || ''
  });

  const executor = new Executor(config);

  // Custom workflow with SEO optimization
  const workflow = new Workflow('SEO-Optimized Content');

  const researcher = Node.agent(
    'SEO Researcher',
    'Research keywords and SEO best practices for: Web Development',
    'seo_researcher'
  );

  const writer = Node.agent(
    'SEO Writer',
    'Write SEO-optimized article using researched keywords',
    'seo_writer'
  );

  const optimizer = Node.agent(
    'SEO Optimizer',
    'Optimize content for search engines: meta tags, structure, keywords',
    'seo_optimizer'
  );

  await workflow.addNode(researcher);
  await workflow.addNode(writer);
  await workflow.addNode(optimizer);

  await workflow.connect('seo_researcher', 'seo_writer');
  await workflow.connect('seo_writer', 'seo_optimizer');

  await workflow.validate();

  console.log('⏳ Executing custom SEO workflow...\n');

  const result = await executor.execute(workflow);

  if (result.isSuccess()) {
    console.log('✅ SEO-optimized content generated successfully');
    const vars = result.variables();
    console.log('\n📄 Optimized Content:\n');
    console.log((vars.seo_optimizer || 'No content').substring(0, 500) + '...');
  } else {
    console.error('❌ Failed:', result.error());
  }
}

// Main execution
async function main() {
  try {
    // Run examples
    await example1_BasicArticle();
    await example2_MultiplePieces();
    await example3_IterativeImprovement();
    await example4_CustomWorkflow();

    console.log('\n✅ All examples completed successfully!\n');
  } catch (error) {
    console.error('❌ Error:', error);
    process.exit(1);
  }
}

// Uncomment to run
// main().catch(console.error);

// Export for use as module
export { ContentGenerationPipeline, ContentRequest, ContentResult };
```

## Key Features

1. **Multi-Stage Pipeline**: Research → Write → Edit → Review → Format
2. **Quality Assurance**: Automated quality scoring and review
3. **Flexible Configuration**: Customizable content type, tone, and audience
4. **Batch Processing**: Generate multiple pieces efficiently
5. **Iterative Improvement**: Refine content through multiple iterations
6. **Custom Workflows**: Build specialized pipelines (SEO, etc.)

## Best Practices

1. **Set realistic targets**: Match word count to content type
2. **Choose appropriate tone**: Align with target audience
3. **Review quality scores**: Iterate if quality is below threshold
4. **Monitor processing time**: Balance quality vs. speed
5. **Handle errors gracefully**: Check result.success before using content
6. **Use batch processing**: Efficient for multiple pieces

## Performance Tips

- Use `gpt-4o-mini` for fast, cost-effective generation
- Batch multiple requests with delays
- Implement caching for common topics
- Monitor API rate limits
- Use streaming for long-form content

## See Also

- [Workflow Builder Guide](../user-guide/workflow-builder-js.md)
- [LLM Integration Example](./llm-integration-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
- [JavaScript API Reference](../api-reference/javascript-api.md)
