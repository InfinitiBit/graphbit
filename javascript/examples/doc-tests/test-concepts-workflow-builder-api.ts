import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, LlmConfig } from 'graphbit';

/**
 * TEST: Workflow Builder API from concepts-js.md
 * 
 * This test validates the CORRECT API patterns used in the fixed documentation:
 * 1. WorkflowBuilder creates workflow with .build()
 * 2. workflow.addNode() adds nodes to BUILT workflow
 * 3. workflow.addEdge() connects nodes
 * 
 * Pattern validated from:
 * - docs/user-guide/concepts-js.md (lines 270-283, 285-316, 318-363)
 */

async function testWorkflowBuilderBasic() {
    console.log('\n=== Test 1: Basic WorkflowBuilder Pattern ===');

    try {
        init();

        // Test: Create workflow with WorkflowBuilder
        const workflowBuilder = new WorkflowBuilder('TestWorkflow')
            .description('A sample workflow');

        console.log('✅ WorkflowBuilder instantiated');

        // Test: Build workflow
        const workflow = await workflowBuilder.build();
        console.log('✅ WorkflowBuilder.build() returns workflow');

        // Verify workflow has required methods
        if (typeof workflow.addNode !== 'function') {
            throw new Error('workflow.addNode() method not found');
        }
        console.log('✅ workflow.addNode() method exists');

        if (typeof workflow.addEdge !== 'function') {
            throw new Error('workflow.addEdge() method not found');
        }
        console.log('✅ workflow.addEdge() method exists');

        if (typeof workflow.validate !== 'function') {
            throw new Error('workflow.validate() method not found');
        }
        console.log('✅ workflow.validate() method exists');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testWorkflowWithAgent() {
    console.log('\n=== Test 2: Workflow with Agent Pattern ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation'
        });

        // Test: Create agent with AgentBuilder
        const agent = await new AgentBuilder('Analyzer', llmConfig)
            .systemPrompt('Analyze this input')
            .build();

        console.log('✅ AgentBuilder.build() returns agent');

        // Test: Get agent metadata
        const agentId = await agent.id();
        const agentName = await agent.name();
        const agentDesc = await agent.description();

        console.log('✅ agent.id() returns:', agentId.uuid.substring(0, 8) + '...');
        console.log('✅ agent.name() returns:', agentName);
        console.log('✅ agent.description() returns:', agentDesc);

        // Test: Build workflow
        const workflow = await new WorkflowBuilder('SimpleWorkflow')
            .description('Workflow with one agent')
            .build();

        console.log('✅ Workflow built');

        // Test: Add agent as node (CRITICAL PATTERN from docs)
        const nodeId = await workflow.addNode({
            id: agentId.uuid,
            name: agentName,
            description: agentDesc,
            nodeType: 'Agent'
        });

        console.log('✅ workflow.addNode() added agent node:', nodeId.substring(0, 8) + '...');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testConnectedWorkflow() {
    console.log('\n=== Test 3: Connected Workflow Pattern ===');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key-for-structure-validation'
        });

        // Create multiple agents
        const agent1 = await new AgentBuilder('Agent 1', llmConfig)
            .systemPrompt('First task')
            .build();

        const agent2 = await new AgentBuilder('Agent 2', llmConfig)
            .systemPrompt('Second task')
            .build();

        const agent3 = await new AgentBuilder('Agent 3', llmConfig)
            .systemPrompt('Third task')
            .build();

        console.log('✅ Created 3 agents');

        // Get agent metadata
        const agent1Id = await agent1.id();
        const agent2Id = await agent2.id();
        const agent3Id = await agent3.id();

        // Build workflow
        const workflow = await new WorkflowBuilder('ConnectedWorkflow')
            .description('Sequential workflow with three agents')
            .build();

        console.log('✅ Workflow built');

        // Add nodes
        const node1Id = await workflow.addNode({
            id: agent1Id.uuid,
            name: await agent1.name(),
            description: await agent1.description(),
            nodeType: 'Agent'
        });

        const node2Id = await workflow.addNode({
            id: agent2Id.uuid,
            name: await agent2.name(),
            description: await agent2.description(),
            nodeType: 'Agent'
        });

        const node3Id = await workflow.addNode({
            id: agent3Id.uuid,
            name: await agent3.name(),
            description: await agent3.description(),
            nodeType: 'Agent'
        });

        console.log('✅ Added 3 nodes to workflow');

        // Test: Connect nodes with edges (CRITICAL PATTERN from docs)
        await workflow.addEdge(node1Id, node2Id);
        await workflow.addEdge(node2Id, node3Id);

        console.log('✅ workflow.addEdge() connected nodes: agent_1 → agent_2 → agent_3');

        // Test: Validate workflow
        const errors = await workflow.validate();
        console.log(`✅ workflow.validate() returned ${errors.length} errors`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testWorkflowBuilderMethods() {
    console.log('\n=== Test 4: Verify WorkflowBuilder Does NOT Have Incorrect Methods ===');

    try {
        init();

        const workflowBuilder = new WorkflowBuilder('Test');

        // Test: Verify .addAgent() does NOT exist (this was the ERROR in original docs)
        if (typeof (workflowBuilder as any).addAgent === 'function') {
            throw new Error('ERROR: .addAgent() should NOT exist on WorkflowBuilder!');
        }
        console.log('✅ Confirmed: .addAgent() does NOT exist on WorkflowBuilder (CORRECT)');

        // Test: Verify .connect() does NOT exist (this was the ERROR in original docs)
        if (typeof (workflowBuilder as any).connect === 'function') {
            throw new Error('ERROR: .connect() should NOT exist on WorkflowBuilder!');
        }
        console.log('✅ Confirmed: .connect() does NOT exist on WorkflowBuilder (CORRECT)');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  TEST SUITE: Workflow Builder API (from concepts-js.md)');
    console.log('═'.repeat(70));
    console.log('\nValidating CORRECT API patterns from fixed documentation...\n');

    const results = await Promise.all([
        testWorkflowBuilderBasic(),
        testWorkflowWithAgent(),
        testConnectedWorkflow(),
        testWorkflowBuilderMethods()
    ]);

    const allPassed = results.every(r => r);

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log('  ✅ ALL TESTS PASSED - Documentation patterns are CORRECT');
    } else {
        console.log('  ❌ SOME TESTS FAILED - Check errors above');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
