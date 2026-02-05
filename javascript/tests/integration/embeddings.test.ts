import { describe, it, expect, beforeAll } from 'vitest';
import { init, EmbeddingConfig, EmbeddingClient } from '../../index';

describe('Embedding Integration Tests', () => {
  beforeAll(() => {
    init();
  });

  // Helper function to calculate cosine similarity
  function cosineSimilarity(vec1: number[], vec2: number[]): number {
    if (vec1.length !== vec2.length) {
      throw new Error('Vectors must have the same length');
    }

    let dotProduct = 0;
    let norm1 = 0;
    let norm2 = 0;

    for (let i = 0; i < vec1.length; i++) {
      dotProduct += vec1[i] * vec2[i];
      norm1 += vec1[i] * vec1[i];
      norm2 += vec2[i] * vec2[i];
    }

    return dotProduct / (Math.sqrt(norm1) * Math.sqrt(norm2));
  }

  // Helper function to check if we have a valid API key
  function hasOpenAIKey(): boolean {
    return !!process.env.OPENAI_API_KEY && process.env.OPENAI_API_KEY.startsWith('sk-');
  }

  function hasHuggingFaceKey(): boolean {
    return !!process.env.HUGGINGFACE_API_KEY && process.env.HUGGINGFACE_API_KEY.length > 0;
  }

  describe('OpenAI Embedding Provider', () => {
    describe('Configuration and Client Creation', () => {
      it('should create OpenAI embedding config with default model', () => {
        const config = EmbeddingConfig.openai('test-key');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create OpenAI embedding config with text-embedding-3-small', () => {
        const config = EmbeddingConfig.openai('test-key', 'text-embedding-3-small');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create OpenAI embedding config with text-embedding-3-large', () => {
        const config = EmbeddingConfig.openai('test-key', 'text-embedding-3-large');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create OpenAI embedding config with text-embedding-ada-002', () => {
        const config = EmbeddingConfig.openai('test-key', 'text-embedding-ada-002');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create embedding client with OpenAI config', () => {
        const config = EmbeddingConfig.openai('test-key');
        const client = new EmbeddingClient(config);
        expect(client).toBeDefined();
        expect(client).toBeInstanceOf(EmbeddingClient);
        expect(client.embed).toBeDefined();
        expect(typeof client.embed).toBe('function');
      });
    });

    describe('Embedding Generation (with API key)', () => {
      it.skipIf(!hasOpenAIKey())('should generate embedding for single text', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const response = await client.embed(['Hello, world!']);

        // Validate response structure
        expect(response).toBeDefined();
        expect(response.embeddings).toBeDefined();
        expect(Array.isArray(response.embeddings)).toBe(true);
        expect(response.embeddings.length).toBe(1);
        expect(response.model).toBeDefined();
        expect(typeof response.model).toBe('string');

        // Validate embedding dimensions (text-embedding-3-small = 1536 dimensions)
        const embedding = response.embeddings[0]!;
        expect(Array.isArray(embedding)).toBe(true);
        expect(embedding.length).toBe(1536);
        expect(embedding.every(val => typeof val === 'number')).toBe(true);
      });

      it.skipIf(!hasOpenAIKey())('should generate embeddings for multiple texts', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = ['First text', 'Second text', 'Third text'];
        const response = await client.embed(texts);

        // Validate response structure
        expect(response.embeddings).toBeDefined();
        expect(response.embeddings.length).toBe(3);

        // Validate each embedding
        response.embeddings.forEach((embedding: any, index) => {
          expect(Array.isArray(embedding)).toBe(true);
          expect(embedding.length).toBe(1536);
          expect(embedding.every(val => typeof val === 'number')).toBe(true);
        });

        // Validate embeddings are different
        const sim01 = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);
        const sim02 = cosineSimilarity(response.embeddings[0]!, response.embeddings[2]!);
        expect(sim01).toBeLessThan(1.0); // Different texts should have similarity < 1
        expect(sim02).toBeLessThan(1.0);
      });

      it.skipIf(!hasOpenAIKey())('should validate text-embedding-3-large dimensions (3072)', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-large');
        const client = new EmbeddingClient(config);

        const response = await client.embed(['Test text for large model']);

        expect(response.embeddings).toBeDefined();
        expect(response.embeddings.length).toBe(1);
        expect(response.embeddings[0]!.length).toBe(3072);
        expect(response.model).toContain('3-large');
      });

      it.skipIf(!hasOpenAIKey())('should validate text-embedding-ada-002 dimensions (1536)', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-ada-002');
        const client = new EmbeddingClient(config);

        const response = await client.embed(['Test text for ada-002']);

        expect(response.embeddings).toBeDefined();
        expect(response.embeddings.length).toBe(1);
        expect(response.embeddings[0]!.length).toBe(1536);
        expect(response.model).toContain('ada-002');
      });
    });



    describe('Similarity Calculations', () => {
      it.skipIf(!hasOpenAIKey())('should calculate high similarity for identical texts', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const text = 'The quick brown fox jumps over the lazy dog';
        const response = await client.embed([text, text]);

        expect(response.embeddings.length).toBe(2);

        const similarity = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);

        // Identical texts should have similarity very close to 1.0
        expect(similarity).toBeGreaterThan(0.99);
        expect(similarity).toBeLessThanOrEqual(1.0);
      });

      it.skipIf(!hasOpenAIKey())('should calculate high similarity for semantically similar texts', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = [
          'The cat sat on the mat',
          'A feline rested on the rug',
        ];
        const response = await client.embed(texts);

        const similarity = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);

        // Semantically similar texts should have high similarity
        expect(similarity).toBeGreaterThan(0.7);
        expect(similarity).toBeLessThan(1.0);
      });

      it.skipIf(!hasOpenAIKey())('should calculate low similarity for unrelated texts', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = [
          'Machine learning is a subset of artificial intelligence',
          'The recipe calls for two cups of flour and three eggs',
        ];
        const response = await client.embed(texts);

        const similarity = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);

        // Unrelated texts should have lower similarity
        expect(similarity).toBeLessThan(0.7);
        expect(similarity).toBeGreaterThan(0); // But still positive
      });
    });

    describe('Batch Processing', () => {
      it.skipIf(!hasOpenAIKey())('should process batch of 10 texts efficiently', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = Array.from({ length: 10 }, (_, i) => `Text number ${i + 1}`);
        const startTime = Date.now();
        const response = await client.embed(texts);
        const duration = Date.now() - startTime;

        // Validate all embeddings generated
        expect(response.embeddings.length).toBe(10);
        response.embeddings.forEach((embedding: any, index) => {
          expect(embedding.length).toBe(1536);
          expect(embedding.every(val => typeof val === 'number')).toBe(true);
        });

        // Batch processing should be reasonably fast (< 10 seconds for 10 texts)
        expect(duration).toBeLessThan(10000);
      });

      it.skipIf(!hasOpenAIKey())('should process batch of 50 texts', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = Array.from({ length: 50 }, (_, i) => `Document ${i + 1}: This is a test document for batch processing.`);
        const response = await client.embed(texts);

        // Validate all embeddings generated
        expect(response.embeddings.length).toBe(50);

        // Validate each embedding has correct dimensions
        response.embeddings.forEach(embedding => {
          expect(embedding.length).toBe(1536);
        });

        // Validate embeddings are unique (not all the same)
        const firstEmbedding = response.embeddings[0]!;
        const lastEmbedding = response.embeddings[49]!;
        const similarity = cosineSimilarity(firstEmbedding, lastEmbedding);
        expect(similarity).toBeLessThan(1.0); // Should be different
      });

      it.skipIf(!hasOpenAIKey())('should handle varying text lengths in batch', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = [
          'Short',
          'Medium length text with more words',
          'This is a much longer text that contains significantly more content and should still be processed correctly by the embedding model without any issues.',
        ];
        const response = await client.embed(texts);

        // All texts should produce embeddings of the same dimension
        expect(response.embeddings.length).toBe(3);
        response.embeddings.forEach(embedding => {
          expect(embedding.length).toBe(1536);
        });
      });
    });

    describe('Error Handling', () => {
      it('should reject with invalid API key', async () => {
        const config = EmbeddingConfig.openai('invalid-key-12345');
        const client = new EmbeddingClient(config);

        await expect(client.embed(['test text'])).rejects.toThrow();
      });

      it('should reject with empty API key', async () => {
        const config = EmbeddingConfig.openai('');
        const client = new EmbeddingClient(config);

        await expect(client.embed(['test text'])).rejects.toThrow();
      });

      it.skipIf(!hasOpenAIKey())('should handle empty text array', async () => {
        const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        // Empty array should either return empty result or throw error
        try {
          const response = await client.embed([]);
          expect(response.embeddings.length).toBe(0);
        } catch (error) {
          expect(error).toBeDefined();
        }
      });
    });
  });

  describe('HuggingFace Embedding Provider', () => {
    describe('Configuration and Client Creation', () => {
      it('should create HuggingFace embedding config with sentence-transformers model', () => {
        const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-MiniLM-L6-v2');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create HuggingFace embedding config with all-mpnet-base-v2', () => {
        const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-mpnet-base-v2');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create HuggingFace embedding config with BAAI model', () => {
        const config = EmbeddingConfig.huggingface('test-key', 'BAAI/bge-small-en-v1.5');
        expect(config).toBeDefined();
        expect(config).toBeInstanceOf(Object);
      });

      it('should create embedding client with HuggingFace config', () => {
        const config = EmbeddingConfig.huggingface('test-key', 'sentence-transformers/all-MiniLM-L6-v2');
        const client = new EmbeddingClient(config);
        expect(client).toBeDefined();
        expect(client).toBeInstanceOf(EmbeddingClient);
        expect(client.embed).toBeDefined();
        expect(typeof client.embed).toBe('function');
      });
    });

    describe('Embedding Generation (with API key)', () => {
      it.skipIf(!hasHuggingFaceKey())('should generate embedding for single text', async () => {
        const config = EmbeddingConfig.huggingface(
          process.env.HUGGINGFACE_API_KEY!,
          'sentence-transformers/all-MiniLM-L6-v2'
        );
        const client = new EmbeddingClient(config);

        const response = await client.embed(['Hello, world!']);

        // Validate response structure
        expect(response).toBeDefined();
        expect(response.embeddings).toBeDefined();
        expect(Array.isArray(response.embeddings)).toBe(true);
        expect(response.embeddings.length).toBe(1);
        expect(response.model).toBeDefined();

        // Validate embedding dimensions (all-MiniLM-L6-v2 = 384 dimensions)
        const embedding = response.embeddings[0]!;
        expect(Array.isArray(embedding)).toBe(true);
        expect(embedding.length).toBe(384);
        expect(embedding.every(val => typeof val === 'number')).toBe(true);
      });

      it.skipIf(!hasHuggingFaceKey())('should generate embeddings for multiple texts', async () => {
        const config = EmbeddingConfig.huggingface(
          process.env.HUGGINGFACE_API_KEY!,
          'sentence-transformers/all-MiniLM-L6-v2'
        );
        const client = new EmbeddingClient(config);

        const texts = ['First text', 'Second text', 'Third text'];
        const response = await client.embed(texts);

        // Validate response structure
        expect(response.embeddings).toBeDefined();
        expect(response.embeddings.length).toBe(3);

        // Validate each embedding
        response.embeddings.forEach((embedding: any, index) => {
          expect(Array.isArray(embedding)).toBe(true);
          expect(embedding.length).toBe(384);
          expect(embedding.every(val => typeof val === 'number')).toBe(true);
        });

        // Validate embeddings are different
        const sim01 = cosineSimilarity(response.embeddings[0]!, response.embeddings[1]!);
        const sim02 = cosineSimilarity(response.embeddings[0]!, response.embeddings[2]!);
        expect(sim01).toBeLessThan(1.0);
        expect(sim02).toBeLessThan(1.0);
      });

      it.skipIf(!hasHuggingFaceKey())('should validate all-mpnet-base-v2 dimensions (768)', async () => {
        const config = EmbeddingConfig.huggingface(
          process.env.HUGGINGFACE_API_KEY!,
          'sentence-transformers/all-mpnet-base-v2'
        );
        const client = new EmbeddingClient(config);

        const response = await client.embed(['Test text for mpnet model']);

        expect(response.embeddings).toBeDefined();
        expect(response.embeddings.length).toBe(1);
        expect(response.embeddings[0]!.length).toBe(768);
      });
    });

    describe('Batch Processing', () => {
      it.skipIf(!hasHuggingFaceKey())('should process batch of 10 texts', async () => {
        const config = EmbeddingConfig.huggingface(
          process.env.HUGGINGFACE_API_KEY!,
          'sentence-transformers/all-MiniLM-L6-v2'
        );
        const client = new EmbeddingClient(config);

        const texts = Array.from({ length: 10 }, (_, i) => `Text number ${i + 1}`);
        const response = await client.embed(texts);

        // Validate all embeddings generated
        expect(response.embeddings.length).toBe(10);
        response.embeddings.forEach(embedding => {
          expect(embedding.length).toBe(384);
          expect(embedding.every(val => typeof val === 'number')).toBe(true);
        });
      });
    });

    describe('Error Handling', () => {
      it('should reject with invalid API key', async () => {
        const config = EmbeddingConfig.huggingface('invalid-key', 'sentence-transformers/all-MiniLM-L6-v2');
        const client = new EmbeddingClient(config);

        await expect(client.embed(['test text'])).rejects.toThrow();
      });
    });
  });

  describe('Cross-Provider Comparisons', () => {
    it.skipIf(!hasOpenAIKey())('should validate OpenAI embeddings are normalized', async () => {
      const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
      const client = new EmbeddingClient(config);

      const response = await client.embed(['Test normalization']);
      const embedding = response.embeddings[0]!;

      // Calculate L2 norm
      const norm = Math.sqrt(embedding.reduce((sum, val) => sum + val * val, 0));

      // OpenAI embeddings should be normalized (L2 norm ≈ 1.0)
      expect(norm).toBeGreaterThan(0.99);
      expect(norm).toBeLessThan(1.01);
    });

    it.skipIf(!hasOpenAIKey())('should validate embedding vector properties', async () => {
      const config = EmbeddingConfig.openai(process.env.OPENAI_API_KEY!, 'text-embedding-3-small');
      const client = new EmbeddingClient(config);

      const response = await client.embed(['Test vector properties']);
      const embedding = response.embeddings[0]!;

      // Validate all values are finite numbers
      expect(embedding.every(val => Number.isFinite(val))).toBe(true);

      // Validate no NaN values
      expect(embedding.every(val => !Number.isNaN(val))).toBe(true);

      // Validate values are in reasonable range (typically -1 to 1 for normalized vectors)
      expect(embedding.every(val => Math.abs(val) <= 1.5)).toBe(true);
    });
  });
});

