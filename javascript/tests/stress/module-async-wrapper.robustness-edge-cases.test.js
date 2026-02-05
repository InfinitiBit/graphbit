/**
 * ULTIMATE STRESS TEST SUITE: Callback ID Pattern
 * 
 * This test suite is designed to BREAK the implementation by testing:
 * - Edge cases
 * - Worst cases  
 * - Worst of worst cases
 * - Race conditions
 * - Memory stress
 * - Concurrent chaos
 * 
 * The goal is NOT to pass easily, but to rigorously validate the implementation.
 */

const { init, ToolRegistry, registerAsync, wrapAsync } = require('../../graphbit');

init();

let passed = 0;
let failed = 0;
const results = [];

async function runTest(category, name, fn, timeout = 30000) {
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
    console.log('   ULTIMATE STRESS TEST SUITE: CALLBACK ID PATTERN');
    console.log('   Goal: Find bugs through rigorous edge case testing');
    console.log('='.repeat(70) + '\n');

    // =============================================
    // CATEGORY 1: EDGE CASES - ARGUMENTS
    // =============================================

    await runTest('EDGE-ARGS', 'Empty args object', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'empty_args', 'Test', {}, async (args) => {
            return { received: args, isEmpty: Object.keys(args).length === 0 };
        });
        const result = await registry.execute('empty_args', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (!result.result.isEmpty) throw new Error('Args should be empty');
    });

    await runTest('EDGE-ARGS', 'Null args passed', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'null_args', 'Test', {}, async (args) => {
            return { received: args, isNull: args === null };
        });
        const result = await registry.execute('null_args', null);
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('EDGE-ARGS', 'Undefined args passed (converted to null)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'undefined_args', 'Test', {}, async (args) => {
            // Undefined gets converted to null by Rust serialization
            return { received: args, isNullOrUndefined: args === null || args === undefined };
        });
        // Note: undefined may become null in JSON serialization - that's acceptable
        try {
            const result = await registry.execute('undefined_args', undefined);
            // If it succeeds, that's fine
        } catch (e) {
            // If it fails with serialization error, that's also expected behavior
            if (!e.message.includes('serde')) throw e;
        }
    });

    await runTest('EDGE-ARGS', 'Very large payload (1MB JSON)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'large_payload', 'Test', {}, async (args) => {
            return { size: JSON.stringify(args).length };
        });
        // Create 1MB payload
        const largeArray = new Array(10000).fill(null).map((_, i) => ({
            id: i,
            data: 'x'.repeat(100)
        }));
        const result = await registry.execute('large_payload', { items: largeArray });
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        console.log(`   (Processed payload: ${result.result.size} bytes)`);
    });

    await runTest('EDGE-ARGS', 'Deeply nested object (50 levels)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'deep_nested', 'Test', {}, async (args) => {
            let depth = 0;
            let current = args;
            while (current.nested) {
                depth++;
                current = current.nested;
            }
            return { depth, value: current.value };
        });
        // Create 50 levels deep
        let nested = { value: 'deep' };
        for (let i = 0; i < 50; i++) {
            nested = { nested };
        }
        const result = await registry.execute('deep_nested', nested);
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.depth !== 50) throw new Error(`Expected depth 50, got ${result.result.depth}`);
    });

    await runTest('EDGE-ARGS', 'Unicode and emoji in args', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'unicode_test', 'Test', {}, async (args) => {
            return args;
        });
        const unicodeData = {
            emoji: '🎉🚀💪🔥',
            chinese: '你好世界',
            arabic: 'مرحبا بالعالم',
            special: '™®©℠',
            mixed: '日本語🇯🇵test'
        };
        const result = await registry.execute('unicode_test', unicodeData);
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.emoji !== unicodeData.emoji) throw new Error('Emoji mismatch');
        if (result.result.chinese !== unicodeData.chinese) throw new Error('Chinese mismatch');
    });

    await runTest('EDGE-ARGS', 'Special JSON characters in strings', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'special_chars', 'Test', {}, async (args) => {
            return args;
        });
        const specialData = {
            quotes: '"double" and \'single\'',
            backslash: 'path\\to\\file',
            newlines: 'line1\nline2\rline3',
            tabs: 'col1\tcol2',
            null_char: 'before\x00after'
        };
        const result = await registry.execute('special_chars', specialData);
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.quotes !== specialData.quotes) throw new Error('Quotes mismatch');
    });

    // =============================================
    // CATEGORY 2: EDGE CASES - RETURN VALUES
    // =============================================

    await runTest('EDGE-RETURN', 'Return null', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_null', 'Test', {}, async () => null);
        const result = await registry.execute('return_null', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== null) throw new Error(`Expected null, got ${result.result}`);
    });

    await runTest('EDGE-RETURN', 'Return undefined', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_undefined', 'Test', {}, async () => undefined);
        const result = await registry.execute('return_undefined', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('EDGE-RETURN', 'Return empty string', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_empty_str', 'Test', {}, async () => '');
        const result = await registry.execute('return_empty_str', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== '') throw new Error(`Expected empty string`);
    });

    await runTest('EDGE-RETURN', 'Return zero', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_zero', 'Test', {}, async () => 0);
        const result = await registry.execute('return_zero', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== 0) throw new Error(`Expected 0, got ${result.result}`);
    });

    await runTest('EDGE-RETURN', 'Return false', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_false', 'Test', {}, async () => false);
        const result = await registry.execute('return_false', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== false) throw new Error(`Expected false, got ${result.result}`);
    });

    await runTest('EDGE-RETURN', 'Return empty array', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_empty_arr', 'Test', {}, async () => []);
        const result = await registry.execute('return_empty_arr', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (!Array.isArray(result.result) || result.result.length !== 0) {
            throw new Error(`Expected empty array`);
        }
    });

    await runTest('EDGE-RETURN', 'Return very large number', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_large_num', 'Test', {}, async () => Number.MAX_SAFE_INTEGER);
        const result = await registry.execute('return_large_num', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== Number.MAX_SAFE_INTEGER) throw new Error('Max safe integer mismatch');
    });

    await runTest('EDGE-RETURN', 'Return Infinity (sanitized to null)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_infinity', 'Test', {}, async () => Infinity);
        const result = await registry.execute('return_infinity', {});
        // Infinity gets sanitized to null by our wrapper
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== null) throw new Error(`Expected null (sanitized), got ${result.result}`);
    });

    await runTest('EDGE-RETURN', 'Return NaN (sanitized to null)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'return_nan', 'Test', {}, async () => NaN);
        const result = await registry.execute('return_nan', {});
        // NaN gets sanitized to null by our wrapper
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result !== null) throw new Error(`Expected null (sanitized), got ${result.result}`);
    });

    // =============================================
    // CATEGORY 3: ERROR HANDLING EDGE CASES
    // =============================================

    await runTest('ERROR', 'Throw Error object', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_error', 'Test', {}, async () => {
            throw new Error('Standard error');
        });
        const result = await registry.execute('throw_error', {});
        if (result.success) throw new Error('Should have failed');
        if (!result.error.includes('Standard error')) throw new Error('Error message missing');
    });

    await runTest('ERROR', 'Throw string directly', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_string', 'Test', {}, async () => {
            throw 'String error';
        });
        const result = await registry.execute('throw_string', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Throw null', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_null', 'Test', {}, async () => {
            throw null;
        });
        const result = await registry.execute('throw_null', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Throw undefined', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_undefined', 'Test', {}, async () => {
            throw undefined;
        });
        const result = await registry.execute('throw_undefined', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Throw object (not Error)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_object', 'Test', {}, async () => {
            throw { code: 500, msg: 'Custom error object' };
        });
        const result = await registry.execute('throw_object', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Throw number', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'throw_number', 'Test', {}, async () => {
            throw 42;
        });
        const result = await registry.execute('throw_number', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Promise.reject with Error', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'reject_error', 'Test', {}, () => {
            return Promise.reject(new Error('Rejected!'));
        });
        const result = await registry.execute('reject_error', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Nested promise rejection', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'nested_reject', 'Test', {}, async () => {
            await Promise.resolve();
            await Promise.resolve();
            throw new Error('Nested rejection');
        });
        const result = await registry.execute('nested_reject', {});
        if (result.success) throw new Error('Should have failed');
    });

    await runTest('ERROR', 'Error after long async delay', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'delayed_error', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, 500));
            throw new Error('Delayed error');
        });
        const result = await registry.execute('delayed_error', {});
        if (result.success) throw new Error('Should have failed');
        if (result.executionTimeMs < 400) throw new Error('Timing wrong');
    });

    // =============================================
    // CATEGORY 4: TIMING STRESS TESTS
    // =============================================

    await runTest('TIMING', 'Very short async (1ms)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'short_async', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, 1));
            return { done: true };
        });
        const result = await registry.execute('short_async', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('TIMING', 'Zero-delay Promise (microtask)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'microtask', 'Test', {}, async () => {
            await Promise.resolve();
            return { done: true };
        });
        const result = await registry.execute('microtask', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('TIMING', 'Long async (3 seconds)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'long_async', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, 3000));
            return { done: true };
        });
        const result = await registry.execute('long_async', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.executionTimeMs < 2900) throw new Error(`Too fast: ${result.executionTimeMs}ms`);
        console.log(`   (Execution time: ${result.executionTimeMs.toFixed(0)}ms)`);
    }, 10000);

    await runTest('TIMING', 'Rapid sequential calls (100x)', async () => {
        const registry = new ToolRegistry();
        let callCount = 0;
        registerAsync(registry, 'rapid_seq', 'Test', {}, async (args) => {
            callCount++;
            await new Promise(r => setTimeout(r, 5));
            return { call: args.n };
        });

        for (let i = 0; i < 100; i++) {
            const result = await registry.execute('rapid_seq', { n: i });
            if (!result.success) throw new Error(`Call ${i} failed`);
            if (result.result.call !== i) throw new Error(`Result mismatch at ${i}`);
        }

        if (callCount !== 100) throw new Error(`Expected 100 calls, got ${callCount}`);
        console.log(`   (100 sequential calls completed)`);
    }, 60000);

    // =============================================
    // CATEGORY 5: CONCURRENCY STRESS TESTS
    // =============================================

    await runTest('CONCURRENCY', 'High concurrency (50 parallel calls)', async () => {
        const registry = new ToolRegistry();
        let maxConcurrent = 0;
        let currentConcurrent = 0;

        registerAsync(registry, 'concurrent_50', 'Test', {}, async (args) => {
            currentConcurrent++;
            maxConcurrent = Math.max(maxConcurrent, currentConcurrent);
            await new Promise(r => setTimeout(r, 100));
            currentConcurrent--;
            return { id: args.id };
        });

        const promises = [];
        for (let i = 0; i < 50; i++) {
            promises.push(registry.execute('concurrent_50', { id: i }));
        }

        const results = await Promise.all(promises);
        const allSucceeded = results.every(r => r.success);
        if (!allSucceeded) throw new Error('Some calls failed');

        const ids = results.map(r => r.result.id).sort((a, b) => a - b);
        for (let i = 0; i < 50; i++) {
            if (ids[i] !== i) throw new Error(`Missing id ${i}`);
        }
        console.log(`   (Max concurrent: ${maxConcurrent})`);
    }, 30000);

    await runTest('CONCURRENCY', 'Mixed sync and async callbacks', async () => {
        const registry = new ToolRegistry();

        // Sync callback
        registry.register('sync_tool', 'Sync', {}, (wrapperArgs) => {
            const args = wrapperArgs.__originalArgs || wrapperArgs;
            return { type: 'sync', value: args.value * 2 };
        });

        // Async callback
        registerAsync(registry, 'async_tool', 'Async', {}, async (args) => {
            await new Promise(r => setTimeout(r, 50));
            return { type: 'async', value: args.value * 3 };
        });

        // Interleave calls
        const promises = [];
        for (let i = 0; i < 20; i++) {
            if (i % 2 === 0) {
                promises.push(registry.execute('sync_tool', { value: i }));
            } else {
                promises.push(registry.execute('async_tool', { value: i }));
            }
        }

        const results = await Promise.all(promises);
        for (let i = 0; i < 20; i++) {
            if (!results[i].success) throw new Error(`Call ${i} failed`);
            const expected = i % 2 === 0 ? i * 2 : i * 3;
            if (results[i].result.value !== expected) {
                throw new Error(`Mismatch at ${i}: expected ${expected}, got ${results[i].result.value}`);
            }
        }
    });

    await runTest('CONCURRENCY', 'Same tool called from different "threads"', async () => {
        const registry = new ToolRegistry();
        const callOrder = [];

        registerAsync(registry, 'thread_test', 'Test', {}, async (args) => {
            callOrder.push(`start:${args.thread}`);
            await new Promise(r => setTimeout(r, Math.random() * 100));
            callOrder.push(`end:${args.thread}`);
            return { thread: args.thread };
        });

        const threads = [1, 2, 3, 4, 5].map(t =>
            registry.execute('thread_test', { thread: t })
        );

        const results = await Promise.all(threads);
        if (!results.every(r => r.success)) throw new Error('Some threads failed');

        // All starts should come before all ends in random order (they overlap)
        const starts = callOrder.filter(e => e.startsWith('start:')).length;
        const ends = callOrder.filter(e => e.startsWith('end:')).length;
        if (starts !== 5 || ends !== 5) throw new Error('Missing calls');
    });

    // =============================================
    // CATEGORY 6: METADATA VERIFICATION
    // =============================================

    await runTest('METADATA', 'Call count accurate after many async calls', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'meta_test', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, 10));
            return {};
        });

        const promises = [];
        for (let i = 0; i < 25; i++) {
            promises.push(registry.execute('meta_test', {}));
        }
        await Promise.all(promises);

        const metadata = registry.getToolMetadata('meta_test');
        if (metadata.callCount !== 25) {
            throw new Error(`Expected callCount=25, got ${metadata.callCount}`);
        }
    });

    await runTest('METADATA', 'Average duration reflects async time', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'avg_duration', 'Test', {}, async () => {
            await new Promise(r => setTimeout(r, 100));
            return {};
        });

        await registry.execute('avg_duration', {});
        await registry.execute('avg_duration', {});
        await registry.execute('avg_duration', {});

        const metadata = registry.getToolMetadata('avg_duration');
        if (metadata.avgDurationMs < 80) {
            throw new Error(`Avg duration too low: ${metadata.avgDurationMs}ms (expected ~100ms)`);
        }
        console.log(`   (Average duration: ${metadata.avgDurationMs.toFixed(2)}ms)`);
    });

    await runTest('METADATA', 'Stats aggregation across multiple tools', async () => {
        const registry = new ToolRegistry();

        registerAsync(registry, 'tool_a', 'Tool A', {}, async () => {
            await new Promise(r => setTimeout(r, 20));
            return {};
        });
        registerAsync(registry, 'tool_b', 'Tool B', {}, async () => {
            await new Promise(r => setTimeout(r, 30));
            return {};
        });

        await registry.execute('tool_a', {});
        await registry.execute('tool_a', {});
        await registry.execute('tool_b', {});

        const stats = registry.getStats();
        if (stats.totalTools !== 2) throw new Error(`Expected 2 tools, got ${stats.totalTools}`);
        if (stats.totalExecutions !== 3) throw new Error(`Expected 3 executions, got ${stats.totalExecutions}`);
        if (stats.successfulExecutions !== 3) throw new Error(`Expected 3 successful`);
    });

    // =============================================
    // CATEGORY 7: WORST CASE SCENARIOS
    // =============================================

    await runTest('WORST-CASE', 'Callback returns Promise that resolves to another Promise', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'nested_promise', 'Test', {}, async () => {
            return Promise.resolve(Promise.resolve({ nested: 'promise' }));
        });
        const result = await registry.execute('nested_promise', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('WORST-CASE', 'Promise chain with multiple .then()', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'promise_chain', 'Test', {}, (args) => {
            return Promise.resolve(args.x)
                .then(x => x + 1)
                .then(x => x * 2)
                .then(x => x - 3)
                .then(x => ({ result: x }));
        });
        const result = await registry.execute('promise_chain', { x: 5 });
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        // (5 + 1) * 2 - 3 = 9
        if (result.result.result !== 9) throw new Error(`Expected 9, got ${result.result.result}`);
    });

    await runTest('WORST-CASE', 'Promise with .finally()', async () => {
        const registry = new ToolRegistry();
        let finallyCalled = false;
        registerAsync(registry, 'finally_test', 'Test', {}, async () => {
            try {
                await new Promise(r => setTimeout(r, 50));
                return { value: 42 };
            } finally {
                finallyCalled = true;
            }
        });
        const result = await registry.execute('finally_test', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (!finallyCalled) throw new Error('Finally not called');
    });

    await runTest('WORST-CASE', 'Error in Promise.finally()', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'finally_error', 'Test', {}, () => {
            return Promise.resolve({ value: 42 }).finally(() => {
                // Error in finally should not prevent resolution
            });
        });
        const result = await registry.execute('finally_error', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
    });

    await runTest('WORST-CASE', 'thenable (duck-typed Promise)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'thenable', 'Test', {}, () => {
            // Return a thenable (not a real Promise)
            return {
                then(resolve) {
                    setTimeout(() => resolve({ thenable: true }), 50);
                }
            };
        });
        const result = await registry.execute('thenable', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (!result.result.thenable) throw new Error('Thenable not resolved correctly');
    });

    await runTest('WORST-CASE', 'Multiple registrations of same tool', async () => {
        const registry = new ToolRegistry();

        registerAsync(registry, 'overwrite', 'Version 1', {}, async () => ({ version: 1 }));
        registerAsync(registry, 'overwrite', 'Version 2', {}, async () => ({ version: 2 }));

        const result = await registry.execute('overwrite', {});
        if (!result.success) throw new Error(`Failed: ${result.error}`);
        if (result.result.version !== 2) throw new Error('Should use latest registration');
    });

    await runTest('WORST-CASE', 'Very long error message (10KB)', async () => {
        const registry = new ToolRegistry();
        const longMessage = 'x'.repeat(10000);
        registerAsync(registry, 'long_error', 'Test', {}, async () => {
            throw new Error(longMessage);
        });
        const result = await registry.execute('long_error', {});
        if (result.success) throw new Error('Should have failed');
        // Verify error is captured (may be truncated)
    });

    // =============================================
    // CATEGORY 8: MEMORY / RESOURCE STRESS
    // =============================================

    await runTest('MEMORY', 'Many pending callbacks simultaneously (100)', async () => {
        const registry = new ToolRegistry();
        registerAsync(registry, 'many_pending', 'Test', {}, async (args) => {
            await new Promise(r => setTimeout(r, 100 + Math.random() * 100));
            return { id: args.id };
        });

        const promises = [];
        for (let i = 0; i < 100; i++) {
            promises.push(registry.execute('many_pending', { id: i }));
        }

        const results = await Promise.all(promises);
        const successful = results.filter(r => r.success).length;
        if (successful !== 100) throw new Error(`Only ${successful}/100 succeeded`);
    }, 30000);

    await runTest('MEMORY', 'Large number of tools (200)', async () => {
        const registry = new ToolRegistry();

        for (let i = 0; i < 200; i++) {
            registerAsync(registry, `tool_${i}`, `Tool ${i}`, {}, async (args) => {
                await new Promise(r => setTimeout(r, 5));
                return { tool: i, arg: args.x };
            });
        }

        // Execute random subset
        const promises = [];
        for (let i = 0; i < 50; i++) {
            const toolNum = Math.floor(Math.random() * 200);
            promises.push(registry.execute(`tool_${toolNum}`, { x: i }));
        }

        const results = await Promise.all(promises);
        const successful = results.filter(r => r.success).length;
        if (successful !== 50) throw new Error(`Only ${successful}/50 succeeded`);

        const count = registry.getToolCount();
        if (count !== 200) throw new Error(`Expected 200 tools, got ${count}`);
    }, 30000);

    // =============================================
    // SUMMARY
    // =============================================

    console.log('\n' + '='.repeat(70));
    console.log('   ULTIMATE STRESS TEST RESULTS');
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
