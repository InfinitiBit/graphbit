# Monitoring and Observability - JavaScript

This guide covers monitoring and observability for GraphBit JavaScript applications, including metrics collection, execution tracking, health monitoring, and best practices for production environments.

## Overview

GraphBit monitoring for JavaScript/Node.js includes:
- **Execution Metrics**: Performance tracking and timing data
- **System Health**: Resource usage and availability monitoring
- **Error Tracking**: Failure detection and analysis
- **Custom Metrics**: Business logic and application-specific metrics
- **Real-time Monitoring**: Live execution tracking

## Basic Monitoring Setup

### Core Metrics Collection

```typescript
import { init, Workflow, Node, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

interface WorkflowMetrics {
  workflowId: string;
  workflowName: string;
  executionId: string;
  startTime: Date;
  endTime?: Date;
  durationMs?: number;
  status: 'running' | 'completed' | 'failed';
  nodeCount: number;
  nodesExecuted: number;
  errorMessage?: string;
}

class WorkflowMonitor {
  private metricsStore: WorkflowMetrics[] = [];
  private activeExecutions: Map<string, WorkflowMetrics> = new Map();

  /**
   * Start monitoring a workflow execution
   */
  startExecution(workflow: Workflow, executionId?: string): string {
    if (!executionId) {
      executionId = `exec_${Date.now()}_${Math.random().toString(36).slice(2, 10)}`;
    }

    const metrics: WorkflowMetrics = {
      workflowId: Math.random().toString(36),
      workflowName: workflow.constructor.name,
      executionId,
      startTime: new Date(),
      status: 'running',
      nodeCount: 0, // Will be updated during execution
      nodesExecuted: 0
    };

    this.activeExecutions.set(executionId, metrics);
    return executionId;
  }

  /**
   * End monitoring a workflow execution
   */
  endExecution(
    executionId: string,
    status: 'completed' | 'failed' = 'completed',
    errorMessage?: string
  ): void {
    const metrics = this.activeExecutions.get(executionId);
    if (!metrics) return;

    metrics.endTime = new Date();
    metrics.durationMs = metrics.endTime.getTime() - metrics.startTime.getTime();
    metrics.status = status;
    metrics.errorMessage = errorMessage;

    // Assume all nodes executed if not tracked
    if (metrics.nodesExecuted === 0) {
      metrics.nodesExecuted = metrics.nodeCount;
    }

    this.metricsStore.push(metrics);
    this.activeExecutions.delete(executionId);
  }

  /**
   * Get metrics summary for specified time window
   */
  getMetricsSummary(timeWindowHours: number = 24): any {
    const cutoffTime = new Date(Date.now() - timeWindowHours * 60 * 60 * 1000);
    const recentMetrics = this.metricsStore.filter(
      m => m.startTime > cutoffTime
    );

    if (recentMetrics.length === 0) {
      return { message: 'No metrics in time window' };
    }

    const totalExecutions = recentMetrics.length;
    const successfulExecutions = recentMetrics.filter(
      m => m.status === 'completed'
    ).length;
    const failedExecutions = recentMetrics.filter(
      m => m.status === 'failed'
    ).length;

    const durations = recentMetrics
      .filter(m => m.durationMs !== undefined)
      .map(m => m.durationMs!);

    const avgDuration = durations.length > 0
      ? durations.reduce((a, b) => a + b, 0) / durations.length
      : 0;

    const maxDuration = durations.length > 0 ? Math.max(...durations) : 0;
    const minDuration = durations.length > 0 ? Math.min(...durations) : 0;

    return {
      timeWindowHours,
      totalExecutions,
      successfulExecutions,
      failedExecutions,
      successRate: (successfulExecutions / totalExecutions) * 100,
      avgDurationMs: Math.round(avgDuration),
      maxDurationMs: maxDuration,
      minDurationMs: minDuration
    };
  }

  /**
   * Get all stored metrics
   */
  getAllMetrics(): WorkflowMetrics[] {
    return [...this.metricsStore];
  }

  /**
   * Get active executions
   */
  getActiveExecutions(): WorkflowMetrics[] {
    return Array.from(this.activeExecutions.values());
  }
}
```

### Usage Example

