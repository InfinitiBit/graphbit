# Contributing to GraphBit JavaScript Bindings

Thank you for your interest in contributing to the GraphBit JavaScript bindings! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Prerequisites

- Node.js >= 16.0.0
- Rust >= 1.70.0
- Cargo
- Git

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/javascript

# Install dependencies
npm install

# Build the project
npm run build:debug

# Run tests
npm test
```

## Project Structure

```
javascript/
├── src/              # Rust source code for bindings
│   ├── lib.rs        # Main entry point
│   ├── errors.rs     # Error handling
│   ├── types.rs      # Type definitions
│   ├── llm.rs        # LLM provider bindings
│   ├── workflow.rs   # Workflow bindings
│   ├── agent.rs      # Agent bindings
│   └── ...
├── tests/            # Test suite
│   ├── unit/         # Unit tests
│   ├── integration/  # Integration tests
│   ├── types/        # Type tests
│   ├── benchmarks/   # Performance benchmarks
│   └── fixtures/     # Test data
├── docs/             # Documentation
├── Cargo.toml        # Rust dependencies
├── package.json      # Node.js dependencies
└── tsconfig.json     # TypeScript configuration
```

## Making Changes

### Adding New Bindings

1. **Add Rust Implementation**

   ```rust
   // src/new_feature.rs
   use napi::bindgen_prelude::*;
   use napi_derive::napi;

   #[napi]
   pub struct NewFeature {
       // implementation
   }
   ```

2. **Export from lib.rs**

   ```rust
   // src/lib.rs
   mod new_feature;
   pub use new_feature::*;
   ```

3. **Add TypeScript Definitions**

   ```typescript
   // src/index.d.ts
   export class NewFeature {
     // type definitions
   }
   ```

4. **Add Tests**
   ```typescript
   // tests/unit/new-feature.test.ts
   describe('NewFeature', () => {
     it('should work correctly', () => {
       // test implementation
     });
   });
   ```

### Updating Existing Bindings

1. Update Rust implementation
2. Update TypeScript definitions
3. Update tests
4. Update documentation
5. Add migration notes if breaking

## Testing

### Running Tests

```bash
# All tests
npm test

# Unit tests only
npm run test:unit

# Integration tests
npm run test:integration

# Type tests
npm run test:types

# With coverage
npm run test:coverage

# Benchmarks
npm run bench
```

### Writing Tests

#### Unit Tests

```typescript
import { describe, it, expect } from 'vitest';
import { YourFeature } from '../../index';

describe('YourFeature', () => {
  it('should do something', () => {
    const feature = new YourFeature();
    expect(feature).toBeDefined();
  });
});
```

#### Integration Tests

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { init } from '../../index';

describe('Integration Test', () => {
  beforeAll(() => {
    init();
  });

  it.skip('should work end-to-end', async () => {
    // Test with real API (skip by default)
  });
});
```

## Code Style

### Rust Code

Follow Rust standard style:

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

### TypeScript/JavaScript Code

Follow the project's ESLint and Prettier configuration:

```bash
# Format code
npm run format

# Lint code
npm run lint

# Fix linting issues
npm run lint:fix
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new text splitter strategy
fix: correct error handling in workflow execution
docs: update API reference
test: add integration tests for embeddings
chore: update dependencies
```

## Submitting Changes

### Pull Request Process

1. **Fork and Clone**

   ```bash
   git clone https://github.com/YOUR_USERNAME/graphbit.git
   cd graphbit/javascript
   ```

2. **Create Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Write code
   - Add tests
   - Update documentation

4. **Test Thoroughly**

   ```bash
   npm test
   npm run lint
   npm run typecheck
   ```

5. **Commit Changes**

   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

### PR Checklist

- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] TypeScript definitions updated
- [ ] CHANGELOG.md updated (for significant changes)
- [ ] No breaking changes (or documented in migration guide)

## Getting Help

- [GitHub Discussions](https://github.com/InfinitiBit/graphbit/discussions)
- [Discord Community](https://discord.gg/graphbit)
- [Documentation](https://docs.graphbit.ai)
