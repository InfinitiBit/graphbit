import 'dotenv/config';
import { init, createToolRegistry } from '@infinitibit_gmbh/graphbit';

/**
 * Test 06: Tool Calling
 * Tests tool registry and tool definitions
 */

async function testToolCalling() {
    console.log('=== Test 06: Tool Calling ===\n');

    try {
        init();

        // Create tool registry
        const registry = createToolRegistry();
        console.log('✅ Created tool registry');

        // Register a simple tool
        await registry.register(
            'calculator',
            JSON.stringify({
                type: 'object',
                properties: {
                    operation: { type: 'string', enum: ['add', 'subtract', 'multiply', 'divide'] },
                    a: { type: 'number' },
                    b: { type: 'number' }
                },
                required: ['operation', 'a', 'b']
            })
        );
        console.log('✅ Registered calculator tool');

        console.log('\n✅ Tool Calling test passed!');
    } catch (error) {
        console.error('❌ Tool Calling test failed:', error);
        throw error;
    }
}

testToolCalling();
