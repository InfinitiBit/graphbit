# Production Deployment Guide

**Level:** Advanced  
**Estimated Time:** 1 hour  
**Prerequisites:** Completed GraphBit application

## Overview

This guide covers deploying GraphBit applications to production with best practices for:

1. Environment configuration
2. Performance optimization
3. Monitoring and logging
4. Security
5. Scaling strategies

---

## 1. Environment Configuration

### Environment Variables

```bash
# .env.production
NODE_ENV=production

# API Keys (use secrets management in production)
OPENAI_API_KEY=sk-proj-...
ANTHROPIC_API_KEY=sk-ant-...

# Application Config
PORT=3000
LOG_LEVEL=info
MAX_RETRIES=3

# Performance
REQUEST_TIMEOUT=30000
MAX_CONCURRENT_REQUESTS=10

# Monitoring
SENTRY_DSN=https://...
DATADOG_API_KEY=...
```

### Loading Configuration

```javascript
const dotenv = require('dotenv');
const path = require('path');

// Load environment-specific config
const envFile = process.env.NODE_ENV === 'production' 
  ? '.env.production' 
  : '.env.development';

dotenv.config({ path: path.resolve(__dirname, envFile) });

const config = {
  env: process.env.NODE_ENV || 'development',
  port: parseInt(process.env.PORT, 10) || 3000,
  apiKeys: {
    openai: process.env.OPENAI_API_KEY,
    anthropic: process.env.ANTHROPIC_API_KEY
  },
  timeouts: {
    request: parseInt(process.env.REQUEST_TIMEOUT, 10) || 30000
  },
  maxConcurrent: parseInt(process.env.MAX_CONCURRENT_REQUESTS, 10) || 10
};

module.exports = config;
```

---

## 2. Performance Optimization

### Connection Pooling

```javascript
class AgentPool {
  constructor(config, poolSize = 5) {
    this.config = config;
    this.poolSize = poolSize;
    this.agents = [];
    this.available = [];
  }

  async initialize() {
    console.log(`Initializing agent pool (size: ${this.poolSize})`);

    for (let i = 0; i < this.poolSize; i++) {
      const agent = await new AgentBuilder(`Agent-${i}`, this.config)
        .temperature(0.7)
        .build();
      
      this.agents.push(agent);
      this.available.push(agent);
    }

    console.log('Agent pool ready');
  }

  async execute(prompt) {
    // Wait for available agent
    while (this.available.length === 0) {
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    const agent = this.available.pop();

    try {
      const result = await agent.execute(prompt);
      return result;
    } finally {
      this.available.push(agent);  // Return to pool
    }
  }

  getMetrics() {
    return {
      total: this.poolSize,
      available: this.available.length,
      inUse: this.poolSize - this.available.length
    };
  }
}

// Usage
const pool = new AgentPool(llmConfig, 10);
await pool.initialize();

// All requests use the pool
const response = await pool.execute(userPrompt);
```

### Caching Strategy

```javascript
const NodeCache = require('node-cache');

class CachedEmbeddingClient {
  constructor(config, options = {}) {
    this.client = new EmbeddingClient(config);
    this.cache = new NodeCache({
      stdTTL: options.cacheTTL || 3600,  // 1 hour
      checkperiod: 600  // Check for expired keys every 10min
    });
  }

  async embed(texts) {
    const cacheKey = this.generateCacheKey(texts);
    
    // Check cache
    const cached = this.cache.get(cacheKey);
    if (cached) {
      console.log('Cache hit');
      return cached;
    }

    // Cache miss - call API
    console.log('Cache miss');
    const result = await this.client.embed(texts);
    
    // Store in cache
    this.cache.set(cacheKey, result);
    
    return result;
  }

  generateCacheKey(texts) {
    const crypto = require('crypto');
    return crypto
      .createHash('md5')
      .update(JSON.stringify(texts))
      .digest('hex');
  }

  getStats() {
    return this.cache.getStats();
  }
}
```

### Request Queuing

