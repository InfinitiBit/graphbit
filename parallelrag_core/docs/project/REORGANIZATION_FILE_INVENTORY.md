# Repository Reorganization - File Inventory

**Date**: 2025-11-18  
**Objective**: Move all non-Ollama files into `parallelrag_core/` directory structure

---

## Summary Statistics

### Files to Move
- **Root Python Files**: 8 files
- **Root Data Files**: 24 files (JSON, PNG charts)
- **Root Documentation**: 68 markdown files
- **Examples Directory**: ~30 files (excluding Ollama)
- **Benchmarks Directory**: ~15 files
- **Tests Directory**: ~80 Python/Rust test files (excluding Ollama)
- **Docs Directory**: ~50 documentation files
- **Workshop Guides**: ~10 files
- **Sample Docs**: 5 files
- **Test Results**: 2 files (non-Ollama)

**Total Estimated Files**: ~290 files

---

## Files to Move by Category

### 1. Root Python Application Files (8 files)
```
parallel_rag_app.py                    → parallelrag_core/parallel_rag_app.py
langchain_rag_app.py                   → parallelrag_core/langchain_rag_app.py
validate_fixes.py                      → parallelrag_core/scripts/validate_fixes.py
validate_rag_equivalence.py            → parallelrag_core/scripts/validate_rag_equivalence.py
test_worker_optimization.py            → parallelrag_core/benchmarks/test_worker_optimization.py
create_visualizations.py               → parallelrag_core/visualizations/create_visualizations.py
create_resource_charts.py              → parallelrag_core/visualizations/create_resource_charts.py
create_additional_visualizations.py    → parallelrag_core/visualizations/create_additional_visualizations.py
```

### 2. Root Data Files (24 files)
```
# Benchmark result JSON files
framework_comparison_results.json      → parallelrag_core/data/benchmark_results/
graphbit_max_capacity_*.json           → parallelrag_core/data/benchmark_results/
graphbit_stress_*.json                 → parallelrag_core/data/benchmark_results/
graphbit_variable_size_*.json          → parallelrag_core/data/benchmark_results/
langchain_stress_*.json                → parallelrag_core/data/benchmark_results/
worker_optimization_results.json       → parallelrag_core/data/benchmark_results/

# Chart PNG files
chart_*.png                            → parallelrag_core/data/charts/
```

### 3. Root Documentation Files (68 files)
```
# Performance & Benchmark Documentation
BENCHMARK_RESULTS.md                   → parallelrag_core/docs/benchmarks/
BENCHMARK_SUITE_COMPLETE.md            → parallelrag_core/docs/benchmarks/
COMPREHENSIVE_BENCHMARKING_SUMMARY.md  → parallelrag_core/docs/benchmarks/
COMPREHENSIVE_PERFORMANCE_ANALYSIS.md  → parallelrag_core/docs/benchmarks/
FRAMEWORK_COMPARISON.md                → parallelrag_core/docs/benchmarks/
FRAMEWORK_PERFORMANCE_COMPARISON_RESULTS.md → parallelrag_core/docs/benchmarks/
MAXIMUM_CAPACITY_COMPARISON.md         → parallelrag_core/docs/benchmarks/
PERFORMANCE_VALIDATION_REPORT.md       → parallelrag_core/docs/benchmarks/
STRESS_TEST_RESULTS.md                 → parallelrag_core/docs/benchmarks/

# GIL & Implementation Documentation
CRITICAL_GIL_FIXES_SUMMARY.md          → parallelrag_core/docs/implementation/
GIL_STATUS_BEFORE_AFTER_COMPARISON.md  → parallelrag_core/docs/implementation/
GIL_STATUS_MATRIX.md                   → parallelrag_core/docs/implementation/
PARALLELRAG_GIL_STATUS_AND_ACTION_PLAN.md → parallelrag_core/docs/implementation/
QUICK_REFERENCE_GIL_STATUS.md          → parallelrag_core/docs/implementation/

# RAG Implementation Documentation
GRAPHBIT_RAG_SPECIFICATION.md          → parallelrag_core/docs/rag/
GRAPHBIT_VS_LANGCHAIN_RAG_COMPARISON.md → parallelrag_core/docs/rag/
RAG_IMPLEMENTATION_GAP_ANALYSIS.md     → parallelrag_core/docs/rag/
RAG_IMPLEMENTATION_SUMMARY.md          → parallelrag_core/docs/rag/

# ParallelRAG Application Documentation
PARALLEL_RAG_APP_DOCUMENTATION.md      → parallelrag_core/docs/applications/
PARALLEL_RAG_APP_SUMMARY.md            → parallelrag_core/docs/applications/
PARALLEL_RAG_FIXES_SUMMARY.md          → parallelrag_core/docs/applications/

# Production & Deployment Documentation
PRODUCTION_DEPLOYMENT_GUIDE.md         → parallelrag_core/docs/production/
PRODUCTION_ERROR_HANDLING.md           → parallelrag_core/docs/production/
PRODUCTION_PERFORMANCE_MONITORING.md   → parallelrag_core/docs/production/
PRODUCTION_READINESS_CHECKLIST.md      → parallelrag_core/docs/production/
PRODUCTION_RUNTIME_CONFIGURATION.md    → parallelrag_core/docs/production/

# Project Management Documentation
DELIVERABLES_SUMMARY.md                → parallelrag_core/docs/project/
EXECUTION_ROADMAP.md                   → parallelrag_core/docs/project/
REVISED_EXECUTION_ROADMAP.md           → parallelrag_core/docs/project/
TASK_DEPENDENCY_ANALYSIS.md            → parallelrag_core/docs/project/

# Phase Documentation (P1, P2, P3)
P1_*.md                                → parallelrag_core/docs/phases/
P2_*.md                                → parallelrag_core/docs/phases/
P3_*.md                                → parallelrag_core/docs/phases/
PARALLELRAG_*.md                       → parallelrag_core/docs/phases/

# Testing Documentation
TESTING_AND_VALIDATION_SUMMARY.md     → parallelrag_core/docs/testing/
TEST_EXECUTION_REPORT.md               → parallelrag_core/docs/testing/
SYNTHESIS_AND_VALIDATION.md            → parallelrag_core/docs/testing/

# Marketing & Presentation Documentation
EXECUTIVE_PRESENTATION.md              → parallelrag_core/docs/marketing/
EXECUTIVE_SUMMARY_GIL_WORK.md          → parallelrag_core/docs/marketing/
EXECUTIVE_SUMMARY_INFOGRAPHIC.md       → parallelrag_core/docs/marketing/
GRAPHBIT_PERFORMANCE_WHITEPAPER.md     → parallelrag_core/docs/marketing/
MARKETING_MATERIALS_SUMMARY.md         → parallelrag_core/docs/marketing/
WORKSHOP_DEMO_GUIDE.md                 → parallelrag_core/docs/marketing/
WORKSHOP_MARKETING_SCRIPT.md           → parallelrag_core/docs/marketing/

# Analysis Documentation
BREAKING_CHANGE_ASSESSMENT.md          → parallelrag_core/docs/analysis/
COMPARATIVE_DIFFERENTIATION_ANALYSIS.md → parallelrag_core/docs/analysis/
ISSUE_287_DEPENDENCY_ANALYSIS.md       → parallelrag_core/docs/analysis/
PYTHON_API_ANALYSIS.md                 → parallelrag_core/docs/analysis/
```

