import os
import sys

import graphbit
from graphbit import (
    Executor,
    GuardRailPolicyConfig,
    LlmConfig,
    Node,
    Workflow,
    tool,
    init,
)

# using our policy
POLICY_DIR = os.path.dirname(os.path.abspath(__file__))
POLICY_PATH = os.path.join(POLICY_DIR, "guardrail_phone_policy.json")


@tool(_description="Sum all digits in a phone number. Input is a string that may look like 555-5555 or a token.")
def sum_digits_in_phone(phone_number: str) -> str:
    """
    Compute the sum of all digits in the given phone number.
    When GuardRail is enabled, this tool receives the DECODED (real) number so it can compute correctly.
    """
    digits = [int(c) for c in phone_number if c.isdigit()]
    total = sum(digits)
    print(f"[Tool received] phone_number = {phone_number!r} -> sum of digits = {total}")
    return str(total)


def main():
    init(enable_tracing=True, log_level="debug")

    print("GuardRail phone example: LLM should never see 123-4567; tool should always receive it.\n")

    # Prefer OpenAI for reliable tool calling; fallback to Ollama
    if os.getenv("OPENAI_API_KEY"):
        llm_config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-5")
    else:
        print("No OPENAI_API_KEY set. Using Ollama (ollama run llama3.2).")
        llm_config = LlmConfig.ollama("llama3.2")

    executor = Executor(llm_config)

    # Load policy that masks 123-4567-style numbers
    if not os.path.isfile(POLICY_PATH):
        print(f"Policy file not found: {POLICY_PATH}")
        sys.exit(1)
    policy = GuardRailPolicyConfig.from_file(POLICY_PATH)
    print(f"Loaded policy: {policy.policy_name()}, active={policy.is_active()}\n")

    workflow = Workflow("Phone digits sum [GuardRail]")
    agent = Node.agent(
        name="Phone Agent",
        prompt=(
            "The user's phone number is 123-4567. "
            "Use the sum_digits_in_phone tool to compute the sum of all digits in that phone number, "
            "then reply with the result."
        ),
        system_prompt="You have a tool to sum digits in a phone number. Use it when asked.",
        tools=[sum_digits_in_phone],
        max_tokens=1000,
    )
    workflow.add_node(agent)
    workflow.validate()

    # Execute WITH policy: LLM sees masked data; tool receives decoded data
    result = executor.execute(workflow, policy=policy)

    print("\n--- Result ---")

    if result.is_success():
        out = result.get_node_output("Phone Agent")
        print(f"Agent output: {out}")
        print("\nVerify: above '[Tool received]' should show phone_number = '123-4567' (decoded).")
        print("In debug logs you should see 'Guardrail: encoding prompt before LLM' and 'decoding tool call parameters'.")
    else:
        print(f"Workflow failed: {result.state()}")
    print("\n--- Node Response Metadata ---")
    print(result.get_all_node_response_metadata())
if __name__ == "__main__":
    main()
