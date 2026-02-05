import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

/**
 * Test 03: Workflow Builder
 * Demonstrates correct workflow construction using WorkflowBuilder and AgentBuilder
 */

async function testWorkflowBuilder() {
    console.log('=== Test 03: Workflow Builder ===\n');

    try {
        init();

        // Create LLM config
        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key'
        });
        console.log('✅ Created LLM config');

        // Create agents using AgentBuilder
        const researcher = await new AgentBuilder('Researcher', llmConfig)
            .systemPrompt('You are a research assistant.')
            .build();

        const writer = await new AgentBuilder('Writer', llmConfig)
            .systemPrompt('You are a writing assistant.')
            .build();

        console.log('✅ Created agents');

        // Create workflow using WorkflowBuilder
        const workflowBuilder = new WorkflowBuilder('ContentPipeline')
            .description('Research and writing pipeline');

        const workflow = await workflowBuilder.build();
        console.log('✅ Created workflow');

        // Add nodes to workflow
        await workflow.addNode({
            id: 'researcher',
            name: await researcher.name(),
            description: await researcher.description(),
            nodeType: 'Agent'
        });

        await workflow.addNode({
            id: 'writer',
            name: await writer.name(),
            description: await writer.description(),
            nodeType: 'Agent'
        });

        console.log('✅ Added nodes to workflow');

        // Add edge
        await workflow.addEdge('researcher', 'writer');
        console.log('✅ Added edge');

        // Validate workflow
        const errors = await workflow.validate();
        console.log(`✅ Validation errors: ${errors.length}`);

        // Create executor
        const executor = new Executor(llmConfig);
        console.log('✅ Created executor');

        console.log('\n✅ Workflow Builder test passed!');
    } catch (error) {
        console.error('❌ Workflow Builder test failed:', error);
        throw error;
    }
}

testWorkflowBuilder();
