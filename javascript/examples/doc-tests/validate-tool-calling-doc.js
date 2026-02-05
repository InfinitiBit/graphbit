/**
 * COMPREHENSIVE VALIDATION TEST
 * 
 * This test validates EVERY example from tool-calling-js.md against
 * the actual working ToolRegistry implementation.
 * 
 * Reference: test_phase4_tools.js (working test)
 */

const { init, ToolRegistry } = require('../../index');
const assert = require('assert');

async function main() {
    console.log('═'.repeat(70));
    console.log('COMPREHENSIVE VALIDATION: tool-calling-js.md Examples');
    console.log('═'.repeat(70));
    console.log();

    init();
    const registry = new ToolRegistry();

    let testNumber = 1;

    // ========== TEST 1: Quick Start - get_weather ==========
    console.log(`Test ${testNumber++}: Quick Start get_weather`);
    registry.register('get_weather', 'Get current weather information for any city', {
        location: { type: 'string' }
    }, (args) => {  // ✓ Sync callback
        return {
            location: args.location,
            temperature: 72,
            condition: 'sunny'
        };
    });

    let result = await registry.execute('get_weather', { location: 'Paris' });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result.location, 'Paris');
    assert.strictEqual(result.result.temperature, 72);
    console.log('✅ PASS\n');

    // ========== TEST 2: Quick Start - add ==========
    console.log(`Test ${testNumber++}: Quick Start add`);
    registry.register('add', 'Add two numbers together', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, (args) => {  // ✓ Sync callback
        return args.a + args.b;
    });

    result = await registry.execute('add', { a: 15, b: 27 });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result, 42);
    console.log('✅ PASS\n');

    // ========== TEST 3: Advanced Tool - search_data ==========
    console.log(`Test ${testNumber++}: Advanced search_data`);
    registry.register('search_data', 'Search and filter data with advanced options', {
        query: { type: 'string' },
        filters: { type: 'object' },
        sortBy: { type: 'string' },
        limit: { type: 'integer' }
    }, (args) => {  // ✓ Sync callback
        const { query, filters = {}, sortBy = 'relevance', limit = 10 } = args;

        const results = [
            { id: 1, title: 'Item 1', category: 'A', price: 10 },
            { id: 2, title: 'Item 2', category: 'B', price: 20 }
        ];

        return {
            query,
            results: results.slice(0, limit),
            total: results.length,
            sortBy
        };
    });

    result = await registry.execute('search_data', {
        query: 'test',
        limit: 1
    });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result.query, 'test');
    assert.strictEqual(result.result.results.length, 1);
    console.log('✅ PASS\n');

    // ========== TEST 4: API Integration (Promise return) ==========
    console.log(`Test ${testNumber++}: API Integration with Promise`);
    registry.register('fetch_api_data', 'Fetch data from API', {
        url: { type: 'string' }
    }, (args) => {  // ✓ Sync but returns Promise
        // Return Promise directly - this is the CORRECT way for async operations
        return Promise.resolve({
            url: args.url,
            data: 'simulated API response',
            status: 200
        });
    });

    result = await registry.execute('fetch_api_data', { url: 'https://api.example.com' });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result.status, 200);
    console.log('✅ PASS\n');

    // ========== TEST 5: Manual Execution - double_number ==========
    console.log(`Test ${testNumber++}: Manual execution double_number`);
    registry.register('double_number', 'Double a number', {
        value: { type: 'number' }
    }, (args) => args.value * 2);  // ✓ Sync callback, inline

    result = await registry.execute('double_number', { value: 5 });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result, 10);
    console.log('✅ PASS\n');

    // ========== TEST 6: List Tools ==========
    console.log(`Test ${testNumber++}: getRegisteredTools()`);
    const toolNames = registry.getRegisteredTools();
    assert(Array.isArray(toolNames));
    assert(toolNames.length >= 5);  // We registered at least 5 tools
    assert(toolNames.includes('get_weather'));
    assert(toolNames.includes('add'));
    console.log('✅ PASS:', toolNames.length, 'tools\n');

    // ========== TEST 7: hasTool ==========
    console.log(`Test ${testNumber++}: hasTool()`);
    assert.strictEqual(registry.hasTool('get_weather'), true);
    assert.strictEqual(registry.hasTool('nonexistent'), false);
    console.log('✅ PASS\n');

    // ========== TEST 8: getToolCount ==========
    console.log(`Test ${testNumber++}: getToolCount()`);
    const count = registry.getToolCount();
    assert(count >= 5);
    assert(count > 0);
    console.log('✅ PASS:', count, 'tools\n');

    // ========== TEST 9: unregisterTool ==========
    console.log(`Test ${testNumber++}: unregisterTool()`);
    registry.register('temp_tool', 'Temporary', {}, () => 'temp');
    assert.strictEqual(registry.hasTool('temp_tool'), true);
    const removed = registry.unregisterTool('temp_tool');
    assert.strictEqual(removed, true);
    assert.strictEqual(registry.hasTool('temp_tool'), false);
    console.log('✅ PASS\n');

    // ========== TEST 10: getToolMetadata ==========
    console.log(`Test ${testNumber++}: getToolMetadata()`);
    // Execute add a few times to generate metadata
    await registry.execute('add', { a: 1, b: 2 });
    await registry.execute('add', { a: 3, b: 4 });

    const metadata = registry.getToolMetadata('add');
    assert(metadata !== null);
    assert(metadata.callCount >= 3);  // At least 3 calls (1 earlier + 2 now)
    assert(typeof metadata.avgDurationMs === 'number');
    console.log('✅ PASS: callCount =', metadata.callCount, '\n');

    // ========== TEST 11: Error Handling ==========
    console.log(`Test ${testNumber++}: Error handling in tool`);
    registry.register('risky_operation', 'Operation that might fail', {
        value: { type: 'string' }
    }, (args) => {  // ✓ Sync callback with error handling
        try {
            if (!args.value) {
                throw new Error('Value required');
            }
            return { success: true, result: args.value.toUpperCase() };
        } catch (error) {
            return { success: false, error: error.message };
        }
    });

    result = await registry.execute('risky_operation', { value: 'test' });
    assert.strictEqual(result.success, true);
    assert.strictEqual(result.result.success, true); // Tool returned success

    result = await registry.execute('risky_operation', { value: '' });
    assert.strictEqual(result.success, true); // Execution succeeded
    assert.strictEqual(result.result.success, false); // But tool returned error
    console.log('✅ PASS\n');

    // ========== TEST 12: Complex object return ==========
    console.log(`Test ${testNumber++}: Complex workflow tools`);
    registry.register('fetch_data', 'Fetch data from API', {
        endpoint: { type: 'string' }
    }, (args) => {
        return { data: 'sample data from ' + args.endpoint };
    });

    registry.register('process_data', 'Process fetched data', {
        data: { type: 'string' }
    }, (args) => {
        return { processed: args.data.toUpperCase() };
    });

    registry.register('save_results', 'Save processed results', {
        result: { type: 'string' }
    }, (args) => {
        return { saved: true, result: args.result };
    });

    result = await registry.execute('fetch_data', { endpoint: '/users' });
    assert.strictEqual(result.success, true);
    assert(result.result.data.includes('/users'));
    console.log('✅ PASS\n');

    // ========== TEST 13: ToolResult structure validation ==========
    console.log(`Test ${testNumber++}: ToolResult structure`);
    result = await registry.execute('add', { a: 100, b: 200 });

    // Validate ToolResult from TypeScript definition
    assert(result.hasOwnProperty('success'), 'Missing success field');
    assert(result.hasOwnProperty('result'), 'Missing result field');
    assert(result.hasOwnProperty('executionTimeMs'), 'Missing executionTimeMs field');
    assert.strictEqual(typeof result.success, 'boolean');
    assert.strictEqual(typeof result.executionTimeMs, 'number');
    assert.strictEqual(result.result, 300);
    console.log('✅ PASS\n');

    // ========== TEST 14: clearAll ==========
    console.log(`Test ${testNumber++}: clearAll()`);
    const countBefore = registry.getToolCount();
    registry.clearAll();
    const countAfter = registry.getToolCount();
    assert.strictEqual(countAfter, 0);
    console.log('✅ PASS: Cleared', countBefore, 'tools\n');

    // ========== SUMMARY ==========
    console.log('═'.repeat(70));
    console.log(`✅ ALL ${testNumber - 1} TESTS PASSED!`);
    console.log('═'.repeat(70));
    console.log();
    console.log('VALIDATION COMPLETE:');
    console.log('- All core examples from tool-calling-js.md work correctly');
    console.log('- ToolRegistry API matches documentation');
    console.log('- ToolResult structure validated');
    console.log('- Error handling verified');
    console.log();
}

main().catch(err => {
    console.error('\n❌ TEST FAILED:', err.message);
    console.error(err.stack);
    process.exit(1);
});
