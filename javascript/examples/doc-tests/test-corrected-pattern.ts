import 'dotenv/config';
import { init, ToolRegistry } from 'graphbit';

async function testCorrectedDocPattern() {
    console.log('Testing CORRECTED doc pattern (sync callbacks)...\n');

    try {
        init();
        const registry = new ToolRegistry();

        // Test 1: Simple tool (from docs pattern)
        registry.register('add', 'Add two numbers', {
            a: { type: 'number' },
            b: { type: 'number' }
        }, (args) => {  // ✅ NO async - as per corrected docs
            return args.a + args.b;
        });
        console.log('✅ Test 1: Tool registered with sync callback');

        const result1 = await registry.execute('add', { a: 5, b: 3 });
        console.log('Result:', JSON.stringify(result1, null, 2));

        if (result1.result !== 8) {
            throw new Error(`Expected result 8, got ${result1.result}`);
        }
        console.log('✅ Test 1 PASSED: Result is correct!\n');

        // Test 2: Complex object return (weather pattern)
        registry.register('get_weather', 'Get weather', {
            location: { type: 'string' }
        }, (args) => {  // ✅ NO async
            return {
                location: args.location,
                temperature: 72,
                condition: 'sunny'
            };
        });
        console.log('✅ Test 2: Weather tool registered');

        const result2 = await registry.execute('get_weather', { location: 'Paris' });
        console.log('Result:', JSON.stringify(result2, null, 2));

        if (!result2.result.location || !result2.result.temperature) {
            throw new Error('Weather result missing properties');
        }
        console.log('✅ Test 2 PASSED: Complex object returned correctly!\n');

        // Test 3: Tool that needs async operations (using Promise return)
        registry.register('delayed_calc', 'Calculation with delay', {
            value: { type: 'number' }
        }, (args) => {  // ✅ NO async, but returns Promise
            return new Promise(resolve => {
                setTimeout(() => {
                    resolve(args.value * 2);
                }, 10);
            });
        });
        console.log('✅ Test 3: Async operation tool registered (returns Promise)');

        const result3 = await registry.execute('delayed_calc', { value: 10 });
        console.log('Result:', JSON.stringify(result3, null, 2));

        if (result3.result !== 20) {
            throw new Error(`Expected result 20, got ${result3.result}`);
        }
        console.log('✅ Test 3 PASSED: Promise-returning tool works!\n');

        console.log('═'.repeat(70));
        console.log('✅ ALL TESTS PASSED - Documentation pattern is now CORRECT!');
        console.log('═'.repeat(70));

    } catch (error) {
        console.error('\n❌ Test failed:', error);
        process.exit(1);
    }
}

testCorrectedDocPattern();
