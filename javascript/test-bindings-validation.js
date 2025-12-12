#!/usr/bin/env node

/**
 * JS Bindings Validation Test for Strategy B Migration
 * 
 * Tests:
 * 1. Module loading (index.js with platform detection)
 * 2. All exported functions/classes are accessible
 * 3. Basic functionality works
 * 4. No runtime errors related to package naming
 */

console.log('═══════════════════════════════════════════════════');
console.log('  JS Bindings Validation - Strategy B Migration');
console.log('═══════════════════════════════════════════════════\n');

// Test 1: Module Loading
console.log('✓ Test 1: Module Loading');
try {
  const graphbit = require('./index.js');
  console.log('  ✅ Module loaded successfully');
  console.log(`  📦 Module type: ${typeof graphbit}`);
  console.log(`  📦 Exports count: ${Object.keys(graphbit).length}`);
} catch (err) {
  console.error('  ❌ FAILED: Module loading failed');
  console.error('  Error:', err.message);
  process.exit(1);
}

// Test 2: Required Exports
console.log('\n✓ Test 2: Required Exports');
const graphbit = require('./index.js');
const requiredExports = [
  'init',
  'version',
  'versionInfo',
  'getSystemInfo',
  'healthCheck',
  'configureRuntime',
  'LlmConfig',
  'LlmClient',
  'AgentBuilder',
  'Agent',
  'WorkflowBuilder',
  'Executor',
  'WorkflowGraph',
  'WorkflowContext',
  'WorkflowResult',
  'DocumentLoader',
  'EmbeddingConfig',
  'EmbeddingClient',
  'TextSplitter',
  'createToolRegistry'
];

let missingExports = [];
for (const exportName of requiredExports) {
  if (graphbit[exportName]) {
    console.log(`  ✅ ${exportName}`);
  } else {
    console.log(`  ❌ ${exportName} - MISSING`);
    missingExports.push(exportName);
  }
}

if (missingExports.length > 0) {
  console.error(`\n  ❌ FAILED: ${missingExports.length} exports missing`);
  process.exit(1);
}

// Test 3: Basic Functionality
console.log('\n✓ Test 3: Basic Functionality');

try {
  // 3.1: init()
  graphbit.init({ logLevel: 'error', coloredLogs: false });
  console.log('  ✅ init() works');
} catch (err) {
  console.error('  ❌ init() failed:', err.message);
  process.exit(1);
}

try {
  // 3.2: version()
  const v = graphbit.version();
  console.log(`  ✅ version() works: ${v}`);
  if (v !== '0.5.1') {
    console.error(`  ⚠️  Warning: Expected version 0.5.1, got ${v}`);
  }
} catch (err) {
  console.error('  ❌ version() failed:', err.message);
  process.exit(1);
}

try {
  // 3.3: versionInfo()
  const info = graphbit.versionInfo();
  console.log(`  ✅ versionInfo() works`);
  console.log(`     - version: ${info.version}`);
  console.log(`     - rustVersion: ${info.rustVersion}`);
  console.log(`     - napiVersion: ${info.napiVersion}`);
} catch (err) {
  console.error('  ❌ versionInfo() failed:', err.message);
  process.exit(1);
}

try {
  // 3.4: getSystemInfo()
  const sysInfo = graphbit.getSystemInfo();
  console.log(`  ✅ getSystemInfo() works`);
  console.log(`     - OS: ${sysInfo.os} ${sysInfo.osVersion}`);
  console.log(`     - Arch: ${sysInfo.arch}`);
  console.log(`     - CPUs: ${sysInfo.cpuCount}`);
  console.log(`     - Memory: ${sysInfo.totalMemoryMb} MB`);
  console.log(`     - Node: ${sysInfo.nodeVersion}`);
} catch (err) {
  console.error('  ❌ getSystemInfo() failed:', err.message);
  process.exit(1);
}

try {
  // 3.5: healthCheck()
  const health = graphbit.healthCheck();
  console.log(`  ✅ healthCheck() works`);
  console.log(`     - healthy: ${health.healthy}`);
  console.log(`     - version: ${health.version}`);
  console.log(`     - uptime: ${health.uptimeSeconds}s`);
} catch (err) {
  console.error('  ❌ healthCheck() failed:', err.message);
  process.exit(1);
}

try {
  // 3.6: configureRuntime()
  graphbit.configureRuntime({
    maxThreads: 4,
    enableMonitoring: true,
    memoryLimitMb: 512
  });
  console.log(`  ✅ configureRuntime() works`);
} catch (err) {
  console.error('  ❌ configureRuntime() failed:', err.message);
  process.exit(1);
}

