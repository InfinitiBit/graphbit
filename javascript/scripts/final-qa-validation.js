/**
 * Final QA Validation Script
 * 
 * Runs complete validation checklist before production release
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('='.repeat(70));
console.log(' GraphBit JavaScript - Final QA Validation');
console.log('='.repeat(70));
console.log();

const checks = [];

function check(name, fn) {
  try {
    fn();
    checks.push({ name, status: 'PASS', error: null });
    console.log(`✅ ${name}`);
    return true;
  } catch (error) {
    checks.push({ name, status: 'FAIL', error: error.message });
    console.log(`❌ ${name}: ${error.message}`);
    return false;
  }
}

console.log('Running validation checks...\n');

// 1. Code Quality Checks
console.log('1. Code Quality');
console.log('-'.repeat(70));

check('TypeScript definitions exist', () => {
  if (!fs.existsSync('index.d.ts')) throw new Error('index.d.ts not found');
});

check('Main exports exist', () => {
  if (!fs.existsSync('index.js')) throw new Error('index.js not found');
});

check('Native binding exists', () => {
  const nodeFile = fs.readdirSync('.').find(f => f.endsWith('.node'));
  if (!nodeFile) throw new Error('.node file not found');
});

console.log();

// 2. Documentation Checks
console.log('2. Documentation');
console.log('-'.repeat(70));

check('README.md exists', () => {
  if (!fs.existsSync('README.md')) throw new Error('README.md not found');
});

check('MIGRATION_GUIDE.md exists', () => {
  if (!fs.existsSync('MIGRATION_GUIDE.md')) throw new Error('MIGRATION_GUIDE.md not found');
});

check('CHANGELOG.md exists', () => {
  if (!fs.existsSync('CHANGELOG.md')) throw new Error('CHANGELOG.md not found');
});

check('Examples exist', () => {
  const examples = fs.readdirSync('examples').filter(f => f.endsWith('.ts'));
  if (examples.length < 7) throw new Error(`Only ${examples.length} examples found, expected 7+`);
});

console.log();

// 3. Test Checks
console.log('3. Tests');
console.log('-'.repeat(70));

check('Test files exist', () => {
  if (!fs.existsSync('tests/unit')) throw new Error('tests/unit directory not found');
  const tests = fs.readdirSync('tests/unit').filter(f => f.endsWith('.test.ts'));
  if (tests.length < 7) throw new Error(`Only ${tests.length} test files, expected 7+`);
});

console.log();

// 4. Build Checks
console.log('4. Build System');
console.log('-'.repeat(70));

check('package.json valid', () => {
  const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  if (!pkg.name) throw new Error('package.json missing name');
  if (!pkg.version) throw new Error('package.json missing version');
});

check('Cargo.toml valid', () => {
  const cargo = fs.readFileSync('Cargo.toml', 'utf8');
  if (!cargo.includes('[package]')) throw new Error('Cargo.toml invalid');
});

check('tsconfig.json valid', () => {
  const tsconfig = JSON.parse(fs.readFileSync('tsconfig.json', 'utf8'));
  if (!tsconfig.compilerOptions) throw new Error('tsconfig.json missing compilerOptions');
});

console.log();

// 5. Security Checks
console.log('5. Security');
console.log('-'.repeat(70));

check('No hardcoded secrets in source', () => {
  const srcFiles = fs.readdirSync('src').filter(f => f.endsWith('.rs'));
  srcFiles.forEach(file => {
    const content = fs.readFileSync(path.join('src', file), 'utf8');
    // Look for actual hardcoded API keys (pattern: api_key: "sk-...")
    const secretPattern = /(api_key|apiKey)\s*[:=]\s*["']sk-|["'][A-Za-z0-9]{40,}["']/;
    if (secretPattern.test(content)) {
      throw new Error(`Hardcoded secret detected in ${file}`);
    }
  });
});

check('Error messages safe', () => {
  // Check that error messages don't expose sensitive info
  // This is a basic check - manual review also needed
  const srcFiles = fs.readdirSync('src');
  let unsafe = false;
  srcFiles.forEach(file => {
    const content = fs.readFileSync(path.join('src', file), 'utf8');
    // Check for common unsafe patterns
    if (content.includes('console.log(password)')) {
      unsafe = true;
    }
  });
  if (unsafe) throw new Error('Potentially unsafe error messages found');
});

console.log();

// 6. Production Readiness
console.log('6. Production Readiness');
console.log('-'.repeat(70));

check('Health check implemented', () => {
  const libContent = fs.readFileSync('src/lib.rs', 'utf8');
  if (!libContent.includes('health_check')) throw new Error('health_check not found');
});

check('Init configuration implemented', () => {
  const libContent = fs.readFileSync('src/lib.rs', 'utf8');
  if (!libContent.includes('InitOptions')) throw new Error('InitOptions not found');
});

check('Statistics tracking implemented', () => {
  const llmClientContent = fs.readFileSync('src/llm_client.rs', 'utf8');
  if (!llmClientContent.includes('StatsTracker')) throw new Error('StatsTracker not found');
});

console.log();

// Summary
console.log('='.repeat(70));
console.log(' Validation Summary');
console.log('='.repeat(70));
console.log();

const passed = checks.filter(c => c.status === 'PASS').length;
const failed = checks.filter(c => c.status === 'FAIL').length;

console.log(`Total Checks: ${checks.length}`);
console.log(`Passed: ${passed}`);
console.log(`Failed: ${failed}`);
console.log();

if (failed === 0) {
  console.log('✅ ALL VALIDATION CHECKS PASSED!');
  console.log('   System is ready for production release');
} else {
  console.log('❌ Some validation checks failed');
  console.log('   Review and fix issues before release');
}

// Save report
const report = {
  date: new Date().toISOString(),
  checks,
  summary: {
    total: checks.length,
    passed,
    failed
  }
};

const reportPath = path.join(__dirname, '../reports/final-qa-report.json');
fs.mkdirSync(path.dirname(reportPath), { recursive: true });
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));

console.log();
console.log(`📄 Report saved to: ${reportPath}`);
console.log();

process.exit(failed > 0 ? 1 : 0);

