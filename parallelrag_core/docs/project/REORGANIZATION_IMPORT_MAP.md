# Repository Reorganization - Import Dependency Map

**Date**: 2025-11-18  
**Purpose**: Document all import dependencies that need updating after reorganization

---

## Critical Import Dependencies

### 1. Test Files Importing Application Files

#### `parallelrag_core/tests/test_parallel_rag_app.py`
**Current Import**:
```python
from parallel_rag_app import ParallelRAG, RAGConfig
```

**New Import** (after moving to `parallelrag_core/tests/`):
```python
from parallelrag_core.parallel_rag_app import ParallelRAG, RAGConfig
```

---

#### `parallelrag_core/tests/test_langchain_rag_app.py`
**Current Import**:
```python
from langchain_rag_app import LangChainRAG, LangChainRAGConfig
```

**New Import** (after moving to `parallelrag_core/tests/`):
```python
from parallelrag_core.langchain_rag_app import LangChainRAG, LangChainRAGConfig
```

---

### 2. Benchmark Files Importing Application Files

#### `tests/benchmarks/benchmark_framework_comparison.py`
**Current Imports**:
```python
sys.path.insert(0, str(Path(__file__).parent.parent.parent))
from langchain_rag_app import LangChainRAG, LangChainRAGConfig
from benchmark_utils import get_system_info
```

**New Imports** (after moving to `parallelrag_core/tests/benchmarks/`):
```python
# Remove sys.path manipulation
from parallelrag_core.langchain_rag_app import LangChainRAG, LangChainRAGConfig
from parallelrag_core.tests.benchmarks.benchmark_utils import get_system_info
```

---

#### `parallelrag_core/benchmarks/run_benchmark.py`
**Current Imports**:
```python
from frameworks.common import (...)
from frameworks.crewai_benchmark import CrewAIBenchmark
from frameworks.graphbit_benchmark import GraphBitBenchmark
from frameworks.langchain_benchmark import LangChainBenchmark
from frameworks.langgraph_benchmark import LangGraphBenchmark
from frameworks.llamaindex_benchmark import LlamaIndexBenchmark
from frameworks.pydantic_ai_benchmark import PydanticAIBenchmark
```

**New Imports** (after moving to `parallelrag_core/benchmarks/`):
```python
from parallelrag_core.benchmarks.frameworks.common import (...)
from parallelrag_core.benchmarks.frameworks.crewai_benchmark import CrewAIBenchmark
from parallelrag_core.benchmarks.frameworks.graphbit_benchmark import GraphBitBenchmark
from parallelrag_core.benchmarks.frameworks.langchain_benchmark import LangChainBenchmark
from parallelrag_core.benchmarks.frameworks.langgraph_benchmark import LangGraphBenchmark
from parallelrag_core.benchmarks.frameworks.llamaindex_benchmark import LlamaIndexBenchmark
from parallelrag_core.benchmarks.frameworks.pydantic_ai_benchmark import PydanticAIBenchmark
```

---

### 3. Application Files (No Changes Needed)

#### `parallel_rag_app.py`
**Current Imports**:
```python
import os
import time
from concurrent.futures import ThreadPoolExecutor
from typing import List, Dict, Any, Optional
from dataclasses import dataclass
import graphbit
```

**Status**: ✅ No changes needed - all imports are from standard library or external packages

---

#### `langchain_rag_app.py`
**Current Imports**:
```python
import os
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Dict, Any, Optional
from langchain_text_splitters import RecursiveCharacterTextSplitter
from langchain_community.vectorstores import FAISS
from langchain_core.documents import Document
from langchain_openai import ChatOpenAI, OpenAIEmbeddings
```

**Status**: ✅ No changes needed - all imports are from standard library or external packages

---

## Import Update Strategy

### Phase 1: Move Files Without Changing Imports
- Move all files to new locations
- Keep imports as-is temporarily
- This allows us to track what breaks

### Phase 2: Update Imports in Test Files
- Update `parallelrag_core/tests/test_parallel_rag_app.py`
- Update `parallelrag_core/tests/test_langchain_rag_app.py`
- Update all files in `tests/benchmarks/`
- Update all files in `tests/python_integration_tests/`
- Update all files in `tests/python_unit_tests/`

### Phase 3: Update Imports in Benchmark Files
- Update `parallelrag_core/benchmarks/run_benchmark.py`
- Update all files in `benchmarks/frameworks/`

### Phase 4: Update sys.path Manipulations
- Remove all `sys.path.insert()` calls
- Replace with proper package imports

### Phase 5: Validation
- Run `python -m pytest parallelrag_core/tests/` to verify imports work
- Run individual test files to verify functionality
- Check for any remaining import errors

---

## Files Requiring Import Updates

### High Priority (Direct Dependencies)
1. `parallelrag_core/tests/test_parallel_rag_app.py` - imports `parallel_rag_app`
2. `parallelrag_core/tests/test_langchain_rag_app.py` - imports `langchain_rag_app`
3. `tests/benchmarks/benchmark_framework_comparison.py` - imports both RAG apps
4. `parallelrag_core/benchmarks/run_benchmark.py` - imports framework benchmarks

### Medium Priority (Indirect Dependencies)
5. All files in `tests/benchmarks/` that import `benchmark_utils`
6. All files in `benchmarks/frameworks/` that import from `common`
7. All files in `tests/python_integration_tests/` that may import test utilities

### Low Priority (Documentation Examples)
8. Code examples in markdown files (will be updated in Phase 5)

---

## Package Structure After Reorganization

```
parallelrag_core/
├── __init__.py                        # Make it a package
├── parallel_rag_app.py                # Importable as: from parallelrag_core.parallel_rag_app import ParallelRAG
├── langchain_rag_app.py               # Importable as: from parallelrag_core.langchain_rag_app import LangChainRAG
├── examples/
│   ├── __init__.py
│   └── parallel_rag_optimized.py
├── benchmarks/
│   ├── __init__.py
│   ├── run_benchmark.py
│   └── frameworks/
│       ├── __init__.py
│       └── ...
├── tests/
│   ├── __init__.py
│   ├── test_parallel_rag_app.py
│   ├── test_langchain_rag_app.py
│   └── benchmarks/
│       ├── __init__.py
│       └── ...
└── ...
```

---

## Next Steps

1. ✅ Complete import dependency mapping
2. ⏳ Map documentation references
3. ⏳ Define target directory structure
4. ⏳ Create directory structure with `__init__.py` files
5. ⏳ Move files
6. ⏳ Update imports systematically
7. ⏳ Validate imports work