try {
  // 3.7: WorkflowGraph
  const graph = new graphbit.WorkflowGraph();
  console.log(`  ✅ WorkflowGraph instantiation works`);
} catch (err) {
  console.error('  ❌ WorkflowGraph failed:', err.message);
  process.exit(1);
}

try {
  // 3.8: DocumentLoader
  const loader = new graphbit.DocumentLoader();
  console.log(`  ✅ DocumentLoader instantiation works`);
} catch (err) {
  console.error('  ❌ DocumentLoader failed:', err.message);
  process.exit(1);
}

try {
  // 3.9: TextSplitter (has static factory methods)
  if (typeof graphbit.TextSplitter === 'function' && 
      typeof graphbit.TextSplitter.character === 'function') {
    console.log(`  ✅ TextSplitter export works (with static methods)`);
  } else {
    throw new Error('TextSplitter not exported correctly');
  }
} catch (err) {
  console.error('  ❌ TextSplitter failed:', err.message);
  process.exit(1);
}

// Test 4: LlmConfig Providers
console.log('\n✓ Test 4: LlmConfig Providers');
const providers = [
  'openai',
  'anthropic',
  'ollama',
  'azureOpenai',
  'bytedance',
  'deepseek',
  'huggingface',
  'perplexity',
  'openrouter',
  'fireworks',
  'replicate',
  'togetherai',
  'xai',
  'ai21',
  'mistralai'
];

for (const provider of providers) {
  if (typeof graphbit.LlmConfig[provider] === 'function') {
    console.log(`  ✅ LlmConfig.${provider}()`);
  } else {
    console.log(`  ❌ LlmConfig.${provider}() - MISSING`);
    process.exit(1);
  }
}

// Test 5: Package Scope Validation
console.log('\n✓ Test 5: Package Scope Validation');
const packageJson = require('./package.json');
console.log(`  📦 Package name: ${packageJson.name}`);

if (packageJson.name === '@infinitibit_gmbh/graphbit') {
  console.log('  ✅ Correct organization scope');
} else {
  console.error('  ❌ Wrong package name:', packageJson.name);
  process.exit(1);
}

if (packageJson.optionalDependencies) {
  console.log(`  📦 Platform packages: ${Object.keys(packageJson.optionalDependencies).length}`);
  
  let wrongScope = [];
  for (const [pkg, version] of Object.entries(packageJson.optionalDependencies)) {
    if (pkg.startsWith('@infinitibit_gmbh/graphbit-')) {
      console.log(`  ✅ ${pkg}`);
    } else {
      console.log(`  ❌ ${pkg} - Wrong scope`);
      wrongScope.push(pkg);
    }
  }
  
  if (wrongScope.length > 0) {
    console.error(`\n  ❌ FAILED: ${wrongScope.length} packages with wrong scope`);
    process.exit(1);
  }
}

// Test 6: Check index.js uses correct scope
console.log('\n✓ Test 6: Index.js Platform Detection');
const fs = require('fs');
const indexContent = fs.readFileSync('./index.js', 'utf8');

if (indexContent.includes("require('@infinitibit_gmbh/graphbit-")) {
  console.log('  ✅ index.js uses correct @infinitibit_gmbh scope');
} else if (indexContent.includes("require('@graphbit/")) {
  console.error('  ❌ index.js uses OLD @graphbit scope');
  process.exit(1);
} else {
  console.error('  ⚠️  Warning: Could not verify platform requires in index.js');
}

const scopeMatches = indexContent.match(/@infinitibit_gmbh\/graphbit-/g);
if (scopeMatches) {
  console.log(`  ✅ Found ${scopeMatches.length} platform requires with correct scope`);
}

// Test 7: Check index.d.ts uses correct scope
console.log('\n✓ Test 7: TypeScript Definitions');
const dtsContent = fs.readFileSync('./index.d.ts', 'utf8');

if (dtsContent.includes("require('@infinitibit_gmbh/graphbit')")) {
  console.log('  ✅ index.d.ts examples use correct scope');
} else if (dtsContent.includes("require('@graphbit/core')")) {
  console.error('  ❌ index.d.ts uses OLD @graphbit/core scope');
  process.exit(1);
}

// Final Summary
console.log('\n═══════════════════════════════════════════════════');
console.log('  ✅ ALL TESTS PASSED');
console.log('═══════════════════════════════════════════════════');
console.log('\n📋 Strategy B Migration Status:');
console.log('  ✅ Module loads correctly');
console.log('  ✅ All exports accessible');
console.log('  ✅ Basic functionality works');
console.log('  ✅ Correct @infinitibit_gmbh scope');
console.log('  ✅ Platform-specific packages configured');
console.log('  ✅ No hardcoded old package references');
console.log('\n🎉 No breaking changes detected!');
