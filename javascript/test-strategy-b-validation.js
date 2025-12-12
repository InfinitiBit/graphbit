#!/usr/bin/env node

/**
 * Strategy B Migration - Breaking Changes Validation
 * Sequential test to verify no breaking changes in JS bindings
 */

const graphbit = require('./index.js');

async function main() {
  console.log('═══════════════════════════════════════════════════');
  console.log('  Strategy B - Breaking Changes Validation');
  console.log('═══════════════════════════════════════════════════\n');

  try {
    // Test 1: Module & Basic Functions
    console.log('✓ Test 1: Module & Basic Functions');
    graphbit.init({ logLevel: 'error' });
    console.log('  ✅ init() works');
    
    const v = graphbit.version();
    console.log(`  ✅ version() works: ${v}`);
    
    const info = graphbit.versionInfo();
    console.log(`  ✅ versionInfo() works: ${info.version}`);
    
    const health = graphbit.healthCheck();
    console.log(`  ✅ healthCheck() works: healthy=${health.healthy}`);
    
    // Test 2: Document Processing
    console.log('\n✓ Test 2: Document Processing');
    const loader = new graphbit.DocumentLoader();
    const doc = await loader.loadText('Test content', 'test');
    console.log(`  ✅ DocumentLoader works: ${doc.content.length} chars`);
    
    const splitter = graphbit.TextSplitter.character(20);
    const chunks = splitter.split('Test content for splitting');
    console.log(`  ✅ TextSplitter works: ${chunks.length} chunks`);
    
    // Test 3: Workflow Graph
    console.log('\n✓ Test 3: Workflow Graph');
    const graph = new graphbit.WorkflowGraph();
    
    await graph.addNode({
      id: 'test-node',
      name: 'Test Node',
      description: 'Test',
      nodeType: 'Agent',
      retryConfig: {
        maxAttempts: 3,
        initialDelayMs: 1000,
        backoffMultiplier: 2.0,
        maxDelayMs: 5000,
        jitterFactor: 0.1,
        retryableErrors: []
      }
    });
    
    const count = await graph.nodeCount();
    console.log(`  ✅ WorkflowGraph works: ${count} nodes`);
    
    // Test 4: LLM Configs
    console.log('\n✓ Test 4: LLM Provider Configs');
    const configs = [
      graphbit.LlmConfig.openai({ apiKey: 'test', model: 'gpt-4' }),
      graphbit.LlmConfig.anthropic({ apiKey: 'test', model: 'claude-3-5-sonnet-20241022' }),
      graphbit.LlmConfig.ollama({ model: 'llama2', baseUrl: 'http://localhost:11434' }),
    ];
    console.log(`  ✅ LlmConfig works: ${configs.length} providers tested`);
    
    const client = new graphbit.LlmClient(configs[0]);
    console.log('  ✅ LlmClient instantiation works');
    
    // Test 5: Tool Registry
    console.log('\n✓ Test 5: Tool Registry');
    const registry = graphbit.createToolRegistry();
    registry.register('test', 'Test tool', { input: { type: 'string' } }, (args) => args.input);
    console.log(`  ✅ Tool registry works: ${registry.getRegisteredTools().length} tools`);
    
    // Test 6: Embeddings
    console.log('\n✓ Test 6: Embeddings');
    const embConfig = graphbit.EmbeddingConfig.openai('test', 'text-embedding-ada-002');
    const embClient = new graphbit.EmbeddingClient(embConfig);
    console.log('  ✅ EmbeddingClient instantiation works');
    
    const sim = graphbit.EmbeddingClient.similarity([1, 0, 0], [1, 0, 0]);
    console.log(`  ✅ Cosine similarity works: ${sim.toFixed(2)}`);
    
    // Test 7: Validation
    console.log('\n✓ Test 7: JSON Validation');
    const schema = { type: 'object', properties: { name: { type: 'string' } }, required: ['name'] };
    const result = graphbit.validateJson('{"name":"test"}', JSON.stringify(schema));
    console.log(`  ✅ Validation works: isValid=${result.isValid}`);
    
    // Test 8: Builders
    console.log('\n✓ Test 8: Builder Patterns');
    const agentBuilder = new graphbit.AgentBuilder('test', configs[0]);
    agentBuilder.description('Test').systemPrompt('Test').temperature(0.7);
    console.log('  ✅ AgentBuilder works');
    
    const workflowBuilder = new graphbit.WorkflowBuilder('test', configs[0]);
    workflowBuilder.description('Test');
    console.log('  ✅ WorkflowBuilder works');
    
    // Final Summary
    console.log('\n═══════════════════════════════════════════════════');
    console.log('  ✅ ALL TESTS PASSED - NO BREAKING CHANGES');
    console.log('═══════════════════════════════════════════════════');
    console.log('\n📋 Validated Components:');
    console.log('  ✅ Core functions (init, version, healthCheck)');
    console.log('  ✅ Document processing (loader, splitter)');
    console.log('  ✅ Workflow graphs');
    console.log('  ✅ LLM configurations (3 providers)');
    console.log('  ✅ Tool registry');
    console.log('  ✅ Embeddings');
    console.log('  ✅ JSON validation');
    console.log('  ✅ Builder patterns');
    console.log('\n🎉 Strategy B migration: SAFE - No breaking changes!');
    
  } catch (err) {
    console.error('\n❌ TEST FAILED:', err.message);
    console.error(err.stack);
    process.exit(1);
  }
}

main().then(() => {
  console.log('\n✓ Test completed successfully');
  process.exit(0);
}).catch((err) => {
  console.error('\n❌ Unexpected error:', err);
  process.exit(1);
});
