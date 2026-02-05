/**
 * End-to-End Workflow Integration Tests
 * 
 * Tests complete workflow scenarios with real APIs
 * Validates all components working together
 */

import { describe, it, expect } from 'vitest';
import {
  init,
  LlmConfig,
  LlmClient,
  WorkflowBuilder,
  Executor,
  EmbeddingClient,
  EmbeddingConfig,
  DocumentLoader,
  TextSplitter,
  createToolRegistry
} from '../../index';

const hasOpenAI = !!process.env.OPENAI_API_KEY;

describe('End-to-End Workflow Integration', () => {
  it('should execute complete RAG pipeline', async () => {
    if (!hasOpenAI) {
      console.log('⚠️  Skipping - requires OPENAI_API_KEY');
      return;
    }

    // Initialize
    init({ logLevel: 'info' });

    // Setup LLM
    const llmConfig = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY!
    });
    const llmClient = new LlmClient(llmConfig);

    // Setup embeddings
    const embConfig = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!);
    const embClient = new EmbeddingClient(embConfig);

    // Step 1: Load documents
    const loader = new DocumentLoader();
    const doc = await loader.loadText(
      "GraphBit is a workflow automation framework for building AI agents.",
      "knowledge_base"
    );
    
    expect(doc?.content).toBeTruthy();
    console.log('✅ Document loaded');

    // Step 2: Split text
    const splitter = TextSplitter.character(100, 20);
    const chunks = splitter.split(doc?.content);
    
    expect(chunks.length).toBeGreaterThan(0);
    console.log(`✅ Text split into ${chunks.length} chunks`);

    // Step 3: Generate embeddings
    const texts = chunks.map(c => c?.content);
    const embResponse = await embClient.embed(texts);
    
    expect(embResponse.embeddings.length).toBe(chunks.length);
    console.log(`✅ Generated ${embResponse.embeddings.length} embeddings`);

    // Step 4: Query with LLM
    const query = "What is GraphBit?";
    const response = await llmClient.complete(
      `Answer based on this context: ${doc.content}\n\nQuestion: ${query}`,
      100
    );
    
    expect(response).toBeTruthy();
    expect(response.toLowerCase()).toContain('workflow');
    console.log(`✅ LLM response generated: "${response.substring(0, 50)}..."`);

    // Step 5: Verify statistics
    const stats = await llmClient.getStats();
    expect(stats.totalRequests).toBeGreaterThan(0);
    console.log(`✅ Statistics tracked: ${stats.totalRequests} requests`);

  }, 120000);

  it('should handle workflow with variables and state', async () => {
    if (!hasOpenAI) {
      console.log('⚠️  Skipping - requires OPENAI_API_KEY');
      return;
    }

    const llmConfig = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY!
    });

    const workflow = new WorkflowBuilder('Test Workflow')
      .description('Test workflow with state')
      .build();

    const executor = new Executor(llmConfig);
    const result = await executor.execute(workflow);

    // Set and get variables
    const context = result.getContext();
    await context.setVariable('test_var', '12345');
    const retrieved = await context.getVariable('test_var');
    
    expect(retrieved).toBe('"12345"'); // JSON string
    console.log('✅ Variables work correctly');

    // Check result methods
    expect(result.isSuccess).toBeDefined();
    expect(result.executionTimeMs).toBeDefined();
    expect(result.workflowId).toBeDefined();
    console.log('✅ WorkflowResult methods accessible');

  }, 60000);

  it('should handle tool registry in workflow', async () => {
    if (!hasOpenAI) {
      console.log('⚠️  Skipping - requires OPENAI_API_KEY');
      return;
    }

    const registry = createToolRegistry();

    // Register tools
    registry.register('add', 'Add two numbers', {
      a: 'number',
      b: 'number'
    }, (args: any) => {
      return args.a + args.b;
    });

    registry.register('multiply', 'Multiply two numbers', {
      a: 'number',
      b: 'number'
    }, (args: any) => {
      return args.a * args.b;
    });

    // Execute tools
    const result1 = await registry.execute('add', { a: 5, b: 3 });
    expect(result1.success).toBe(true);
    expect(result1.result).toBe(8);
    console.log('✅ Tool execution works');

    // Check metadata
    const metadata = registry.getToolMetadata('add');
    expect(metadata).toBeTruthy();
    expect(metadata!.callCount).toBe(1);
    console.log('✅ Metadata tracking works');

    // Check history
    const history = registry.getExecutionHistory();
    expect(history.length).toBeGreaterThan(0);
    console.log('✅ Execution history works');

    // Check stats
    const stats = registry.getStats();
    expect(stats.totalExecutions).toBeGreaterThan(0);
    console.log('✅ Statistics tracking works');

  });
});

// Export for use in other tests
export { benchmark, BenchmarkResult };

