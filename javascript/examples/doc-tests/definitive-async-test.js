/**
 * DEFINITIVE TEST SUITE
 * Testing all possible ToolRegistry callback patterns
 */

const { init, ToolRegistry } = require('../../index');

async function runDefinitiveTests() {
    console.log('═'.repeat(70));
    console.log('DEFINITIVE ASYNC SUPPORT INVESTIGATION');
    console.log('═'.repeat(70));
    console.log();

    init();

    // ========== TEST 1: SYNC CALLBACK (BASELINE) ==========
    console.log('TEST 1: Sync callback (baseline)');
    const r1 = new ToolRegistry();
    r1.register('sync_test', 'Sync test', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, (args) => {
        return args.a + args.b;
    });

    const result1 = await r1.execute('sync_test', { a: 5, b: 3 });
    console.log('Result:', JSON.stringify(result1, null, 2));
    console.log('Verdict:', result1.result === 8 ? '✅ WORKS' : '❌ FAILS');
    console.log();

    // ========== TEST 2: ASYNC CALLBACK ==========
    console.log('TEST 2: Async callback');
    const r2 = new ToolRegistry();
    r2.register('async_test', 'Async test', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, async (args) => {
        return args.a + args.b;
    });

    const result2 = await r2.execute('async_test', { a: 5, b: 3 });
    console.log('Result:', JSON.stringify(result2, null, 2));
    console.log('Verdict:', result2.result === 8 ? '✅ WORKS' : '❌ FAILS - returns Promise object');
    console.log();

    // ========== TEST 3: PROMISE.RESOLVE ==========
    console.log('TEST 3: Promise.resolve');
    const r3 = new ToolRegistry();
    r3.register('promise_test', 'Promise test', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, (args) => {
        return Promise.resolve(args.a + args.b);
    });

    const result3 = await r3.execute('promise_test', { a: 5, b: 3 });
    console.log('Result:', JSON.stringify(result3, null, 2));
    console.log('Verdict:', result3.result === 8 ? '✅ WORKS' : '❌ FAILS - Promise not awaited');
    console.log();

    // ========== TEST 4: ASYNC IIFE ==========
    console.log('TEST 4: Async IIFE');
    const r4 = new ToolRegistry();
    r4.register('iife_test', 'IIFE test', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, (args) => {
        return (async () => {
            return args.a + args.b;
        })();
    });

    const result4 = await r4.execute('iife_test', { a: 5, b: 3 });
    console.log('Result:', JSON.stringify(result4, null, 2));
    console.log('Verdict:', result4.result === 8 ? '✅ WORKS' : '❌ FAILS - Returns Promise');
    console.log();

    // ========== TEST 5: PROMISE WITH THEN ==========
    console.log('TEST 5: Promise.then chain');
    const r5 = new ToolRegistry();
    r5.register('then_test', 'Then test', {
        a: { type: 'number' },
        b: { type: 'number' }
    }, (args) => {
        return Promise.resolve(args.a).then(a => a + args.b);
    });

    const result5 = await r5.execute('then_test', { a: 5, b: 3 });
    console.log('Result:', JSON.stringify(result5, null, 2));
    console.log('Verdict:', result5.result === 8 ? '✅ WORKS' : '❌ FAILS - Promise not awaited');
    console.log();

    // ========== TEST 6: NESTED ASYNC OPERATIONS ==========
    console.log('TEST 6: Nested async (setTimeout)');
    const r6 = new ToolRegistry();
    r6.register('timeout_test', 'Timeout test', {
        value: { type: 'number' }
    }, (args) => {
        return new Promise(resolve => {
            setTimeout(() => {
                resolve(args.value * 2);
            }, 10);
        });
    });

    const result6 = await r6.execute('timeout_test', { value: 5 });
    console.log('Result:', JSON.stringify(result6, null, 2));
    console.log('Verdict:', result6.result === 10 ? '✅ WORKS' : '❌ FAILS - Promise not awaited');
    console.log();

    // ========== SUMMARY ==========
    console.log('═'.repeat(70));
    console.log('SUMMARY:');
    console.log('═'.repeat(70));

    const results = [
        { name: 'Sync callback', works: result1.result === 8 },
        { name: 'Async callback', works: result2.result === 8 },
        { name: 'Promise.resolve', works: result3.result === 8 },
        { name: 'Async IIFE', works: result4.result === 8 },
        { name: 'Promise.then', works: result5.result === 8 },
        { name: 'Nested Promise', works: result6.result === 10 }
    ];

    results.forEach((r, i) => {
        const status = r.works ? '✅ WORKS' : '❌ BROKEN';
        console.log(`${i + 1}. ${r.name}: ${status}`);
    });

    const workingCount = results.filter(r => r.works).length;
    console.log();
    console.log(`VERDICT: ${workingCount}/${results.length} patterns work`);
    console.log();
}

runDefinitiveTests().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
