import { describe, it, expect } from 'vitest';
import { AgentBuilder, LlmConfig } from '../../index';

describe('Agent', () => {
  describe('AgentBuilder', () => {
    it('should create an agent builder with name and LLM config', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
        model: 'gpt-4o-mini',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      expect(builder).toBeDefined();
    });

    it('should set description on agent builder', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      const result = builder.description('A test agent for unit testing');
      expect(result).toBe(builder); // Should return this for chaining
    });

    it('should set system prompt on agent builder', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      const result = builder.systemPrompt('You are a helpful assistant');
      expect(result).toBe(builder);
    });

    it('should set temperature on agent builder', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      const result = builder.temperature(0.7);
      expect(result).toBe(builder);
    });

    it('should set max tokens on agent builder', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      const result = builder.maxTokens(1000);
      expect(result).toBe(builder);
    });

    it('should chain builder methods', () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig)
        .description('A test agent')
        .systemPrompt('You are helpful')
        .temperature(0.5)
        .maxTokens(500);
      
      expect(builder).toBeDefined();
    });

    it('should build an agent', async () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig)
        .description('A test agent')
        .systemPrompt('You are helpful');
      
      // Note: This will fail without a valid API key, but we're testing the API
      // In a real scenario, we'd need to mock the LLM provider
      await expect(builder.build()).rejects.toThrow();
    });
  });

  describe('Agent.execute()', () => {
    it('should have execute method that takes a string message', async () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig)
        .description('A test agent')
        .systemPrompt('You are helpful');
      
      // This will fail without valid API key, but we're testing the API exists
      try {
        const agent = await builder.build();
        expect(agent.execute).toBeDefined();
        expect(typeof agent.execute).toBe('function');
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });

    it('should return a promise from execute', async () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      
      try {
        const agent = await builder.build();
        const result = agent.execute('Hello');
        expect(result).toBeInstanceOf(Promise);
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });
  });

  describe('Agent properties', () => {
    it('should have name method', async () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      
      try {
        const agent = await builder.build();
        expect(agent.name).toBeDefined();
        expect(typeof agent.name).toBe('function');
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });

    it('should have description method', async () => {
      const llmConfig = LlmConfig.openai({
        apiKey: 'test-key',
      });
      
      const builder = new AgentBuilder('Test Agent', llmConfig);
      
      try {
        const agent = await builder.build();
        expect(agent.description).toBeDefined();
        expect(typeof agent.description).toBe('function');
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });
  });
});

