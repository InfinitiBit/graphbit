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


@tool(_description="Sum two phone numbers digit-by-digit by position. e.g. 111-1111 and 222-2222 yield 333-3333.")
def sum_digits_in_phones(phone_number_1: str, phone_number_2: str) -> str:
    """
    Sum two phone numbers by adding digits at each position (same length).
    When GuardRail is enabled, this tool receives the DECODED (real) numbers so it can compute correctly.
    """
    def digits_only(s: str) -> list[int]:
        return [int(c) for c in s if c.isdigit()]
    d1 = digits_only(phone_number_1)
    d2 = digits_only(phone_number_2)
    n = max(len(d1), len(d2))
    d1 = [0] * (n - len(d1)) + d1
    d2 = [0] * (n - len(d2)) + d2
    summed = [a + b for a, b in zip(d1, d2)]
    # Format as XXX-XXXX (7 digits)
    s = "".join(str(d) for d in summed)
    result = f"{s[:3]}-{s[3:]}" if len(s) >= 7 else s
    print(
        f"[Tool received] phone_number_1 = {phone_number_1!r}, phone_number_2 = {phone_number_2!r} -> position-wise sum = {result}"
    )
    return result


def main():
    init(enable_tracing=True, log_level="debug")

    print(
        "GuardRail phone example: LLM should never see 111-1111 or 222-2222; tool should always receive them.\n"
    )

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
            "The user's phone numbers are 111-1111 and 222-2222. "
            "Use the sum_digits_in_phones tool to sum them digit-by-digit by position (result will be 333-3333). "
            "Then reply with the result."
        ),
        system_prompt="You have a tool to sum two phone numbers by position. Use it when asked.",
        tools=[sum_digits_in_phones],
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
        print(
            "\nVerify: above '[Tool received]' should show phone_number_1 = '111-1111', phone_number_2 = '222-2222' (decoded)."
        )
        print("In debug logs you should see 'Guardrail: encoding prompt before LLM' and 'decoding tool call parameters'.")
    else:
        print(f"Workflow failed: {result.state()}")
    print("\n--- Node Response Metadata (params and output should be masked) ---")
    print(result.get_all_node_response_metadata())
if __name__ == "__main__":
    main()
