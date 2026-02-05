import 'dotenv/config';
import { init, ToolRegistry } from 'graphbit';

async function simpleDebugTest() {
    console.log('Starting simple debug test...');

    try {
        init();
        console.log('✓ init() called');

        const registry = new ToolRegistry();
        console.log('✓ ToolRegistry created');

        registry.register('add', 'Add two numbers', {
            a: { type: 'number' },
            b: { type: 'number' }
        }, async (args: any) => {
            console.log('  Tool handler called with args:', args);
            const result = args.a + args.b;
            console.log('  Tool handler returning:', result);
            return result;
        });
        console.log('✓ Tool registered');

        const hasTool = registry.hasTool('add');
        console.log('✓ hasTool:', hasTool);

        console.log('\nExecuting tool...');
        const result = await registry.execute('add', { a: 5, b: 3 });
        console.log('\nRaw result from execute():');
        console.log(JSON.stringify(result, null, 2));
        console.log('\nResult type:', typeof result);
        console.log('Result keys:', Object.keys(result));

    } catch (error: any) {
        console.error('\n❌ Error:', error.message);
        console.error('Stack:', error.stack);
    }
}

simpleDebugTest();
