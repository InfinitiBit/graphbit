# Configuration Options - JavaScript

GraphBit's JavaScript bindings provide extensive configuration options to customize workflow execution, LLM providers, reliability features, and performance settings.

## Library Initialization

### Basic Initialization

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

// Basic initialization
init();
```

### Advanced Initialization

```typescript
import { init } from '@infinitibit_gmbh/graphbit';

// With debugging and logging
init({
  logLevel: 'info',        // Log level: trace, debug, info, warn, error
  enableTracing: true,     // Enable detailed tracing
  debug: true              // Enable debug mode (alias for enableTracing)
});
```

### Runtime Configuration

Configure the runtime before initialization for advanced control:

```typescript
import { configureRuntime, init } from '@infinitibit_gmbh/graphbit';

// Configure runtime (call before init)
configureRuntime({
  workerThreads: 8,          // Number of worker threads
  maxBlockingThreads: 16,    // Maximum blocking threads
  threadStackSizeMb: 8       // Thread stack size in MB
});

// Then initialize
init();
```

## LLM Configuration

### OpenAI Configuration

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Basic OpenAI configuration
const openaiConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini'        // Optional, defaults to gpt-4o-mini
});

// With default model
const openaiConfigAlt = LlmConfig.openai({ 
  apiKey: process.env.OPENAI_API_KEY || '' 
});
```

### Anthropic Configuration

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Basic Anthropic configuration
const anthropicConfig = LlmConfig.anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || '',
  model: 'claude-sonnet-4-20250514'  // Optional, defaults to claude-sonnet-4-20250514
});

// With default model
const anthropicConfigAlt = LlmConfig.anthropic({ 
  apiKey: process.env.ANTHROPIC_API_KEY || '' 
});
```

### Azure OpenAI Configuration

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Basic Azure OpenAI configuration
const azureConfig = LlmConfig.azureOpenai({
  apiKey: process.env.AZURE_OPENAI_API_KEY || '',
  deploymentName: 'gpt-4o-mini',  // Your Azure deployment name
  endpoint: process.env.AZURE_OPENAI_ENDPOINT || ''  // Your Azure OpenAI endpoint
});

// With custom API version
const azureConfigCustom = LlmConfig.azureOpenai({
  apiKey: process.env.AZURE_OPENAI_API_KEY || '',
  deploymentName: 'gpt-4o',
  endpoint: process.env.AZURE_OPENAI_ENDPOINT || '',
  apiVersion: '2024-10-21'  // Optional, defaults to '2024-10-21'
});
```

### Perplexity Configuration

> [!IMPORTANT]
> The `model` parameter is **required** for Perplexity.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Perplexity configuration
const perplexityConfig = LlmConfig.perplexity({
  apiKey: process.env.PERPLEXITY_API_KEY || '',
  model: 'llama-3.1-sonar-small-128k-online'  // Required
});

// Different models for specific use cases
const researchConfig = LlmConfig.perplexity({
  apiKey: process.env.PERPLEXITY_API_KEY || '',
  model: 'llama-3.1-sonar-large-128k-online'
});
```

### DeepSeek Configuration

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Basic DeepSeek configuration
const llmConfig = LlmConfig.deepseek({
  apiKey: 'your-deepseek-key',
  model: 'deepseek-chat'        // Optional, defaults to deepseek-chat
});

// With default model
const llmConfig = LlmConfig.deepseek({ apiKey: 'your-deepseek-key' });

// Different models for specific use cases
const codingConfig = LlmConfig.deepseek({ apiKey: 'your-deepseek-key', model: 'deepseek-coder' });
const reasoningConfig = LlmConfig.deepseek({ apiKey: 'your-deepseek-key', model: 'deepseek-reasoner' });
```

### MistralAI Configuration

> [!IMPORTANT]
> The `model` parameter is **required** for MistralAI.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// MistralAI configuration
const mistralConfig = LlmConfig.mistralai({
  apiKey: process.env.MISTRALAI_API_KEY || '',
  model: 'mistral-large-latest'  // Required
});

// Different models for specific use cases
const largeConfig = LlmConfig.mistralai({
  apiKey: process.env.MISTRALAI_API_KEY || '',
  model: 'mistral-large-latest'  // Most capable
});

