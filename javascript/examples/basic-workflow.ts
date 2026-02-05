/**
 * Basic Workflow Example
 *
 * This example demonstrates how to create and execute a simple workflow
 * using the GraphBit JavaScript bindings.
 */

import 'dotenv/config';
import { init, WorkflowBuilder, Executor, LlmConfig, AgentBuilder } from 'graphbit';

async function main() {
  // Initialize the GraphBit library
  init();
  console.log('GraphBit initialized');

  // Configure the LLM provider (OpenAI in this example)
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || 'sk-proj-**',
    model: 'gpt-4o-mini',
  });

  // Create an agent
  const agentBuilder = new AgentBuilder('Assistant', llmConfig)
    .description('A helpful assistant')
    .systemPrompt('You are a helpful assistant.')
    .temperature(0.7);

  const agent = await agentBuilder.build();
  console.log('Agent created:', await agent.name());

  // Create a simple workflow
  const workflow = new WorkflowBuilder('Hello World Workflow')
    .description('A simple workflow that demonstrates basic functionality')
    .addMetadata('version', JSON.stringify('1.0'))
    .addMetadata('author', JSON.stringify('GraphBit Team'))
    .build();

  console.log('Workflow created:', await workflow.name());
  console.log('Workflow ID:', await workflow.id());

  // Add the agent to the workflow
  const agentId = await agent.id();
  const nodeId = await workflow.addNode({
    id: agentId.uuid,
    name: await agent.name(),
    description: await agent.description(),
    nodeType: "Agent"
  });
  console.log('Agent added to workflow as node:', nodeId);

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
    const isCompleted = await result.isSuccess();
    const isFailed = await result.isFailed();

    if (isCompleted) {
      console.log('✓ Workflow completed successfully');

      // Get execution statistics
      const stats = await result.getStats();
      if (stats) {
        console.log('Execution stats:', {
          duration: `${stats.totalExecutionTimeMs}ms`,
          nodesExecuted: stats.successfulNodes,
          nodesFailed: stats.failedNodes,
        });
      }

      // Get all outputs
      const outputs = await result.getAllNodeOutputs();
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
