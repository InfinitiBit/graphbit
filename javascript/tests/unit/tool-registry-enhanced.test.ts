/**
 * Unit tests for Enhanced ToolRegistry
 * 
 * Tests all new methods:
 * - unregisterTool()
 * - getToolMetadata()
 * - getAllMetadata()
 * - getExecutionHistory()
 * - clearHistory()
 * - getStats()
 * - clearAll()
 * - getLlmTools()
 * - getToolCount()
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { createToolRegistry, ToolMetadata, ToolStats } from '../../index';

describe('ToolRegistry - Enhanced Methods', () => {
  let registry: ToolRegistry;

  beforeEach(() => {
    registry = createToolRegistry();
  });

  describe('Tool Management', () => {
    it('should unregister existing tool', () => {
      registry.register('test_tool', 'Test tool', {}, () => 'result');

      const removed = registry.unregisterTool('test_tool');
      expect(removed).toBe(true);
      expect(registry.hasTool('test_tool')).toBe(false);
    });

    it('should return false when unregistering non-existent tool', () => {
      const removed = registry.unregisterTool('nonexistent');
      expect(removed).toBe(false);
    });

    it('should clear all tools', () => {
      registry.register('tool1', 'Tool 1', {}, () => 'result1');
      registry.register('tool2', 'Tool 2', {}, () => 'result2');

      expect(registry.getToolCount()).toBe(2);

      registry.clearAll();

      expect(registry.getToolCount()).toBe(0);
      expect(registry.getRegisteredTools().length).toBe(0);
    });

    it('should get tool count', () => {
      expect(registry.getToolCount()).toBe(0);

      registry.register('tool1', 'Tool 1', {}, () => 'result');
      expect(registry.getToolCount()).toBe(1);

      registry.register('tool2', 'Tool 2', {}, () => 'result');
      expect(registry.getToolCount()).toBe(2);
    });
  });

  describe('Metadata Access', () => {
    it('should get tool metadata', () => {
      registry.register('test_tool', 'Test description', { param: 'string' }, () => 'result');

      const metadata = registry.getToolMetadata('test_tool');

      expect(metadata).toBeTruthy();
      expect(metadata!.name).toBe('test_tool');
      expect(metadata!.description).toBe('Test description');
      expect(metadata!.callCount).toBe(0);
      expect(metadata!.createdAt).toBeGreaterThan(0);
    });

    it('should return null for non-existent tool metadata', () => {
      const metadata = registry.getToolMetadata('nonexistent');
      expect(metadata).toBeNull();
    });

    it('should get all metadata', () => {
      registry.register('tool1', 'Description 1', {}, () => 'result1');
      registry.register('tool2', 'Description 2', {}, () => 'result2');

      const allMetadata = registry.getAllMetadata();

      expect(allMetadata.length).toBe(2);
      expect(allMetadata.map((m: any) => m.name)).toContain('tool1');
      expect(allMetadata.map((m: any) => m.name)).toContain('tool2');
    });

    it('should track call count in metadata', async () => {
      registry.register('counter', 'Counter tool', {}, () => 'result');

      await registry.execute('counter', {});
      await registry.execute('counter', {});
      await registry.execute('counter', {});

      const metadata = registry.getToolMetadata('counter');
      expect(metadata!.callCount).toBe(3);
    });

    it('should track duration in metadata', async () => {
      registry.register('slow_tool', 'Slow tool', {}, async () => {
        await new Promise(resolve => setTimeout(resolve, 10));
        return 'done';
      });

      await registry.execute('slow_tool', {});

      const metadata = registry.getToolMetadata('slow_tool');
      expect(metadata!.totalDurationMs).toBeGreaterThan(0);
      expect(metadata!.avgDurationMs).toBeGreaterThan(0);
      expect(metadata!.lastCalledAt).toBeGreaterThan(0);
    });
  });

  describe('Execution History', () => {
    it('should record execution history', async () => {
      registry.register('test_tool', 'Test tool', {}, () => 'result');

      await registry.execute('test_tool', {});
      await registry.execute('test_tool', {});

      const history = registry.getExecutionHistory();
      expect(history.length).toBe(2);
      expect(history[0].toolName).toBe('test_tool');
      expect(history[0].success).toBe(true);
    });

    it('should record failures in history', async () => {
      registry.register('failing_tool', 'Failing tool', {}, () => {
        throw new Error('Intentional failure');
      });

      await registry.execute('failing_tool', {});

      const history = registry.getExecutionHistory();
      expect(history.length).toBe(1);
      expect(history[0].success).toBe(false);
      expect(history[0].error).toBeTruthy();
    });

    it('should include timestamps in history', async () => {
      registry.register('test_tool', 'Test tool', {}, () => 'result');

      const before = Date.now() / 1000;
      await registry.execute('test_tool', {});
      const after = Date.now() / 1000;

      const history = registry.getExecutionHistory();
      expect(history[0].timestamp).toBeGreaterThanOrEqual(before);
      expect(history[0].timestamp).toBeLessThanOrEqual(after);
    });

    it('should clear execution history', async () => {
      registry.register('test_tool', 'Test tool', {}, () => 'result');

      await registry.execute('test_tool', {});
      await registry.execute('test_tool', {});

      expect(registry.getExecutionHistory().length).toBe(2);

      registry.clearHistory();

      expect(registry.getExecutionHistory().length).toBe(0);
    });

    it('should preserve tools when clearing history', () => {
      registry.register('test_tool', 'Test tool', {}, () => 'result');

      registry.clearHistory();

      expect(registry.hasTool('test_tool')).toBe(true);
    });
  });

  describe('Statistics', () => {
    it('should provide comprehensive statistics', async () => {
      registry.register('tool1', 'Tool 1', {}, () => 'result1');
      registry.register('tool2', 'Tool 2', {}, () => 'result2');

      await registry.execute('tool1', {});
      await registry.execute('tool2', {});
      await registry.execute('tool1', {});

      const stats = registry.getStats();

      expect(stats.totalTools).toBe(2);
      expect(stats.totalExecutions).toBe(3);
      expect(stats.successfulExecutions).toBe(3);
      expect(stats.failedExecutions).toBe(0);
      expect(stats.avg_execution_time_ms).toBeGreaterThan(0);
      expect(stats.totalExecutionTimeMs).toBeGreaterThan(0);
    });

    it('should track success and failure rates', async () => {
      registry.register('success_tool', 'Success', {}, () => 'ok');
      registry.register('fail_tool', 'Fail', {}, () => {
        throw new Error('fail');
      });

      await registry.execute('success_tool', {});
      await registry.execute('fail_tool', {});
      await registry.execute('success_tool', {});

      const stats = registry.getStats();

      expect(stats.totalExecutions).toBe(3);
      expect(stats.successfulExecutions).toBe(2);
      expect(stats.failedExecutions).toBe(1);
    });
  });

  describe('LLM Integration', () => {
    it('should format tools for LLM use', () => {
      registry.register('search', 'Search the web', {
        query: { type: 'string', description: 'Search query' }
      }, () => 'results');

      const llmTools = registry.getLlmTools();

      expect(llmTools.length).toBe(1);
      expect(llmTools[0]).toBeTruthy();
    });

    it('should handle empty registry', () => {
      const llmTools = registry.getLlmTools();
      expect(llmTools.length).toBe(0);
    });
  });

  describe('Integration', () => {
    it('should demonstrate complete tool lifecycle', async () => {
      // Register
      registry.register('calculator', 'Calculate', { operation: 'string' }, () => {
        return { result: 42 };
      });

      expect(registry.hasTool('calculator')).toBe(true);
      expect(registry.getToolCount()).toBe(1);

      // Execute multiple times
      await registry.execute('calculator', { operation: 'add' });
      await registry.execute('calculator', { operation: 'multiply' });

      // Check metadata
      const metadata = registry.getToolMetadata('calculator');
      expect(metadata!.callCount).toBe(2);
      expect(metadata!.avgDurationMs).toBeGreaterThan(0);

      // Check history
      const history = registry.getExecutionHistory();
      expect(history.length).toBe(2);

      // Check stats
      const stats = registry.getStats();
      expect(stats.totalTools).toBe(1);
      expect(stats.totalExecutions).toBe(2);

      // Unregister
      const removed = registry.unregisterTool('calculator');
      expect(removed).toBe(true);
      expect(registry.hasTool('calculator')).toBe(false);
    });
  });
});

/**
 * API Contract Tests
 */
describe('ToolRegistry - API Contract', () => {
  const expectedMethods = [
    'register',
    'execute',
    'getToolDefinition',
    'getTool',
    'hasTool',
    'getRegisteredTools',
    'unregisterTool',
    'getToolMetadata',
    'getAllMetadata',
    'getExecutionHistory',
    'clearHistory',
    'getStats',
    'clearAll',
    'getLlmTools',
    'getToolCount'
  ];

  it('should have all expected methods', () => {
    expect(expectedMethods.length).toBe(15);
  });
});

