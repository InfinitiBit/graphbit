"""Showcase: Personal Financial Advisor — tool-based balance intake, then condition routing.

Summary
-------
1. **FetchBalance** — An agent calls ``get_financial_details()`` (stub tool). The reply must
   include a parseable **ROUTING_BALANCE:** line (and typically echoes JSON with ``balance``).
2. **RouteByBalance** — Parses the balance and returns the next node's **name**:
   under **$1,000** → Budget Planner; **$1,000–$50,000** → Small Investment Advisor;
   above **$50,000** → Large Investment Advisor.
3. Exactly **one** of the three advisor agents runs. Each has a different mindset, tone, and
   **distinct tools** (expenses/bills vs market/ETFs vs portfolio/tax).

**Skipped branches** are not scheduled and **never execute** (no LLM call, no ``node_outputs``).

**Condition timing:** the route handler runs when the fetch node completes in the Rust executor.
If the LLM chose tools, that completion is a ``tool_calls_required`` blob (tools run afterward in
Python), so routing uses the same balance as ``get_financial_details()`` — ``DEMO_ACCOUNT_BALANCE``.
The printed fetch transcript may show the **post-tool** reply (different wording); branch choice still
tracks the demo knob unless the parent output is plain text/JSON containing ``balance``.

The stub tool ``get_financial_details()`` uses **`DEMO_ACCOUNT_BALANCE`** at the top of this file
(change it to try Budget vs Small vs Large tiers).

**LLM backend:** loads ``.env`` from the repo root (and cwd). If ``OPENAI_API_KEY`` is set,
uses OpenAI (optional ``OPENAI_MODEL``, default ``gpt-4o-mini``); otherwise Ollama
(``OLLAMA_MODEL``, default ``llama3.2``). Tool calling is more reliable on OpenAI.

On success the script prints **only** the output of the five workflow nodes (by name), in order.

If every advisor branch appears to run, reinstall GraphBit from this repo (e.g. ``maturin develop`` in ``python/``) and ensure ``route_by_balance`` returns exactly ``NODE_BUDGET`` / ``NODE_SMALL`` / ``NODE_LARGE``.

Run from repo root::

    python examples/tasks_examples/conditional_branch_local_model.py
"""

from __future__ import annotations

import json
import os
import re
import warnings
from pathlib import Path

from dotenv import load_dotenv

from graphbit import Executor, LlmConfig, Node, Workflow, tool


# ---------------------------------------------------------------------------
# Workflow node names (must match handler return values and graph validation)
# ---------------------------------------------------------------------------
NODE_FETCH = "FetchBalance"
NODE_COND = "RouteByBalance"
NODE_BUDGET = "BudgetPlanner"
NODE_SMALL = "SmallInvestmentAdvisor"
NODE_LARGE = "LargeInvestmentAdvisor"

ALL_NODES_IN_ORDER = (
    NODE_FETCH,
    NODE_COND,
    NODE_BUDGET,
    NODE_SMALL,
    NODE_LARGE,
)

USER_FINANCIAL_ASK = (
    "I'm trying to get my finances in order and want personalized advice "
    "based on where I stand today. What should I focus on first?"
)

# Stub balance returned by get_financial_details() — edit to exercise different condition branches.
DEMO_ACCOUNT_BALANCE = 847.50

# --- Intake tool ------------------------------------------------------------


@tool(
    _description=(
        "Fetch a snapshot of the user's account. Returns JSON with a numeric balance field, "
        "e.g. {\"balance\": 847.50}. Call once when starting intake."
    )
)
def get_financial_details() -> str:
    """Stub: simulated account balance (see DEMO_ACCOUNT_BALANCE)."""
    bal = DEMO_ACCOUNT_BALANCE
    payload = {"balance": round(bal, 2)}
    return json.dumps(payload)


# --- Budget Planner tools ---------------------------------------------------


@tool(
    _description=(
        "Return high-level spending categories and approximate monthly totals (USD) "
        "for budgeting."
    )
)
def get_expense_categories() -> str:
    """Stub expense breakdown."""
    data = {
        "categories": [
            {"name": "Housing", "monthly_usd": 980.0},
            {"name": "Food", "monthly_usd": 420.0},
            {"name": "Transport", "monthly_usd": 190.0},
            {"name": "Subscriptions", "monthly_usd": 85.0},
        ],
        "note": "Illustrative demo data — not real user transactions.",
    }
    return json.dumps(data)


