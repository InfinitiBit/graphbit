import logging

from graphbit_agent_planner import get_plan_from_graphbit
from plan_executor import run_browser_plan

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("GraphbitBrowserAgent")


def main():
    user_prompt = input("Enter your query (e.g., 'What is the current price of Bitcoin?'): ")
    url = input("Enter the target website URL (e.g., https://www.coindesk.com/price/bitcoin): ")

    logger.info(f"Prompt: {user_prompt}")
    logger.info(f"URL: {url}")

    plan, selectors = get_plan_from_graphbit(user_prompt, url)
    logger.info(f"Action Plan: {plan}")

    if not plan:
        print("❌ LLM failed to generate a valid plan. Try a simpler site or question.")
        return

    result = run_browser_plan(plan, selectors)
    logger.info(f"Result: {result}")

    print("\n✅ FINAL RESULT:", result)
    if not result:
        print("⚠️ No data extracted. See error logs or screenshots for details.")


if __name__ == "__main__":
    main()
