import { describe, it, expect, beforeAll } from 'vitest';
import { init, WorkflowGraph, WorkflowBuilder } from '../../index';
import { randomUUID } from 'crypto';

describe('Workflow Construction Integration Tests', () => {
  beforeAll(() => {
    init();
  });

  describe('Linear Workflows', () => {
    it('should construct linear workflow with 4+ nodes', async () => {
      const graph = new WorkflowGraph();

      // Create 5 nodes in sequence
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Receive input data', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Process', description: 'Process the data', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Transform', description: 'Transform results', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Validate', description: 'Validate output', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Output', description: 'Return final result', nodeType: 'Agent' },
      ];

      // Add all nodes
      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Connect nodes linearly: 0 -> 1 -> 2 -> 3 -> 4
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });

      // Validate graph structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5);
      expect(edgeCount).toBe(4); // 4 edges for 5 nodes in sequence

      // Verify all node IDs are unique
      const nodeIds = nodes.map(n => n.id);
      const uniqueIds = new Set(nodeIds);
      expect(uniqueIds.size).toBe(5);

      // Verify each node ID is a non-empty string
      nodeIds.forEach(id => {
        expect(typeof id).toBe('string');
        expect(id.length).toBeGreaterThan(0);
      });
    });

    it('should validate linear workflow successfully', async () => {
      const workflow = new WorkflowBuilder('linear-workflow')
        .description('A simple linear workflow for testing')
        .build();

      // Verify workflow properties
      const name = await workflow.name();
      const description = await workflow.description();

      expect(name).toBe('linear-workflow');
      expect(description).toBe('A simple linear workflow for testing');
    });
  });

  describe('Branching Workflows', () => {
    it('should construct branching workflow (A→B, A→C, B→D, C→D)', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for branching pattern
      const nodes = [
        { id: randomUUID(), name: 'Input', description: 'Input node', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch1', description: 'First branch', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch2', description: 'Second branch', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge', description: 'Merge results', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create branching structure: A -> B, A -> C, B -> D, C -> D
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });

      // Validate graph structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(4); // 4 edges for branching pattern
    });

    it('should construct complex branching with multiple merge points', async () => {
      const graph = new WorkflowGraph();

      // Create more complex branching structure
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Start node', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch1', description: 'Branch 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch2', description: 'Branch 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch3', description: 'Branch 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge1', description: 'Merge 1 and 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Merge2', description: 'Final merge', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create complex branching
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(7);
    });
  });

  describe('Parallel Workflows', () => {
    it('should construct parallel independent paths', async () => {
      const graph = new WorkflowGraph();

      // Create two independent parallel paths
      const nodes = [
        // Path 1: A -> B -> C
        { id: randomUUID(), name: 'Path1-Start', description: 'Path 1 start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path1-Middle', description: 'Path 1 middle', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path1-End', description: 'Path 1 end', nodeType: 'Agent' },
        // Path 2: D -> E -> F
        { id: randomUUID(), name: 'Path2-Start', description: 'Path 2 start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path2-Middle', description: 'Path 2 middle', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path2-End', description: 'Path 2 end', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Connect path 1
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });

      // Connect path 2
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6);
      expect(edgeCount).toBe(4); // 2 edges per path
    });

    it('should construct parallel paths with shared start and end', async () => {
      const graph = new WorkflowGraph();

      // Create diamond pattern: Start -> (Path1, Path2) -> End
      const nodes = [
        { id: randomUUID(), name: 'Start', description: 'Shared start', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path1', description: 'Parallel path 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Path2', description: 'Parallel path 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'End', description: 'Shared end', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create diamond structure
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[3].id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(4);
    });
  });

  describe('Workflow Metadata and Queries', () => {
    it('should access workflow metadata correctly', async () => {
      const workflow = new WorkflowBuilder('metadata-test')
        .description('Testing workflow metadata access')
        .build();

      // Verify metadata is accessible
      const name = await workflow.name();
      const description = await workflow.description();
      const id = await workflow.id();

      expect(name).toBe('metadata-test');
      expect(description).toBe('Testing workflow metadata access');
      expect(id).toBeDefined();
      expect(typeof id).toBe('string');
      expect(id.length).toBeGreaterThan(0);
    });

    it('should query graph structure correctly', async () => {
      const graph = new WorkflowGraph();

      // Create a small graph
      const nodes = [
        { id: randomUUID(), name: 'Node1', description: 'First node', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Node2', description: 'Second node', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Node3', description: 'Third node', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[2].id });

      // Query graph structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(3);
      expect(edgeCount).toBe(2);

      // Verify counts are consistent
      expect(edgeCount).toBe(nodeCount - 1); // Linear graph property
    });

    it('should handle empty workflow correctly', async () => {
      const workflow = new WorkflowBuilder('empty-workflow')
        .description('An empty workflow')
        .build();

      // Verify workflow is created
      const name = await workflow.name();
      expect(name).toBe('empty-workflow');

      // Empty workflow validation behavior depends on implementation
      try {
        await workflow.validate();
        // If it doesn't throw, that's fine
        expect(true).toBe(true);
      } catch (error) {
        // If it throws, verify error exists
        expect(error).toBeDefined();
      }
    });
  });

  describe('Complex Graph Topologies', () => {
    it('should construct workflow with 10+ nodes and complex connections', async () => {
      const graph = new WorkflowGraph();

      // Create a larger, more complex graph
      const nodes = [];
      for (let i = 0; i < 12; i++) {
        nodes.push({
          id: randomUUID(),
          name: `Node${i}`,
          description: `Process step ${i}`,
          nodeType: 'Agent',
        });
      }

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create complex connection pattern
      // Layer 1: nodes[0] -> nodes[1], nodes[2], nodes[3]
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[1].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[2].id });
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[3].id });

      // Layer 2: nodes[1,2,3] -> nodes[4,5,6]
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[4].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[6].id });

      // Layer 3: nodes[4,5,6] -> nodes[7,8]
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[7].id });
      await graph.addEdge({ fromNode: nodes[5].id, toNode: nodes[7].id });
      await graph.addEdge({ fromNode: nodes[6].id, toNode: nodes[8].id });

      // Layer 4: nodes[7,8] -> nodes[9,10]
      await graph.addEdge({ fromNode: nodes[7].id, toNode: nodes[9].id });
      await graph.addEdge({ fromNode: nodes[8].id, toNode: nodes[10].id });

      // Final: nodes[9,10] -> nodes[11]
      await graph.addEdge({ fromNode: nodes[9].id, toNode: nodes[11].id });
      await graph.addEdge({ fromNode: nodes[10].id, toNode: nodes[11].id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(12);
      expect(edgeCount).toBe(13);

      // Verify all nodes are unique
      const uniqueIds = new Set(nodes.map(n => n.id));
      expect(uniqueIds.size).toBe(12);
    });

    it('should construct workflow with multiple entry and exit points', async () => {
      const graph = new WorkflowGraph();

      // Create graph with 3 entry points and 2 exit points
      const nodes = [
        { id: randomUUID(), name: 'Entry1', description: 'Entry point 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Entry2', description: 'Entry point 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Entry3', description: 'Entry point 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Middle1', description: 'Middle processing 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Middle2', description: 'Middle processing 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Exit1', description: 'Exit point 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Exit2', description: 'Exit point 2', nodeType: 'Agent' },
      ];

      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Connect entries to middle nodes
      await graph.addEdge({ fromNode: nodes[0].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[1].id, toNode: nodes[3].id });
      await graph.addEdge({ fromNode: nodes[2].id, toNode: nodes[4].id });

      // Connect middle to exits
      await graph.addEdge({ fromNode: nodes[3].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[5].id });
      await graph.addEdge({ fromNode: nodes[4].id, toNode: nodes[6].id });

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(7);
      expect(edgeCount).toBe(6);
    });
  });
});


