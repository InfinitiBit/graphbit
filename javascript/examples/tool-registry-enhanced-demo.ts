/**
 * Enhanced ToolRegistry Demo
 * 
 * Demonstrates all new ToolRegistry features:
 * - Tool metadata tracking
 * - Execution history
 * - Statistics monitoring
 * - Tool management
 *
 * Run with: npx ts-node examples/tool-registry-enhanced-demo.ts
 */

import { createToolRegistry, ToolRegistry } from '../index';

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Enhanced ToolRegistry Demo');
  console.log('='.repeat(70));
  console.log();

  // Create registry
  console.log('📝 Creating tool registry...');
  const registry = createToolRegistry();
  console.log('✅ Registry created!\n');

  // Example 1: Register Tools
  console.log('Example 1: Register Tools');
  console.log('-'.repeat(70));
  
  registry.register('add', 'Add two numbers', {
    a: { type: 'number' },
    b: { type: 'number' }
  }, (args: any) => {
    return args.a + args.b;
  });
  
  registry.register('multiply', 'Multiply two numbers', {
    a: { type: 'number' },
    b: { type: 'number' }
  }, (args: any) => {
    return args.a * args.b;
  });
  
  registry.register('greet', 'Greet a person', {
    name: { type: 'string' }
  }, (args: any) => {
    return `Hello, ${args.name}!`;
  });
  
  console.log(`✅ Registered ${registry.getToolCount()} tools`);
  console.log(`   Tools: ${registry.getRegisteredTools().join(', ')}`);
  console.log();

  // Example 2: Execute Tools and Track Usage
  console.log('Example 2: Execute Tools');
  console.log('-'.repeat(70));
  
  console.log('Executing tools...\n');
  
  let result1 = await registry.execute('add', { a: 5, b: 3 });
  console.log(`  add(5, 3) = ${result1.result} (${result1.executionTimeMs.toFixed(2)}ms)`);
  
  let result2 = await registry.execute('multiply', { a: 4, b: 7 });
  console.log(`  multiply(4, 7) = ${result2.result} (${result2.executionTimeMs.toFixed(2)}ms)`);
  
  let result3 = await registry.execute('greet', { name: 'Alice' });
  console.log(`  greet("Alice") = "${result3.result}" (${result3.executionTimeMs.toFixed(2)}ms)`);
  
  let result4 = await registry.execute('add', { a: 10, b: 20 });
  console.log(`  add(10, 20) = ${result4.result} (${result4.executionTimeMs.toFixed(2)}ms)`);
  console.log();

  // Example 3: View Metadata
  console.log('Example 3: Tool Metadata');
  console.log('-'.repeat(70));
  
  const addMetadata = registry.getToolMetadata('add');
  if (addMetadata) {
    console.log('📊 "add" tool metadata:');
    console.log(`   Name: ${addMetadata.name}`);
    console.log(`   Description: ${addMetadata.description}`);
    console.log(`   Call count: ${addMetadata.callCount}`);
    console.log(`   Total duration: ${addMetadata.totalDurationMs.toFixed(2)}ms`);
    console.log(`   Avg duration: ${addMetadata.avgDurationMs.toFixed(2)}ms`);
    if (addMetadata.lastCalledAt) {
      const lastCall = new Date(addMetadata.lastCalledAt * 1000);
      console.log(`   Last called: ${lastCall.toISOString()}`);
    }
  }
  console.log();

  // Example 4: View All Metadata
  console.log('Example 4: All Tool Metadata');
  console.log('-'.repeat(70));
  
  const allMetadata = registry.getAllMetadata();
  console.log('📊 All registered tools:\n');
  
  allMetadata.forEach((meta, idx) => {
    console.log(`${idx + 1}. ${meta.name}`);
    console.log(`   Calls: ${meta.callCount}`);
    console.log(`   Avg duration: ${meta.avgDurationMs.toFixed(2)}ms`);
  });
  console.log();

  // Example 5: Execution History
  console.log('Example 5: Execution History');
  console.log('-'.repeat(70));
  
  const history = registry.getExecutionHistory();
  console.log(`📋 Execution history (${history.length} records):\n`);
  
  history.forEach((exec, idx) => {
    const status = exec.success ? '✅' : '❌';
    const time = new Date(exec.timestamp * 1000).toLocaleTimeString();
    console.log(`${idx + 1}. ${status} ${exec.toolName} (${exec.durationMs.toFixed(2)}ms) at ${time}`);
  });
  console.log();

  // Example 6: Statistics
  console.log('Example 6: Comprehensive Statistics');
  console.log('-'.repeat(70));
  
  const stats = registry.getStats();
  console.log('📊 Registry Statistics:');
  console.log(`   Total tools: ${stats.totalTools}`);
  console.log(`   Total executions: ${stats.totalExecutions}`);
  console.log(`   Successful: ${stats.successfulExecutions}`);
  console.log(`   Failed: ${stats.failedExecutions}`);
  
  if (stats.totalExecutions > 0) {
    const successRate = (stats.successfulExecutions / stats.totalExecutions * 100).toFixed(2);
    console.log(`   Success rate: ${successRate}%`);
  }
  
  console.log(`   Avg execution time: ${stats.avgExecutionTimeMs.toFixed(2)}ms`);
  console.log(`   Total execution time: ${stats.totalExecutionTimeMs.toFixed(2)}ms`);
  console.log();

  // Example 7: LLM Integration
  console.log('Example 7: LLM Tool Format');
  console.log('-'.repeat(70));
  
  const llmTools = registry.getLlmTools();
  console.log(`📋 ${llmTools.length} tools in LLM format`);
  console.log('   (Can be passed to LLM APIs that support function calling)');
  console.log();

  // Example 8: Error Handling
  console.log('Example 8: Error Handling');
  console.log('-'.repeat(70));
  
  registry.register('error_tool', 'Tool that throws', {}, () => {
    throw new Error('Intentional error for demo');
  });
  
  const errorResult = await registry.execute('error_tool', {});
  console.log(`Status: ${errorResult.success ? '✅ Success' : '❌ Failed'}`);
  console.log(`Error: ${errorResult.error}`);
  console.log(`Duration: ${errorResult.executionTimeMs.toFixed(2)}ms`);
  console.log();

  // Example 9: Tool Lifecycle Management
  console.log('Example 9: Tool Lifecycle Management');
  console.log('-'.repeat(70));
  
  console.log(`Initial tool count: ${registry.getToolCount()}`);
  
  registry.register('temp_tool', 'Temporary tool', {}, () => 'temp');
  console.log(`After register: ${registry.getToolCount()}`);
  
  const removed = registry.unregisterTool('temp_tool');
  console.log(`After unregister: ${registry.getToolCount()} (removed: ${removed})`);
  console.log();

  // Example 10: History Analysis
  console.log('Example 10: History Analysis');
  console.log('-'.repeat(70));
  
  const slowExecutions = registry.getExecutionHistory().filter(e => e.durationMs > 5);
  console.log(`Slow executions (>5ms): ${slowExecutions.length}`);
  
  const failures = registry.getExecutionHistory().filter(e => !e.success);
  console.log(`Failed executions: ${failures.length}`);
  
  const mostUsedTool = allMetadata.sort((a, b) => b.callCount - a.callCount)[0];
  if (mostUsedTool) {
    console.log(`Most used tool: "${mostUsedTool.name}" (${mostUsedTool.callCount} calls)`);
  }
  console.log();

  // Example 11: Monitoring Pattern
  console.log('Example 11: Production Monitoring Pattern');
  console.log('-'.repeat(70));
  console.log('async function monitorTools(registry: ToolRegistry) {');
  console.log('  const stats = registry.getStats();');
  console.log('  ');
  console.log('  // Alert on high failure rate');
  console.log('  const failureRate = stats.failedExecutions / stats.totalExecutions;');
  console.log('  if (failureRate > 0.1) {');
  console.log('    sendAlert(`Tool failure rate: ${failureRate * 100}%`);');
  console.log('  }');
  console.log('  ');
  console.log('  // Alert on slow executions');
  console.log('  if (stats.avgExecutionTimeMs > 1000) {');
  console.log('    sendAlert(`Slow tools detected: ${stats.avgExecutionTimeMs}ms avg`);');
  console.log('  }');
  console.log('  ');
  console.log('  // Log for analytics');
  console.log('  analytics.track("tool_usage", {');
  console.log('    totalTools: stats.totalTools,');
  console.log('    executions: stats.totalExecutions,');
  console.log('    successRate: stats.successfulExecutions / stats.totalExecutions');
  console.log('  });');
  console.log('}');
  console.log();

  // Final Summary
  console.log('='.repeat(70));
  console.log(' Summary');
  console.log('='.repeat(70));
  
  const finalStats = registry.getStats();
  console.log();
  console.log('✅ New ToolRegistry Features:');
  console.log('   - unregisterTool() - Remove tools');
  console.log('   - getToolMetadata() - Get tool usage info');
  console.log('   - getAllMetadata() - Get all tool info');
  console.log('   - getExecutionHistory() - View execution log');
  console.log('   - clearHistory() - Clear execution records');
  console.log('   - getStats() - Comprehensive statistics');
  console.log('   - clearAll() - Remove all tools');
  console.log('   - getLlmTools() - LLM-compatible format');
  console.log('   - getToolCount() - Count registered tools');
  console.log();
  console.log('💡 Benefits:');
  console.log('   - Complete tool lifecycle management');
  console.log('   - Production monitoring capabilities');
  console.log('   - Performance tracking');
  console.log('   - Debugging and analytics');
  console.log();
  console.log('📊 Current Stats:');
  console.log(`   Tools: ${finalStats.totalTools}`);
  console.log(`   Executions: ${finalStats.totalExecutions}`);
  console.log(`   Success rate: ${(finalStats.successfulExecutions / finalStats.totalExecutions * 100).toFixed(2)}%`);
  console.log();
}

main().catch(console.error);

