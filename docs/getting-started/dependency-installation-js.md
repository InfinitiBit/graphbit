# Dependency Installation - JavaScript

This guide covers installing GraphBit and its dependencies in JavaScript/Node.js environments.

## System Requirements

### Node.js

**Minimum**: Node.js 16.0.0 or later

**Recommended**: Node.js 18.0.0 or later (LTS)

**Check your version**:
```bash
node --version
```

**Installation**:
- **macOS**: `brew install node`
- **Windows**: Download from https://nodejs.org or `choco install nodejs`
- **Linux**: `apt-get install nodejs npm` or equivalent

### npm

Included with Node.js.

**Check version**:
```bash
npm --version
```

**Update npm**:
```bash
npm install -g npm@latest
```

### Operating System Support

GraphBit supports:
- ✅ Linux (x86_64, aarch64)
- ✅ macOS (x86_64, arm64/M1/M2)
- ✅ Windows (x86_64)

## Installing GraphBit

### Step 1: Create Project

```bash
mkdir my-graphbit-project
cd my-graphbit-project

# Initialize npm project
npm init -y
```

### Step 2: Install GraphBit Package

```bash
npm install @infinitibit_gmbh/graphbit
```

**Verify installation**:
```bash
npm list @infinitibit_gmbh/graphbit
```

**Expected output**:
```
my-graphbit-project@1.0.0
└── @infinitibit_gmbh/graphbit@0.5.1
```

### Step 3: Verify Native Module

The JavaScript bindings are built as native modules (.node files). During installation, the correct binary for your platform should be downloaded automatically.

**Check installed binary**:
```bash
# Linux/macOS
ls node_modules/@infinitibit_gmbh/graphbit/*.node

# Windows
dir node_modules\@infinitibit_gmbh\graphbit\*.node
```

**If binary is missing**:
```bash
npm rebuild @infinitibit_gmbh/graphbit
```

### Step 4: Test Installation

Create a test file `test.js`:

```javascript
const { init, getSystemInfo } = require('@infinitibit_gmbh/graphbit');

init();
const info = getSystemInfo();
console.log('GraphBit initialized successfully!');
console.log('System info:', info);
```

Run it:
```bash
node test.js
```

**Expected output**:
```
GraphBit initialized successfully!
System info: { nodeVersion: 'v18.17.0', cpuCount: 8, ... }
```

## Installing TypeScript (Optional but Recommended)

### Step 1: Install TypeScript

```bash
npm install --save-dev typescript ts-node @types/node
```

### Step 2: Initialize TypeScript

```bash
npx tsc --init
```

This creates `tsconfig.json`. Update it:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules"]
}
```

### Step 3: Create TypeScript Example

Create `src/example.ts`:

```typescript
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

