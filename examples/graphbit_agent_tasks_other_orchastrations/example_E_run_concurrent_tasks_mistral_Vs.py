"""# Example: Run Concurrent Tasks with Mistral Model via Ollama using Graphbit."""

import os
import uuid

import graphbit

CONCURRENT_TASK_PROMPTS = [
    "Identify the most suitable software licensing models for protecting proprietary applications during the pre-patent phase (e.g., Proprietary License, Custom EULA, Dual Licensing).",
    "Draft example legal clauses that prohibit tampering, reverse engineering, and unauthorized redistribution of software.",
    "Explain how to legally enforce software licensing violations internationally under frameworks like DMCA, WIPO Copyright Treaty, and TRIPS.",
    "List best practices across major jurisdictions (e.g., US, EU, APAC) for enforcing IP rights in software without a granted patent.",
    "Evaluate technical enforcement methods for software protection, including license key systems, obfuscation, and anti-debugging tools.",
    "Describe how watermarking or fingerprinting can be used to trace unauthorized distribution or leaked versions of software.",
    "Explain how legal notices and embedded EULAs within software runtimes contribute to enforceability of IP rights.",
    "Recommend tools and services (e.g., activation servers, DRM solutions, compliance checkers) that support technical licensing enforcement.",
    "Propose interim strategies for IP protection before a patent is granted, such as copyright registration or trade secret protocols.",
    "Summarize how a combined legal + technical approach can offer enforceable protection for novel software even without an active patent.",
]


def count_tokens_estimate(text: str) -> int:
    """Estimate token count based on a simple heuristic."""
    return len(text) // 4


def extract_output(context, agent_id: str, fallback_name="Task") -> str:
    """Extract output from the context variables, handling JSON decoding and fallbacks."""
    import contextlib
    import json

    variables = context.variables()
    if variables:
        for _key, value in variables:
            value_str = str(value).strip()
            if value_str.lower() in ["null", "none", '"null"', '"none"']:
                continue
            with contextlib.suppress(json.JSONDecodeError):
                if value_str.startswith('"') and value_str.endswith('"'):
                    decoded = json.loads(value_str)
                    if decoded and str(decoded).strip().lower() != "null":
                        return str(decoded)
            if value_str:
                return value_str
    return f"{fallback_name} completed successfully, but no detailed output was captured."


def run_concurrent_tasks_mistral():
    """Run simulated concurrent tasks using Mistral model via Ollama with Graphbit."""
    print("[LOG]  Running Simulated Concurrent Tasks with Mistral...")
    os.environ["OLLAMA_MODEL"] = "mistral"
    model = os.getenv("OLLAMA_MODEL", "mistral")

    graphbit.init()
    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.PyLlmConfig.ollama(model)

    executor = (
        graphbit.PyWorkflowExecutor.new_high_throughput(llm_config)
        .with_max_node_execution_time(20000)
        .with_fail_fast(False)
        .with_retry_config(graphbit.PyRetryConfig(max_attempts=2).with_exponential_backoff(initial_delay_ms=100, multiplier=1.5, max_delay_ms=3000))
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig(failure_threshold=3, recovery_timeout_ms=15000))
    )

    agent_id = str(uuid.uuid4())
    total_tokens = 0
    successful = 0

    print("\n[LOG]  Executing 10 tasks sequentially (simulated parallel):\n" + "-" * 60)

    for i, prompt in enumerate(CONCURRENT_TASK_PROMPTS):
        task_name = f"Task {i+1}"
        print(f"\n[DEBUG] Executing {task_name}: {prompt}")

        builder = graphbit.PyWorkflowBuilder(task_name)
        node = graphbit.PyWorkflowNode.agent_node(
            name=task_name,
            description=task_name,
            agent_id=agent_id,
            prompt=prompt,
        )
        builder.add_node(node)
        workflow = builder.build()
        workflow.validate()

        result = executor.execute(workflow)

        if result.is_failed():
            print(f"[ERROR] {task_name} failed: {result.state()}")
            continue

        output = extract_output(result, agent_id, task_name)
        tokens = count_tokens_estimate(prompt + output)
        total_tokens += tokens
        successful += 1

        preview = output[:200].replace("\n", " ")
        print(f"{task_name} → {preview}... [Tokens: {tokens}]")

    print("\n" + "-" * 60)
    print(f"[LOG]  {successful}/{len(CONCURRENT_TASK_PROMPTS)} tasks succeeded.")
    print(f"[LOG]  Total tokens used: {total_tokens}")
    print("[LOG]  Simulated concurrent tasks completed.")


if __name__ == "__main__":
    run_concurrent_tasks_mistral()