const mediumConfig = LlmConfig.mistralai({
  apiKey: process.env.MISTRALAI_API_KEY || '',
  model: 'mistral-medium-latest'  // Balanced
});

const smallConfig = LlmConfig.mistralai({
  apiKey: process.env.MISTRALAI_API_KEY || '',
  model: 'mistral-small-latest'  // Fastest
});
```

### Ollama Configuration

> [!IMPORTANT]
> The `model` parameter is **required** for Ollama. Make sure the model is pulled locally with `ollama pull <model-name>` before use.

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// Local Ollama configuration
const ollamaConfig = LlmConfig.ollama({
  model: 'llama3.2'  // Required
});

// With custom Ollama server
const ollamaCustomConfig = LlmConfig.ollama({
  model: 'llama3.2',
  baseUrl: 'http://localhost:11434'  // Optional, defaults to http://localhost:11434
});

// Different models
const codingConfig = LlmConfig.ollama({ model: 'codellama' });
const chatConfig = LlmConfig.ollama({ model: 'mistral' });
```

### OpenRouter Configuration

```typescript
import 'dotenv/config';
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

// OpenRouter configuration
const llmConfig = LlmConfig.openrouter({
  apiKey: 'your-openrouter-key',
  model: 'anthropic/claude-3.5-sonnet'  // Model identifier
});
```

### Configuration Properties

```typescript
// Access configuration properties
const provider = llmConfig.provider();  // 'openai', 'azure_openai', 'anthropic', etc.
const model = llmConfig.model();        // Model name
```

## LLM Client Configuration

### Basic Client

```typescript
import { LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

// Simple client
const llmConfig = LlmConfig.openai({ apiKey: 'your-api-key' });
const client = new LlmClient(llmConfig);
```

### Client with Debug Mode

```typescript
import { LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

// Client with debugging enabled
const llmConfig = LlmConfig.openai({ apiKey: 'your-api-key' });
const client = new LlmClient(llmConfig, { debug: true });
```

### Client Statistics and Monitoring

```typescript
// Get performance statistics
const stats = client.getStats();
console.log(`Total requests: ${stats.totalRequests}`);
console.log(`Success rate: ${stats.successRate}`);
console.log(`Average response time: ${stats.averageResponseTimeMs}ms`);

// Reset statistics
client.resetStats();

// Warm up client for better performance
await client.warmup();
```

## Executor Configuration

### Basic Executor

```typescript
import { LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

// Simple executor
const llmConfig = LlmConfig.openai({ apiKey: 'your-api-key' });
const executor = new Executor(llmConfig);
```

### Executor with Options

```typescript
import { LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

// Executor with configuration
const llmConfig = LlmConfig.openai({ apiKey: 'your-api-key' });
const executor = new Executor(llmConfig, {
  lightweightMode: false,    // Enable lightweight/low-latency mode
  timeoutSeconds: 300,       // Execution timeout (1-3600 seconds)
  debug: true                // Enable debug mode
});
```

### Specialized Executors

#### High Throughput Executor

```typescript
import { Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

// Optimized for high throughput
const executor = Executor.newHighThroughput(llmConfig, {
  timeoutSeconds: 600,       // Optional timeout override
  debug: false               // Optional debug mode
});
```

#### Low Latency Executor

```typescript
import { Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

// Optimized for low latency
const executor = Executor.newLowLatency(llmConfig, {
  timeoutSeconds: 30,        // Shorter timeout for low latency
  debug: false
});
```

#### Balanced Executor

```typescript
import { Executor, LlmConfig } from '@infinitibit_gmbh/graphbit';

// Balanced configuration
const executor = Executor.newBalanced(llmConfig, {
  timeoutSeconds: 120,
  debug: false
});
```

### Runtime Configuration

```typescript
// Configure executor settings
executor.configure({
  timeoutSeconds: 600,       // Execution timeout (1-3600 seconds)
  maxRetries: 5,            // Maximum retries (0-10)
  enableMetrics: true,      // Enable performance metrics
  debug: false              // Debug mode
});

// Legacy configuration methods
executor.setLightweightMode(true);  // Enable lightweight mode
const isLightweight = executor.isLightweightMode();  // Check mode
```

### Executor Statistics

