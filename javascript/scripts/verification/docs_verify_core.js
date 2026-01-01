const { init, version } = require('../../index');

async function verifyCoreUsage() {
    console.log('Verifying Core Functions for Documentation...');

    // 1. Test init()
    try {
        init();
        console.log('✅ init() executed successfully');
    } catch (error) {
        console.error('❌ init() failed:', error);
        process.exit(1);
    }

    // 2. Test version()
    try {
        const ver = version();
        console.log(`✅ version() returned: ${ver}`);

        // Validate it's a string
        if (typeof ver !== 'string') {
            throw new Error('version() should return a string');
        }
    } catch (error) {
        console.error('❌ version() failed:', error);
        process.exit(1);
    }

    console.log('\n✨ All core functions verified successfully!');
}

verifyCoreUsage().catch(console.error);
