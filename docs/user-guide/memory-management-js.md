# Memory Management - JavaScript

This guide covers memory management in GraphBit JavaScript applications, including monitoring, optimization strategies, and configuration for resource-constrained environments.

## Overview

GraphBit's memory management for JavaScript/Node.js includes:
- **Memory Monitoring**: Real-time tracking of memory usage
- **Memory Optimization**: Strategies to minimize memory footprint
- **Resource Configuration**: Settings for constrained environments
- **Memory Profiling**: Identifying memory leaks and bottlenecks
- **Garbage Collection**: Understanding Node.js GC behavior with native modules

## Memory Architecture

### Native Module Memory

GraphBit uses native Rust modules via napi-rs, which means memory is allocated in two places:

```
┌─────────────────────────────────────┐
│   JavaScript Heap (V8)              │
│   - JS objects                      │
│   - Closures                        │
│   - Callback references             │
├─────────────────────────────────────┤
│   External Memory (Native)          │
│   - Rust allocations               │
│   - Native module data             │
│   - Zero-copy buffers              │
└─────────────────────────────────────┘
```

**Key insight**: GraphBit's zero-copy architecture means most data stays in native memory, reducing JS heap pressure by 140× compared to pure JavaScript alternatives.

## Memory Monitoring

### Basic Memory Tracking

```typescript
import { init, getSystemInfo } from '@infinitibit_gmbh/graphbit';

function logMemoryUsage(label?: string): void {
  const usage = process.memoryUsage();
  const prefix = label ? `[${label}] ` : '';

  console.log(`${prefix}Memory usage:`, {
    heapUsed: `${Math.round(usage.heapUsed / 1024 / 1024)}MB`,
    heapTotal: `${Math.round(usage.heapTotal / 1024 / 1024)}MB`,
    external: `${Math.round(usage.external / 1024 / 1024)}MB`,
    rss: `${Math.round(usage.rss / 1024 / 1024)}MB`
  });
}

// Example usage
init();
logMemoryUsage('After init');

const info = getSystemInfo();
console.log('System info:', info);
logMemoryUsage('After getSystemInfo');
```

### Memory Snapshot Class

```typescript
interface MemorySnapshot {
  timestamp: Date;
  heapUsed: number;
  heapTotal: number;
  external: number;
  rss: number;
}

class MemoryTracker {
  private snapshots: MemorySnapshot[] = [];

  /**
   * Take a memory snapshot
   */
  snapshot(label?: string): MemorySnapshot {
    const usage = process.memoryUsage();
    
    const snapshot: MemorySnapshot = {
      timestamp: new Date(),
      heapUsed: usage.heapUsed,
      heapTotal: usage.heapTotal,
      external: usage.external,
      rss: usage.rss
    };

    this.snapshots.push(snapshot);

    if (label) {
      console.log(`[${label}] Memory snapshot:`, this.formatSnapshot(snapshot));
    }

    // Keep only last 100 snapshots
    if (this.snapshots.length > 100) {
      this.snapshots.shift();
    }

    return snapshot;
  }

  /**
   * Format memory values
   */
  formatSnapshot(snapshot: MemorySnapshot): Record<string, string> {
    return {
      heapUsed: `${Math.round(snapshot.heapUsed / 1024 / 1024)}MB`,
      heapTotal: `${Math.round(snapshot.heapTotal / 1024 / 1024)}MB`,
      external: `${Math.round(snapshot.external / 1024 / 1024)}MB`,
      rss: `${Math.round(snapshot.rss / 1024 / 1024)}MB`
    };
  }

  /**
   * Calculate delta between two snapshots
   */
  delta(snap1: MemorySnapshot, snap2: MemorySnapshot): Record<string, string> {
    return {
      heapUsed: `${Math.round((snap2.heapUsed - snap1.heapUsed) / 1024 / 1024)}MB`,
      heapTotal: `${Math.round((snap2.heapTotal - snap1.heapTotal) / 1024 / 1024)}MB`,
      external: `${Math.round((snap2.external - snap1.external) / 1024 / 1024)}MB`,
      rss: `${Math.round((snap2.rss - snap1.rss) / 1024 / 1024)}MB`
    };
  }

  /**
   * Get all snapshots
   */
  getSnapshots(): MemorySnapshot[] {
    return [...this.snapshots];
  }

  /**
   * Get memory statistics
   */
  getStats(): any {
    if (this.snapshots.length < 2) {
      return { message: 'Not enough data' };
    }

    const first = this.snapshots[0];
    const last = this.snapshots[this.snapshots.length - 1];

    const heapGrowth = last.heapUsed - first.heapUsed;
    const externalGrowth = last.external - first.external;

    return {
      snapshotCount: this.snapshots.length,
      timeRange: {
        start: first.timestamp,
        end: last.timestamp,
        durationMs: last.timestamp.getTime() - first.timestamp.getTime()
      },
      heapGrowthMB: Math.round(heapGrowth / 1024 / 1024),
      externalGrowthMB: Math.round(externalGrowth / 1024 / 1024),
      current: this.formatSnapshot(last)
    };
  }
}

// Usage example
async function trackMemoryUsage() {
  init();

  const tracker = new MemoryTracker();
  
  const before = tracker.snapshot('Before execution');

  // Perform operations
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const executor = new Executor(config);
  const workflow = new Workflow('MemoryTest');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();
  await executor.execute(workflow);

  const after = tracker.snapshot('After execution');

  console.log('Memory delta:', tracker.delta(before, after));
  console.log('Statistics:', tracker.getStats());
}
```

