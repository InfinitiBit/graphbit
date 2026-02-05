# Contributing to GraphBit - JavaScript/TypeScript

Welcome to contributing to GraphBit's JavaScript/TypeScript bindings! This guide covers everything you need to know about developing, testing, and contributing to the JavaScript ecosystem of GraphBit.

> **Note**: This document is specific to JavaScript/TypeScript development. For Rust core or Python contributions, see the main [Contributing Guide](contributing.md).

## Quick Start for JavaScript/TypeScript Contributors

### 1. Development Setup

```bash
# Clone the repository
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit/javascript

# Install dependencies
npm install

# Build the native module
npm run build

# Run tests to verify setup
npm test
```

### 2. Project Structure

```
javascript/
├── src/              # Rust source for napi-rs bindings
│   └── lib.rs       # Main binding entry point
├── tests/           # TypeScript/JavaScript tests
├── examples/        # Example applications
├── index.d.ts       # TypeScript type definitions
├── package.json     # Node.js package configuration
├── tsconfig.json    # TypeScript configuration
├── Cargo.toml       # Rust dependencies
└── build.rs         # Build script
```

## Ways to Contribute

### 🐛 Bug Reports

File JavaScript-specific issues with:
- Node.js version (`node --version`)
- Operating system
- Package version
- Minimal reproducible code example

```typescript
// Example bug report code
import { init, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

init();
const config = LlmConfig.openai({ apiKey: 'test' });
// ... steps to reproduce bug
```

### ✨ Feature Requests

For JavaScript features:
- Describe the JavaScript API you envision
- Provide TypeScript interface proposals
- Include usage examples
- Consider compatibility with Node.js versions

### 📚 Documentation

Improve JavaScript documentation:
- Add TypeScript examples
- Create tutorials and guides
- Document common patterns
- Add JSDoc comments

### 🧹 Code Contributions

Follow JavaScript best practices:
- Write TypeScript for type safety
- Include tests with vitest
- Follow ESLint rules
- Update type definitions

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/graphbit.git
cd graphbit
```

### 2. Create Feature Branch

```bash
git checkout -b feature/js-new-feature
```

### 3. Development Cycle

```bash
# Make changes to JavaScript/TypeScript or Rust bindings

# Build the module
npm run build

# Run tests
npm test

# Run specific test file
npm test -- path/to/test.ts

# Run tests in watch mode
npm run test:watch

# Lint code
npm run lint

# Format code
npm run format
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat(js): add new feature"
```

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat(js):` - New feature
- `fix(js):` - Bug fix
- `docs(js):` - Documentation
- `test(js):` - Tests
- `refactor(js):` - Code refactoring
- `chore(js):` - Maintenance

### 5. Submit Pull Request

- Push to your fork
- Create pull request to `main`
- Fill out PR template
- Wait for CI checks and review

## Code Quality Standards

### TypeScript Code Style

```typescript
// Use explicit types
function processData(input: string): Promise<Result> {
  // Implementation
}

// Use interfaces for objects
interface WorkflowConfig {
  name: string;
  timeout?: number;
}

// Use async/await (not callbacks)
async function executeWorkflow() {
  const result = await executor.execute(workflow);
  return result;
}

// Handle errors explicitly
try {
  const result = await executor.execute(workflow);
} catch (error) {
  console.error('Execution failed:', error);
}
```

### Testing Guidelines

```typescript
import { describe, it, expect, beforeEach } from 'vitest';
import { init, LlmConfig, Workflow, Node, Executor } from '@infinitibit_gmbh/graphbit';

describe('Workflow Execution', () => {
  beforeEach(() => {
    init();
  });

  it('should execute simple workflow', async () => {
    const config = LlmConfig.ollama({ model: 'llama3.2' });
    const executor = new Executor(config);
    
    const workflow = new Workflow('Test Workflow');
    const node = Node.agent('Test Agent', 'Say hello', 'agent1');
    
    await workflow.addNode(node);
    await workflow.validate();
    
    const result = await executor.execute(workflow);
    
    expect(result.isSuccess()).toBe(true);
  });

  it('should handle errors gracefully', async () => {
    // Test error handling
  });
});
```

### Documentation Standards

```typescript
/**
 * Executes a workflow with the configured LLM.
 * 
 * @param workflow - The workflow to execute
 * @returns A promise that resolves to the execution result
 * @throws {Error} If workflow validation fails
 * 
 * @example
 * ```typescript
 * const executor = new Executor(config);
 * const result = await executor.execute(workflow);
 * 
 * if (result.isSuccess()) {
 *   console.log(result.variables());
 * }
 * ```
 */
async execute(workflow: Workflow): Promise<WorkflowResult>;
```

## Building and Testing

### Build Commands

```bash
# Development build
npm run build

# Production build (optimized)
npm run build:release

# Clean build artifacts
npm run clean

# Rebuild from scratch
npm run rebuild
```

### Testing

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run specific test suite
npm test -- --grep "Workflow"

# Run tests in watch mode
npm run test:watch

# Run integration tests
npm run test:integration
```

### Linting and Formatting

```bash
# Lint TypeScript code
npm run lint

# Fix linting issues
npm run lint:fix

# Format code with Prettier
npm run format

# Check formatting
npm run format:check
```

## Architecture Overview

### napi-rs Binding Layer

GraphBit's JavaScript bindings use [napi-rs](https://napi.rs/) for Rust-Node.js interop:

```rust
// src/lib.rs
#[napi]
pub struct Executor {
    inner: Arc<RustExecutor>,
}

#[napi]
impl Executor {
    #[napi(constructor)]
    pub fn new(config: &LlmConfig) -> napi::Result<Self> {
        // Convert JS config to Rust
        Ok(Self {
            inner: Arc::new(RustExecutor::new(config.inner.clone()))
        })
    }

