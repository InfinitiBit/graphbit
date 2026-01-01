const {
    WorkflowGraph,
    ToolRegistry,
    Agent,
    Workflow,
    Executor,
    WorkflowBuilder,
    AgentBuilder
} = require('../../index');

const EXPECTED_METHODS = {
    WorkflowGraph: [
        'nodeCount', 'edgeCount', 'isEmpty', 'validate',
        'addNode', 'addEdge', 'getNode', 'getNodes',
        'topologicalSort', 'hasCycles',
        'getDependencies', 'getDependents',
        'getRootNodes', 'getLeafNodes'
    ],
    ToolRegistry: [
        'register', 'getTool', 'hasTool',
        'getRegisteredTools', 'execute'
    ],
    Agent: [
        'id', 'name', 'description', 'execute', 'config'
    ],
    Workflow: [
        'id', 'name', 'description', 'validate'
    ],
    Executor: [
        'execute'
    ],
    WorkflowBuilder: [
        'build', 'description', 'addMetadata'
    ],
    AgentBuilder: [
        'build', 'systemPrompt', 'description', 'temperature', 'maxTokens'
    ]
};

function checkMethods() {
    console.log('Verifying Runtime API Methods...\n');
    let totalClasses = 0;
    let totalMethods = 0;
    let missingMethods = 0;

    for (const [className, methods] of Object.entries(EXPECTED_METHODS)) {
        console.log(`Checking ${className}...`);
        totalClasses++;

        // Get the class from the module
        const Class = require('../../index')[className];

        if (!Class) {
            console.error(`❌ Class ${className} not found in exports!`);
            missingMethods += methods.length;
            continue;
        }

        // Inspect prototype for instance methods
        const proto = Class.prototype;
        const actualMethods = Object.getOwnPropertyNames(proto);

        for (const method of methods) {
            totalMethods++;

            let exists = false;
            try {
                // Try to instantiate (might fail if constructor requires args)
                if (className === 'WorkflowGraph') {
                    const instance = new Class();
                    exists = typeof instance[method] === 'function';
                } else if (className === 'ToolRegistry') {
                    const instance = new Class();
                    exists = typeof instance[method] === 'function';
                } else {
                    // Fallback to prototype check
                    exists = actualMethods.includes(method);
                }
            } catch (e) {
                // If instantiation fails, fall back to prototype check
                exists = actualMethods.includes(method);
            }

            if (exists) {
                console.log(`  ✅ ${method}`);
            } else {
                // Double check if it's a static method?
                if (typeof Class[method] === 'function') {
                    console.log(`  ✅ ${method} (static)`);
                } else {
                    console.log(`  ❌ ${method} - Missing`);
                    missingMethods++;
                }
            }
        }
        console.log('');
    }

    console.log('-'.repeat(50));
    console.log(`Total Classes Checked: ${totalClasses}`);
    console.log(`Total Methods Checked: ${totalMethods}`);
    console.log(`Missing Methods: ${missingMethods}`);

    if (missingMethods === 0) {
        console.log('\n✨ All expected methods are present!');
    } else {
        console.log('\n⚠️  Some methods are missing.');
        process.exit(1);
    }
}

checkMethods();