## Memory Optimization Strategies

### 1. Use Appropriate Executor Type

```typescript
import { LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

async function memoryOptimizedExecution() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  // Low-latency executor uses single thread = lower memory
  const executor = Executor.newLowLatency(config);

  // For memory-constrained environments
  console.log('Using low-latency executor for minimal memory footprint');

  return executor;
}
```

### 2. Stream Large Responses

```typescript
import { LlmClient } from '@infinitibit_gmbh/graphbit';

async function streamToReduceMemory() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);

  // Instead of loading entire response into memory
  // Stream and process incrementally
  const stream = await client.stream('Generate a long document');

  let totalChunks = 0;
  let processedData = 0;

  for await (const chunk of stream) {
    totalChunks++;
    processedData += chunk.length;
    
    // Process chunk immediately and let GC clean up
    processChunk(chunk);
  }

  console.log(`Processed ${totalChunks} chunks, ${processedData} bytes total`);
}

function processChunk(chunk: string): void {
  // Process chunk (e.g., write to file, send to client, etc.)
  // Chunk memory is released after function returns
}
```

### 3. Batch with Memory Limits

```typescript
async function memoryAwareBatchProcessing(items: string[]) {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const client = new LlmClient(config);
  const tracker = new MemoryTracker();

  const MEMORY_THRESHOLD_MB = 500; // Stop if memory exceeds 500MB
  const BATCH_SIZE = 10;

  for (let i = 0; i < items.length; i += BATCH_SIZE) {
    const batch = items.slice(i, i + BATCH_SIZE);

    // Check memory before processing
    const snapshot = tracker.snapshot();
    if (snapshot.heapUsed / 1024 / 1024 > MEMORY_THRESHOLD_MB) {
      console.warn('Memory threshold exceeded, forcing GC...');
      
      if (global.gc) {
        global.gc();
      }

      // Wait for GC to complete
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    // Process batch
    await client.completeBatch(batch, 100, 0.7, 3);
    
    console.log(`Processed batch ${Math.floor(i / BATCH_SIZE) + 1}`);
  }
}
```

### 4. Cleanup and Resource Management

```typescript
class ManagedExecutor {
  private executor: Executor | null = null;
  private config: any;

  constructor(config: any) {
    this.config = config;
  }

  /**
   * Get or create executor
   */
  getExecutor(): Executor {
    if (!this.executor) {
      this.executor = new Executor(this.config);
    }
    return this.executor;
  }

  /**
   * Cleanup resources
   */
  cleanup(): void {
    this.executor = null;
    
    // Force GC if available
    if (global.gc) {
      global.gc();
    }

    console.log('Executor resources cleaned up');
  }

  /**
   * Execute with automatic cleanup
   */
  async executeAndCleanup(workflow: Workflow): Promise<any> {
    try {
      const executor = this.getExecutor();
      return await executor.execute(workflow);
    } finally {
      this.cleanup();
    }
  }
}

// Usage
async function managedExecution() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const managed = new ManagedExecutor(config);

  const workflow = new Workflow('ManagedWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  const result = await managed.executeAndCleanup(workflow);
  
  console.log('Execution complete, resources cleaned up');
}
```

## Memory Profiling

### Detect Memory Leaks

```typescript
class MemoryLeakDetector {
  private baselineSnapshot?: MemorySnapshot;
  private tracker = new MemoryTracker();
  private warningThresholdMB = 100;

  /**
   * Set baseline memory usage
   */
  setBaseline(): void {
    this.baselineSnapshot = this.tracker.snapshot('Baseline');
    console.log('Memory baseline set');
  }

  /**
   * Check for memory leaks
   */
  checkForLeaks(): boolean {
    if (!this.baselineSnapshot) {
      console.warn('No baseline set, call setBaseline() first');
      return false;
    }

    const current = this.tracker.snapshot('Current');
    const delta = this.tracker.delta(this.baselineSnapshot, current);

    const heapGrowthMB = (current.heapUsed - this.baselineSnapshot.heapUsed) / 1024 / 1024;
    const externalGrowthMB = (current.external - this.baselineSnapshot.external) / 1024 / 1024;

    console.log('Memory growth since baseline:', {
      heapMB: Math.round(heapGrowthMB),
      externalMB: Math.round(externalGrowthMB)
    });

    if (heapGrowthMB > this.warningThresholdMB) {
      console.warn(`⚠️ Potential memory leak detected! Heap grew by ${Math.round(heapGrowthMB)}MB`);
      return true;
    }

    return false;
  }

  /**
   * Monitor continuously
   */
  startMonitoring(intervalMs: number = 30000): NodeJS.Timeout {
    console.log(`Starting memory leak monitoring (${intervalMs}ms interval)`);
    
    this.setBaseline();

    return setInterval(() => {
      this.checkForLeaks();
    }, intervalMs);
  }
}

// Usage
async function detectLeaks() {
  init();

  const detector = new MemoryLeakDetector();
  detector.setBaseline();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const executor = new Executor(config);

  // Run multiple executions
  for (let i = 0; i < 10; i++) {
    const workflow = new Workflow(`Workflow ${i}`);
    const node = Node.agent('Agent', 'Task', `agent_${i}`);
    
    await workflow.addNode(node);
    await workflow.validate();
    await executor.execute(workflow);

    // Check after each execution
    detector.checkForLeaks();

    await new Promise(resolve => setTimeout(resolve, 1000));
  }
}
```

