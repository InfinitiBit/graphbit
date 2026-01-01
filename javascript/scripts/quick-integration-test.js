/**
 * Quick Integration Test
 * Tests LlmClient with real OpenAI API
 */

const { LlmClient, LlmConfig } = require('../index');

async function main() {
  console.log('='.repeat(70));
  console.log(' Quick Integration Test - LlmClient with OpenAI');
  console.log('='.repeat(70));
  console.log();

  if (!process.env.OPENAI_API_KEY) {
    console.log('❌ OPENAI_API_KEY not set');
    process.exit(1);
  }

  console.log('✅ API key detected');
  console.log();

  try {
    // Create client
    console.log('1. Creating LlmClient...');
    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY,
      model: 'gpt-4o-mini'
    });
    const client = new LlmClient(config);
    console.log('✅ Client created\n');

    // Test 1: Simple completion
    console.log('2. Testing simple completion...');
    const response1 = await client.complete("What is 2+2? Answer with just the number.", 10, 0.0);
    console.log(`   Response: "${response1}"`);
    console.log(`   ✅ Simple completion works\n`);

    // Test 2: Full completion
    console.log('3. Testing full completion with metadata...');
    const response2 = await client.completeFull("What is AI? One sentence.", 50, 0.7);
    console.log(`   Response: "${response2.content.substring(0, 80)}..."`);
    console.log(`   Tokens: ${response2.usage?.totalTokens || 0}`);
    console.log(`   Model: ${response2.model}`);
    console.log(`   ✅ Full completion works\n`);

    // Test 3: Batch processing
    console.log('4. Testing batch processing...');
    const prompts = [
      "What is 1+1? Just the number.",
      "What is 2+2? Just the number.",
      "What is 3+3? Just the number."
    ];
    const responses = await client.completeBatch(prompts, 10, 0.0, 2);
    console.log(`   Got ${responses.length} responses`);
    responses.forEach((r, i) => console.log(`   [${i+1}] ${r}`));
    console.log(`   ✅ Batch processing works\n`);

    // Test 4: Statistics
    console.log('5. Testing statistics...');
    const stats = await client.getStats();
    console.log(`   Total requests: ${stats.totalRequests}`);
    console.log(`   Successful: ${stats.successfulRequests}`);
    console.log(`   Avg time: ${stats.avgResponseTimeMs.toFixed(2)}ms`);
    console.log(`   ✅ Statistics tracking works\n`);

    console.log('='.repeat(70));
    console.log('✅ ALL INTEGRATION TESTS PASSED!');
    console.log('='.repeat(70));
    console.log();
    console.log('🎉 LlmClient is working perfectly with real OpenAI API!');
    console.log();

  } catch (error) {
    console.log();
    console.log('❌ ERROR:', error.message);
    console.log();
    process.exit(1);
  }
}

main().catch(console.error);

