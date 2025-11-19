/**
 * TypeScript type tests
 *
 * These tests verify that the TypeScript types are correct and provide
 * proper type safety. They don't run at runtime but are checked by the
 * TypeScript compiler.
 */

import { describe, it, expectTypeOf } from 'vitest';
import {
  init,
  version,
  versionInfo,
  LlmConfig,
  WorkflowBuilder,
  Workflow,
  Executor,
  WorkflowContext,
  AgentBuilder,
  Agent,
  TextSplitter,
  DocumentLoader,
  EmbeddingClient,
  EmbeddingConfig,
  WorkflowState,
  AgentCapability,
  ErrorKind,
} from '../../index';

describe('Type Tests', () => {
  it('should have correct function signatures', () => {
    expectTypeOf(init).toBeFunction();
    expectTypeOf(init).returns.toBeVoid();

    expectTypeOf(version).toBeFunction();
    expectTypeOf(version).returns.toBeString();

    expectTypeOf(versionInfo).toBeFunction();
    expectTypeOf(versionInfo).returns.toMatchTypeOf<{
      version: string;
      rustVersion: string;
      napiVersion: string;
    }>();
  });

  it('should have correct LlmConfig types', () => {
    expectTypeOf(LlmConfig.openai).toBeFunction();
    expectTypeOf(LlmConfig.anthropic).toBeFunction();
    expectTypeOf(LlmConfig.ollama).toBeFunction();
    expectTypeOf(LlmConfig.azureOpenai).toBeFunction();
  });

  it('should have correct WorkflowBuilder types', () => {
    const builder = new WorkflowBuilder('test');

    expectTypeOf(builder.description).toBeFunction();
    expectTypeOf(builder.description).parameter(0).toBeString();
    expectTypeOf(builder.description).returns.toMatchTypeOf<WorkflowBuilder>();

    expectTypeOf(builder.addMetadata).toBeFunction();
    expectTypeOf(builder.addMetadata).parameter(0).toBeString();
    expectTypeOf(builder.addMetadata).parameter(1).toBeString();

    expectTypeOf(builder.build).toBeFunction();
    expectTypeOf(builder.build).returns.toMatchTypeOf<Workflow>();
  });

  it('should have correct Workflow types', () => {
    const builder = new WorkflowBuilder('test');
    const workflow = builder.build();

    expectTypeOf(workflow.id).toBeFunction();
    expectTypeOf(workflow.id).returns.resolves.toBeString();

    expectTypeOf(workflow.name).toBeFunction();
    expectTypeOf(workflow.name).returns.resolves.toBeString();

    expectTypeOf(workflow.description).toBeFunction();
    expectTypeOf(workflow.description).returns.resolves.toBeString();
  });

  it('should have correct Executor types', () => {
    const config = LlmConfig.openai({ apiKey: 'test' });
    const executor = new Executor(config);

    expectTypeOf(executor.execute).toBeFunction();
    expectTypeOf(executor.execute).returns.resolves.toMatchTypeOf<WorkflowContext>();
  });

  it('should have correct WorkflowContext types', () => {
    expectTypeOf<WorkflowContext>().toHaveProperty('isCompleted');
    expectTypeOf<WorkflowContext>().toHaveProperty('isFailed');
    expectTypeOf<WorkflowContext>().toHaveProperty('state');
    expectTypeOf<WorkflowContext>().toHaveProperty('stats');
    expectTypeOf<WorkflowContext>().toHaveProperty('error');
    expectTypeOf<WorkflowContext>().toHaveProperty('getAllOutputs');
  });

  it('should have correct AgentBuilder types', () => {
    const llmConfig = LlmConfig.openai({ apiKey: 'test', model: 'gpt-4o-mini' });
    const builder = new AgentBuilder('test', llmConfig);

    expectTypeOf(builder.description).toBeFunction();
    expectTypeOf(builder.systemPrompt).toBeFunction();
    expectTypeOf(builder.temperature).toBeFunction();
    expectTypeOf(builder.maxTokens).toBeFunction();
    expectTypeOf(builder.build).toBeFunction();
    expectTypeOf(builder.build).returns.toMatchTypeOf<Agent>();
  });

  it('should have correct TextSplitter types', () => {
    expectTypeOf(TextSplitter.character).toBeFunction();
    expectTypeOf(TextSplitter.recursive).toBeFunction();
    expectTypeOf(TextSplitter.sentence).toBeFunction();
    expectTypeOf(TextSplitter.token).toBeFunction();

    const splitter = TextSplitter.character(100);
    expectTypeOf(splitter.split).toBeFunction();
    expectTypeOf(splitter.split).parameter(0).toBeString();
  });

  it('should have correct DocumentLoader types', () => {
    const loader = new DocumentLoader();

    expectTypeOf(loader.loadFile).toBeFunction();
    expectTypeOf(loader.loadFile).parameter(0).toBeString();
    expectTypeOf(loader.loadFile).returns.resolves.toMatchTypeOf<{
      content: string;
      metadata?: string;
      source: string;
    }>();

    expectTypeOf(loader.loadText).toBeFunction();
    expectTypeOf(loader.loadText).parameter(0).toBeString();
  });

  it('should have correct EmbeddingClient types', () => {
    const config = EmbeddingConfig.openai('test-key');
    const client = new EmbeddingClient(config);

    expectTypeOf(client.embed).toBeFunction();
    expectTypeOf(client.embed).parameter(0).toBeArray();
    expectTypeOf(client.embed).returns.resolves.toMatchTypeOf<{
      embeddings: number[][];
      model: string;
      usage?: { promptTokens: number; totalTokens: number };
    }>();
  });

  it('should have correct enum types', () => {
    // Enums are numeric const enums in the generated TypeScript
    expectTypeOf<WorkflowState>().toBeNumber();
    expectTypeOf<AgentCapability>().toBeNumber();
    expectTypeOf<ErrorKind>().toBeNumber();
  });
});
