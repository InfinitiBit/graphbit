const { createToolRegistry, tool } = require('../javascript/index');

async function verifyToolsUsage() {
    console.log('Verifying Tool System for Documentation...');

    // 1. Test createToolRegistry()
    try {
        const registry = createToolRegistry();
        console.log('✅ createToolRegistry() works');
    } catch (error) {
        console.error('❌ createToolRegistry() failed:', error);
        process.exit(1);
    }

    // 2. Test tool() helper function
    try {
        const myTool = tool(
            'calculator',
            'Performs basic math calculations',
            { operation: 'string', a: 'number', b: 'number' },
            (args) => {
                const { operation, a, b } = args;
                switch (operation) {
                    case 'add': return a + b;
                    case 'subtract': return a - b;
                    case 'multiply': return a * b;
                    case 'divide': return a / b;
                    default: throw new Error('Unknown operation');
                }
            }
        );

        console.log('✅ tool() helper works');
        console.log('  Tool name:', myTool.name);
        console.log('  Description:', myTool.description);
    } catch (error) {
        console.error('❌ tool() helper failed:', error);
        process.exit(1);
    }

    // 3. Test ToolRegistry.register()
    try {
        const registry = createToolRegistry();

        registry.register(
            'greet',
            'Greets a person by name',
            { name: 'string' },
            (args) => `Hello, ${args.name}!`
        );

        console.log('✅ ToolRegistry.register() works');
    } catch (error) {
        console.error('❌ ToolRegistry.register() failed:', error);
        process.exit(1);
    }

    // 4. Test ToolRegistry.execute()
    try {
        const registry = createToolRegistry();

        registry.register(
            'add',
            'Adds two numbers',
            { a: 'number', b: 'number' },
            (args) => args.a + args.b
        );

        const result = await registry.execute('add', { a: 5, b: 3 });

        if (result.success && result.result === 8) {
            console.log('✅ ToolRegistry.execute() works:', result.result);
        } else {
            throw new Error('Execute returned unexpected result');
        }

        console.log('  Execution time:', result.executionTimeMs, 'ms');
    } catch (error) {
        console.error('❌ ToolRegistry.execute() failed:', error);
        process.exit(1);
    }

    // 5. Test ToolRegistry.getTool()
    try {
        const registry = createToolRegistry();

        registry.register('test', 'Test tool', {}, () => 'test');

        const toolDef = registry.getTool('test');

        if (toolDef) {
            console.log('✅ ToolRegistry.getTool() works');
        } else {
            throw new Error('getTool returned null');
        }
    } catch (error) {
        console.error('❌ ToolRegistry.getTool() failed:', error);
        process.exit(1);
    }

    // 6. Test ToolRegistry.hasTool()
    try {
        const registry = createToolRegistry();

        registry.register('exists', 'Exists', {}, () => true);

        const exists = registry.hasTool('exists');
        const notExists = registry.hasTool('nonexistent');

        if (exists && !notExists) {
            console.log('✅ ToolRegistry.hasTool() works');
        } else {
            throw new Error('hasTool returned unexpected result');
        }
    } catch (error) {
        console.error('❌ ToolRegistry.hasTool() failed:', error);
        process.exit(1);
    }

    // 7. Test ToolRegistry.getRegisteredTools()
    try {
        const registry = createToolRegistry();

        registry.register('tool1', 'Tool 1', {}, () => 1);
        registry.register('tool2', 'Tool 2', {}, () => 2);

        const tools = registry.getRegisteredTools();

        if (Array.isArray(tools) && tools.length === 2) {
            console.log('✅ ToolRegistry.getRegisteredTools() works:', tools);
        } else {
            throw new Error('getRegisteredTools returned unexpected result');
        }
    } catch (error) {
        console.error('❌ ToolRegistry.getRegisteredTools() failed:', error);
        process.exit(1);
    }

    console.log('\n📊 Tool System methods verified:');
    console.log('  - createToolRegistry()');
    console.log('  - tool(name, description, parameters, callback)');
    console.log('  - ToolRegistry.register()');
    console.log('  - ToolRegistry.execute()');
    console.log('  - ToolRegistry.getTool()');
    console.log('  - ToolRegistry.hasTool()');
    console.log('  - ToolRegistry.getRegisteredTools()');

    console.log('\n✨ All Tool System methods verified successfully!');
    console.log('Note: Error handling in callbacks causes NAPI fatal errors - avoid throwing in tool functions.');
}

verifyToolsUsage().catch(console.error);
