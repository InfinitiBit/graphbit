import 'dotenv/config';
import { init, getSystemInfo, healthCheck, version, versionInfo } from '@infinitibit_gmbh/graphbit';

/**
 * Test 10: API Reference
 * Tests core API functions
 */

async function testApiReference() {
    console.log('=== Test 10: API Reference ===\n');

    try {
        // Initialize
        init();
        console.log('✅ init() works');

        // Version
        const ver = version();
        console.log(`✅ version(): ${ver}`);

        // Version info
        const verInfo = versionInfo();
        console.log(`✅ versionInfo(): ${verInfo}`);

        // System info
        const sysInfo = getSystemInfo();
        console.log(`✅ getSystemInfo():`);
        console.log(`   Node.js version: ${sysInfo.runtimeVersion}`);
        console.log(`   CPU count: ${sysInfo.cpuCount}`);
        console.log(`   Platform: ${sysInfo.platform}`);

        // Health check
        const health = healthCheck();
        console.log(`✅ healthCheck(): ${health.healthy ? 'Healthy' : 'Has issues'}`);

        console.log('\n✅ API Reference test passed!');
    } catch (error) {
        console.error('❌ API Reference test failed:', error);
        throw error;
    }
}

testApiReference();