    #[napi]
    pub async fn execute(&self, workflow: &Workflow) -> napi::Result<WorkflowResult> {
        // Execute workflow
    }
}
```

### TypeScript Type Definitions

Types are auto-generated by napi-rs but can be supplemented:

```typescript
// Custom types in index.d.ts
export interface ExecutorOptions {
  timeoutSeconds?: number;
  debug?: boolean;
  lightweightMode?: boolean;
}

export interface WorkflowResultVariables {
  [agentId: string]: string;
}
```

### Async Patterns

All I/O operations use async/await:

```typescript
// Good - async/await
const result = await executor.execute(workflow);

// Avoid - callbacks
executor.execute(workflow, (error, result) => {
  // Don't use this pattern
});
```

## Common Development Tasks

### Adding a New API Method

1. **Add Rust implementation** in `src/lib.rs`:

```rust
#[napi]
impl Executor {
    #[napi]
    pub async fn new_method(&self, param: String) -> napi::Result<String> {
        Ok(format!("Processed: {}", param))
    }
}
```

2. **Rebuild** the module:

```bash
npm run build
```

3. **Add tests** in `tests/`:

```typescript
it('should call new method', async () => {
  const result = await executor.newMethod('test');
  expect(result).toBe('Processed: test');
});
```

4. **Update documentation** in `docs/`:

```markdown
### newMethod(param)

Processes the provided parameter.

**Parameters:**
- `param` (string): The parameter to process

**Returns:** Promise<string>
```

### Adding Examples

Create comprehensive examples in `examples/`:

```typescript
// examples/new-feature-example.ts
import { init, LlmConfig, Executor } from '@infinitibit_gmbh/graphbit';

async function main() {
  init();

  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || ''
  });

  const executor = new Executor(config);

  // Demonstrate new feature
  console.log('Example output:', await executor.newFeature());
}

main().catch(console.error);
```

### Debugging

```typescript
// Enable debug mode
init({ debug: true, logLevel: 'debug' });

// Use Node.js debugger
// In VS Code launch.json:
{
  "type": "node",
  "request": "launch",
  "name": "Debug Tests",
  "runtimeExecutable": "npm",
  "runtimeArgs": ["test"],
  "console": "integratedTerminal"
}
```

## Performance Considerations

### Memory Management

```typescript
// Good - let objects be garbage collected
async function processMany() {
  for (const item of items) {
    const result = await process(item);
    // Result goes out of scope after each iteration
  }
}

// Avoid - accumulating large objects
async function processMany() {
  const results = [];
  for (const item of items) {
    results.push(await process(item)); // Can accumulate memory
  }
  return results;
}
```

### Async Best Practices

```typescript
// Good - parallel execution
const results = await Promise.all(
  workflows.map(w => executor.execute(w))
);

// Avoid - sequential when parallel is possible
const results = [];
for (const workflow of workflows) {
  results.push(await executor.execute(workflow));
}
```

## Security Guidelines

### API Key Handling

```typescript
// Good - environment variables
const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || ''
});

// Never - hardcoded keys
const config = LlmConfig.openai({
  apiKey: 'sk-...'  // Never commit this!
});
```

### Input Validation

```typescript
function validateInput(input: string): void {
  if (typeof input !== 'string') {
    throw new TypeError('Input must be a string');
  }
  
  if (input.length === 0) {
    throw new Error('Input cannot be empty');
  }
  
  if (input.length > 100000) {
    throw new Error('Input too large');
  }
}
```

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: JavaScript Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16.x, 18.x, 20.x]
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: ${{ matrix.node-version }}
      
      - run: npm ci
      - run: npm run build
      - run: npm test
      - run: npm run lint
```

## Release Process

1. **Update version** in `package.json`
2. **Update CHANGELOG.md**
3. **Build release**:

```bash
npm run build:release
```

4. **Test release build**:

```bash
npm pack
npm install ./infinitibit_gmbh-graphbit-*.tgz
```

5. **Publish** (maintainers only):

```bash
npm publish --access public
```

## Common Issues and Solutions

### Build Failures

```bash
# Clean and rebuild
npm run clean
npm run rebuild

# Ensure Rust toolchain is updated
rustup update

# Check napi-rs CLI version
npx @napi-rs/cli -v
```

### Type Definition Mismatches

```bash
# Regenerate type definitions
npm run build
```

### Test Failures

```bash
# Run tests with verbose output
npm test -- --reporter=verbose

# Run single test file
npm test -- tests/specific-test.ts
```

## Resources

- [napi-rs Documentation](https://napi.rs/)
- [Node.js API Documentation](https://nodejs.org/api/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Vitest Documentation](https://vitest.dev/)
- [GraphBit JavaScript API Reference](../api-reference/javascript-api.md)

## Getting Help

- **Documentation**: Check [JavaScript docs](../getting-started/quickstart-js.md)
- **GitHub Issues**: Use `javascript` label
- **Discussions**: Ask in GitHub discussions
- **Examples**: Study `examples/` directory

## Recognition

JavaScript contributors are recognized in:
- CHANGELOG.md
- Package contributors list
- GitHub contributors page

## Next Steps

Ready to contribute? Here are some good first steps:

1. **Setup**: Complete development environment setup
2. **Explore**: Study existing tests and examples
3. **Build**: Try building the project
4. **Test**: Run the test suite
5. **Contribute**: Start with [good first issue](https://github.com/InfinitiBit/graphbit/labels/good%20first%20issue)

Thank you for contributing to GraphBit's JavaScript ecosystem! 🚀

---

For core development or Python contributions, see:
- [Main Contributing Guide](contributing.md)
- [JavaScript Bindings Architecture](javascript-bindings.md)
- [Development Guide](architecture-js.md)
