/**
 * Workflow Result Demo
 * 
 * Demonstrates the new WorkflowResult class for structured
 * access to workflow execution results.
 *
 * Run with: npx ts-node examples/workflow-result-demo.ts
 */

import { WorkflowResult, Executor, WorkflowBuilder, LlmConfig } from '../index';

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit WorkflowResult Demo');
  console.log('='.repeat(70));
  console.log();

  console.log('📝 WorkflowResult provides structured access to execution results\n');

  console.log('Example 1: Basic Success/Failure Checking');
  console.log('-'.repeat(70));
  console.log('const result = await executor.execute(workflow);');
  console.log();
  console.log('if (result.isSuccess()) {');
  console.log('  console.log("✅ Workflow completed successfully!");');
  console.log('  processResults(result);');
  console.log('} else if (result.isFailed()) {');
  console.log('  console.error("❌ Workflow failed:", result.error());');
  console.log('  handleError(result);');
  console.log('}');
  console.log();

  console.log('Example 2: Accessing Node Outputs');
  console.log('-'.repeat(70));
  console.log('// Get specific node output');
  console.log('const analyzerOutput = result.getNodeOutput("analyzer_node");');
  console.log('if (analyzerOutput) {');
  console.log('  const data = JSON.parse(analyzerOutput);');
  console.log('  console.log("Sentiment:", data.sentiment);');
  console.log('  console.log("Confidence:", data.confidence);');
  console.log('}');
  console.log();
  console.log('// Get all node outputs');
  console.log('const allOutputs = result.getAllNodeOutputs();');
  console.log('const outputs = JSON.parse(allOutputs);');
  console.log('console.log("All node outputs:", outputs);');
  console.log();

  console.log('Example 3: Working with Variables');
  console.log('-'.repeat(70));
  console.log('// Get individual variable');
  console.log('const userId = result.getVariable("user_id");');
  console.log('console.log("User ID:", userId);');
  console.log();
  console.log('// Get all variables');
  console.log('const vars = result.getAllVariables();');
  console.log('const varsObj = JSON.parse(vars);');
  console.log('console.log("User ID:", varsObj.user_id);');
  console.log('console.log("Session:", varsObj.session_id);');
  console.log();

  console.log('Example 4: Execution Metadata');
  console.log('-'.repeat(70));
  console.log('// Get execution time');
  console.log('const duration = result.executionTimeMs();');
  console.log('console.log(`⏱️  Execution time: ${duration}ms`);');
  console.log();
  console.log('// Get workflow ID');
  console.log('const workflowId = result.workflowId();');
  console.log('console.log("Workflow ID:", workflowId);');
  console.log();
  console.log('// Get detailed statistics');
  console.log('const stats = result.getStats();');
  console.log('if (stats) {');
  console.log('  console.log(`📈 Nodes: ${stats.successfulNodes}/${stats.totalNodes}`);');
  console.log('  console.log(`⏱️  Avg time per node: ${stats.avg_execution_time_ms}ms`);');
  console.log('  console.log(`🔀 Max concurrent: ${stats.maxConcurrentNodes}`);');
  console.log('}');
  console.log();

  console.log('Example 5: Complete Result Snapshot');
  console.log('-'.repeat(70));
  console.log('// Get everything in one call');
  console.log('const resultData = result.toDict();');
  console.log('const data = JSON.parse(resultData);');
  console.log();
  console.log('console.log("Complete Result:");');
  console.log('console.log("  Variables:", data.variables);');
  console.log('console.log("  Node Outputs:", data.nodeOutputs);');
  console.log('console.log("  State:", data.state);');
  console.log('console.log("  Workflow ID:", data.workflowId);');
  console.log('console.log("  Duration:", data.executionDurationMs, "ms");');
  console.log();

  console.log('Example 6: Practical Use Case - Logging');
  console.log('-'.repeat(70));
  console.log('async function logWorkflowResult(result: WorkflowResult) {');
  console.log('  const workflowId = result.workflowId();');
  console.log('  const duration = result.executionTimeMs();');
  console.log('  const state = result.state();');
  console.log('  ');
  console.log('  logger.info({');
  console.log('    workflowId,');
  console.log('    duration,');
  console.log('    state,');
  console.log('    timestamp: new Date().toISOString()');
  console.log('  });');
  console.log('  ');
  console.log('  if (result.isFailed()) {');
  console.log('    logger.error({');
  console.log('      workflowId,');
  console.log('      error: result.error(),');
  console.log('      context: result.toDict()');
  console.log('    });');
  console.log('  }');
  console.log('}');
  console.log();

  console.log('Example 7: Practical Use Case - Database Persistence');
  console.log('-'.repeat(70));
  console.log('async function saveWorkflowResult(result: WorkflowResult) {');
  console.log('  const data = JSON.parse(result.toDict());');
  console.log('  ');
  console.log('  await database.workflowResults.insert({');
  console.log('    id: data.workflowId,');
  console.log('    state: data.state,');
  console.log('    duration_ms: data.executionDurationMs,');
  console.log('    variables: data.variables,');
  console.log('    outputs: data.nodeOutputs,');
  console.log('    created_at: new Date(),');
  console.log('    success: await result.isSuccess()');
  console.log('  });');
  console.log('}');
  console.log();

  console.log('Example 8: Practical Use Case - Result Processing');
  console.log('-'.repeat(70));
  console.log('async function processWorkflowResult(result: WorkflowResult) {');
  console.log('  if (!result.isSuccess()) {');
  console.log('    throw new Error(`Workflow failed: ${result.error()}`);');
  console.log('  }');
  console.log('  ');
  console.log('  // Extract final output');
  console.log('  const finalOutput = result.getNodeOutput("final_node");');
  console.log('  if (!finalOutput) {');
  console.log('    throw new Error("Final node output not found");');
  console.log('  }');
  console.log('  ');
  console.log('  const data = JSON.parse(finalOutput);');
  console.log('  ');
  console.log('  // Log performance');
  console.log('  const duration = result.executionTimeMs();');
  console.log('  if (duration > 10000) {');
  console.log('    console.warn(`Slow execution: ${duration}ms`);');
  console.log('  }');
  console.log('  ');
  console.log('  return data;');
  console.log('}');
  console.log();

  console.log('Example 9: Accessing Underlying Context');
  console.log('-'.repeat(70));
  console.log('// For advanced use cases, access the context directly');
  console.log('const result = await executor.execute(workflow);');
  console.log('const context = result.getContext();');
  console.log();
  console.log('// Use context methods');
  console.log('const nestedValue = await context.getNestedOutput("node.data.field");');
  console.log('await context.setVariable("processed", "true");');
  console.log();

  console.log('='.repeat(70));
  console.log(' Comparison: Old vs New API');
  console.log('='.repeat(70));
  console.log();
  console.log('❌ OLD WAY (Limited):');
  console.log('  const context = await executor.execute(workflow);');
  console.log('  if (await context.isCompleted()) {');
  console.log('    const outputs = await context.getAllOutputs();');
  console.log('    // Limited access to data');
  console.log('  }');
  console.log();
  console.log('✅ NEW WAY (Complete):');
  console.log('  const result = await executor.execute(workflow);');
  console.log('  if (result.isSuccess()) {');
  console.log('    const output = result.getNodeOutput("node1");');
  console.log('    const variable = result.getVariable("user_id");');
  console.log('    const duration = result.executionTimeMs();');
  console.log('    const stats = result.getStats();');
  console.log('    // Full access to everything!');
  console.log('  }');
  console.log();

  console.log('='.repeat(70));
  console.log(' Benefits');
  console.log('='.repeat(70));
  console.log();
  console.log('✅ Structured Result Handling');
  console.log('   - Clear success/failure checking');
  console.log('   - Easy error access');
  console.log('   - Consistent API');
  console.log();
  console.log('✅ Complete Data Access');
  console.log('   - All node outputs');
  console.log('   - All variables');
  console.log('   - Execution metadata');
  console.log('   - Performance statistics');
  console.log();
  console.log('✅ Production-Ready');
  console.log('   - Perfect for logging');
  console.log('   - Easy database persistence');
  console.log('   - Monitoring integration');
  console.log();
  console.log('✅ API Parity with Python');
  console.log('   - Now matches Python\'s WorkflowResult');
  console.log('   - Same method names');
  console.log('   - Same capabilities');
  console.log();
}

main().catch(console.error);

