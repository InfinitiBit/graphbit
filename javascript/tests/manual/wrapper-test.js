
const { ToolRegistry, registerAsync, init } = require('../../graphbit.js');

async function testWrapper() {
    console.log('Testing graphbit.js wrapper...');

    // Initialize (mocked if needed, or real)
    try {
        init();
    } catch (e) {
        console.log('Init skipped or failed (expected if no native bindings):', e.message);
    }

    const registry = new ToolRegistry();
    console.log('ToolRegistry created.');

    // Register async tool
    console.log('Registering async tool...');
    registerAsync(registry, 'test_tool', 'Test Tool', {
        type: 'object',
        properties: { val: { type: 'string' } }
    }, async (args) => {
        console.log('Async callback executed with:', args);
        return { success: true, val: args.val };
    });

    // Verify tool is registered
    const tools = await registry.getRegisteredTools();
    console.log('Registered tools:', tools);

    if (tools.includes('test_tool')) {
        console.log('✅ SUCCESS: Tool registered via wrapper!');
    } else {
        console.error('❌ FAILURE: Tool not found.');
        process.exit(1);
    }
}

testWrapper().catch(err => {
    console.error('❌ FATAL ERROR:', err);
    process.exit(1);
});
