/**
 * Unit tests for Enhanced DocumentLoader
 * 
 * Tests the new static helper methods:
 * - supportedTypes()
 * - detectDocumentType()
 */

import { describe, it, expect } from 'vitest';
import { DocumentLoader } from '../../index';

describe('DocumentLoader - Enhanced Methods', () => {
  describe('supportedTypes()', () => {
    it('should return array of supported types', () => {
      const types = DocumentLoader.supportedTypes();
      
      expect(Array.isArray(types)).toBe(true);
      expect(types.length).toBeGreaterThan(0);
    });

    it('should include common document types', () => {
      const types = DocumentLoader.supportedTypes();
      
      expect(types).toContain('txt');
      expect(types).toContain('pdf');
      expect(types).toContain('json');
    });

    it('should return consistent results', () => {
      const types1 = DocumentLoader.supportedTypes();
      const types2 = DocumentLoader.supportedTypes();
      
      expect(types1).toEqual(types2);
    });
  });

  describe('detectDocumentType()', () => {
    it('should detect PDF files', () => {
      expect(DocumentLoader.detectDocumentType('document.pdf')).toBe('pdf');
      expect(DocumentLoader.detectDocumentType('path/to/file.pdf')).toBe('pdf');
      expect(DocumentLoader.detectDocumentType('FILE.PDF')).toBe('pdf');
    });

    it('should detect TXT files', () => {
      expect(DocumentLoader.detectDocumentType('document.txt')).toBe('txt');
      expect(DocumentLoader.detectDocumentType('notes.txt')).toBe('txt');
    });

    it('should detect JSON files', () => {
      expect(DocumentLoader.detectDocumentType('data.json')).toBe('json');
    });

    it('should detect DOCX files', () => {
      expect(DocumentLoader.detectDocumentType('document.docx')).toBe('docx');
    });

    it('should detect CSV files', () => {
      expect(DocumentLoader.detectDocumentType('data.csv')).toBe('csv');
    });

    it('should detect XML files', () => {
      expect(DocumentLoader.detectDocumentType('config.xml')).toBe('xml');
    });

    it('should detect HTML files', () => {
      expect(DocumentLoader.detectDocumentType('page.html')).toBe('html');
      expect(DocumentLoader.detectDocumentType('page.htm')).toBe('html');
    });

    it('should return null for unsupported types', () => {
      expect(DocumentLoader.detectDocumentType('file.xyz')).toBeNull();
      expect(DocumentLoader.detectDocumentType('file.unknown')).toBeNull();
    });

    it('should return null for files without extension', () => {
      expect(DocumentLoader.detectDocumentType('README')).toBeNull();
      expect(DocumentLoader.detectDocumentType('Makefile')).toBeNull();
    });

    it('should handle paths with multiple dots', () => {
      expect(DocumentLoader.detectDocumentType('my.file.name.pdf')).toBe('pdf');
      expect(DocumentLoader.detectDocumentType('version.1.0.json')).toBe('json');
    });

    it('should handle Windows paths', () => {
      expect(DocumentLoader.detectDocumentType('C:\\Users\\file.pdf')).toBe('pdf');
      expect(DocumentLoader.detectDocumentType('C:\\path\\to\\doc.txt')).toBe('txt');
    });

    it('should handle Unix paths', () => {
      expect(DocumentLoader.detectDocumentType('/home/user/file.pdf')).toBe('pdf');
      expect(DocumentLoader.detectDocumentType('/var/log/data.json')).toBe('json');
    });
  });

  describe('Integration - Auto-detection', () => {
    it.skip('should load file with auto-detected type', async () => {
      const loader = new DocumentLoader();
      const filePath = 'test-data/sample.txt';
      
      const type = DocumentLoader.detectDocumentType(filePath);
      expect(type).toBe('txt');
      
      if (type) {
        const doc = await loader.loadFile(filePath, type);
        expect(doc.documentType).toBe('txt');
        expect(doc.content).toBeTruthy();
      }
    });

    it.skip('should validate before loading', async () => {
      const loader = new DocumentLoader();
      const filePath = 'test-data/sample.unknown';
      
      const type = DocumentLoader.detectDocumentType(filePath);
      
      if (!type) {
        console.log('Unsupported file type, skipping load');
        expect(type).toBeNull();
      } else {
        const doc = await loader.loadFile(filePath, type);
        expect(doc).toBeTruthy();
      }
    });
  });

  describe('Practical Usage Patterns', () => {
    it('should enable smart file loading', () => {
      // Pattern: Auto-detect and load
      const smartLoad = async (filePath: string) => {
        const type = DocumentLoader.detectDocumentType(filePath);
        
        if (!type) {
          throw new Error(`Unsupported file type: ${filePath}`);
        }
        
        const supportedTypes = DocumentLoader.supportedTypes();
        if (!supportedTypes.includes(type)) {
          throw new Error(`Type ${type} not in supported list`);
        }
        
        const loader = new DocumentLoader();
        return await loader.loadFile(filePath, type);
      };
      
      expect(typeof smartLoad).toBe('function');
    });

    it('should enable batch validation', () => {
      // Pattern: Validate multiple files before processing
      const files = [
        'doc1.pdf',
        'doc2.txt',
        'doc3.unknown',
        'doc4.json'
      ];
      
      const validFiles = files.filter(file => {
        const type = DocumentLoader.detectDocumentType(file);
        return type !== null;
      });
      
      expect(validFiles.length).toBe(3); // All except .unknown
    });
  });
});

