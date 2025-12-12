/**
 * Integration Test Runner
 * 
 * Runs all integration tests and generates report
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('='.repeat(70));
console.log(' GraphBit JavaScript - Integration Test Runner');
console.log('='.repeat(70));
console.log();

// Check for API keys
const hasOpenAI = !!process.env.OPENAI_API_KEY;
const hasAnthropic = !!process.env.ANTHROPIC_API_KEY;

console.log('API Keys Status:');
console.log(`  OPENAI_API_KEY: ${hasOpenAI ? '✅ Set' : '❌ Not set'}`);
console.log(`  ANTHROPIC_API_KEY: ${hasAnthropic ? '✅ Set' : '❌ Not set (optional)'}`);
console.log();

if (!hasOpenAI) {
  console.log('⚠️  WARNING: OPENAI_API_KEY not set');
  console.log('   Integration tests will be skipped');
  console.log('   Set API key with: $env:OPENAI_API_KEY="your-key"');
  console.log();
  process.exit(1);
}

console.log('🔥 Running integration tests...\n');

const tests = [
  {
    name: 'LlmClient Integration',
    path: 'tests/integration/llm-client-integration.test.ts',
    priority: 'CRITICAL'
  },
  {
    name: 'End-to-End Workflows',
    path: 'tests/integration/end-to-end-workflow.test.ts',
    priority: 'CRITICAL'
  },
  {
    name: 'Performance Suite',
    path: 'benchmarks/performance-suite.ts',
    priority: 'HIGH'
  }
];

const results = [];

tests.forEach(test => {
  console.log(`Running: ${test.name} (${test.priority})`);
  console.log('-'.repeat(70));
  
  try {
    const output = execSync(`npm test ${test.path}`, {
      encoding: 'utf8',
      stdio: 'pipe'
    });
    
    results.push({
      test: test.name,
      status: 'PASS',
      output: output
    });
    
    console.log('✅ PASSED\n');
  } catch (error) {
    results.push({
      test: test.name,
      status: 'FAIL',
      error: error.message
    });
    
    console.log(`❌ FAILED: ${error.message}\n`);
  }
});

// Generate Report
console.log('='.repeat(70));
console.log(' Integration Test Summary');
console.log('='.repeat(70));
console.log();

const passed = results.filter(r => r.status === 'PASS').length;
const failed = results.filter(r => r.status === 'FAIL').length;

console.log(`Total Tests: ${results.length}`);
console.log(`Passed: ${passed}`);
console.log(`Failed: ${failed}`);
console.log();

if (failed === 0) {
  console.log('✅ ALL INTEGRATION TESTS PASSED!');
} else {
  console.log('❌ Some integration tests failed');
  console.log('   Review errors above for details');
}

// Save report
const report = {
  date: new Date().toISOString(),
  results,
  summary: {
    total: results.length,
    passed,
    failed
  }
};

const reportPath = path.join(__dirname, '../reports/integration-test-report.json');
fs.mkdirSync(path.dirname(reportPath), { recursive: true });
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));

console.log();
console.log(`📄 Report saved to: ${reportPath}`);

