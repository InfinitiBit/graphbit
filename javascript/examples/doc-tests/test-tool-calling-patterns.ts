import 'dotenv/config';
import { init, ToolRegistry } from 'graphbit';

/**
 * TEST: Tool Calling Patterns from tool-calling-js.md
 */

async function testBasicToolRegistration() {
    console.log('\n=== Test 1: Basic Tool Registration ===');

    try {
        init();
        const registry = new ToolRegistry();
        console.log('✅ ToolRegistry instantiated');

        registry.register('add', 'Add two numbers together', {
            a: { type: 'number' },
            b: { type: 'number' }
        }, async (args: any) => args.a + args.b);

        console.log('✅ Tool registered successfully');

        const isRegistered = registry.hasTool('add');
        if (!isRegistered) throw new Error('Tool should be registered');
        console.log('✅ registry.hasTool() confirmed tool exists');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testManualToolExecution() {
    console.log('\n=== Test 2: Manual Tool Execution ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('double_number', 'Double a number', {
            value: { type: 'number' }
        }, async (args: any) => args.value * 2);

        console.log('✅ Tool registered');

        const result = await registry.execute('double_number', { value: 5 });
        if (result !== 10) throw new Error(`Expected 10, got ${result}`);

        console.log('✅ registry.execute() returned correct result:', result);
        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testToolManagement() {
    console.log('\n=== Test 3: Tool Management Operations ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('tool1', 'First tool', {}, async () => 'result1');
        registry.register('tool2', 'Second tool', {}, async () => 'result2');
        console.log('✅ Registered 2 tools');

        const toolNames = registry.getRegisteredTools();
        if (!toolNames.includes('tool1') || !toolNames.includes('tool2')) {
            throw new Error('getRegisteredTools() should return registered tools');
        }
        console.log('✅ registry.getRegisteredTools() returned:', toolNames);

        const hasTools = registry.getToolCount() > 0;
        if (!hasTools) throw new Error('getToolCount() > 0 should return true');
        console.log('✅ registry.getToolCount() returned', registry.getToolCount());

        const removed = registry.unregisterTool('tool1');
        console.log('✅ registry.unregisterTool() removed tool:', removed);

        const stillRegistered = registry.hasTool('tool1');
        if (stillRegistered) throw new Error('Tool should be unregistered');
        console.log('✅ Confirmed tool1 was unregistered');

        registry.clearAll();
        console.log('✅ registry.clearAll() executed');

        const hasToolsAfterClear = registry.getToolCount() > 0;
        if (hasToolsAfterClear) throw new Error('Should have no tools after clearAll()');
        console.log('✅ Confirmed all tools cleared');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testAdvancedToolRegistration() {
    console.log('\n=== Test 4: Advanced Tool with Complex Parameters ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('search_data', 'Search and filter data with advanced options', {
            query: { type: 'string' },
            filters: { type: 'object' },
            sortBy: { type: 'string' },
            limit: { type: 'integer' }
        }, async (args: any) => {
            const { query, sortBy = 'relevance', limit = 10 } = args;
            const results = [
                { id: 1, title: 'Item 1', category: 'A', price: 10 },
                { id: 2, title: 'Item 2', category: 'B', price: 20 }
            ];
            return { query, results: results.slice(0, limit), total: results.length, sortBy };
        });

        console.log('✅ Registered advanced tool with complex schema');

        const result = await registry.execute('search_data', { query: 'test', limit: 1 }) as any;
        if (!result.query || !result.results) {
            throw new Error('Advanced tool should return structured data');
        }
        console.log('✅ Advanced tool executed correctly');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function testToolWithErrorHandling() {
    console.log('\n=== Test 5: Tool with Error Handling ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('risky_operation', 'An operation that might fail', {
            value: { type: 'string' }
        }, async (args: any) => {
            try {
                if (!args.value) throw new Error('Value required');
                return { success: true, result: args.value.toUpperCase() };
            } catch (error: any) {
                return { success: false, error: error.message };
            }
        });

        console.log('✅ Registered tool with error handling');

        const successResult = await registry.execute('risky_operation', { value: 'test' }) as any;
        if (!successResult.success) throw new Error('Should succeed with valid input');
        console.log('✅ Tool handled success case');

        const errorResult = await registry.execute('risky_operation', { value: '' }) as any;
        if (errorResult.success !== false) throw new Error('Should fail with invalid input');
        console.log('✅ Tool handled error case gracefully');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  TEST SUITE: Tool Calling Patterns (from tool-calling-js.md)');
    console.log('═'.repeat(70));
    console.log('\nValidating tool calling patterns from fixed documentation...\n');

    const results: boolean[] = [];
    results.push(await testBasicToolRegistration());
    results.push(await testManualToolExecution());
    results.push(await testToolManagement());
    results.push(await testAdvancedToolRegistration());
    results.push(await testToolWithErrorHandling());

    const allPassed = results.every(r => r);

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log('  ✅ ALL TESTS PASSED - Tool calling patterns are CORRECT');
    } else {
        console.log('  ❌ SOME TESTS FAILED - Check errors above');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
