# Serper API Integration with Graphbit


## Overview

This guideline explains how to connect the Serper API to Graphbit, enabling Graphbit to orchestrate the retrieval, processing, and utilization of web search results in your AI workflows. This integration allows you to automate research, enrich LLM prompts, and build intelligent pipelines that leverage real-time web data with a simple and cost-effective search API.

---

## Prerequisites

- **Serper API Key**: Obtain from [Serper.dev](https://serper.dev/api-key) by creating a free account.
- **OpenAI API Key**: For LLM summarization (or another supported LLM provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `requests`, `python-dotenv`, and `graphbit` installed.
- **.env file** in your project root with the following variables:
  ```env
  SERPER_API_KEY=your_serper_api_key_here
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Step 1: Implement the Serper Search Connector

Define a function to query the Serper API, loading credentials from environment variables:

```python
import requests
import os
from dotenv import load_dotenv

load_dotenv()
SERPER_API_KEY = os.getenv("SERPER_API_KEY")

def serper_search(query, num_results=10):
    url = "https://google.serper.dev/search"
    headers = {
        "X-API-KEY": SERPER_API_KEY,
        "Content-Type": "application/json"
    }
    payload = {
        "q": query,
        "num": num_results
    }
    response = requests.post(url, json=payload, headers=headers)
    response.raise_for_status()
    return response.json()
```

---

## Step 2: Process the Search Results

Extract relevant information (title, link, and snippet) from the search results for downstream use. By default, only the top 3 results are included, but you can override this by specifying the max_snippets parameter:

```python
def process_search_results(results, max_snippets=3):
    """
    Extracts up to max_snippets search results (default: 3) as formatted strings.
    """
    organic_results = results.get("organic", [])[:max_snippets]
    snippets = [
        f"{result['title']} ({result['link']}): {result['snippet']}"
        for result in organic_results
    ]
    return "\n\n".join(snippets)
```

- If you call `process_search_results(results)`, it will use the default of 3 results.
- To use a different number, call `process_search_results(results, max_snippets=10)` (for example).

---

## Step 3: Build the Graphbit Workflow

1. **Run the Serper Search and process the results:**

    ```python
    search_results = serper_search("Graphbit open source", num_results=10)
    snippets_text = process_search_results(search_results, max_snippets=10)
    ```

2. **Create a Graphbit agent node for summarization:**

    ```python
    from graphbit import Node, Workflow

    agent = Node.agent(
        name="Summarizer",
        prompt=f"Summarize these search results: {snippets_text}"
    )
    workflow = Workflow("Serper Search Workflow")
    workflow.add_node(agent)
    ```

---

## Step 4: Orchestrate and Execute with Graphbit

1. **Initialize Graphbit and configure your LLM:**

    ```python
    from graphbit import LlmConfig, Executor
    from dotenv import load_dotenv
    import os
    load_dotenv()
    llm_config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"))
    executor = Executor(llm_config)
    ```

2. **Run the workflow and retrieve the summary:**

    ```python
    result = executor.execute(workflow)
    if result.is_success():
        print("Summary:", result.get_variable("Summarizer"))
    else:
        print("Workflow failed:", result.state())
    ```

---

## Full Example

```python
import requests
from graphbit import Node, Workflow, LlmConfig, Executor
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()
SERPER_API_KEY = os.getenv("SERPER_API_KEY")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

def serper_search(query, num_results=10):
    url = "https://google.serper.dev/search"
    headers = {
        "X-API-KEY": SERPER_API_KEY,
        "Content-Type": "application/json"
    }
    payload = {"q": query, "num": num_results}
    response = requests.post(url, json=payload, headers=headers)
    response.raise_for_status()
    return response.json()

def process_search_results(results, max_snippets=10):
    organic_results = results.get("organic", [])[:max_snippets]
    snippets = [
        f"{result['title']} ({result['link']}): {result['snippet']}"
        for result in organic_results
    ]
    return "\n\n".join(snippets)

search_results = serper_search("Graphbit open source", num_results=10)
snippets_text = process_search_results(search_results, max_snippets=10)

agent = Node.agent(
    name="Summarizer",
    prompt=f"Summarize these search results: {snippets_text}"
)
workflow = Workflow("Serper Search Workflow")
workflow.add_node(agent)

llm_config = LlmConfig.openai(OPENAI_API_KEY)
executor = Executor(llm_config)

result = executor.execute(workflow)
if result.is_success():
    print("Summary:", result.get_variable("Summarizer"))
else:
    print("Workflow failed:", result.state())
```

---

## Advanced Features

### News Search

Serper API also supports news search for recent articles:

```python
def serper_news_search(query, num_results=10):
    url = "https://google.serper.dev/news"
    headers = {
        "X-API-KEY": SERPER_API_KEY,
        "Content-Type": "application/json"
    }
    payload = {"q": query, "num": num_results}
    response = requests.post(url, json=payload, headers=headers)
    response.raise_for_status()
    return response.json()

def process_news_results(results, max_snippets=3):
    news_results = results.get("news", [])[:max_snippets]
    snippets = [
        f"{result['title']} ({result['link']}) - {result['date']}: {result['snippet']}"
        for result in news_results
    ]
    return "\n\n".join(snippets)
```

### Image Search

For image search capabilities:

```python
def serper_image_search(query, num_results=10):
    url = "https://google.serper.dev/images"
    headers = {
        "X-API-KEY": SERPER_API_KEY,
        "Content-Type": "application/json"
    }
    payload = {"q": query, "num": num_results}
    response = requests.post(url, json=payload, headers=headers)
    response.raise_for_status()
    return response.json()
```

---

**This connector pattern enables you to seamlessly blend external web data into your AI workflows, orchestrated by Graphbit with the simplicity and cost-effectiveness of Serper API.**