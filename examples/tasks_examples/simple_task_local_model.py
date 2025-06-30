"""Run a simple task using llama3.2 model via Ollama with Graphbit."""

import os
import uuid

import graphbit

SIMPLE_TASK_PROMPT = (
    "You've discovered a mysterious journal entry from a time traveler. "
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


def run_simple_task_local():
    """Run a simple task using llama3.2 model via Ollama with Graphbit."""
    print("[LOG]  Starting Simple Task with llama3.2 (Ollama)...")
    os.environ["OLLAMA_MODEL"] = "llama3.2"  # use available model
    model = os.getenv("OLLAMA_MODEL", "llama3.2")

    graphbit.init()

    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.LlmConfig.ollama(model)

    executor = graphbit.Executor.new_low_latency(llm_config)

    agent_id = str(uuid.uuid4())

    workflow = graphbit.Workflow("Simple Task Workflow")

    node = graphbit.Node.agent(name="Task Executor", prompt=SIMPLE_TASK_PROMPT, agent_id=agent_id)

    workflow.add_node(node)
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
    run_simple_task_local()
