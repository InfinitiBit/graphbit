# Reliability and Fault Tolerance - JavaScript

This guide covers building robust and resilient GraphBit workflows in JavaScript/Node.js, with comprehensive error handling, fault tolerance, and recovery strategies.

## Overview

Reliability in GraphBit JavaScript encompasses:
- **Error Handling**: Graceful handling of failures and exceptions
- **Fault Tolerance**: Continuing operation despite component failures
- **Recovery Strategies**: Automatic and manual recovery mechanisms
- **Retry Logic**: Intelligent retry patterns for transient failures
- **Circuit Breakers**: Preventing cascading failures (via LLM client)
- **Health Monitoring**: Continuous system health assessment

## Error Handling Patterns

### Basic Error Handling

```typescript
import { init, Workflow, Node, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

async function safeWorkflowExecution(
  workflow: Workflow,
  executor: Executor,
  maxRetries: number = 3
): Promise<any> {
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      console.log(`Execution attempt ${attempt + 1}/${maxRetries + 1}`);

      const result = await executor.execute(workflow);

      if (result.isSuccess()) {
        console.log('✅ Workflow executed successfully');
        return result;
      } else {
        const errorMsg = result.error();
        console.error(`❌ Workflow failed: ${errorMsg}`);

        if (attempt < maxRetries) {
          const waitTime = Math.pow(2, attempt) * 1000; // Exponential backoff
          console.log(`⏳ Retrying in ${waitTime / 1000} seconds...`);
          await new Promise(resolve => setTimeout(resolve, waitTime));
        } else {
          console.error('❌ Max retries exceeded');
          return result;
        }
      }
    } catch (error) {
      console.error(`❌ Execution exception:`, error);

      if (attempt < maxRetries) {
        const waitTime = Math.pow(2, attempt) * 1000;
        console.log(`⏳ Retrying in ${waitTime / 1000} seconds...`);
        await new Promise(resolve => setTimeout(resolve, waitTime));
      } else {
        console.error('❌ Max retries exceeded');
        throw error;
      }
    }
  }

  return null;
}

// Usage
async function example() {
  init();

  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('ReliableWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  const result = await safeWorkflowExecution(workflow, executor, 3);
}

example().catch(console.error);
```

### Fault-Tolerant Workflow Creation

```typescript
async function createFaultTolerantWorkflow(): Promise<Workflow> {
  const workflow = new Workflow('FaultTolerantWorkflow');

  // Input validator with error handling
  const validator = Node.agent(
    'Input Validator',
    `Validate this input and handle any issues gracefully:

If the input is invalid:
1. Identify the specific issues
2. Suggest corrections if possible
3. Return a status indicating validation result

If valid, return the input with validation confirmation.`,
    'validator'
  );

  // Robust processor with fallback logic
  const processor = Node.agent(
    'Robust Processor',
    `Process the validated input with error resilience:

If processing encounters issues:
1. Try alternative processing methods
2. Provide partial results if possible
3. Report any limitations or warnings

Always return some form of useful output.`,
    'processor'
  );

  // Error recovery node
  const recovery = Node.agent(
    'Error Recovery',
    `Review the processing results and recover from any failures:

If errors occurred:
1. Attempt to salvage useful information
2. Provide alternative solutions
3. Document what went wrong and why

Return the best possible outcome given the circumstances.`,
    'recovery'
  );

  await workflow.addNode(validator);
  await workflow.addNode(processor);
  await workflow.addNode(recovery);

  await workflow.connect('validator', 'processor');
  await workflow.connect('processor', 'recovery');

  await workflow.validate();

  return workflow;
}
```

## Retry Strategies

### Exponential Backoff Retry

