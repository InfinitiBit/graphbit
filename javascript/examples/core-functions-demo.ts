/**
 * Core Functions Demo
 * 
 * Demonstrates enhanced core runtime functions:
 * - init() with configuration options
 * - getSystemInfo() for environment details
 * - healthCheck() for monitoring
 * - configureRuntime() for resource management
 *
 * Run with: npx ts-node examples/core-functions-demo.ts
 */

import { init, version, versionInfo, getSystemInfo, healthCheck, configureRuntime } from '../index';

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Enhanced Core Functions Demo');
  console.log('='.repeat(70));
  console.log();

  // Example 1: Basic Initialization
  console.log('Example 1: Basic Initialization');
  console.log('-'.repeat(70));
  console.log('// Simple initialization with defaults');
  console.log('init();');
  console.log('console.log("GraphBit initialized!");');
  
  init();
  console.log('✅ GraphBit initialized with defaults\n');

  // Example 2: Initialization with Configuration
  console.log('Example 2: Initialization with Configuration');
  console.log('-'.repeat(70));
  console.log('// Initialize with custom log level');
  console.log('init({');
  console.log('  logLevel: "debug",      // trace | debug | info | warn | error');
  console.log('  coloredLogs: true,      // Enable colored output');
  console.log('  logOutput: "stdout"     // stdout | stderr');
  console.log('});');
  console.log();

  // Example 3: Version Information
  console.log('Example 3: Version Information');
  console.log('-'.repeat(70));
  
  const versionStr = version();
  console.log(`Version (simple): ${versionStr}`);
  
  const versionDetails = versionInfo();
  console.log('\nVersion (detailed):');
  console.log(`  GraphBit: ${versionDetails.version}`);
  console.log(`  Rust: ${versionDetails.rustVersion}`);
  console.log(`  NAPI: ${versionDetails.napiVersion}`);
  console.log();

  // Example 4: System Information
  console.log('Example 4: System Information');
  console.log('-'.repeat(70));
  
  const sysInfo = getSystemInfo();
  console.log('📊 System Information:');
  console.log(`  OS: ${sysInfo.os}`);
  console.log(`  Architecture: ${sysInfo.arch}`);
  console.log(`  CPUs: ${sysInfo.cpuCount}`);
  console.log(`  GraphBit: ${sysInfo.graphbitVersion}`);
  console.log();
  console.log('💡 Use Cases:');
  console.log('   - Diagnostics and debugging');
  console.log('   - Performance optimization');
  console.log('   - System requirements checking');
  console.log();

  // Example 5: Health Check
  console.log('Example 5: Health Check');
  console.log('-'.repeat(70));
  
  const health = healthCheck();
  console.log('🏥 Health Status:');
  console.log(`  Status: ${health.healthy ? '✅ Healthy' : '❌ Unhealthy'}`);
  console.log(`  Version: ${health.version}`);
  console.log(`  Timestamp: ${new Date(health.timestamp * 1000).toISOString()}`);
  console.log();
  console.log('💡 Use Cases:');
  console.log('   - Monitoring endpoints');
  console.log('   - Load balancer health checks');
  console.log('   - Service discovery');
  console.log();

  // Example 6: Runtime Configuration
  console.log('Example 6: Runtime Configuration');
  console.log('-'.repeat(70));
  console.log('// Configure runtime settings');
  console.log('configureRuntime({');
  console.log('  maxThreads: 4,           // Limit thread pool size');
  console.log('  enableMonitoring: true,  // Enable performance monitoring');
  console.log('  memoryLimitMb: 2048      // Set memory limit');
  console.log('});');
  
  configureRuntime({
    maxThreads: 4,
    enableMonitoring: true,
    memoryLimitMb: 2048
  });
  
  console.log('✅ Runtime configured');
  console.log();

  // Example 7: Production Setup Pattern
  console.log('Example 7: Production Setup Pattern');
  console.log('-'.repeat(70));
  console.log('// Complete production initialization');
  console.log('function initializeForProduction() {');
  console.log('  // Initialize with production logging');
  console.log('  init({');
  console.log('    logLevel: "warn",        // Only warnings and errors');
  console.log('    coloredLogs: false,      // Plain text for log aggregators');
  console.log('    logOutput: "stderr"      // Errors to stderr');
  console.log('  });');
  console.log('  ');
  console.log('  // Configure runtime limits');
  console.log('  configureRuntime({');
  console.log('    maxThreads: 4,');
  console.log('    enableMonitoring: true,');
  console.log('    memoryLimitMb: 2048');
  console.log('  });');
  console.log('  ');
  console.log('  // Log system information');
  console.log('  const info = getSystemInfo();');
  console.log('  console.log(`Started on ${info.os} with ${info.cpuCount} CPUs`);');
  console.log('  ');
  console.log('  // Verify health');
  console.log('  const health = healthCheck();');
  console.log('  if (!health.healthy) {');
  console.log('    throw new Error("Health check failed");');
  console.log('  }');
  console.log('}');
  console.log();

  // Example 8: Development Setup Pattern
  console.log('Example 8: Development Setup Pattern');
  console.log('-'.repeat(70));
  console.log('// Complete development initialization');
  console.log('function initializeForDevelopment() {');
  console.log('  // Initialize with verbose logging');
  console.log('  init({');
  console.log('    logLevel: "debug",       // Verbose logging');
  console.log('    coloredLogs: true,       // Colored for readability');
  console.log('    logOutput: "stdout"      // All to stdout');
  console.log('  });');
  console.log('  ');
  console.log('  // No resource limits in development');
  console.log('  configureRuntime({');
  console.log('    maxThreads: 8,');
  console.log('    enableMonitoring: true,');
  console.log('    memoryLimitMb: 4096');
  console.log('  });');
  console.log('  ');
  console.log('  // Display system info for debugging');
  console.log('  const info = getSystemInfo();');
  console.log('  console.log("System Info:", info);');
  console.log('}');
  console.log();

  // Example 9: Monitoring Endpoint Pattern
  console.log('Example 9: Monitoring Endpoint Pattern');
  console.log('-'.repeat(70));
  console.log('// Express.js health endpoint');
  console.log('app.get("/health", (req, res) => {');
  console.log('  const health = healthCheck();');
  console.log('  const info = getSystemInfo();');
  console.log('  ');
  console.log('  res.status(health.healthy ? 200 : 503).json({');
  console.log('    ...health,');
  console.log('    system: {');
  console.log('      os: info.os,');
  console.log('      cpus: info.cpuCount,');
  console.log('      memory: info.totalMemoryMb');
  console.log('    }');
  console.log('  });');
  console.log('});');
  console.log();

  console.log('='.repeat(70));
  console.log(' Summary');
  console.log('='.repeat(70));
  console.log();
  console.log('✅ Enhanced Core Functions:');
  console.log('   - init() with configuration options');
  console.log('   - getSystemInfo() for environment details');
  console.log('   - healthCheck() for monitoring');
  console.log('   - configureRuntime() for resource management');
  console.log();
  console.log('💡 Benefits:');
  console.log('   - Configurable logging (level, colors, output)');
  console.log('   - System diagnostics and debugging');
  console.log('   - Production health monitoring');
  console.log('   - Runtime resource control');
  console.log();
  console.log('📚 Use Cases:');
  console.log('   - Production deployments');
  console.log('   - Development debugging');
  console.log('   - Monitoring and alerting');
  console.log('   - Load balancer integration');
  console.log();
}

main().catch(console.error);

