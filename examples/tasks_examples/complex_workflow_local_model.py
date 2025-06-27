"""Run a complex multi-step workflow using the Mistral model with Graphbit."""

import os
import uuid

import graphbit

COMPLEX_WORKFLOW_STEPS = [
    {
        "task": "ip_threat_analysis",
        "prompt": (
            "You've developed a software application with novel architecture and logic, but the patenting process "
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


def extract_output(result, agent_id: str, fallback_name="Task") -> str:
    """Extract output from the workflow result variables, handling JSON decoding and fallbacks."""
    import contextlib
    import json

    # Get all variables from the result
    variables = result.variables()
    
    if not variables:
        return f"{fallback_name} completed successfully, but no variables were found."
    
    # Try to find the specific output for this agent
    for key, value in variables:
        value_str = str(value).strip()
        
        # Skip null/none values
        if value_str.lower() in ["null", "none", '"null"', '"none"', ""]:
            continue
            
        # Try to decode JSON if it's a JSON string
        try:
            if value_str.startswith('"') and value_str.endswith('"'):
                decoded = json.loads(value_str)
                if decoded and str(decoded).strip().lower() not in ["null", "none", ""]:
                    return str(decoded)
        except json.JSONDecodeError:
            pass
        
        # If it's not JSON, return the raw value if it's meaningful
        if value_str and len(value_str) > 10:  # Only consider meaningful outputs
            return value_str
    
    # If we couldn't find meaningful output, try to get any non-empty variable
    for key, value in variables:
        value_str = str(value).strip()
        if value_str and value_str.lower() not in ["null", "none", '"null"', '"none"']:
            return value_str
    
    return f"{fallback_name} completed successfully, but no detailed output was captured."


def run_complex_workflow_mistral():
    """Run a complex multi-step workflow using the LLaMA model with Graphbit."""
    print("[LOG]  Running Complex Workflow with LLaMA...")
    os.environ["OLLAMA_MODEL"] = "llama3.2"
    model = os.getenv("OLLAMA_MODEL", "llama3.2")

    graphbit.init()
    print(f"[LOG] Using Ollama model: {model}")
    llm_config = graphbit.LlmConfig.ollama(model)

    # Create memory-optimized executor with timeout
    executor = graphbit.Executor.new_memory_optimized(llm_config, timeout_seconds=300)

    agent_id = str(uuid.uuid4())
    
    # Create workflow
    workflow = graphbit.Workflow("Complex Workflow Benchmark")

    # Create nodes and track their IDs
    node_ids = {}
    for step in COMPLEX_WORKFLOW_STEPS:
        node = graphbit.Node.agent(
            name=step["task"],
            prompt=step["prompt"],
            agent_id=agent_id
        )
        node_id = workflow.add_node(node)
        node_ids[step["task"]] = node_id

    # Connect nodes based on dependencies
    for step in COMPLEX_WORKFLOW_STEPS:
        for dep in step.get("depends_on", []):
            workflow.connect(node_ids[dep], node_ids[step["task"]])

    workflow.validate()

    result = executor.execute(workflow)

    if result.is_failed():
        print("[ERROR] Workflow execution failed:", result.state())
        return

    variables = result.variables()
    total_tokens = 0

    print("\n[LOG]  Complex Workflow Output:\n" + "-" * 60)
    
    # Debug: print all variables to understand the structure
    print(f"[DEBUG] Found {len(variables)} variables:")
    for i, (key, value) in enumerate(variables):
        print(f"  Variable {i}: key='{key}', value='{str(value)[:100]}...' (length: {len(str(value))})")
    
    for i, (key, value) in enumerate(variables):
        if i < len(COMPLEX_WORKFLOW_STEPS):
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
