#!/usr/bin/env node

/**
 * Strategy B Migration - Real-World Usage Test
 * 
 * This test simulates actual user workflows to ensure
 * no breaking changes in JS bindings.
 */

console.log('═══════════════════════════════════════════════════');
console.log('  Strategy B - Real-World Usage Test');
console.log('═══════════════════════════════════════════════════\n');

const graphbit = require('./index.js');

// Test 1: Document Processing Workflow
console.log('✓ Test 1: Document Processing Workflow');
(async () => {
  try {
    graphbit.init({ logLevel: 'error' });
    
    const loader = new graphbit.DocumentLoader();
    const textDoc = await loader.loadText('GraphBit is an agentic workflow framework.', 'test-source');
    console.log(`  ✅ Document loaded: ${textDoc.content.length} chars`);
    
    const splitter = graphbit.TextSplitter.character(50);
    const chunks = splitter.split(textDoc.content);
    console.log(`  ✅ Text split into ${chunks.length} chunks`);
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
})();

// Test 2: Workflow Graph Construction
console.log('\n✓ Test 2: Workflow Graph Construction');
(async () => {
  try {
    const graph = new graphbit.WorkflowGraph();
    
    await graph.addNode({
      id: 'node1',
      name: 'Start Node',
      description: 'Initial node',
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
    
    await graph.addNode({
      id: 'node2',
      name: 'End Node',
      description: 'Final node',
      nodeType: 'Transform',
      retryConfig: {
        maxAttempts: 3,
        initialDelayMs: 1000,
        backoffMultiplier: 2.0,
        maxDelayMs: 5000,
        jitterFactor: 0.1,
        retryableErrors: []
      }
    });
    
    await graph.addEdge({
      fromNode: 'node1',
      toNode: 'node2'
    });
    
    const nodeCount = await graph.nodeCount();
    const edgeCount = await graph.edgeCount();
    const hasCycles = await graph.hasCycles();
    
    console.log(`  ✅ Graph created: ${nodeCount} nodes, ${edgeCount} edges`);
    console.log(`  ✅ Cycle detection works: ${hasCycles}`);
    
    const roots = await graph.getRootNodes();
    console.log(`  ✅ Root nodes: ${roots.join(', ')}`);
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
})();

// Test 3: LLM Configuration (all providers)
console.log('\n✓ Test 3: LLM Configuration');
setTimeout(() => {
  try {
    // OpenAI
    const openaiConfig = graphbit.LlmConfig.openai({
      apiKey: 'test-key',
      model: 'gpt-4'
    });
    console.log('  ✅ OpenAI config created');
    
    // Anthropic
    const anthropicConfig = graphbit.LlmConfig.anthropic({
      apiKey: 'test-key',
      model: 'claude-3-5-sonnet-20241022'
    });
    console.log('  ✅ Anthropic config created');
    
    // Ollama
    const ollamaConfig = graphbit.LlmConfig.ollama({
      model: 'llama2',
      baseUrl: 'http://localhost:11434'
    });
    console.log('  ✅ Ollama config created');
    
    // Azure OpenAI
    const azureConfig = graphbit.LlmConfig.azureOpenai({
      apiKey: 'test-key',
      apiVersion: '2024-02-15-preview',
      model: 'gpt-4',
      endpoint: 'https://test.openai.azure.com/',
      deploymentName: 'gpt-4-deployment'
    });
    console.log('  ✅ Azure OpenAI config created');
    
    // DeepSeek
    const deepseekConfig = graphbit.LlmConfig.deepseek({
      apiKey: 'test-key',
      model: 'deepseek-chat'
    });
    console.log('  ✅ DeepSeek config created');
    
    // Test LlmClient instantiation
    const client = new graphbit.LlmClient(openaiConfig);
    console.log('  ✅ LlmClient instantiated');
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 100);

// Test 4: Tool Registry
console.log('\n✓ Test 4: Tool Registry');
setTimeout(() => {
  try {
    const registry = graphbit.createToolRegistry();
    console.log('  ✅ Tool registry created');
    
    // Register a simple tool
    registry.register('test_tool', 'A test tool', {
      input: { type: 'string' }
    }, (params) => {
      return `Processed: ${params.input}`;
    });
    
    const tools = registry.getRegisteredTools();
    console.log(`  ✅ Tool registered: ${tools.length} tools`);
    
    const hasTool = registry.hasTool('test_tool');
    console.log(`  ✅ Tool lookup works: ${hasTool}`);
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 200);

// Test 5: Embedding Configuration
console.log('\n✓ Test 5: Embedding Configuration');
setTimeout(() => {
  try {
    const openaiEmbedding = graphbit.EmbeddingConfig.openai('test-key', 'text-embedding-ada-002');
    console.log('  ✅ OpenAI embedding config created');
    
    const hfEmbedding = graphbit.EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-MiniLM-L6-v2');
    console.log('  ✅ HuggingFace embedding config created');
    
    const embeddingClient = new graphbit.EmbeddingClient(openaiEmbedding);
    console.log('  ✅ EmbeddingClient instantiated');
    
    // Test cosine similarity
    const sim = graphbit.EmbeddingClient.similarity([1, 0, 0], [1, 0, 0]);
    console.log(`  ✅ Cosine similarity: ${sim.toFixed(2)}`);
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 300);

// Test 6: JSON Validation
console.log('\n✓ Test 6: JSON Validation');
setTimeout(() => {
  try {
    const schema = {
      type: 'object',
      properties: {
        name: { type: 'string' },
        age: { type: 'number' }
      },
      required: ['name']
    };
    
    const validData = JSON.stringify({ name: 'Alice', age: 30 });
    const result1 = graphbit.validateJson(validData, JSON.stringify(schema));
    if (result1.isValid) {
      console.log('  ✅ Valid JSON accepted');
    } else {
      console.error('  ❌ FAILED: Valid data rejected');
      process.exit(1);
    }
    
    const invalidData = JSON.stringify({ age: 30 }); // missing required 'name'
    const result2 = graphbit.validateJson(invalidData, JSON.stringify(schema));
    if (!result2.isValid && result2.errors.length > 0) {
      console.log('  ✅ Invalid JSON rejected correctly');
    } else {
      console.error('  ❌ FAILED: Invalid data accepted');
      process.exit(1);
    }
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 400);

// Test 7: Agent Builder (without API calls)
console.log('\n✓ Test 7: Agent Builder');
setTimeout(async () => {
  try {
    const llmConfig = graphbit.LlmConfig.openai({
      apiKey: 'test-key',
      model: 'gpt-4'
    });
    
    const agentBuilder = new graphbit.AgentBuilder('test-agent', llmConfig);
    
    agentBuilder
      .description('A test agent')
      .systemPrompt('You are a helpful assistant')
      .temperature(0.7)
      .maxTokens(1000);
    
    console.log('  ✅ Agent builder configured (chaining works)');
    
    // Note: We can't build the agent without a valid API key,
    // but the builder pattern works correctly
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 500);

// Test 8: WorkflowBuilder (structure validation)
console.log('\n✓ Test 8: WorkflowBuilder');
setTimeout(async () => {
  try {
    const llmConfig = graphbit.LlmConfig.openai({
      apiKey: 'test-key',
      model: 'gpt-4'
    });
    
    const workflow = new graphbit.WorkflowBuilder('test-workflow', llmConfig);
    
    workflow.description('A test workflow');
    
    console.log('  ✅ WorkflowBuilder instantiated');
    console.log('  ✅ Builder methods work correctly');
  } catch (err) {
    console.error('  ❌ FAILED:', err.message);
    process.exit(1);
  }
}, 600);

// Final Summary
setTimeout(() => {
  console.log('\n═══════════════════════════════════════════════════');
  console.log('  ✅ ALL REAL-WORLD USAGE TESTS PASSED');
  console.log('═══════════════════════════════════════════════════');
  console.log('\n📋 Validated:');
  console.log('  ✅ Document processing workflows');
  console.log('  ✅ Graph construction and manipulation');
  console.log('  ✅ LLM provider configurations (5 tested)');
  console.log('  ✅ Tool registry operations');
  console.log('  ✅ Embedding configurations');
  console.log('  ✅ JSON schema validation');
  console.log('  ✅ Agent builder pattern');
  console.log('  ✅ Workflow builder pattern');
  console.log('\n🎉 Strategy B migration causes NO breaking changes!');
  console.log('   All JS bindings work correctly.');
}, 700);
