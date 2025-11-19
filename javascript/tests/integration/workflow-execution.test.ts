import { describe, it, expect, beforeAll } from 'vitest';
import { init, WorkflowBuilder, Executor, LlmConfig } from '../../index';

describe('Workflow Execution Integration Tests', () => {
  beforeAll(() => {
    init();
  });

  it.skip('should execute a simple workflow with OpenAI', async () => {
    // Skip by default as it requires API key and network access
    const apiKey = process.env.OPENAI_API_KEY;
    if (!apiKey) {
      console.log('Skipping test: OPENAI_API_KEY not set');
      return;
    }

    const llmConfig = LlmConfig.openai({
      apiKey,
      model: 'gpt-4o-mini',
    });

    const workflow = new WorkflowBuilder('Test Workflow')
      .description('A simple test workflow')
      .build();

    const executor = new Executor(llmConfig, {
      timeoutSeconds: 60,
      debug: true,
    });

    const result = await executor.execute(workflow);

    expect(result).toBeDefined();
    const isCompleted = await result.isCompleted();
    const isFailed = await result.isFailed();

    expect(isCompleted || isFailed).toBe(true);
  });

  it.skip('should execute a workflow with Anthropic', async () => {
    // Skip by default as it requires API key and network access
    const apiKey = process.env.ANTHROPIC_API_KEY;
    if (!apiKey) {
      console.log('Skipping test: ANTHROPIC_API_KEY not set');
      return;
    }

    const llmConfig = LlmConfig.anthropic({
      apiKey,
      model: 'claude-3-5-sonnet-20241022',
    });

    const workflow = new WorkflowBuilder('Anthropic Test Workflow')
      .description('Testing with Anthropic')
      .build();

    const executor = new Executor(llmConfig, {
      timeoutSeconds: 60,
      debug: true,
    });

    const result = await executor.execute(workflow);

    expect(result).toBeDefined();
    const isCompleted = await result.isCompleted();
    const isFailed = await result.isFailed();

    expect(isCompleted || isFailed).toBe(true);
  });

  it.skip('should handle workflow timeout', async () => {
    // Skip by default as it requires API key
    const apiKey = process.env.OPENAI_API_KEY;
    if (!apiKey) {
      console.log('Skipping test: OPENAI_API_KEY not set');
      return;
    }

    const llmConfig = LlmConfig.openai({
      apiKey,
      model: 'gpt-4o-mini',
    });

    const workflow = new WorkflowBuilder('Timeout Test').build();

    const executor = new Executor(llmConfig, {
      timeoutSeconds: 1, // Very short timeout
      debug: false,
    });

    await expect(executor.execute(workflow)).rejects.toThrow();
  });

  it.skip('should get execution statistics', async () => {
    // Skip by default as it requires API key
    const apiKey = process.env.OPENAI_API_KEY;
    if (!apiKey) {
      console.log('Skipping test: OPENAI_API_KEY not set');
      return;
    }

    const llmConfig = LlmConfig.openai({
      apiKey,
      model: 'gpt-4o-mini',
    });

    const workflow = new WorkflowBuilder('Stats Test').build();

    const executor = new Executor(llmConfig, {
      timeoutSeconds: 60,
    });

    const result = await executor.execute(workflow);
    const stats = await result.stats();

    if (stats) {
      expect(stats).toHaveProperty('totalDurationMs');
      expect(stats).toHaveProperty('nodesExecuted');
      expect(stats).toHaveProperty('nodesFailed');
      expect(stats).toHaveProperty('nodesSkipped');
    }
  });
});
