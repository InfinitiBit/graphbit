"""Run a simple task using Mistral model via Ollama with Graphbit."""

import os
import uuid

import graphbit

SIMPLE_TASK_PROMPT = (
    "You’ve discovered a mysterious journal entry from a time traveler. "
    "Summarize what you can infer about their journey and its impact:\n"
    "'March 14, 3024: Landed in Neo-Rome. Artificial sun powered by quantum batteries. "
    "Warned them about the fusion instability. Might have changed history—again.'"
)


def count_tokens_estimate(text: str) -> int:
    """Estimate token count based on a simple heuristic."""
    return len(text) // 4


def extract_output(context, agent_id: str, fallback_name="Task") -> str:
    """Extract output from the context variables, handling JSON decoding and fallbacks."""
    import contextlib
    import json

    variables = context.variables()

    print("[DEBUG] Inspecting context variables for output...")
    if variables:
        for _key, value in variables:
            value_str = str(value).strip()
            print(f"[DEBUG] Variable {_key}: {value_str!r}")

            if value_str.lower() in ["null", "none", '"null"', '"none"']:
                continue

            with contextlib.suppress(json.JSONDecodeError):
                if value_str.startswith('"') and value_str.endswith('"'):
                    decoded = json.loads(value_str)
                    if decoded and str(decoded).strip().lower() != "null":
                        return str(decoded)

            if value_str:
                return value_str

    print("[WARN]  No usable output found — using fallback.")
    return f"{fallback_name} completed successfully, but no detailed output was captured."


def run_simple_task_mistral():
    """Run a simple task using Mistral model via Ollama with Graphbit."""
    print("[LOG]  Starting Simple Task with Mistral (Ollama)...")
    os.environ["OLLAMA_MODEL"] = "mistral"  # enforce if not already set
    model = os.getenv("OLLAMA_MODEL", "mistral")

    graphbit.init()

    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.PyLlmConfig.ollama(model)

    executor = (
        graphbit.PyWorkflowExecutor.new_low_latency(llm_config)
        .with_max_node_execution_time(10000)
        .with_fail_fast(True)
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig(failure_threshold=3, recovery_timeout_ms=15000))
    )

    agent_id = str(uuid.uuid4())

    builder = graphbit.PyWorkflowBuilder("Simple Task Workflow")
    builder.description("Single task benchmark using Mistral")

    node = graphbit.PyWorkflowNode.agent_node(
        name="Task Executor",
        description="Executes simple summarization task",
        agent_id=agent_id,
        prompt=SIMPLE_TASK_PROMPT,
    )
    builder.add_node(node)
    workflow = builder.build()
    workflow.validate()

    result = executor.execute(workflow)

    print(f"[DEBUG] Workflow state: {result.state()}")

    if result.is_failed():
        print("[LOG]  Workflow execution failed:", result.state())
        return

    output = extract_output(result, agent_id, "Task Executor")
    print(f"\n[LOG]  Final Output:\n{output}")

    token_count = count_tokens_estimate(SIMPLE_TASK_PROMPT + output)
    print(f"[LOG]  Tokens used: {token_count}")


if __name__ == "__main__":
    run_simple_task_mistral()
