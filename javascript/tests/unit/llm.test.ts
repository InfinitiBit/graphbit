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
});
