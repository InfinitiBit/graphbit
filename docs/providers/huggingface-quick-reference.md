# HuggingFace Quick Reference

## Installation

```bash
pip install graphbit huggingface_hub python-dotenv
```

## Setup

```python
from graphbit import init
from dotenv import load_dotenv

load_dotenv()
init()
```

---

## Rust-Based Integration (Workflows)

### Simple Chat

```python
from graphbit import LlmConfig, LlmClient
import os

config = LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="meta-llama/Llama-3.3-70B-Instruct"
)

client = LlmClient(config)
response = client.complete(prompt="Hello!")
print(response)
```

### Single Agent Workflow

```python
from graphbit import LlmConfig, Workflow, Node, Executor
import os

config = LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="mistralai/Mistral-7B-Instruct-v0.3"
)

executor = Executor(config, timeout_seconds=120)
workflow = Workflow("My Workflow")

node = Node.agent(
    name="Agent",
    prompt="Your prompt here",
    agent_id="agent1"
)

workflow.add_node(node)
workflow.validate()

result = executor.execute(workflow)
print(result.get_node_output("Agent"))
```

### Multi-Provider Workflow

```python
from graphbit import LlmConfig, Workflow, Node, Executor
import os

hf_config = LlmConfig.huggingface(
    api_key=os.getenv("HUGGINGFACE_API_KEY"),
    model="mistralai/Mistral-7B-Instruct-v0.3"
)

openai_config = LlmConfig.openai(
    api_key=os.getenv("OPENAI_API_KEY"),
    model="gpt-4o-mini"
)

executor = Executor(openai_config, timeout_seconds=120)
workflow = Workflow("Multi-Provider")

# HuggingFace node
node1 = Node.agent(
    name="HF Agent",
    prompt="Generate content",
    llm_config=hf_config,  # Override
    agent_id="hf"
)

# OpenAI node (uses executor default)
node2 = Node.agent(
    name="OpenAI Agent",
    prompt="Summarize",
    agent_id="openai"
)

n1 = workflow.add_node(node1)
n2 = workflow.add_node(node2)
workflow.connect(n1, n2)
workflow.validate()

result = executor.execute(workflow)
```

---

## Direct Python Wrapper

### Chat Completion

```python
from graphbit.providers import Huggingface
import os

hf = Huggingface(api_token=os.getenv("HUGGINGFACE_API_KEY"))

messages = [{"role": "user", "content": "Hello!"}]

response = hf.llm.chat(
    model="meta-llama/Llama-3.3-70B-Instruct",
    messages=messages,
    max_tokens=100
)

content = hf.llm.get_output_content(response)
print(content)
```

### Embeddings

```python
from graphbit.providers import Huggingface
import os

hf = Huggingface(api_token=os.getenv("HUGGINGFACE_API_KEY"))

embeddings = hf.embeddings.embed(
    model="sentence-transformers/all-MiniLM-L6-v2",
    text="Your text here"
)

print(f"Dimension: {len(embeddings)}")
```

### Sentence Similarity

```python
from graphbit.providers import Huggingface
import os

hf = Huggingface(api_token=os.getenv("HUGGINGFACE_API_KEY"))

similarities = hf.embeddings.similarity(
    model="sentence-transformers/all-MiniLM-L6-v2",
    sentence="I love coding",
    other_sentences=["I enjoy programming", "I like pizza"]
)

print(similarities)
```
