import { describe, it, expect } from 'vitest';
import { LlmConfig } from '../../index';

describe('LlmConfig', () => {
  describe('OpenAI', () => {
    it('should create OpenAI config with default model', () => {
      const config = LlmConfig.openai({
        apiKey: 'test-key',
      });
      expect(config).toBeDefined();
    });

    it('should create OpenAI config with custom model', () => {
      const config = LlmConfig.openai({
        apiKey: 'test-key',
        model: 'gpt-4',
      });
      expect(config).toBeDefined();
    });

    it('should create OpenAI config with custom model', () => {
      const config = LlmConfig.openai({
        apiKey: 'test-key',
        model: 'gpt-4o-mini',
      });
      expect(config).toBeDefined();
    });

    it('should create OpenAI config without model', () => {
      const config = LlmConfig.openai({
        apiKey: 'test-key',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Anthropic', () => {
    it('should create Anthropic config with default model', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'test-key',
      });
      expect(config).toBeDefined();
    });

    it('should create Anthropic config with custom model', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'test-key',
        model: 'claude-3-opus-20240229',
      });
      expect(config).toBeDefined();
    });

    it('should create Anthropic config with custom model', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'test-key',
        model: 'claude-3-5-sonnet-20241022',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Ollama', () => {
    it('should create Ollama config with default base URL', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
      });
      expect(config).toBeDefined();
    });

    it('should create Ollama config with custom base URL', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
        baseUrl: 'http://localhost:11434',
      });
      expect(config).toBeDefined();
    });

    it('should create Ollama config with model only', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Azure OpenAI', () => {
    it('should create Azure OpenAI config', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'test-key',
        deploymentName: 'gpt-4-deployment',
        endpoint: 'https://test.openai.azure.com',
      });
      expect(config).toBeDefined();
    });

    it('should create Azure OpenAI config with API version', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'test-key',
        deploymentName: 'gpt-4-deployment',
        endpoint: 'https://test.openai.azure.com',
        apiVersion: '2024-02-15-preview',
      });
      expect(config).toBeDefined();
    });
  });

  describe('ByteDance', () => {
    it('should create ByteDance config', () => {
      const config = LlmConfig.bytedance({
        apiKey: 'test-key',
        model: 'skylark-lite',
      });
      expect(config).toBeDefined();
    });
  });

  describe('DeepSeek', () => {
    it('should create DeepSeek config', () => {
      const config = LlmConfig.deepseek({
        apiKey: 'test-key',
        model: 'deepseek-chat',
      });
      expect(config).toBeDefined();
    });
  });

  describe('HuggingFace', () => {
    it('should create HuggingFace config', () => {
      const config = LlmConfig.huggingface({
        apiKey: 'test-key',
        model: 'meta-llama/Llama-2-7b-chat-hf',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Perplexity', () => {
    it('should create Perplexity config', () => {
      const config = LlmConfig.perplexity({
        apiKey: 'test-key',
        model: 'llama-3.1-sonar-small-128k-online',
      });
      expect(config).toBeDefined();
    });
  });

  describe('OpenRouter', () => {
    it('should create OpenRouter config', () => {
      const config = LlmConfig.openrouter({
        apiKey: 'test-key',
        model: 'openai/gpt-4o',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Fireworks', () => {
    it('should create Fireworks config', () => {
      const config = LlmConfig.fireworks({
        apiKey: 'test-key',
        model: 'accounts/fireworks/models/llama-v3p1-70b-instruct',
      });
      expect(config).toBeDefined();
    });
  });

  describe('Replicate', () => {
    it('should create Replicate config', () => {
      const config = LlmConfig.replicate({
        apiKey: 'test-key',
        model: 'meta/llama-2-70b-chat',
      });
      expect(config).toBeDefined();
    });
  });

  describe('TogetherAI', () => {
    it('should create TogetherAI config', () => {
      const config = LlmConfig.togetherai({
        apiKey: 'test-key',
        model: 'meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo',
      });
      expect(config).toBeDefined();
    });
  });

  describe('xAI', () => {
    it('should create xAI config', () => {
      const config = LlmConfig.xai({
        apiKey: 'test-key',
        model: 'grok-beta',
      });
      expect(config).toBeDefined();
    });
  });

  describe('AI21', () => {
    it('should create AI21 config', () => {
      const config = LlmConfig.ai21({
        apiKey: 'test-key',
        model: 'jamba-1.5-large',
      });
      expect(config).toBeDefined();
    });
  });

  describe('MistralAI', () => {
    it('should create MistralAI config', () => {
      const config = LlmConfig.mistralai({
        apiKey: 'test-key',
        model: 'mistral-large-latest',
      });
      expect(config).toBeDefined();
    });
  });
});