## Node.js Configuration for Memory

### Increase Heap Size

```bash
# Run Node.js with larger heap (4GB)
node --max-old-space-size=4096 app.js

# Run with explicit GC control
node --expose-gc --max-old-space-size=4096 app.js
```

### Package.json Script

```json
{
  "scripts": {
    "start": "node --max-old-space-size=4096 dist/index.js",
    "start:debug": "node --expose-gc --max-old-space-size=4096 dist/index.js",
    "start:low-memory": "node --max-old-space-size=512 dist/index.js"
  }
}
```

### Programmatic Configuration

```typescript
// Check Node.js heap configuration
function checkHeapConfig(): void {
  const v8 = require('v8');
  const heapStats = v8.getHeapStatistics();

  console.log('Heap Configuration:', {
    heapSizeLimit: `${Math.round(heapStats.heap_size_limit / 1024 / 1024)}MB`,
    totalHeapSize: `${Math.round(heapStats.total_heap_size / 1024 / 1024)}MB`,
    usedHeapSize: `${Math.round(heapStats.used_heap_size / 1024 / 1024)}MB`,
    mallocedMemory: `${Math.round(heapStats.malloced_memory / 1024 / 1024)}MB`
  });
}

// Trigger garbage collection (requires --expose-gc)
function forceGC(): void {
  if (global.gc) {
    console.log('Forcing garbage collection...');
    const before = process.memoryUsage().heapUsed;
    
    global.gc();
    
    const after = process.memoryUsage().heapUsed;
    const freed = Math.round((before - after) / 1024 / 1024);
    console.log(`GC freed ${freed}MB`);
  } else {
    console.warn('GC not exposed. Run with --expose-gc flag.');
  }
}
```

## Memory-Constrained Environments

### Docker Configuration

```dockerfile
FROM node:18-slim

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy application
COPY . .

# Set memory limits
ENV NODE_OPTIONS="--max-old-space-size=512"

# Run with memory constraints
CMD ["node", "dist/index.js"]
```

### Kubernetes Resource Limits

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: graphbit-app
spec:
  containers:
  - name: app
    image: graphbit-app:latest
    resources:
      requests:
        memory: "256Mi"
        cpu: "100m"
      limits:
        memory: "512Mi"
        cpu: "500m"
    env:
    - name: NODE_OPTIONS
      value: "--max-old-space-size=384"
```

## Best Practices

1. **Monitor memory continuously** in production
   ```typescript
   const tracker = new MemoryTracker();
   setInterval(() => tracker.snapshot('Periodic'), 60000);
   ```

2. **Use streaming** for large responses
   ```typescript
   const stream = await client.stream(prompt);
   for await (const chunk of stream) processChunk(chunk);
   ```

3. **Choose appropriate executor type**
   ```typescript
   // Low memory: Use low-latency executor
   const executor = Executor.newLowLatency(config);
   ```

4. **Batch with memory awareness**
   ```typescript
   // Check memory before each batch
   if (usage.heapUsed > THRESHOLD) await cleanup();
   ```

5. **Cleanup resources explicitly**
   ```typescript
   // Set references to null when done
   executor = null;
   if (global.gc) global.gc();
   ```

6. **Profile memory regularly**
   ```typescript
   const detector = new MemoryLeakDetector();
   detector.startMonitoring(30000);
   ```

7. **Configure Node.js appropriately**
   ```bash
   node --max-old-space-size=4096 app.js
   ```

8. **Set resource limits in deployment**
   ```yaml
   resources:
     limits:
       memory: "512Mi"
   ```

## Memory Performance

GraphBit achieves:
- **140× lower memory** vs pure JavaScript alternatives
- **Zero-copy data transfer** between JS and Rust
- **Minimal heap pressure** from native allocations
- **Efficient garbage collection** with small JS footprint
- **~20MB base overhead** for Rust runtime

## See Also

- [Performance Optimization](./performance-js.md)
- [Monitoring Guide](./monitoring-js.md)
- [Architecture Guide](../development/architecture-js.md)
- [Debugging Guide](../development/debugging-js.md)
