import 'dotenv/config';
import { init, AgentBuilder, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

/**
 * Test 05: Agents
 * Tests agent creation and configuration
 */

async function testAgents() {
    console.log('=== Test 05: Agents ===\n');

    try {
        init();

        const llmConfig = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY || 'test-key' });

        // Create basic agent
        const agent = await new AgentBuilder('TestAgent', llmConfig)
            .systemPrompt('You are a test assistant.')
            .description('A test agent')
            .build();

        console.log(`✅ Created agent: ${await agent.name()}`);
        console.log(`   Description: ${await agent.description()}`);

        // Create agent with tools (if tool registry is available)
        // const agentWithTools = await new AgentBuilder('ToolAgent', llmConfig)
        //   .systemPrompt('You are a helpful assistant with tools.')
        //   .build();

        console.log('\n✅ Agents test passed!');
    } catch (error) {
        console.error('❌ Agents test failed:', error);
        throw error;
    }
}

testAgents();
