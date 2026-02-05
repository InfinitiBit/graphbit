
const { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig, NodeType } = require('../../graphbit');

async function verifyWorkflowBuilder() {
    console.log('Verifying Workflow Builder Documentation Examples (Corrected)...');

    try {
        init();
        console.log('✅ init() called');
    } catch (e) {
        console.log('ℹ️ init skipped/failed:', e.message);
    }

    const config = LlmConfig.openai({
        apiKey: 'sk-fake-key-for-verification',
        model: 'gpt-4o-mini'
    });

    // 1. Create Workflow
    console.log('Step 1: Building Workflow...');
    const workflowBuilder = new WorkflowBuilder('Test Pipeline')
        .description('Verifying documentation');
    const workflow = await workflowBuilder.build();
    console.log('✅ Workflow built');

    // 2. Create Agents (MOCK)
    // We skip the real AgentBuilder because it fails auth without real key.
    const analyzer = {
        name: async () => 'Analyzer',
        description: async () => 'Desc 1'
    };
    const summarizer = {
        name: async () => 'Summarizer',
        description: async () => 'Desc 2'
    };
    console.log('✅ Mock agents ready');

    // 3. Add Nodes
    console.log('Step 3: Adding Nodes...');
    await workflow.addNode({
        id: 'analyzer',
        name: await analyzer.name(), // 'Analyzer'
        description: await analyzer.description(),
        nodeType: 'Agent'
    });
    console.log('✅ Analyzer node added');

    await workflow.addNode({
        id: 'summarizer',
        name: await summarizer.name(),
        description: await summarizer.description(),
        nodeType: 'Agent'
    });
    console.log('✅ Summarizer node added');

    // 4. Connect Nodes (Using 3 args as per corrected doc/types)
    console.log('Step 4: Connecting Nodes...');
    try {
        await workflow.addEdge('analyzer', 'summarizer', {
            fromNode: 'analyzer',
            toNode: 'summarizer'
        });
        console.log('✅ Edge added (3-arg)');
    } catch (e) {
        console.error('❌ Failed to add edge (3-arg):', e);
        process.exit(1);
    }

    // 5. Validate (Success)
    console.log('Step 5: Validating...');
    const isValid = await workflow.validate();
    console.log('DEBUG: workflow.validate() returned:', typeof isValid, isValid);

    if (isValid === true) {
        console.log('✅ Validation passed');
    } else {
        console.error('❌ Validation failed (Expected true)');
        process.exit(1);
    }

    console.log('\n🎉 DOC VERIFICATION SUCCESSFUL: Code is consistent with Types and Runtime.');
}

verifyWorkflowBuilder().catch(err => {
    console.error('❌ FATAL ERROR in verification:', err);
    process.exit(1);
});
