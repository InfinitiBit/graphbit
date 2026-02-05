import 'dotenv/config';
import { init, AgentBuilder, LlmConfig } from 'graphbit';

/**
 * TEST: Agent Builder API from concepts-js.md
 * 
 * This test validates the AgentBuilder patterns used in the fixed documentation:
 * 1. AgentBuilder creates agents with .systemPrompt() and .build()
 * 2. Agent provides id(), name(), description() methods
 * 3. Different agent configurations (basic, with context, advanced)
 * 
 * Pattern validated from:
 * - docs/user-guide/concepts-js.md (lines 367-448)
 */

async function testBasicAgentCreation() {
    console.log('\n=== Test 1: Basic Agent Creation ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation'
        });

        // Test: Create simple agent
        const agent = await new AgentBuilder('Summarizer', llmConfig)
            .systemPrompt('Summarize the following text')
            .build();

        console.log('✅ AgentBuilder created agent');

        // Verify agent has required methods
        if (typeof agent.id !== 'function') {
            throw new Error('agent.id() method not found');
        }
        console.log('✅ agent.id() method exists');

        if (typeof agent.name !== 'function') {
            throw new Error('agent.name() method not found');
        }
        console.log('✅ agent.name() method exists');

        if (typeof agent.description !== 'function') {
            throw new Error('agent.description() method not found');
        }
        console.log('✅ agent.description() method exists');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testAgentWithContextVariables() {
    console.log('\n=== Test 2: Agent with Context Variables ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation'
        });

        // Test: Agent with template variables
        const agent = await new AgentBuilder('Analyzer', llmConfig)
            .systemPrompt('Analyze the data: {{data}}')  // Template with variable
            .build();

        console.log('✅ Created agent with template variable {{data}}');

        const name = await agent.name();
        console.log('✅ Agent name:', name);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testAdvancedAgentConfiguration() {
    console.log('\n=== Test 3: Advanced Agent Configuration ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation',
            model: 'gpt-4o-mini'
        });

        // Test: Agent with advanced configuration
        const agent = await new AgentBuilder('Research Agent', llmConfig)
            .systemPrompt('Conduct research on: {{topic}}')
            .description('Performs detailed research on given topics')
            .maxTokens(1000)
            .temperature(0.7)
            .build();

        console.log('✅ Created agent with advanced configuration');

        const description = await agent.description();
        console.log('✅ Agent description:', description);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testMultiProviderAgents() {
    console.log('\n=== Test 4: Multi-Provider Agent Creation ===');

    try {
        init();

        // Test: OpenAI agent
        const openaiConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key',
            model: 'gpt-4o-mini'
        });

        const openaiAgent = await new AgentBuilder('OpenAI Agent', openaiConfig)
            .systemPrompt('Process with OpenAI')
            .build();

        console.log('✅ Created OpenAI agent');

        // Test: Anthropic agent
        const anthropicConfig = LlmConfig.anthropic({
            apiKey: process.env.ANTHROPIC_API_KEY || 'test-key',
            model: 'claude-3-5-sonnet-20241022'
        });

        const anthropicAgent = await new AgentBuilder('Anthropic Agent', anthropicConfig)
            .systemPrompt('Process with Anthropic')
            .build();

        console.log('✅ Created Anthropic agent');

        // Test: OpenRouter agent
        const openrouterConfig = LlmConfig.openRouter({
            apiKey: process.env.OPENROUTER_API_KEY || 'test-key',
            model: 'anthropic/claude-3.5-sonnet'
        });

        const openrouterAgent = await new AgentBuilder('OpenRouter Agent', openrouterConfig)
            .systemPrompt('Process with OpenRouter')
            .build();

        console.log('✅ Created OpenRouter agent');

        // Test: Ollama agent (local)
        const ollamaConfig = LlmConfig.ollama({
            baseUrl: 'http://localhost:11434',
            model: 'llama2'
        });

        const ollamaAgent = await new AgentBuilder('Ollama Agent', ollamaConfig)
            .systemPrompt('Process with Ollama')
            .build();

        console.log('✅ Created Ollama agent');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testAgentMetadataAccess() {
    console.log('\n=== Test 5: Agent Metadata Access ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation'
        });

        const agent = await new AgentBuilder('Test Agent', llmConfig)
            .systemPrompt('Test prompt')
            .description('Test description for the agent')
            .build();

        // Test: Access all metadata
        const agentId = await agent.id();
        const agentName = await agent.name();
        const agentDesc = await agent.description();

        console.log('✅ agent.id():', agentId.uuid);
        console.log('✅ agent.name():', agentName);
        console.log('✅ agent.description():', agentDesc);

        // Verify metadata types
        if (!agentId || !agentId.uuid) {
            throw new Error('agent.id() should return object with uuid property');
        }

        if (typeof agentName !== 'string') {
            throw new Error('agent.name() should return string');
        }

        if (typeof agentDesc !== 'string') {
            throw new Error('agent.description() should return string');
        }

        console.log('✅ All metadata types validated');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  TEST SUITE: Agent Builder API (from concepts-js.md)');
    console.log('═'.repeat(70));
    console.log('\nValidating AgentBuilder patterns from fixed documentation...\n');

    const results = await Promise.all([
        testBasicAgentCreation(),
        testAgentWithContextVariables(),
        testAdvancedAgentConfiguration(),
        testMultiProviderAgents(),
        testAgentMetadataAccess()
    ]);

    const allPassed = results.every(r => r);

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log('  ✅ ALL TESTS PASSED - AgentBuilder patterns are CORRECT');
    } else {
        console.log('  ❌ SOME TESTS FAILED - Check errors above');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
