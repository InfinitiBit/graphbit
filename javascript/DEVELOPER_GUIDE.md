# GraphBit JavaScript Bindings - Developer Guide

**For Contributors & Developers**  
**Last Updated:** 2025-12-09

This guide helps developers build, test, and contribute to GraphBit JavaScript bindings.

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Building from Source](#building-from-source)
3. [Running Examples](#running-examples)
4. [Running Tests](#running-tests)
5. [Development Workflow](#development-workflow)
6. [Project Structure](#project-structure)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software
- **Node.js:** v16 or higher ([Download](https://nodejs.org/))
- **npm:** v7 or higher (comes with Node.js)
- **Rust:** Latest stable ([Install](https://rustup.rs/))
- **Cargo:** Comes with Rust

### Verify Installation
```bash
node --version    # Should be >= v16.0.0
npm --version     # Should be >= 7.0.0
rustc --version   # Should be >= 1.70.0
cargo --version   # Should be >= 1.70.0
```

### Optional (for live tests)
- **OpenAI API Key** - For running live integration tests

---

## Building from Source

### 1. Clone the Repository
```bash
git clone https://github.com/your-org/graphbit.git
cd graphbit
```

### 2. Install Dependencies
```bash
# Install Node.js dependencies
npm install

# This also builds the Rust bindings automatically via napi-rs
```

### 3. Build Manually (if needed)
```bash
# Build the Rust NAPI bindings
npm run build

# Or use cargo directly
cargo build --release
```

### 4. Verify the Build
```bash
# Check that the binding was created
ls graphbit-js.*.node

# Test the binding
node -e "const gb = require('./javascript/index.js'); console.log(gb.version())"
```

Expected output: GraphBit version number (e.g., `0.5.1`)

---

## Running Examples

### Quick Start Example

Create a file `test-graphbit.js`:

```javascript
const { init, version, LlmConfig, AgentBuilder } = require('./javascript/index.js');

async function main() {
  // Initialize GraphBit
  init();
  console.log('GraphBit version:', version());

  // Create LLM config (using Ollama for local testing)
  const config = LlmConfig.ollama({
    model: 'llama3.2'
  });

  // Create an agent
  const agent = await new AgentBuilder('TestAgent', config)
    .systemPrompt('You are a helpful assistant.')
    .temperature(0.7)
    .build();

  console.log('Agent created successfully!');
  
  // Optional: Execute if Ollama is running
  // const response = await agent.execute('Hello!');
  // console.log('Agent response:', response);
}

main().catch(console.error);
```

Run it:
```bash
node test-graphbit.js
```

### Running Documentation Examples

All examples in `javascript/docs/examples/` are runnable:

```bash
# Copy example code from docs/examples/rag-pipeline.md
# Save to a file, e.g., rag-example.js
# Run it
node rag-example.js
```

**Note:** Most examples require API keys. Set them in environment:
```bash
export OPENAI_API_KEY=your_key_here
node rag-example.js
```

---

## Running Tests

### 1. Verification Scripts (No API Key Required)

These test the API without making external calls:

```bash
# Run all verification scripts
cd javascript
for file in scripts/verification/*.js; do
  echo "Running $file"
  node "$file"
done
```

Or run individual tests:
```bash
node javascript/scripts/verification/docs_verify_core.js
node javascript/scripts/verification/docs_verify_workflow.js
node javascript/scripts/verification/test_rag_example.js
```

**Expected output:** All tests should pass with ✅ marks

### 2. Live API Tests (Requires API Key)

Set your API key first:
```bash
export OPENAI_API_KEY=sk-proj-...
```

Then run live tests:
```bash
node javascript/scripts/live-tests/test_rag_live.js
node javascript/scripts/live-tests/test_multiagent_live.js
```

**Expected:** Tests will make real API calls and verify end-to-end functionality

### 3. Method Verification

Verify that all expected methods are present:
```bash
node javascript/scripts/verify_methods.js
```

---

## Development Workflow

### Making Changes to Rust Code

1. **Edit Rust files** in `javascript/src/`
   ```bash
   vim javascript/src/agent.rs
   ```

2. **Rebuild the bindings**
   ```bash
   npm run build
   # Or
   cargo build --release
   ```

3. **Test your changes**
   ```bash
   node javascript/scripts/verification/docs_verify_agent.js
   ```

### Adding New Features

1. **Implement in Rust** (`javascript/src/*.rs`)
2. **Export via NAPI** (add to appropriate module)
3. **Write verification script** (`javascript/scripts/verification/`)
4. **Run verification** to ensure it works
5. **Write documentation** (`javascript/docs/`)
6. **Update README** if needed

### Testing Against Local Build

When testing, make sure to use the local build:

```javascript
// Use relative path to your local build
const graphbit = require('./javascript/index.js');
```

**Don't use:**
```javascript
// This uses the installed package, not your local build
const graphbit = require('graphbit');
```

---

## Project Structure

```
graphbit/
├── javascript/                      # JS bindings directory
│   ├── src/                         # Rust source (NAPI-RS)
│   │   ├── lib.rs                   # Main entry point
│   │   ├── agent.rs                 # Agent bindings
│   │   ├── workflow.rs              # Workflow bindings
│   │   └── ...
│   ├── index.js                     # JS entry point
│   ├── index.d.ts                   # TypeScript definitions
│   ├── package.json                 # NPM package config
│   ├── Cargo.toml                   # Rust package config
│   ├── docs/                        # Documentation
│   ├── scripts/                     # Test scripts
│   └── reports/                     # Verification reports
│
├── Cargo.toml                       # Root Cargo config
└── package.json                     # Root package config
```

### Key Files

| File | Purpose |
|------|---------|
| `javascript/index.js` | Main JS entry point, exports all APIs |
| `javascript/src/lib.rs` | Rust library root, defines NAPI module |
| `javascript/Cargo.toml` | Rust dependencies and build config |
| `graphbit-js.*.node` | Compiled native addon (gitignored) |

---

## Troubleshooting

### Build Failures

**Issue:** `cargo build` fails with compiler errors

**Solution:**
```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
npm run build
```

---

**Issue:** `npm install` fails with NAPI errors

**Solution:**
```bash
# Ensure you have the required build tools
# On Ubuntu/Debian:
sudo apt-get install build-essential

# On macOS:
xcode-select --install

# On Windows:
# Install Visual Studio Build Tools
```

---

### Runtime Errors

**Issue:** `Error: Cannot find module './graphbit-js.*.node'`

**Solution:**
```bash
# The native addon wasn't built
npm run build

# Or rebuild
npm rebuild
```

---

**Issue:** `TypeError: LlmConfig.openai is not a function`

**Solution:**
- Make sure you're using the correct import path
- Verify the build was successful
- Check you're running from the correct directory

---

### Test Failures

**Issue:** Verification scripts fail

**Solution:**
```bash
# Rebuild bindings
npm run build

# Re-run the specific test
node javascript/scripts/verification/docs_verify_core.js
```

---

**Issue:** Live tests fail with API errors

**Solution:**
- Check your API key is set: `echo $OPENAI_API_KEY`
- Verify the API key is valid
- Check your network connection
- API rate limits may be hit - wait and retry

---

## Quick Commands Reference

```bash
# Build bindings
npm run build

# Run all verification tests
for f in javascript/scripts/verification/*.js; do node "$f"; done

# Run live tests (requires API key)
node javascript/scripts/live-tests/test_rag_live.js

# Check build status
ls -lh graphbit-js.*.node

# Test basic functionality
node -e "const g = require('./javascript/index.js'); console.log(g.version())"

# Clean rebuild
cargo clean && npm run build
```

---

## Environment Variables

```bash
# For live tests
export OPENAI_API_KEY=sk-proj-...

# For development debugging
export RUST_BACKTRACE=1
export NODE_OPTIONS=--trace-warnings
```

---

## Next Steps

1. **Read the docs:** Start with [javascript/docs/README.md](./docs/README.md)
2. **Try examples:** Run examples from [javascript/docs/examples/](./docs/examples/)
3. **Run tests:** Verify everything works with test scripts
4. **Contribute:** See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for guidelines

---

## Getting Help

- **Documentation:** See [javascript/docs/](./docs/)
- **Examples:** Check [javascript/docs/examples/](./docs/examples/)
- **Issues:** Report bugs on GitHub
- **API Questions:** See [javascript/API_REFERENCE.md](./API_REFERENCE.md)

---

**Last Updated:** 2025-12-09  
**GraphBit Version:** 0.5.1  
**Status:** ✅ Complete
