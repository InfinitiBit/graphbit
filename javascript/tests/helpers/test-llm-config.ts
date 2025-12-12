/**
 * Test LLM Configuration Helpers
 * 
 * Provides mock LLM configurations for testing without requiring real API keys.
 * Uses Ollama as the base provider since it doesn't require API keys.
 */

import { LlmConfig } from '../../index';

/**
 * Create a test LLM configuration using Ollama
 * 
 * This configuration can be used for testing without requiring API keys.
 * Note: Actual execution will fail if Ollama is not running, but the
 * configuration itself is valid for testing workflow construction.
 * 
 * @param model - Model name (default: 'llama2')
 * @param baseUrl - Base URL for Ollama (default: 'http://localhost:11434')
 * @returns LlmConfig instance for testing
 */
export function createTestLlmConfig(
  model: string = 'llama2',
  baseUrl: string = 'http://localhost:11434'
): LlmConfig {
  return LlmConfig.ollama({
    model,
    baseUrl,
  });
}

/**
 * Create a test LLM configuration with a fake API key
 * 
 * This uses OpenAI configuration with a dummy API key.
 * Useful for testing configuration creation and validation.
 * Note: Actual API calls will fail with authentication errors.
 * 
 * @param model - Model name (default: 'gpt-4o-mini')
 * @returns LlmConfig instance for testing
 */
export function createTestLlmConfigWithFakeKey(
  model: string = 'gpt-4o-mini'
): LlmConfig {
  return LlmConfig.openai({
    apiKey: 'sk-test-key-for-testing-only-not-real-1234567890abcdef',
    model,
  });
}

/**
 * Get a real LLM configuration from environment variables
 * 
 * Checks for API keys in environment variables and returns the first
 * available configuration. Returns null if no API keys are found.
 * 
 * Priority order:
 * 1. OPENAI_API_KEY
 * 2. ANTHROPIC_API_KEY
 * 3. Ollama (no key required)
 * 
 * @returns LlmConfig instance or null if no API keys available
 */
export function getRealLlmConfig(): LlmConfig | null {
  // Try OpenAI first
  const openaiKey = process.env.OPENAI_API_KEY;
  if (openaiKey && openaiKey.length > 0) {
    return LlmConfig.openai({
      apiKey: openaiKey,
      model: 'gpt-4o-mini',
    });
  }

  // Try Anthropic
  const anthropicKey = process.env.ANTHROPIC_API_KEY;
  if (anthropicKey && anthropicKey.length > 0) {
    return LlmConfig.anthropic({
      apiKey: anthropicKey,
      model: 'claude-3-5-sonnet-20241022',
    });
  }

  // Fall back to Ollama (no key required, but may not be running)
  return LlmConfig.ollama({
    model: 'llama2',
    baseUrl: 'http://localhost:11434',
  });
}

/**
 * Check if real API keys are available
 * 
 * @returns true if at least one real API key is available
 */
export function hasRealApiKeys(): boolean {
  return !!(
    process.env.OPENAI_API_KEY ||
    process.env.ANTHROPIC_API_KEY
  );
}

/**
 * Skip test if no real API keys are available
 * 
 * Helper function to conditionally skip tests that require real API access.
 * 
 * @param testFn - Test function to run if API keys are available
 * @returns Test function that skips if no API keys
 */
export function skipIfNoApiKeys<T>(testFn: () => T): () => T | void {
  return () => {
    if (!hasRealApiKeys()) {
      console.log('Skipping test: No API keys available');
      return;
    }
    return testFn();
  };
}

/**
 * Create multiple test LLM configurations for testing different providers
 * 
 * @returns Array of test LLM configurations
 */
export function createMultipleTestConfigs(): LlmConfig[] {
  const configs: LlmConfig[] = [];

  // Always add Ollama (no key required)
  configs.push(createTestLlmConfig());

  // Add real providers if keys are available
  if (process.env.OPENAI_API_KEY) {
    configs.push(LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY,
      model: 'gpt-4o-mini',
    }));
  }

  if (process.env.ANTHROPIC_API_KEY) {
    configs.push(LlmConfig.anthropic({
      apiKey: process.env.ANTHROPIC_API_KEY,
      model: 'claude-3-5-sonnet-20241022',
    }));
  }

  return configs;
}

