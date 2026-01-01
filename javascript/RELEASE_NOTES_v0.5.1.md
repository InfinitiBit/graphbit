## Major Features

### LlmClient (11 methods)
Direct language model access without workflow overhead. Supports single completions, batch processing, streaming, and statistics tracking.

- `complete()` - Simple text completion with basic parameters
- `completeAsync()` - Async alias for completion operations
- `completeFull()` - Full response with metadata and token usage
- `completeFullAsync()` - Async version returning full response
- `completeBatch()` - Process multiple prompts with concurrency control
- `completeStream()` - Get streaming responses from LLM
- `chatOptimized()` - Optimized for chat-based interactions
- `getStats()` - Retrieve client statistics and metrics
- `resetStats()` - Clear accumulated statistics
- `warmup()` - Pre-establish connections to LLM provider

---

### Workflow Orchestration (24 methods)
Build and execute multi-step DAG workflows with agent, task, condition, parallel, transform, split/join, and delay nodes.

- `id()` - Get workflow unique identifier
- `name()` - Get workflow name
- `description()` - Get workflow description
- `addNode()` - Add execution node to workflow
- `addEdge()` - Create connection between nodes
- `connect()` - Connect two nodes with data flow
- `validate()` - Check workflow structure for errors
- `isCompleted()` - Check if workflow finished
- `isFailed()` - Check if workflow encountered errors
- `state()` - Get current execution state
- `stats()` - Get execution statistics
- `error()` - Get error message if failed
- `getAllOutputs()` - Retrieve all node outputs
- `setVariable()` - Set workflow variable
- `getVariable()` - Retrieve workflow variable
- `getAllVariables()` - Get all variables
- `getNodeOutput()` - Get output from specific node
- `getNestedOutput()` - Access nested output values
- `getWorkflowId()` - Get workflow identifier
- `getExecutionDuration()` - Get total execution time
- `toDict()` - Serialize workflow to dictionary

---

### Workflow Results (13 methods)
Structured execution output with success/failure checking, node output access, variable management, and execution metrics.

- `isSuccess()` - Check if execution succeeded
- `isFailed()` - Check if execution failed
- `allOutputs()` - Get all node outputs
- `getNodeOutput()` - Retrieve specific node output
- `getNestedOutput()` - Access nested output paths
- `getAllVariables()` - Get all workflow variables
- `setVariable()` - Set workflow variable
- `getVariable()` - Retrieve variable value
- `error()` - Get error message
- `stats()` - Get execution statistics
- `getExecutionDuration()` - Get elapsed time

---

### Tool Registry (16 methods)
Define and manage tools for LLM function calling with execution history, performance metrics, and lifecycle management.

- `register()` - Add new tool with definition
- `unregister()` - Remove tool by name
- `execute()` - Call registered tool
- `listTools()` - Get all registered tools
- `getTool()` - Retrieve tool definition
- `hasTools()` - Check if any tools exist
- `getMetrics()` - Get performance metrics for tool
- `getExecutionHistory()` - Get tool call history
- `clearHistory()` - Clear execution records
- `getLastExecution()` - Get most recent call info
- `getFailureCount()` - Count failed executions
- `setDescription()` - Update tool description
- `enableTool()` - Activate tool
- `disableTool()` - Deactivate tool
- `clearAllTools()` - Remove all tools

---

### Vector Embeddings (3 methods)
Generate embeddings for documents and queries with support for single, batch, and similarity search operations.

- `embed()` - Generate embedding for single text
- `embedBatch()` - Create embeddings for multiple texts
- `findSimilar()` - Find most similar texts from collection

---

### Document Loader (7 methods)
Load and process multi-format documents including PDF, text, CSV, JSON, HTML, Markdown, and DOCX files.

- `loadPdf()` - Load and parse PDF files
- `loadText()` - Load plain text files
- `loadCsv()` - Load and parse CSV data
- `loadJson()` - Load JSON documents
- `loadHtml()` - Load HTML web pages
- `loadMarkdown()` - Load Markdown files
- `loadDocx()` - Load Word documents

---

### Text Splitting (5+ methods)
Intelligent chunking strategies including recursive character splitting, token-aware splitting, and custom delimiter-based splitting.

- `recursiveCharacterSplit()` - Smart character-based chunking with overlap
- `tokenSplit()` - Split by token count for LLM models
- `customSplit()` - Split using custom delimiters
- `paragraphSplit()` - Split on paragraph boundaries
- `sentenceSplit()` - Split on sentence boundaries

---

### Executor
Workflow execution engine with standard, low-latency, and high-throughput performance profiles plus custom configuration options.

- Execute workflows with configurable concurrency and timeouts
- Support for low-latency profile (fast response times)
- Support for high-throughput profile (batch processing)
- Custom configuration with retry policies and circuit breakers
- Initial variable setup and execution monitoring

---

### LLM Configuration (8 providers)
Configure connections to OpenAI, Anthropic, Ollama, OpenRouter, Azure OpenAI, DeepSeek, Replicate, and TogetherAI.

- **OpenAI** - gpt-4, gpt-4o, gpt-4o-mini models
- **Anthropic** - Claude 3 series models
- **Ollama** - Local model inference
- **OpenRouter** - 400+ models via single API
- **Azure OpenAI** - Enterprise Azure deployment
- **DeepSeek** - DeepSeek model access
- **Replicate** - Replicate model hosting
- **TogetherAI** - Together.ai model platform

---

### Core Utilities (6 methods)
Essential helper functions for UUID generation, workflow ID management, timestamps, parsing, and base64 encoding/decoding.

- `generateUUID()` - Create UUID v4
- `generateWorkflowId()` - Generate workflow-specific ID
- `getCurrentTimestamp()` - Get current Unix timestamp
- `parseWorkflowId()` - Parse workflow ID format
- `encodeBase64()` - Encode text to base64
- `decodeBase64()` - Decode base64 to text

---

### Workflow Context (8+ methods)
Access execution environment within agent handlers including node info, variables, outputs, and metadata.

- `getNodeId()` - Get current node identifier
- `getNodeName()` - Get current node name
- `getAllVariables()` - Get all workflow variables
- `getVariable()` - Retrieve specific variable
- `setVariable()` - Set variable value
- `getNodeOutput()` - Get output from node
- `getTimestamp()` - Get execution timestamp
- `getParentWorkflowId()` - Get parent workflow ID

---

## Supporting Features

### Circuit Breaker & Fault Tolerance
Prevents cascading failures across distributed workflows with configurable failure thresholds and recovery mechanisms.

### Retry Configuration & Backoff Strategies
Exponential backoff with configurable retry limits per node, allowing automatic recovery from transient failures.

### Concurrency Management & Performance Tuning
Controls parallel execution limits and optimizes throughput based on workload characteristics.

### Node Types
Flexible workflow composition with Agent (LLM-based), Task (deterministic), Condition (branching), Parallel (concurrent), and Loop (iterative) node types.

### Executor Profiles
Low-latency profile for fast response times and high-throughput profile for batch processing optimization.

### Workflow Validation
Cycle detection, node connectivity validation, and dependency checking to ensure valid workflow structure.

### Performance Metrics
Execution time tracking, token usage monitoring, and resource consumption analytics for optimization insights.

### Error Handling & Recovery
Custom error types with detailed context and recovery strategies for robust workflow execution.

### Text Encoding & Token Utilities
Base64, UTF-8 encoding/decoding and token counting helpers for text processing workflows.
