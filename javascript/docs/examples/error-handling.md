# Error Handling Patterns

**Level:** Intermediate  
**Estimated Time:** 20 minutes  
**Prerequisites:** Basic GraphBit knowledge

## Overview

Production-grade error handling is critical for reliable GraphBit applications. This guide covers:

1. Common error types
2. Retry strategies
3. Circuit breakers
4. Graceful degradation
5. Error logging and monitoring

---

## Common Error Types

### 1. API Errors (LLM/Embedding)

- Rate limits
- Invalid API keys
- Model unavailable
- Timeout errors

### 2. Validation Errors

- Invalid parameters
- Type mismatches (NAPI-RS specific)
- Null vs undefined issues

### 3. Runtime Errors

- Network failures
- Memory limits
- Unexpected responses

---

## Pattern 1: Basic Try-Catch

```javascript
const { AgentBuilder, LlmConfig } = require('@infinitibit_gmbh/graphbit');

async function basicErrorHandling() {
  try {
    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY
    });
    
    const agent = await new AgentBuilder('Assistant', config).build();
    const response = await agent.execute('Hello');
    
    return response;
    
  } catch (error) {
    console.error('Error:', error.message);
    
    // Return fallback response
    return 'Service temporarily unavailable';
  }
}
```

---

## Pattern 2: Retry with Exponential Backoff

```javascript
async function retryWithBackoff(fn, maxRetries = 3, baseDelay = 1000) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
      
    } catch (error) {
      if (attempt === maxRetries) {
        throw new Error(`Failed after ${maxRetries} attempts: ${error.message}`);
      }
      
      // Check if error is retryable
      if (!isRetryable(error)) {
        throw error;
      }
      
      const delay = baseDelay * Math.pow(2, attempt - 1);
      console.log(`Attempt ${attempt} failed, retrying in ${delay}ms...`);
      
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
}

function isRetryable(error) {
  const retryableErrors = [
    'rate limit',
    'timeout',
    'network',
    'ECONNRESET',
    '429',
    '503'
  ];
  
  const errorMessage = error.message.toLowerCase();
  return retryableErrors.some(msg => errorMessage.includes(msg));
}

// Usage
const result = await retryWithBackoff(async () => {
  return await agent.execute('process this');
});
```

---

## Pattern 3: Circuit Breaker

```javascript
class CircuitBreaker {
  constructor(threshold = 5, timeout = 60000) {
    this.failureCount = 0;
    this.threshold = threshold;
    this.timeout = timeout;
    this.state = 'CLOSED';  // CLOSED, OPEN, HALF_OPEN
    this.nextAttempt = Date.now();
  }

  async execute(fn) {
    if (this.state === 'OPEN') {
      if (Date.now() < this.nextAttempt) {
        throw new Error('Circuit breaker is OPEN');
      }
      
      this.state = 'HALF_OPEN';
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

  onSuccess() {
    this.failureCount = 0;
    
    if (this.state === 'HALF_OPEN') {
      this.state = 'CLOSED';
      console.log('✅ Circuit breaker CLOSED');
    }
  }

  onFailure() {
    this.failureCount++;
    
    if (this.failureCount >= this.threshold) {
      this.state = 'OPEN';
      this.nextAttempt = Date.now() + this.timeout;
      console.log(`⚠️  Circuit breaker OPEN for ${this.timeout}ms`);
    }
  }

  getState() {
    return this.state;
  }
}

// Usage
const breaker = new CircuitBreaker(5, 60000);

async function protectedExecution(agent, prompt) {
  return await breaker.execute(async () => {
    return await agent.execute(prompt);
  });
}
```

---

## Pattern 4: Timeout Handler

```javascript
async function withTimeout(promise, timeoutMs) {
  const timeoutPromise = new Promise((_, reject) => {
    setTimeout(() => reject(new Error('Timeout')), timeoutMs);
  });

  return Promise.race([promise, timeoutPromise]);
}

// Usage
try {
  const response = await withTimeout(
    agent.execute('long running task'),
    30000  // 30 second timeout
  );
} catch (error) {
  if (error.message === 'Timeout') {
    console.log('Request timed out');
    // Handle timeout
  }
}
```

---

## Pattern 5: Graceful Degradation

