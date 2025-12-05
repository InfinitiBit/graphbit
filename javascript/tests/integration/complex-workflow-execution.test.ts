/**
 * Complex Workflow Execution Integration Tests
 * 
 * Comprehensive tests for complex workflow patterns including conditional branching,
 * parallel execution, error handling, timeout management, and state management.
 * 
 * These tests validate workflow graph construction and configuration without requiring
 * actual LLM execution, focusing on the structural integrity and configuration of
 * complex workflow patterns.
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { init, WorkflowGraph, WorkflowBuilder, Executor, LlmConfig } from '../../index';
import { randomUUID } from 'crypto';
import { createTestLlmConfig } from '../helpers/test-llm-config';

// Initialize GraphBit before running tests
beforeAll(async () => {
  await init();
});

describe('Complex Workflow Execution Integration Tests', () => {
  describe('Conditional Branching Workflows', () => {
    it('should create workflow with if-then-else conditional logic', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for conditional workflow
      const inputNode = { id: randomUUID(), name: 'Input', description: 'Input node', nodeType: 'Agent' };
      const conditionNode = { id: randomUUID(), name: 'Condition', description: 'Check condition', nodeType: 'Condition' };
      const thenNode = { id: randomUUID(), name: 'ThenBranch', description: 'Execute if true', nodeType: 'Agent' };
      const elseNode = { id: randomUUID(), name: 'ElseBranch', description: 'Execute if false', nodeType: 'Agent' };
      const mergeNode = { id: randomUUID(), name: 'Merge', description: 'Merge results', nodeType: 'Join' };

      // Add all nodes
      await graph.addNode(inputNode);
      await graph.addNode(conditionNode);
      await graph.addNode(thenNode);
      await graph.addNode(elseNode);
      await graph.addNode(mergeNode);

      // Create conditional edges
      await graph.addEdge({ fromNode: inputNode.id, toNode: conditionNode.id });
      await graph.addEdge({ fromNode: conditionNode.id, toNode: thenNode.id, condition: 'result == true' });
      await graph.addEdge({ fromNode: conditionNode.id, toNode: elseNode.id, condition: 'result == false' });
      await graph.addEdge({ fromNode: thenNode.id, toNode: mergeNode.id });
      await graph.addEdge({ fromNode: elseNode.id, toNode: mergeNode.id });

      // Validate graph structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(5);
      expect(inputNode.id).toBeDefined();
      expect(conditionNode.nodeType).toBe('Condition');
      expect(mergeNode.nodeType).toBe('Join');
    });

    it('should create workflow with multiple conditional branches', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for multi-branch conditional
      const inputNode = { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' };
      const conditionNode = { id: randomUUID(), name: 'Condition', description: 'Multi-way condition', nodeType: 'Condition' };
      const branch1 = { id: randomUUID(), name: 'Branch1', description: 'Priority high', nodeType: 'Agent' };
      const branch2 = { id: randomUUID(), name: 'Branch2', description: 'Priority medium', nodeType: 'Agent' };
      const branch3 = { id: randomUUID(), name: 'Branch3', description: 'Priority low', nodeType: 'Agent' };
      const outputNode = { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(inputNode);
      await graph.addNode(conditionNode);
      await graph.addNode(branch1);
      await graph.addNode(branch2);
      await graph.addNode(branch3);
      await graph.addNode(outputNode);

      // Create conditional edges with different conditions
      await graph.addEdge({ fromNode: inputNode.id, toNode: conditionNode.id });
      await graph.addEdge({ fromNode: conditionNode.id, toNode: branch1.id, condition: 'priority == "high"' });
      await graph.addEdge({ fromNode: conditionNode.id, toNode: branch2.id, condition: 'priority == "medium"' });
      await graph.addEdge({ fromNode: conditionNode.id, toNode: branch3.id, condition: 'priority == "low"' });
      await graph.addEdge({ fromNode: branch1.id, toNode: outputNode.id });
      await graph.addEdge({ fromNode: branch2.id, toNode: outputNode.id });
      await graph.addEdge({ fromNode: branch3.id, toNode: outputNode.id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(7);
      expect(conditionNode.nodeType).toBe('Condition');
    });

    it('should create workflow with nested conditional logic', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for nested conditionals
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' },
        { id: randomUUID(), name: 'OuterCondition', description: 'Outer check', nodeType: 'Condition' },
        { id: randomUUID(), name: 'InnerCondition1', description: 'Inner check 1', nodeType: 'Condition' },
        { id: randomUUID(), name: 'InnerCondition2', description: 'Inner check 2', nodeType: 'Condition' },
        { id: randomUUID(), name: 'Action1', description: 'Action 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Action2', description: 'Action 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Action3', description: 'Action 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      // Add all nodes
      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create nested conditional structure
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id }); // Input -> OuterCondition
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id, condition: 'type == "A"' }); // OuterCondition -> InnerCondition1
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id, condition: 'type == "B"' }); // OuterCondition -> InnerCondition2
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'subtype == "1"' }); // InnerCondition1 -> Action1
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[5].id, condition: 'subtype == "2"' }); // InnerCondition1 -> Action2
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[6].id }); // InnerCondition2 -> Action3
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[7].id }); // Action1 -> Output
      await graph.addEdge({ fromNode: nodes[5].id, toNode: nodes[7].id }); // Action2 -> Output
      await graph.addEdge({ fromNode: nodes[6].id, toNode: nodes[7].id }); // Action3 -> Output

      // Validate nested structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(8);
      expect(edgeCount).toBe(9);
      expect(nodes[1].nodeType).toBe('Condition');
      expect(nodes[2].nodeType).toBe('Condition');
      expect(nodes[3].nodeType).toBe('Condition');
    });

    it('should create workflow with dynamic routing based on state', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for dynamic routing
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Router', description: 'Route based on state', nodeType: 'Condition' },
        { id: randomUUID(), name: 'PathA', description: 'Path A', nodeType: 'Agent' },
        { id: randomUUID(), name: 'PathB', description: 'Path B', nodeType: 'Agent' },
        { id: randomUUID(), name: 'PathC', description: 'Path C', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create dynamic routing edges
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id, condition: 'state.route == "A"' });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id, condition: 'state.route == "B"' });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[4].id, condition: 'state.route == "C"' });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4);
      expect(nodes[1].nodeType).toBe('Condition');
    });

    it('should validate correct branch execution paths', async () => {
      const graph = new WorkflowGraph();

      // Create simple conditional workflow
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Check', description: 'Check', nodeType: 'Condition' },
        { id: randomUUID(), name: 'Success', description: 'Success path', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Failure', description: 'Failure path', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id, condition: 'success == true' });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id, condition: 'success == false' });

      // Validate paths exist
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3);
      expect(nodes[0].name).toBe('Start');
      expect(nodes[2].name).toBe('Success');
      expect(nodes[3].name).toBe('Failure');
    });
  });

  describe('Parallel Execution Workflows', () => {
    it('should create workflow with multiple parallel execution paths', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for parallel execution
      const inputNode = { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' };
      const splitNode = { id: randomUUID(), name: 'Split', description: 'Split execution', nodeType: 'Split' };
      const parallel1 = { id: randomUUID(), name: 'Parallel1', description: 'Parallel task 1', nodeType: 'Agent' };
      const parallel2 = { id: randomUUID(), name: 'Parallel2', description: 'Parallel task 2', nodeType: 'Agent' };
      const parallel3 = { id: randomUUID(), name: 'Parallel3', description: 'Parallel task 3', nodeType: 'Agent' };
      const joinNode = { id: randomUUID(), name: 'Join', description: 'Join results', nodeType: 'Join' };
      const outputNode = { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(inputNode);
      await graph.addNode(splitNode);
      await graph.addNode(parallel1);
      await graph.addNode(parallel2);
      await graph.addNode(parallel3);
      await graph.addNode(joinNode);
      await graph.addNode(outputNode);

      // Create parallel structure
      await graph.addEdge({ fromNode: inputNode.id, toNode: splitNode.id });
      await graph.addEdge({ fromNode: splitNode.id, toNode: parallel1.id });
      await graph.addEdge({ fromNode: splitNode.id, toNode: parallel2.id });
      await graph.addEdge({ fromNode: splitNode.id, toNode: parallel3.id });
      await graph.addEdge({ fromNode: parallel1.id, toNode: joinNode.id });
      await graph.addEdge({ fromNode: parallel2.id, toNode: joinNode.id });
      await graph.addEdge({ fromNode: parallel3.id, toNode: joinNode.id });
      await graph.addEdge({ fromNode: joinNode.id, toNode: outputNode.id });

      // Validate parallel structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(7);
      expect(edgeCount).toBe(8);
      expect(splitNode.nodeType).toBe('Split');
      expect(joinNode.nodeType).toBe('Join');
    });

    it('should create workflow with concurrent agent execution', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for concurrent execution
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Agent1', description: 'Concurrent agent 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Agent2', description: 'Concurrent agent 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Agent3', description: 'Concurrent agent 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge', description: 'Merge', nodeType: 'Join' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create concurrent execution pattern
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(6);
      expect(nodes[4].nodeType).toBe('Join');
    });

    it('should create workflow with parallel branches that merge', async () => {
      const graph = new WorkflowGraph();

      // Create diamond pattern (parallel with merge)
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' },
        { id: randomUUID(), name: 'BranchA', description: 'Branch A', nodeType: 'Agent' },
        { id: randomUUID(), name: 'BranchB', description: 'Branch B', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge', description: 'Merge point', nodeType: 'Join' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(4);
      expect(nodes[3].nodeType).toBe('Join');
    });

    it('should handle synchronization in parallel workflows', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with synchronization point
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Task1', description: 'Task 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Task2', description: 'Task 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Sync', description: 'Synchronization point', nodeType: 'Join' },
        { id: randomUUID(), name: 'Continue', description: 'Continue after sync', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(5);
      expect(nodes[3].nodeType).toBe('Join');
      expect(nodes[3].name).toBe('Sync');
    });

    it('should validate parallel execution completion', async () => {
      const graph = new WorkflowGraph();

      // Create parallel workflow with validation
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ParallelA', description: 'Parallel A', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ParallelB', description: 'Parallel B', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Join', description: 'Join', nodeType: 'Join' },
        { id: randomUUID(), name: 'Validate', description: 'Validate completion', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(5);
      expect(nodes[4].name).toBe('Validate');
    });
  });

  describe('Error Recovery and Handling', () => {
    it('should create workflow with error handling nodes', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with error handling
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'RiskyOperation', description: 'Operation that may fail', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ErrorCheck', description: 'Check for errors', nodeType: 'Condition' },
        { id: randomUUID(), name: 'ErrorHandler', description: 'Handle error', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Success', description: 'Success path', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'error != null' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'error == null' });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(6);
      expect(nodes[2].nodeType).toBe('Condition');
      expect(nodes[3].name).toBe('ErrorHandler');
    });

    it('should create workflow with retry logic', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with retry pattern
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Attempt', description: 'Attempt operation', nodeType: 'Agent' },
        { id: randomUUID(), name: 'CheckSuccess', description: 'Check if successful', nodeType: 'Condition' },
        { id: randomUUID(), name: 'Retry', description: 'Retry logic', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Success', description: 'Success', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'success == true' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'success == false && retries < 3' });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[1].id }); // Retry loop

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(5);
      expect(nodes[3].name).toBe('Retry');
    });

    it('should create workflow with fallback paths', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with fallback
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'PrimaryPath', description: 'Primary operation', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Check', description: 'Check result', nodeType: 'Condition' },
        { id: randomUUID(), name: 'FallbackPath', description: 'Fallback operation', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'success == true' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'success == false' });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(5);
      expect(nodes[3].name).toBe('FallbackPath');
    });

    it('should handle error propagation through workflow', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with error propagation
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Step1', description: 'Step 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Step2', description: 'Step 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ErrorHandler', description: 'Global error handler', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'error != null' });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3);
      expect(nodes[3].name).toBe('ErrorHandler');
    });

    it('should create graceful degradation workflow', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with graceful degradation
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'FullFeature', description: 'Full feature', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Check', description: 'Check availability', nodeType: 'Condition' },
        { id: randomUUID(), name: 'ReducedFeature', description: 'Reduced feature', nodeType: 'Agent' },
        { id: randomUUID(), name: 'MinimalFeature', description: 'Minimal feature', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[1].id, condition: 'available == "full"' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'available == "partial"' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'available == "minimal"' });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(7);
      expect(nodes[4].name).toBe('MinimalFeature');
    });
  });

  describe('Timeout Handling', () => {
    it('should create executor with timeout constraints', () => {
      const llmConfig = createTestLlmConfig();

      const executor = new Executor(llmConfig, {
        timeoutSeconds: 30,
        debug: false,
      });

      expect(executor).toBeDefined();
      expect(executor).toBeInstanceOf(Executor);
    });

    it('should create executor with different timeout values', () => {
      const llmConfig = createTestLlmConfig();

      // Short timeout
      const shortTimeout = new Executor(llmConfig, { timeoutSeconds: 10 });
      expect(shortTimeout).toBeDefined();

      // Medium timeout
      const mediumTimeout = new Executor(llmConfig, { timeoutSeconds: 60 });
      expect(mediumTimeout).toBeDefined();

      // Long timeout
      const longTimeout = new Executor(llmConfig, { timeoutSeconds: 300 });
      expect(longTimeout).toBeDefined();
    });

    it('should create workflow with delay nodes for timeout testing', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with delay nodes
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Delay1', description: 'Delay 1 second', nodeType: 'Delay' },
        { id: randomUUID(), name: 'Process', description: 'Process', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Delay2', description: 'Delay 2 seconds', nodeType: 'Delay' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4);
      expect(nodes[1].nodeType).toBe('Delay');
      expect(nodes[3].nodeType).toBe('Delay');
    });

    it('should create workflow with timeout recovery mechanism', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with timeout recovery
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'LongRunning', description: 'Long running task', nodeType: 'Agent' },
        { id: randomUUID(), name: 'TimeoutCheck', description: 'Check timeout', nodeType: 'Condition' },
        { id: randomUUID(), name: 'TimeoutHandler', description: 'Handle timeout', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Success', description: 'Success', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'timeout == true' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id, condition: 'timeout == false' });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4);
      expect(nodes[3].name).toBe('TimeoutHandler');
    });
  });

  describe('State Management', () => {
    it('should create workflow with state passing between nodes', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with state flow
      const nodes = [
        { id: randomUUID(), name: 'InitState', description: 'Initialize state', nodeType: 'Agent' },
        { id: randomUUID(), name: 'UpdateState1', description: 'Update state 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'UpdateState2', description: 'Update state 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'FinalState', description: 'Finalize state', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3);
      expect(nodes[0].name).toBe('InitState');
      expect(nodes[3].name).toBe('FinalState');
    });

    it('should create workflow with complex state transformations', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with transform nodes
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Transform1', description: 'Transform to uppercase', nodeType: 'Transform' },
        { id: randomUUID(), name: 'Transform2', description: 'Transform to JSON', nodeType: 'Transform' },
        { id: randomUUID(), name: 'Transform3', description: 'Transform to array', nodeType: 'Transform' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4);
      expect(nodes[1].nodeType).toBe('Transform');
      expect(nodes[2].nodeType).toBe('Transform');
      expect(nodes[3].nodeType).toBe('Transform');
    });

    it('should create workflow with state validation at different stages', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with validation checkpoints
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Process1', description: 'Process 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Validate1', description: 'Validate state 1', nodeType: 'Condition' },
        { id: randomUUID(), name: 'Process2', description: 'Process 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Validate2', description: 'Validate state 2', nodeType: 'Condition' },
        { id: randomUUID(), name: 'Output', description: 'Output', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id, condition: 'valid == true' });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id, condition: 'valid == true' });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(5);
      expect(nodes[2].nodeType).toBe('Condition');
      expect(nodes[4].nodeType).toBe('Condition');
    });

    it('should create stateful workflow with multiple execution paths', async () => {
      const graph = new WorkflowGraph();

      // Create workflow with state-dependent routing
      const nodes = [
        { id: randomUUID(), name: 'InitState', description: 'Initialize state', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Router', description: 'Route based on state', nodeType: 'Condition' },
        { id: randomUUID(), name: 'PathA', description: 'Path A updates state', nodeType: 'Agent' },
        { id: randomUUID(), name: 'PathB', description: 'Path B updates state', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge', description: 'Merge states', nodeType: 'Join' },
        { id: randomUUID(), name: 'FinalState', description: 'Final state', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id, condition: 'state.type == "A"' });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id, condition: 'state.type == "B"' });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(6);
      expect(nodes[4].nodeType).toBe('Join');
    });

    it('should create workflow with state accumulation pattern', async () => {
      const graph = new WorkflowGraph();

      // Create workflow that accumulates state
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start with empty state', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Accumulate1', description: 'Add data 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Accumulate2', description: 'Add data 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Accumulate3', description: 'Add data 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Finalize', description: 'Finalize accumulated state', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4);
      expect(nodes[4].name).toBe('Finalize');
    });
  });
});

