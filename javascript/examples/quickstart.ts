/**
 * Quickstart: Standalone Agent
 *
 * This example demonstrates how to create and execute a standalone agent
 * using the GraphBit JavaScript bindings.
 */

import { init, AgentBuilder, LlmConfig } from '../index';

async function main() {
    // Initialize the library
    init();

    // Configure LLM (using OpenAI as example)
    // You can also use .ollama(), .anthropic(), etc.
    const apiKey = process.env.OPENAI_API_KEY;
    if (!apiKey) {
        console.log('Skipping execution: OPENAI_API_KEY not set');
        return;
    }

    const config = LlmConfig.openai({
        apiKey,
        model: 'gpt-4o-mini',
    });

    // Create an agent
    console.log('Building agent...');
    const agent = await new AgentBuilder('Quickstart Agent', config)
        .description('A helpful assistant for quickstart guide')
        .systemPrompt('You are a helpful AI assistant. Answer concisely.')
        .temperature(0.7)
        .build();

    // Execute the agent
    const input = 'What is the capital of France?';
    console.log(`\nUser: ${input}`);

    try {
        const response = await agent.execute(input);
        console.log(`Agent: ${response}`);
    } catch (error) {
        console.error('Execution failed:', error);
    }
}

main().catch(console.error);