```typescript
// Get execution statistics
const stats = executor.getStats();
console.log(`Total executions: ${stats.totalExecutions}`);
console.log(`Success rate: ${stats.successRate}`);
console.log(`Average duration: ${stats.averageDurationMs}ms`);
console.log(`Execution mode: ${stats.executionMode}`);

// Reset statistics
executor.resetStats();

// Get current execution mode
const mode = executor.getExecutionMode();  // Returns: HighThroughput, LowLatency, etc.
```

## Embeddings Configuration

### OpenAI Embeddings

```typescript
import { EmbeddingConfig } from '@infinitibit_gmbh/graphbit';

// OpenAI embeddings configuration
const embedConfig = EmbeddingConfig.openai({
  apiKey: 'your-api-key',
  model: 'text-embedding-3-small'  // Optional, defaults to text-embedding-3-small
});

// With default model
const embedConfig = EmbeddingConfig.openai({ apiKey: 'your-api-key' });
```

### Embeddings Client

```typescript
import { EmbeddingConfig, EmbeddingClient } from '@infinitibit_gmbh/graphbit';

// Create embeddings client
const embedConfig = EmbeddingConfig.openai({ apiKey: 'your-api-key' });
const embedClient = new EmbeddingClient(embedConfig);

// Generate single embedding
const embedding = await embedClient.embed('Hello world');

// Generate multiple embeddings
const embeddings = await embedClient.embedMany(['Text 1', 'Text 2']);

// Calculate similarity
const similarity = EmbeddingClient.similarity(embedding1, embedding2);
```

## Environment Variables

### Required Environment Variables

```bash
# OpenAI
export OPENAI_API_KEY="your-openai-api-key"

# Anthropic
export ANTHROPIC_API_KEY="your-anthropic-api-key"
```

### GraphBit-Specific Environment Variables

```bash
# Runtime configuration
export GRAPHBIT_WORKER_THREADS="8"
export GRAPHBIT_MAX_BLOCKING_THREADS="16"

# Logging
export GRAPHBIT_LOG_LEVEL="INFO"
export GRAPHBIT_DEBUG="true"
```

## System Information and Health

### System Information

```typescript
import { getSystemInfo } from '@infinitibit_gmbh/graphbit';

// Get comprehensive system information
const info = getSystemInfo();
console.log(`Version: ${info.version}`);
console.log(`CPU count: ${info.cpuCount}`);
console.log(`Runtime initialized: ${info.runtimeInitialized}`);
console.log(`Worker threads: ${info.runtimeWorkerThreads}`);
console.log(`Memory allocator: ${info.memoryAllocator}`);
```

### Health Checks

```typescript
import { healthCheck } from '@infinitibit_gmbh/graphbit';

// Perform health check
const health = healthCheck();
if (health.overallHealthy) {
  console.log('✅ System is healthy');
  console.log(`Memory healthy: ${health.memoryHealthy}`);
  console.log(`Runtime healthy: ${health.runtimeHealthy}`);
} else {
  console.log('❌ System has issues');
}
```

### Version Information

```typescript
import { version } from '@infinitibit_gmbh/graphbit';

// Get current version
const graphbitVersion = version();
console.log(`GraphBit version: ${graphbitVersion}`);
```

## Configuration Examples

### Development Configuration

```typescript
import { init, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

function createDevConfig(): Executor {
  // Initialize with debugging
  init({ debug: true, logLevel: 'info' });

  // Use faster, cheaper model for development
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // Low-latency executor for development
  const executor = Executor.newLowLatency(config, {
    timeoutSeconds: 60,
    debug: true
  });

  return executor;
}
```

### Production Configuration

```typescript
import { init, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

function createProdConfig(): Executor {
  // Initialize without debugging
  init({ debug: false, logLevel: 'warn' });

  // High-quality model for production
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  });

  // High-throughput executor for production
  const executor = Executor.newHighThroughput(config, {
    timeoutSeconds: 300,
    debug: false
  });

  // Configure for production reliability
  executor.configure({
    timeoutSeconds: 300,
    maxRetries: 3,
    enableMetrics: true,
    debug: false
  });

  return executor;
}
```

### Local Development Configuration