```typescript
async function monitoredWorkflowExecution() {
  init();

  const monitor = new WorkflowMonitor();
  
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const workflow = new Workflow('MonitoredWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  const executor = new Executor(config);
  const executionId = monitor.startExecution(workflow);

  try {
    console.log(`Starting execution ${executionId}...`);
    const result = await executor.execute(workflow);

    if (result.isSuccess()) {
      monitor.endExecution(executionId, 'completed');
      console.log('✅ Execution successful');
    } else {
      monitor.endExecution(executionId, 'failed', result.error());
      console.error('❌ Execution failed:', result.error());
    }
  } catch (error) {
    monitor.endExecution(
      executionId, 
      'failed', 
      error instanceof Error ? error.message : 'Unknown error'
    );
    console.error('❌ Execution error:', error);
  }

  // Get summary
  const summary = monitor.getMetricsSummary(24);
  console.log('Metrics Summary:', summary);
}

monitoredWorkflowExecution().catch(console.error);
```

## System Health Monitoring

### Health Check API

```typescript
import { init, healthCheck, getSystemInfo } from '@infinitibit_gmbh/graphbit';

async function checkSystemHealth() {
  init();

  const health = healthCheck();
  console.log('System Health Check:');
  console.log(`  Overall healthy: ${health.overallHealthy}`);
  console.log(`  Memory status: ${health.memoryHealthy ? 'OK' : 'WARNING'}`);
  console.log(`  CPU status: ${health.cpuHealthy ? 'OK' : 'WARNING'}`);

  const info = getSystemInfo();
  console.log('\nSystem Info:');
  console.log(`  Node.js version: ${info.nodeVersion}`);
  console.log(`  CPU count: ${info.cpuCount}`);
  console.log(`  Platform: ${process.platform}`);
}

checkSystemHealth().catch(console.error);
```

### Periodic Health Monitoring

```typescript
class HealthMonitor {
  private intervalId?: NodeJS.Timeout;
  private healthHistory: Array<{ timestamp: Date; healthy: boolean }> = [];

  /**
   * Start periodic health checks
   */
  start(intervalSeconds: number = 60): void {
    init();

    this.intervalId = setInterval(() => {
      const health = healthCheck();
      
      this.healthHistory.push({
        timestamp: new Date(),
        healthy: health.overallHealthy
      });

      if (!health.overallHealthy) {
        console.warn('⚠️ System health degraded:', health);
      }

      // Keep only last 100 checks
      if (this.healthHistory.length > 100) {
        this.healthHistory.shift();
      }
    }, intervalSeconds * 1000);

    console.log(`Health monitoring started (interval: ${intervalSeconds}s)`);
  }

  /**
   * Stop periodic health checks
   */
  stop(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = undefined;
      console.log('Health monitoring stopped');
    }
  }

  /**
   * Get health history
   */
  getHealthHistory(): Array<{ timestamp: Date; healthy: boolean }> {
    return [...this.healthHistory];
  }

  /**
   * Get health statistics
   */
  getHealthStats(): any {
    if (this.healthHistory.length === 0) {
      return { message: 'No health data available' };
    }

    const healthyCount = this.healthHistory.filter(h => h.healthy).length;
    const totalChecks = this.healthHistory.length;
    const uptimePercentage = (healthyCount / totalChecks) * 100;

    return {
      totalChecks,
      healthyChecks: healthyCount,
      unhealthyChecks: totalChecks - healthyCount,
      uptimePercentage: uptimePercentage.toFixed(2),
      lastCheck: this.healthHistory[this.healthHistory.length - 1]
    };
  }
}

// Usage
const healthMonitor = new HealthMonitor();
healthMonitor.start(30); // Check every 30 seconds

// Later...
// healthMonitor.stop();
```

## Performance Monitoring

### Execution Timing

```typescript
class PerformanceMonitor {
  private timings: Map<string, number[]> = new Map();

  /**
   * Time an async operation
   */
  async time<T>(label: string, fn: () => Promise<T>): Promise<T> {
    const start = Date.now();
    
    try {
      const result = await fn();
      const duration = Date.now() - start;
      
      this.recordTiming(label, duration);
      console.log(`[${label}] completed in ${duration}ms`);
      
      return result;
    } catch (error) {
      const duration = Date.now() - start;
      this.recordTiming(label, duration);
      console.error(`[${label}] failed after ${duration}ms:`, error);
      throw error;
    }
  }

  private recordTiming(label: string, duration: number): void {
    if (!this.timings.has(label)) {
      this.timings.set(label, []);
    }
    this.timings.get(label)!.push(duration);
  }

  /**
   * Get timing statistics for a label
   */
  getStats(label: string): any {
    const durations = this.timings.get(label);
    
    if (!durations || durations.length === 0) {
      return { message: 'No timings recorded' };
    }

    const sorted = [...durations].sort((a, b) => a - b);
    const avg = durations.reduce((a, b) => a + b, 0) / durations.length;
    const min = sorted[0];
    const max = sorted[sorted.length - 1];
    const median = sorted[Math.floor(sorted.length / 2)];
    const p95 = sorted[Math.floor(sorted.length * 0.95)];
    const p99 = sorted[Math.floor(sorted.length * 0.99)];

    return {
      count: durations.length,
      avgMs: Math.round(avg),
      minMs: min,
      maxMs: max,
      medianMs: median,
      p95Ms: p95,
      p99Ms: p99
    };
  }

  /**
   * Get all statistics
   */
  getAllStats(): Record<string, any> {
    const stats: Record<string, any> = {};
    
    for (const label of this.timings.keys()) {
      stats[label] = this.getStats(label);
    }

    return stats;
  }
}

// Usage
async function monitoredOperations() {
  init();

  const perfMonitor = new PerformanceMonitor();
  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const client = new LlmClient(config);

  // Time multiple operations
  for (let i = 0; i < 5; i++) {
    await perfMonitor.time('llm-completion', () =>
      client.complete('Hello')
    );
  }

  // Get statistics
  const stats = perfMonitor.getStats('llm-completion');
  console.log('Completion stats:', stats);
}
```

