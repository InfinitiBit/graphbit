const { WorkflowGraph } = require('../../index.js');

function verifyWorkflowGraph() {
    console.log('🧪 Testing WorkflowGraph Example\n');

    try {
        const graph = new WorkflowGraph();
        console.log('✅ new WorkflowGraph() works');

        // Add nodes with ALL required fields
        graph.addNode({
            id: 'node1',
            name: 'Node 1',
            description: 'First node',
            nodeType: 'Agent',
            config: {}
        });
        graph.addNode({
            id: 'node2',
            name: 'Node 2',
            description: 'Second node',
            nodeType: 'Agent',
            config: {}
        });
        graph.addNode({
            id: 'node3',
            name: 'Node 3',
            description: 'Third node',
            nodeType: 'Agent',
            config: {}
        });
        console.log('✅ graph.addNode() works');

        // Add edges
        graph.addEdge({ from: 'node1', to: 'node2' });
        graph.addEdge({ from: 'node2', to: 'node3' });
        console.log('✅ graph.addEdge() works');

        // Validate
        graph.validate();
        console.log('✅ graph.validate() works');

        // Get execution order
        const order = graph.topologicalSort();
        console.log(`✅ graph.topologicalSort() = [${order.join(', ')}]`);

        // Check properties
        console.log(`✅ graph.nodeCount() = ${graph.nodeCount()}`);
        console.log(`✅ graph.edgeCount() = ${graph.edgeCount()}`);
        console.log(`✅ graph.isEmpty() = ${graph.isEmpty()}`);

        console.log('\n✨ WorkflowGraph example VERIFIED!');
        return true;

    } catch (error) {
        console.error('❌ Error:', error.message);
        return false;
    }
}

const success = verifyWorkflowGraph();
process.exit(success ? 0 : 1);