```typescript
import { init, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

function createLocalConfig(): Executor {
  init({ debug: true, logLevel: 'debug' });

  // Use local Ollama
  const config = LlmConfig.ollama({ model: 'llama3.2' });

  // Low-latency for quick iteration
  const executor = Executor.newLowLatency(config, {
    timeoutSeconds: 180,  // Longer timeout for local inference
    debug: true
  });

  return executor;
}
```

## Configuration Validation

### Environment Validation

```typescript
function validateEnvironment(): void {
  // Check required environment variables
  const requiredVars: Record<string, string> = {
    OPENAI_API_KEY: 'OpenAI API key',
    // Add others as needed
  };

  const missingVars: string[] = [];
  for (const [varName, description] of Object.entries(requiredVars)) {
    if (!process.env[varName]) {
      missingVars.push(`${varName} (${description})`);
    }
  }

  if (missingVars.length > 0) {
    throw new Error(`Missing environment variables: ${missingVars.join(', ')}`);
  }

  console.log('✅ Environment validation passed');
}
```

### Configuration Testing

```typescript
import { LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function testConfiguration(config: LlmConfig): Promise<boolean> {
  try {
    // Create client
    const client = new LlmClient(config);

    // Test simple completion
    const response = await client.complete('Say "Configuration test successful"');

    if (response.toLowerCase().includes('successful')) {
      console.log('✅ Configuration test passed');
      return true;
    } else {
      console.log('❌ Configuration test failed - unexpected response');
      return false;
    }
  } catch (error) {
    console.error('❌ Configuration test failed:', error);
    return false;
  }
}
```

### Health Check Function

```typescript
import { healthCheck, getSystemInfo, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function comprehensiveHealthCheck(): Promise<boolean> {
  // Check system health
  const health = healthCheck();
  if (!health.overallHealthy) {
    console.error('❌ System health check failed');
    return false;
  }

  // Check system info
  const info = getSystemInfo();
  if (!info.runtimeInitialized) {
    console.error('❌ Runtime not initialized');
    return false;
  }

  // Test basic functionality
  try {
    const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY || '' });
    const client = new LlmClient(config);

    // Quick test
    const response = await client.complete('Test');
    if (!response) {
      console.error('❌ LLM test failed');
      return false;
    }
  } catch (error) {
    console.error('❌ LLM test failed:', error);
    return false;
  }

  console.log('✅ Comprehensive health check passed');
  return true;
}
```

## Best Practices

### 1. Environment-Based Configuration

```typescript
function getConfigForEnvironment(): Executor {
  const env = process.env.NODE_ENV || 'development';

  switch (env) {
    case 'production':
      return createProdConfig();
    case 'staging':
      return createStagingConfig();
    case 'local':
      return createLocalConfig();
    default:
      return createDevConfig();
  }
}
```

### 2. Secure Configuration

```typescript
import { LlmConfig } from '@infinitibit_gmbh/graphbit';

function secureConfigSetup(): LlmConfig {
  // Validate API key exists
  const apiKey = process.env.OPENAI_API_KEY;
  if (!apiKey) {
    throw new Error('OPENAI_API_KEY environment variable required');
  }

  // Validate API key format (basic check)
  if (apiKey.length < 20) {
    throw new Error('Invalid API key format');
  }

  const config = LlmConfig.openai({ apiKey });
  return config;
}
```

### 3. Performance Monitoring

```typescript
async function monitorPerformance(executor: Executor, workflow: Workflow): Promise<void> {
  // Get initial stats
  const initialStats = executor.getStats();

  const startTime = Date.now();

  // Execute workflow
  const result = await executor.execute(workflow);

  const endTime = Date.now();
  const executionTime = endTime - startTime;

  // Get final stats
  const finalStats = executor.getStats();

  // Log performance metrics
  console.log(`Execution time: ${executionTime}ms`);
  console.log(`Total executions: ${finalStats.totalExecutions}`);
  console.log(`Success rate: ${finalStats.successRate}`);

  // Alert on performance issues
  if (executionTime > 60000) {  // 60 second threshold
    console.warn('⚠️  Slow execution detected - consider tuning configuration');
  }

  if (finalStats.successRate < 0.95) {  // 95% success rate threshold
    console.warn('⚠️  Low success rate - check configuration and API health');
  }
}
```

### 4. Graceful Error Handling

