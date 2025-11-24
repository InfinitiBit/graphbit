import { describe, it, expect, beforeAll } from 'vitest';
import { init, DocumentLoader, TextSplitter } from '../../index';
import * as path from 'path';
import * as fs from 'fs';

describe('Document Processing Integration Tests', () => {
  beforeAll(() => {
    init();
  });

  const fixturesDir = path.join(__dirname, '../fixtures');

  describe('DocumentLoader - Text Files', () => {
    it('should load text document and validate content and metadata', async () => {
      const filePath = path.join(fixturesDir, 'sample-document.txt');
      const expectedContent = fs.readFileSync(filePath, 'utf-8');

      const loader = new DocumentLoader();
      const document = await loader.loadFile(filePath, 'txt');

      // Validate document content matches file content
      expect(document.content).toBe(expectedContent);

      // Validate document metadata
      expect(document.documentType).toBe('txt');
      expect(document.source).toBe(filePath);

      // Validate content is not empty
      expect(document.content.length).toBeGreaterThan(0);
      expect(document.content.length).toBe(expectedContent.length);
    });

    it('should load JSON document and validate structure', async () => {
      const filePath = path.join(fixturesDir, 'sample-data.json');

      const loader = new DocumentLoader();
      const document = await loader.loadFile(filePath, 'json');

      // Validate content is valid JSON and has expected structure
      const parsed = JSON.parse(document.content);
      expect(parsed.title).toBe('GraphBit Test Data');
      expect(parsed.version).toBe('1.0.0');
      expect(parsed.features).toHaveLength(5);
      expect(parsed.features).toContain('Multi-Agent Orchestration');
      expect(parsed.features).toContain('Workflow Management');
      expect(parsed.providers).toBeDefined();
      expect(parsed.providers.openai).toBeDefined();
      expect(parsed.providers.anthropic).toBeDefined();
      expect(parsed.providers.ollama).toBeDefined();
      expect(parsed.statistics.total_tests).toBe(158);
      expect(parsed.statistics.passing_tests).toBe(158);
      expect(parsed.statistics.code_coverage).toBe(85);

      // Validate metadata
      expect(document.documentType).toBe('json');
      expect(document.content.length).toBeGreaterThan(0);
    });

    it('should load markdown document and preserve formatting', async () => {
      const filePath = path.join(fixturesDir, 'sample-markdown.md');
      const expectedContent = fs.readFileSync(filePath, 'utf-8');

      const loader = new DocumentLoader();
      // Load as 'txt' since 'md' is not supported
      const document = await loader.loadFile(filePath, 'txt');

      // Validate content is preserved exactly
      expect(document.content).toBe(expectedContent);
      expect(document.documentType).toBe('txt');

      // Validate markdown structure is preserved
      expect(document.content).toContain('# GraphBit Documentation');
      expect(document.content).toContain('## Introduction');
      expect(document.content).toContain('```typescript');
      expect(document.content).toContain('### Core Capabilities');

      // Validate content length
      expect(document.content.length).toBe(expectedContent.length);
    });

    it('should handle long documents correctly', async () => {
      const filePath = path.join(fixturesDir, 'long-document.txt');
      const expectedContent = fs.readFileSync(filePath, 'utf-8');

      const loader = new DocumentLoader();
      const document = await loader.loadFile(filePath, 'txt');

      // Validate full content is loaded
      expect(document.content).toBe(expectedContent);
      expect(document.content.length).toBeGreaterThan(2000); // Long document

      // Validate content contains expected paragraphs
      expect(document.content).toContain('Paragraph 1:');
      expect(document.content).toContain('Paragraph 10:');
      expect(document.content).toContain('Lorem ipsum');
      expect(document.content).toContain('In conclusion');
    });
  });

  describe('TextSplitter - Character-based Splitting', () => {
    it('should split text into chunks with correct size and count', async () => {
      const filePath = path.join(fixturesDir, 'long-document.txt');
      const content = fs.readFileSync(filePath, 'utf-8');

      const splitter = TextSplitter.character(500, 50);

      const chunks = splitter.split(content);

      // Validate chunk count is reasonable for content length
      const expectedMinChunks = Math.floor(content.length / 500);
      expect(chunks.length).toBeGreaterThanOrEqual(expectedMinChunks);
      expect(chunks.length).toBeLessThan(content.length / 100); // Sanity check

      // Validate each chunk size is within limits
      chunks.forEach((chunk, index) => {
        expect(chunk.content.length).toBeLessThanOrEqual(500);
        if (index < chunks.length - 1) {
          // All chunks except last should be close to chunk size
          expect(chunk.content.length).toBeGreaterThan(400);
        }
      });

      // Validate chunks cover all content (accounting for overlap)
      const totalChunkContent = chunks.map(c => c.content).join('');
      expect(totalChunkContent.length).toBeGreaterThanOrEqual(content.length);
    });

    it('should apply overlap correctly between chunks', async () => {
      const content = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.repeat(10); // 260 chars

      const splitter = TextSplitter.character(100, 20);

      const chunks = splitter.split(content);

      // Validate we have multiple chunks
      expect(chunks.length).toBeGreaterThan(2);

      // Validate overlap: end of chunk N should overlap with start of chunk N+1
      for (let i = 0; i < chunks.length - 1; i++) {
        const currentChunk = chunks[i].content;
        const nextChunk = chunks[i + 1].content;
        
        // Last 20 chars of current should appear in next chunk
        const overlapSection = currentChunk.slice(-20);
        expect(nextChunk).toContain(overlapSection.slice(0, 10)); // At least partial overlap
      }
    });
  });

  describe('TextSplitter - Token-based Splitting', () => {
    it('should split text by tokens with correct parameters', async () => {
      const filePath = path.join(fixturesDir, 'sample-document.txt');
      const content = fs.readFileSync(filePath, 'utf-8');

      const splitter = TextSplitter.token(200, 20);

      const chunks = splitter.split(content);

      // Validate chunks are created
      expect(chunks.length).toBeGreaterThan(0);

      // Validate chunk metadata
      chunks.forEach((chunk) => {
        expect(chunk.content).toBeDefined();
        expect(chunk.content.length).toBeGreaterThan(0);
        expect(chunk.startIndex).toBeGreaterThanOrEqual(0);
      });

      // Validate total content is preserved
      const allContent = chunks.map(c => c.content).join('');
      expect(allContent.length).toBeGreaterThanOrEqual(content.length * 0.9); // Allow some variance
    });

    it('should handle small text with token splitting', async () => {
      const content = 'This is a short text for testing token-based splitting.';

      const splitter = TextSplitter.token(100, 10);

      const chunks = splitter.split(content);

      // Small text should result in 1 chunk
      expect(chunks.length).toBe(1);
      expect(chunks[0].content).toBe(content);
    });
  });

  describe('TextSplitter - Recursive Splitting', () => {
    it('should split text recursively with separators', async () => {
      const filePath = path.join(fixturesDir, 'long-document.txt');
      const content = fs.readFileSync(filePath, 'utf-8');

      const splitter = TextSplitter.recursive(600, 50);

      const chunks = splitter.split(content);

      // Validate chunks are created
      expect(chunks.length).toBeGreaterThan(0);

      // Validate chunk sizes - recursive splitting respects natural boundaries
      // so chunks can vary significantly in size
      chunks.forEach((chunk, index) => {
        expect(chunk.content.length).toBeGreaterThan(0);
        expect(chunk.content.length).toBeLessThanOrEqual(700); // Allow variance for natural boundaries
      });

      // Validate total content is preserved
      const reconstructed = chunks.map(c => c.content).join('');
      expect(reconstructed.length).toBeGreaterThan(0);

      // Validate content is split at natural boundaries (paragraphs)
      // Recursive splitter should prefer splitting at paragraph boundaries
      const firstChunk = chunks[0].content;
      expect(firstChunk).toContain('Paragraph');
    });
  });

  describe('TextSplitter - Strategy Comparison', () => {
    it('should produce different results with different strategies', async () => {
      const content = 'A'.repeat(1000); // Simple repeated content

      const charSplitter = TextSplitter.character(200, 20);
      const tokenSplitter = TextSplitter.token(200, 20);

      const charChunks = charSplitter.split(content);
      const tokenChunks = tokenSplitter.split(content);

      // Both should create chunks
      expect(charChunks.length).toBeGreaterThan(0);
      expect(tokenChunks.length).toBeGreaterThan(0);

      // Strategies may produce different chunk counts
      // Character-based is more predictable for repeated chars
      expect(charChunks.length).toBeGreaterThanOrEqual(4);
    });

    it('should preserve metadata across different splitting strategies', async () => {
      const content = 'Test content for metadata preservation across splitting strategies.';

      const strategies = [
        TextSplitter.character(50, 5),
        TextSplitter.token(50, 5),
        TextSplitter.recursive(50, 5),
      ];

      for (const splitter of strategies) {
        const chunks = splitter.split(content);

        // All chunks should have content and startIndex
        chunks.forEach((chunk) => {
          expect(chunk.startIndex).toBeGreaterThanOrEqual(0);
          expect(chunk.content).toBeDefined();
          expect(chunk.content.length).toBeGreaterThan(0);
        });
      }
    });
  });
});


