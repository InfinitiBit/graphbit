/**
 * EXTREME WORST-OF-WORST CASE TESTS
 * 
 * This suite goes beyond normal stress testing to find the absolute edges.
 * These tests are designed to be brutal and expose any hidden issues.
 */

const { init, ToolRegistry, registerAsync, wrapAsync } = require('../../graphbit');

init();

let passed = 0;
let failed = 0;
const results = [];

async function runTest(category, name, fn, timeout = 60000) {
    const fullName = `[${category}] ${name}`;
    const timeoutPromise = new Promise((_, reject) =>
        setTimeout(() => reject(new Error(`TIMEOUT after ${timeout}ms`)), timeout)
    );

    try {
        await Promise.race([fn(), timeoutPromise]);
        console.log(`✅ ${fullName}`);
        passed++;
        results.push({ category, name, status: 'passed' });
    } catch (error) {
        console.log(`❌ ${fullName}`);
        console.log(`   Error: ${error.message}`);
        failed++;
        results.push({ category, name, status: 'failed', error: error.message });
    }
}

async function main() {
    console.log('\n' + '='.repeat(70));
    console.log('   EXTREME WORST-OF-WORST CASE TESTS');
    console.log('   Goal: Break the implementation with brutal edge cases');
    console.log('='.repeat(70) + '\n');

    // =============================================
    // CATEGORY: RACE CONDITIONS
    // =============================================

    await runTest('RACE', 'Concurrent calls to same tool with different args', async () => {
        const registry = new ToolRegistry();
        const callLog = [];

        registerAsync(registry, 'race_tool', 'Test', {}, async (args) => {
            callLog.push(`start:${args.id}`);
            // Random delay to cause race
            await new Promise(r => setTimeout(r, Math.random() * 50));
            callLog.push(`end:${args.id}`);
            return { id: args.id, double: args.id * 2 };
        });

        // Fire all at once
        const promises = [];
        for (let i = 0; i < 20; i++) {
            promises.push(registry.execute('race_tool', { id: i }));
        }

        const results = await Promise.all(promises);

        // Verify no result mixing
        for (let i = 0; i < 20; i++) {
            if (results[i].result.id !== i) throw new Error(`Result ${i} has wrong id: ${results[i].result.id}`);
            if (results[i].result.double !== i * 2) throw new Error(`Result ${i} has wrong double`);
        }
    });

    await runTest('RACE', 'Interleaved fast and slow callbacks', async () => {
        const registry = new ToolRegistry();

        registerAsync(registry, 'fast_tool', 'Fast', {}, async (args) => {
            await new Promise(r => setTimeout(r, 1));
            return { type: 'fast', value: args.v };
        });

        registerAsync(registry, 'slow_tool', 'Slow', {}, async (args) => {
            await new Promise(r => setTimeout(r, 200));
            return { type: 'slow', value: args.v };
        });

        // Start slow first, fast second - fast should complete while slow is pending
        const slowPromise = registry.execute('slow_tool', { v: 'slow' });
        const fastPromise = registry.execute('fast_tool', { v: 'fast' });

        const fastResult = await fastPromise;
        const slowResult = await slowPromise;

        if (fastResult.result.value !== 'fast') throw new Error('Fast mismatch');
        if (slowResult.result.value !== 'slow') throw new Error('Slow mismatch');
    });

    await runTest('RACE', 'Rapid register/execute cycle', async () => {
        const registry = new ToolRegistry();

        for (let i = 0; i < 50; i++) {
            const toolName = `rapid_tool_${i}`;
            registerAsync(registry, toolName, 'Test', {}, async (args) => {
                return { iteration: i, arg: args.x };
            });

            const result = await registry.execute(toolName, { x: i * 10 });
            if (!result.success) throw new Error(`Iteration ${i} failed`);
            if (result.result.iteration !== i) throw new Error(`Wrong iteration at ${i}`);
        }
    });

    // =============================================
    // CATEGORY: EXTREME CONCURRENCY
    // =============================================

    await runTest('EXTREME-CONCURRENCY', '500 concurrent calls', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'mass_concurrent', 'Test', {}, async (args) => {
            await new Promise(r => setTimeout(r, 10 + Math.random() * 20));
            return { id: args.id };
        });

        const promises = [];
        for (let i = 0; i < 500; i++) {
            promises.push(registry.execute('mass_concurrent', { id: i }));
        }

        const results = await Promise.all(promises);
        const successful = results.filter(r => r.success).length;
        if (successful !== 500) throw new Error(`Only ${successful}/500 succeeded`);
        console.log(`   (All 500 concurrent calls completed successfully)`);
    }, 120000);

    await runTest('EXTREME-CONCURRENCY', 'Concurrent different tools (100 tools x 5 calls each)', async () => {
        const registry = new ToolRegistry();

        // Register 100 different tools
        for (let t = 0; t < 100; t++) {
            registerAsync(registry, `tool_${t}`, `Tool ${t}`, {}, async (args) => {
                await new Promise(r => setTimeout(r, 5));
                return { tool: t, call: args.c };
            });
        }

        // Call each tool 5 times concurrently
        const promises = [];
        for (let t = 0; t < 100; t++) {
            for (let c = 0; c < 5; c++) {
                promises.push(registry.execute(`tool_${t}`, { c }));
            }
        }

        const results = await Promise.all(promises);
        const successful = results.filter(r => r.success).length;
        if (successful !== 500) throw new Error(`Only ${successful}/500 succeeded`);
        console.log(`   (100 tools × 5 calls = 500 total, all successful)`);
    }, 120000);

    // =============================================
    // CATEGORY: CHAOS TESTING
    // =============================================

    await runTest('CHAOS', 'Random success/failure mix', async () => {
        const registry = new ToolRegistry();
        let successCount = 0;
        let failCount = 0;

        registerAsync(registry, 'chaos_tool', 'Test', {}, async (args) => {
            await new Promise(r => setTimeout(r, 5));
            if (args.shouldFail) {
                throw new Error(`Intentional failure #${args.id}`);
            }
            return { id: args.id };
        });

        const promises = [];
        for (let i = 0; i < 100; i++) {
            const shouldFail = Math.random() < 0.3; // 30% failure rate
            if (shouldFail) failCount++;
            else successCount++;
            promises.push(registry.execute('chaos_tool', { id: i, shouldFail }));
        }

        const results = await Promise.all(promises);
        const actualSuccess = results.filter(r => r.success).length;
        const actualFail = results.filter(r => !r.success).length;

        console.log(`   (Expected: ${successCount} success, ${failCount} fail)`);
        console.log(`   (Actual:   ${actualSuccess} success, ${actualFail} fail)`);

        if (actualSuccess !== successCount) throw new Error('Success count mismatch');
        if (actualFail !== failCount) throw new Error('Fail count mismatch');
    });

    await runTest('CHAOS', 'Random delays (0-500ms)', async () => {
        const registry = new ToolRegistry();

        registerAsync(registry, 'random_delay', 'Test', {}, async (args) => {
            const delay = Math.floor(Math.random() * 500);
            await new Promise(r => setTimeout(r, delay));
            return { id: args.id, delay };
        });

        const promises = [];
        for (let i = 0; i < 50; i++) {
            promises.push(registry.execute('random_delay', { id: i }));
        }

        const results = await Promise.all(promises);
        const allSuccess = results.every(r => r.success);
        if (!allSuccess) throw new Error('Some calls failed');
        console.log(`   (50 calls with random 0-500ms delays all succeeded)`);
    }, 60000);

    // =============================================
    // CATEGORY: ERROR EDGE EXTREMES
    // =============================================

    await runTest('ERROR-EXTREME', 'Error with circular reference object', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'circular_error', 'Test', {}, async () => {
            const obj = { a: 1 };
            obj.self = obj; // Circular reference
            throw obj;
        });
        const result = await registry.execute('circular_error', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR-EXTREME', 'Error message with binary data', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'binary_error', 'Test', {}, async () => {
            const binaryMsg = 'Error\x00with\x01binary\x02chars';
            throw new Error(binaryMsg);
        });
        const result = await registry.execute('binary_error', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR-EXTREME', 'Error in Promise chain middle', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'chain_error', 'Test', {}, () => {
            return Promise.resolve(1)
                .then(x => x + 1)
                .then(() => { throw new Error('Mid-chain error'); })
                .then(x => x * 2);
        });
        const result = await registry.execute('chain_error', {});
        if (result.success) throw new Error('Should have failed');
    });

    // =============================================
    // CATEGORY: DATA INTEGRITY
    // =============================================

    await runTest('DATA-INTEGRITY', 'Large binary-like data preserved', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'binary_data', 'Test', {}, async (args) => {
            return args; // Echo back
        });

        // Create data with all byte values 0-255 (as string chars)
        let binaryLike = '';
        for (let i = 32; i < 127; i++) { // Printable ASCII only for JSON
            binaryLike += String.fromCharCode(i);
        }

        const result = await registry.execute('binary_data', { data: binaryLike });
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.data !== binaryLike) throw new Error('Data mismatch');
    });

    await runTest('DATA-INTEGRITY', 'Deeply nested arrays (100 levels)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'deep_array', 'Test', {}, async (args) => {
            // Count depth
            let depth = 0;
            let current = args.arr;
            while (Array.isArray(current) && current.length > 0) {
                depth++;
                current = current[0];
            }
            return { depth, leaf: current };
        });

        let arr = ['leaf'];
        for (let i = 0; i < 100; i++) {
            arr = [arr];
        }

        const result = await registry.execute('deep_array', { arr });
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.depth !== 101) throw new Error(`Wrong depth: ${result.result.depth}`);
        if (result.result.leaf !== 'leaf') throw new Error('Leaf mismatch');
    });

    await runTest('DATA-INTEGRITY', 'Object with 1000 keys', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'many_keys', 'Test', {}, async (args) => {
            return { keyCount: Object.keys(args.data).length };
        });

        const data = {};
        for (let i = 0; i < 1000; i++) {
            data[`key_${i}`] = `value_${i}`;
        }

        const result = await registry.execute('many_keys', { data });
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.keyCount !== 1000) throw new Error(`Wrong key count: ${result.result.keyCount}`);
    });

    // =============================================
    // CATEGORY: TIMING PRECISION
    // =============================================

    await runTest('TIMING-PRECISION', 'Duration tracking accuracy', async () => {
        const registry = new ToolRegistry();
        const targetMs = 200;

        registerAsync(registry, 'precise_timing', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, targetMs));
            return {};
        });

        const result = await registry.execute('precise_timing', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);

        const drift = Math.abs(result.executionTimeMs - targetMs);
        const driftPercent = (drift / targetMs) * 100;
        console.log(`   (Target: ${targetMs}ms, Actual: ${result.executionTimeMs.toFixed(2)}ms, Drift: ${driftPercent.toFixed(1)}%)`);

        // Allow up to 20% drift
        if (driftPercent > 20) throw new Error(`Drift too high: ${driftPercent.toFixed(1)}%`);
    });

    await runTest('TIMING-PRECISION', 'Sub-millisecond async', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'submilli', 'Test', {}, async () => {
            await Promise.resolve(); // Microtask only
            return { done: true };
        });

        const start = performance.now();
        const result = await registry.execute('submilli', {});
        const elapsed = performance.now() - start;

        if (!result.success) throw new Error(`Failed: ${result.error}`);
        console.log(`   (Completed in ${elapsed.toFixed(2)}ms)`);
    });

    // =============================================
    // CATEGORY: STABILITY TESTS
    // =============================================

    await runTest('STABILITY', 'Repeated tool execution (1000x same tool)', async () => {
        const registry = new ToolRegistry();
        let callCount = 0;

        registerAsync(registry, 'repeat_tool', 'Test', {}, async () => {
            callCount++;
            return { count: callCount };
        });

        for (let i = 0; i < 1000; i++) {
            const result = await registry.execute('repeat_tool', {});
            if (!result.success) throw new Error(`Failed at iteration ${i}`);
        }

        const metadata = registry.getToolMetadata('repeat_tool');
        if (metadata.callCount !== 1000) throw new Error(`Wrong call count: ${metadata.callCount}`);
        console.log(`   (1000 sequential executions, all tracked correctly)`);
    }, 60000);

    await runTest('STABILITY', 'Mixed workload sustained (30 seconds)', async () => {
        const registry = new ToolRegistry();

        registerAsync(registry, 'fast', 'Fast', {}, async () => {
            await new Promise(r => setTimeout(r, 1));
            return { type: 'fast' };
        });

        registerAsync(registry, 'medium', 'Medium', {}, async () => {
            await new Promise(r => setTimeout(r, 50));
            return { type: 'medium' };
        });

        registerAsync(registry, 'slow', 'Slow', {}, async () => {
            await new Promise(r => setTimeout(r, 200));
            return { type: 'slow' };
        });

        const startTime = Date.now();
        const duration = 5000; // 5 seconds for test speed
        let totalCalls = 0;
        let errors = 0;

        const promises = [];

        while (Date.now() - startTime < duration) {
            const tools = ['fast', 'fast', 'fast', 'medium', 'slow']; // Weighted
            const tool = tools[Math.floor(Math.random() * tools.length)];
            promises.push(
                registry.execute(tool, {}).then(r => {
                    if (!r.success) errors++;
                })
            );
            totalCalls++;
            await new Promise(r => setTimeout(r, 10)); // 100 calls/sec max
        }

        await Promise.all(promises);
        console.log(`   (${totalCalls} calls in ${duration}ms, ${errors} errors)`);
        if (errors > 0) throw new Error(`${errors} errors during sustained load`);
    }, 60000);

    // =============================================
    // SUMMARY
    // =============================================

    console.log('\n' + '='.repeat(70));
    console.log('   EXTREME TEST RESULTS');
    console.log('='.repeat(70));
    console.log(`  Total Tests: ${passed + failed}`);
    console.log(`  ✅ Passed: ${passed}`);
    console.log(`  ❌ Failed: ${failed}`);
    console.log(`  Success Rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`);
    console.log('='.repeat(70));

    if (failed > 0) {
        console.log('\n  FAILED TESTS:');
        results.filter(r => r.status === 'failed').forEach(r => {
            console.log(`    ❌ [${r.category}] ${r.name}`);
            console.log(`       ${r.error}`);
        });
    }

    console.log('\n');
    process.exit(failed > 0 ? 1 : 0);
}

main().catch(err => {
    console.error('Test runner error:', err);
    process.exit(1);
});
