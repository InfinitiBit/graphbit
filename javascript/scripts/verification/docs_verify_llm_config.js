const { LlmConfig } = require('../../index');

async function verifyLlmConfigUsage() {
    console.log('Verifying LlmConfig for Documentation...');

    // 1. Test OpenAI config
    try {
        const openaiConfig = LlmConfig.openai({
            apiKey: 'test-api-key',
            model: 'gpt-4o-mini'
        });
        console.log('✅ LlmConfig.openai() created successfully');

        // Verify it's an instance
        if (!(openaiConfig instanceof LlmConfig)) {
            throw new Error('openai() should return LlmConfig instance');
        }
    } catch (error) {
        console.error('❌ LlmConfig.openai() failed:', error);
        process.exit(1);
    }

    // 2. Test Anthropic config
    try {
        const anthropicConfig = LlmConfig.anthropic({
            apiKey: 'test-anthropic-key',
            model: 'claude-3-5-sonnet-20241022'
        });
        console.log('✅ LlmConfig.anthropic() created successfully');
    } catch (error) {
        console.error('❌ LlmConfig.anthropic() failed:', error);
        process.exit(1);
    }

    // 3. Test Ollama config
    try {
        const ollamaConfig = LlmConfig.ollama({
            model: 'llama3.2',
            baseUrl: 'http://localhost:11434'
        });
        console.log('✅ LlmConfig.ollama() created successfully');
    } catch (error) {
        console.error('❌ LlmConfig.ollama() failed:', error);
        process.exit(1);
    }

    // 4. Test DeepSeek config
    try {
        const deepseekConfig = LlmConfig.deepseek({
            apiKey: 'test-deepseek-key',
            model: 'deepseek-chat'
        });
        console.log('✅ LlmConfig.deepseek() created successfully');
    } catch (error) {
        console.error('❌ LlmConfig.deepseek() failed:', error);
        process.exit(1);
    }

    console.log('\n✨ All LlmConfig factory methods verified successfully!');
}

verifyLlmConfigUsage().catch(console.error);
