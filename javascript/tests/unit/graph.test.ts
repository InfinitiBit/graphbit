import { describe, it, expect, beforeEach } from 'vitest';
import { WorkflowGraph } from '../../index';

describe('WorkflowGraph', () => {
  let graph: WorkflowGraph;

  beforeEach(() => {
    graph = new WorkflowGraph();
  });

  describe('constructor', () => {
    it('should create a new workflow graph', () => {
      expect(graph).toBeDefined();
    });

    it('should start with zero nodes', async () => {
      const count = await graph.nodeCount();
      expect(count).toBe(0);
    });

    it('should start with zero edges', async () => {
      const count = await graph.edgeCount();
      expect(count).toBe(0);
    });

    it('should be empty initially', async () => {
      const isEmpty = await graph.isEmpty();
      expect(isEmpty).toBe(true);
    });
  });

  describe('addNode()', () => {
    it('should add an Agent node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440000',
        name: 'Test Agent',
        description: 'A test agent node',
        nodeType: 'Agent',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);

      const count = await graph.nodeCount();
      expect(count).toBe(1);
    });

    it('should add a Condition node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440001',
        name: 'Test Condition',
        description: 'A test condition node',
        nodeType: 'Condition',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);
    });

    it('should add a Transform node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440002',
        name: 'Test Transform',
        description: 'A test transform node',
        nodeType: 'Transform',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);
    });

    it('should add a Split node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440003',
        name: 'Test Split',
        description: 'A test split node',
        nodeType: 'Split',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);
    });

    it('should add a Join node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440004',
        name: 'Test Join',
        description: 'A test join node',
        nodeType: 'Join',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);
    });

    it('should add a Delay node to the graph', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440005',
        name: 'Test Delay',
        description: 'A test delay node',
        nodeType: 'Delay',
      };

      const nodeId = await graph.addNode(node);
      expect(nodeId).toBe(node.id);
    });

    it('should increment node count when adding nodes', async () => {
      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440010',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440011',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await graph.addNode(node1);
      let count = await graph.nodeCount();
      expect(count).toBe(1);

      await graph.addNode(node2);
      count = await graph.nodeCount();
      expect(count).toBe(2);
    });

    it('should reject invalid node type', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440020',
        name: 'Invalid Node',
        description: 'A node with invalid type',
        nodeType: 'InvalidType',
      };

      await expect(graph.addNode(node)).rejects.toThrow();
    });
  });

  describe('addEdge()', () => {
    it('should add an edge between two nodes', async () => {
      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440030',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440031',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await graph.addNode(node1);
      await graph.addNode(node2);

      const edge = {
        fromNode: node1.id,
        toNode: node2.id,
        condition: undefined,
      };

      await graph.addEdge(edge);

      const edgeCount = await graph.edgeCount();
      expect(edgeCount).toBe(1);
    });

    it('should add an edge with a condition', async () => {
      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440040',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440041',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await graph.addNode(node1);
      await graph.addNode(node2);

      const edge = {
        fromNode: node1.id,
        toNode: node2.id,
        condition: 'result > 0',
      };

      await graph.addEdge(edge);

      const edgeCount = await graph.edgeCount();
      expect(edgeCount).toBe(1);
    });

    it('should increment edge count when adding edges', async () => {
      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440050',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440051',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      const node3 = {
        id: '550e8400-e29b-41d4-a716-446655440052',
        name: 'Node 3',
        description: 'Third node',
        nodeType: 'Agent',
      };

      await graph.addNode(node1);
      await graph.addNode(node2);
      await graph.addNode(node3);

      const edge1 = {
        fromNode: node1.id,
        toNode: node2.id,
        condition: undefined,
      };

      const edge2 = {
        fromNode: node2.id,
        toNode: node3.id,
        condition: undefined,
      };

      await graph.addEdge(edge1);
      let count = await graph.edgeCount();
      expect(count).toBe(1);

      await graph.addEdge(edge2);
      count = await graph.edgeCount();
      expect(count).toBe(2);
    });

    it('should reject edge with non-existent source node', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440060',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await graph.addNode(node);

      const edge = {
        fromNode: '550e8400-e29b-41d4-a716-446655440099', // Non-existent
        toNode: node.id,
        condition: undefined,
      };

      await expect(graph.addEdge(edge)).rejects.toThrow();
    });

    it('should reject edge with non-existent target node', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440070',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await graph.addNode(node);

      const edge = {
        fromNode: node.id,
        toNode: '550e8400-e29b-41d4-a716-446655440099', // Non-existent
        condition: undefined,
      };

      await expect(graph.addEdge(edge)).rejects.toThrow();
    });

    it('should reject edge with invalid node ID format', async () => {
      const edge = {
        fromNode: 'invalid-id',
        toNode: 'also-invalid',
        condition: undefined,
      };

      await expect(graph.addEdge(edge)).rejects.toThrow();
    });
  });

  describe('validate()', () => {
    it('should validate an empty graph', async () => {
      const isValid = await graph.validate();
      expect(isValid).toBe(true);
    });

    it('should validate a graph with nodes but no edges', async () => {
      const node = {
        id: '550e8400-e29b-41d4-a716-446655440080',
        name: 'Node',
        description: 'A node',
        nodeType: 'Agent',
      };

      await graph.addNode(node);
      const isValid = await graph.validate();
      expect(isValid).toBe(true);
    });

    it('should validate a graph with nodes and edges', async () => {
      const node1 = {
        id: '550e8400-e29b-41d4-a716-446655440090',
        name: 'Node 1',
        description: 'First node',
        nodeType: 'Agent',
      };

      const node2 = {
        id: '550e8400-e29b-41d4-a716-446655440091',
        name: 'Node 2',
        description: 'Second node',
        nodeType: 'Agent',
      };

      await graph.addNode(node1);
      await graph.addNode(node2);

      const edge = {
        fromNode: node1.id,
        toNode: node2.id,
        condition: undefined,
      };

      await graph.addEdge(edge);

      const isValid = await graph.validate();
      expect(isValid).toBe(true);
    });
  });
});