## Memory Monitoring

### Track Memory Usage

```typescript
class MemoryMonitor {
  private snapshots: Array<{ timestamp: Date; usage: NodeJS.MemoryUsage }> = [];

  /**
   * Take a memory snapshot
   */
  snapshot(): NodeJS.MemoryUsage {
    const usage = process.memoryUsage();
    this.snapshots.push({ timestamp: new Date(), usage });

    // Keep only last 100 snapshots
    if (this.snapshots.length > 100) {
      this.snapshots.shift();
    }

    return usage;
  }

  /**
   * Format memory usage
   */
  formatMemory(bytes: number): string {
    return `${Math.round(bytes / 1024 / 1024)}MB`;
  }

  /**
   * Log current memory usage
   */
  logMemory(label?: string): void {
    const usage = this.snapshot();
    const prefix = label ? `[${label}] ` : '';

    console.log(`${prefix}Memory usage:`, {
      heapUsed: this.formatMemory(usage.heapUsed),
      heapTotal: this.formatMemory(usage.heapTotal),
      external: this.formatMemory(usage.external),
      rss: this.formatMemory(usage.rss)
    });
  }

  /**
   * Get memory delta between two snapshots
   */
  getDelta(snapshot1: NodeJS.MemoryUsage, snapshot2: NodeJS.MemoryUsage): any {
    return {
      heapUsed: this.formatMemory(snapshot2.heapUsed - snapshot1.heapUsed),
      heapTotal: this.formatMemory(snapshot2.heapTotal - snapshot1.heapTotal),
      external: this.formatMemory(snapshot2.external - snapshot1.external),
      rss: this.formatMemory(snapshot2.rss - snapshot1.rss)
    };
  }

  /**
   * Monitor memory during operation
   */
  async monitorOperation<T>(label: string, fn: () => Promise<T>): Promise<T> {
    const before = this.snapshot();
    console.log(`[${label}] Starting...`);
    this.logMemory(`${label} - Before`);

    const result = await fn();

    const after = this.snapshot();
    this.logMemory(`${label} - After`);

    const delta = this.getDelta(before, after);
    console.log(`[${label}] Memory delta:`, delta);

    return result;
  }
}

// Usage
async function monitorMemory() {
  init();

  const memMonitor = new MemoryMonitor();
  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('MemoryTest');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  await memMonitor.monitorOperation('workflow-execution', () =>
    executor.execute(workflow)
  );
}
```

## Error Tracking

### Error Logger

```typescript
interface ErrorLog {
  timestamp: Date;
  executionId?: string;
  errorType: string;
  message: string;
  stack?: string;
}

class ErrorTracker {
  private errors: ErrorLog[] = [];

  /**
   * Log an error
   */
  logError(error: Error | string, executionId?: string): void {
    const errorLog: ErrorLog = {
      timestamp: new Date(),
      executionId,
      errorType: error instanceof Error ? error.constructor.name : 'Unknown',
      message: error instanceof Error ? error.message : error,
      stack: error instanceof Error ? error.stack : undefined
    };

    this.errors.push(errorLog);

    // Keep only last 1000 errors
    if (this.errors.length > 1000) {
      this.errors.shift();
    }

    console.error(`[ERROR] ${errorLog.errorType}: ${errorLog.message}`);
  }

  /**
   * Get error statistics
   */
  getErrorStats(timeWindowHours: number = 24): any {
    const cutoffTime = new Date(Date.now() - timeWindowHours * 60 * 60 * 1000);
    const recentErrors = this.errors.filter(e => e.timestamp > cutoffTime);

    if (recentErrors.length === 0) {
      return { message: 'No errors in time window' };
    }

    // Group by error type
    const errorsByType: Record<string, number> = {};
    recentErrors.forEach(e => {
      errorsByType[e.errorType] = (errorsByType[e.errorType] || 0) + 1;
    });

    return {
      timeWindowHours,
      totalErrors: recentErrors.length,
      errorsByType,
      mostCommonError: Object.keys(errorsByType).reduce((a, b) =>
        errorsByType[a] > errorsByType[b] ? a : b
      )
    };
  }

  /**
   * Get recent errors
   */
  getRecentErrors(count: number = 10): ErrorLog[] {
    return this.errors.slice(-count);
  }
}
```