```javascript
class ResilientAgent {
  constructor(primaryConfig, fallbackConfig) {
    this.primary = null;
    this.fallback = null;
    this.init(primaryConfig, fallbackConfig);
  }

  async init(primaryConfig, fallbackConfig) {
    this.primary = await new AgentBuilder('Primary', primaryConfig).build();
    
    if (fallbackConfig) {
      this.fallback = await new AgentBuilder('Fallback', fallbackConfig).build();
    }
  }

  async execute(prompt, options = {}) {
    const { useFallback = true, maxRetries = 2 } = options;

    // Try primary
    try {
      return await retryWithBackoff(
        async () => this.primary.execute(prompt),
        maxRetries
      );
      
    } catch (primaryError) {
      console.error('Primary agent failed:', primaryError.message);

      // Try fallback if available
      if (useFallback && this.fallback) {
        console.log('Falling back to secondary agent...');
        
        try {
          return await this.fallback.execute(prompt);
        } catch (fallbackError) {
          console.error('Fallback also failed:', fallbackError.message);
          throw new Error('Both primary and fallback failed');
        }
      }

      throw primaryError;
    }
  }
}

// Usage
const gpt4Config = LlmConfig.openai({ apiKey, model: 'gpt-4o' });
const gpt3Config = LlmConfig.openai({ apiKey, model: 'gpt-4o-mini' });

const resilientAgent = new ResilientAgent (gpt4Config, gpt3Config);
const response = await resilientAgent.execute('analyze this');
```

---

## Pattern 6: Error Classification and Handling

```javascript
class ErrorHandler {
  static classify(error) {
    const message = error.message.toLowerCase();

    if (message.includes('rate limit') || message.includes('429')) {
      return 'RATE_LIMIT';
    }
    if (message.includes('api key') || message.includes('401')) {
      return 'AUTH_ERROR';
    }
    if (message.includes('timeout')) {
      return 'TIMEOUT';
    }
    if (message.includes('network') || message.includes('ECONNRESET')) {
      return 'NETWORK_ERROR';
    }
    if (message.includes('model not found') || message.includes('404')) {
      return 'NOT_FOUND';
    }

    return 'UNKNOWN_ERROR';
  }

  static async handle(error, context = {}) {
    const errorType = this.classify(error);

    switch (errorType) {
      case 'RATE_LIMIT':
        console.log('Rate limit hit, waiting 60s...');
        await new Promise(r => setTimeout(r, 60000));
        return { retry: true, delay: 60000 };

      case 'AUTH_ERROR':
        console.error('Authentication failed - check API key');
        return { retry: false, fatal: true };

      case 'TIMEOUT':
        console.log('Request timed out');
        return { retry: true, delay: 5000 };

      case 'NETWORK_ERROR':
        console.log('Network error, retrying...');
        return { retry: true, delay: 2000 };

      case 'NOT_FOUND':
        console.error('Resource not found');
        return { retry: false, fatal: true };

      default:
        console.error('Unknown error:', error.message);
        return { retry: false, fatal: false };
    }
  }
}

// Usage
async function smartExecute(agent, prompt) {
  let attempts = 0;
  const maxAttempts = 3;

  while (attempts < maxAttempts) {
    try {
      return await agent.execute(prompt);
      
    } catch (error) {
      const strategy = await ErrorHandler.handle(error);

      if (strategy.fatal) {
        throw error;
      }

      if (!strategy.retry || attempts === maxAttempts - 1) {
        throw error;
      }

      attempts++;
      if (strategy.delay) {
        await new Promise(r => setTimeout(r, strategy.delay));
      }
    }
  }
}
```

---

## Pattern 7: Structured Error Logging

```javascript
class ErrorLogger {
  constructor(options = {}) {
    this.logToFile = options.logToFile || false;
    this.logToConsole = options.logToConsole !== false;
  }

  log(error, context = {}) {
    const errorLog = {
      timestamp: new Date().toISOString(),
      error: {
        message: error.message,
        stack: error.stack,
        type: error.constructor.name
      },
      context: {
        ...context,
        nodeVersion: process.version,
        platform: process.platform
      }
    };

    if (this.logToConsole) {
      console.error(JSON.stringify(errorLog, null, 2));
    }

    if (this.logToFile) {
      // Append to log file
      const fs = require('fs').promises;
      fs.appendFile(
        'errors.log',
        JSON.stringify(errorLog) + '\n'
      ).catch(console.error);
    }

    return errorLog;
  }
}

// Usage
const logger = new ErrorLogger({ logToFile: true });

try {
  await agent.execute(prompt);
} catch (error) {
  logger.log(error, {
    operation: 'agent_execution',
    prompt: prompt.substring(0, 100),
    agent: 'research_agent'
  });
}
```

