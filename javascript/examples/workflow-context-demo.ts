/**
 * Workflow Context Demo
 * 
 * Demonstrates the enhanced WorkflowContext methods:
 * - Variable management
 * - Node output access
 * - Workflow introspection
 * - Complete context snapshots
 *
 * Run with: npx ts-node examples/workflow-context-demo.ts
 */

import { WorkflowContext, WorkflowBuilder, Executor, LlmConfig, WorkflowGraph } from '../index';

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Enhanced WorkflowContext Demo');
  console.log('='.repeat(70));
  console.log();

  // Note: This is a demo showing the API usage
  // Actual workflow execution would require proper setup

  console.log('📝 WorkflowContext now has 8 new powerful methods!\n');

  console.log('Example 1: Variable Management');
  console.log('-'.repeat(70));
  console.log('// Set variables during workflow execution');
  console.log('await context.setVariable("user_id", "12345");');
  console.log('await context.setVariable("preferences", JSON.stringify({');
  console.log('  theme: "dark",');
  console.log('  language: "en",');
  console.log('  notifications: true');
  console.log('}));');
  console.log();
  console.log('// Get individual variables');
  console.log('const userId = await context.getVariable("user_id");');
  console.log('console.log("User ID:", userId); // "12345"');
  console.log();
  console.log('// Get all variables');
  console.log('const allVars = await context.getAllVariables();');
  console.log('const vars = JSON.parse(allVars);');
  console.log('console.log(vars.user_id, vars.preferences);');
  console.log();

  console.log('Example 2: Node Output Access');
  console.log('-'.repeat(70));
  console.log('// Get a specific node\'s output');
  console.log('const analyzerOutput = await context.getNodeOutput("analyzer_node");');
  console.log('if (analyzerOutput) {');
  console.log('  const result = JSON.parse(analyzerOutput);');
  console.log('  console.log("Analysis:", result);');
  console.log('}');
  console.log();
  console.log('// Get nested values using dot notation');
  console.log('const score = await context.getNestedOutput("analyzer.results.score");');
  console.log('const confidence = await context.getNestedOutput("analyzer.results.confidence");');
  console.log('console.log(`Score: ${score}, Confidence: ${confidence}`);');
  console.log();

  console.log('Example 3: Workflow Metadata');
  console.log('-'.repeat(70));
  console.log('// Get workflow identifier');
  console.log('const workflowId = await context.getWorkflowId();');
  console.log('console.log("Workflow ID:", workflowId);');
  console.log('// Useful for logging, tracking, correlation');
  console.log();
  console.log('// Get execution duration');
  console.log('const duration = await context.getExecutionDuration();');
  console.log('console.log(`Execution time: ${duration}ms`);');
  console.log();
  console.log('if (duration > 30000) {');
  console.log('  console.warn("Workflow took longer than 30 seconds!");');
  console.log('}');
  console.log();

  console.log('Example 4: Complete Context Snapshot');
  console.log('-'.repeat(70));
  console.log('// Get everything in one call');
  console.log('const contextSnapshot = await context.toDict();');
  console.log('const data = JSON.parse(contextSnapshot);');
  console.log();
  console.log('console.log("Complete Context:");');
  console.log('console.log("  Variables:", data.variables);');
  console.log('console.log("  Node Outputs:", data.nodeOutputs);');
  console.log('console.log("  State:", data.state);');
  console.log('console.log("  Workflow ID:", data.workflowId);');
  console.log('console.log("  Duration:", data.executionDurationMs, "ms");');
  console.log();
  console.log('// Perfect for logging, debugging, or saving to database');
  console.log('await database.saveWorkflowContext(data.workflowId, data);');
  console.log();

  console.log('Example 5: Existing Methods Still Work');
  console.log('-'.repeat(70));
  console.log('// All existing methods are still available');
  console.log('const isCompleted = await context.isCompleted();');
  console.log('const isFailed = await context.isFailed();');
  console.log('const state = await context.state();');
  console.log('const stats = await context.stats();');
  console.log('const error = await context.error();');
  console.log('const allOutputs = await context.getAllOutputs();');
  console.log();

  console.log('Example 6: Practical Use Case - Monitoring');
  console.log('-'.repeat(70));
  console.log('// Monitor workflow execution');
  console.log('async function monitorWorkflow(context: WorkflowContext) {');
  console.log('  const workflowId = await context.getWorkflowId();');
  console.log('  const duration = await context.getExecutionDuration();');
  console.log('  const state = await context.state();');
  console.log('  ');
  console.log('  console.log(`[${workflowId}] State: ${state}, Duration: ${duration}ms`);');
  console.log('  ');
  console.log('  if (await context.isFailed()) {');
  console.log('    const error = await context.error();');
  console.log('    console.error(`[${workflowId}] Failed: ${error}`);');
  console.log('    // Send alert');
  console.log('  }');
  console.log('  ');
  console.log('  if (duration > 60000) {');
  console.log('    console.warn(`[${workflowId}] Slow execution detected`);');
  console.log('    // Send monitoring alert');
  console.log('  }');
  console.log('}');
  console.log();

  console.log('Example 7: Practical Use Case - Data Pipeline');
  console.log('-'.repeat(70));
  console.log('// Pass data between nodes using variables');
  console.log('async function dataPipeline(context: WorkflowContext) {');
  console.log('  // Node 1: Extract data');
  console.log('  const rawData = await fetchData();');
  console.log('  await context.setVariable("raw_data", JSON.stringify(rawData));');
  console.log('  ');
  console.log('  // Node 2: Transform data');
  console.log('  const rawDataStr = await context.getVariable("raw_data");');
  console.log('  const raw = JSON.parse(rawDataStr!);');
  console.log('  const transformed = transform(raw);');
  console.log('  await context.setVariable("transformed_data", JSON.stringify(transformed));');
  console.log('  ');
  console.log('  // Node 3: Load data');
  console.log('  const transformedStr = await context.getVariable("transformed_data");');
  console.log('  const final = JSON.parse(transformedStr!);');
  console.log('  await saveToDatabase(final);');
  console.log('}');
  console.log();

  console.log('Example 8: Practical Use Case - Result Aggregation');
  console.log('-'.repeat(70));
  console.log('// Collect results from multiple nodes');
  console.log('async function aggregateResults(context: WorkflowContext) {');
  console.log('  const results = {');
  console.log('    sentiment: await context.getNestedOutput("sentiment_node.score"),');
  console.log('    entities: await context.getNestedOutput("entity_node.entities"),');
  console.log('    summary: await context.getNestedOutput("summary_node.text"),');
  console.log('    keywords: await context.getNestedOutput("keyword_node.keywords")');
  console.log('  };');
  console.log('  ');
  console.log('  // Save aggregated results');
  console.log('  await context.setVariable("final_analysis", JSON.stringify(results));');
  console.log('  ');
  console.log('  return results;');
  console.log('}');
  console.log();

  console.log('='.repeat(70));
  console.log(' Benefits of Enhanced WorkflowContext');
  console.log('='.repeat(70));
  console.log();
  console.log('✅ Full Workflow Introspection');
  console.log('   - Access any variable or node output');
  console.log('   - Navigate nested data structures');
  console.log('   - Get workflow metadata');
  console.log();
  console.log('✅ Better Debugging');
  console.log('   - Complete context snapshots');
  console.log('   - Execution duration tracking');
  console.log('   - Workflow ID for correlation');
  console.log();
  console.log('✅ Production-Ready Monitoring');
  console.log('   - Track execution times');
  console.log('   - Monitor workflow states');
  console.log('   - Log complete context');
  console.log();
  console.log('✅ API Parity with Python');
  console.log('   - Now matches Python\'s 14 methods');
  console.log('   - Same capabilities');
  console.log('   - Consistent API');
  console.log();

  console.log('💡 Migration Tips:');
  console.log();
  console.log('Old Way:');
  console.log('  const outputs = await context.getAllOutputs();');
  console.log('  const data = JSON.parse(outputs);');
  console.log('  // Limited access to workflow data');
  console.log();
  console.log('New Way:');
  console.log('  const data = await context.toDict();');
  console.log('  const contextData = JSON.parse(data);');
  console.log('  // Full access: variables, outputs, metadata, state, duration');
  console.log();

  console.log('📚 For more information:');
  console.log('  - See VERIFICATION_REPORT.md for full API comparison');
  console.log('  - See REMEDIATION_PLAN.md for implementation details');
  console.log('  - Check tests/unit/workflow-context.test.ts for examples');
  console.log();
}

// Helper functions for demo (not actual implementations)
async function fetchData() {
  return { data: 'example' };
}

function transform(data: any) {
  return { ...data, transformed: true };
}

async function saveToDatabase(data: any) {
  console.log('Saved:', data);
}

main().catch(console.error);