@tool(
    _description=(
        "List upcoming bills with due dates and amounts to help avoid overdrafts "
        "and late fees."
    )
)
def get_bill_due_dates() -> str:
    """Stub bills."""
    data = {
        "bills": [
            {"name": "Rent", "due": "1st", "amount_usd": 950.0},
            {"name": "Utilities", "due": "12th", "amount_usd": 95.0},
            {"name": "Phone", "due": "18th", "amount_usd": 55.0},
        ]
    }
    return json.dumps(data)


# --- Small Investment Advisor tools ----------------------------------------


@tool(
    _description="Return a short snapshot of broad U.S. market indexes (not investment advice)."
)
def get_market_index_summary() -> str:
    data = {
        "SPY": {"label": "S&P 500 proxy", "note": "Broad large-cap U.S. equity"},
        "QQQ": {"label": "Nasdaq-100 proxy", "note": "Tech-heavy growth tilt"},
        "disclaimer": "Hypothetical demo snapshot for education only.",
    }
    return json.dumps(data)


@tool(
    _description=(
        "List example low-cost, diversified index ETFs often used by beginners "
        "(symbols only; not a recommendation)."
    )
)
def get_low_risk_funds() -> str:
    data = {
        "examples": [
            {"symbol": "VTI", "idea": "Total U.S. stock market"},
            {"symbol": "VXUS", "idea": "International stocks"},
            {"symbol": "BND", "idea": "U.S. aggregate bonds"},
        ],
        "disclaimer": "Educational examples only — verify fees and suitability yourself.",
    }
    return json.dumps(data)


# --- Large Investment Advisor tools -----------------------------------------


@tool(
    _description=(
        "Describe diversified portfolio strategy labels (aggressive, balanced, conservative) "
        "with example stock/bond splits — illustrative."
    )
)
def get_portfolio_options() -> str:
    data = {
        "strategies": [
            {"name": "Conservative", "stocks_pct": 40, "bonds_pct": 60},
            {"name": "Balanced", "stocks_pct": 60, "bonds_pct": 40},
            {"name": "Aggressive", "stocks_pct": 85, "bonds_pct": 15},
        ],
        "note": "Illustrative only — not personalized advice.",
    }
    return json.dumps(data)


@tool(
    _description=(
        "Summarize common tax-advantaged account types (401k, IRA, Roth concepts) "
        "in plain language."
    )
)
def get_tax_saving_instruments() -> str:
    data = {
        "accounts": [
            {"type": "401(k)", "idea": "Employer plan; pre-tax or Roth options often exist."},
            {"type": "Traditional IRA", "idea": "Tax-deferred growth; rules on deductions."},
            {"type": "Roth IRA", "idea": "After-tax contributions; qualified withdrawals tax-free."},
        ],
        "disclaimer": "Educational overview — consult a tax pro for your situation.",
    }
    return json.dumps(data)


# ---------------------------------------------------------------------------
# Prompts
# ---------------------------------------------------------------------------

SYSTEM_FETCH = (
    "You are a financial intake assistant. Call get_financial_details exactly once. "
    "Then reply with a single short sentence acknowledging the user, followed by a blank line, "
    "then a line EXACTLY of the form: ROUTING_BALANCE: <number> "
    "where <number> is IDENTICAL to the numeric \"balance\" in the tool JSON (copy it exactly; "
    "do not use example amounts like 847.5 unless that is what the tool returned). "
    "Do not invent a balance — use only the tool result."
)

PROMPT_FETCH = USER_FINANCIAL_ASK

SYSTEM_BUDGET = """\
You are a Budget Planner for users with limited cushion.

Tone: Empathetic, practical, no-nonsense.
Mindset: "Let's stabilize before we grow."

Use your tools when they help ground the plan. Then produce: a strict weekly spending plan, \
what to trim first, and a savings target with the first milestone being a $1,000 emergency fund."""

SYSTEM_SMALL = """\
You are a Small Investment Advisor for users who have a foundation but are still building.

Tone: Encouraging, educational, growth-focused.
Mindset: "You have a foundation, let's grow it carefully."

Use your tools when useful. Then propose: how to split between cash reserves and low-risk, \
diversified investing, and explain risk in simple terms."""

SYSTEM_LARGE = """\
You are a Large Investment Advisor for users with meaningful assets.

Tone: Professional, strategic, opportunity-focused.
Mindset: "Let's make this money work seriously."

Use your tools when useful. Then outline: diversified wealth-building across stocks, bonds, \
tax-advantaged accounts, and liquid reserves — structured and specific."""