async function main() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || 'test',
    model: 'gpt-4o-mini'
  });

  const client = new LlmClient(config);
  
  try {
    const result = await client.complete('Hello, world!');
    console.log('Response:', result);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

### Step 4: Run TypeScript Code

```bash
# Direct execution
npx ts-node src/example.ts

# Or compile and run
npx tsc
node dist/example.js
```

## Installing Development Dependencies

### TypeScript & Tooling

```bash
npm install --save-dev \
  typescript \
  ts-node \
  @types/node \
  eslint \
  prettier
```

### Testing Framework (Vitest)

```bash
npm install --save-dev vitest
```

Create `vitest.config.ts`:

```typescript
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    testTimeout: 30000
  }
});
```

Create test file `src/example.test.ts`:

```typescript
import { describe, it, expect } from 'vitest';
import { init } from '@infinitibit_gmbh/graphbit';

describe('GraphBit', () => {
  it('should initialize', () => {
    expect(() => init()).not.toThrow();
  });
});
```

Run tests:
```bash
npx vitest
```

### Web Development (Optional)

If building a web API or frontend that uses GraphBit:

```bash
npm install --save-dev \
  express \
  @types/express \
  cors \
  dotenv
```

Create `src/server.ts`:

```typescript
import express from 'express';
import { init, LlmConfig, LlmClient } from '@infinitibit_gmbh/graphbit';

const app = express();
app.use(express.json());

init();

app.post('/api/complete', async (req, res) => {
  try {
    const { prompt } = req.body;
    
    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY
    });
    
    const client = new LlmClient(config);
    const result = await client.complete(prompt);
    
    res.json({ result });
  } catch (error) {
    res.status(500).json({ 
      error: error instanceof Error ? error.message : 'Unknown error' 
    });
  }
});

app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
```

## Environment Configuration

### API Keys

Set environment variables for LLM providers:

**Linux/macOS**:
```bash
export OPENAI_API_KEY=sk-...
export ANTHROPIC_API_KEY=sk-ant-...
```

**Windows (PowerShell)**:
```powershell
$env:OPENAI_API_KEY='sk-...'
$env:ANTHROPIC_API_KEY='sk-ant-...'
```

**Windows (CMD)**:
```cmd
set OPENAI_API_KEY=sk-...
set ANTHROPIC_API_KEY=sk-ant-...
```

**Using .env file** (with `dotenv` package):

Create `.env`:
```
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
OLLAMA_URL=http://localhost:11434
```

Load in your code:
```typescript
import dotenv from 'dotenv';

dotenv.config();

const apiKey = process.env.OPENAI_API_KEY;
```

## Troubleshooting Installation

### Error: Cannot find module '@infinitibit_gmbh/graphbit'

**Cause**: Package not installed

**Solution**:
```bash
npm install @infinitibit_gmbh/graphbit
npm list @infinitibit_gmbh/graphbit
```

### Error: Native module failed to load

**Cause**: Binary not compatible with your Node.js version/platform

**Solution**:
```bash
# Check Node.js version
node --version  # Must be >= 16.0.0

# Rebuild native module
npm rebuild @infinitibit_gmbh/graphbit

# Or reinstall
rm -rf node_modules/@infinitibit_gmbh/graphbit
npm install @infinitibit_gmbh/graphbit
```

### Error: EACCES permission denied (on Linux/macOS)

**Cause**: npm needs elevated permissions

**Solution** (don't use sudo):
```bash
npm install --global-style @infinitibit_gmbh/graphbit
# or
npm install --no-save @infinitibit_gmbh/graphbit
```

### Error: Different platform binary installed

**Cause**: Installing on Linux, then running on macOS/Windows (or vice versa)

**Solution**:
```bash
# Clear npm cache
npm cache clean --force

# Reinstall for current platform
npm install @infinitibit_gmbh/graphbit
```

### Error: TypeScript files not found

**Cause**: TypeScript definitions not installed

**Solution**:
```bash
# Definitions are bundled with @infinitibit_gmbh/graphbit
# Ensure TypeScript is installed
npm install --save-dev typescript

# Clear cache and reinstall
npm cache clean --force
npm install
```

## Monorepo Setup (Advanced)

If using GraphBit in a monorepo with workspaces:

**package.json**:
```json
{
  "name": "graphbit-monorepo",
  "private": true,
  "workspaces": [
    "packages/*"
  ]
}
```

**packages/api/package.json**:
```json
{
  "name": "@my-org/api",
  "dependencies": {
    "@infinitibit_gmbh/graphbit": "^0.5.1"
  }
}
```

**Installation**:
```bash
npm install
npm run build -w packages/api
```

## Docker Installation

### Dockerfile

```dockerfile
FROM node:18-slim

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy application code
COPY src ./src

# Copy TypeScript config
COPY tsconfig.json .

# Build TypeScript
RUN npm run build

# Run application
CMD ["node", "dist/index.js"]
```

### docker-compose.yml

```yaml
version: '3.9'

services:
  graphbit-app:
    build: .
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
      - NODE_ENV=production
    ports:
      - "3000:3000"
```

Run:
```bash
docker-compose up
```

## Version Management

### Check Installed Version

```bash
npm list @infinitibit_gmbh/graphbit
```

### Update to Latest Version

```bash
npm update @infinitibit_gmbh/graphbit
```

### Update to Specific Version

```bash
npm install @infinitibit_gmbh/graphbit@0.6.0
```

### Lock Version

In `package.json`:
```json
{
  "dependencies": {
    "@infinitibit_gmbh/graphbit": "0.5.1"
  }
}
```

Or use lock file:
```bash
npm ci  # Uses package-lock.json for exact versions
```

## Performance Optimization

### Native Module Caching

GraphBit caches compiled bindings. For CI/CD:

```bash
# Restore from cache before install
npm ci

# Build/test
npm run build

# Cache node_modules for next run
```

### Memory Optimization

For resource-constrained environments:

**package.json**:
```json
{
  "scripts": {
    "start": "NODE_OPTIONS=--max-old-space-size=512 node dist/index.js"
  }
}
```

## Next Steps

After installation:

1. **Read the Quick Start**: [JavaScript Getting Started](../getting-started/javascript-quickstart.md)
2. **Explore Examples**: [Examples in Repository](../../examples/)
3. **API Reference**: [JavaScript API Reference](../api-reference/javascript-api.md)
4. **Learn Architecture**: [Architecture Guide](../development/architecture-js.md)

## Getting Help

- **Installation issues**: Check [Debugging Guide](./debugging-js.md)
- **API questions**: See [JavaScript API Reference](../api-reference/javascript-api.md)
- **Examples**: Check [Examples Directory](../../examples/)
- **GitHub Issues**: https://github.com/InfinitiBit/graphbit/issues
