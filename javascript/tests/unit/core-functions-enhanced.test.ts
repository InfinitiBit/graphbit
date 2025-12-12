/**
 * Unit tests for Enhanced Core Functions
 * 
 * Tests:
 * - init() with configuration options
 * - getSystemInfo()
 * - healthCheck()
 * - configureRuntime()
 */

import { describe, it, expect } from 'vitest';
import { init, getSystemInfo, healthCheck, configureRuntime } from '../../index';

describe('Core Functions - Enhanced', () => {
  describe('init()', () => {
    it('should initialize without options', () => {
      expect(() => init()).not.toThrow();
    });

    it('should initialize with options', () => {
      expect(() => init({
        logLevel: 'info',
        coloredLogs: true,
        logOutput: 'stdout'
      })).not.toThrow();
    });

    it('should accept all log levels', () => {
      const levels = ['trace', 'debug', 'info', 'warn', 'error'];
      
      levels.forEach(level => {
        expect(() => init({ logLevel: level })).not.toThrow();
      });
    });

    it('should accept colored logs option', () => {
      expect(() => init({ coloredLogs: true })).not.toThrow();
      expect(() => init({ coloredLogs: false })).not.toThrow();
    });

    it('should accept log output option', () => {
      expect(() => init({ logOutput: 'stdout' })).not.toThrow();
      expect(() => init({ logOutput: 'stderr' })).not.toThrow();
    });
  });

  describe('getSystemInfo()', () => {
    it('should return system information', () => {
      const info = getSystemInfo();
      
      expect(info).toHaveProperty('os');
      expect(info).toHaveProperty('arch');
      expect(info).toHaveProperty('cpuCount');
      expect(info).toHaveProperty('graphbitVersion');
    });

    it('should have valid OS', () => {
      const info = getSystemInfo();
      expect(typeof info.os).toBe('string');
      expect(info.os.length).toBeGreaterThan(0);
    });

    it('should have valid architecture', () => {
      const info = getSystemInfo();
      expect(typeof info.arch).toBe('string');
      // More permissive check for different architectures
      expect(info.arch.length).toBeGreaterThan(0);
      // Common architectures include: x86_64, x86, aarch64, arm, etc.
    });

    it('should have positive CPU count', () => {
      const info = getSystemInfo();
      expect(info.cpuCount).toBeGreaterThan(0);
    });

    it('should have valid version', () => {
      const info = getSystemInfo();
      expect(typeof info.graphbitVersion).toBe('string');
      expect(info.graphbitVersion).toMatch(/\d+\.\d+\.\d+/);
    });
  });

  describe('healthCheck()', () => {
    it('should return health status', () => {
      const health = healthCheck();
      
      expect(health).toHaveProperty('healthy');
      expect(health).toHaveProperty('timestamp');
      expect(health).toHaveProperty('version');
    });

    it('should report as healthy', () => {
      const health = healthCheck();
      expect(health.healthy).toBe(true);
    });

    it('should have valid timestamp', () => {
      const health = healthCheck();
      const now = Date.now() / 1000;
      
      expect(health.timestamp).toBeGreaterThan(0);
      expect(health.timestamp).toBeLessThanOrEqual(now + 1);
    });

    it('should include version', () => {
      const health = healthCheck();
      expect(typeof health.version).toBe('string');
      expect(health.version).toMatch(/\d+\.\d+\.\d+/);
    });
  });

  describe('configureRuntime()', () => {
    it('should accept runtime configuration', () => {
      expect(() => configureRuntime({
        maxThreads: 4,
        enableMonitoring: true,
        memoryLimitMb: 2048
      })).not.toThrow();
    });

    it('should accept partial configuration', () => {
      expect(() => configureRuntime({ maxThreads: 2 })).not.toThrow();
      expect(() => configureRuntime({ enableMonitoring: true })).not.toThrow();
      expect(() => configureRuntime({ memoryLimitMb: 1024 })).not.toThrow();
    });

    it('should accept empty configuration', () => {
      expect(() => configureRuntime({})).not.toThrow();
    });
  });

  describe('Integration - Production Setup', () => {
    it('should support complete initialization flow', () => {
      // Initialize
      init({
        logLevel: 'warn',
        coloredLogs: false,
        logOutput: 'stderr'
      });

      // Configure runtime
      configureRuntime({
        maxThreads: 4,
        enableMonitoring: true,
        memoryLimitMb: 2048
      });

      // Check system
      const info = getSystemInfo();
      expect(info.cpuCount).toBeGreaterThan(0);

      // Verify health
      const health = healthCheck();
      expect(health.healthy).toBe(true);
    });
  });

  describe('API Contract', () => {
    it('should have correct function signatures', () => {
      expect(typeof init).toBe('function');
      expect(typeof getSystemInfo).toBe('function');
      expect(typeof healthCheck).toBe('function');
      expect(typeof configureRuntime).toBe('function');
    });
  });
});

