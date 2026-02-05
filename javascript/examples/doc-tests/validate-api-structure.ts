import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, LlmConfig, ToolRegistry } from 'graphbit';

/**
 * API Structure Validation - NO API CALLS
 * 
 * This validates the CORRECT GraphBit JavaScript API structure
 * WITHOUT making actual API calls that require keys.
 */

async function validateAPIStructure() {
    console.log('\n=== GraphBit JavaScript API Structure Validation ===\n');

    try {
        init();

        const llmConfig = LlmConfig.openai({
            apiKey: 'fake-key-for-structure-test'
        });

        console.log('✅ init() works');
        console.log('✅ LlmConfig.openai() works\n');

        // ==============================================
        // TEST 1: WorkflowBuilder API
        // ==============================================
        console.log('TEST 1: WorkflowBuilder API Structure');
        console.log('─'.repeat(50));

        const workflowBuilder = new WorkflowBuilder('TestWorkflow')
            .description('Test workflow');

        console.log('✅ new WorkflowBuilder(name) creates builder');
        console.log('✅ .description() method exists');

        // Check if WRONG methods exist
        const hasAddAgent = typeof (workflowBuilder as any).addAgent === 'function';
        const hasConnect = typeof (workflowBuilder as any).connect === 'function';

        if (hasAddAgent || hasConnect) {
            console.log('❌ ERROR: WorkflowBuilder has .addAgent() or .connect()!');
            console.log('   This means the API changed or documentation is outdated');
            return false;
        }

        console.log('✅ CONFIRMED: .addAgent() DOES NOT exist on WorkflowBuilder');
        console.log('✅ CONFIRMED: .connect() DOES NOT exist on WorkflowBuilder\n');

        // Build the workflow
        const workflow = await workflowBuilder.build();
        console.log('✅ await workflowBuilder.build() returns workflow\n');

        // ==============================================
        // TEST 2: Workflow (after build) API
        // ==============================================
        console.log('TEST 2: Built Workflow API Structure');
        console.log('─'.repeat(50));

        const hasAddNode = typeof (workflow as any).addNode === 'function';
        const hasAddEdge = typeof (workflow as any).addEdge === 'function';
        const hasValidate = typeof (workflow as any).validate === 'function';

        if (!hasAddNode) {
            console.log('❌ ERROR: workflow.addNode() does NOT exist!');
            return false;
        }
        console.log('✅ CONFIRMED: .addNode() EXISTS on built workflow');

        if (!hasAddEdge) {
            console.log('❌ ERROR: workflow.addEdge() does NOT exist!');
            return false;
        }
        console.log('✅ CONFIRMED: .addEdge() EXISTS on built workflow');

        if (!hasValidate) {
            console.log('❌ ERROR: workflow.validate() does NOT exist!');
            return false;
        }
        console.log('✅ CONFIRMED: .validate() EXISTS on built workflow\n');

        // ==============================================
        // TEST 3: AgentBuilder API
        // ==============================================
        console.log('TEST 3: AgentBuilder API Structure');
        console.log('─'.repeat(50));

        const agentBuilder = new AgentBuilder('TestAgent', llmConfig)
            .systemPrompt('test')
            .description('test agent');

        console.log('✅ new AgentBuilder(name, config) creates builder');
        console.log('✅ .systemPrompt() method exists');
        console.log('✅ .description() method exists\n');

        // ==============================================
        // TEST 4: ToolRegistry API
        // ==============================================
        console.log('TEST 4: ToolRegistry API Structure');
        console.log('─'.repeat(50));

        const registry = new ToolRegistry();
        console.log('✅ new ToolRegistry() creates registry');

        const hasRegister = typeof (registry as any).register === 'function';
        const hasExecute = typeof (registry as any).execute === 'function';

        if (!hasRegister) {
            console.log('❌ ERROR: registry.register() does NOT exist!');
            return false;
        }
        console.log('✅ CONFIRMED: .register() EXISTS on ToolRegistry');

        if (!hasExecute) {
            console.log('❌ ERROR: registry.execute() does NOT exist!');
            return false;
        }
        console.log('✅ CONFIRMED: .execute() EXISTS on ToolRegistry\n');

        // ==============================================
        // SUMMARY
        // ==============================================
        console.log('═'.repeat(60));
        console.log('              ✅ ALL VALIDATIONS PASSED!');
        console.log('═'.repeat(60));
        console.log('\n📋 CORRECT API PATTERNS FOR DOCUMENTATION:\n');
        console.log('1. Build workflow FIRST:');
        console.log('   const workflow = await new WorkflowBuilder(name)');
        console.log('     .description(desc)');
        console.log('     .build();\n');

        console.log('2. Create agents:');
        console.log('   const agent = await new AgentBuilder(name, config)');
        console.log('     .systemPrompt(prompt)');
        console.log('     .build();\n');

        console.log('3. Add nodes to BUILT workflow:');
        console.log('   await workflow.addNode({');
        console.log('     id: agentId.uuid,');
        console.log('    name: agentName,');
        console.log('     description: agentDesc,');
        console.log('     nodeType: "Agent"');
        console.log('   });\n');

        console.log('4. Connect nodes with edges:');
        console.log('   await workflow.addEdge(sourceNodeId, targetNodeId);\n');

        console.log('5. Validate workflow:');
        console.log('   const errors = await workflow.validate();\n');

        console.log('═'.repeat(60));
        console.log('\n❌ INCORRECT PATTERNS (DO NOT USE):\n');
        console.log('✗ workflowBuilder.addAgent(agent)  // Method does not exist!');
        console.log('✗ workflowBuilder.connect(a, b)    // Method does not exist!\n');
        console.log('═'.repeat(60));

        return true;
    } catch (error) {
        console.error('\n❌ Validation failed:', error);
        return false;
    }
}

async function main() {
    const success = await validateAPIStructure();
    process.exit(success ? 0 : 1);
}

main();
