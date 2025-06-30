"""Run a sequential pipeline with Mistral using Graphbit, executing a series of tasks with dependencies."""

import os
import uuid

import graphbit

SEQUENTIAL_TASKS = [
    "Research and outline a legal strategy to protect a software library (and future SDK) from unauthorized use, replication, and reverse engineering—especially during the pre-patent stage.",
    "Focus on US, EU, and international IP law.\n(Max tokens: 150)",
    "Propose enforceable licensing models suitable for enterprise-level distribution that include clear terms for usage, restrictions, and legal remedies.\n(Max tokens: 100)",
    "Identify technical safeguards that can be integrated into the software to prevent reverse engineering or unauthorized redistribution.\n(Max tokens: 100)",
    "Summarize the complete legal and technical protection framework in a concise executive summary for stakeholders.\n(Max tokens: 120)",
]


def count_tokens_estimate(text: str) -> int:
    """Estimate token count based on a simple heuristic."""
    return len(text) // 4


def extract_output(context, agent_id: str, fallback_name="Task") -> str:
    """Extract output from the context variables, handling JSON decoding and fallbacks."""
    import contextlib
    import json

    variables = context.variables()
    print("[DEBUG] Extracting output...")
    if variables:
        for _key, value in variables:
            value_str = str(value).strip()
            print(f"[DEBUG] {_key}: {value_str!r}")
            if value_str.lower() in ["null", "none", '"null"', '"none"']:
                print("[WARN]  Model returned 'null'. Possibly no valid output.")
                continue
            with contextlib.suppress(json.JSONDecodeError):
                if value_str.startswith('"') and value_str.endswith('"'):
                    decoded = json.loads(value_str)
                    if decoded and str(decoded).strip().lower() != "null":
                        return str(decoded)
            if value_str:
                return value_str
    return f"{fallback_name} completed successfully, but no detailed output was captured."


def run_sequential_pipeline_mistral():
    """Run a sequential pipeline with llama3.2 using Graphbit, executing a series of tasks with dependencies."""
    print("[LOG]  Starting Sequential Pipeline with llama3.2 (context-chained)...")
    os.environ["OLLAMA_MODEL"] = "llama3.2"
    model = os.getenv("OLLAMA_MODEL", "llama3.2")

    graphbit.init()
    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.LlmConfig.ollama(model)

    # Create executor with high throughput mode and configure timeout
    executor = graphbit.Executor.new_high_throughput(llm_config, timeout_seconds=60)
    # Configure additional settings
    executor.configure(timeout_seconds=60, max_retries=2, enable_metrics=True, debug=True)

    # Create workflow directly (no builder pattern in Python bindings)
    workflow = graphbit.Workflow("Sequential Pipeline Workflow")

    previous_node_id = None
    total_tokens = 0
    node_ids = []

    # Create nodes for each task
    for i, task in enumerate(SEQUENTIAL_TASKS):
        print(f"[DEBUG] Adding node for Task {i+1}: {task}")

        # Generate unique agent ID for each step
        agent_id = str(uuid.uuid4())

        node = graphbit.Node.agent(name=f"Step {i+1}", prompt=task, agent_id=agent_id)

        node_id = workflow.add_node(node)
        node_ids.append(node_id)

        # Connect to previous node to create sequential flow
        if previous_node_id is not None:
            workflow.connect(previous_node_id, node_id)

        previous_node_id = node_id

    workflow.validate()

    result = executor.execute(workflow)
    print(f"[DEBUG] Workflow state: {result.state()}")

    if result.is_failed():
        print("[ERROR]  Workflow failed:", result.state())
        return

    variables = result.variables()
    print("\n[LOG]  Step-by-step outputs:\n" + "-" * 60)

    # Extract outputs for each step
    for i, (_key, _value) in enumerate(variables):
        if i < len(SEQUENTIAL_TASKS):
            output = extract_output(result, f"agent_{i}", f"Step {i+1}")
            prompt = SEQUENTIAL_TASKS[i]
            tokens = count_tokens_estimate(prompt + output)
            total_tokens += tokens
            print(f"\nStep {i+1}: {prompt}\n→ {output}\n[Tokens: {tokens}]")

    print("\n" + "-" * 60)
    print(f"[LOG]  Total tokens used: {total_tokens}")
    print("[LOG]  Sequential pipeline completed successfully.")


if __name__ == "__main__":
    run_sequential_pipeline_mistral()
