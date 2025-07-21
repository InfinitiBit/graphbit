import logging
from graphbit_agent_planner import get_plan_from_graphbit
from plan_executor import run_browser_plan

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("GraphbitBrowserAgent")

def main():
    user_prompt = "What is the current price of Bitcoin?"
    logger.info(f"Prompt: {user_prompt}")

    # 1️ Generate the action plan using Graphbit+LLM
    plan = get_plan_from_graphbit(user_prompt)
    logger.info(f"Action Plan: {plan}")

    # 2️ Execute the plan in the browser
    result = run_browser_plan(plan)
    logger.info(f"Result: {result}")

    print("\n FINAL RESULT:", result)

if __name__ == "__main__":
    main()