```typescript
interface RetryConfig {
  maxRetries: number;
  initialDelayMs: number;
  maxDelayMs: number;
  backoffMultiplier: number;
}

class RetryHandler {
  constructor(private config: RetryConfig = {
    maxRetries: 3,
    initialDelayMs: 1000,
    maxDelayMs: 30000,
    backoffMultiplier: 2
  }) {}

  /**
   * Execute function with exponential backoff retry
   */
  async withRetry<T>(
    fn: () => Promise<T>,
    label: string = 'operation'
  ): Promise<T> {
    let lastError: Error | undefined;

    for (let attempt = 0; attempt <= this.config.maxRetries; attempt++) {
      try {
        console.log(`[${label}] Attempt ${attempt + 1}/${this.config.maxRetries + 1}`);
        return await fn();
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error));
        console.error(`[${label}] Attempt ${attempt + 1} failed:`, lastError.message);

        if (attempt < this.config.maxRetries) {
          const delay = Math.min(
            this.config.initialDelayMs * Math.pow(this.config.backoffMultiplier, attempt),
            this.config.maxDelayMs
          );
          console.log(`[${label}] Retrying in ${delay}ms...`);
          await new Promise(resolve => setTimeout(resolve, delay));
        }
      }
    }

    throw new Error(`[${label}] Failed after ${this.config.maxRetries + 1} attempts: ${lastError?.message}`);
  }

  /**
   * Execute workflow with retry
   */
  async executeWorkflowWithRetry(
    executor: Executor,
    workflow: Workflow
  ): Promise<any> {
    return this.withRetry(
      async () => {
        const result = await executor.execute(workflow);
        if (!result.isSuccess()) {
          throw new Error(result.error());
        }
        return result;
      },
      'workflow-execution'
    );
  }
}

// Usage
async function retryExample() {
  init();

  const retryHandler = new RetryHandler({
    maxRetries: 5,
    initialDelayMs: 1000,
    maxDelayMs: 10000,
    backoffMultiplier: 2
  });

  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('RetryWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  try {
    const result = await retryHandler.executeWorkflowWithRetry(executor, workflow);
    console.log('✅ Execution successful:', result.variables());
  } catch (error) {
    console.error('❌ All retries failed:', error);
  }
}
```

### Conditional Retry

```typescript
class ConditionalRetryHandler {
  /**
   * Determine if error is retryable
   */
  private isRetryable(error: Error): boolean {
    const retryablePatterns = [
      /timeout/i,
      /network/i,
      /connection/i,
      /ECONNRESET/i,
      /ETIMEDOUT/i,
      /rate limit/i,
      /429/
    ];

    return retryablePatterns.some(pattern => 
      pattern.test(error.message)
    );
  }

  /**
   * Execute with conditional retry
   */
  async withConditionalRetry<T>(
    fn: () => Promise<T>,
    maxRetries: number = 3
  ): Promise<T> {
    let lastError: Error | undefined;

    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        return await fn();
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error));

        // Check if error is retryable
        if (!this.isRetryable(lastError)) {
          console.error('Non-retryable error:', lastError.message);
          throw lastError;
        }

        if (attempt < maxRetries) {
          const delay = Math.pow(2, attempt) * 1000;
          console.log(`Retryable error detected. Retrying in ${delay}ms...`);
          await new Promise(resolve => setTimeout(resolve, delay));
        }
      }
    }

    throw lastError!;
  }
}
```

## Circuit Breaker Pattern

### Simple Circuit Breaker

```typescript
enum CircuitState {
  Closed = 'CLOSED',
  Open = 'OPEN',
  HalfOpen = 'HALF_OPEN'
}

interface CircuitBreakerConfig {
  failureThreshold: number;
  successThreshold: number;
  timeout: number;
}

class CircuitBreaker {
  private state: CircuitState = CircuitState.Closed;
  private failureCount: number = 0;
  private successCount: number = 0;
  private nextAttempt: number = Date.now();

  constructor(private config: CircuitBreakerConfig = {
    failureThreshold: 5,
    successThreshold: 2,
    timeout: 60000
  }) {}

  /**
   * Execute function with circuit breaker
   */
  async execute<T>(fn: () => Promise<T>): Promise<T> {
    if (this.state === CircuitState.Open) {
      if (Date.now() < this.nextAttempt) {
        throw new Error('Circuit breaker is OPEN');
      }
      this.state = CircuitState.HalfOpen;
      console.log('Circuit breaker entering HALF_OPEN state');
    }

    try {
      const result = await fn();
      this.onSuccess();
      return result;
    } catch (error) {
      this.onFailure();
      throw error;
    }
  }

  private onSuccess(): void {
    this.failureCount = 0;

    if (this.state === CircuitState.HalfOpen) {
      this.successCount++;
      if (this.successCount >= this.config.successThreshold) {
        this.state = CircuitState.Closed;
        this.successCount = 0;
        console.log('Circuit breaker CLOSED');
      }
    }
  }

  private onFailure(): void {
    this.failureCount++;
    this.successCount = 0;

    if (this.failureCount >= this.config.failureThreshold) {
      this.state = CircuitState.Open;
      this.nextAttempt = Date.now() + this.config.timeout;
      console.error(`Circuit breaker OPEN. Will retry after ${this.config.timeout}ms`);
    }
  }

  getState(): CircuitState {
    return this.state;
  }
}

// Usage
async function circuitBreakerExample() {
  init();

  const breaker = new CircuitBreaker({
    failureThreshold: 3,
    successThreshold: 2,
    timeout: 30000
  });

  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('CircuitBreakerWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  for (let i = 0; i < 10; i++) {
    try {
      console.log(`\nAttempt ${i + 1}`);
      const result = await breaker.execute(() => 
        executor.execute(workflow)
      );
      console.log('✅ Execution successful');
    } catch (error) {
      console.error('❌ Execution failed:', error instanceof Error ? error.message : error);
    }

    await new Promise(resolve => setTimeout(resolve, 5000));
  }
}
```

