/**
 * LLM Provider Integration Tests
 * 
 * Comprehensive tests for all LLM provider configurations, validation,
 * and error handling.
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { LlmConfig, AgentBuilder, init } from '../../index';
import { createTestLlmConfig, hasRealApiKeys } from '../helpers/test-llm-config';

// Initialize GraphBit before running tests
beforeAll(async () => {
  await init();
});

describe('LLM Provider Integration Tests', () => {
  describe('LLM Provider Configuration', () => {
    it('should create OpenAI configuration with all parameters', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create OpenAI configuration with default model', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Anthropic configuration with all parameters', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Anthropic configuration with default model', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Ollama configuration with base URL', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
        baseUrl: 'http://localhost:11434',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Ollama configuration without base URL', () => {
      const config = LlmConfig.ollama({
        model: 'llama3',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Azure OpenAI configuration with all parameters', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'gpt-4o-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
        apiVersion: '2024-10-21',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Azure OpenAI configuration with default API version', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'gpt-4o-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create ByteDance configuration', () => {
      const config = LlmConfig.bytedance({
        apiKey: 'bytedance-test-key-1234567890abcdef',
        model: 'skylark-lite',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create DeepSeek configuration', () => {
      const config = LlmConfig.deepseek({
        apiKey: 'deepseek-test-key-1234567890abcdef',
        model: 'deepseek-chat',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create HuggingFace configuration', () => {
      const config = LlmConfig.huggingface({
        apiKey: 'hf-test-key-1234567890abcdef',
        model: 'meta-llama/Llama-2-7b-chat-hf',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Perplexity configuration', () => {
      const config = LlmConfig.perplexity({
        apiKey: 'pplx-test-key-1234567890abcdef',
        model: 'llama-3.1-sonar-small-128k-online',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create OpenRouter configuration', () => {
      const config = LlmConfig.openrouter({
        apiKey: 'sk-or-test-key-1234567890abcdef',
        model: 'openai/gpt-4o',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Fireworks AI configuration', () => {
      const config = LlmConfig.fireworks({
        apiKey: 'fw-test-key-1234567890abcdef',
        model: 'accounts/fireworks/models/llama-v3p1-70b-instruct',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create Replicate configuration', () => {
      const config = LlmConfig.replicate({
        apiKey: 'r8-test-key-1234567890abcdef',
        model: 'meta/llama-2-70b-chat',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create TogetherAI configuration', () => {
      const config = LlmConfig.togetherai({
        apiKey: 'together-test-key-1234567890abcdef',
        model: 'meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create xAI configuration', () => {
      const config = LlmConfig.xai({
        apiKey: 'xai-test-key-1234567890abcdef',
        model: 'grok-beta',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create AI21 configuration', () => {
      const config = LlmConfig.ai21({
        apiKey: 'ai21-test-key-1234567890abcdef',
        model: 'jamba-1.5-large',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should create MistralAI configuration', () => {
      const config = LlmConfig.mistralai({
        apiKey: 'mistral-test-key-1234567890abcdef',
        model: 'mistral-large-latest',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });
  });

  describe('LLM Client Creation and Validation', () => {
    it('should create agent builder with OpenAI configuration', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builder with Anthropic configuration', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builder with Ollama configuration', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
        baseUrl: 'http://localhost:11434',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builder with Azure OpenAI configuration', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'gpt-4o-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builder with HuggingFace configuration', () => {
      const config = LlmConfig.huggingface({
        apiKey: 'hf-test-key-1234567890abcdef',
        model: 'meta-llama/Llama-2-7b-chat-hf',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builder with MistralAI configuration', () => {
      const config = LlmConfig.mistralai({
        apiKey: 'mistral-test-key-1234567890abcdef',
        model: 'mistral-large-latest',
      });

      const builder = new AgentBuilder('TestAgent', config);

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create multiple agent builders with different providers', () => {
      const openaiConfig = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      const anthropicConfig = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      const ollamaConfig = LlmConfig.ollama({
        model: 'llama2',
      });

      const builder1 = new AgentBuilder('OpenAIAgent', openaiConfig);
      const builder2 = new AgentBuilder('AnthropicAgent', anthropicConfig);
      const builder3 = new AgentBuilder('OllamaAgent', ollamaConfig);

      expect(builder1).toBeInstanceOf(AgentBuilder);
      expect(builder2).toBeInstanceOf(AgentBuilder);
      expect(builder3).toBeInstanceOf(AgentBuilder);
    });

    it('should create agent builders with different model variants', () => {
      const gpt4Config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o',
      });

      const gpt4MiniConfig = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      const gpt35Config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-3.5-turbo',
      });

      const builder1 = new AgentBuilder('GPT4Agent', gpt4Config);
      const builder2 = new AgentBuilder('GPT4MiniAgent', gpt4MiniConfig);
      const builder3 = new AgentBuilder('GPT35Agent', gpt35Config);

      expect(builder1).toBeInstanceOf(AgentBuilder);
      expect(builder2).toBeInstanceOf(AgentBuilder);
      expect(builder3).toBeInstanceOf(AgentBuilder);
    });
  });

  describe('Provider-Specific Features', () => {
    it('should create configuration for providers with streaming support (OpenAI)', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      const builder = new AgentBuilder('StreamingAgent', config)
        .description('Agent with streaming support')
        .systemPrompt('You are a helpful assistant.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for providers with streaming support (Anthropic)', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      const builder = new AgentBuilder('StreamingAgent', config)
        .description('Agent with streaming support')
        .systemPrompt('You are a helpful assistant.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for providers with function calling (OpenAI)', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o',
      });

      const builder = new AgentBuilder('FunctionCallingAgent', config)
        .description('Agent with function calling support')
        .systemPrompt('You are a helpful assistant that can call functions.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for providers with function calling (Anthropic)', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      const builder = new AgentBuilder('ToolUseAgent', config)
        .description('Agent with tool use support')
        .systemPrompt('You are a helpful assistant that can use tools.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for providers with vision support (OpenAI)', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: 'gpt-4o',
      });

      const builder = new AgentBuilder('VisionAgent', config)
        .description('Agent with vision support')
        .systemPrompt('You are a helpful assistant that can analyze images.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for providers with vision support (Anthropic)', () => {
      const config = LlmConfig.anthropic({
        apiKey: 'sk-ant-test-key-1234567890abcdef',
        model: 'claude-3-5-sonnet-20241022',
      });

      const builder = new AgentBuilder('VisionAgent', config)
        .description('Agent with vision support')
        .systemPrompt('You are a helpful assistant that can analyze images.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for local model providers (Ollama)', () => {
      const config = LlmConfig.ollama({
        model: 'llama3',
        baseUrl: 'http://localhost:11434',
      });

      const builder = new AgentBuilder('LocalAgent', config)
        .description('Agent using local model')
        .systemPrompt('You are a helpful assistant running locally.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });

    it('should create configuration for enterprise providers (Azure OpenAI)', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'gpt-4o-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
        apiVersion: '2024-10-21',
      });

      const builder = new AgentBuilder('EnterpriseAgent', config)
        .description('Agent using enterprise deployment')
        .systemPrompt('You are a helpful assistant in an enterprise environment.');

      expect(builder).toBeDefined();
      expect(builder).toBeInstanceOf(AgentBuilder);
    });
  });

  describe('Error Handling and Edge Cases', () => {
    it('should handle empty API key for OpenAI', () => {
      expect(() => {
        LlmConfig.openai({
          apiKey: '',
          model: 'gpt-4o-mini',
        });
      }).not.toThrow();
    });

    it('should handle empty API key for Anthropic', () => {
      expect(() => {
        LlmConfig.anthropic({
          apiKey: '',
          model: 'claude-3-5-sonnet-20241022',
        });
      }).not.toThrow();
    });

    it('should handle empty model name for Ollama', () => {
      expect(() => {
        LlmConfig.ollama({
          model: '',
        });
      }).not.toThrow();
    });

    it('should handle special characters in API keys', () => {
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-with-special-chars-!@#$%^&*()',
        model: 'gpt-4o-mini',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should handle very long model names', () => {
      const longModelName = 'a'.repeat(200);
      const config = LlmConfig.openai({
        apiKey: 'sk-test-key-1234567890abcdef',
        model: longModelName,
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should handle custom base URLs for Ollama', () => {
      const config = LlmConfig.ollama({
        model: 'llama2',
        baseUrl: 'https://custom-ollama-server.example.com:8080',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should handle Azure OpenAI with custom endpoint formats', () => {
      const config = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'my-deployment',
        endpoint: 'https://my-resource.openai.azure.com/',
      });

      expect(config).toBeDefined();
      expect(config).toBeInstanceOf(LlmConfig);
    });

    it('should handle Azure OpenAI with different API versions', () => {
      const config1 = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'my-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
        apiVersion: '2023-05-15',
      });

      const config2 = LlmConfig.azureOpenai({
        apiKey: 'azure-test-key-1234567890abcdef',
        deploymentName: 'my-deployment',
        endpoint: 'https://my-resource.openai.azure.com',
        apiVersion: '2024-10-21',
      });

      expect(config1).toBeDefined();
      expect(config2).toBeDefined();
      expect(config1).toBeInstanceOf(LlmConfig);
      expect(config2).toBeInstanceOf(LlmConfig);
    });

    it('should handle multiple configurations for the same provider', () => {
      const config1 = LlmConfig.openai({
        apiKey: 'sk-test-key-1-1234567890abcdef',
        model: 'gpt-4o',
      });

      const config2 = LlmConfig.openai({
        apiKey: 'sk-test-key-2-1234567890abcdef',
        model: 'gpt-4o-mini',
      });

      const config3 = LlmConfig.openai({
        apiKey: 'sk-test-key-3-1234567890abcdef',
        model: 'gpt-3.5-turbo',
      });

      expect(config1).toBeDefined();
      expect(config2).toBeDefined();
      expect(config3).toBeDefined();
      expect(config1).toBeInstanceOf(LlmConfig);
      expect(config2).toBeInstanceOf(LlmConfig);
      expect(config3).toBeInstanceOf(LlmConfig);
    });
  });
});