```javascript
const PQueue = require('p-queue').default;

class RateLimitedAgent {
  constructor(agent, options = {}) {
    this.agent = agent;
    this.queue = new PQueue({
      concurrency: options.concurrency || 5,
      interval: options.interval || 1000,
      intervalCap: options.intervalCap || 10
    });
  }

  async execute(prompt) {
    return this.queue.add(async () => {
      return await this.agent.execute(prompt);
    });
  }

  getQueueSize() {
    return this.queue.size + this.queue.pending;
  }
}

// Usage: Max 10 requests per second
const rateLimitedAgent = new RateLimitedAgent(agent, {
  concurrency: 5,
  interval: 1000,
  intervalCap: 10
});
```

---

## 3. Monitoring and Logging

### Structured Logging

```javascript
const winston = require('winston');

const logger = winston.createLogger({
  level: process.env.LOG_LEVEL || 'info',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.errors({ stack: true }),
    winston.format.json()
  ),
  defaultMeta: { service: 'graphbit-app' },
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

if (process.env.NODE_ENV !== 'production') {
  logger.add(new winston.transports.Console({
    format: winston.format.simple()
  }));
}

// Usage
logger.info('Agent execution started', { prompt: userPrompt });
logger.error('Agent execution failed', { error: error.message,stack: error.stack });
```

### Metrics Collection

```javascript
class MetricsCollector {
  constructor() {
    this.metrics = {
      requests: 0,
      successes: 0,
      failures: 0,
      totalLatency: 0,
      errors: {}
    };
  }

  recordRequest(success, latency, error = null) {
    this.metrics.requests++;
    
    if (success) {
      this.metrics.successes++;
    } else {
      this.metrics.failures++;
      
      const errorType = error?.constructor.name || 'Unknown';
      this.metrics.errors[errorType] = (this.metrics.errors[errorType] || 0) + 1;
    }
    
    this.metrics.totalLatency += latency;
  }

  getMetrics() {
    return {
      ...this.metrics,
      successRate: (this.metrics.successes / this.metrics.requests * 100).toFixed(2),
      avgLatency: (this.metrics.totalLatency / this.metrics.requests).toFixed(2)
    };
  }

  reset() {
    this.metrics = {
      requests: 0,
      successes: 0,
      failures: 0,
      totalLatency: 0,
      errors: {}
    };
  }
}

// Usage
const metrics = new MetricsCollector();

async function instrumentedExecute(agent, prompt) {
  const startTime = Date.now();
  let success = false;
  let error = null;

  try {
    const result = await agent.execute(prompt);
    success = true;
    return result;
  } catch (e) {
    error = e;
    throw e;
  } finally {
    const latency = Date.now() - startTime;
    metrics.recordRequest(success, latency, error);
  }
}
```

### Health Checks

```javascript
const express = require('express');
const app = express();

app.get('/health', async (req, res) => {
  const health = {
    status: 'ok',
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    checks: {}
  };

  // Check agent pool
  try {
    const poolMetrics = agentPool.getMetrics();
    health.checks.agentPool = {
      status: poolMetrics.available > 0 ? 'healthy' : 'degraded',
      ...poolMetrics
    };
  } catch (error) {
    health.checks.agentPool = { status: 'unhealthy', error: error.message };
    health.status = 'degraded';
  }

  // Check API connectivity
  try {
    await testAgent.execute('health check');
    health.checks.llmAPI = { status: 'healthy' };
  } catch (error) {
    health.checks.llmAPI = { status: 'unhealthy', error: error.message };
    health.status = 'degraded';
  }

  const statusCode = health.status === 'ok' ? 200 : 503;
  res.status(statusCode).json(health);
});

app.get('/metrics', (req, res) => {
  res.json(metrics.getMetrics());
});
```

---

## 4. Security Best Practices

### API Key Management

```javascript
// ❌ DON'T: Hardcode API keys
const config = LlmConfig.openai({ apiKey: 'sk-proj-abc123...' });

// ✅ DO: Use environment variables
const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });

// ✅ BETTER: Use secrets manager (AWS Secrets Manager, etc.)
const AWS = require('aws-sdk');

async function getApiKey() {
  const secretsManager = new AWS.SecretsManager();
  const data = await secretsManager.getSecretValue({ SecretId: 'openai-key' }).promise();
  return JSON.parse(data.SecretString).apiKey;
}
```

