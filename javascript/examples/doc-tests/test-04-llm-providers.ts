import 'dotenv/config';
import { init, LlmConfig } from '@infinitibit_gmbh/graphbit';

/**
 * Test 04: LLM Providers
 * Tests different LLM provider configurations
 */

async function testLlmProviders() {
    console.log('=== Test 04: LLM Providers ===\n');

    try {
        init();

        // OpenAI
        const openaiConfig = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY || 'test-key' });
        console.log('✅ OpenAI config created');

        // Anthropic
        const anthropicConfig = LlmConfig.anthropic({ apiKey: process.env.ANTHROPIC_API_KEY || 'test-key' });
        console.log('✅ Anthropic config created');

        // Groq
        const groqConfig = LlmConfig.groq({ apiKey: process.env.GROQ_API_KEY || 'test-key' });
        console.log('✅ Groq config created');

        // Ollama
        const ollamaConfig = LlmConfig.ollama({});
        console.log('✅ Ollama config created');

        console.log('\n✅ LLM Providers test passed!');
    } catch (error) {
        console.error('❌ LLM Providers test failed:', error);
        throw error;
    }
}

testLlmProviders();
