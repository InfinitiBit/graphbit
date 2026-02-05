import { describe, it, expect } from 'vitest';
import { WorkflowBuilder, LlmConfig, Executor } from '../../index';

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

  describe('addNode()', () => {
    it('should add an Agent node to the workflow', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440100',
        name: 'Test Agent',
        description: 'A test agent node',
        nodeType: 'Agent',
      };

      const nodeId = await workflow.addNode(node);
      expect(nodeId).toBe(node!.id);
    });

    it('should add a Condition node to the workflow', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440101',
        name: 'Test Condition',
        description: 'A test condition node',
        nodeType: 'Condition',
      };

      const nodeId = await workflow.addNode(node);
      expect(nodeId).toBe(node!.id);
    });

    it('should add a Transform node to the workflow', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440102',
        name: 'Test Transform',
        description: 'A test transform node',
        nodeType: 'Transform',
      };

      const nodeId = await workflow.addNode(node);
      expect(nodeId).toBe(node!.id);
    });

    it('should add multiple nodes to the workflow', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440110',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440111',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      const id1 = await workflow.addNode(node1);
      const id2 = await workflow.addNode(node2);

      expect(id1).toBe(node1!.id);
      expect(id2).toBe(node2!.id);
    });

    it('should reject invalid node type', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440120',
        name: 'Invalid Node',
        description: 'A node with invalid type',
        nodeType: 'InvalidType',
      };

      await expect(workflow.addNode(node)).rejects.toThrow();
    });
  });

  describe('addEdge()', () => {
    it('should add an edge between two nodes', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440130',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440131',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node1);
      await workflow.addNode(node2);

      const edge = {
        fromNode: node1!.id,
        toNode: node2!.id,
        condition: undefined,
      };

      await expect(workflow.addEdge(node1!.id, node2!.id, edge)).resolves.not.toThrow();
    });

    it('should add an edge with a condition', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440140',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440141',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node1);
      await workflow.addNode(node2);

      const edge = {
        fromNode: node1!.id,
        toNode: node2!.id,
        condition: 'result > 0',
      };

      await expect(workflow.addEdge(node1!.id, node2!.id, edge)).resolves.not.toThrow();
    });

    it('should reject edge with non-existent source node', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440150',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node);

      const edge = {
        fromNode: '550e8400-e29b-41d4-a716-446655440199', // Non-existent
        toNode: node!.id,
        condition: undefined,
      };

      await expect(workflow.addEdge('550e8400-e29b-41d4-a716-446655440199', node!.id, edge)).rejects.toThrow();
    });

    it('should reject edge with non-existent target node', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440160',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node);

      const edge = {
        fromNode: node!.id,
        toNode: '550e8400-e29b-41d4-a716-446655440199', // Non-existent
        condition: undefined,
      };

      await expect(workflow.addEdge(node!.id, '550e8400-e29b-41d4-a716-446655440199', edge)).rejects.toThrow();
    });
  });

  describe('validate()', () => {
    it('should validate an empty workflow', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const isValid = await workflow.validate();
      expect(isValid).toBe(true);
    });

    it('should validate a workflow with nodes but no edges', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node = {
        id: '550e8400-e29b-41d4-a716-446655440170',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node);
      const isValid = await workflow.validate();
      expect(isValid).toBe(true);
    });

    it('should validate a workflow with nodes and edges', async () => {
      const builder = new WorkflowBuilder('Test Workflow');
      const workflow = builder.build();

      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440180',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440181',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await workflow.addNode(node1);
      await workflow.addNode(node2);

      const edge = {
        fromNode: node1!.id,
        toNode: node2!.id,
        condition: undefined,
      };

      await workflow.addEdge(node1!.id, node2!.id, edge);

      const isValid = await workflow.validate();
      expect(isValid).toBe(true);
    });
  });
});

describe('WorkflowContext', () => {
  // Note: These tests create WorkflowContext but don't execute workflows
  // Full execution tests are in integration tests

  it('should create workflow context from executor result', async () => {
    // WorkflowContext is created by executing a workflow
    // We can't test it directly without execution, but we can verify
    // that the type exists and is exported
    const { WorkflowContext } = await import('../../index');
    expect(WorkflowContext).toBeDefined();
  });

  it('should have workflow context methods available', async () => {
    // Verify that WorkflowContext class exists and has expected structure
    const { WorkflowContext } = await import('../../index');
    expect(WorkflowContext).toBeDefined();
    expect(typeof WorkflowContext).toBe('function');
  });
});

describe('Executor', () => {
  it('should create an executor with test config', () => {
    const llmConfig = LlmConfig.ollama({
      model: 'llama2',
      baseUrl: 'http://localhost:11434',
    });

    const executor = new Executor(llmConfig, {
      timeoutSeconds: 60,
      debug: false,
    });

    expect(executor).toBeDefined();
  });

  it('should create executor with different configurations', () => {
    // Test with Ollama
    const ollamaConfig = LlmConfig.ollama({
      model: 'llama2',
    });
    const executor1 = new Executor(ollamaConfig);
    expect(executor1).toBeDefined();

    // Test with OpenAI (fake key)
    const openaiConfig = LlmConfig.openai({
      apiKey: 'sk-test-key-not-real',
      model: 'gpt-4o-mini',
    });
    const executor2 = new Executor(openaiConfig, {
      timeoutSeconds: 30,
    });
    expect(executor2).toBeDefined();

    // Test with Anthropic (fake key)
    const anthropicConfig = LlmConfig.anthropic({
      apiKey: 'sk-ant-test-key-not-real',
      model: 'claude-3-5-sonnet-20241022',
    });
    const executor3 = new Executor(anthropicConfig, {
      debug: true,
    });
    expect(executor3).toBeDefined();
  });

  it('should accept executor configuration options', () => {
    const llmConfig = LlmConfig.ollama({ model: 'llama2' });

    // Test with various config options
    const executor1 = new Executor(llmConfig, {
      timeoutSeconds: 120,
    });
    expect(executor1).toBeDefined();

    const executor2 = new Executor(llmConfig, {
      debug: true,
    });
    expect(executor2).toBeDefined();

    const executor3 = new Executor(llmConfig, {
      timeoutSeconds: 30,
      debug: false,
    });
    expect(executor3).toBeDefined();
  });

  it('should create executor and workflow for integration', () => {
    const llmConfig = LlmConfig.ollama({ model: 'llama2' });
    const executor = new Executor(llmConfig);

    const workflow = new WorkflowBuilder('Test Workflow')
      .description('Integration test workflow')
      .build();

    expect(executor).toBeDefined();
    expect(workflow).toBeDefined();

    // Note: Actual execution requires a running LLM service
    // and is tested in integration tests
  });
});
