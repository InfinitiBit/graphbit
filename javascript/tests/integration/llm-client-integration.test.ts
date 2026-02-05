/**
 * LlmClient Integration Tests
 * 
 * Tests LlmClient with real API providers
 * Requires API keys to run
 *
 * Run with: npm test tests/integration/llm-client-integration.test.ts
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { LlmClient, LlmConfig } from '../../index';

const hasOpenAI = !!process.env.OPENAI_API_KEY;
const hasAnthropic = !!process.env.ANTHROPIC_API_KEY;

describe('LlmClient Integration Tests', () => {
  describe('OpenAI Integration', () => {
    let client: LlmClient;

    beforeAll(() => {
      if (!hasOpenAI) {
        console.log('⚠️  Skipping OpenAI tests - No API key');
        return;
      }
      
      const config = LlmConfig.openai({
        apiKey: process.env.OPENAI_API_KEY!,
        model: 'gpt-4o-mini'
      });
      client = new LlmClient(config);
    });

    it('should complete simple prompt', async () => {
      if (!hasOpenAI) return;
      
      const response = await client.complete(
        "What is 2+2? Answer with just the number.",
        10,
        0.0
      );
      
      expect(response).toBeTruthy();
      expect(response.toLowerCase()).toContain('4');
    });

    it('should return full response with metadata', async () => {
      if (!hasOpenAI) return;
      
      const response = await client.completeFull(
        "What is AI? Answer in one sentence.",
        100,
        0.7
      );
      
      expect(response?.content).toBeTruthy();
      expect(response.usage).toBeTruthy();
      expect(response.usage!.totalTokens).toBeGreaterThan(0);
      expect(response.usage!.promptTokens).toBeGreaterThan(0);
      expect(response.usage!.completionTokens).toBeGreaterThan(0);
      expect(response.finishReason).toBeTruthy();
      expect(response.model).toBeTruthy();
    });

    it('should process batch requests', async () => {
      if (!hasOpenAI) return;
      
      const prompts = [
        "What is 2+2? Just the number.",
        "What is 3+3? Just the number.",
        "What is 4+4? Just the number."
      ];
      
      const responses = await client.completeBatch(prompts, 10, 0.0, 2);
      
      expect(responses).toHaveLength(3);
      expect(responses[0]).toContain('4');
      expect(responses[1]).toContain('6');
      expect(responses[2]).toContain('8');
    });

    it('should handle chat completion', async () => {
      if (!hasOpenAI) return;
      
      const messages = [
        ['system', 'You are a helpful assistant who answers briefly.'],
        ['user', 'What is 2+2? Just the number.']
      ];
      
      const response = await client.chatOptimized(messages, 10, 0.0);
      
      expect(response).toBeTruthy();
      expect(response).toContain('4');
    });

    it('should track statistics', async () => {
      if (!hasOpenAI) return;
      
      await client.resetStats();
      
      await client.complete("Test 1", 10);
      await client.complete("Test 2", 10);
      await client.complete("Test 3", 10);
      
      const stats = await client.getStats();
      
      expect(stats.totalRequests).toBe(3);
      expect(stats.successfulRequests).toBe(3);
      expect(stats.failedRequests).toBe(0);
      expect(stats.avgResponseTimeMs).toBeGreaterThan(0);
      expect(stats.totalTokens).toBeGreaterThan(0);
    });

    it('should handle warmup', async () => {
      if (!hasOpenAI) return;
      
      await client.warmup();
      // Should complete without error
      expect(true).toBe(true);
    });

    it('should handle errors gracefully', async () => {
      if (!hasOpenAI) return;
      
      // Invalid config should fail gracefully
      const badConfig = LlmConfig.openai({
        apiKey: 'invalid-key',
        model: 'gpt-4o-mini'
      });
      const badClient = new LlmClient(badConfig);
      
      await expect(badClient.complete("Test")).rejects.toThrow();
    });
  });

  describe('Anthropic Integration', () => {
    let client: LlmClient;

    beforeAll(() => {
      if (!hasAnthropic) {
        console.log('⚠️  Skipping Anthropic tests - No API key');
        return;
      }
      
      const config = LlmConfig.anthropic({
        apiKey: process.env.ANTHROPIC_API_KEY!,
        model: 'claude-3-5-sonnet-20241022'
      });
      client = new LlmClient(config);
    });

    it('should complete with Anthropic', async () => {
      if (!hasAnthropic) return;
      
      const response = await client.complete(
        "What is 2+2? Answer with just the number.",
        10,
        0.0
      );
      
      expect(response).toBeTruthy();
      expect(response).toContain('4');
    });

    it('should return full response', async () => {
      if (!hasAnthropic) return;
      
      const response = await client.completeFull(
        "What is AI? One sentence.",
        100
      );
      
      expect(response?.content).toBeTruthy();
      expect(response.usage).toBeTruthy();
      expect(response.model).toContain('claude');
    });
  });

  describe('Performance Testing', () => {
    let client: LlmClient;

    beforeAll(() => {
      if (!hasOpenAI) return;
      
      const config = LlmConfig.openai({
        apiKey: process.env.OPENAI_API_KEY!
      });
      client = new LlmClient(config);
    });

    it('should handle concurrent requests', async () => {
      if (!hasOpenAI) return;
      
      const promises = Array(10).fill(null).map(() =>
        client.complete("Test", 10, 0.0)
      );
      
      const start = Date.now();
      const results = await Promise.all(promises);
      const duration = Date.now() - start;
      
      expect(results).toHaveLength(10);
      expect(results.every(r => typeof r === 'string')).toBe(true);
      console.log(`10 concurrent requests: ${duration}ms`);
    }, 60000);

    it('should handle large batch efficiently', async () => {
      if (!hasOpenAI) return;
      
      const prompts = Array(20).fill("Test");
      const start = Date.now();
      
      const results = await client.completeBatch(prompts, 10, 0.0, 5);
      const duration = Date.now() - start;
      
      expect(results).toHaveLength(20);
      console.log(`20 batch requests: ${duration}ms`);
      console.log(`Avg per request: ${(duration / 20).toFixed(0)}ms`);
    }, 120000);
  });
});

