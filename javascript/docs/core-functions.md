# Core Functions

This document covers the core initialization and utility functions available in the GraphBit JavaScript bindings.

## Overview

The core functions provide essential initialization and version information for the GraphBit library. These are typically the first functions you'll use when integrating GraphBit into your application.

## Functions

### `init()`

Initialize the GraphBit library.

**Signature:**

```typescript
function init(): void
```

**Description:**
This function should be called once at the start of your application. It sets up logging and other global state required by the library.

**Parameters:** None

**Returns:** `void`

**Throws:** May throw an error if initialization fails (rare)

### 🟢 Verified Example

```javascript
const { init } = require('@infinitibit_gmbh/graphbit');

// Initialize the library
init();
console.log('GraphBit initialized successfully');
```

**Differences from Python:**

- Python accepts optional parameters: `log_level`, `enable_tracing`, `debug`
- JavaScript version currently has no configuration options
- Both versions are idempotent (safe to call multiple times)

---

### `version()`

Get the current version of the GraphBit library.

**Signature:**

```typescript
function version(): string
```

**Description:**
Returns the semantic version string of the GraphBit core library.

**Parameters:** None

**Returns:** `{ version: string; rustVersion: string; napiVersion: string }`

---

### `getSystemInfo()`

Get detailed system information.

**Signature:**

```typescript
function getSystemInfo(): SystemInfo
```

**Returns:** `SystemInfo` object

---

### `healthCheck()`

Perform a library health check.

**Signature:**

```typescript
function healthCheck(): HealthStatus
```

**Returns:** `HealthStatus` object

---

### `configureRuntime(config)`

Configure runtime settings.

**Signature:**

```typescript
function configureRuntime(config: RuntimeConfig): void
```

### 🟢 Verified Example

```javascript
const { version } = require('@infinitibit_gmbh/graphbit');

const ver = version();
console.log(`GraphBit version: ${ver}`); // GraphBit version: 0.5.1
```

**Differences from Python:**

- Identical behavior in both languages
- Both return a string representation of the version

---

## Functions NOT Available in JavaScript

The following Python functions are **not currently available** in the JavaScript bindings:

### `get_system_info()` ❌

Python only. Returns comprehensive system information including runtime stats, CPU count, memory allocator, etc.

### `health_check()` ❌

Python only. Performs comprehensive health checks on the runtime, memory, and worker threads.

### `configure_runtime()` ❌

Python only. Allows configuration of worker threads, blocking threads, and stack sizes.

### `shutdown()` ❌

Python only. Gracefully shuts down the library (primarily for testing).

---

## Usage Patterns

### Basic Initialization

```javascript
const { init, version } = require('@infinitibit_gmbh/graphbit');

// Always initialize first
init();

// Log version for debugging
console.log(`Running GraphBit v${version()}`);

// Continue with your application logic...
```

### Error Handling

```javascript
const { init } = require('@infinitibit_gmbh/graphbit');

try {
  init();
  console.log('Initialized successfully');
} catch (error) {
  console.error('Failed to initialize GraphBit:', error);
  process.exit(1);
}
```

---

## Best Practices

1. **Call `init()` early**: Initialize GraphBit as early as possible in your application lifecycle
2. **Single initialization**: While idempotent, there's no need to call `init()` multiple times
3. **Version logging**: Log the version in production for easier debugging
4. **No cleanup needed**: Unlike Python, JavaScript bindings don't expose a `shutdown()` function - cleanup is automatic

---

## Related Documentation

- [LLM Configuration](./llm-config.md) - Configure language model providers
- [Workflow Management](./workflow.md) - Create and execute workflows