# {{node.FetchBalance}} is resolved by GraphBit from workflow context (intake output).
# Each tier gets a different user/task prompt so the model is steered to call the tools it actually has.

_ADVISOR_CONTEXT_BLOCK = (
    "The user said:\n'''"
    + USER_FINANCIAL_ASK
    + "'''\n\n"
    "Intake / account context (from FetchBalance node output):\n"
    "'''\n{{node.FetchBalance}}\n'''\n\n"
)

PROMPT_BUDGET_ADVISOR = _ADVISOR_CONTEXT_BLOCK + (
    "You were routed here because the balance is under $1,000 — stabilization first.\n\n"
    "You MUST call both tools you have, in any order, before your final answer:\n"
    "1) get_expense_categories — use the category breakdown to name where money goes.\n"
    "2) get_bill_due_dates — use upcoming bills to avoid overdrafts and late fees.\n\n"
    "Then deliver: a strict weekly spending plan, what to cut first, and a savings path to a "
    "$1,000 emergency fund. Use the tool data explicitly; do not invent account details."
)

PROMPT_SMALL_ADVISOR = _ADVISOR_CONTEXT_BLOCK + (
    "You were routed here because the balance is in the building-wealth band ($1,000–$50,000).\n\n"
    "You MUST call both tools you have, in any order, before your final answer:\n"
    "1) get_market_index_summary — ground your education in the index snapshot (brief).\n"
    "2) get_low_risk_funds — use the example ETFs/funds list to discuss beginner diversification.\n\n"
    "Then deliver: a beginner investment plan, how to split cash vs low-risk diversified investing, "
    "and a plain-English risk explanation. Use the tool outputs explicitly."
)

PROMPT_LARGE_ADVISOR = _ADVISOR_CONTEXT_BLOCK + (
    "You were routed here because the balance is above $50,000 — strategic wealth planning.\n\n"
    "You MUST call both tools you have, in any order, before your final answer:\n"
    "1) get_portfolio_options — use aggressive/balanced/conservative examples for allocation framing.\n"
    "2) get_tax_saving_instruments — weave in 401(k), IRA, and Roth concepts from the tool output.\n\n"
    "Then deliver: a diversified plan across stocks, bonds, tax-advantaged accounts, and liquid reserves. "
    "Use the tool outputs explicitly; stay professional and structured."
)

def load_example_env() -> None:
    """Load repo ``.env`` then cwd (same pattern as other GraphBit examples)."""
    repo_root = Path(__file__).resolve().parents[2]
    load_dotenv(repo_root / ".env")
    load_dotenv()


def build_llm_config() -> LlmConfig:
    """Prefer OpenAI when ``OPENAI_API_KEY`` is set; otherwise Ollama."""
    openai_api_key = (os.getenv("OPENAI_API_KEY") or "").strip()
    if openai_api_key:
        model = (os.getenv("OPENAI_MODEL") or "gpt-4o-mini").strip()
        return LlmConfig.openai(openai_api_key, model)

    os.environ.setdefault("OLLAMA_MODEL", "llama3.2")
    ollama_model = os.getenv("OLLAMA_MODEL", "llama3.2")
    return LlmConfig.ollama(ollama_model)


def parent_output_text(routing: dict) -> str:
    """
    Normalize ``routing["parent_output"]`` to text for ``parse_balance_from_fetch_output``.

    Matches core stringification: raw ``str`` from the parent node, otherwise JSON text.
    """
    po = routing["parent_output"]
    if isinstance(po, str):
        return po
    return json.dumps(po)


def _deep_balance(obj: object) -> float | None:
    """Return first numeric ``balance`` field found in nested dict/list structures."""
    if isinstance(obj, dict):
        raw = obj.get("balance")
        if isinstance(raw, (int, float)):
            return float(raw)
        if isinstance(raw, str):
            try:
                return float(raw)
            except ValueError:
                pass
        for v in obj.values():
            found = _deep_balance(v)
            if found is not None:
                return found
    elif isinstance(obj, list):
        for item in obj:
            found = _deep_balance(item)
            if found is not None:
                return found
    return None


