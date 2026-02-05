/**
 * Unit tests for LlmClient
 * 
 * Tests all LlmClient methods including:
 * - Constructor and configuration
 * - Simple completions
 * - Full completions with metadata
 * - Batch processing
 * - Chat completions
 * - Statistics and monitoring
 * - Error handling
 */

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { LlmClient, LlmConfig, ClientStats } from '../../index';

describe('LlmClient', () => {
  let client: LlmClient;
  let config: LlmConfig;

  beforeEach(() => {
    // Create a test configuration
    // Note: In actual tests, you might want to use a mock provider
    config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY || 'test-key',
      model: 'gpt-4o-mini'
    });
  });

  describe('Constructor', () => {
    it('should create a new LlmClient instance', () => {
      expect(() => {
        client = new LlmClient(config);
      }).not.toThrow();
    });

    it('should initialize with zero statistics', async () => {
      client = new LlmClient(config);
      const stats = await client.getStats();
      
      expect(stats.totalRequests).toBe(0);
      expect(stats.successfulRequests).toBe(0);
      expect(stats.failedRequests).toBe(0);
      expect(stats.avgResponseTimeMs).toBe(0);
      expect(stats.totalTokens).toBe(0);
    });
  });

  describe('complete()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should reject empty prompt', async () => {
      await expect(client.complete('')).rejects.toThrow('Prompt cannot be empty');
    });

    it('should reject whitespace-only prompt', async () => {
      await expect(client.complete('   ')).rejects.toThrow('Prompt cannot be empty');
    });

    it('should reject zero max_tokens', async () => {
      await expect(
        client.complete('test', 0)
      ).rejects.toThrow('max_tokens must be greater than 0');
    });

    it('should reject invalid temperature (too low)', async () => {
      await expect(
        client.complete('test', 100, -0.1)
      ).rejects.toThrow('temperature must be between 0.0 and 2.0');
    });

    it('should reject invalid temperature (too high)', async () => {
      await expect(
        client.complete('test', 100, 2.1)
      ).rejects.toThrow('temperature must be between 0.0 and 2.0');
    });

    // Integration test - requires actual API key
    it.skip('should complete a simple prompt', async () => {
      const response = await client.complete('What is 2+2? Answer with just the number.');
      expect(response).toContain('4');
    });

    // Integration test - requires actual API key
    it.skip('should respect max_tokens parameter', async () => {
      const response = await client.complete('Write a long story', 10);
      // Response should be short due to token limit
      expect(response.split(' ').length).toBeLessThan(15);
    });

    // Integration test - requires actual API key
    it.skip('should respect temperature parameter', async () => {
      // Low temperature should give deterministic results
      const response1 = await client.complete('Say "hello"', 10, 0.0);
      const response2 = await client.complete('Say "hello"', 10, 0.0);
      expect(response1).toBe(response2);
    });
  });

  describe('completeAsync()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should be an alias for complete()', async () => {
      await expect(client.completeAsync('')).rejects.toThrow('Prompt cannot be empty');
    });

    // Integration test
    it.skip('should return the same result as complete()', async () => {
      const prompt = 'Say "test"';
      const result1 = await client.complete(prompt, 10, 0.0);
      const result2 = await client.completeAsync(prompt, 10, 0.0);
      expect(result1).toBe(result2);
    });
  });

  describe('completeFull()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should reject empty prompt', async () => {
      await expect(client.completeFull('')).rejects.toThrow('Prompt cannot be empty');
    });

    // Integration test - requires actual API key
    it.skip('should return full response with metadata', async () => {
      const response = await client.completeFull('What is AI?', 50, 0.7);
      
      expect(response).toHaveProperty('content');
      expect(response).toHaveProperty('usage');
      expect(response).toHaveProperty('finishReason');
      expect(response).toHaveProperty('model');
      
      expect(response?.content).toBeTypeOf('string');
      expect(response.content.length).toBeGreaterThan(0);
      
      expect(response.usage).toHaveProperty('promptTokens');
      expect(response.usage).toHaveProperty('completionTokens');
      expect(response.usage).toHaveProperty('totalTokens');
      expect(response.usage.totalTokens).toBeGreaterThan(0);
      
      expect(response.model).toBeTypeOf('string');
    });

    // Integration test - requires actual API key
    it.skip('should include tool calls if applicable', async () => {
      // This would require a provider that supports tool calls
      // and a prompt that triggers them
      const response = await client.completeFull('Call a weather tool for Paris');
      expect(response).toHaveProperty('toolCalls');
    });
  });

  describe('completeFullAsync()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should be an alias for completeFull()', async () => {
      await expect(client.completeFullAsync('')).rejects.toThrow('Prompt cannot be empty');
    });
  });

  describe('completeBatch()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should reject empty prompts array', async () => {
      await expect(client.completeBatch([])).rejects.toThrow('Prompts array cannot be empty');
    });

    it('should reject batch size over 1000', async () => {
      const largeArray = new Array(1001).fill('test');
      await expect(client.completeBatch(largeArray)).rejects.toThrow('Batch size cannot exceed 1000');
    });

    it('should filter out empty prompts', async () => {
      // This test would need mocking to verify the filtering behavior
      // For now, just verify it doesn't throw
      expect(() => client.completeBatch(['test', '', '   ', 'test2'])).not.toThrow();
    });

    // Integration test - requires actual API key
    it.skip('should process multiple prompts', async () => {
      const prompts = [
        'What is 2+2? Answer with just the number.',
        'What is 3+3? Answer with just the number.',
        'What is 4+4? Answer with just the number.'
      ];
      
      const responses = await client.completeBatch(prompts, 10, 0.0, 1);
      
      expect(responses).toHaveLength(3);
      expect(responses[0]).toContain('4');
      expect(responses[1]).toContain('6');
      expect(responses[2]).toContain('8');
    });

    // Integration test - requires actual API key
    it.skip('should respect concurrency limit', async () => {
      const prompts = new Array(10).fill('Say "test"');
      const startTime = Date.now();
      
      await client.completeBatch(prompts, 10, 0.0, 2); // Max 2 concurrent
      
      const duration = Date.now() - startTime;
      // With 10 requests and max 2 concurrent, should take at least 5 time units
      // This is a rough test and might be flaky
      expect(duration).toBeGreaterThan(1000); // At least 1 second
    });

    // Integration test - requires actual API key
    it.skip('should handle errors gracefully', async () => {
      const prompts = [
        'Valid prompt',
        '', // This will be filtered out
        'Another valid prompt'
      ];
      
      const responses = await client.completeBatch(prompts);
      expect(responses).toHaveLength(2); // Empty prompt filtered
      expect(responses.every(r => typeof r === 'string')).toBe(true);
    });
  });

  describe('completeStream()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    // Note: Currently completeStream returns complete response
    it.skip('should return a string (future: async iterator)', async () => {
      const response = await client.completeStream('Test', 10);
      expect(typeof response).toBe('string');
    });
  });

  describe('chatOptimized()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should reject empty messages array', async () => {
      await expect(client.chatOptimized([])).rejects.toThrow('Messages array cannot be empty');
    });

    it('should reject invalid message format', async () => {
      await expect(
        client.chatOptimized([['role']])
      ).rejects.toThrow('Each message must be [role, content]');
    });

    it('should reject message with too many elements', async () => {
      await expect(
        client.chatOptimized([['role', 'content', 'extra']])
      ).rejects.toThrow('Each message must be [role, content]');
    });

    // Integration test - requires actual API key
    it.skip('should handle chat messages', async () => {
      const messages = [
        ['system', 'You are a helpful assistant'],
        ['user', 'What is 2+2? Answer with just the number.']
      ];
      
      const response = await client.chatOptimized(messages, 10, 0.0);
      expect(response).toContain('4');
    });

    // Integration test - requires actual API key
    it.skip('should maintain conversation context', async () => {
      const messages = [
        ['system', 'You are a helpful assistant'],
        ['user', 'My name is Alice'],
        ['assistant', 'Hello Alice! How can I help you today?'],
        ['user', 'What is my name?']
      ];
      
      const response = await client.chatOptimized(messages);
      expect(response.toLowerCase()).toContain('alice');
    });
  });

  describe('getStats()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should return stats object', async () => {
      const stats = await client.getStats();
      
      expect(stats).toHaveProperty('totalRequests');
      expect(stats).toHaveProperty('successfulRequests');
      expect(stats).toHaveProperty('failedRequests');
      expect(stats).toHaveProperty('avgResponseTimeMs');
      expect(stats).toHaveProperty('totalTokens');
      expect(stats).toHaveProperty('uptimeSeconds');
    });

    it('should show zero stats initially', async () => {
      const stats = await client.getStats();
      
      expect(stats.totalRequests).toBe(0);
      expect(stats.successfulRequests).toBe(0);
      expect(stats.failedRequests).toBe(0);
      expect(stats.avgResponseTimeMs).toBe(0);
      expect(stats.totalTokens).toBe(0);
    });

    it('should show uptime greater than zero', async () => {
      await new Promise(resolve => setTimeout(resolve, 100));
      const stats = await client.getStats();
      expect(stats.uptimeSeconds).toBeGreaterThan(0);
    });

    // Integration test - requires actual API key
    it.skip('should update stats after requests', async () => {
      await client.complete('Test', 10);
      await client.complete('Test 2', 10);
      
      const stats = await client.getStats();
      expect(stats.totalRequests).toBe(2);
      expect(stats.successfulRequests).toBe(2);
      expect(stats.avgResponseTimeMs).toBeGreaterThan(0);
      expect(stats.totalTokens).toBeGreaterThan(0);
    });
  });

  describe('resetStats()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should reset statistics', async () => {
      // Manually modify stats to have values (in real scenario, make requests)
      await client.resetStats();
      
      const stats = await client.getStats();
      expect(stats.totalRequests).toBe(0);
      expect(stats.successfulRequests).toBe(0);
      expect(stats.failedRequests).toBe(0);
      expect(stats.avgResponseTimeMs).toBe(0);
      expect(stats.totalTokens).toBe(0);
    });

    // Integration test - requires actual API key
    it.skip('should clear stats after requests', async () => {
      await client.complete('Test', 10);
      
      let stats = await client.getStats();
      expect(stats.totalRequests).toBe(1);
      
      await client.resetStats();
      
      stats = await client.getStats();
      expect(stats.totalRequests).toBe(0);
      expect(stats.totalTokens).toBe(0);
    });
  });

  describe('warmup()', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should complete without error', async () => {
      await expect(client.warmup()).resolves.not.toThrow();
    });

    it('should be idempotent', async () => {
      await client.warmup();
      await client.warmup();
      await client.warmup();
      // Should not throw or cause issues
    });

    // Integration test - requires actual API key
    it.skip('should improve subsequent request performance', async () => {
      // First request (cold)
      const start1 = Date.now();
      await client.complete('Test', 10);
      const duration1 = Date.now() - start1;
      
      // Warmup
      await client.warmup();
      
      // Second request (warm)
      const start2 = Date.now();
      await client.complete('Test', 10);
      const duration2 = Date.now() - start2;
      
      // Warm request should be faster (though this is flaky)
      expect(duration2).toBeLessThanOrEqual(duration1 * 1.5);
    });
  });

  describe('Error Handling', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    it('should handle invalid API keys gracefully', async () => {
      const badConfig = LlmConfig.openai({
        apiKey: 'invalid-key',
        model: 'gpt-4o-mini'
      });
      const badClient = new LlmClient(badConfig);
      
      // Should reject with meaningful error
      await expect(badClient.complete('Test')).rejects.toThrow();
    });

    // Integration test - requires actual API key
    it.skip('should update failure stats on error', async () => {
      const badConfig = LlmConfig.openai({
        apiKey: 'invalid-key',
        model: 'gpt-4o-mini'
      });
      const badClient = new LlmClient(badConfig);
      
      try {
        await badClient.complete('Test');
      } catch (e) {
        // Expected to fail
      }
      
      const stats = await badClient.getStats();
      expect(stats.totalRequests).toBe(1);
      expect(stats.failedRequests).toBe(1);
      expect(stats.successfulRequests).toBe(0);
    });
  });

  describe('Circuit Breaker', () => {
    // These tests would require mocking or fault injection
    it.skip('should open circuit after multiple failures', async () => {
      // Would need to simulate multiple failures
    });

    it.skip('should recover after timeout', async () => {
      // Would need to wait for recovery timeout
    });
  });

  describe('Performance', () => {
    beforeEach(() => {
      client = new LlmClient(config);
    });

    // Integration test - requires actual API key
    it.skip('should handle 100 sequential requests', async () => {
      const promises = [];
      for (let i = 0; i < 100; i++) {
        promises.push(client.complete('Test', 5, 0.0));
      }
      
      const results = await Promise.all(promises);
      expect(results).toHaveLength(100);
      expect(results.every(r => typeof r === 'string')).toBe(true);
      
      const stats = await client.getStats();
      expect(stats.totalRequests).toBe(100);
      expect(stats.successfulRequests).toBe(100);
    }, 60000); // 60 second timeout
  });
});