### 4. Examples Directory
```
parallelrag_core/examples/parallel_rag_optimized.py     → parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py
examples/benchmark_gil_fixes.py        → parallelrag_core/examples/benchmark_gil_fixes.py
examples/README.md                     → parallelrag_core/examples/README.md
examples/tasks_examples/               → parallelrag_core/examples/tasks_examples/
examples/browser-automation-agent/     → parallelrag_core/examples/browser-automation-agent/
examples/chatbot/                      → parallelrag_core/examples/chatbot/
examples/research-paper-summarizer-agent/ → parallelrag_core/examples/research-paper-summarizer-agent/
```

### 5. Benchmarks Directory
```
benchmarks/                            → parallelrag_core/benchmarks/
```

### 6. Tests Directory
```
tests/benchmarks/                      → parallelrag_core/tests/benchmarks/
tests/python_integration_tests/        → parallelrag_core/tests/python_integration_tests/
tests/python_unit_tests/               → parallelrag_core/tests/python_unit_tests/
parallelrag_core/tests/test_parallel_rag_app.py         → parallelrag_core/parallelrag_core/tests/test_parallel_rag_app.py
parallelrag_core/tests/test_langchain_rag_app.py        → parallelrag_core/parallelrag_core/tests/test_langchain_rag_app.py
```

---

## Files to EXCLUDE (Keep in Root)

### Configuration Files
- `pyproject.toml`
- `Cargo.toml`
- `pytest.ini`
- `clippy.toml`
- `rustfmt.toml`
- `tarpaulin.toml`

### Main Documentation
- `README.md` (and all language variants)
- `LICENSE.md`
- `SECURITY.md`
- `CONTRIBUTING.md`
- `CHANGELOG.md`

### Rust Source Code
- `src/` directory
- `core/` directory
- `python/` directory (Python bindings source)
- `target/` directory (build artifacts)

### Build & Deployment Scripts
- `scripts/` directory

### Ollama Integration
- `ollama_integration/` directory (already organized)

### Rust Tests
- `tests/main.rs`
- `tests/rust_integration_tests/`
- `tests/rust_unit_tests/`

---

## Next Steps

1. ✅ Complete file inventory
2. ⏳ Map import dependencies
3. ⏳ Map documentation references
4. ⏳ Define target directory structure
5. ⏳ Create directory structure
6. ⏳ Move files
7. ⏳ Update imports
8. ⏳ Update documentation
9. ⏳ Validate and test

