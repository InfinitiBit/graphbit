import { describe, it, expect } from 'vitest';
import { validateJson } from '../../index';

describe('validateJson', () => {
  describe('valid JSON validation', () => {
    it('should validate simple object against schema', () => {
      const data = JSON.stringify({ name: 'John', age: 30 });
      const schema = JSON.stringify({
        type: 'object',
        properties: {
          name: { type: 'string' },
          age: { type: 'number' },
        },
        required: ['name', 'age'],
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate array against schema', () => {
      const data = JSON.stringify([1, 2, 3, 4, 5]);
      const schema = JSON.stringify({
        type: 'array',
        items: { type: 'number' },
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate nested object against schema', () => {
      const data = JSON.stringify({
        user: {
          name: 'Alice',
          email: 'alice@example.com',
        },
      });
      const schema = JSON.stringify({
        type: 'object',
        properties: {
          user: {
            type: 'object',
            properties: {
              name: { type: 'string' },
              email: { type: 'string' },
            },
          },
        },
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate string against schema', () => {
      const data = JSON.stringify('hello world');
      const schema = JSON.stringify({ type: 'string' });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate number against schema', () => {
      const data = JSON.stringify(42);
      const schema = JSON.stringify({ type: 'number' });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate boolean against schema', () => {
      const data = JSON.stringify(true);
      const schema = JSON.stringify({ type: 'boolean' });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate null against schema', () => {
      const data = JSON.stringify(null);
      const schema = JSON.stringify({ type: 'null' });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });
  });

  describe('invalid JSON validation', () => {
    it('should reject object with wrong type', () => {
      const data = JSON.stringify({ name: 'John', age: '30' }); // age is string, not number
      const schema = JSON.stringify({
        type: 'object',
        properties: {
          name: { type: 'string' },
          age: { type: 'number' },
        },
        required: ['name', 'age'],
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it('should reject object with missing required field', () => {
      const data = JSON.stringify({ name: 'John' }); // missing age
      const schema = JSON.stringify({
        type: 'object',
        properties: {
          name: { type: 'string' },
          age: { type: 'number' },
        },
        required: ['name', 'age'],
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it('should reject array with wrong item type', () => {
      const data = JSON.stringify([1, 2, 'three', 4]); // 'three' is string, not number
      const schema = JSON.stringify({
        type: 'array',
        items: { type: 'number' },
      });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it('should reject wrong primitive type', () => {
      const data = JSON.stringify(42);
      const schema = JSON.stringify({ type: 'string' });

      const result = validateJson(data, schema);
      expect(result).toBeDefined();
      expect(result.isValid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });
});

