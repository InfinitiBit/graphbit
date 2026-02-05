
const { init, WorkflowBuilder, AgentBuilder, Workflow } = require('../../graphbit');

async function runDeepVerification() {
    console.log('🧪 Starting Deep Verification of Workflow Implementation 🧪');
    console.log('======================================================');

    init();

    await verifyEdgeSignature();
    await verifyValidationLogic();
    await verifyValidationCycles();

    console.log('\n✅ Deep Verification Complete');
}

async function verifyEdgeSignature() {
    console.log('\n🔍 Verifying addEdge() Signature Enforcement');
    console.log('-------------------------------------------');

    const workflow = new WorkflowBuilder('Signature Test').build();

    // Create nodes
    await workflow.addNode({
        id: 'node1',
        name: 'Node 1',
        description: 'Test',
        nodeType: 'Agent'
    });
    await workflow.addNode({
        id: 'node2',
        name: 'Node 2',
        description: 'Test',
        nodeType: 'Agent'
    });

    // Test 1: Correct Usage
    try {
        await workflow.addEdge('node1', 'node2', {
            fromNode: 'node1',
            toNode: 'node2'
        });
        console.log('✅ Correct addEdge() usage passed');
    } catch (e) {
        console.error('❌ Correct usage failed:', e.message);
    }

    // Test 2: Missing Edge Object (should fail js signature match or rust parsing)
    try {
        await workflow.addEdge('node1', 'node2'); // Missing 3rd arg
        console.error('❌ Missing edge object DID NOT throw (unexpected)');
    } catch (e) {
        console.log('✅ Missing edge object threw correct error:', e.message);
    }

    // Test 3: Empty Edge Object (missing required fields)
    try {
        await workflow.addEdge('node1', 'node2', {});
        // This relies on how NAPI maps empty object to the struct. 
        // If fields are non-optional in struct, this should fail.
        console.error('❌ Empty edge object DID NOT throw (unexpected)');
    } catch (e) {
        console.log('✅ Empty edge object threw correct error (fields required):', e.message);
    }
}

async function verifyValidationLogic() {
    console.log('\n🔍 Verifying validate() Logic (Orphans & Connectivity)');
    console.log('----------------------------------------------------');

    const workflow = new WorkflowBuilder('Validation Test').build();

    // Add single orphan node
    await workflow.addNode({
        id: 'orphan',
        name: 'Orphan Node',
        description: 'I have no friends',
        nodeType: 'Agent'
    });

    const isValid = await workflow.validate();
    console.log(`Orphan node validation result: ${isValid}`);

    if (isValid === true) {
        console.log('✅ Confirmed: validate() allows disconnected graphs (permissive)');
    } else {
        console.log('ℹ️ Confirmed: validate() enforces connectivity');
    }
}

async function verifyValidationCycles() {
    console.log('\n🔍 Verifying validate() Logic (Cycles)');
    console.log('------------------------------------');

    const workflow = new WorkflowBuilder('Cycle Test').build();

    await workflow.addNode({ id: 'A', name: 'A', description: 'A', nodeType: 'Agent' });
    await workflow.addNode({ id: 'B', name: 'B', description: 'B', nodeType: 'Agent' });

    // Create A -> B
    await workflow.addEdge('A', 'B', { fromNode: 'A', toNode: 'B' });

    // Create B -> A (Cycle)
    await workflow.addEdge('B', 'A', { fromNode: 'B', toNode: 'A' });

    try {
        const isValid = await workflow.validate();
        // Native bind might throw on validation failure, OR return false. 
        // The implementation I read suggests it might return Result<bool> but the inner logic throws errors?
        // Let's see what actually happens.
        console.log(`Cycle validation result: ${isValid}`);
        if (!isValid) {
            console.log('✅ Cycle detected (returned false)');
        } else {
            console.error('❌ Cycle NOT detected (returned true)');
        }
    } catch (e) {
        console.log('✅ Cycle detected (threw error):', e.message);
    }
}

runDeepVerification().catch(console.error);
