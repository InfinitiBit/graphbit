
"""Example Graphbit  Playwright agent that retrieves the current Bitcoin price."""

import json
import os
import uuid
from dataclasses import dataclass

import graphbit
from playwright.sync_api import sync_playwright

# Prompt for Graphbit agent to generate browsing instructions
BROWSER_PROMPT = (
    "You are a planning agent for web automation. "
    "Given the question: '{question}', provide a JSON object "
    "with the URL to visit and a CSS selector for the element containing the answer."
)

@dataclass
class Plan:
    url: str
    selector: str


def extract_output(result: graphbit.WorkflowResult, agent_id: str, name: str) -> str:
    """Utility to extract agent output string from workflow result."""
    variables = result.variables()
    for key, value in variables:
        if key == agent_id or key == name:
            return str(value)
    # Fallback to the first variable
    if variables:
        return str(variables[0][1])
    return ""


def generate_plan(question: str) -> Plan:
    """Run Graphbit to produce a browsing plan."""
    graphbit.init()
    model = os.getenv("OLLAMA_MODEL", "llama3.2")
    llm_config = graphbit.LlmConfig.ollama(model)
    executor = graphbit.Executor.new_low_latency(llm_config)

    agent_id = str(uuid.uuid4())
    workflow = graphbit.Workflow("Browser Plan")
    node = graphbit.Node.agent(
        name="planner",
        prompt=BROWSER_PROMPT.format(question=question),
        agent_id=agent_id,
    )
    workflow.add_node(node)
    workflow.validate()

    result = executor.execute(workflow)
    if result.is_failed():
        raise RuntimeError(f"Workflow failed: {result.state()}")

    output = extract_output(result, agent_id, "planner")
    data = json.loads(output)
    return Plan(url=data["url"], selector=data["selector"])


def fetch_price(plan: Plan) -> str:
    """Use Playwright to execute the browsing plan and return extracted text."""
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        page.goto(plan.url)
        element = page.wait_for_selector(plan.selector)
        text = element.inner_text()
        browser.close()
    return text


def run(question: str) -> None:
    """Entry point to run the browser automation agent."""
    plan = generate_plan(question)
    result = fetch_price(plan)
    print(f"Answer: {result}")


if __name__ == "__main__":
    run("What is the current price of Bitcoin?")
