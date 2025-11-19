# JavaScript Bindings Compilation Errors Summary

## Overview

The JavaScript bindings have 90 compilation errors due to mismatches between the bindings code and the actual GraphBit core API.

## Critical Issues Found

### 1. LlmConfig API Mismatch

**Problem**: Bindings assume builder pattern methods like `with_temperature()`, `with_max_tokens()`, `with_base_url()`
**Reality**: LlmConfig is an enum with struct variants, no builder methods
**Solution**: Use direct struct construction or factory methods

**Actual API**:

- `LlmConfig::openai(api_key, model)` - takes 2 args
- `LlmConfig::anthropic(api_key, model)` - takes 2 args
- `LlmConfig::ollama(model)` - takes 1 arg (no base_url parameter)
- `LlmConfig::azure_openai(api_key, deployment_name, endpoint, api_version)` - takes 4 args

### 2. WorkflowBuilder/AgentBuilder Clone Issue

**Problem**: Bindings call `.clone()` on builders
**Reality**: Builders don't implement Clone (they consume self)
**Solution**: Remove clone() calls, use consuming pattern

### 3. Error Type Names

**Problem**: Bindings use `ConfigurationError`, `ValidationError`, etc.
**Reality**: Core uses `Configuration`, `Validation`, etc. (without "Error" suffix)
**Solution**: Update error variant names

### 4. WorkflowState Enum Variants

**Problem**: Bindings treat `Running` and `Failed` as unit variants
**Reality**: They are struct variants with fields

- `Running { current_node: NodeId }`
- `Failed { error: String }`
  **Solution**: Match with field destructuring

### 5. AgentCapability Enum

**Problem**: Bindings use `TextGeneration`, `ToolCalling`, `FunctionCalling`, `Vision`, `CodeGeneration`, `Reasoning`
**Reality**: Need to check actual enum variants in core
**Solution**: Update to match core enum

### 6. WorkflowExecutionStats Fields

**Problem**: Bindings expect `total_duration`, `nodes_executed`, `nodes_failed`, `nodes_skipped`, `total_tokens`, `total_cost_usd`
**Reality**: Core has different fields: `total_nodes`, `successful_nodes`, `failed_nodes`, `avg_execution_time_ms`, `max_concurrent_nodes`
**Solution**: Update field mappings

### 7. Type Compatibility Issues

**Problem**: napi-rs doesn't support `f32`, `u64`, `usize` directly
**Reality**: Need to use `f64`, `i64`, `i32` for JavaScript compatibility
**Solution**: Convert types in bindings

### 8. Optional vs Required Fields

**Problem**: Bindings use `.map()` on non-Option types
**Reality**: Some fields are not Option<T>

- `finish_reason` is not Option
- `usage` is not Option
- `tool_calls` is Vec, not Option<Vec>
  **Solution**: Remove incorrect `.map()` calls

### 9. EmbeddingConfig API

**Problem**: Bindings assume `EmbeddingConfig::openai()` and `EmbeddingConfig::huggingface()` exist
**Reality**: Need to check actual embedding API
**Solution**: Update to match core API

### 10. DocumentLoaderConfig Fields

**Problem**: Bindings use `extract_images`, `extract_tables`
**Reality**: Core has `max_file_size`, `default_encoding`, `preserve_formatting`, `extraction_settings`
**Solution**: Update config structure

### 11. TextSplitter Strategy

**Problem**: Bindings use incorrect field names and structure
**Reality**: Need to match actual SplitterStrategy enum
**Solution**: Update strategy construction

### 12. Embedding Response Type

**Problem**: Bindings expect `Vec<Vec<f64>>`
**Reality**: Core returns `Vec<Vec<f32>>`
**Solution**: Convert f32 to f64 or update type

### 13. Validation API

**Problem**: Bindings call `result.is_valid()` and `result.errors()` as methods
**Reality**: These are fields, not methods
**Solution**: Access as fields: `result.is_valid`, `result.errors`

### 14. HashMap Methods

**Problem**: Bindings call `.and_then()` on HashMap
**Reality**: HashMap doesn't have `.and_then()` method
**Solution**: Use different approach for optional metadata

### 15. EmbeddingService Constructor

**Problem**: Bindings assume `EmbeddingService::new()` returns `EmbeddingService`
**Reality**: Returns `Result<EmbeddingService, GraphBitError>`
**Solution**: Handle Result with `?` or `.map_err()`

### 16. EmbeddingRequest Fields

**Problem**: Bindings include `model` field
**Reality**: Core has `input`, `user`, `params` fields
**Solution**: Update request structure

### 17. Embedding Method Name

**Problem**: Bindings call `service.embed()`
**Reality**: Method is `service.embed_text()`
**Solution**: Update method name

### 18. AgentBuilder Constructor

**Problem**: Bindings call `AgentBuilder::new(name)`
**Reality**: Requires `AgentBuilder::new(name, llm_config)` - 2 args
**Solution**: Update constructor call

### 19. DocumentLoader Constructor

**Problem**: Bindings call `DocumentLoader::new(config)`
**Reality**: Takes no arguments: `DocumentLoader::new()`
**Solution**: Remove config parameter

## Action Plan

1. **Phase 1**: Fix LlmConfig bindings (highest priority)
2. **Phase 2**: Fix error type mappings
3. **Phase 3**: Fix builder patterns (remove clone())
4. **Phase 4**: Fix type conversions (f32→f64, u64→i64, etc.)
5. **Phase 5**: Fix struct field mappings
6. **Phase 6**: Fix API method names and signatures
7. **Phase 7**: Test compilation

## Recommendation

Given the extensive API mismatches, the bindings need to be rewritten to match the actual GraphBit core API. This requires:

1. Examining each core module's actual API
2. Updating bindings to match reality
3. Updating TypeScript definitions to match
4. Updating tests to match new API
5. Updating documentation and examples

Estimated effort: 4-6 hours of focused work to fix all issues.
