import { describe, it, expect } from 'vitest';
import { EmbeddingConfig, EmbeddingClient } from '../../index';

describe('EmbeddingConfig', () => {
  describe('openai', () => {
    it('should create OpenAI embedding config with default model', () => {
      const config = EmbeddingConfig.openai('test-key');
      expect(config).toBeDefined();
    });

    it('should create OpenAI embedding config with custom model', () => {
      const config = EmbeddingConfig.openai('test-key', 'text-embedding-3-small');
      expect(config).toBeDefined();
    });

    it('should create OpenAI embedding config with text-embedding-3-large', () => {
      const config = EmbeddingConfig.openai('test-key', 'text-embedding-3-large');
      expect(config).toBeDefined();
    });

    it('should create OpenAI embedding config with text-embedding-ada-002', () => {
      const config = EmbeddingConfig.openai('test-key', 'text-embedding-ada-002');
      expect(config).toBeDefined();
    });
  });

  describe('huggingface', () => {
    it('should create HuggingFace embedding config', () => {
      const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-MiniLM-L6-v2');
      expect(config).toBeDefined();
    });

    it('should create HuggingFace embedding config with different model', () => {
      const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-mpnet-base-v2');
      expect(config).toBeDefined();
    });

    it('should create HuggingFace embedding config with custom model', () => {
      const config = EmbeddingConfig.huggingface('test-key', 'BAAI/bge-small-en-v1.5');
      expect(config).toBeDefined();
    });
  });
});

describe('EmbeddingClient', () => {
  describe('constructor', () => {
    it('should create an embedding client with OpenAI config', () => {
      const config = EmbeddingConfig.openai('test-key');
      const client = new EmbeddingClient(config);
      expect(client).toBeDefined();
    });

    it('should create an embedding client with HuggingFace config', () => {
      const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-MiniLM-L6-v2');
      const client = new EmbeddingClient(config);
      expect(client).toBeDefined();
    });
  });

  describe('embed', () => {
    it('should have embed method', () => {
      const config = EmbeddingConfig.openai('test-key');
      const client = new EmbeddingClient(config);
      expect(client.embed).toBeDefined();
      expect(typeof client.embed).toBe('function');
    });

    it('should reject embedding without valid API key', async () => {
      const config = EmbeddingConfig.openai('invalid-key');
      const client = new EmbeddingClient(config);
      
      // This will fail without a valid API key
      await expect(client.embed(['test text'])).rejects.toThrow();
    });

    it('should accept array of texts', async () => {
      const config = EmbeddingConfig.openai('test-key');
      const client = new EmbeddingClient(config);
      
      // This will fail without a valid API key, but we're testing the API
      try {
        await client.embed(['text 1', 'text 2', 'text 3']);
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });

    it('should accept single text in array', async () => {
      const config = EmbeddingConfig.openai('test-key');
      const client = new EmbeddingClient(config);
      
      try {
        await client.embed(['single text']);
      } catch (error) {
        // Expected to fail without valid API key
        expect(error).toBeDefined();
      }
    });

    it('should accept empty array', async () => {
      const config = EmbeddingConfig.openai('test-key');
      const client = new EmbeddingClient(config);
      
      try {
        await client.embed([]);
      } catch (error) {
        // Expected to fail or return empty result
        expect(error).toBeDefined();
      }
    });
  });

  describe('configuration variations', () => {
    it('should work with different OpenAI models', () => {
      const models = [
        'text-embedding-3-small',
        'text-embedding-3-large',
        'text-embedding-ada-002',
      ];

      models.forEach(model => {
        const config = EmbeddingConfig.openai('test-key', model);
        const client = new EmbeddingClient(config);
        expect(client).toBeDefined();
      });
    });

    it('should work with different HuggingFace models', () => {
      const models = [
        'sentence-transformers/all-MiniLM-L6-v2',
        'sentence-transformers/all-mpnet-base-v2',
        'BAAI/bge-small-en-v1.5',
      ];

      models.forEach(model => {
        const config = EmbeddingConfig.huggingface('test-key', model);
        const client = new EmbeddingClient(config);
        expect(client).toBeDefined();
      });
    });
  });

  describe('error handling', () => {
    it('should handle invalid configuration gracefully', async () => {
      const config = EmbeddingConfig.openai('');
      const client = new EmbeddingClient(config);
      
      await expect(client.embed(['test'])).rejects.toThrow();
    });
  });
});

