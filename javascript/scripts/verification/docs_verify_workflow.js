const { WorkflowBuilder, RetryableErrorType } = require('../javascript/index');

async function verifyWorkflowUsage() {
    console.log('Verifying Workflow Usage for Documentation...');

    // 1. Create Workflow
    const builder = new WorkflowBuilder('Doc Verification Workflow');
    const workflow = builder.build();

    console.log('✅ Workflow created:', await workflow.name());

    // 2. Add Node
    const agentNode = {
        id: 'agent1',
        name: 'Research Agent',
        description: 'Researches topics',
        nodeType: 'Agent',
        retryConfig: {
            maxAttempts: 3,
            initialDelayMs: 100,
            backoffMultiplier: 2.0,
            maxDelayMs: 1000,
            jitterFactor: 0.1,
            retryableErrors: [RetryableErrorType.NetworkError]
        }
    };

    const nodeId = await workflow.addNode(agentNode);
    console.log(`✅ Node added with ID: ${nodeId}`);

    // 3. Add another node to connect
    const summarizeNode = {
        id: 'agent2',
        name: 'Summarizer',
        description: 'Summarizes text',
        nodeType: 'Agent'
    };
    await workflow.addNode(summarizeNode);

    // 4. Connect (Add Edge)
    // Note: Omit optional fields (like condition) instead of passing null.
    // Note: Must include fromNode/toNode.
    const edge = {
        fromNode: 'agent1',
        toNode: 'agent2'
    };
    await workflow.addEdge('agent1', 'agent2', edge);
    console.log('✅ Edge added between agent1 and agent2');

    // 5. Validate
    const isValid = await workflow.validate();
    console.log(`✅ Workflow validation result: ${isValid}`);
}

verifyWorkflowUsage().catch(console.error);
