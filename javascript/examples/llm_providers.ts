import { LlmConfig } from '../index';

function main() {
    console.log('Testing LLM Provider Configurations...');

    const configs = [];

    // OpenAI
    configs.push(LlmConfig.openai({ apiKey: 'test', model: 'gpt-4o' }));

    // Anthropic
    configs.push(LlmConfig.anthropic({ apiKey: 'test', model: 'claude-3' }));

    // Ollama
    configs.push(LlmConfig.ollama({ model: 'llama2' }));

    // Azure OpenAI
    configs.push(LlmConfig.azureOpenai({
        apiKey: 'test',
        endpoint: 'https://test.openai.azure.com',
        deploymentName: 'gpt-4'
    }));

    // ByteDance
    configs.push(LlmConfig.bytedance({ apiKey: 'test', model: 'skylark' }));

    // DeepSeek
    configs.push(LlmConfig.deepseek({ apiKey: 'test', model: 'deepseek-chat' }));

    // HuggingFace
    configs.push(LlmConfig.huggingface({ apiKey: 'test', model: 'gpt2' }));

    // Perplexity
    configs.push(LlmConfig.perplexity({ apiKey: 'test', model: 'sonar' }));

    // OpenRouter
    configs.push(LlmConfig.openrouter({ apiKey: 'test', model: 'openai/gpt-4' }));

    // Fireworks
    configs.push(LlmConfig.fireworks({ apiKey: 'test', model: 'llama-v3' }));

    // Replicate
    configs.push(LlmConfig.replicate({ apiKey: 'test', model: 'meta/llama-2' }));

    // TogetherAI
    configs.push(LlmConfig.togetherai({ apiKey: 'test', model: 'gpt-oss' }));

    // xAI
    configs.push(LlmConfig.xai({ apiKey: 'test', model: 'grok-1' }));

    // AI21
    configs.push(LlmConfig.ai21({ apiKey: 'test', model: 'jamba' }));

    // MistralAI
    configs.push(LlmConfig.mistralai({ apiKey: 'test', model: 'mistral-large' }));

    console.log(`Verified ${configs.length} provider configurations.`);
}

main();
