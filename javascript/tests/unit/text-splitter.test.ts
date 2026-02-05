import { describe, it, expect } from 'vitest';
import { TextSplitter } from '../../index';

describe('TextSplitter', () => {
  const sampleText = `This is a test document.
It has multiple sentences.
And multiple paragraphs.

This is the second paragraph.
It also has multiple sentences.`;

  describe('Character Splitter', () => {
    it('should create a character splitter', () => {
      const splitter = TextSplitter.character(100);
      expect(splitter).toBeDefined();
    });

    it('should create a character splitter with overlap', () => {
      const splitter = TextSplitter.character(100, 20);
      expect(splitter).toBeDefined();
    });

    it('should split text into chunks', async () => {
      const splitter = TextSplitter.character(50, 10);
      const chunks = await splitter.split(sampleText);
      expect(chunks).toBeDefined();
      expect(Array.isArray(chunks)).toBe(true);
      expect(chunks.length).toBeGreaterThan(0);
    });

    it('should return chunks with correct properties', async () => {
      const splitter = TextSplitter.character(50, 10);
      const chunks = await splitter.split(sampleText);

      chunks.forEach((chunk) => {
        expect(chunk).toHaveProperty('content');
        expect(chunk).toHaveProperty('startIndex');
        expect(chunk).toHaveProperty('endIndex');
        expect(typeof chunk?.content).toBe('string');
        expect(typeof chunk.startIndex).toBe('number');
        expect(typeof chunk.endIndex).toBe('number');
      });
    });
  });

  describe('Recursive Splitter', () => {
    it('should create a recursive splitter', () => {
      const splitter = TextSplitter.recursive(100);
      expect(splitter).toBeDefined();
    });

    it('should create a recursive splitter with overlap', () => {
      const splitter = TextSplitter.recursive(100, 20);
      expect(splitter).toBeDefined();
    });

    it('should split text into chunks', async () => {
      const splitter = TextSplitter.recursive(50, 10);
      const chunks = await splitter.split(sampleText);
      expect(chunks).toBeDefined();
      expect(Array.isArray(chunks)).toBe(true);
      expect(chunks.length).toBeGreaterThan(0);
    });
  });

  describe('Sentence Splitter', () => {
    it('should create a sentence splitter with default max sentences', () => {
      const splitter = TextSplitter.sentence();
      expect(splitter).toBeDefined();
    });

    it('should create a sentence splitter with custom max sentences', () => {
      const splitter = TextSplitter.sentence(3);
      expect(splitter).toBeDefined();
    });

    it('should split text into sentence chunks', async () => {
      const splitter = TextSplitter.sentence(2);
      const chunks = await splitter.split(sampleText);
      expect(chunks).toBeDefined();
      expect(Array.isArray(chunks)).toBe(true);
      expect(chunks.length).toBeGreaterThan(0);
    });
  });

  describe('Token Splitter', () => {
    it('should create a token splitter', () => {
      const splitter = TextSplitter.token(100);
      expect(splitter).toBeDefined();
    });

    it('should create a token splitter with overlap', () => {
      const splitter = TextSplitter.token(100, 20);
      expect(splitter).toBeDefined();
    });

    it('should split text into token chunks', async () => {
      const splitter = TextSplitter.token(50, 10);
      const chunks = await splitter.split(sampleText);
      expect(chunks).toBeDefined();
      expect(Array.isArray(chunks)).toBe(true);
      expect(chunks.length).toBeGreaterThan(0);
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty text', async () => {
      const splitter = TextSplitter.character(100);
      const chunks = await splitter.split('');
      expect(chunks).toBeDefined();
      expect(Array.isArray(chunks)).toBe(true);
    });

    it('should handle very short text', async () => {
      const splitter = TextSplitter.character(100);
      const chunks = await splitter.split('Short');
      expect(chunks).toBeDefined();
      expect(chunks.length).toBeGreaterThan(0);
    });

    it('should handle text shorter than chunk size', async () => {
      const splitter = TextSplitter.character(1000);
      const chunks = await splitter.split(sampleText);
      expect(chunks).toBeDefined();
      expect(chunks.length).toBeGreaterThan(0);
    });
  });
});