### Input Validation

```javascript
function validatePrompt(prompt) {
  if (!prompt || typeof prompt !== 'string') {
    throw new Error('Prompt must be a non-empty string');
  }

  if (prompt.length > 10000) {
    throw new Error('Prompt too long (max 10000 chars)');
  }

  // Sanitize potentially harmful content
  const sanitized = prompt
    .replace(/<script>/gi, '')
    .replace(/javascript:/gi, '')
    .trim();

  return sanitized;
}

// Usage
app.post('/api/generate', async (req, res) => {
  try {
    const prompt = validatePrompt(req.body.prompt);
    const result = await agent.execute(prompt);
    res.json({ result });
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});
```

### Rate Limiting

```javascript
const rateLimit = require('express-rate-limit');

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,  // 15 minutes
  max: 100,  // Max 100 requests per window
  message: 'Too many requests, please try again later'
});

app.use('/api/', limiter);
```

---

## 5. Deployment Patterns

### Docker Deployment

```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy application code
COPY . .

# Run as non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001
USER nodejs

EXPOSE 3000

CMD ["node", "server.js"]
```

```yaml
# docker-compose.yml
version: '3.8'

services:
  graphbit-app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - OPENAI_API_KEY=${OPENAI_API_KEY}
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    restart: unless-stopped
```

### Kubernetes Deployment

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphbit-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: graphbit
  template:
    metadata:
      labels:
        app: graphbit
    spec:
      containers:
      - name: graphbit
        image: your-registry/graphbit-app:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_ENV
          value: "production"
        - name: OPENAI_API_KEY
          valueFrom:
            secretKeyRef:
              name: api-keys
              key: openai-key
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: graphbit-service
spec:
  selector:
    app: graphbit
  ports:
  - port: 80
    targetPort: 3000
  type: LoadBalancer
```

---

## 6. Complete Production Setup

```javascript
// server.js
const express = require('express');
const helmet = require('helmet');
const cors = require('cors');
const { init, AgentBuilder, LlmConfig } = require('graphbit');
const config = require('./config');
const logger = require('./logger');
const metrics = require('./metrics');

class ProductionServer {
  constructor() {
    this.app = express();
    this.agentPool = null;
  }

  async initialize() {
    // Initialize GraphBit
    init();

    // Configure Express
    this.app.use(helmet());  // Security headers
    this.app.use(cors());
    this.app.use(express.json());

    // Request logging
    this.app.use((req, res, next) => {
      logger.info('Request received', {
        method: req.method,
        path: req.path,
        ip: req.ip
      });
      next();
    });

    // Initialize agent pool
    const llmConfig = LlmConfig.openai({
      apiKey: config.apiKeys.openai,
      model: 'gpt-4o-mini'
    });

    this.agentPool = new AgentPool(llmConfig, 10);
    await this.agentPool.initialize();

    // Routes
    this.setupRoutes();

    // Error handling
    this.setupErrorHandling();

    logger.info('Server initialized');
  }

  setupRoutes() {
    // Health check
    this.app.get('/health', (req, res) => {
      res.json({
        status: 'ok',
        timestamp: new Date().toISOString(),
        poolMetrics: this.agentPool.getMetrics()
      });
    });

    // Metrics
    this.app.get('/metrics', (req, res) => {
      res.json(metrics.getMetrics());
    });

    // Main API
    this.app.post('/api/generate', async (req, res) => {
      const startTime = Date.now();

      try {
        const { prompt } = req.body;

        if (!prompt) {
          return res.status(400).json({ error: 'Prompt required' });
        }

        const result = await this.agentPool.execute(prompt);

        const latency = Date.now() - startTime;
        metrics.recordRequest(true, latency);

        res.json({
          result,
          latency: `${latency}ms`
        });

      } catch (error) {
        const latency = Date.now() - startTime;
        metrics.recordRequest(false, latency, error);

        logger.error('Request failed', {
          error: error.message,
          stack: error.stack
        });

        res.status(500).json({
          error: 'Internal server error'
        });
      }
    });
  }