def parse_balance_from_fetch_output(fetched: str) -> float:
    """
    Extract balance for routing.

    Prefer **JSON ``"balance"``** (what ``get_financial_details`` returns) over
    ``ROUTING_BALANCE:`` from the LLM — models often echo a stale or example amount in
    ``ROUTING_BALANCE`` while the tool output has the real ``DEMO_ACCOUNT_BALANCE``.
    If multiple ``"balance"`` numbers appear, use the **last** (latest tool result).

    If nothing matches (common with Ollama when the final reply omits tool JSON), fall back to
    ``DEMO_ACCOUNT_BALANCE`` so this demo still branches off the knob at the top of the file;
    OpenAI-style runs usually include parseable tool output in the trace.
    """
    text = fetched.strip()
    json_balances = re.findall(
        r'"balance"\s*:\s*([0-9]+(?:\.[0-9]+)?)',
        text,
    )
    if json_balances:
        return float(json_balances[-1])
    m = re.search(
        r'ROUTING_BALANCE:\s*([0-9]+(?:\.[0-9]+)?)',
        text,
        re.IGNORECASE,
    )
    if m:
        return float(m.group(1))
    # Demo fallback: weak tool callers may not paste tool JSON into the agent's final message.
    snippet = text.replace("\n", " ")[:240]
    warnings.warn(
        f"{NODE_COND}: no JSON \"balance\" or ROUTING_BALANCE in FetchBalance text; "
        f"using DEMO_ACCOUNT_BALANCE={DEMO_ACCOUNT_BALANCE}. Snippet: {snippet!r}",
        UserWarning,
        stacklevel=2,
    )
    return float(DEMO_ACCOUNT_BALANCE)


def balance_for_routing(routing: dict) -> float:
    """
    Balance used for branch choice.

    FetchBalance often finishes the Rust workflow step with ``type: tool_calls_required`` (tools are
    executed later in ``handle_tool_calls_in_context``). That payload has no ``balance`` field yet, so
    we use the same stub as ``get_financial_details()`` — keep ``DEMO_ACCOUNT_BALANCE`` as the single
    source of truth. When the parent output is final text/JSON, parse as before.
    """
    po = routing["parent_output"]
    if isinstance(po, dict) and po.get("type") == "tool_calls_required":
        return float(json.loads(get_financial_details())["balance"])
    if isinstance(po, dict):
        found = _deep_balance(po)
        if found is not None:
            return found
    return parse_balance_from_fetch_output(parent_output_text(routing))


def route_by_balance(routing: dict) -> str:
    """Map parsed balance to the advisor node ``name`` (exact string for condition routing).

    ``routing`` is the snapshot dict: ``parent_output``, ``parent_node_id``, ``variables``,
    ``node_outputs``, ``metadata`` — use any field for richer routing.
    """
    bal = balance_for_routing(routing)

    if bal < 1_000:
        return NODE_BUDGET
    if bal <= 50_000:
        return NODE_SMALL
    return NODE_LARGE


def main() -> None:
    load_example_env()

    llm_config = build_llm_config()
    executor = Executor(llm_config, lightweight_mode=True)

    node_fetch = Node.agent(
        name=NODE_FETCH,
        prompt=PROMPT_FETCH,
        system_prompt=SYSTEM_FETCH,
        tools=[get_financial_details],
        max_iterations=8,
    )
    node_cond = Node.condition(name=NODE_COND, handler=route_by_balance)
    node_budget = Node.agent(
        name=NODE_BUDGET,
        prompt=PROMPT_BUDGET_ADVISOR,
        system_prompt=SYSTEM_BUDGET,
        tools=[get_expense_categories, get_bill_due_dates],
        max_iterations=12,
    )
    node_small = Node.agent(
        name=NODE_SMALL,
        prompt=PROMPT_SMALL_ADVISOR,
        system_prompt=SYSTEM_SMALL,
        tools=[get_market_index_summary, get_low_risk_funds],
        max_iterations=12,
    )
    node_large = Node.agent(
        name=NODE_LARGE,
        prompt=PROMPT_LARGE_ADVISOR,
        system_prompt=SYSTEM_LARGE,
        tools=[get_portfolio_options, get_tax_saving_instruments],
        max_iterations=12,
    )

    workflow = Workflow("Personal Financial Advisor (conditional)")
    fid = workflow.add_node(node_fetch)
    cid = workflow.add_node(node_cond)
    bid = workflow.add_node(node_budget)
    sid = workflow.add_node(node_small)
    lid = workflow.add_node(node_large)

    workflow.connect(fid, cid)
    workflow.connect(cid, bid)
    workflow.connect(cid, sid)
    workflow.connect(cid, lid)
    workflow.validate()

    result = executor.execute(workflow)

    if result.is_failed():
        print(result.state())
        return

    for name in ALL_NODES_IN_ORDER:
        text = result.get_node_output(name)
        print(name)
        print("-" * len(name))
        print("(no output)" if text is None else text)
        print()


if __name__ == "__main__":
    main()
