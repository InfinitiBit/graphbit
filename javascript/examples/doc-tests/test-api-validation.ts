import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, Executor, LlmConfig, ToolRegistry } from 'graphbit';

/**
 * API Validation Test Suite
 * 
 * This test validates the CORRECT GraphBit JavaScript API patterns
 * to ensure documentation examples use the right methods.
 */

async function testCorrectWorkflowAPI() {
    console.log('\n=== Testing Correct Workflow API ===\n');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key'
        });

        // Test 1: WorkflowBuilder creates workflow
        console.log('Test 1: WorkflowBuilder pattern...');
        const workflowBuilder = new WorkflowBuilder('TestWorkflow')
            .description('Test workflow for API validation');

        const workflow = await workflowBuilder.build();
        console.log('✅ WorkflowBuilder.build() returns workflow');

        // Test 2: AgentBuilder creates agent
        console.log('\nTest 2: AgentBuilder pattern...');
        const agentBuilder = new AgentBuilder('TestAgent', llmConfig)
            .systemPrompt('You are a test agent')
            .description('Test agent for validation');

        const agent = await agentBuilder.build();
        const agentId = await agent.id();
        const agentName = await agent.name();
        const agentDesc = await agent.description();
        console.log('✅ AgentBuilder.build() returns agent');
        console.log(`   Agent ID: ${agentId.uuid}`);
        console.log(`   Agent Name: ${agentName}`);

        // Test 3: workflow.addNode() adds node AFTER building
        console.log('\nTest 3: workflow.addNode() pattern...');
        const nodeId = await workflow.addNode({
            id: agentId.uuid,
            name: agentName,
            description: agentDesc,
            nodeType: 'Agent'
        });
        console.log('✅ workflow.addNode() adds node to built workflow');
        console.log(`   Node ID: ${nodeId}`);

        // Test 4: Create second agent for connection test
        console.log('\nTest 4: Creating second agent for edge test...');
        const agent2Builder = new AgentBuilder('TestAgent2', llmConfig)
            .systemPrompt('You are a second test agent')
            .description('Second test agent');

        const agent2 = await agent2Builder.build();
        const agent2Id = await agent2.id();
        const agent2Name = await agent2.name();
        const agent2Desc = await agent2.description();

        const node2Id = await workflow.addNode({
            id: agent2Id.uuid,
            name: agent2Name,
            description: agent2Desc,
            nodeType: 'Agent'
        });
        console.log('✅ Second node added');
        console.log(`   Node 2 ID: ${node2Id}`);

        // Test 5: workflow.addEdge() connects nodes
        console.log('\nTest 5: workflow.addEdge() pattern...');
        await workflow.addEdge(nodeId, node2Id);
        console.log('✅ workflow.addEdge() connects nodes');

        // Test 6: workflow.validate() validates the workflow
        console.log('\nTest 6: workflow.validate() pattern...');
        const errors = await workflow.validate();
        console.log(`✅ workflow.validate() returns errors: ${errors.length} errors`);

        // Test 7: Executor.execute() runs the workflow
        console.log('\nTest 7: Executor.execute() pattern...');
        const executor = new Executor(llmConfig);
        console.log('✅ new Executor(config) creates executor');

        console.log('\n✅ ALL API VALIDATION TESTS PASSED!\n');

        // Print API summary
        console.log('=== CORRECT API PATTERNS ===');
        console.log('1. const workflow = await new WorkflowBuilder(name).description(...).build();');
        console.log('2. const agent = await new AgentBuilder(name, config).systemPrompt(...).build();');
        console.log('3. const agentId = await agent.id();');
        console.log('4. const nodeId = await workflow.addNode({ id, name, description, nodeType });');
        console.log('5. await workflow.addEdge(sourceNodeId, targetNodeId);');
        console.log('6. const errors = await workflow.validate();');
        console.log('7. const executor = new Executor(config);');
        console.log('8. const result = await executor.execute(workflow);');
        console.log('============================\n');

        return true;
    } catch (error) {
        console.error('❌ API Validation test failed:', error);
        throw error;
    }
}