## Graceful Degradation

### Fallback Strategies

```typescript
class FallbackHandler {
  /**
   * Execute with fallback function
   */
  async withFallback<T>(
    primary: () => Promise<T>,
    fallback: () => Promise<T>,
    label: string = 'operation'
  ): Promise<T> {
    try {
      console.log(`[${label}] Attempting primary method...`);
      return await primary();
    } catch (primaryError) {
      console.warn(`[${label}] Primary failed, using fallback:`, 
        primaryError instanceof Error ? primaryError.message : primaryError);
      
      try {
        return await fallback();
      } catch (fallbackError) {
        console.error(`[${label}] Both primary and fallback failed`);
        throw new Error(
          `Primary: ${primaryError instanceof Error ? primaryError.message : primaryError}, ` +
          `Fallback: ${fallbackError instanceof Error ? fallbackError.message : fallbackError}`
        );
      }
    }
  }

  /**
   * Execute with multiple fallbacks
   */
  async withMultipleFallbacks<T>(
    strategies: Array<() => Promise<T>>,
    label: string = 'operation'
  ): Promise<T> {
    const errors: Error[] = [];

    for (let i = 0; i < strategies.length; i++) {
      try {
        console.log(`[${label}] Trying strategy ${i + 1}/${strategies.length}...`);
        return await strategies[i]();
      } catch (error) {
        console.warn(`[${label}] Strategy ${i + 1} failed:`, 
          error instanceof Error ? error.message : error);
        errors.push(error instanceof Error ? error : new Error(String(error)));
        
        if (i === strategies.length - 1) {
          throw new Error(
            `All strategies failed: ${errors.map(e => e.message).join(', ')}`
          );
        }
      }
    }

    throw new Error('No strategies provided');
  }
}

// Usage
async function fallbackExample() {
  init();

  const fallbackHandler = new FallbackHandler();

  const primaryConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  const fallbackConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-3.5-turbo'
  });

  const workflow = new Workflow('FallbackWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  try {
    const result = await fallbackHandler.withFallback(
      () => new Executor(primaryConfig).execute(workflow),
      () => new Executor(fallbackConfig).execute(workflow),
      'workflow-execution'
    );
    console.log('✅ Execution successful with fallback strategy');
  } catch (error) {
    console.error('❌ All strategies failed:', error);
  }
}
```

## Health Checks

### Continuous Health Monitoring

```typescript
import { init, healthCheck } from '@infinitibit_gmbh/graphbit';

class HealthChecker {
  private healthyState: boolean = true;
  private checkInterval?: NodeJS.Timeout;

  /**
   * Start continuous health monitoring
   */
  start(intervalSeconds: number = 30): void {
    init();

    this.checkInterval = setInterval(() => {
      const health = healthCheck();
      
      if (!health.overallHealthy && this.healthyState) {
        console.error('⚠️ System health degraded!');
        this.onHealthDegraded(health);
      } else if (health.overallHealthy && !this.healthyState) {
        console.log('✅ System health restored');
        this.onHealthRestored(health);
      }

      this.healthyState = health.overallHealthy;
    }, intervalSeconds * 1000);

    console.log(`Health monitoring started (${intervalSeconds}s interval)`);
  }

  /**
   * Stop health monitoring
   */
  stop(): void {
    if (this.checkInterval) {
      clearInterval(this.checkInterval);
      this.checkInterval = undefined;
      console.log('Health monitoring stopped');
    }
  }

  private onHealthDegraded(health: any): void {
    // Implement alerting, logging, or recovery actions
    console.error('Health check details:', health);
  }

  private onHealthRestored(health: any): void {
    // Implement recovery confirmation actions
    console.log('Health check details:', health);
  }

  /**
   * Check if system is healthy
   */
  isHealthy(): boolean {
    const health = healthCheck();
    return health.overallHealthy;
  }
}
```

