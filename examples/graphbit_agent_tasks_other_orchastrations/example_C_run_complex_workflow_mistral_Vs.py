"""Run a complex multi-step workflow using the Mistral model with Graphbit."""

import os
import uuid

import graphbit

COMPLEX_WORKFLOW_STEPS = [
    {
        "task": "ip_threat_analysis",
        "prompt": (
            "You’ve developed a software application with novel architecture and logic, but the patenting process "
            "will take time. Analyze and outline the key intellectual property threats during the pre-patent period, "
            "including unauthorized use, reverse engineering, and replication."
        ),
        "depends_on": [],
    },
    {
        "task": "legal_framework_design",
        "prompt": (
            "Based on the threat analysis, draft a comprehensive legal protection strategy. "
            "This should include enforceable licensing models (e.g., proprietary, dual, or custom EULA), "
            "DMCA/WIPO/TRIPS coverage, and international jurisdiction best practices."
        ),
        "depends_on": ["ip_threat_analysis"],
    },
    {
        "task": "technical_enforcement_plan",
        "prompt": (
            "List and describe technical safeguards that should be integrated into the software to prevent reverse engineering, "
            "source code extraction, unauthorized redistribution, or tampering. Include methods like license validation, "
            "binary obfuscation, watermarking, anti-debugging, and embedded EULA notices."
        ),
        "depends_on": ["ip_threat_analysis"],
    },
    {
        "task": "licensing_clause_examples",
        "prompt": (
            "Draft enforceable example clauses for the licensing agreement that prohibit unauthorized use, modification, "
            "distribution, or creation of derivative works. Ensure clauses align with global software licensing and IP law."
        ),
        "depends_on": ["legal_framework_design", "technical_enforcement_plan"],
    },
    {
        "task": "pre_patent_protection_recommendations",
        "prompt": (
            "Recommend immediate, non-patent-based IP protections such as copyright registration, trade secret protocols, "
            "software escrow, and terms of service strategies. Emphasize how these support enforceability in global markets."
        ),
        "depends_on": ["legal_framework_design"],
    },
    {
        "task": "final_strategy_summary",
        "prompt": (
            "Summarize the full legal and technical protection strategy for your software during the pre-patent period. "
            "Highlight how legal clauses, technical safeguards, and international enforcement work together to protect IP "
            "and ensure violations are punishable by law even without a granted patent."
        ),
        "depends_on": ["licensing_clause_examples", "technical_enforcement_plan", "pre_patent_protection_recommendations"],
    },
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


def run_complex_workflow_mistral():
    """Run a complex multi-step workflow using the Mistral model with Graphbit."""
    print("[LOG]  Running Complex Workflow with Mistral...")
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
    builder = graphbit.PyWorkflowBuilder("Complex Workflow Benchmark")
    builder.description("Multi-node DAG benchmark")

    node_ids = {}
    for step in COMPLEX_WORKFLOW_STEPS:
        node = graphbit.PyWorkflowNode.agent_node(
            name=step["task"],
            description=f"Executes: {step['task']}",
            agent_id=agent_id,
            prompt=step["prompt"],
        )
        node_ids[step["task"]] = builder.add_node(node)

    for step in COMPLEX_WORKFLOW_STEPS:
        for dep in step.get("depends_on", []):
            builder.connect(node_ids[dep], node_ids[step["task"]], graphbit.PyWorkflowEdge.data_flow())

    workflow = builder.build()
    workflow.validate()

    result = executor.execute(workflow)

    if result.is_failed():
        print("[ERROR] Workflow execution failed:", result.state())
        return

    variables = result.variables()
    total_tokens = 0

    print("\n[LOG]  Complex Workflow Output:\n" + "-" * 60)
    for i, (_key, _value) in enumerate(variables):
        prompt = COMPLEX_WORKFLOW_STEPS[i]["prompt"]
        task = COMPLEX_WORKFLOW_STEPS[i]["task"]
        output = extract_output(result, agent_id, task)
        tokens = count_tokens_estimate(prompt + output)
        total_tokens += tokens

        print(f"\nStep {i+1}: {task}\n→ {output[:300]}...\n[Tokens: {tokens}]")

    print("\n" + "-" * 60)
    print(f"[LOG]  Total tokens used: {total_tokens}")
    print("[LOG]  Complex workflow completed.")


if __name__ == "__main__":
    run_complex_workflow_mistral()