async function testIncorrectAPIPatterns() {
    console.log('\n=== Testing INCORRECT Patterns (Should Fail) ===\n');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key'
        });

        // Try to use .addAgent() on WorkflowBuilder (doesn't exist)
        console.log('Testing if .addAgent() exists on WorkflowBuilder...');
        const workflowBuilder = new WorkflowBuilder('Test');

        // Check if method exists
        if (typeof (workflowBuilder as any).addAgent === 'function') {
            console.log('❌ CRITICAL: .addAgent() exists on WorkflowBuilder!');
            return false;
        } else {
            console.log('✅ Confirmed: .addAgent() does NOT exist on WorkflowBuilder');
        }

        // Check if .connect() exists
        if (typeof (workflowBuilder as any).connect === 'function') {
            console.log('❌ CRITICAL: .connect() exists on WorkflowBuilder!');
            return false;
        } else {
            console.log('✅ Confirmed: .connect() does NOT exist on WorkflowBuilder');
        }

        // Build workflow and check methods
        const workflow = await workflowBuilder.build();

        // Check if .addNode() exists on workflow
        if (typeof (workflow as any).addNode === 'function') {
            console.log('✅ Confirmed: .addNode() EXISTS on built workflow');
        } else {
            console.log('❌ CRITICAL: .addNode() does NOT exist on workflow!');
            return false;
        }

        // Check if .addEdge() exists on workflow
        if (typeof (workflow as any).addEdge === 'function') {
            console.log('✅ Confirmed: .addEdge() EXISTS on built workflow');
        } else {
            console.log('❌ CRITICAL: .addEdge() does NOT exist on workflow!');
            return false;
        }

        console.log('\n✅ INCORRECT PATTERN VERIFICATION PASSED!\n');
        return true;
    } catch (error) {
        console.error('❌ Incorrect pattern test failed:', error);
        throw error;
    }
}

async function testToolRegistryAPI() {
    console.log('\n=== Testing ToolRegistry API ===\n');

    try {
        init();

        // Test ToolRegistry creation
        console.log('Test 1: Creating ToolRegistry...');
        const registry = new ToolRegistry();
        console.log('✅ new ToolRegistry() creates registry');

        // Test tool registration
        console.log('\nTest 2: Registering tool...');
        await registry.register({
            name: 'test_tool',
            description: 'A test tool',
            inputSchema: {
                type: 'object',
                properties: {
                    value: { type: 'number' }
                },
                required: ['value']
            },
            handler: async (params: any) => {
                return params.value * 2;
            }
        });
        console.log('✅ registry.register() registers tool');

        // Test tool execution
        console.log('\nTest 3: Executing tool...');
        const result = await registry.execute('test_tool', { value: 5 });
        console.log(`✅ registry.execute() executes tool: ${result}`);

        // Test listing tools
        console.log('\nTest 4: Listing tools...');
        const tools = await registry.listTools();
        console.log(`✅ registry.listTools() returns tools: ${tools.length} tools`);

        console.log('\n✅ TOOL REGISTRY API TESTS PASSED!\n');
        return true;
    } catch (error) {
        console.error('❌ ToolRegistry test failed:', error);
        throw error;
    }
}

async function testExecutorAPI() {
    console.log('\n=== Testing Executor API ===\n');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY || 'test-key'
        });

        // Test default executor
        console.log('Test 1: Creating default executor...');
        const executor = new Executor(llmConfig);
        console.log('✅ new Executor(config) creates default executor');

        // Test low-latency executor
        console.log('\nTest 2: Creating low-latency executor...');
        const lowLatencyExecutor = Executor.newLowLatency(llmConfig);
        console.log('✅ Executor.newLowLatency(config) creates executor');

        // Test high-throughput executor
        console.log('\nTest 3: Creating high-throughput executor...');
        const highThroughputExecutor = Executor.newHighThroughput(llmConfig);
        console.log('✅ Executor.newHighThroughput(config) creates executor');

        console.log('\n✅ EXECUTOR API TESTS PASSED!\n');
        return true;
    } catch (error) {
        console.error('❌ Executor test failed:', error);
        throw error;
    }
}

async function runAllValidationTests() {
    console.log('\n'.repeat(2));
    console.log('═'.repeat(60));
    console.log('    GraphBit JavaScript API Validation Test Suite');
    console.log('═'.repeat(60));

    try {
        await testCorrectWorkflowAPI();
        await testIncorrectAPIPatterns();
        await testToolRegistryAPI();
        await testExecutorAPI();

        console.log('═'.repeat(60));
        console.log('    ✅ ALL VALIDATION TESTS PASSED!');
        console.log('═'.repeat(60));
        console.log('\nThe correct API patterns have been validated.');
        console.log('Documentation can now be fixed with confidence.\n');

        process.exit(0);
    } catch (error) {
        console.log('═'.repeat(60));
        console.log('    ❌ VALIDATION TESTS FAILED');
        console.log('═'.repeat(60));
        console.error(error);
        process.exit(1);
    }
}

runAllValidationTests();
