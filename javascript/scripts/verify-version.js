
const { version } = require('../index');

console.log(`Checking version...`);
const v = version();
console.log(`Exported Version: ${v}`);

if (v === '0.5.5') {
    console.log('✅ Version verification PASSED');
    process.exit(0);
} else {
    console.error(`❌ Version verification FAILED. Expected 0.5.5, got ${v}`);
    process.exit(1);
}
