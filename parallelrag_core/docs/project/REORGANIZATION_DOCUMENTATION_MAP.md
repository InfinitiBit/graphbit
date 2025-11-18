# Repository Reorganization - Documentation Reference Map

**Date**: 2025-11-18  
**Purpose**: Document all file path references in markdown files that need updating

---

## Workshop Guides Documentation

### `workshop_guides/NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md`

**File Path References** (15 occurrences):

1. Line 167: `python parallelrag_core/examples/parallel_rag_optimized.py`
   - **New**: `python parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

2. Line 194: `python parallelrag_core/parallel_rag_app.py`
   - **New**: `python parallelrag_core/parallel_rag_app.py`

3. Line 227: `parallelrag_core/examples/parallel_rag_optimized.py` (354 lines)
   - **New**: `parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

4. Line 231: `python parallelrag_core/examples/parallel_rag_optimized.py`
   - **New**: `python parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

5. Line 312: `tests/benchmarks/benchmark_framework_comparison.py` (733 lines)
   - **New**: `parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`

6. Line 313: `parallelrag_core/benchmarks/run_benchmark.py` (748 lines)
   - **New**: `parallelrag_core/parallelrag_core/benchmarks/run_benchmark.py`

7. Line 319: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`
   - **New**: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`

8. Line 380: `tests/benchmarks/benchmark_stress_test.py`
   - **New**: `parallelrag_core/tests/benchmarks/benchmark_stress_test.py`

9. Line 384: `python tests/benchmarks/benchmark_stress_test.py`
   - **New**: `python parallelrag_core/tests/benchmarks/benchmark_stress_test.py`

10. Line 535: `python parallelrag_core/examples/parallel_rag_optimized.py`
    - **New**: `python parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

11. Line 540: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`

12. Line 545: `python tests/benchmarks/benchmark_stress_test.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_stress_test.py`

13. Line 563: `parallelrag_core/examples/parallel_rag_optimized.py`
    - **New**: `parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

14. Line 569: `tests/benchmarks/`
    - **New**: `parallelrag_core/tests/benchmarks/`

15. Line 634: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`

---

### `workshop_guides/command_reference.md`

**File Path References** (18 occurrences):

1. Line 43: `python parallelrag_core/examples/parallel_rag_optimized.py`
   - **New**: `python parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`

2. Line 56: `python parallelrag_core/parallel_rag_app.py`
   - **New**: `python parallelrag_core/parallel_rag_app.py`

3. Line 73: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`
   - **New**: `python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py`

4. Line 91: `cd benchmarks` + `python run_benchmark.py`
   - **New**: `cd parallelrag_core/benchmarks` + `python run_benchmark.py`

5. Line 106: `python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks graphbit`
   - **New**: `cd parallelrag_core/benchmarks` + `python run_benchmark.py ...`

6. Line 109: `python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks langchain`
   - **New**: `cd parallelrag_core/benchmarks` + `python run_benchmark.py ...`

7. Line 112: `python run_benchmark.py --provider openai --model gpt-4o-mini --frameworks graphbit,langchain`
   - **New**: `cd parallelrag_core/benchmarks` + `python run_benchmark.py ...`

8. Line 124: `python tests/benchmarks/benchmark_stress_test.py`
   - **New**: `python parallelrag_core/tests/benchmarks/benchmark_stress_test.py`

9. Line 139: `python tests/benchmarks/benchmark_stress_test.py`
   - **New**: `python parallelrag_core/tests/benchmarks/benchmark_stress_test.py`

10. Line 154: `python test_worker_optimization.py`
    - **New**: `python parallelrag_core/benchmarks/test_worker_optimization.py`

11. Line 171: `python parallelrag_core/visualizations/create_visualizations.py`
    - **New**: `python parallelrag_core/visualizations/create_visualizations.py`

12. Line 186: `python create_resource_charts.py`
    - **New**: `python parallelrag_core/visualizations/create_resource_charts.py`

13. Line 199: `python create_additional_visualizations.py`
    - **New**: `python parallelrag_core/visualizations/create_additional_visualizations.py`

14. Line 217: `python tests/benchmarks/benchmark_chunking.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_chunking.py`

15. Line 225: `python tests/benchmarks/benchmark_embedding.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_embedding.py`

16. Line 233: `python tests/benchmarks/benchmark_llm.py`
    - **New**: `python parallelrag_core/tests/benchmarks/benchmark_llm.py`

---

### `workshop_guides/file_inventory.md`

**File Path References** (22 occurrences in table):

