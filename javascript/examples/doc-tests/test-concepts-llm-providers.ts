import 'dotenv/config';
import { init, LlmClient, LlmConfig } from 'graphbit';

/**
 * TEST: LLM Client and Providers from concepts-js.md
 * 
 * This test validates LLM provider configurations and client usage:
 * 1. Different LLM provider configurations (OpenAI, Anthropic, OpenRouter, Ollama)
 * 2. LlmClient for direct completions
 * 
 * Pattern validated from:
 * - docs/user-guide/concepts-js.md (lines 50-214)
 */

async function testLlmConfigProviders() {
    console.log('\n=== Test 1: LLM Provider Configurations ===');

    try {
        init();

        // Test: OpenAI configuration (from docs line ~50-62)
        const openaiConfig = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation',
            model: 'gpt-4o-mini',
            organizationId: 'optional-org-id'
        });
        console.log('✅ LlmConfig.openai() created configuration');

        // Test: Anthropic configuration (from docs line ~66-75)
        const anthropicConfig = LlmConfig.anthropic({
            apiKey: 'test-key-for-structure-validation',
            model: 'claude-3-5-sonnet-20241022'
        });
        console.log('✅ LlmConfig.anthropic() created configuration');

        // Test: OpenRouter configuration (from docs line ~79-88)
        const openrouterConfig = LlmConfig.openRouter({
            apiKey: 'test-key-for-structure-validation',
            model: 'anthropic/claude-3.5-sonnet'
        });
        console.log('✅ LlmConfig.openRouter() created configuration');

        // Test: Ollama configuration (from docs line ~92-101)
        const ollamaConfig = LlmConfig.ollama({
            baseUrl: 'http://localhost:11434',
            model: 'llama2'
        });
        console.log('✅ LlmConfig.ollama() created configuration');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testLlmClientInstantiation() {
    console.log('\n=== Test 2: LLM Client Instantiation ===');

    try {
        init();

        // Test: LlmClient creation (from docs line ~105-134)
        const config = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation'
        });

        const client = new LlmClient(config);
        console.log('✅ LlmClient instantiated with configuration');

        // Verify client has required methods
        if (typeof client.complete !== 'function') {
            throw new Error('client.complete() method not found');
        }
        console.log('✅ client.complete() method exists');

        if (typeof client.completeBatch !== 'function') {
            throw new Error('client.completeBatch() method not found');
        }
        console.log('✅ client.completeBatch() method exists');

        if (typeof client.completeStream !== 'function') {
            throw new Error('client.completeStream() method not found');
        }
        console.log('✅ client.completeStream() method exists');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testLibraryInitialization() {
    console.log('\n=== Test 3: Library Initialization ===');

    try {
        // Test: Library init (from docs line ~17-47)
        const { init: initModule, version, getSystemInfo, healthCheck } = await import('graphbit');

        initModule();
        console.log('✅ init() executed');

        const versionStr = version();
        console.log('✅ version() returned:', versionStr);

        const sysInfo = getSystemInfo();
        console.log('✅ getSystemInfo() returned system information');

        const health = healthCheck();
        console.log('✅ healthCheck() returned:', health);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  TEST SUITE: LLM Providers and Client (from concepts-js.md)');
    console.log('═'.repeat(70));
    console.log('\nValidating LLM configuration patterns...\n');

    const results = await Promise.all([
        testLlmConfigProviders(),
        testLlmClientInstantiation(),
        testLibraryInitialization()
    ]);

    const allPassed = results.every(r => r);

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log('  ✅ ALL TESTS PASSED - LLM provider patterns are CORRECT');
    } else {
        console.log('  ❌ SOME TESTS FAILED - Check errors above');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
