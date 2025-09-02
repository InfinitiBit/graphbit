# Google Search API Integration with Graphbit


## Overview

This guideline explains how to connect the Google Search API to Graphbit, enabling Graphbit to orchestrate the retrieval, processing, and utilization of web search results in your AI workflows. This integration allows you to automate research, enrich LLM prompts, and build intelligent pipelines that leverage real-time web data.

---

## Prerequisites

- **Google Custom Search API Key**: Obtain from [Google Cloud Console](https://console.cloud.google.com/apis/credentials).
- **Custom Search Engine (CSE) ID**: Set up a CSE at [Google CSE](https://cse.google.com/cse/all) and ensure it is configured to search the public web.
- **OpenAI API Key**: For LLM summarization (or another supported LLM provider).
- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `requests`, `python-dotenv`, and `graphbit` installed.
- **.env file** in your project root with the following variables:
  ```env
  GOOGLE_API_KEY=your_google_api_key_here
  GOOGLE_CSE_ID=your_search_engine_id_here
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Step 1: Implement the Google Search Connector

Define a function to query the Google Search API, loading credentials from environment variables:

```python
import requests
import os
from dotenv import load_dotenv

load_dotenv()
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
GOOGLE_CSE_ID = os.getenv("GOOGLE_CSE_ID")

def google_search(query):
    url = "https://www.googleapis.com/customsearch/v1"
    params = {
        "key": GOOGLE_API_KEY,
        "cx": GOOGLE_CSE_ID,
        "q": query
    }
    response = requests.get(url, params=params)
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
    items = results.get("items", [])[:max_snippets]
    snippets = [
        f"{item['title']} ({item['link']}): {item['snippet']}"
        for item in items
    ]
    return "\n\n".join(snippets)
```

- If you call `process_search_results(results)`, it will use the default of 3 results.
- To use a different number, call `process_search_results(results, max_snippets=10)` (for example).

---

## Step 3: Build the Graphbit Workflow

1. **Run the Google Search and process the results:**

    ```python
    search_results = google_search("Graphbit open source")
    snippets_text = process_search_results(search_results, max_snippets=10)
    ```

2. **Create a Graphbit agent node for summarization:**

    ```python
    from graphbit import Node, Workflow

    agent = Node.agent(
        name="Summarizer",
        prompt=f"Summarize these search results: {snippets_text}"
    )
    workflow = Workflow("Google Search Workflow")
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
        print("Summary:", result.get_variable("node_result_1"))
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
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
GOOGLE_CSE_ID = os.getenv("GOOGLE_CSE_ID")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

def google_search(query):
    url = "https://www.googleapis.com/customsearch/v1"
    params = {"key": GOOGLE_API_KEY, "cx": GOOGLE_CSE_ID, "q": query}
    response = requests.get(url, params=params)
    response.raise_for_status()
    return response.json()

def process_search_results(results, max_snippets=10):
    items = results.get("items", [])[:max_snippets]
    snippets = [
        f"{item['title']} ({item['link']}): {item['snippet']}"
        for item in items
    ]
    return "\n\n".join(snippets)

search_results = google_search("Graphbit open source")
snippets_text = process_search_results(search_results, max_snippets=10)

agent = Node.agent(
    name="Summarizer",
    prompt=f"Summarize these search results: {snippets_text}"
)
workflow = Workflow("Google Search Workflow")
workflow.add_node(agent)

llm_config = LlmConfig.openai(OPENAI_API_KEY)
executor = Executor(llm_config)

result = executor.execute(workflow)
if result.is_success():
    print("Summary:", result.get_variable("node_result_1"))
else:
    print("Workflow failed:", result.state())
```

---

**This connector pattern enables you to seamlessly blend external web data into your AI workflows, orchestrated by Graphbit.** 
