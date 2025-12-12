/**
 * Unit tests for Enhanced EmbeddingClient
 * 
 * Tests the new similarity() method for comparing embeddings.
 */

import { describe, it, expect } from 'vitest';
import { EmbeddingClient, EmbeddingConfig } from '../../index';

describe('EmbeddingClient - Enhanced Methods', () => {
  describe('similarity()', () => {
    it('should calculate similarity for identical vectors', () => {
      const emb1 = [1.0, 2.0, 3.0, 4.0];
      const emb2 = [1.0, 2.0, 3.0, 4.0];
      
      const similarity = EmbeddingClient.similarity(emb1, emb2);
      expect(similarity).toBeCloseTo(1.0, 4);
    });

    it('should calculate similarity for orthogonal vectors', () => {
      const emb1 = [1.0, 0.0, 0.0];
      const emb2 = [0.0, 1.0, 0.0];
      
      const similarity = EmbeddingClient.similarity(emb1, emb2);
      expect(similarity).toBeCloseTo(0.0, 4);
    });

    it('should calculate similarity for opposite vectors', () => {
      const emb1 = [1.0, 0.0, 0.0];
      const emb2 = [-1.0, 0.0, 0.0];
      
      const similarity = EmbeddingClient.similarity(emb1, emb2);
      expect(similarity).toBeCloseTo(-1.0, 4);
    });

    it('should calculate similarity for similar vectors', () => {
      const emb1 = [0.1, 0.2, 0.3, 0.4];
      const emb2 = [0.15, 0.25, 0.35, 0.45];
      
      const similarity = EmbeddingClient.similarity(emb1, emb2);
      expect(similarity).toBeGreaterThan(0.99);
      expect(similarity).toBeLessThanOrEqual(1.0);
    });

    it('should reject empty embeddings', () => {
      expect(() => {
        EmbeddingClient.similarity([], [1.0, 2.0]);
      }).toThrow('Embeddings cannot be empty');
    });

    it('should reject mismatched lengths', () => {
      expect(() => {
        EmbeddingClient.similarity([1.0, 2.0], [1.0, 2.0, 3.0]);
      }).toThrow('Embeddings must have same length');
    });

    it('should handle high-dimensional vectors', () => {
      const dim = 1536; // OpenAI embedding dimension
      const emb1 = Array(dim).fill(0).map((_, i) => Math.sin(i * 0.01));
      const emb2 = Array(dim).fill(0).map((_, i) => Math.sin(i * 0.01 + 0.1));
      
      const similarity = EmbeddingClient.similarity(emb1, emb2);
      expect(similarity).toBeGreaterThan(-1.0);
      expect(similarity).toBeLessThanOrEqual(1.0);
    });

    it('should be symmetric', () => {
      const emb1 = [1.0, 2.0, 3.0];
      const emb2 = [4.0, 5.0, 6.0];
      
      const sim1 = EmbeddingClient.similarity(emb1, emb2);
      const sim2 = EmbeddingClient.similarity(emb2, emb1);
      
      expect(sim1).toBeCloseTo(sim2, 10);
    });
  });

  describe('Integration - Semantic Search', () => {
    it.skip('should rank documents by similarity', async () => {
      const config = EmbeddingConfig.openai(
        process.env.OPENAI_API_KEY || 'test-key'
      );
      const client = new EmbeddingClient(config);

      // Generate embeddings for query and documents
      const texts = [
        'What is artificial intelligence?', // Query
        'AI is the simulation of human intelligence by machines',
        'The weather is sunny today',
        'Machine learning is a subset of AI'
      ];

      const response = await client.embed(texts);
      const queryEmb = response.embeddings[0];

      // Calculate similarities
      const scores = response.embeddings.slice(1).map((emb, idx) => ({
        text: texts[idx + 1],
        score: EmbeddingClient.similarity(queryEmb, emb)
      }));

      // Rank by similarity
      scores.sort((a, b) => b.score - a.score);

      // Most similar should be AI-related documents
      expect(scores[0].text).toContain('AI');
      expect(scores[0].score).toBeGreaterThan(scores[2].score);
    });
  });

  describe('Integration - Clustering', () => {
    it.skip('should identify similar items', async () => {
      const config = EmbeddingConfig.openai(
        process.env.OPENAI_API_KEY || 'test-key'
      );
      const client = new EmbeddingClient(config);

      const texts = [
        'dog',
        'cat',
        'puppy',
        'car',
        'truck',
        'kitten'
      ];

      const response = await client.embed(texts);

      // Animals should be more similar to each other
      const dogCatSim = EmbeddingClient.similarity(
        response.embeddings[0],
        response.embeddings[1]
      );
      const dogCarSim = EmbeddingClient.similarity(
        response.embeddings[0],
        response.embeddings[3]
      );

      expect(dogCatSim).toBeGreaterThan(dogCarSim);
    });
  });
});

