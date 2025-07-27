"""Run a simple task using Perplexity's search-enabled models with GraphBit."""

import os
import uuid

import graphbit

# Demonstration task that benefits from real-time web search
SEARCH_TASK_PROMPT = (
    "As a research assistant, provide a comprehensive but concise summary of the latest "
    "developments in artificial intelligence and machine learning that have occurred in "
    "the past 30 days. Include any major breakthroughs, new model releases, research "
    "papers, or industry announcements. Focus on the most significant and impactful "
    "developments and include sources where possible."
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


def run_simple_task_perplexity():
    """Run a search task using Perplexity's search-enabled models with GraphBit."""
    print("[LOG]  Starting Search Task with Perplexity...")

    # Check for API key
    api_key = os.getenv("PERPLEXITY_API_KEY")
    if not api_key:
        print("[ERROR] PERPLEXITY_API_KEY environment variable not set!")
        print("[INFO]  Please set your Perplexity API key:")
        print("        export PERPLEXITY_API_KEY='your-api-key-here'")
        return

    # Choose model - can be overridden with environment variable
    model = os.getenv("PERPLEXITY_MODEL", "sonar")

    print(f"[LOG]  Using Perplexity model: {model}")
    print("[LOG]  This model has real-time web search capabilities for current information")

    graphbit.init()

    # Configure Perplexity with search-enabled model
    llm_config = graphbit.LlmConfig.perplexity(api_key, model)

    # Create executor optimized for cloud API performance
    executor = graphbit.Executor.new_low_latency(llm_config)

    agent_id = str(uuid.uuid4())

    workflow = graphbit.Workflow("Real-time Search Task Workflow")

    # Create agent node with search task
    node = graphbit.Node.agent(name="Search Research Assistant", prompt=SEARCH_TASK_PROMPT, agent_id=agent_id)

    workflow.add_node(node)
    workflow.validate()

    print("[LOG]  Executing workflow with real-time search...")
    print("[LOG]  Note: This may take longer as the model searches for current information")

    result = executor.execute(workflow)

    print(f"[DEBUG] Workflow state: {result.state()}")

    if result.is_failed():
        print("[LOG]  Workflow execution failed:", result.state())
        return

    output = extract_output(result, agent_id, "Search Research Assistant")
    print(f"\n[LOG]  Final Output with Real-time Data:\n{output}")

    token_count = count_tokens_estimate(SEARCH_TASK_PROMPT + output)
    print(f"[LOG]  Estimated tokens used: {token_count}")

    print(f"\n[LOG]  ✅ Successfully completed search task using Perplexity's {model} model")
    print("[LOG]  The response above includes real-time information from web search!")


def example_with_different_models():
    """Demonstrate different Perplexity models for various use cases."""
    api_key = os.getenv("PERPLEXITY_API_KEY")
    if not api_key:
        print("[INFO]  To try different models, set PERPLEXITY_API_KEY")
        return

    models_demo = {
        "sonar": "General purpose with search",
        "sonar-reasoning": "Complex reasoning with search",
        "sonar-deep-research": "Comprehensive research",
        "pplx-7b-online": "Fast online inference",
        "pplx-7b-chat": "General chat without search",
    }

    print("\n[LOG]  Available Perplexity Models:")
    for model, description in models_demo.items():
        print(f"        {model}: {description}")

    print("\n[LOG]  To use a specific model, set PERPLEXITY_MODEL:")
    print("        export PERPLEXITY_MODEL='sonar-reasoning'")
    print("        python simple_task_perplexity.py")


if __name__ == "__main__":
    run_simple_task_perplexity()
    example_with_different_models()