## Comprehensive Monitoring System

```typescript
class ComprehensiveMonitor {
  private workflowMonitor: WorkflowMonitor;
  private healthMonitor: HealthMonitor;
  private perfMonitor: PerformanceMonitor;
  private memMonitor: MemoryMonitor;
  private errorTracker: ErrorTracker;

  constructor() {
    this.workflowMonitor = new WorkflowMonitor();
    this.healthMonitor = new HealthMonitor();
    this.perfMonitor = new PerformanceMonitor();
    this.memMonitor = new MemoryMonitor();
    this.errorTracker = new ErrorTracker();
  }

  /**
   * Start all monitoring
   */
  startMonitoring(): void {
    init();
    this.healthMonitor.start(60); // Check every 60 seconds
    console.log('Comprehensive monitoring started');
  }

  /**
   * Stop all monitoring
   */
  stopMonitoring(): void {
    this.healthMonitor.stop();
    console.log('Comprehensive monitoring stopped');
  }

  /**
   * Execute workflow with full monitoring
   */
  async executeWithMonitoring(
    workflow: Workflow,
    executor: Executor
  ): Promise<any> {
    const executionId = this.workflowMonitor.startExecution(workflow);

    try {
      const result = await this.perfMonitor.time(
        'workflow-execution',
        async () => {
          return await this.memMonitor.monitorOperation(
            executionId,
            () => executor.execute(workflow)
          );
        }
      );

      if (result.isSuccess()) {
        this.workflowMonitor.endExecution(executionId, 'completed');
      } else {
        this.workflowMonitor.endExecution(executionId, 'failed', result.error());
        this.errorTracker.logError(result.error(), executionId);
      }

      return result;
    } catch (error) {
      this.workflowMonitor.endExecution(
        executionId,
        'failed',
        error instanceof Error ? error.message : 'Unknown error'
      );
      this.errorTracker.logError(
        error instanceof Error ? error : new Error(String(error)),
        executionId
      );
      throw error;
    }
  }

  /**
   * Get comprehensive dashboard
   */
  getDashboard(): any {
    return {
      workflows: this.workflowMonitor.getMetricsSummary(24),
      health: this.healthMonitor.getHealthStats(),
      performance: this.perfMonitor.getAllStats(),
      errors: this.errorTracker.getErrorStats(24)
    };
  }
}

// Usage
async function fullMonitoringExample() {
  const monitor = new ComprehensiveMonitor();
  monitor.startMonitoring();

  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('MonitoredWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  // Execute with monitoring
  const result = await monitor.executeWithMonitoring(workflow, executor);

  // Get dashboard
  const dashboard = monitor.getDashboard();
  console.log('Monitoring Dashboard:', JSON.stringify(dashboard, null, 2));

  // Cleanup
  monitor.stopMonitoring();
}
```

## Best Practices

1. **Always monitor production workflows**
   ```typescript
   const monitor = new WorkflowMonitor();
   const executionId = monitor.startExecution(workflow);
   // ... execute workflow ...
   monitor.endExecution(executionId, status);
   ```

2. **Enable periodic health checks**
   ```typescript
   const healthMonitor = new HealthMonitor();
   healthMonitor.start(60); // Every 60 seconds
   ```

3. **Track performance metrics**
   ```typescript
   const perfMonitor = new PerformanceMonitor();
   await perfMonitor.time('operation', () => doWork());
   ```

4. **Monitor memory in long-running processes**
   ```typescript
   const memMonitor = new MemoryMonitor();
   setInterval(() => memMonitor.logMemory(), 60000);
   ```

5. **Log all errors**
   ```typescript
   const errorTracker = new ErrorTracker();
   try {
     await operation();
   } catch (error) {
     errorTracker.logError(error, executionId);
   }
   ```

## See Also

- [Reliability Guide](./reliability-js.md)
- [Performance Optimization](./performance-js.md)
- [Debugging Guide](../development/debugging-js.md)
- [Architecture Guide](../development/architecture-js.md)
