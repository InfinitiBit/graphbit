import 'dotenv/config';
import { init, LlmConfig, AgentBuilder, LlmClient } from '@infinitibit_gmbh/graphbit';

/**
 * Test 02: Quick Start Example
 * Fixed import from '@infinitibit_gmbh/graphbit' (not '@graphbit/core')
 */

async function testQuickStart() {
    console.log('=== Test 02: Quick Start ===\n');

    try {
        // Initialize
        init();
        console.log('✅ Initialized');

        // Create LLM configuration
        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key'
        });
        console.log('✅ Created LLM config');

        // Create LLM client
        const client = new LlmClient(llmConfig);
        console.log('✅ Created LLM client');

        // Create an agent
        const agent = await new AgentBuilder('QuickStartAgent', llmConfig)
            .systemPrompt('You are a helpful assistant.')
            .build();
        console.log('✅ Created agent');

        // Simple completion test (will fail without valid API key, but tests API)
        try {
            const response = await client.complete(['Hello!']);
            console.log(`✅ Completion test: ${response.substring(0, 50)}...`);
        } catch (err: any) {
            if (err.code === 'GenericFailure' && err.message.includes('API')) {
                console.log('⚠️  Completion skipped (needs valid API key)');
            } else {
                throw err;
            }
        }

        console.log('\n✅ Quick Start test passed!');
    } catch (error) {
        console.error('❌ Quick Start test failed:', error);
        throw error;
    }
}

testQuickStart();
