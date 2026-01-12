# JavaScript Scripts Directory

This directory contains all testing and verification scripts for GraphBit JavaScript bindings.

## 📁 Structure

```
scripts/
├── verification/          # API verification scripts (13 files)
├── live-tests/           # Live API integration tests (2 files)
└── verify_methods.js     # Method verification utility
```

---

## 🧪 Verification Scripts (`verification/`)

These scripts verify that the JavaScript API documentation is accurate by testing each feature.

### Core API Verification (9 scripts)
```bash
node verification/docs_verify_core.js
node verification/docs_verify_llm_config.js
node verification/docs_verify_workflow.js
node verification/docs_verify_executor.js
node verification/docs_verify_text_splitter.js
node verification/docs_verify_document_loader.js
node verification/docs_verify_embeddings.js
node verification/docs_verify_agent.js
node verification/docs_verify_tools.js
```

### Example Tests (4 scripts)
```bash
node verification/test_rag_example.js
node verification/test_multiagent_example.js
node verification/test_errorhandling_example.js
node verification/test_production_example.js
```

**Status:** ✅ All 13 scripts passing (100%)

---

## 🔴 Live API Tests (`live-tests/`)

These scripts test end-to-end functionality with real API calls.

**⚠️ Requires API Key:** Set `OPENAI_API_KEY` environment variable

```bash
# RAG pipeline with real embeddings and LLM
node live-tests/test_rag_live.js

# Multi-agent collaboration with real LLM
node live-tests/test_multiagent_live.js
```

**Status:** ✅ Both tests passing (100%)

---

## 🔧 Utility Scripts

### verify_methods.js
Verifies that exported methods match expected API surface.

```bash
node verify_methods.js
```

---

## 📊 Test Coverage

| Category | Scripts | Pass Rate |
|----------|---------|-----------|
| API Verification | 9 | 100% ✅ |
| Example Tests | 4 | 100% ✅ |
| Live Integration | 2 | 100% ✅ |
| Utilities | 1 | ✅ |

**Total:** 16 scripts, 100% success rate

---

## 🚀 Running All Tests

### Run All Verification Scripts
```bash
# From javascript/ directory
for file in scripts/verification/*.js; do
  echo "Running $file"
  node "$file"
done
```

### Run All Tests (Including Live)
```bash
# Requires OPENAI_API_KEY
export OPENAI_API_KEY=your_key_here

# Run all verification
for file in scripts/verification/*.js; do node "$file"; done

# Run all live tests
for file in scripts/live-tests/*.js; do node "$file"; done
```

---

## 📝 Adding New Tests

1. Create test file in appropriate directory:
   - `verification/` for API tests
   - `live-tests/` for integration tests

2. Follow naming convention:
   - `docs_verify_*.js` for API verification
   - `test_*_example.js` for example tests
   - `test_*_live.js` for live tests

3. Run test to verify it passes

4. Update this README if needed

---

**Last Updated:** 2025-12-09  
**Total Scripts:** 16  
**Status:** ✅ All Passing
