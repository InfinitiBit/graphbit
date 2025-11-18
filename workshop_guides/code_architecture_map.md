# Non-Ollama ParallelRAG Code Architecture Map

**Purpose**: Detailed code architecture documentation for workshop demonstrations  
**Last Updated**: November 17, 2025  
**Scope**: Cloud-based LLM providers (OpenAI, Anthropic) - **EXCLUDES** Ollama integration

---

## 📚 Table of Contents

1. [Example Implementations](#example-implementations)
2. [Benchmark Infrastructure](#benchmark-infrastructure)
3. [Framework Comparison](#framework-comparison)
4. [Visualization Scripts](#visualization-scripts)
5. [Utility Modules](#utility-modules)

---

## 1. Example Implementations

### 1.1 `examples/parallel_rag_optimized.py` (354 lines)

**Purpose**: Optimized ParallelRAG implementation showcasing GraphBit's GIL-releasing architecture

#### Key Classes

**`ParallelRAG` (lines 30-354)**
- **Responsibility**: Massively concurrent RAG system with true parallelism
- **Key Features**:
  - GIL-releasing document loading (10-50x speedup)
  - GIL-releasing embedding generation (5-10x speedup)
  - Lock-free batch processing (10-50x speedup)
  - Async LLM queries (5-20x speedup)

#### Critical Code Sections

**Initialization (lines 41-72)**
```python
def __init__(self, openai_api_key: str, max_workers: int = 10, 
             chunk_size: int = 500, chunk_overlap: int = 50):
    # Initialize GraphBit components
    self.loader = DocumentLoader()                              # Line 60
    self.splitter = RecursiveSplitter(...)                      # Line 61
    self.embed_client = EmbeddingClient(embed_config)           # Line 65
    self.llm_client = LlmClient(llm_config)                     # Line 69
```

**Parallel Document Loading (lines 74-95)**
```python
def load_documents_parallel(self, doc_paths: List[str]) -> List[Dict[str, Any]]:
    # GIL is released during load_document() - TRUE parallelism
    with ThreadPoolExecutor(max_workers=self.max_workers) as executor:  # Line 84
        futures = [executor.submit(self._load_single_document, path) 
                   for path in doc_paths]                               # Lines 85-88
        documents = [f.result() for f in futures if f.result() is not None]  # Line 89
```

**Parallel Chunking (lines 119-140)**
```python
def chunk_documents_parallel(self, documents: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    # Parallel chunking with ThreadPoolExecutor
    with ThreadPoolExecutor(max_workers=self.max_workers) as executor:  # Line 127
        futures = [executor.submit(self._chunk_single_document, doc) 
                   for doc in documents]                                # Lines 128-131
```

**Optimized Embedding Generation (lines 142-191)**
```python
def embed_chunks_parallel_optimized(self, chunks: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    # OPTIMIZED: Lock-free parallel batch processing
    batch_size = 100                                            # Line 153
    batches = [chunks[i:i + batch_size] 
               for i in range(0, len(chunks), batch_size)]      # Lines 154-155
    
    # Process batches in parallel (GIL released)
    with ThreadPoolExecutor(max_workers=self.max_workers) as executor:  # Line 158
        futures = [executor.submit(self._embed_batch, batch) 
                   for batch in batches]                        # Lines 159-162
```

**Async LLM Query (lines 234-267)**
```python
async def query_async(self, query: str, top_k: int = 5) -> str:
    # Async LLM completion (non-blocking)
    response = await self.llm_client.complete_async(prompt, max_tokens=500)  # Line 263
```

#### Entry Point

**`main()` function (lines 270-354)**
- Demonstrates complete RAG pipeline
- Creates sample documents
- Processes documents in parallel
- Performs async query

---

### 1.2 `parallel_rag_app.py` (334 lines)

**Purpose**: Production-ready RAG application with optimal configurations from benchmarks

#### Key Classes

**`RAGConfig` (lines 29-48)**
- **Responsibility**: Configuration dataclass with optimal settings
- **Key Settings**:
  - `chunk_size=200, chunk_overlap=20` (optimal from benchmarks)
  - `chunking_workers=20` (6.20x speedup)
  - `embedding_workers=20` (34.81x speedup)
  - `llm_workers=20` (19.04x speedup)

**`ParallelRAG` (lines 51-334)**
- **Responsibility**: Production RAG system with error handling
- **Key Features**:
  - TokenSplitter (best performance from benchmarks)
  - Comprehensive statistics tracking
  - Production error handling
  - Resource monitoring

#### Critical Code Sections

**Initialization (lines 54-100)**
```python
def __init__(self, config: Optional[RAGConfig] = None):
    # Initialize GraphBit
    graphbit.init()                                             # Line 71
    
    # Create TokenSplitter (best performance)
    self.splitter = graphbit.TokenSplitter(...)                 # Lines 74-77
    
    # Create embedding client
    self.embed_client = graphbit.EmbeddingClient(embed_config)  # Line 84
    
    # Create LLM client
    self.llm_client = graphbit.LlmClient(llm_config)            # Line 91
```

**Parallel Chunking (lines 102-135)**
```python
def chunk_documents(self, documents: List[str]) -> List[str]:
    # Parallel chunking with optimal worker count
    with ThreadPoolExecutor(max_workers=self.config.chunking_workers) as executor:  # Line 113
        results = list(executor.map(self.splitter.split_text, documents))           # Line 114
```

**Parallel Embedding (lines 137-175)**
```python
def generate_embeddings(self, chunks: List[str]) -> List[List[float]]:
    # Batch processing for efficiency
    batch_size = 100                                            # Line 148
    
    # Parallel batch processing
    with ThreadPoolExecutor(max_workers=self.config.embedding_workers) as executor:  # Line 156
        futures = [executor.submit(self._embed_batch, batch) 
                   for batch in batches]                        # Lines 157-160
```

#### Entry Point

**`main()` function (lines 177-334)**
- Demonstrates production RAG usage
- Includes error handling
- Shows statistics tracking

---

## 2. Benchmark Infrastructure

### 2.1 `tests/benchmarks/benchmark_framework_comparison.py` (733 lines)

**Purpose**: GraphBit vs LangChain RAG comparison with identical workloads

#### Key Classes

**`FrameworkTestResult` (lines 82-120)**
- **Responsibility**: Store test results for a single framework
- **Key Metrics**:
  - Performance: `total_time`, `load_time`, `chunk_time`, `embed_time`
  - Output: `documents_loaded`, `chunks_created`, `embeddings_generated`
  - Resources: `peak_memory_mb`, `avg_cpu_percent`
  - Safety: `memory_threshold_exceeded`, `cpu_threshold_exceeded`

**`ComparisonResult` (lines 122-135)**
- **Responsibility**: Store comparison results for both frameworks
- **Key Data**: `graphbit_result`, `langchain_result`, `speedup_factor`

#### Critical Functions

**`test_graphbit_rag()` (lines 137-250)**
```python
def test_graphbit_rag(doc_paths, num_workers):
    # Initialize GraphBit RAG
    rag = ParallelRAG(...)                                      # Lines 148-152
    
    # Load documents in parallel
    documents = rag.load_documents_parallel(doc_paths)          # Line 165
    
    # Chunk documents in parallel
    chunks = rag.chunk_documents_parallel(documents)            # Line 172
    
    # Generate embeddings (optimized)
    chunks_with_embeddings = rag.embed_chunks_parallel_optimized(chunks)  # Line 179
```

**`test_langchain_rag()` (lines 252-370)**
```python
def test_langchain_rag(doc_paths):
    # Initialize LangChain RAG
    config = LangChainRAGConfig(...)                            # Lines 263-267
    rag = LangChainRAG(config)                                  # Line 268
    
    # Process documents (sequential)
    rag.process_documents(doc_paths)                            # Line 281
```

**`run_comparison_test()` (lines 472-550)**
- Generates test documents
- Runs both frameworks
- Calculates speedup
- Monitors resources

#### Entry Point

**`main()` function (lines 652-733)**
- Parses command-line arguments
- Runs progressive load tests
- Prints comparison summary
- Saves results to JSON

---

### 2.2 `tests/benchmarks/benchmark_stress_test.py` (~600 lines)

**Purpose**: Progressive load testing and worker scaling for maximum capacity analysis

#### Key Functions

**`run_progressive_load_test()` (lines ~400-480)**
```python
def run_progressive_load_test(document_counts, num_workers, words_per_doc):
    # Test with increasing document counts: 100, 500, 1000, 5000, 10000
    for num_docs in document_counts:
        result = run_stress_test(num_docs, num_workers, words_per_doc)
        results.append(result)

        # Check safety thresholds
        if result.hit_memory_threshold:
            print("⚠️  Memory threshold exceeded - stopping tests")
            break
```

**`run_worker_scaling_test()` (lines ~482-550)**
```python
def run_worker_scaling_test(num_documents, worker_counts, words_per_doc):
    # Test with different worker counts: 5, 10, 20, 50, 100
    for num_workers in worker_counts:
        result = run_stress_test(num_documents, num_workers, words_per_doc)
        results.append(result)
```

**`run_stress_test()` (lines ~200-398)**
- Generates test documents
- Monitors resources (CPU%, Memory MB)
- Checks safety thresholds (90% memory, 95% CPU)
- Returns comprehensive metrics

#### Entry Point

**`main()` function (lines 533-594)**
- Parses command-line arguments
- Runs progressive load test
- Runs worker scaling test
- Prints summary with best performance

---

### 2.3 `tests/benchmarks/benchmark_utils.py` (~300 lines)

**Purpose**: Shared utilities for all benchmark scripts

#### Key Functions

**`get_system_info()` (lines ~50-80)**
```python
def get_system_info() -> Dict[str, Any]:
    # Returns: platform, processor, python_version, cpu_count, total_memory_gb
    return {
        "platform": platform.platform(),
        "processor": platform.processor(),
        "python_version": platform.python_version(),
        "cpu_count": psutil.cpu_count(logical=False),
        "total_memory_gb": psutil.virtual_memory().total / (1024**3)
    }
```

**`measure_execution_time()` (lines ~82-100)**
```python
def measure_execution_time(func, *args, **kwargs) -> Tuple[Any, float]:
    # Measures execution time of a function
    start_time = time.time()
    result = func(*args, **kwargs)
    duration = time.time() - start_time
    return result, duration
```

**`measure_memory_usage()` (lines ~102-130)**
```python
def measure_memory_usage() -> Dict[str, float]:
    # Returns: memory_mb, memory_percent, num_threads
    process = psutil.Process()
    memory_info = process.memory_info()
    return {
        "memory_mb": memory_info.rss / (1024 * 1024),
        "memory_percent": process.memory_percent(),
        "num_threads": process.num_threads()
    }
```

**`calculate_throughput()` (lines ~132-145)**
```python
def calculate_throughput(num_items: int, duration: float) -> float:
    # Returns items per second
    return num_items / duration if duration > 0 else 0.0
```

---

## 3. Framework Comparison

### 3.1 `benchmarks/run_benchmark.py` (748 lines)

**Purpose**: Multi-framework comparison runner (GraphBit, LangChain, LangGraph, CrewAI, LlamaIndex, PydanticAI)

#### Key Classes

**`ComprehensiveBenchmarkRunner` (lines ~100-580)**
- **Responsibility**: Orchestrate benchmarks across all frameworks
- **Key Methods**:
  - `run_all_benchmarks()` - Run all framework benchmarks
  - `run_framework_benchmarks()` - Run benchmarks for specific framework
  - `run_scenario()` - Run specific scenario
  - `generate_comparison_report()` - Generate comparison report
  - `create_visualizations()` - Create performance charts

#### Critical Code Sections

**Framework Configuration (lines ~120-180)**
```python
self.frameworks = {
    FrameworkType.GRAPHBIT: {
        "name": "GraphBit",
        "benchmark": GraphBitBenchmark(llm_config),
        "results": {},
        "errors": {}
    },
    FrameworkType.LANGCHAIN: {
        "name": "LangChain",
        "benchmark": LangChainBenchmark(llm_config),
        "results": {},
        "errors": {}
    },
    # ... other frameworks
}
```

**Scenario Definitions (lines ~182-220)**
```python
self.scenarios = [
    (BenchmarkScenario.SIMPLE_TASK, "Simple Task"),
    (BenchmarkScenario.SEQUENTIAL_PIPELINE, "Sequential Pipeline"),
    (BenchmarkScenario.PARALLEL_PIPELINE, "Parallel Pipeline"),
    (BenchmarkScenario.COMPLEX_WORKFLOW, "Complex Workflow"),
    (BenchmarkScenario.MEMORY_INTENSIVE, "Memory Intensive"),
    (BenchmarkScenario.CONCURRENT_TASKS, "Concurrent Tasks")
]
```

**Run Scenario (lines ~238-280)**
```python
async def run_scenario(self, framework_type, scenario):
    # Run scenario multiple times (num_runs)
    all_metrics = []
    for run_idx in range(self.num_runs):
        metrics = await benchmark.run_scenario(scenario)
        all_metrics.append(metrics)

    # Average results
    avg_metrics = self._average_metrics(all_metrics)
    return avg_metrics
```

#### Entry Point

**`main()` function (lines 600-748)**
- Parses command-line arguments (provider, model, frameworks, scenarios)
- Validates API keys
- Creates benchmark runner
- Runs all benchmarks
- Generates reports and visualizations

**Command-Line Options**:
- `--provider`: LLM provider (openai, anthropic, ollama)
- `--model`: Model name (gpt-4o-mini, claude-sonnet-4, etc.)
- `--frameworks`: Comma-separated frameworks to test
- `--scenarios`: Comma-separated scenarios to run
- `--num-runs`: Number of runs per scenario (results averaged)
- `--verbose`: Enable verbose output

---

### 3.2 `benchmarks/frameworks/graphbit_benchmark.py` (~200 lines)

**Purpose**: GraphBit framework implementation for benchmarks

#### Key Class

**`GraphBitBenchmark` (lines ~30-200)**
- **Responsibility**: Implement benchmark scenarios using GraphBit
- **Key Methods**:
  - `run_scenario()` - Run specific benchmark scenario
  - `_simple_task()` - Simple LLM completion
  - `_sequential_pipeline()` - Sequential task pipeline
  - `_parallel_pipeline()` - Parallel task pipeline
  - `_complex_workflow()` - Complex multi-step workflow

#### Critical Code Sections

**Simple Task (lines ~60-80)**
```python
async def _simple_task(self) -> BenchmarkMetrics:
    # Direct LLM client call (minimal overhead)
    response = self.llm_client.complete(SIMPLE_TASK_PROMPT, max_tokens=100)
```

**Parallel Pipeline (lines ~120-160)**
```python
async def _parallel_pipeline(self) -> BenchmarkMetrics:
    # Execute tasks in parallel using asyncio.gather
    tasks = [self.llm_client.complete_async(task, max_tokens=100)
             for task in PARALLEL_TASKS]
    responses = await asyncio.gather(*tasks)
```

---

### 3.3 `benchmarks/frameworks/langchain_benchmark.py` (~200 lines)

**Purpose**: LangChain framework implementation for benchmarks

#### Key Class

**`LangChainBenchmark` (lines ~30-200)**
- **Responsibility**: Implement benchmark scenarios using LangChain
- **Key Methods**: Same as GraphBitBenchmark but using LangChain APIs

#### Critical Code Sections

**LLM Client Setup (lines ~40-70)**
```python
def _get_llm_client(self):
    if self.llm_config.provider == LLMProvider.OPENAI:
        return ChatOpenAI(
            api_key=SecretStr(self.llm_config.api_key),
            model=self.llm_config.model,
            temperature=self.llm_config.temperature
        )
    elif self.llm_config.provider == LLMProvider.ANTHROPIC:
        return ChatAnthropic(...)
```

**Simple Task (lines ~80-100)**
```python
async def _simple_task(self) -> BenchmarkMetrics:
    # LangChain LCEL pattern
    prompt = PromptTemplate.from_template("{input}")
    chain = prompt | self.llm_client
    response = await chain.ainvoke({"input": SIMPLE_TASK_PROMPT})
```

---

## 4. Visualization Scripts

### 4.1 `create_visualizations.py` (230 lines)

**Purpose**: Generate 5 main performance charts from benchmark results

#### Key Functions

**`create_chart1_total_time()` (lines 43-70)**
```python
def create_chart1_total_time(graphbit_data, langchain_data, output_file='chart_total_time.png'):
    # Chart: Total Time vs Document Count
    plt.figure(figsize=(12, 7))
    plt.plot(gb_docs, gb_times, 'o-', label='GraphBit', color='#2E86AB')
    plt.plot(lc_docs, lc_times, 's-', label='LangChain', color='#A23B72')
    plt.xlabel('Document Count')
    plt.ylabel('Total Time (seconds)')
    plt.title('Total Processing Time: GraphBit vs LangChain')
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
```

**`create_chart2_throughput()` (lines 72-100)**
```python
def create_chart2_throughput(graphbit_data, langchain_data, output_file='chart_throughput.png'):
    # Chart: Throughput (docs/sec) vs Document Count
    # Shows GraphBit's superior throughput at scale
```

**`create_chart3_speedup()` (lines 102-130)**
```python
def create_chart3_speedup(graphbit_data, langchain_data, output_file='chart_speedup.png'):
    # Chart: GraphBit Speedup Factor vs LangChain
    # Shows speedup increasing with document count
```

**`create_chart4_component_breakdown()` (lines 132-170)**
```python
def create_chart4_component_breakdown(graphbit_data, langchain_data, output_file='chart_component_breakdown.png'):
    # Chart: Component Time Breakdown (Load, Chunk, Embed, Store)
    # Stacked bar chart showing time distribution
```

**`create_chart5_extended_capacity()` (lines 172-200)**
```python
def create_chart5_extended_capacity(output_file='chart_extended_capacity.png'):
    # Chart: Extended Capacity Results (100K, 250K, 500K docs)
    # Shows GraphBit's ability to scale to massive datasets
```

#### Entry Point

**`main()` function (lines 202-230)**
- Loads JSON results
- Extracts data
- Creates all 5 charts
- Saves PNG files

---

### 4.2 `create_resource_charts.py` (~240 lines)

**Purpose**: Generate 3 resource utilization charts

#### Key Functions

**`create_chart_memory_usage()` (lines 37-100)**
```python
def create_chart_memory_usage(output_file='chart_memory_usage.png'):
    # Chart: Memory Usage Across Document Scales
    # Shows memory consumption from 100 to 500K documents
```

**`create_chart_cpu_utilization()` (lines 102-162)**
```python
def create_chart_cpu_utilization(output_file='chart_cpu_utilization.png'):
    # Chart: CPU Utilization Patterns
    # Shows CPU usage across different scales
```

**`create_chart_resource_efficiency()` (lines 164-218)**
```python
def create_chart_resource_efficiency(output_file='chart_resource_efficiency.png'):
    # Chart: Throughput per GB Memory
    # Shows resource efficiency metric
```

---

### 4.3 `create_additional_visualizations.py` (~305 lines)

**Purpose**: Generate 4 additional analysis charts

#### Key Functions

**`create_chart_worker_optimization()` (lines ~30-60)**
- Chart: Worker Count Optimization
- Shows optimal worker count for different hardware

**`create_chart_document_size_impact()` (lines ~63-138)**
- Chart: Document Size Impact on Performance
- Shows performance with 100, 2000, 10000 words/doc

**`create_chart_cost_comparison()` (lines ~140-206)**
- Chart: Cost Comparison GraphBit vs LangChain
- Shows cost savings with GraphBit

**`create_chart_scaling_efficiency()` (lines ~208-290)**
- Chart: Scaling Efficiency (100 to 500K documents)
- Shows linear scaling characteristics

---

## 5. Utility Modules

### 5.1 `benchmarks/frameworks/common.py` (~300 lines)

**Purpose**: Shared utilities for framework benchmarks

#### Key Classes

**`FrameworkType` (lines 103-112)**
```python
class FrameworkType(Enum):
    GRAPHBIT = "GraphBit"
    LANGCHAIN = "LangChain"
    LANGGRAPH = "LangGraph"
    PYDANTIC_AI = "PydanticAI"
    LLAMAINDEX = "LlamaIndex"
    CREWAI = "CrewAI"
```

**`BenchmarkScenario` (lines 114-122)**
```python
class BenchmarkScenario(Enum):
    SIMPLE_TASK = "simple_task"
    SEQUENTIAL_PIPELINE = "sequential_pipeline"
    PARALLEL_PIPELINE = "parallel_pipeline"
    COMPLEX_WORKFLOW = "complex_workflow"
    MEMORY_INTENSIVE = "memory_intensive"
    CONCURRENT_TASKS = "concurrent_tasks"
```

**`BenchmarkMetrics` (lines ~50-100)**
- Dataclass for storing benchmark results
- Fields: execution_time, memory_usage, cpu_usage, throughput, tokens_processed

#### Key Constants

**Benchmark Prompts (lines ~10-48)**
```python
SIMPLE_TASK_PROMPT = "Explain quantum computing in one sentence."
SEQUENTIAL_TASKS = ["Task 1", "Task 2", "Task 3"]
PARALLEL_TASKS = ["Parallel task 1", "Parallel task 2", "Parallel task 3"]
COMPLEX_WORKFLOW_STEPS = [...]
MEMORY_INTENSIVE_PROMPT = "..."
CONCURRENT_TASK_PROMPTS = [...]
```

---

## 📝 Summary

This architecture map provides:
- **Detailed class and function documentation** with line numbers
- **Critical code sections** with actual code snippets
- **Entry points** for all executable scripts
- **Key configurations** and constants
- **Cross-references** between related files

Use this map to:
- Navigate the codebase during workshops
- Explain code architecture to attendees
- Reference specific implementations
- Understand data flow and dependencies