All file paths in the table need to be updated with `parallelrag_core/` prefix:

- `parallelrag_core/examples/parallel_rag_optimized.py` → `parallelrag_core/parallelrag_core/examples/parallel_rag_optimized.py`
- `parallel_rag_app.py` → `parallelrag_core/parallel_rag_app.py`
- `langchain_rag_app.py` → `parallelrag_core/langchain_rag_app.py`
- `tests/benchmarks/*.py` → `parallelrag_core/tests/benchmarks/*.py`
- `benchmarks/*.py` → `parallelrag_core/benchmarks/*.py`
- `benchmarks/frameworks/*.py` → `parallelrag_core/benchmarks/frameworks/*.py`

---

## Other Documentation Files

### Root-level Documentation Files

Many root-level markdown files contain code examples and file references that need updating:

1. **`PARALLEL_RAG_APP_DOCUMENTATION.md`**
   - Import examples: `from parallel_rag_app import ParallelRAG, RAGConfig`
   - Command examples: `python parallelrag_core/parallel_rag_app.py`
   - Test commands: `pytest parallelrag_core/parallelrag_core/tests/test_parallel_rag_app.py -v`

2. **`PARALLEL_RAG_APP_SUMMARY.md`**
   - File references: `parallel_rag_app.py`, `parallelrag_core/tests/test_parallel_rag_app.py`
   - Import examples: `from parallel_rag_app import ParallelRAG`
   - Command examples: `python parallelrag_core/parallel_rag_app.py`

3. **`RAG_IMPLEMENTATION_SUMMARY.md`**
   - File references: `langchain_rag_app.py`, `parallel_rag_app.py`
   - Import examples: `from langchain_rag_app import LangChainRAG`
   - Test commands: `python tests/benchmarks/benchmark_stress_test.py`

4. **`WORKSHOP_DEMO_GUIDE.md`**
   - File references: `langchain_rag_app.py`, `parallelrag_core/examples/parallel_rag_optimized.py`
   - Command examples: `python parallelrag_core/langchain_rag_app.py`

5. **Benchmark Documentation Files**
   - `BENCHMARK_RESULTS.md`
   - `FRAMEWORK_COMPARISON.md`
   - `COMPREHENSIVE_BENCHMARKING_SUMMARY.md`
   - All contain references to benchmark scripts and result files

---

## Update Strategy

### Phase 1: Workshop Guides (High Priority)
1. Update `workshop_guides/NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md` (15 references)
2. Update `workshop_guides/command_reference.md` (18 references)
3. Update `workshop_guides/file_inventory.md` (22 references)
4. Update `workshop_guides/code_architecture_map.md` (if it has file references)

### Phase 2: Application Documentation (High Priority)
1. Update `PARALLEL_RAG_APP_DOCUMENTATION.md`
2. Update `PARALLEL_RAG_APP_SUMMARY.md`
3. Update `RAG_IMPLEMENTATION_SUMMARY.md`
4. Update `WORKSHOP_DEMO_GUIDE.md`

### Phase 3: Benchmark Documentation (Medium Priority)
1. Update all benchmark-related markdown files
2. Update performance analysis documents
3. Update test execution reports

### Phase 4: Implementation Documentation (Low Priority)
1. Update GIL-related documentation
2. Update production deployment guides
3. Update project management documents

---

## Automated Update Script

We can use a PowerShell script to automate most of these updates:

```powershell
# Replace file path references in all markdown files
Get-ChildItem -Path . -Filter "*.md" -Recurse | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $content = $content -replace 'python examples/', 'python parallelrag_core/examples/'
    $content = $content -replace 'python tests/benchmarks/', 'python parallelrag_core/tests/benchmarks/'
    $content = $content -replace 'python parallel_rag_app\.py', 'python parallelrag_core/parallel_rag_app.py'
    $content = $content -replace 'python langchain_rag_app\.py', 'python parallelrag_core/langchain_rag_app.py'
    $content = $content -replace 'from parallel_rag_app import', 'from parallelrag_core.parallel_rag_app import'
    $content = $content -replace 'from langchain_rag_app import', 'from parallelrag_core.langchain_rag_app import'
    Set-Content -Path $_.FullName -Value $content
}
```

---

## Next Steps

1. ✅ Complete documentation reference mapping
2. ⏳ Define target directory structure
3. ⏳ Create directory structure
4. ⏳ Move files
5. ⏳ Update imports in Python files
6. ⏳ Update file references in documentation
7. ⏳ Validate all changes

