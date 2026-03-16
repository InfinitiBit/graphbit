"""
GuardRail Financial Example: Secure Payment Processing with PII Masking

This example demonstrates how GuardRail masks sensitive financial information
(credit card numbers, emails, SSNs) from the LLM while ensuring tools receive
the actual decoded values for proper processing.

Pattern:
- LLM sees masked tokens (e.g., [CREDIT_CARD_1], [EMAIL_1])
- Tools receive the real unmasked values for accurate computation
- GuardRail automatically handles encode/decode boundaries
"""

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
POLICY_PATH = os.path.join(POLICY_DIR, "guardrail_financial_policy.json")


@tool(_description="Validate a credit card number and return the last 4 digits safely. Input is a full CC number in format XXXX-XXXX-XXXX-XXXX.")
def validate_credit_card(card_number: str) -> str:
    """
    Validate a credit card by checking digit count and return last 4 digits.
    When GuardRail is enabled, this tool receives the DECODED (real) card number.
    """
    digits_only = card_number.replace("-", "")
    is_valid = len(digits_only) == 16 and digits_only.isdigit()
    last_four = digits_only[-4:] if len(digits_only) >= 4 else "INVALID"
    
    print(f"[Tool received] card_number = {card_number!r}")
    print(f"[Tool validates] Valid: {is_valid}, Last 4 digits: {last_four}")
    
    return f"Card valid: {is_valid}, Last 4: {last_four}"


@tool(_description="Send a payment confirmation email. Recipient email must be in format user@domain.com")
def send_payment_confirmation(recipient_email: str, amount: str) -> str:
    """
    Send payment confirmation to recipient.
    When GuardRail is enabled, this tool receives the DECODED (real) email address.
    """
    print(f"[Tool received] recipient_email = {recipient_email!r}, amount = {amount!r}")
    
    # Simulate email validation
    if "@" in recipient_email and "." in recipient_email.split("@")[1]:
        result = f"Confirmation email sent to {recipient_email} for amount ${amount}"
        print(f"[Tool result] {result}")
        return result
    else:
        return "Error: Invalid email format"


@tool(_description="Calculate transaction fee based on amount. Amount should be a number like 100.50")
def calculate_transaction_fee(amount: str) -> str:
    """
    Calculate the transaction fee (2% of amount).
    """
    try:
        amt = float(amount)
        fee = amt * 0.02
        print(f"[Tool received] amount = {amount!r} -> fee = ${fee:.2f}")
        return f"Transaction fee: ${fee:.2f}"
    except ValueError:
        return "Error: Amount must be a number"


def main():
    init(enable_tracing=True, log_level="debug")

    print("=" * 80)
    print("GuardRail Financial Example: Secure Payment Processing")
    print("=" * 80)
    print("\nScenario: Process a payment with sensitive information")
    print("- LLM should NOT see real credit card, email, or SSN")
    print("- Tools MUST receive actual values for proper processing\n")

    # Prefer OpenAI for reliable tool calling; fallback to Ollama
    llm_config = LlmConfig.openai(os.getenv("OPENAI_API_KEY"), "gpt-4o-mini")
  
    executor = Executor(llm_config)

    # Load policy that masks credit cards, emails, and SSNs
    if not os.path.isfile(POLICY_PATH):
        print(f"Policy file not found: {POLICY_PATH}")
        sys.exit(1)
    
    policy = GuardRailPolicyConfig.from_file(POLICY_PATH)
    print(f"Loaded policy: {policy.policy_name()}, active={policy.is_active()}\n")

    workflow = Workflow("Secure Payment Processing [GuardRail]")
    
    payment_agent = Node.agent(
        name="Payment Processor",
        prompt=(
            "I want to send money using my bank with this credit card and sender info:\n"
            "Credit Card: 4532-1234-5678-9010\n"
            "Recipient Email: customer@example.com\n"
            "Amount: 250.00\n"
            "Customer SSN (for verification): 123-45-6789\n"
           
        ),
        system_prompt=(
            "You are a secure payment processor. Use the available tools to validate "
            "payments, calculate fees, and send confirmations. Always use the tools "
            "with the exact information provided."
        ),
        tools=[validate_credit_card, calculate_transaction_fee, send_payment_confirmation],
    )
    
    workflow.add_node(payment_agent)
    workflow.validate()

    # Execute WITH policy: LLM sees masked data; tools receive decoded data
    print("Executing workflow with GuardRail policy...\n")
    result = executor.execute(workflow, policy=policy)

    print("\n" + "=" * 80)
    print("--- Result ---")
    print("=" * 80)
    
    if result.is_success():
        out = result.get_node_output("Payment Processor")
        print(f"\nAgent Output:\n{out}")
        print("\n" + "-" * 80)
        print("Verification Notes:")
        print("- Above '[Tool received]' entries should show REAL values (unmasked)")
        print("  - Credit card: 4532-1234-5678-9010")
        print("  - Email: customer@example.com")
        print("  - SSN: 123-45-6789")
        print("\n- In debug logs you should see:")
        print("  - 'Guardrail: encoding prompt before LLM'")
        print("  - 'Guardrail: decoding tool call parameters before execution'")
        print("  - Masked tokens like [CREDIT_CARD_1], [EMAIL_1], [SSN_1]")
        print("-" * 80)
    else:
        print(f"Workflow failed: {result.state()}")


if __name__ == "__main__":
    main()