## Timeout Management

### Timeout Wrapper

```typescript
class TimeoutManager {
  /**
   * Execute function with timeout
   */
  async withTimeout<T>(
    fn: () => Promise<T>,
    timeoutMs: number,
    label: string = 'operation'
  ): Promise<T> {
    return Promise.race([
      fn(),
      new Promise<T>((_, reject) =>
        setTimeout(() => reject(new Error(`${label} timed out after ${timeoutMs}ms`)), timeoutMs)
      )
    ]);
  }

  /**
   * Execute workflow with timeout
   */
  async executeWithTimeout(
    executor: Executor,
    workflow: Workflow,
    timeoutMs: number = 30000
  ): Promise<any> {
    return this.withTimeout(
      () => executor.execute(workflow),
      timeoutMs,
      'workflow-execution'
    );
  }
}

// Usage
async function timeoutExample() {
  init();

  const timeoutManager = new TimeoutManager();
  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('TimeoutWorkflow');
  const node = Node.agent('Agent', 'Long task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  try {
    const result = await timeoutManager.executeWithTimeout(
      executor,
      workflow,
      30000 // 30 seconds
    );
    console.log('✅ Completed within timeout');
  } catch (error) {
    console.error('❌ Timeout or execution error:', error);
  }
}
```

## Comprehensive Reliability System

```typescript
class ReliabilityManager {
  private retryHandler: RetryHandler;
  private circuitBreaker: CircuitBreaker;
  private fallbackHandler: FallbackHandler;
  private timeoutManager: TimeoutManager;
  private healthChecker: HealthChecker;

  constructor() {
    this.retryHandler = new RetryHandler();
    this.circuitBreaker = new CircuitBreaker();
    this.fallbackHandler = new FallbackHandler();
    this.timeoutManager = new TimeoutManager();
    this.healthChecker = new HealthChecker();
  }

  /**
   * Start reliability monitoring
   */
  start(): void {
    this.healthChecker.start(30);
    console.log('Reliability system started');
  }

  /**
   * Stop reliability monitoring
   */
  stop(): void {
    this.healthChecker.stop();
    console.log('Reliability system stopped');
  }

  /**
   * Execute workflow with full reliability features
   */
  async executeReliably(
    executor: Executor,
    workflow: Workflow,
    options: {
      timeoutMs?: number;
      enableRetry?: boolean;
      enableCircuitBreaker?: boolean;
    } = {}
  ): Promise<any> {
    const {
      timeoutMs = 30000,
      enableRetry = true,
      enableCircuitBreaker = true
    } = options;

    // Check health before execution
    if (!this.healthChecker.isHealthy()) {
      throw new Error('System health check failed');
    }

    // Build execution pipeline
    const execute = async () => {
      const result = await executor.execute(workflow);
      if (!result.isSuccess()) {
        throw new Error(result.error());
      }
      return result;
    };

    // Apply timeout
    let operation = () => this.timeoutManager.withTimeout(
      execute,
      timeoutMs,
      'workflow'
    );

    // Apply circuit breaker
    if (enableCircuitBreaker) {
      const prevOperation = operation;
      operation = () => this.circuitBreaker.execute(prevOperation);
    }

    // Apply retry
    if (enableRetry) {
      return this.retryHandler.withRetry(operation, 'reliable-execution');
    }

    return operation();
  }
}

// Usage
async function reliabilityExample() {
  init();

  const reliability = new ReliabilityManager();
  reliability.start();

  const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
  const executor = new Executor(config);

  const workflow = new Workflow('ReliableWorkflow');
  const node = Node.agent('Agent', 'Task', 'agent_1');
  
  await workflow.addNode(node);
  await workflow.validate();

  try {
    const result = await reliability.executeReliably(executor, workflow, {
      timeoutMs: 30000,
      enableRetry: true,
      enableCircuitBreaker: true
    });
    console.log('✅ Execution successful:', result.variables());
  } catch (error) {
    console.error('❌ Execution failed:', error);
  } finally {
    reliability.stop();
  }
}
```

## Best Practices

1. **Always implement retry logic**
2. **Use circuit breakers for external dependencies**
3. **Implement fallback strategies**
4. **Set appropriate timeouts**
5. **Monitor system health continuously**
6. **Log all failures for analysis**
7. **Test failure scenarios**

## See Also

- [Monitoring Guide](./monitoring-js.md)
- [Performance Optimization](./performance-js.md)
- [Error Handling in API Reference](../api-reference/javascript-api.md)
