import 'dotenv/config';
import { init, ToolRegistry } from 'graphbit';

/**
 * COMPREHENSIVE TEST SUITE: ToolRegistry API Validation
 * 
 * Validates ALL examples from docs/user-guide/tool-calling-js.md against
 * the actual GraphBit JavaScript bindings implementation.
 * 
 * Coverage:
 * - 14+ tool registration patterns from documentation
 * - 9 ToolRegistry methods (hasTool, getRegisteredTools, execute, etc.)
 * - ToolResult structure validation
 * - Error handling patterns
 * - All examples from Quick Start through Best Practices sections
 */

interface ToolResult {
    success: boolean;
    result: any;
    executionTimeMs: number;
}

async function test01_BasicToolRegistration() {
    console.log('\n=== Test 01: Basic Tool Registration (add) ===');

    try {
        init();
        const registry = new ToolRegistry();

        // From docs line 91-96: Basic registration pattern
        registry.register('add', 'Add two numbers together', {
            a: { type: 'number' },
            b: { type: 'number' }
        }, async (args: any) => args.a + args.b);

        console.log('✅ Tool registered with function-based API');

        // Validate registration
        if (!registry.hasTool('add')) {
            throw new Error('Tool should be registered');
        }
        console.log('✅ hasTool() confirms registration');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test02_ToolExecutionWithToolResult() {
    console.log('\n=== Test 02: Tool Execution with ToolResult Handling ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('multiply', 'Multiply two numbers', {
            x: { type: 'number' },
            y: { type: 'number' }
        }, async (args: any) => args.x * args.y);

        // Execute and validate ToolResult structure
        const result = await registry.execute('multiply', { x: 5, y: 3 }) as ToolResult;

        // Validate ToolResult structure from smoke test discovery
        if (!result.hasOwnProperty('success')) {
            throw new Error('ToolResult missing success property');
        }
        if (!result.hasOwnProperty('result')) {
            throw new Error('ToolResult missing result property');
        }
        if (!result.hasOwnProperty('executionTimeMs')) {
            throw new Error('ToolResult missing executionTimeMs property');
        }

        console.log('✅ ToolResult structure validated');

        if (result.success !== true) {
            throw new Error('Tool execution should succeed');
        }
        if (result.result !== 15) {
            throw new Error(`Expected result 15, got ${result.result}`);
        }

        console.log('✅ Tool execution returned correct result:', result.result);
        console.log(`✅ Execution time: ${result.executionTimeMs}ms`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test03_AdvancedToolComplexParameters() {
    console.log('\n=== Test 03: Advanced Tool (search_data) ===');

    try {
        init();
        const registry = new ToolRegistry();

        // From docs line 101-120: Advanced tool with 4 parameters
        registry.register('search_data', 'Search and filter data with advanced options', {
            query: { type: 'string' },
            filters: { type: 'object' },
            sortBy: { type: 'string' },
            limit: { type: 'integer' }
        }, async (args: any) => {
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

        console.log('✅ Advanced tool registered with 4 parameters');

        const result = await registry.execute('search_data', {
            query: 'test',
            limit: 1
        }) as ToolResult;

        if (!result.success) {
            throw new Error('Search should succeed');
        }
        if (!result.result.query || !result.result.results) {
            throw new Error('Search result should have query and results');
        }

        console.log('✅ Advanced tool executed correctly');
        console.log(`  Query: ${result.result.query}`);
        console.log(`  Results: ${result.result.results.length} items`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test04_GetRegisteredTools() {
    console.log('\n=== Test 04: getRegisteredTools() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('tool1', 'First tool', {}, async () => 'result1');
        registry.register('tool2', 'Second tool', {}, async () => 'result2');
        registry.register('tool3', 'Third tool', {}, async () => 'result3');

        const tools = registry.getRegisteredTools();

        if (!Array.isArray(tools)) {
            throw new Error('getRegisteredTools() should return array');
        }
        if (tools.length !== 3) {
            throw new Error(`Expected 3 tools, got ${tools.length}`);
        }
        if (!tools.includes('tool1') || !tools.includes('tool2') || !tools.includes('tool3')) {
            throw new Error('Missing expected tools');
        }

        console.log('✅ getRegisteredTools() returned correct array:', tools);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test05_GetToolCount() {
    console.log('\n=== Test 05: getToolCount() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        const initialCount = registry.getToolCount();
        if (initialCount !== 0) {
            throw new Error(`Expected 0 tools initially, got ${initialCount}`);
        }
        console.log('✅ Initial count: 0');

        registry.register('test1', 'Test', {}, async () => 1);
        registry.register('test2', 'Test', {}, async () => 2);

        const count = registry.getToolCount();
        if (count !== 2) {
            throw new Error(`Expected 2 tools, got ${count}`);
        }

        console.log('✅ getToolCount() returned correct count:', count);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test06_UnregisterTool() {
    console.log('\n=== Test 06: unregisterTool() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('temp_tool', 'Temporary tool', {}, async () => 'temp');

        if (!registry.hasTool('temp_tool')) {
            throw new Error('Tool should be registered');
        }
        console.log('✅ Tool registered');

        const removed = registry.unregisterTool('temp_tool');
        if (!removed) {
            throw new Error('unregisterTool() should return true');
        }
        console.log('✅ unregisterTool() returned true');

        if (registry.hasTool('temp_tool')) {
            throw new Error('Tool should be unregistered');
        }
        console.log('✅ Tool successfully removed');

        // Test removing non-existent tool
        const notRemoved = registry.unregisterTool('nonexistent');
        if (notRemoved) {
            throw new Error('Removing non-existent tool should return false');
        }
        console.log('✅ Correctly returns false for non-existent tool');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test07_GetToolMetadata() {
    console.log('\n=== Test 07: getToolMetadata() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('test_meta', 'Test metadata tracking', {
            value: { type: 'number' }
        }, async (args: any) => args.value * 2);

        // Execute tool multiple times to generate metadata
        await registry.execute('test_meta', { value: 1 });
        await registry.execute('test_meta', { value: 2 });
        await registry.execute('test_meta', { value: 3 });

        const metadata = registry.getToolMetadata('test_meta');

        if (!metadata) {
            throw new Error('getToolMetadata() should return metadata');
        }
        if (!metadata.hasOwnProperty('callCount')) {
            throw new Error('Metadata should have callCount');
        }
        if (metadata.callCount !== 3) {
            throw new Error(`Expected 3 calls, got ${metadata.callCount}`);
        }

        console.log('✅ getToolMetadata() returned valid metadata');
        console.log(`  Call count: ${metadata.callCount}`);
        console.log(`  Has avgDurationMs: ${metadata.hasOwnProperty('avgDurationMs')}`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test08_GetStats() {
    console.log('\n=== Test 08: getStats() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('stat_test1', 'Test 1', {}, async () => 'a');
        registry.register('stat_test2', 'Test 2', {}, async () => 'b');

        await registry.execute('stat_test1', {});
        await registry.execute('stat_test2', {});
        await registry.execute('stat_test1', {});

        const stats = registry.getStats();

        if (!stats) {
            throw new Error('getStats() should return stats');
        }
        if (!stats.hasOwnProperty('totalTools')) {
            throw new Error('Stats should have totalTools');
        }
        if (!stats.hasOwnProperty('totalExecutions')) {
            throw new Error('Stats should have totalExecutions');
        }

        console.log('✅ getStats() returned valid statistics');
        console.log(`  Total tools: ${stats.totalTools}`);
        console.log(`  Total executions: ${stats.totalExecutions}`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test09_ErrorHandlingTool() {
    console.log('\n=== Test 09: Error Handling in Tools (risky_operation) ===');

    try {
        init();
        const registry = new ToolRegistry();

        // From docs line 229-240: Error handling pattern
        registry.register('risky_operation', 'An operation that might fail', {
            value: { type: 'string' }
        }, async (args: any) => {
            try {
                if (!args.value) {
                    throw new Error('Value required');
                }
                return { success: true, result: args.value.toUpperCase() };
            } catch (error: any) {
                return { success: false, error: error.message };
            }
        });

        console.log('✅ Error handling tool registered');

        // Test success case
        const successResult = await registry.execute('risky_operation', { value: 'test' }) as ToolResult;
        if (!successResult.success) {
            throw new Error('Tool execution wrapper should succeed');
        }
        if (successResult.result.success !== true) {
            throw new Error('Tool should succeed with valid input');
        }
        console.log('✅ Tool handled success case correctly');

        // Test error case (tool handles error internally)
        const errorResult = await registry.execute('risky_operation', { value: '' }) as ToolResult;
        if (!errorResult.success) {
            throw new Error('Tool execution wrapper should succeed even when tool returns error');
        }
        if (errorResult.result.success !== false) {
            throw new Error('Tool should return error object for invalid input');
        }
        console.log('✅ Tool handled error case gracefully');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test10_ClearAll() {
    console.log('\n=== Test 10: clearAll() Method ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('temp1', 'Temp 1', {}, async () => 1);
        registry.register('temp2', 'Temp 2', {}, async () => 2);
        registry.register('temp3', 'Temp 3', {}, async () => 3);

        let count = registry.getToolCount();
        if (count !== 3) {
            throw new Error(`Expected 3 tools before clear, got ${count}`);
        }
        console.log('✅ Registered 3 tools');

        registry.clearAll();
        console.log('✅ clearAll() executed');

        count = registry.getToolCount();
        if (count !== 0) {
            throw new Error(`Expected 0 tools after clear, got ${count}`);
        }
        console.log('✅ All tools cleared successfully');

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test11_QuickStartExample() {
    console.log('\n=== Test 11: Quick Start Example (get_weather) ===');

    try {
        init();
        const registry = new ToolRegistry();

        // From docs line 36-44: Quick start get_weather tool
        registry.register('get_weather', 'Get current weather information for any city', {
            location: { type: 'string' }
        }, async (args: any) => {
            return {
                location: args.location,
                temperature: 72,
                condition: 'sunny'
            };
        });

        console.log('✅ get_weather tool registered (Quick Start pattern)');

        const result = await registry.execute('get_weather', { location: 'Paris' }) as ToolResult;

        if (!result.success) {
            throw new Error('Weather tool should succeed');
        }
        if (result.result.location !== 'Paris') {
            throw new Error('Result should include location');
        }

        console.log('✅ Quick Start example validated');
        console.log(`  Location: ${result.result.location}`);
        console.log(`  Temperature: ${result.result.temperature}°F`);
        console.log(`  Condition: ${result.result.condition}`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function test12_NoParameterTool() {
    console.log('\n=== Test 12: Tool with No Parameters ===');

    try {
        init();
        const registry = new ToolRegistry();

        registry.register('current_time', 'Get current timestamp', {}, async () => {
            return Date.now();
        });

        console.log('✅ No-parameter tool registered');

        const result = await registry.execute('current_time', {}) as ToolResult;

        if (!result.success) {
            throw new Error('Tool should succeed');
        }
        if (typeof result.result !== 'number') {
            throw new Error('Should return number timestamp');
        }

        console.log('✅ No-parameter tool executed successfully');
        console.log(`  Timestamp: ${result.result}`);

        return true;
    } catch (error) {
        console.error('❌ Test failed:', error);
        return false;
    }
}

async function main() {
    console.log('═'.repeat(70));
    console.log('  COMPREHENSIVE TEST SUITE: ToolRegistry API Validation');
    console.log('  Validating ALL patterns from tool-calling-js.md');
    console.log('═'.repeat(70));

    const results: boolean[] = [];

    // Run all tests sequentially to avoid interference
    results.push(await test01_BasicToolRegistration());
    results.push(await test02_ToolExecutionWithToolResult());
    results.push(await test03_AdvancedToolComplexParameters());
    results.push(await test04_GetRegisteredTools());
    results.push(await test05_GetToolCount());
    results.push(await test06_UnregisterTool());
    results.push(await test07_GetToolMetadata());
    results.push(await test08_GetStats());
    results.push(await test09_ErrorHandlingTool());
    results.push(await test10_ClearAll());
    results.push(await test11_QuickStartExample());
    results.push(await test12_NoParameterTool());

    const allPassed = results.every(r => r);
    const passedCount = results.filter(r => r).length;
    const totalTests = results.length;

    console.log('\n' + '═'.repeat(70));
    if (allPassed) {
        console.log(`  ✅ ALL ${totalTests} TESTS PASSED`);
        console.log('  Documentation patterns validated against JS bindings!');
    } else {
        console.log(`  ❌ ${totalTests - passedCount}/${totalTests} TESTS FAILED`);
        console.log('  Check errors above for details');
    }
    console.log('═'.repeat(70) + '\n');

    process.exit(allPassed ? 0 : 1);
}

main();