---

## Pattern 8: NAPI-RS Specific Error Handling

```javascript
function handleNAPIError(error) {
  const message = error.message;

  // Enum as string instead of number
  if (message.includes('NumberExpected')) {
    console.error('❌ Use enum values, not strings');
    console.log('Fix: RetryableErrorType.NetworkError instead of "NetworkError"');
    return 'ENUM_ERROR';
  }

  // Null instead of undefined
  if (message.includes('StringExpected') ||message.includes('expected object')) {
    console.error('❌ Use undefined for optional fields, not null');
    console.log('Fix: Omit field entirely or set to undefined');
    return 'NULL_ERROR';
  }

  // Tool callback error (fatal)
  if (message.includes('GenericFailure')) {
    console.error('❌ FATAL: Tool callback threw error');
    console.log('Fix: Return error values instead of throwing');
    return 'TOOL_ERROR';
  }

  return 'UNKNOWN_NAPI_ERROR';
}

// Usage
try {
  await workflow.addNode(nodeConfig);
} catch (error) {
  const errorType = handleNAPIError(error);
  
  if (errorType === 'TOOL_ERROR') {
    console.error('Fatal NAPI error - restart required');
    process.exit(1);
  }
}
```

---

## Complete Example

```javascript
const { AgentBuilder, LlmConfig } = require('@infinitibit_gmbh/graphbit');

class ProductionAgent {
  constructor(config, options = {}) {
    this.config = config;
    this.retryOptions = {
      maxRetries: options.maxRetries || 3,
      baseDelay: options.baseDelay || 1000
    };
    this.breaker = new CircuitBreaker(5, 60000);
    this.logger = new ErrorLogger({ logToFile: true });
    this.agent = null;
  }

  async initialize() {
    this.agent = await new AgentBuilder('Production Agent', this.config)
      .systemPrompt('You are a helpful assistant')
      .temperature(0.7)
      .build();
  }

  async execute(prompt, timeout = 30000) {
    return await this.breaker.execute(async () => {
      return await retryWithBackoff(async () => {
        return await withTimeout(
          this.agent.execute(prompt),
          timeout
        );
      }, this.retryOptions.maxRetries, this.retryOptions.baseDelay);
    });
  }

  async safeExecute(prompt) {
    try {
      return {
        success: true,
        result: await this.execute(prompt)
      };
    } catch (error) {
      this.logger.log(error, { prompt, operation: 'execute' });

      return {
        success: false,
        error: error.message,
        fallback: 'Unable to process request. Please try again later.'
      };
    }
  }
}

// Usage
async function main() {
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  const agent = new ProductionAgent(config);
  await agent.initialize();

  const result = await agent.safeExecute('What is 2+2?');

  if (result.success) {
    console.log('Answer:', result.result);
  } else {
    console.log('Error:', result.error);
    console.log('Fallback:', result.fallback);
  }
}

main().catch(console.error);
```

---

## Best Practices

1. **Always use try-catch** for async operations
2. **Implement retries** for transient failures
3. **Use circuit breakers** to prevent cascading failures
4. **Log errors with context** for debugging
5. **Provide fallbacks** for critical operations
6. **Handle NAPI-RS errors** specifically
7. **Set timeouts** to prevent hanging
8. **Monitor error rates** in production

---

## Testing Error Handling

```javascript
// Simulate errors for testing
async function testErrorHandling() {
  // Test rate limit
  for (let i = 0; i < 100; i++) {
    await agent.execute('test');
  }

  // Test timeout
  await withTimeout(agent.execute('long task'), 100);

  // Test network error
  // (disconnect network)

  // Test invalid API key
  const badConfig = LlmConfig.openai({ apiKey: 'invalid' });
}
```

---

## Related Examples

- [RAG Pipeline](./rag-pipeline.md) - Apply error handling to RAG
- [Multi-Agent System](./multi-agent-system.md) - Multi-agent error handling
- [Production Deployment](./production-deployment.md) - Production patterns

---

**Example Created:** 2025-12-05  
**GraphBit Version:** 0.5.1  
**Difficulty:** Intermediate
