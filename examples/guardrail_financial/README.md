# GuardRail Examples

This directory contains examples demonstrating GraphBit's GuardRail feature for protecting Personally Identifiable Information (PII) in LLM workflows.

## How GuardRail Works

GuardRail provides intelligent masking of sensitive data:
- **LLM sees**: Masked tokens (e.g., `[CREDIT_CARD_1]`, `[EMAIL_1]`)
- **Tools receive**: Real unmasked values for accurate processing
- **Automatic handling**: Encode/decode at LLM and tool boundaries

## Examples

### 1. Phone Number Sum (`guardrail_phone/`)

**Purpose**: Demonstrate GuardRail protecting phone numbers while allowing tools to process them correctly.

**Pattern**:
- Policy masks phone numbers matching pattern `\d{3}-\d{4}` (e.g., `123-4567`)
- Tool `sum_digits_in_phone()` receives the real unmasked number
- LLM only sees masked token (e.g., `[PHONE_NUMBER_1]`)

**Files**:
- `guardrail_phone_policy.json` - Policy defining phone number masking rules
- `run_guardrail_phone.py` - Example workflow with tool calling

**Run**:
```bash
.venv/bin/python examples/guardrail_phone/run_guardrail_phone.py
```

### 2. Financial Payment Processing (`guardrail_financial/`)

**Purpose**: Demonstrate GuardRail protecting multiple types of sensitive financial data (credit cards, emails, SSNs) in a payment processing workflow.

**Pattern**:
- Policy masks:
  - Credit cards: `\d{4}-\d{4}-\d{4}-\d{4}` (e.g., `4532-1234-5678-9010`)
  - Emails: Standard email pattern (e.g., `customer@example.com`)
  - SSNs: `\d{3}-\d{2}-\d{4}` (e.g., `123-45-6789`)
- Three tools with different data requirements:
  - `validate_credit_card()` - Validates the card and returns last 4 digits
  - `calculate_transaction_fee()` - Computes 2% fee on amount
  - `send_payment_confirmation()` - Sends confirmation to recipient email
- LLM sees only masked tokens, tools get real values

**Files**:
- `guardrail_financial_policy.json` - Policy defining credit card, email, and SSN masking
- `run_guardrail_financial.py` - Complete payment processing workflow

**Run**:
```bash
.venv/bin/python examples/guardrail_financial/run_guardrail_financial.py
```

## Key Implementation Details

### Policy Definition (JSON)

```json
{
  "policy_name": "example_policy",
  "policy_version": "1.0.0",
  "active": true,
  "guardrail_policy": {
    "pii_rules": [
      {
        "type": "regex",
        "name": "RULE_NAME",
        "pattern": "regex_pattern_here"
      }
    ],
    "masking": true,
    "mapping": true
  }
}
```

### Tool Definition

```python
from graphbit import tool

@tool(_description="Description shown to LLM")
def my_tool(param: str) -> str:
    """
    When GuardRail is enabled, this tool receives DECODED values.
    """
    print(f"[Tool received] param = {param!r}")
    # Process the real unmasked value
    return result
```

### Workflow Execution with GuardRail

```python
from graphbit import Executor, GuardRailPolicyConfig

# Load policy
policy = GuardRailPolicyConfig.from_file("policy.json")

# Execute with policy - LLM gets masked, tools get real data
result = executor.execute(workflow, policy=policy)
```

## Testing Without OpenAI

These examples use OpenAI (GPT-4o-mini) if `OPENAI_API_KEY` is set, otherwise fallback to Ollama.

**Using Ollama**:
```bash
# Ensure Ollama is running
ollama pull llama3.2
ollama serve

# In another terminal
.venv/bin/python examples/guardrail_financial/run_guardrail_financial.py
```

**Using OpenAI**:
```bash
export OPENAI_API_KEY="sk-..."
.venv/bin/python examples/guardrail_financial/run_guardrail_financial.py
```

## Debug Output

Run with debug logging to see GuardRail in action:
- `Guardrail: encoding prompt before LLM` - Shows data being masked
- `Guardrail: decoding tool call parameters` - Shows unmasking at tool boundary
- `[Tool received]` - Shows the real unmasked values tools receive

Look for masked tokens in logs like:
- `<CREDIT_CARD:BKDKVA>`
- `<EMAIL:6M4NMS>`
- `<SSN:RYJX13>`

## Creating Your Own Example

Follow this pattern:

1. **Define a policy** (JSON with PII patterns)
2. **Create tools** (Python functions marked with `@tool`)
3. **Build workflow** (Node.agent with tools parameter)
4. **Execute with policy** (executor.execute(workflow, policy=policy))
5. **Verify** (check tool received real values in debug output)
