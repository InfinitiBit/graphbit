# Python Task Examples

This directory contains ready-to-run Python scripts demonstrating various GraphBit workflow patterns using both local and cloud LLMs.

## How to Run

1. **Set up your environment:**
- For local models (Ollama):  
```bash
ollama serve
ollama pull llama3.2
```
- For Perplexity (cloud):  
```bash
export PERPLEXITY_API_KEY="your-api-key"
```

2. **Run an example:**
```bash
python examples/tasks_examples/simple_task_local_model.py
```

---

## 🛠️ GraphBit Python API Tutorial

All example scripts in this directory use the **GraphBit Python API** to build and run AI workflows. Here’s a minimal step-by-step guide to the core GraphBit workflow pattern, as seen in these scripts:

### 1. **Initialize GraphBit**
```python
import graphbit
graphbit.init()
```
This sets up the GraphBit runtime and logging.

---

### 2. **Configure Your LLM Provider**
- **openai (cloud):**
  ```python
  llm_config = graphbit.LlmConfig.openai(model=gpt-3.5-turbo, api_key=api_key)
  ```
- **Ollama (local):**
  ```python
  llm_config = graphbit.LlmConfig.ollama("llama3.2")
  ```
- **Perplexity (cloud):**
  ```python
  llm_config = graphbit.LlmConfig.perplexity(api_key, "sonar")
  ```

---

### 3. **Create an Executor**
Choose the executor type based on your use case:
```python
executor = graphbit.Executor.new_low_latency(llm_config)

# or for high-throughput pipelines:
executor = graphbit.Executor.new_high_throughput(llm_config, timeout_seconds=60)

# or for memory-intensive tasks:
executor = graphbit.Executor.new_memory_optimized(llm_config, timeout_seconds=300)

# Configure additional settings for memory-intensive tasks if needed
executor.configure(timeout_seconds=300, max_retries=3, enable_metrics=True, debug=False)
```
---

### 4. **Build a Workflow**
Create a workflow and add agent nodes:
```python
workflow = graphbit.Workflow("My Example Workflow")
node = graphbit.Node.agent(
    name="Task Executor",
    prompt="Summarize this text: {input}",
    agent_id="unique-agent-id"
)
workflow.add_node(node1)
workflow.add_node(node2)
workflow.connect(node1,node2)
workflow.validate()
```
For multi-step or complex workflows, add multiple nodes and connect them as needed.

---

### 5. **Run the Workflow**
```python
result = executor.execute(workflow)
if result.is_failed():
    print("Workflow failed:", result.state())
else:
    print("Output:", result.variables())
```

---

### 6. **Example: Minimal End-to-End Script**
```python
import graphbit
import uuid

graphbit.init()
llm_config = graphbit.LlmConfig.ollama("llama3.2")
executor = graphbit.Executor.new_low_latency(llm_config)
workflow = graphbit.Workflow("Simple Task")
agent_id = str(uuid.uuid4())
node = graphbit.Node.agent(
    name="Summarizer",
    prompt="Summarize: {input}",
    agent_id=agent_id
)
workflow.add_node(node1)
workflow.add_node(node2)
workflow.connect(node1,node2)
workflow.validate()
result = executor.execute(workflow)
print("Result:", result.variables())
```

---

**Explore the scripts in this folder for more advanced patterns:**
- Real-time web search with Perplexity (`simple_task_perplexity.py`)
- Memory-optimized large prompt tasks (`memory_task_local_model.py`)
- Multi-step and dependency-based workflows (`sequential_task_local_model.py`, `complex_workflow_local_model.py`)

*For more details, see the [GraphBit Python API documentation](../docs/index.md).*

---

## Available Python Examples

**simple_task_local_model.py**  
*Single-agent workflow using the local Llama 3.2 model via Ollama. Summarizes a fictional journal entry.*  
_Requires Ollama running locally._

**sequential_task_local_model.py**  
*Sequential multi-step pipeline using Llama 3.2. Each step addresses a different aspect of software IP protection, with outputs chained stepwise.*  
_Requires Ollama running locally._

**complex_workflow_local_model.py**  
*Complex, multi-step workflow with explicit dependencies between tasks, covering a comprehensive IP protection strategy.*  
_Requires Ollama running locally._

**memory_task_local_model.py**  
*Memory-intensive, single-agent task with a large prompt, using Llama 3.2 via Ollama. Provides a deep legal/technical analysis.*  
_Requires Ollama running locally._

**simple_task_perplexity.py**  
*Single-agent workflow using Perplexity’s cloud models (with real-time web search). Summarizes recent AI/ML developments.*  
_Requires `PERPLEXITY_API_KEY` environment variable._

**chatbot**  
*A conversational AI chatbot with vector database integration for context retrieval and memory storage. Includes a FastAPI backend and Streamlit frontend.*  
_Requires OpenAI API key and ChromaDB._

**llm_guided_browser_automation.py**  
*Automates browser interactions using LLMs to guide actions. Demonstrates how to use GraphBit for real-time decision-making in web automation tasks.*  
_Requires Selenium and a configured LLM provider._