```typescript
import { LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

async function robustConfigCreation(): Promise<Executor> {
  try {
    // Primary configuration
    const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY || '' });
    const executor = new Executor(config);

    // Test configuration
    if (await testConfiguration(config)) {
      return executor;
    } else {
      throw new Error('Configuration test failed');
    }
  } catch (error) {
    console.error('❌ Primary configuration failed:', error);

    try {
      // Fallback to local Ollama
      console.log('🔄 Falling back to local Ollama...');
      const fallbackConfig = LlmConfig.ollama();
      const fallbackExecutor = new Executor(fallbackConfig);

      if (await testConfiguration(fallbackConfig)) {
        return fallbackExecutor;
      } else {
        throw new Error('Fallback configuration failed');
      }
    } catch (fallbackError) {
      console.error('❌ Fallback configuration failed:', fallbackError);
      throw new Error('All configuration options exhausted');
    }
  }
}
```

### 5. Resource Cleanup

```typescript
import { shutdown } from '@infinitibit_gmbh/graphbit';

function cleanupResources(): void {
  try {
    // Shutdown GraphBit (for testing/cleanup)
    shutdown();
    console.log('✅ Resources cleaned up successfully');
  } catch (error) {
    console.error('❌ Error during cleanup:', error);
  }
}
```

## Configuration Troubleshooting

### Common Issues and Solutions

#### 1. API Key Issues

```typescript
// Check API key validity
const apiKey = process.env.OPENAI_API_KEY;
if (!apiKey) {
  console.error('❌ OPENAI_API_KEY not set');
} else if (apiKey.length < 20) {
  console.error('❌ API key appears invalid (too short)');
} else if (!apiKey.startsWith('sk-')) {
  console.error('❌ OpenAI API key should start with "sk-"');
} else {
  console.log('✅ API key format looks correct');
}
```

#### 2. Runtime Issues

```typescript
import { getSystemInfo } from '@infinitibit_gmbh/graphbit';

// Check runtime status
const info = getSystemInfo();
if (!info.runtimeInitialized) {
  console.error('❌ Runtime not initialized - call init()');
} else {
  console.log(`✅ Runtime initialized with ${info.runtimeWorkerThreads} workers`);
}
```

#### 3. Memory Issues

```typescript
import { healthCheck } from '@infinitibit_gmbh/graphbit';

// Check memory status
const health = healthCheck();
if (!health.memoryHealthy) {
  console.warn(`⚠️  Low memory: ${health.availableMemoryMb}MB available`);
  console.warn('Consider using memory-optimized executor');
} else {
  console.log('✅ Memory status OK');
}
```

## TypeScript Configuration

### Type Safety

```typescript
import { LlmConfig, Executor, ExecutorOptions } from '@infinitibit_gmbh/graphbit';

// Define configuration with types
interface AppConfig {
  llm: LlmConfig;
  executor: ExecutorOptions;
}

const appConfig: AppConfig = {
  llm: LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o-mini'
  }),
  executor: {
    timeoutSeconds: 300,
    debug: false,
    lightweightMode: false
  }
};

// Create executor with typed config
const executor = new Executor(appConfig.llm, appConfig.executor);
```

### Configuration Interfaces

```typescript
interface LlmProviderConfig {
  provider: 'openai' | 'anthropic' | 'ollama' | 'deepseek';
  apiKey?: string;
  model?: string;
}

function createConfigFromInterface(config: LlmProviderConfig): LlmConfig {
  switch (config.provider) {
    case 'openai':
      return LlmConfig.openai({ apiKey: config.apiKey!, model: config.model });
    case 'anthropic':
      return LlmConfig.anthropic({ apiKey: config.apiKey!, model: config.model });
    case 'ollama':
      return LlmConfig.ollama({ model: config.model });
    case 'deepseek':
      return LlmConfig.deepseek({ apiKey: config.apiKey!, model: config.model });
    default:
      throw new Error(`Unknown provider: ${config.provider}`);
  }
}
```

Proper configuration is essential for optimal GraphBit performance in JavaScript/TypeScript applications. Choose settings that match your use case, environment, and performance requirements.

## See Also

- [JavaScript API Reference](javascript-api.md)
- [Node Types Reference](node-types-js.md)
- [Quick Start Guide](../getting-started/quickstart-js.md)
- [Performance Optimization](../user-guide/performance-js.md)
