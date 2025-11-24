# Pydantic Validation Integration with Graphbit

## Overview

This guide explains how to integrate Pydantic validation with Graphbit workflows to ensure data quality, type safety, and structured validation. Pydantic provides powerful data validation using Python type hints, making it ideal for validating inputs, outputs, and intermediate data in LLM workflows.

---

## Prerequisites

- **Graphbit installed and configured** (see [installation guide](../getting-started/installation.md)).
- **Python environment** with `pydantic` and `graphbit` installed:
  ```bash
  pip install pydantic graphbit
  ```
- **OpenAI API Key** (or another supported LLM provider).
- **.env file** in your project root with the following variables:
  ```env
  OPENAI_API_KEY=your_openai_api_key_here
  ```

---

## Use Case 1: Validating LLM Input Data

Ensure input data meets required schema before processing in workflows:

```python
import os
from pydantic import BaseModel, Field, field_validator
from graphbit import LlmConfig, Executor, Workflow, Node

class UserQuery(BaseModel):
    """Validated user query model."""
    
    query: str = Field(..., min_length=3, max_length=500)
    user_id: str = Field(..., pattern=r'^[a-zA-Z0-9_-]+$')
    priority: int = Field(default=1, ge=1, le=5)
    
    @field_validator('query')
    @classmethod
    def query_not_empty(cls, v: str) -> str:
        if not v.strip():
            raise ValueError('Query cannot be empty or whitespace')
        return v.strip()

# Validate input before workflow execution
try:
    user_input = UserQuery(
        query="What is machine learning?",
        user_id="user_123",
        priority=2
    )
    
    # Create workflow with validated input
    config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
    executor = Executor(config)
    
    workflow = Workflow("Query Processing")
    agent = Node.agent(
        name="Query Handler",
        prompt=f"Answer this question: {user_input.query}",
        agent_id="handler"
    )
    workflow.add_node(agent)
    
    result = executor.execute(workflow)
    print(result.get_node_output("Query Handler"))
    
except ValueError as e:
    print(f"Validation error: {e}")
```

---

## Use Case 2: Structured LLM Output Validation

Parse and validate LLM responses to ensure they match expected structure:

```python
import os
import json
from pydantic import BaseModel, Field
from typing import List
from graphbit import LlmConfig, LlmClient

class SentimentAnalysis(BaseModel):
    """Validated sentiment analysis output."""
    
    sentiment: str = Field(..., pattern=r'^(positive|negative|neutral)$')
    confidence: float = Field(..., ge=0.0, le=1.0)
    key_phrases: List[str] = Field(default_factory=list, max_length=10)
    
config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
client = LlmClient(config)

text = "This product exceeded my expectations! Highly recommended."

prompt = f"""Analyze the sentiment of this text and return a JSON response:

Text: {text}

Return format:
{{
    "sentiment": "positive|negative|neutral",
    "confidence": 0.0-1.0,
    "key_phrases": ["phrase1", "phrase2"]
}}
"""

response = client.complete(prompt)

# Parse and validate LLM output
try:
    # Extract JSON from response
    json_start = response.find('{')
    json_end = response.rfind('}') + 1
    json_str = response[json_start:json_end]
    
    # Validate with Pydantic
    analysis = SentimentAnalysis.model_validate_json(json_str)
    
    print(f"Sentiment: {analysis.sentiment}")
    print(f"Confidence: {analysis.confidence:.2f}")
    print(f"Key phrases: {', '.join(analysis.key_phrases)}")
    
except ValueError as e:
    print(f"Invalid LLM output: {e}")
```

---

## Use Case 3: Multi-Step Workflow with Validation

Validate data between workflow nodes to ensure data integrity:

```python
import os
from pydantic import BaseModel, Field, field_validator
from typing import Optional
from graphbit import LlmConfig, Executor, Workflow, Node

class DocumentMetadata(BaseModel):
    """Validated document metadata."""
    
    title: str = Field(..., min_length=1, max_length=200)
    summary: str = Field(..., min_length=10)
    category: str = Field(..., pattern=r'^(technical|business|research|other)$')
    word_count: int = Field(..., gt=0)
    
class ProcessedDocument(BaseModel):
    """Validated processed document."""
    
    metadata: DocumentMetadata
    key_insights: list[str] = Field(..., min_length=1, max_length=5)
    action_items: Optional[list[str]] = None

# Sample document
document = """
Machine learning is transforming industries by enabling computers to learn from data.
Companies should invest in ML infrastructure and training to stay competitive.
Key applications include predictive analytics, natural language processing, and computer vision.
"""

config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
executor = Executor(config)

workflow = Workflow("Document Processing Pipeline")

# Step 1: Extract metadata
metadata_extractor = Node.agent(
    name="Metadata Extractor",
    prompt=f"""Extract metadata from this document and return JSON:

Document: {document}

Return format:
{{
    "title": "document title",
    "summary": "brief summary",
    "category": "technical|business|research|other",
    "word_count": number
}}
""",
    agent_id="metadata_extractor"
)

# Step 2: Extract insights
insight_extractor = Node.agent(
    name="Insight Extractor",
    prompt="""Based on the document, extract 3-5 key insights and action items.

Return JSON format:
{
    "key_insights": ["insight1", "insight2", ...],
    "action_items": ["action1", "action2", ...]
}
""",
    agent_id="insight_extractor"
)

metadata_id = workflow.add_node(metadata_extractor)
insight_id = workflow.add_node(insight_extractor)
workflow.connect(metadata_id, insight_id)

result = executor.execute(workflow)

# Validate outputs at each step
try:
    # Validate metadata
    metadata_output = result.get_node_output("Metadata Extractor")
    json_start = metadata_output.find('{')
    json_end = metadata_output.rfind('}') + 1
    metadata_json = metadata_output[json_start:json_end]
    metadata = DocumentMetadata.model_validate_json(metadata_json)

    # Validate insights
    insight_output = result.get_node_output("Insight Extractor")
    json_start = insight_output.find('{')
    json_end = insight_output.rfind('}') + 1
    insight_json = insight_output[json_start:json_end]
    insights_data = json.loads(insight_json)

    # Combine and validate final output
    processed = ProcessedDocument(
        metadata=metadata,
        key_insights=insights_data.get("key_insights", []),
        action_items=insights_data.get("action_items")
    )

    print(f"Title: {processed.metadata.title}")
    print(f"Category: {processed.metadata.category}")
    print(f"Insights: {len(processed.key_insights)}")

except ValueError as e:
    print(f"Validation failed: {e}")
```

---

**This integration enables you to leverage Pydantic's powerful validation capabilities with Graphbit workflows for type-safe, validated LLM applications with stronger data quality.**
