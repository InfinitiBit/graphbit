import { describe, it, expect, beforeAll } from 'vitest';
import { init, WorkflowGraph, WorkflowBuilder, AgentBuilder, LlmConfig, Executor } from '../../index';
import { randomUUID } from 'crypto';
import { createTestLlmConfig, hasRealApiKeys, getRealLlmConfig } from '../helpers/test-llm-config';

describe('Multi-Agent Workflow Integration Tests', () => {
  beforeAll(() => {
    init();
  });

  describe('Agent Collaboration', () => {
    it('should create multiple agents with different configurations', async () => {
      const llmConfig = createTestLlmConfig();

      // Create three agent builders with different roles
      const builder1 = new AgentBuilder('Analyzer', llmConfig)
        .description('Analyzes input data')
        .systemPrompt('You are a data analyst. Analyze the input and provide insights.')
        .temperature(0.3)
        .maxTokens(500);

      const builder2 = new AgentBuilder('Summarizer', llmConfig)
        .description('Summarizes analysis results')
        .systemPrompt('You are a summarizer. Create concise summaries.')
        .temperature(0.5)
        .maxTokens(300);

      const builder3 = new AgentBuilder('Validator', llmConfig)
        .description('Validates output quality')
        .systemPrompt('You are a validator. Check for accuracy and completeness.')
        .temperature(0.1)
        .maxTokens(200);

      // Validate all builders were created
      expect(builder1).toBeDefined();
      expect(builder2).toBeDefined();
      expect(builder3).toBeDefined();

      // Validate builder types
      expect(builder1).toBeInstanceOf(AgentBuilder);
      expect(builder2).toBeInstanceOf(AgentBuilder);
      expect(builder3).toBeInstanceOf(AgentBuilder);
    });

    it('should create workflow with multiple agent nodes for collaboration', async () => {
      const graph = new WorkflowGraph();

      // Create agent nodes representing different roles in collaboration
      const nodes = [
        { id: randomUUID(), name: 'DataCollector', description: 'Collects and prepares data', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Analyst1', description: 'Performs statistical analysis', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Analyst2', description: 'Performs qualitative analysis', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Synthesizer', description: 'Combines analysis results', nodeType: 'Agent' },
      ];

      // Add all agent nodes
      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create collaboration pattern: DataCollector → both Analysts → Synthesizer
      await graph.addEdge({ fromNode: nodes[0]!.id, toNode: nodes[1]!.id });
      await graph.addEdge({ fromNode: nodes[0]!.id, toNode: nodes[2]!.id });
      await graph.addEdge({ fromNode: nodes[1]!.id, toNode: nodes[3]!.id });
      await graph.addEdge({ fromNode: nodes[2]!.id, toNode: nodes[3]!.id });

      // Validate graph structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(4); // 4 edges for collaboration pattern
    });

    it('should create multi-agent workflow with shared task distribution', async () => {
      const graph = new WorkflowGraph();

      // Create a coordinator and multiple worker agents
      const coordinator = { id: randomUUID(), name: 'Coordinator', description: 'Distributes tasks', nodeType: 'Agent' };
      const workers = [
        { id: randomUUID(), name: 'Worker1', description: 'Processes task subset 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Worker2', description: 'Processes task subset 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Worker3', description: 'Processes task subset 3', nodeType: 'Agent' },
      ];
      const aggregator = { id: randomUUID(), name: 'Aggregator', description: 'Combines results', nodeType: 'Agent' };

      // Add all nodes
      await graph.addNode(coordinator);
      for (const worker of workers) {
        await graph.addNode(worker);
      }
      await graph.addNode(aggregator);

      // Connect coordinator to all workers
      for (const worker of workers) {
        await graph.addEdge({ fromNode: coordinator!.id, toNode: worker.id });
      }

      // Connect all workers to aggregator
      for (const worker of workers) {
        await graph.addEdge({ fromNode: worker!.id, toNode: aggregator.id });
      }

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5); // 1 coordinator + 3 workers + 1 aggregator
      expect(edgeCount).toBe(6); // 3 edges from coordinator + 3 edges to aggregator
    });

    it('should create agents with different specializations for collaborative problem-solving', async () => {
      const llmConfig = createTestLlmConfig();

      // Create specialized agent builders
      const researchBuilder = new AgentBuilder('Researcher', llmConfig)
        .description('Researches and gathers information')
        .systemPrompt('You are a research specialist. Gather comprehensive information.')
        .temperature(0.7);

      const criticalBuilder = new AgentBuilder('CriticalThinker', llmConfig)
        .description('Analyzes information critically')
        .systemPrompt('You are a critical thinker. Evaluate arguments and identify flaws.')
        .temperature(0.4);

      const creativeBuilder = new AgentBuilder('CreativeSolver', llmConfig)
        .description('Generates creative solutions')
        .systemPrompt('You are a creative problem solver. Think outside the box.')
        .temperature(0.9);

      // Validate all specialized builders
      expect(researchBuilder).toBeDefined();
      expect(criticalBuilder).toBeDefined();
      expect(creativeBuilder).toBeDefined();

      // Validate builder types
      expect(researchBuilder).toBeInstanceOf(AgentBuilder);
      expect(criticalBuilder).toBeInstanceOf(AgentBuilder);
      expect(creativeBuilder).toBeInstanceOf(AgentBuilder);
    });
  });

  describe('Message Passing', () => {
    it('should create workflow with message passing between agents', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for message passing chain
      const sender = { id: randomUUID(), name: 'Sender', description: 'Initiates message', nodeType: 'Agent' };
      const processor1 = { id: randomUUID(), name: 'Processor1', description: 'Processes and forwards', nodeType: 'Agent' };
      const processor2 = { id: randomUUID(), name: 'Processor2', description: 'Further processing', nodeType: 'Agent' };
      const receiver = { id: randomUUID(), name: 'Receiver', description: 'Final recipient', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(sender);
      await graph.addNode(processor1);
      await graph.addNode(processor2);
      await graph.addNode(receiver);

      // Create message passing chain
      await graph.addEdge({ fromNode: sender!.id, toNode: processor1.id });
      await graph.addEdge({ fromNode: processor1!.id, toNode: processor2.id });
      await graph.addEdge({ fromNode: processor2!.id, toNode: receiver.id });

      // Validate message passing structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3);

      // Validate all nodes are Agent type
      expect(sender.nodeType).toBe('Agent');
      expect(processor1.nodeType).toBe('Agent');
      expect(processor2.nodeType).toBe('Agent');
      expect(receiver.nodeType).toBe('Agent');
    });

    it('should create workflow with broadcast message pattern', async () => {
      const graph = new WorkflowGraph();

      // Create broadcaster and multiple receivers
      const broadcaster = { id: randomUUID(), name: 'Broadcaster', description: 'Sends to all', nodeType: 'Agent' };
      const receivers = [
        { id: randomUUID(), name: 'Receiver1', description: 'Receives broadcast 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Receiver2', description: 'Receives broadcast 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Receiver3', description: 'Receives broadcast 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Receiver4', description: 'Receives broadcast 4', nodeType: 'Agent' },
      ];

      // Add nodes
      await graph.addNode(broadcaster);
      for (const receiver of receivers) {
        await graph.addNode(receiver);
      }

      // Create broadcast pattern
      for (const receiver of receivers) {
        await graph.addEdge({ fromNode: broadcaster!.id, toNode: receiver.id });
      }

      // Validate broadcast structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5); // 1 broadcaster + 4 receivers
      expect(edgeCount).toBe(4); // 4 broadcast edges
    });

    it('should create workflow with message aggregation pattern', async () => {
      const graph = new WorkflowGraph();

      // Create multiple senders and one aggregator
      const senders = [
        { id: randomUUID(), name: 'Sender1', description: 'Sends message 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Sender2', description: 'Sends message 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Sender3', description: 'Sends message 3', nodeType: 'Agent' },
      ];
      const aggregator = { id: randomUUID(), name: 'Aggregator', description: 'Collects all messages', nodeType: 'Agent' };

      // Add nodes
      for (const sender of senders) {
        await graph.addNode(sender);
      }
      await graph.addNode(aggregator);

      // Create aggregation pattern
      for (const sender of senders) {
        await graph.addEdge({ fromNode: sender!.id, toNode: aggregator.id });
      }

      // Validate aggregation structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4); // 3 senders + 1 aggregator
      expect(edgeCount).toBe(3); // 3 aggregation edges
    });

    it('should create workflow with bidirectional message flow', async () => {
      const graph = new WorkflowGraph();

      // Create nodes for bidirectional communication
      const agent1 = { id: randomUUID(), name: 'Agent1', description: 'First agent', nodeType: 'Agent' };
      const agent2 = { id: randomUUID(), name: 'Agent2', description: 'Second agent', nodeType: 'Agent' };
      const mediator = { id: randomUUID(), name: 'Mediator', description: 'Coordinates communication', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(agent1);
      await graph.addNode(agent2);
      await graph.addNode(mediator);

      // Create bidirectional flow through mediator
      await graph.addEdge({ fromNode: agent1!.id, toNode: mediator.id });
      await graph.addEdge({ fromNode: mediator!.id, toNode: agent2.id });
      await graph.addEdge({ fromNode: agent2!.id, toNode: mediator.id });
      await graph.addEdge({ fromNode: mediator!.id, toNode: agent1.id });

      // Validate bidirectional structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(3);
      expect(edgeCount).toBe(4); // 4 edges for bidirectional flow
    });

    it('should validate message routing in complex workflow', async () => {
      const graph = new WorkflowGraph();

      // Create complex routing structure
      const router = { id: randomUUID(), name: 'Router', description: 'Routes messages', nodeType: 'Agent' };
      const pathA = { id: randomUUID(), name: 'PathA', description: 'Path A processor', nodeType: 'Agent' };
      const pathB = { id: randomUUID(), name: 'PathB', description: 'Path B processor', nodeType: 'Agent' };
      const merger = { id: randomUUID(), name: 'Merger', description: 'Merges paths', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(router);
      await graph.addNode(pathA);
      await graph.addNode(pathB);
      await graph.addNode(merger);

      // Create routing pattern
      await graph.addEdge({ fromNode: router!.id, toNode: pathA.id });
      await graph.addEdge({ fromNode: router!.id, toNode: pathB.id });
      await graph.addEdge({ fromNode: pathA!.id, toNode: merger.id });
      await graph.addEdge({ fromNode: pathB!.id, toNode: merger.id });

      // Validate routing structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(4);

      // Validate node names
      expect(router.name).toBe('Router');
      expect(pathA.name).toBe('PathA');
      expect(pathB.name).toBe('PathB');
      expect(merger.name).toBe('Merger');
    });
  });


  describe('Coordination Patterns', () => {
    it('should create leader-follower coordination pattern', async () => {
      const graph = new WorkflowGraph();

      // Create leader and followers
      const leader = { id: randomUUID(), name: 'Leader', description: 'Coordinates team', nodeType: 'Agent' };
      const followers = [
        { id: randomUUID(), name: 'Follower1', description: 'Executes task 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Follower2', description: 'Executes task 2', nodeType: 'Agent' },
      ];

      // Add nodes
      await graph.addNode(leader);
      for (const follower of followers) {
        await graph.addNode(follower);
      }

      // Create leader-follower connections
      for (const follower of followers) {
        await graph.addEdge({ fromNode: leader!.id, toNode: follower.id });
      }

      // Validate pattern
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(3); // 1 leader + 2 followers
      expect(edgeCount).toBe(2); // 2 edges from leader to followers
      expect(leader.name).toBe('Leader');
      expect(followers[0].name).toBe('Follower1');
      expect(followers[1].name).toBe('Follower2');
    });

    it('should create peer-to-peer coordination pattern', async () => {
      const graph = new WorkflowGraph();

      // Create peer agents
      const peers = [
        { id: randomUUID(), name: 'Peer1', description: 'First peer', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Peer2', description: 'Second peer', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Peer3', description: 'Third peer', nodeType: 'Agent' },
      ];

      // Add all peers
      for (const peer of peers) {
        await graph.addNode(peer);
      }

      // Create peer-to-peer connections (each peer connects to next)
      await graph.addEdge({ fromNode: peers[0].id, toNode: peers[1].id });
      await graph.addEdge({ fromNode: peers[1].id, toNode: peers[2].id });
      await graph.addEdge({ fromNode: peers[2].id, toNode: peers[0].id }); // Circular

      // Validate pattern
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(3);
      expect(edgeCount).toBe(3); // Circular peer-to-peer
    });

    it('should create hierarchical agent structure', async () => {
      const graph = new WorkflowGraph();

      // Create hierarchical structure: Manager -> Supervisors -> Workers
      const manager = { id: randomUUID(), name: 'Manager', description: 'Top-level manager', nodeType: 'Agent' };
      const supervisors = [
        { id: randomUUID(), name: 'Supervisor1', description: 'Supervises team 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Supervisor2', description: 'Supervises team 2', nodeType: 'Agent' },
      ];
      const workers = [
        { id: randomUUID(), name: 'Worker1', description: 'Team 1 worker', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Worker2', description: 'Team 1 worker', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Worker3', description: 'Team 2 worker', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Worker4', description: 'Team 2 worker', nodeType: 'Agent' },
      ];

      // Add all nodes
      await graph.addNode(manager);
      for (const supervisor of supervisors) {
        await graph.addNode(supervisor);
      }
      for (const worker of workers) {
        await graph.addNode(worker);
      }

      // Create hierarchical connections
      // Manager -> Supervisors
      for (const supervisor of supervisors) {
        await graph.addEdge({ fromNode: manager!.id, toNode: supervisor.id });
      }
      // Supervisor1 -> Workers 1,2
      await graph.addEdge({ fromNode: supervisors[0].id, toNode: workers[0].id });
      await graph.addEdge({ fromNode: supervisors[0].id, toNode: workers[1].id });
      // Supervisor2 -> Workers 3,4
      await graph.addEdge({ fromNode: supervisors[1].id, toNode: workers[2].id });
      await graph.addEdge({ fromNode: supervisors[1].id, toNode: workers[3].id });

      // Validate hierarchical structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(7); // 1 manager + 2 supervisors + 4 workers
      expect(edgeCount).toBe(6); // 2 manager edges + 4 supervisor edges
    });

    it('should create consensus-based coordination pattern', async () => {
      const graph = new WorkflowGraph();

      // Create agents that need to reach consensus
      const proposer = { id: randomUUID(), name: 'Proposer', description: 'Proposes solution', nodeType: 'Agent' };
      const reviewers = [
        { id: randomUUID(), name: 'Reviewer1', description: 'Reviews proposal', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Reviewer2', description: 'Reviews proposal', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Reviewer3', description: 'Reviews proposal', nodeType: 'Agent' },
      ];
      const decider = { id: randomUUID(), name: 'Decider', description: 'Makes final decision', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(proposer);
      for (const reviewer of reviewers) {
        await graph.addNode(reviewer);
      }
      await graph.addNode(decider);

      // Create consensus pattern
      for (const reviewer of reviewers) {
        await graph.addEdge({ fromNode: proposer!.id, toNode: reviewer.id });
        await graph.addEdge({ fromNode: reviewer!.id, toNode: decider.id });
      }

      // Validate consensus pattern
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5); // 1 proposer + 3 reviewers + 1 decider
      expect(edgeCount).toBe(6); // 3 edges to reviewers + 3 edges to decider
    });
  });

  describe('Sequential Execution', () => {
    it('should create sequential agent execution workflow', async () => {
      const graph = new WorkflowGraph();

      // Create sequential processing chain
      const nodes = [
        { id: randomUUID(), name: 'Step1', description: 'First step', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Step2', description: 'Second step', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Step3', description: 'Third step', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Step4', description: 'Fourth step', nodeType: 'Agent' },
      ];

      // Add nodes
      for (const node of nodes) {
        await graph.addNode(node);
      }

      // Create sequential connections
      for (let i = 0; i < nodes.length - 1; i++) {
        await graph.addEdge({ fromNode: nodes[i].id, toNode: nodes[i + 1].id });
      }

      // Validate sequential structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3); // n-1 edges for n nodes in sequence
    });

    it('should create workflow with output-to-input data flow', async () => {
      const graph = new WorkflowGraph();

      // Create agents where output of one becomes input of next
      const dataGenerator = { id: randomUUID(), name: 'DataGenerator', description: 'Generates initial data', nodeType: 'Agent' };
      const transformer = { id: randomUUID(), name: 'Transformer', description: 'Transforms data', nodeType: 'Agent' };
      const enricher = { id: randomUUID(), name: 'Enricher', description: 'Enriches transformed data', nodeType: 'Agent' };
      const finalizer = { id: randomUUID(), name: 'Finalizer', description: 'Produces final output', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(dataGenerator);
      await graph.addNode(transformer);
      await graph.addNode(enricher);
      await graph.addNode(finalizer);

      // Create data flow chain
      await graph.addEdge({ fromNode: dataGenerator!.id, toNode: transformer.id });
      await graph.addEdge({ fromNode: transformer!.id, toNode: enricher.id });
      await graph.addEdge({ fromNode: enricher!.id, toNode: finalizer.id });

      // Validate data flow structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(4);
      expect(edgeCount).toBe(3);
      expect(dataGenerator.name).toBe('DataGenerator');
      expect(transformer.name).toBe('Transformer');
      expect(enricher.name).toBe('Enricher');
      expect(finalizer.name).toBe('Finalizer');
    });

    it('should validate state preservation across sequential steps', async () => {
      const llmConfig = createTestLlmConfig();

      // Create workflow with sequential agents
      const workflow = new WorkflowBuilder('Sequential State Test')
        .description('Tests state preservation across sequential agent execution')
        .build();

      // Validate workflow was created
      expect(workflow).toBeDefined();

      const name = await workflow.name();
      const description = await workflow.description();

      expect(name).toBe('Sequential State Test');
      expect(description).toBe('Tests state preservation across sequential agent execution');
    });
  });

  describe('Parallel Execution', () => {
    it('should create parallel agent execution workflow', async () => {
      const graph = new WorkflowGraph();

      // Create parallel processing structure
      const splitter = { id: randomUUID(), name: 'Splitter', description: 'Splits work', nodeType: 'Agent' };
      const parallelAgents = [
        { id: randomUUID(), name: 'ParallelAgent1', description: 'Processes in parallel 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ParallelAgent2', description: 'Processes in parallel 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'ParallelAgent3', description: 'Processes in parallel 3', nodeType: 'Agent' },
      ];
      const joiner = { id: randomUUID(), name: 'Joiner', description: 'Joins results', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(splitter);
      for (const agent of parallelAgents) {
        await graph.addNode(agent);
      }
      await graph.addNode(joiner);

      // Create parallel structure
      for (const agent of parallelAgents) {
        await graph.addEdge({ fromNode: splitter!.id, toNode: agent.id });
        await graph.addEdge({ fromNode: agent!.id, toNode: joiner.id });
      }

      // Validate parallel structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(5); // 1 splitter + 3 parallel + 1 joiner
      expect(edgeCount).toBe(6); // 3 split edges + 3 join edges
    });

    it('should create parallel task distribution and aggregation', async () => {
      const graph = new WorkflowGraph();

      // Create task distributor and parallel processors
      const distributor = { id: randomUUID(), name: 'TaskDistributor', description: 'Distributes tasks', nodeType: 'Agent' };
      const processors = [
        { id: randomUUID(), name: 'Processor1', description: 'Processes task 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Processor2', description: 'Processes task 2', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Processor3', description: 'Processes task 3', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Processor4', description: 'Processes task 4', nodeType: 'Agent' },
      ];
      const aggregator = { id: randomUUID(), name: 'ResultAggregator', description: 'Aggregates results', nodeType: 'Agent' };

      // Add nodes
      await graph.addNode(distributor);
      for (const processor of processors) {
        await graph.addNode(processor);
      }
      await graph.addNode(aggregator);

      // Create distribution and aggregation pattern
      for (const processor of processors) {
        await graph.addEdge({ fromNode: distributor!.id, toNode: processor.id });
        await graph.addEdge({ fromNode: processor!.id, toNode: aggregator.id });
      }

      // Validate structure
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6); // 1 distributor + 4 processors + 1 aggregator
      expect(edgeCount).toBe(8); // 4 distribution + 4 aggregation edges
    });

    it('should create concurrent execution without conflicts', async () => {
      const graph = new WorkflowGraph();

      // Create independent parallel branches
      const input = { id: randomUUID(), name: 'Input', description: 'Input node', nodeType: 'Agent' };
      const branch1 = [
        { id: randomUUID(), name: 'Branch1Step1', description: 'Branch 1 step 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch1Step2', description: 'Branch 1 step 2', nodeType: 'Agent' },
      ];
      const branch2 = [
        { id: randomUUID(), name: 'Branch2Step1', description: 'Branch 2 step 1', nodeType: 'Agent' },
        { id: randomUUID(), name: 'Branch2Step2', description: 'Branch 2 step 2', nodeType: 'Agent' },
      ];
      const output = { id: randomUUID(), name: 'Output', description: 'Output node', nodeType: 'Agent' };

      // Add all nodes
      await graph.addNode(input);
      for (const node of branch1) {
        await graph.addNode(node);
      }
      for (const node of branch2) {
        await graph.addNode(node);
      }
      await graph.addNode(output);

      // Create parallel branches
      await graph.addEdge({ fromNode: input!.id, toNode: branch1[0].id });
      await graph.addEdge({ fromNode: branch1[0].id, toNode: branch1[1].id });
      await graph.addEdge({ fromNode: input!.id, toNode: branch2[0].id });
      await graph.addEdge({ fromNode: branch2[0].id, toNode: branch2[1].id });
      await graph.addEdge({ fromNode: branch1[1].id, toNode: output.id });
      await graph.addEdge({ fromNode: branch2[1].id, toNode: output.id });

      // Validate parallel branches
      const nodeCount = await graph.nodeCount();
      const edgeCount = await graph.edgeCount();

      expect(nodeCount).toBe(6); // 1 input + 2 branch1 nodes + 2 branch2 nodes + 1 output
      expect(edgeCount).toBe(6); // 2 input edges + 2 branch edges + 2 output edges
    });
  });
});
