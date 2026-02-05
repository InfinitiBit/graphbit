/**
 * LlmClient Demo
 * 
 * Demonstrates all features of the new LlmClient class:
 * - Simple completions
 * - Full completions with metadata
 * - Batch processing
 * - Chat completions
 * - Statistics monitoring
 * - Error handling
 *
 * Run with: npx ts-node examples/llm-client-demo.ts
 */

import { LlmClient, LlmConfig, LlmResponse, ClientStats } from '../index';

// Check for API key
if (!process.env.OPENAI_API_KEY) {
  console.error('Error: OPENAI_API_KEY environment variable is required');
  console.error('Set it with: export OPENAI_API_KEY=your-key-here');
  process.exit(1);
}

async function main() {
  console.log('='.repeat(60));
  console.log(' GraphBit LlmClient Demo');
  console.log('='.repeat(60));
  console.log();

  // Create LLM client
  console.log('📝 Creating LLM client...');
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY!,
    model: 'gpt-4o-mini'
  });
  const client = new LlmClient(config);
  console.log('✅ Client created successfully!\n');

  // Example 1: Simple completion
  console.log('Example 1: Simple Completion');
  console.log('-'.repeat(60));
  const prompt1 = 'What is the capital of France? Answer in one word.';
  console.log(`Prompt: "${prompt1}"`);
  
  const response1 = await client.complete(prompt1, 10);
  console.log(`Response: "${response1}"`);
  console.log();

  // Example 2: Full completion with metadata
  console.log('Example 2: Full Completion with Metadata');
  console.log('-'.repeat(60));
  const prompt2 = 'Explain quantum computing in one sentence.';
  console.log(`Prompt: "${prompt2}"`);
  
  const response2: LlmResponse = await client.completeFull(prompt2, 100, 0.7);
  console.log(`Response: "${response2.content}"`);
  console.log(`Tokens used: ${response2.usage?.totalTokens || 0} (prompt: ${response2.usage?.promptTokens || 0}, completion: ${response2.usage?.completionTokens || 0})`);
  console.log(`Finish reason: ${response2.finishReason || 'unknown'}`);
  console.log(`Model: ${response2.model}`);
  console.log();

  // Example 3: Temperature control
  console.log('Example 3: Temperature Control');
  console.log('-'.repeat(60));
  const creativePrompt = 'Write a creative tagline for a tech startup';
  
  console.log('Low temperature (0.0) - Deterministic:');
  const deterministicResponse = await client.complete(creativePrompt, 20, 0.0);
  console.log(`  "${deterministicResponse}"`);
  
  console.log('High temperature (1.5) - Creative:');
  const creativeResponse = await client.complete(creativePrompt, 20, 1.5);
  console.log(`  "${creativeResponse}"`);
  console.log();

  // Example 4: Batch processing
  console.log('Example 4: Batch Processing');
  console.log('-'.repeat(60));
  const batchPrompts = [
    'What is 2+2? Answer with just the number.',
    'What is the capital of Spain? One word.',
    'What color is the sky? One word.',
    'What is the largest planet? One word.',
    'What is 10*10? Answer with just the number.'
  ];
  
  console.log(`Processing ${batchPrompts.length} prompts with max concurrency of 2...`);
  const startTime = Date.now();
  
  const batchResponses = await client.completeBatch(batchPrompts, 10, 0.0, 2);
  
  const duration = Date.now() - startTime;
  console.log(`Completed in ${duration}ms\n`);
  
  batchResponses.forEach((response: any, index) => {
    console.log(`  ${index + 1}. "${batchPrompts[index]}" → "${response}"`);
  });
  console.log();

  // Example 5: Chat completion
  console.log('Example 5: Chat Completion');
  console.log('-'.repeat(60));
  const chatMessages = [
    ['system', 'You are a helpful AI assistant who gives brief, accurate answers.'],
    ['user', 'What is artificial intelligence?'],
    ['assistant', 'AI is the simulation of human intelligence by machines.'],
    ['user', 'Give an example']
  ];
  
  console.log('Chat conversation:');
  chatMessages.forEach(([role, content]) => {
    console.log(`  ${role.toUpperCase()}: ${content}`);
  });
  console.log();
  
  const chatResponse = await client.chatOptimized(chatMessages, 100, 0.7);
  console.log(`ASSISTANT: ${chatResponse}`);
  console.log();

  // Example 6: Statistics monitoring
  console.log('Example 6: Statistics Monitoring');
  console.log('-'.repeat(60));
  const stats: ClientStats = await client.getStats();
  
  console.log('Client Statistics:');
  console.log(`  Total Requests: ${stats.totalRequests}`);
  console.log(`  Successful: ${stats.successfulRequests}`);
  console.log(`  Failed: ${stats.failedRequests}`);
  console.log(`  Success Rate: ${((stats.successfulRequests / stats.totalRequests) * 100).toFixed(2)}%`);
  console.log(`  Avg Response Time: ${stats.avgResponseTimeMs.toFixed(2)}ms`);
  console.log(`  Total Tokens: ${stats.totalTokens}`);
  console.log(`  Uptime: ${stats.uptimeSeconds.toFixed(2)}s`);
  console.log();

  // Example 7: Error handling
  console.log('Example 7: Error Handling');
  console.log('-'.repeat(60));
  
  try {
    console.log('Attempting completion with empty prompt...');
    await client.complete('');
  } catch (error) {
    console.log(`✅ Caught error: ${error}`);
  }

  try {
    console.log('Attempting completion with invalid temperature...');
    await client.complete('test', 100, 3.0);
  } catch (error) {
    console.log(`✅ Caught error: ${error}`);
  }
  
  try {
    console.log('Attempting batch with empty array...');
    await client.completeBatch([]);
  } catch (error) {
    console.log(`✅ Caught error: ${error}`);
  }
  console.log();

  // Example 8: Multiple clients
  console.log('Example 8: Multiple Providers');
  console.log('-'.repeat(60));
  console.log('You can create multiple clients for different providers:');
  
  const openaiClient = new LlmClient(LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY!
  }));
  console.log('✅ OpenAI client created');
  
  if (process.env.ANTHROPIC_API_KEY) {
    const anthropicClient = new LlmClient(LlmConfig.anthropic({
      apiKey: process.env.ANTHROPIC_API_KEY
    }));
    console.log('✅ Anthropic client created');
    
    // Compare responses from different providers
    const question = 'What is 2+2?';
    const [openaiAnswer, anthropicAnswer] = await Promise.all([
      openaiClient.complete(question, 10, 0.0),
      anthropicClient.complete(question, 10, 0.0)
    ]);
    
    console.log(`OpenAI: "${openaiAnswer}"`);
    console.log(`Anthropic: "${anthropicAnswer}"`);
  } else {
    console.log('ℹ️  Set ANTHROPIC_API_KEY to test multiple providers');
  }
  console.log();

  // Example 9: Warmup for better performance
  console.log('Example 9: Warmup');
  console.log('-'.repeat(60));
  console.log('Warming up client for better performance...');
  await client.warmup();
  console.log('✅ Client warmed up!');
  console.log('Subsequent requests should be faster.');
  console.log();

  // Example 10: Reset statistics
  console.log('Example 10: Reset Statistics');
  console.log('-'.repeat(60));
  console.log('Current stats:', await client.getStats());
  await client.resetStats();
  console.log('After reset:', await client.getStats());
  console.log();

  // Final stats
  console.log('='.repeat(60));
  console.log(' Demo Complete! Final Statistics:');
  console.log('='.repeat(60));
  const finalStats = await client.getStats();
  console.log(JSON.stringify(finalStats, null, 2));
  console.log();
  
  console.log('💡 Tips:');
  console.log('  - Use complete() for simple text responses');
  console.log('  - Use completeFull() when you need token counts and metadata');
  console.log('  - Use completeBatch() for processing multiple prompts efficiently');
  console.log('  - Use chatOptimized() for conversation-style interactions');
  console.log('  - Monitor getStats() for performance and usage tracking');
  console.log('  - Call warmup() at application startup for best performance');
  console.log();
}

// Run the demo
main().catch((error) => {
  console.error('Error running demo:', error);
  process.exit(1);
});

