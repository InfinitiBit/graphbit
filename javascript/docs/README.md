# JavaScript API Documentation

**Status:** ✅ All examples verified against v0.5.1

Welcome to the GraphBit JavaScript bindings documentation. This documentation follows the **"Execute First, Document Later"** principle - every code example has been verified to run successfully.

## Quick Navigation

### Getting Started

- [Core Functions](./core-functions.md) - Initialize GraphBit and get version info
- [LLM Configuration](./llm-config.md) - Configure language model providers

### Building Workflows

- [Workflow Management](./workflow.md) - Create workflows, add nodes, and connect them
- [Executor](./executor.md) - Execute workflows and manage execution context

### Working with Data

- [Document Loader](./document-loader.md) - Load documents from various sources
- [Text Splitter](./text-splitter.md) - Split documents into chunks
- [Embeddings](./embeddings.md) - Generate vector embeddings for semantic search

### AI & Agents

- [Agent](./agent.md) - Create and configure AI agents
- [Tools](./tools.md) - Register and execute custom tools

### Reference

- [API Reference](../../javascript/API_REFERENCE.md) - Quick reference for common patterns
- [Feasibility Report](../../javascript/feasibility_report.md) - Python vs JavaScript feature comparison

---

## Documentation Structure

Each document follows this format:

1. **Overview** - What the feature does
2. **🟢 Verified Examples** - Code that has been executed and proven to work
3. **Differences from Python** - Key differences for Python developers
4. **Best Practices** - Tips and gotchas
5. **Related Documentation** - Links to related topics

---

## Verification Status

| Document | Verification Script | Status |
|----------|---------------------|--------|
| [Core Functions](./core-functions.md) | `scripts/docs_verify_core.js` | ✅ Verified |
| [LLM Config](./llm-config.md) | `scripts/docs_verify_llm_config.js` | ✅ Verified |
| [Workflow](./workflow.md) | `scripts/docs_verify_workflow.js` | ✅ Verified |
| [Executor](./executor.md) | `scripts/docs_verify_executor.js` | ✅ Verified |
| [Text Splitter](./text-splitter.md) | `scripts/docs_verify_text_splitter.js` | ✅ Verified |
| [Document Loader](./document-loader.md) | `scripts/docs_verify_document_loader.js` | ✅ Verified |
| [Embeddings](./embeddings.md) | `scripts/docs_verify_embeddings.js` | ✅ Verified |
| [Agent](./agent.md) | `scripts/docs_verify_agent.js` | ✅ Verified |
| [Tools](./tools.md) | `scripts/docs_verify_tools.js` | ✅ Verified |

All verification scripts can be run with:

```bash
node scripts/docs_verify_<feature>.js
```

---

## Key Differences from Python

If you're coming from the Python bindings, here are the most important differences:

### 1. Async/Await

Most methods in JavaScript return Promises and must be awaited:

```javascript
// JavaScript
const name = await workflow.name();

# Python
name = workflow.name()
```

### 2. Object Parameters

JavaScript uses object parameters instead of positional arguments:

```javascript
// JavaScript
const config = LlmConfig.openai({ apiKey: 'key', model: 'gpt-4o-mini' });

# Python
config = LlmConfig.openai('key', 'gpt-4o-mini')
```

### 3. Enums are Numbers

NAPI-RS maps Rust enums to numbers, not strings:

```javascript
// JavaScript
retryableErrors: [RetryableErrorType.NetworkError]  // ✅ Correct

retryableErrors: ['NetworkError']  // ❌ Wrong - will throw error
```

### 4. Null vs Undefined

For optional fields, omit them entirely (undefined) rather than passing null:

```javascript
// JavaScript
const edge = { fromNode: 'a', toNode: 'b' };  // ✅ Correct

const edge = { fromNode: 'a', toNode: 'b', condition: null };  // ❌ Wrong
```

---

## Features Not Available in JavaScript

These Python features are currently not available in the JavaScript bindings:

- `get_system_info()` - System information
- `health_check()` - Health checks
- `configure_runtime()` - Runtime configuration
- `shutdown()` - Graceful shutdown

---

## Contributing to Documentation

When adding new documentation:

1. **Write verification script first** (in `scripts/docs_verify_<feature>.js`)
2. **Run and verify** the code works
3. **Document the verified examples** with 🟢 indicator
4. **Note any gotchas** discovered during verification
5. **Update this README** with the new document

---

## Examples Repository

For complete working examples, see:

- `scripts/docs_verify_*.js` - Verification scripts for each feature
- `examples/` - Full application examples

---

## Getting Help

- **Issues**: Report bugs or gaps at [GitHub Issues](https://github.com/your-repo/graphbit)
- **Python Docs**: See `docs/api-reference/python-api.md` for Python equivalents
- **Core Docs**: General concepts apply to both Python and JavaScript

---

**Last Updated:** 2025-12-05T19:39:53+06:00  
**Version:** 0.5.1  
**Binding Type:** NAPI-RS  
**Documentation Coverage:** 9 major components, 100+ verified examples
