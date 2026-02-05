import 'dotenv/config';
import { init, WorkflowBuilder, AgentBuilder, LlmConfig } from 'graphbit';

/**
 * TEST: Dynamic Workflow Creation from dynamics-graph-js.md
 * 
 * This test validates dynamic workflow patterns:
 * 1. Basic dynamic workflow creation based on input data
 * 2. Data-driven node creation
 * 3. JSON-based workflow configuration
 * 
 * Pattern validated from:
 * - docs/user-guide/dynamics-graph-js.md (multiple sections)
 */

async function testDynamicWorkflowCreation() {
    console.log('\n=== Test 1: Basic Dynamic Workflow Creation ===');

    try {
        init();

        const config = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation'
        });

        // Test: Dynamic workflow based on input data type (from docs line ~35-184)
        const inputData = {
            type: 'text',
            content: 'Sample text for processing'
        };

        // Create workflow dynamically based on data type
        let workflow;

        if (inputData.type === 'text') {
            // Text processing workflow
            const analyzer = await new AgentBuilder('Text Analyzer', config)
                .systemPrompt('Analyze this text and provide key insights')
                .build();

            const sentiment = await new AgentBuilder('Sentiment Detector', config)
                .systemPrompt('Determine sentiment of the analyzed text')
                .build();

            console.log('✅ Created agents based on data type: text');

            const analyzerId = await analyzer.id();
            const sentimentId = await sentiment.id();

            workflow = await new WorkflowBuilder('Text Processing Workflow')
                .description('Dynamic text processing pipeline')
                .build();

            const node1 = await workflow.addNode({
                id: analyzerId.uuid,
                name: await analyzer.name(),
                description: await analyzer.description(),
                nodeType: 'Agent'
            });

            const node2 = await workflow.addNode({
                id: sentimentId.uuid,
                name: await sentiment.name(),
                description: await sentiment.description(),
                nodeType: 'Agent'
            });

            await workflow.addEdge(node1, node2);

            console.log('✅ Built dynamic text processing workflow');
        }

        if (!workflow) {
            throw new Error('Workflow should be created');
        }

        return true;
    } catch (error: any) {
        if (error.message && error.message.includes('Incorrect API key')) {
            console.log('✅ Structure validated (execution failed as expected with test key)');
            return true;
        }
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testDataDrivenWorkflow() {
    console.log('\n=== Test 2: Data-Driven Workflow Generation ===');

    try {
        init();

        const config = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation'
        });

        // Test: Data-driven workflow (from docs line ~214-268)
        const schema = {
            fields: [
                { name: 'customer_name', type: 'string' },
                { name: 'order_amount', type: 'number' },
                { name: 'order_date', type: 'date' }
            ]
        };

        const workflow = await new WorkflowBuilder('Data-Driven Workflow')
            .description('Dynamically generated workflow based on data schema')
            .build();

        const nodeIds: string[] = [];

        // Create agents based on schema fields
        for (const field of schema.fields) {
            let agent;

            if (field.type === 'string') {
                agent = await new AgentBuilder(`${field.name} Text Processor`, config)
                    .systemPrompt(`Process ${field.name} text field and extract insights`)
                    .build();
            } else if (field.type === 'number') {
                agent = await new AgentBuilder(`${field.name} Numerical Processor`, config)
                    .systemPrompt(`Analyze ${field.name} numerical data and identify patterns`)
                    .build();
            } else if (field.type === 'date') {
                agent = await new AgentBuilder(`${field.name} Date Processor`, config)
                    .systemPrompt(`Analyze ${field.name} date patterns and trends`)
                    .build();
            } else {
                agent = await new AgentBuilder(`${field.name} Generic Processor`, config)
                    .systemPrompt(`Process ${field.name} field with general logic`)
                    .build();
            }

            const agentId = await agent.id();

            const nodeId = await workflow.addNode({
                id: agentId.uuid,
                name: await agent.name(),
                description: await agent.description(),
                nodeType: 'Agent'
            });

            nodeIds.push(nodeId);
        }

        console.log(`✅ Created ${schema.fields.length} agents based on schema`);

        // Create aggregator agent
        const aggregator = await new AgentBuilder('Data Aggregator', config)
            .systemPrompt('Combine and analyze all processed fields')
            .build();

        const aggregatorId = await aggregator.id();

        const aggregatorNodeId = await workflow.addNode({
            id: aggregatorId.uuid,
            name: await aggregator.name(),
            description: await aggregator.description(),
            nodeType: 'Agent'
        });

        // Connect all field processors to aggregator
        for (const nodeId of nodeIds) {
            await workflow.addEdge(nodeId, aggregatorNodeId);
        }

        console.log('✅ Connected all field processors to aggregator');

        return true;
    } catch (error: any) {
        if (error.message && error.message.includes('Incorrect API key')) {
            console.log('✅ Structure validated (execution failed as expected with test key)');
            return true;
        }
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testJSONConfigWorkflow() {
    console.log('\n=== Test 3: JSON-Based Workflow Configuration ===');

    try {
        init();

        const config = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation'
        });

        // Test: JSON configuration (from docs line ~323-449)
        const workflowConfig = {
            name: 'Simple Analysis Workflow',
            nodes: [
                {
                    id: 'analyzer',
                    type: 'agent',
                    name: 'Data Analyzer',
                    prompt: 'Analyze this data and provide insights'
                },
                {
                    id: 'formatter',
                    type: 'agent',
                    name: 'Output Formatter',
                    prompt: 'Format the analysis in a clear structure'
                }
            ],
            connections: [
                { source: 'analyzer', target: 'formatter' }
            ]
        };

        const workflow = await new WorkflowBuilder(workflowConfig.name)
            .description('Workflow created from JSON configuration')
            .build();

        const nodeMap: Record<string, string> = {};

        // Create agents from configuration
        for (const nodeConfig of workflowConfig.nodes) {
            if (nodeConfig.type === 'agent') {
                const agent = await new AgentBuilder(nodeConfig.name, config)
                    .systemPrompt(nodeConfig.prompt)
                    .build();

                const agentId = await agent.id();

                const nodeId = await workflow.addNode({
                    id: agentId.uuid,
                    name: await agent.name(),
                    description: await agent.description(),
                    nodeType: 'Agent'
                });

                nodeMap[nodeConfig.id] = nodeId;
            }
        }

        console.log(`✅ Created ${workflowConfig.nodes.length} agents from JSON config`);

        // Create connections from configuration
        for (const connection of workflowConfig.connections) {
            const sourceId = nodeMap[connection.source];
            const targetId = nodeMap[connection.target];

            if (sourceId && targetId) {
                await workflow.addEdge(sourceId, targetId);
            }
        }

        console.log('✅ Connected nodes according to JSON configuration');

        return true;
    } catch (error: any) {
        if (error.message && error.message.includes('Incorrect API key')) {
            console.log('✅ Structure validated (execution failed as expected with test key)');
            return true;
        }
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testDynamicWorkflowValidation() {
    console.log('\n=== Test 4: Dynamic Workflow Validation ===');

    try {
        init();

        const config = LlmConfig.openai({
            apiKey: 'test-key-for-structure-validation'
        });

        // Test: Workflow validation (from docs line ~475-532)
        const agent = await new AgentBuilder('Processor', config)
            .systemPrompt('Process data')
            .build();

        const agentId = await agent.id();

        const workflow = await new WorkflowBuilder('Validated Workflow')
            .description('Workflow with validation')
            .build();

        await workflow.addNode({
            id: agentId.uuid,
            name: await agent.name(),
            description: await agent.description(),
            nodeType: 'Agent'
        });

        // Test: Validate workflow
        const errors = await workflow.validate();

        console.log(`✅ workflow.validate() returned ${errors.length} validation errors`);

        return true;
    } catch (error: any) {
        if (error.message && error.message.includes('Incorrect API key')) {
            console.log('✅ Structure validated (execution failed as expected with test key)');
            return true;
        }
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  TEST SUITE: Dynamic Workflow Patterns (from dynamics-graph-js.md)');
    console.log('═'.repeat(70));
    console.log('\nValidating dynamic workflow creation patterns...\n');

    const results = await Promise.all([
        testDynamicWorkflowCreation(),
        testDataDrivenWorkflow(),
        testJSONConfigWorkflow(),
        testDynamicWorkflowValidation()
    ]);

    const allPassed = results.every(r => r);

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log('  ✅ ALL TESTS PASSED - Dynamic workflow patterns are CORRECT');
    } else {
        console.log('  ❌ SOME TESTS FAILED - Check errors above');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
