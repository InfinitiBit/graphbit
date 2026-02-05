import 'dotenv/config';
import { init, version } from '../../index.js';

/**
 * Test 01: Installation Verification
 * Tests basic installation and version check
 */

async function testInstallation() {
    console.log('=== Test 01: Installation Verification ===\n');

    try {
        // Initialize GraphBit
        init();
        console.log('✅ GraphBit initialized successfully');

        // Check version
        const ver = version();
        console.log(`✅ GraphBit version: ${ver}`);

        console.log('\n✅ All installation tests passed!');
    } catch (error) {
        console.error('❌ Installation test failed:', error);
        throw error;
    }
}

testInstallation();
