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
    """Run a sequential pipeline with Mistral using Graphbit, executing a series of tasks with dependencies."""
    print("[LOG]  Starting Sequential Pipeline with Mistral (context-chained)...")
    os.environ["OLLAMA_MODEL"] = "mistral"
    model = os.getenv("OLLAMA_MODEL", "mistral")

    graphbit.init()
    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.PyLlmConfig.ollama(model)

    executor = (
        graphbit.PyWorkflowExecutor.new_high_throughput(llm_config)
        .with_max_node_execution_time(15000)
        .with_fail_fast(False)
        .with_retry_config(graphbit.PyRetryConfig(max_attempts=2).with_exponential_backoff(initial_delay_ms=100, multiplier=1.5, max_delay_ms=3000))
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig(failure_threshold=3, recovery_timeout_ms=15000))
    )

    agent_id = str(uuid.uuid4())
    builder = graphbit.PyWorkflowBuilder("Sequential Pipeline Workflow")
    builder.description("Chained 4-task pipeline with dependency")

    previous_node_id = None
    total_tokens = 0

    for i, task in enumerate(SEQUENTIAL_TASKS):
        print(f"[DEBUG] Adding node for Task {i+1}: {task}")
        node = graphbit.PyWorkflowNode.agent_node(
            name=f"Step {i+1}",
            description=f"Executes step {i+1}",
            agent_id=agent_id,
            prompt=task,
        )
        node_id = builder.add_node(node)

        if previous_node_id is not None:
            builder.connect(previous_node_id, node_id, graphbit.PyWorkflowEdge.data_flow())

        previous_node_id = node_id

    workflow = builder.build()
    workflow.validate()

    result = executor.execute(workflow)
    print(f"[DEBUG] Workflow state: {result.state()}")

    if result.is_failed():
        print("[ERROR]  Workflow failed:", result.state())
        return

    variables = result.variables()
    print("\n[LOG]  Step-by-step outputs:\n" + "-" * 60)

    for i, (_key, _value) in enumerate(variables):
        output = extract_output(result, agent_id, f"Step {i+1}")
        prompt = SEQUENTIAL_TASKS[i]
        tokens = count_tokens_estimate(prompt + output)
        total_tokens += tokens
        print(f"\nStep {i+1}: {prompt}\n→ {output}\n[Tokens: {tokens}]")

    print("\n" + "-" * 60)
    print(f"[LOG]  Total tokens used: {total_tokens}")
    print("[LOG]  Sequential pipeline completed successfully.")


if __name__ == "__main__":
    run_sequential_pipeline_mistral()
