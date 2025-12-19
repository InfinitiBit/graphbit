# Installation Guide (JavaScript/TypeScript)

This guide will help you install GraphBit on your system and set up your development environment.

## System Requirements

- **Node.js**: 16.0.0 or higher
- **Operating System**: Linux, macOS, or Windows
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 1GB free space

---

## Installation

```bash
npm install @graphbit/core
```

---

## Verify Installation

Test your installation with this simple script:

```typescript
import { init, version, versionInfo, LlmConfig } from '@graphbit/core';

function main() {
  console.log('Testing GraphBit installation...');
  
  // Initialize
  init();
  
  // Check version
  console.log(`GraphBit version: ${version()}`);
  
  // Check detailed version info
  const info = versionInfo();
  console.log('Version Info:', info);
  
  // Test LLM configuration (requires API key)
  if (process.env.OPENAI_API_KEY) {
    const config = LlmConfig.openai({ 
      apiKey: process.env.OPENAI_API_KEY,
      model: 'gpt-4o-mini'
    });
    console.log('LLM Config created successfully');
  } else {
    console.log('No OPENAI_API_KEY found - set up API keys to use LLM features');
  }
  
  console.log('Installation successful!');
}

main();
```

Save this as `test_installation.ts` and run (using `ts-node`):

```bash
npx ts-node test_installation.ts
```

Expected output:
```
Testing GraphBit installation...
GraphBit version: 0.5.1
Version Info: { version: '0.5.1', rustVersion: '...', napiVersion: '...' }
LLM Config created successfully
Installation successful!
```

---

## Development Installation

For contributors:

```bash
# Clone and setup
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/javascript

# Install dependencies
npm install

# Build bindings
npm run build
```

---

## Next Steps

Once installed, proceed to the [Quick Start Tutorial](quickstart-js.md) to build your first AI agent!