  setupErrorHandling() {
    // 404 handler
    this.app.use((req, res) => {
      res.status(404).json({ error: 'Not found' });
    });

    // Global error handler
    this.app.use((err, req, res, next) => {
      logger.error('Unhandled error', {
        error: err.message,
        stack: err.stack
      });

      res.status(500).json({ error: 'Internal server error' });
    });
  }

  start() {
    const port = config.port;
    this.app.listen(port, () => {
      logger.info(`Server running on port ${port}`);
      console.log(`✅ Production server running on port ${port}`);
    });
  }

  async shutdown() {
    logger.info('Server shutting down');
    process.exit(0);
  }
}

// Graceful shutdown
process.on('SIGTERM', async () => {
  logger.info('SIGTERM received');
  await server.shutdown();
});

process.on('SIGINT', async () => {
  logger.info('SIGINT received');
  await server.shutdown();
});

// Start server
const server = new ProductionServer();
server.initialize()
  .then(() => server.start())
  .catch(error => {
    logger.error('Failed to start server', { error: error.message });
    process.exit(1);
  });
```

---

## 7. Monitoring Dashboard

```javascript
// metrics-dashboard.js
const express = require('express');
const app = express();

app.get('/dashboard', (req, res) => {
  const metricsData = metrics.getMetrics();
  const poolMetrics = agentPool.getMetrics();

  res.send(`
    <!DOCTYPE html>
    <html>
    <head>
      <title>GraphBit Metrics</title>
      <meta http-equiv="refresh" content="5">
      <style>
        body { font-family: Arial; margin: 20px; }
        .metric { margin: 10px 0; padding: 10px; background: #f0f0f0; }
        .healthy { color: green; }
        .warning { color: orange; }
        .error { color: red; }
      </style>
    </head>
    <body>
      <h1>GraphBit Production Metrics</h1>
      
      <div class="metric">
        <h2>Requests</h2>
        <p>Total: ${metricsData.requests}</p>
        <p class="${metricsData.successRate >95 ? 'healthy' : 'warning'}">
          Success Rate: ${metricsData.successRate}%
        </p>
        <p>Average Latency: ${metricsData.avgLatency}ms</p>
      </div>

      <div class="metric">
        <h2>Agent Pool</h2>
        <p>Total Agents: ${poolMetrics.total}</p>
        <p>Available: ${poolMetrics.available}</p>
        <p>In Use: ${poolMetrics.inUse}</p>
      </div>

      <div class="metric">
        <h2>Errors</h2>
        <pre>${JSON.stringify(metricsData.errors, null, 2)}</pre>
      </div>
    </body>
    </html>
  `);
});
```

---

## 8. Checklist for Production

### Pre-Deployment

- [ ] Environment variables configured
- [ ] API keys in secrets manager
- [ ] Logging configured
- [ ] Metrics collection enabled
- [ ] Health checks implemented
- [ ] Rate limiting configured
- [ ] Input validation implemented
- [ ] Error handling comprehensive

### Deployment

- [ ] Docker image built and tested
- [ ] Resource limits set
- [ ] Auto-scaling configured
- [ ] Load balancer configured
- [ ] SSL/TLS certificates installed
- [ ] Monitoring alerts configured

### Post-Deployment

- [ ] Health checks passing
- [ ] Metrics being collected
- [ ] Logs being aggregated
- [ ] Error rates within acceptable limits
- [ ] Performance benchmarks met
- [ ] Backup and recovery tested

---

## Related Examples

- [Error Handling](./error-handling.md) - Production error patterns
- [RAG Pipeline](./rag-pipeline.md) - Example application
- [Multi-Agent System](./multi-agent-system.md) - Complex workflows

---

**Guide Created:** 2025-12-05  
**GraphBit Version:** 0.5.1  
**Difficulty:** Advanced
