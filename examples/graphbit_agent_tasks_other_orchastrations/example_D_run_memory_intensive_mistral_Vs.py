"""This example demonstrates how to run a memory-intensive task using the Mistral model with Ollama."""

import os
import uuid

import graphbit

MEMORY_INTENSIVE_PROMPT = (
    "You are tasked with conducting an in-depth legal and technical research analysis for a novel software application "
    "with proprietary architecture, logic, and workflow. The developer seeks immediate protection of intellectual property, "
    "as the patenting process is underway and may take a significant amount of time.\n\n"
    "Below is a detailed brief provided by the developer:\n\n"
    + (
        "[DEVELOPER BRIEF START]\n"
        "I have developed a software application with a novel architecture, logic, and workflow. While the patenting process "
        "may take a significant amount of time, I need to ensure immediate protection of my intellectual property through "
        "legally binding and technically enforced licensing mechanisms. My primary concern is to prevent unauthorized use, "
        "replication, or exploitation of the software in any form, especially during the pre-patent period.\n\n"
        "Specifically, I am seeking a comprehensive legal and technical framework that will:\n"
        "- Prohibit unauthorized use, redistribution, or modification of the software without explicit consent, enforced through a legally sound licensing agreement.\n"
        "- Prevent reverse engineering, source code extraction, and replication of the core algorithms, programming logic, and internal architecture.\n"
        "- Include strong legal language in the license that defines such violations as punishable under relevant intellectual property and software licensing laws, "
        "with enforceability in both national and international jurisdictions.\n"
        "- Ensure that even in the absence of a granted patent, the licensing terms provide substantive legal protection under frameworks such as the DMCA, "
        "WIPO Copyright Treaty, and TRIPS Agreement.\n"
        "- Be supported by technical enforcement mechanisms, such as:\n"
        "  • License key validation systems to control usage\n"
        "  • Code obfuscation and binary protection to deter reverse engineering\n"
        "  • Watermarking or fingerprinting to trace leaked copies\n"
        "  • Tamper detection and anti-debugging tools to protect runtime behavior\n"
        "  • Integration of legal notices and EULAs directly within the software runtime\n"
        "[DEVELOPER BRIEF END]\n\n"
    )
    * 3  # Repeating for token load simulation
    + "Please provide a deep and structured analysis covering the following:\n"
    "1. The most suitable licensing models for this context (e.g., Proprietary Commercial License, Custom EULA, Dual Licensing, Source-available with restrictions).\n"
    "2. Concrete examples of enforceable legal clauses that deter tampering, unauthorized distribution, and derivative work creation.\n"
    "3. Best practices across jurisdictions (US, EU, global) for enforcing such protections pre-patent.\n"
    "4. Recommended tools and services for technical enforcement (e.g., DRM, license key servers, anti-tamper).\n"
    "5. Immediate IP protection strategies (copyright, trade secrets, software escrow).\n"
    "6. Final recommendations that ensure the software cannot be copied, cloned, or reverse-engineered without enforceable legal consequences, "
    "even if a formal patent is still pending.\n\n"
    "Focus on providing actionable insights with a balance of legal rigor and technical practicality. Assume the reader is an IP-conscious founder "
    "preparing for enterprise distribution globally."
)


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


def run_memory_intensive_mistral():
    """Run a memory-intensive task using the Mistral model with Ollama."""
    print("[LOG]  Running Memory-Intensive Task with Mistral...")
    os.environ["OLLAMA_MODEL"] = "mistral"
    model = os.getenv("OLLAMA_MODEL", "mistral")

    graphbit.init()
    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.PyLlmConfig.ollama(model)

    executor = (
        graphbit.PyWorkflowExecutor.new_memory_optimized(llm_config)
        .with_max_node_execution_time(60000)
        .with_fail_fast(False)
        .with_retry_config(graphbit.PyRetryConfig(max_attempts=3).with_exponential_backoff(initial_delay_ms=100, multiplier=1.5, max_delay_ms=3000))
        .with_circuit_breaker_config(graphbit.PyCircuitBreakerConfig(failure_threshold=3, recovery_timeout_ms=15000))
    )

    agent_id = str(uuid.uuid4())
    builder = graphbit.PyWorkflowBuilder("Memory Intensive Workflow")
    builder.description("Runs one task with large input")

    node = graphbit.PyWorkflowNode.agent_node(
        name="Memory Intensive Task",
        description="Analyzes large dataset",
        agent_id=agent_id,
        prompt=MEMORY_INTENSIVE_PROMPT,
    )
    builder.add_node(node)

    workflow = builder.build()
    workflow.validate()

    result = executor.execute(workflow)

    if result.is_failed():
        print("[ERROR] Workflow execution failed:", result.state())
        return

    output = extract_output(result, agent_id, "Memory Intensive Task")
    tokens = count_tokens_estimate(MEMORY_INTENSIVE_PROMPT + output)

    print("\n[LOG]  Output (truncated):")
    print(output[:500] + ("..." if len(output) > 500 else ""))
    print(f"[LOG]  Tokens used: {tokens}")
    print("[LOG]  Memory-intensive task completed.")


if __name__ == "__main__":
    run_memory_intensive_mistral()
