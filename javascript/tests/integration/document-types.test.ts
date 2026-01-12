/**
 * Document Type Integration Tests
 * 
 * Comprehensive tests for loading different document types (HTML, CSV, XML, TXT)
 * and validating content extraction, metadata parsing, and error handling.
 * 
 * Note: PDF and DOCX tests are excluded as they require binary fixture files.
 * The DocumentLoader supports these formats, but creating valid binary fixtures
 * programmatically is complex. These formats are tested in the Rust core tests.
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { init, DocumentLoader } from '../../index';
import * as path from 'path';
import * as fs from 'fs';

// Initialize GraphBit before running tests
beforeAll(async () => {
  await init();
});

describe('Document Type Integration Tests', () => {
  const fixturesDir = path.join(__dirname, '../fixtures/documents');

  describe('HTML Document Loading', () => {
    it('should load HTML file and extract text content', async () => {
      const filePath = path.join(fixturesDir, 'sample.html');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'html');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.content.length).toBeGreaterThan(0);

      // Validate document type and source
      expect(document.documentType).toBe('html');
      expect(document.source).toBe(filePath);

      // Validate HTML content extraction (text should be extracted from tags)
      expect(document.content).toContain('GraphBit');
      expect(document.content).toContain('Documentation');
    });

    it('should handle HTML with various tags and extract structured content', async () => {
      const filePath = path.join(fixturesDir, 'sample.html');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'html');

      // Validate content contains text from different HTML elements
      expect(document.content).toContain('Features');
      expect(document.content).toContain('Multi-Agent Orchestration');
      expect(document.content).toContain('Workflow Management');
      expect(document.content).toContain('Getting Started');
      expect(document.content).toContain('Conclusion');
    });

    it('should extract metadata from HTML meta tags', async () => {
      const filePath = path.join(fixturesDir, 'sample.html');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'html');

      // Validate metadata is present
      expect(document.metadata).toBeDefined();
      
      // Validate document properties
      expect(document.documentType).toBe('html');
      expect(document.source).toBe(filePath);
      expect(document.content.length).toBeGreaterThan(100);
    });

    it('should handle malformed HTML gracefully', async () => {
      const filePath = path.join(fixturesDir, 'malformed.html');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'html');

      // Should still load and extract content even if HTML is malformed
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.content.length).toBeGreaterThan(0);
      expect(document.documentType).toBe('html');
    });
  });

  describe('CSV Document Loading', () => {
    it('should load CSV file and parse content', async () => {
      const filePath = path.join(fixturesDir, 'sample.csv');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'csv');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.content.length).toBeGreaterThan(0);

      // Validate document type and source
      expect(document.documentType).toBe('csv');
      expect(document.source).toBe(filePath);

      // Validate CSV content (structured format)
      expect(document.content).toContain('CSV Document Content');
      expect(document.content).toContain('Alice');
      expect(document.content).toContain('Bob');
    });

    it('should handle CSV with headers correctly', async () => {
      const filePath = path.join(fixturesDir, 'sample.csv');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'csv');

      // Validate headers are present
      expect(document.content).toContain('name');
      expect(document.content).toContain('age');
      expect(document.content).toContain('city');
      expect(document.content).toContain('occupation');

      // Validate data rows
      expect(document.content).toContain('Engineer');
      expect(document.content).toContain('Designer');
      expect(document.content).toContain('Manager');
    });

    it('should handle CSV with different delimiters (semicolon)', async () => {
      const filePath = path.join(fixturesDir, 'sample-semicolon.csv');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'csv');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.documentType).toBe('csv');

      // Validate semicolon-delimited content
      expect(document.content).toContain('product');
      expect(document.content).toContain('Laptop');
      expect(document.content).toContain('999.99');
    });

    it('should validate CSV metadata and properties', async () => {
      const filePath = path.join(fixturesDir, 'sample.csv');
      const loader = new DocumentLoader();
      
      const document = await loader.loadFile(filePath, 'csv');

      // Validate metadata
      expect(document.metadata).toBeDefined();
      expect(document.source).toBe(filePath);
      expect(document.documentType).toBe('csv');
      
      // Validate content is not empty
      expect(document.content.length).toBeGreaterThan(50);
    });
  });

  describe('XML Document Loading', () => {
    it('should load XML file and extract content', async () => {
      const filePath = path.join(fixturesDir, 'sample.xml');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'xml');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.content.length).toBeGreaterThan(0);

      // Validate document type and source
      expect(document.documentType).toBe('xml');
      expect(document.source).toBe(filePath);

      // Validate XML content
      expect(document.content).toContain('GraphBit');
      expect(document.content).toContain('Configuration');
    });

    it('should handle XML with nested elements', async () => {
      const filePath = path.join(fixturesDir, 'sample.xml');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'xml');

      // Validate nested content is extracted (structured format)
      expect(document.content).toContain('Multi-Agent Orchestration');
      expect(document.content).toContain('Workflow Management');
      expect(document.content).toContain('LLM Integration');
      expect(document.content).toContain('Element: graphbit');
      expect(document.content).toContain('Element: features');
    });

    it('should validate XML metadata and properties', async () => {
      const filePath = path.join(fixturesDir, 'sample.xml');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'xml');

      // Validate metadata
      expect(document.metadata).toBeDefined();
      expect(document.source).toBe(filePath);
      expect(document.documentType).toBe('xml');

      // Validate content length
      expect(document.content.length).toBeGreaterThan(100);
    });
  });

  describe('Text Encoding Handling', () => {
    it('should handle UTF-8 encoded documents correctly', async () => {
      const filePath = path.join(fixturesDir, 'sample-utf8.txt');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'txt');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.documentType).toBe('txt');

      // Validate UTF-8 content
      expect(document.content).toContain('Hello World');
      expect(document.content).toContain('Hola Mundo');
      expect(document.content).toContain('Bonjour le Monde');
    });

    it('should handle special characters and emoji in UTF-8', async () => {
      const filePath = path.join(fixturesDir, 'sample-utf8.txt');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'txt');

      // Validate special characters
      expect(document.content).toContain('©');
      expect(document.content).toContain('€');
      expect(document.content).toContain('™');

      // Validate emoji (if supported)
      expect(document.content).toContain('🌍');
      expect(document.content).toContain('🚀');
      expect(document.content).toContain('💻');
    });

    it('should handle non-Latin scripts in UTF-8', async () => {
      const filePath = path.join(fixturesDir, 'sample-utf8.txt');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'txt');

      // Validate different scripts
      expect(document.content).toContain('Привет мир'); // Russian
      expect(document.content).toContain('你好世界'); // Chinese
      expect(document.content).toContain('こんにちは世界'); // Japanese
      expect(document.content).toContain('مرحبا بالعالم'); // Arabic
    });
  });

  describe('Markdown Document Loading', () => {
    it('should load Markdown file as text and preserve formatting', async () => {
      const filePath = path.join(__dirname, '../fixtures/sample-markdown.md');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'txt');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.documentType).toBe('txt');

      // Validate markdown structure is preserved
      expect(document.content).toContain('# GraphBit Documentation');
      expect(document.content).toContain('## Introduction');
    });

    it('should preserve Markdown code blocks and formatting', async () => {
      const filePath = path.join(__dirname, '../fixtures/sample-markdown.md');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'txt');

      // Validate code blocks are preserved
      expect(document.content).toContain('```typescript');
      expect(document.content).toContain('### Core Capabilities');

      // Validate content length
      expect(document.content.length).toBeGreaterThan(100);
    });
  });

  describe('Error Handling and Edge Cases', () => {
    it('should throw error for non-existent file', async () => {
      const filePath = path.join(fixturesDir, 'non-existent-file.txt');
      const loader = new DocumentLoader();

      await expect(loader.loadFile(filePath, 'txt')).rejects.toThrow();
    });

    it('should throw error for unsupported document type', async () => {
      const filePath = path.join(fixturesDir, 'sample.html');
      const loader = new DocumentLoader();

      await expect(loader.loadFile(filePath, 'unsupported')).rejects.toThrow();
    });

    it('should handle empty file gracefully', async () => {
      // Create a temporary empty file
      const emptyFilePath = path.join(fixturesDir, 'empty.txt');
      fs.writeFileSync(emptyFilePath, '');

      const loader = new DocumentLoader();
      const document = await loader.loadFile(emptyFilePath, 'txt');

      // Should load but have empty content
      expect(document).toBeDefined();
      expect(document.content).toBe('');
      expect(document.documentType).toBe('txt');

      // Clean up
      fs.unlinkSync(emptyFilePath);
    });

    it('should validate document type parameter', async () => {
      const filePath = path.join(fixturesDir, 'sample.html');
      const loader = new DocumentLoader();

      // Test with invalid type
      await expect(loader.loadFile(filePath, 'xyz')).rejects.toThrow();
      await expect(loader.loadFile(filePath, '')).rejects.toThrow();
    });

    it('should handle very large file paths', async () => {
      const longPath = path.join(fixturesDir, 'a'.repeat(200) + '.txt');
      const loader = new DocumentLoader();

      // Should throw error for non-existent file
      await expect(loader.loadFile(longPath, 'txt')).rejects.toThrow();
    });
  });

  describe('DocumentLoader Configuration', () => {
    it('should create DocumentLoader with default configuration', () => {
      const loader = new DocumentLoader();

      expect(loader).toBeDefined();
      expect(loader).toBeInstanceOf(DocumentLoader);
    });

    it('should create DocumentLoader with custom configuration', () => {
      const loader = DocumentLoader.withConfig({
        maxFileSize: 5 * 1024 * 1024, // 5MB
        defaultEncoding: 'utf-8',
        preserveFormatting: true,
      });

      expect(loader).toBeDefined();
      expect(loader).toBeInstanceOf(DocumentLoader);
    });

    it('should load text content directly without file', async () => {
      const loader = new DocumentLoader();
      const textContent = 'This is a test document loaded from text.';

      const document = await loader.loadText(textContent, 'test-source');

      expect(document).toBeDefined();
      expect(document.content).toBe(textContent);
      expect(document.source).toBe('test-source');
      expect(document.documentType).toBe('txt');
    });

    it('should load text content with default source', async () => {
      const loader = new DocumentLoader();
      const textContent = 'Another test document.';

      const document = await loader.loadText(textContent);

      expect(document).toBeDefined();
      expect(document.content).toBe(textContent);
      expect(document.source).toBe('text');
      expect(document.documentType).toBe('txt');
    });
  });

  describe('JSON Document Loading', () => {
    it('should load JSON file and validate structure', async () => {
      const filePath = path.join(__dirname, '../fixtures/sample-data.json');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'json');

      // Validate document was loaded
      expect(document).toBeDefined();
      expect(document.content).toBeDefined();
      expect(document.documentType).toBe('json');

      // Validate JSON can be parsed
      const parsed = JSON.parse(document.content);
      expect(parsed).toBeDefined();
      expect(parsed.title).toBe('GraphBit Test Data');
    });

    it('should validate JSON content structure', async () => {
      const filePath = path.join(__dirname, '../fixtures/sample-data.json');
      const loader = new DocumentLoader();

      const document = await loader.loadFile(filePath, 'json');

      const parsed = JSON.parse(document.content);

      expect(parsed.version).toBe('1.0.0');
      expect(parsed.features).toHaveLength(5);
      expect(parsed.providers).toBeDefined();
      expect(parsed.statistics).toBeDefined();
    });
  });
});

