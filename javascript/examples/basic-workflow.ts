/**
 * Basic Workflow Example
 *
 * This example demonstrates how to create and execute a simple workflow
 * using the GraphBit JavaScript bindings.
 */

import { init, WorkflowBuilder, Executor, LlmConfig } from '@graphbit/core';

async function main() {
  // Initialize the GraphBit library
  init();
  console.log('GraphBit initialized');

  // Configure the LLM provider (OpenAI in this example)
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini',
    temperature: 0.7,
    maxTokens: 1000,
  });

  // Create a simple workflow
  const workflow = new WorkflowBuilder('Hello World Workflow')
    .description('A simple workflow that demonstrates basic functionality')
    .addMetadata('version', JSON.stringify('1.0'))
    .addMetadata('author', JSON.stringify('GraphBit Team'))
    .build();

  console.log('Workflow created:', await workflow.name());
  console.log('Workflow ID:', await workflow.id());

  // Create an executor with configuration
  const executor = new Executor(llmConfig, {
    timeoutSeconds: 60,
    debug: true,
    maxParallel: 4,
  });

  try {
    // Execute the workflow
    console.log('Executing workflow...');
    const result = await executor.execute(workflow);

    // Check the result
    const isCompleted = await result.isCompleted();
    const isFailed = await result.isFailed();

    if (isCompleted) {
      console.log('✓ Workflow completed successfully');

      // Get execution statistics
      const stats = await result.stats();
      if (stats) {
        console.log('Execution stats:', {
          duration: `${stats.totalDurationMs}ms`,
          nodesExecuted: stats.nodesExecuted,
          nodesFailed: stats.nodesFailed,
          nodesSkipped: stats.nodesSkipped,
        });
      }

      // Get all outputs
      const outputs = await result.getAllOutputs();
      console.log('Outputs:', outputs);
    } else if (isFailed) {
      console.error('✗ Workflow failed');
      const error = await result.error();
      console.error('Error:', error);
    }
  } catch (error) {
    console.error('Failed to execute workflow:', error);
    process.exit(1);
  }
}

// Run the example
main().catch(console.error);
