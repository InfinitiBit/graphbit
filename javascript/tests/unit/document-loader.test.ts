import { describe, it, expect } from 'vitest';
import { DocumentLoader } from '../../index';

describe('DocumentLoader', () => {
  describe('constructor', () => {
    it('should create a document loader with default configuration', () => {
      const loader = new DocumentLoader();
      expect(loader).toBeDefined();
    });
  });

  describe('withConfig', () => {
    it('should create a document loader with custom configuration', () => {
      const loader = DocumentLoader.withConfig({
        maxFileSize: 10485760, // 10MB
        defaultEncoding: 'utf-8',
        preserveFormatting: true,
      });
      expect(loader).toBeDefined();
    });

    it('should create a document loader with partial configuration', () => {
      const loader = DocumentLoader.withConfig({
        maxFileSize: 5242880, // 5MB
      });
      expect(loader).toBeDefined();
    });

    it('should create a document loader with preserve formatting disabled', () => {
      const loader = DocumentLoader.withConfig({
        preserveFormatting: false,
      });
      expect(loader).toBeDefined();
    });
  });

  describe('loadText', () => {
    it('should load text content', async () => {
      const loader = new DocumentLoader();
      const text = 'This is a test document.';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });

    it('should load text content with source', async () => {
      const loader = new DocumentLoader();
      const text = 'This is a test document.';
      const source = 'test-source';
      
      const doc = await loader.loadText(text, source);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
      expect(doc.source).toBe(source);
    });

    it('should load text content without source', async () => {
      const loader = new DocumentLoader();
      const text = 'This is a test document.';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });

    it('should load empty text', async () => {
      const loader = new DocumentLoader();
      const text = '';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });

    it('should load multiline text', async () => {
      const loader = new DocumentLoader();
      const text = 'Line 1\nLine 2\nLine 3';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });

    it('should load text with special characters', async () => {
      const loader = new DocumentLoader();
      const text = 'Special chars: !@#$%^&*()_+-=[]{}|;:,.<>?';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });

    it('should load text with unicode characters', async () => {
      const loader = new DocumentLoader();
      const text = 'Unicode: 你好世界 🌍 café';
      
      const doc = await loader.loadText(text);
      expect(doc).toBeDefined();
      expect(doc?.content).toBe(text);
    });
  });

  describe('loadFile', () => {
    it('should reject loading non-existent file', async () => {
      const loader = new DocumentLoader();
      
      await expect(
        loader.loadFile('/non/existent/file.txt', 'text')
      ).rejects.toThrow();
    });

    it('should have loadFile method', () => {
      const loader = new DocumentLoader();
      expect(loader.loadFile).toBeDefined();
      expect(typeof loader.loadFile).toBe('function');
    });
  });

  describe('configuration options', () => {
    it('should accept maxFileSize configuration', () => {
      const loader = DocumentLoader.withConfig({
        maxFileSize: 1048576, // 1MB
      });
      expect(loader).toBeDefined();
    });

    it('should accept defaultEncoding configuration', () => {
      const loader = DocumentLoader.withConfig({
        defaultEncoding: 'utf-8',
      });
      expect(loader).toBeDefined();
    });

    it('should accept preserveFormatting configuration', () => {
      const loader = DocumentLoader.withConfig({
        preserveFormatting: true,
      });
      expect(loader).toBeDefined();
    });

    it('should accept all configuration options', () => {
      const loader = DocumentLoader.withConfig({
        maxFileSize: 2097152, // 2MB
        defaultEncoding: 'utf-8',
        preserveFormatting: false,
      });
      expect(loader).toBeDefined();
    });
  });
});

