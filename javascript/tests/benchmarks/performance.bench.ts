import { bench, describe } from 'vitest';
import { TextSplitter, WorkflowBuilder, LlmConfig } from '../../index';

describe('Performance Benchmarks', () => {
  const largeText = 'Lorem ipsum dolor sit amet. '.repeat(1000);

  describe('Text Splitting Performance', () => {
    bench('Character splitter - 1000 chunks', async () => {
      const splitter = TextSplitter.character(100, 20);
      await splitter.split(largeText);
    });

    bench('Recursive splitter - 1000 chunks', async () => {
      const splitter = TextSplitter.recursive(100, 20);
      await splitter.split(largeText);
    });

    bench('Sentence splitter - 1000 chunks', async () => {
      const splitter = TextSplitter.sentence(5);
      await splitter.split(largeText);
    });

    bench('Token splitter - 1000 chunks', async () => {
      const splitter = TextSplitter.token(100, 20);
      await splitter.split(largeText);
    });
  });

  describe('Workflow Builder Performance', () => {
    bench('Create simple workflow', () => {
      const builder = new WorkflowBuilder('Benchmark Workflow');
      builder.description('A benchmark workflow');
      builder.build();
    });

    bench('Create workflow with metadata', () => {
      const builder = new WorkflowBuilder('Benchmark Workflow');
      builder.description('A benchmark workflow');
      builder.addMetadata('key1', JSON.stringify({ value: 'test1' }));
      builder.addMetadata('key2', JSON.stringify({ value: 'test2' }));
      builder.addMetadata('key3', JSON.stringify({ value: 'test3' }));
      builder.build();
    });
  });

  describe('LLM Config Creation Performance', () => {
    bench('Create OpenAI config', () => {
      LlmConfig.openai({
        apiKey: 'test-key',
        model: 'gpt-4o-mini',
      });
    });

    bench('Create Anthropic config', () => {
      LlmConfig.anthropic({
        apiKey: 'test-key',
        model: 'claude-3-5-sonnet-20241022',
      });
    });

    bench('Create Ollama config', () => {
      LlmConfig.ollama({
        model: 'llama2',
        baseUrl: 'http://localhost:11434',
      });
    });
  });

  describe('Memory Efficiency', () => {
    bench('Split large text without memory leak', async () => {
      const splitter = TextSplitter.character(1000, 100);
      const veryLargeText = 'Lorem ipsum dolor sit amet. '.repeat(10000);
      await splitter.split(veryLargeText);
    });

    bench('Create multiple workflows', () => {
      for (let i = 0; i < 100; i++) {
        const builder = new WorkflowBuilder(`Workflow ${i}`);
        builder.build();
      }
    });
  });
});
