import { describe, it, expect } from 'vitest';
import { WorkflowBuilder } from '../../index';

describe('WorkflowBuilder', () => {
  it('should create a workflow builder', () => {
    const builder = new WorkflowBuilder('Test Workflow');
    expect(builder).toBeDefined();
  });

  it('should set workflow description', () => {
    const builder = new WorkflowBuilder('Test Workflow');
    builder.description('A test workflow');
    expect(builder).toBeDefined();
  });

  it('should add metadata', () => {
    const builder = new WorkflowBuilder('Test Workflow');
    builder.addMetadata('key', JSON.stringify({ value: 'test' }));
    expect(builder).toBeDefined();
  });

  it('should build a workflow', () => {
    const builder = new WorkflowBuilder('Test Workflow');
    const workflow = builder.build();
    expect(workflow).toBeDefined();
  });

  it('should chain builder methods', () => {
    const builder = new WorkflowBuilder('Test Workflow');
    const workflow = builder
      .description('A test workflow')
      .addMetadata('version', JSON.stringify('1.0'))
      .build();
    expect(workflow).toBeDefined();
  });
});

describe('Workflow', () => {
  it('should get workflow ID', async () => {
    const builder = new WorkflowBuilder('Test Workflow');
    const workflow = builder.build();
    const id = await workflow.id();
    expect(id).toBeDefined();
    expect(typeof id).toBe('string');
  });

  it('should get workflow name', async () => {
    const builder = new WorkflowBuilder('Test Workflow');
    const workflow = builder.build();
    const name = await workflow.name();
    expect(name).toBe('Test Workflow');
  });

  it('should get workflow description', async () => {
    const builder = new WorkflowBuilder('Test Workflow');
    builder.description('A test workflow');
    const workflow = builder.build();
    const description = await workflow.description();
    expect(description).toBe('A test workflow');
  });

  it('should get workflow description', async () => {
    const builder = new WorkflowBuilder('Test Workflow');
    const workflow = builder.build();
    const description = await workflow.description();
    expect(description).toBeDefined();
    expect(typeof description).toBe('string');
  });
});

describe('WorkflowContext', () => {
  // Note: These tests would require actual workflow execution
  // which needs LLM configuration and network access
  // They are placeholders for integration tests

  it.skip('should check if workflow is completed', async () => {
    // This would be tested in integration tests
  });

  it.skip('should check if workflow failed', async () => {
    // This would be tested in integration tests
  });

  it.skip('should get workflow state', async () => {
    // This would be tested in integration tests
  });

  it.skip('should get execution statistics', async () => {
    // This would be tested in integration tests
  });

  it.skip('should get error message if failed', async () => {
    // This would be tested in integration tests
  });

  it.skip('should get all node outputs', async () => {
    // This would be tested in integration tests
  });
});

describe('Executor', () => {
  it.skip('should create an executor', () => {
    // This would be tested in integration tests with real LLM config
  });

  it.skip('should execute a workflow', async () => {
    // This would be tested in integration tests
  });
});
