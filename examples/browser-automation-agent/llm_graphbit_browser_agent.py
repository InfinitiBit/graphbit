"""Main entry point for Graphbit LLM-guided browser automation."""

import logging

import openai
from graphbit_agent_planner import get_plan_from_graphbit
from plan_executor import run_browser_plan

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("GraphbitBrowserAgent")


def main():
    """Prompt user, generate automation plan via Graphbit, and execute it."""
    user_prompt = input("Enter your query (e.g., 'What is the current price of Bitcoin?'): ")
    url = input("Enter the target website URL (e.g., https://www.binance.com/en/price/bitcoin): ")

    logger.info(f"Prompt: {user_prompt}")
    logger.info(f"URL: {url}")

    plan, selectors = get_plan_from_graphbit(user_prompt, url)
    logger.info(f"Action Plan: {plan}")

    if not plan:
        print(" LLM failed to generate a valid plan. Try a simpler site or question.")
        return

    result = run_browser_plan(plan, selectors)
    logger.info(f"Result: {result}")

    if result and isinstance(result, dict):
        all_text = "\n".join([t if isinstance(t, str) else "\n".join(t) for t in result.values()])
        summarizer_prompt = f"""
    User query: {user_prompt}
    Extracted texts:
    {all_text}

    From these texts, choose the one that best matches the user query.
    Return ONLY that text.
    """
        response = openai.chat.completions.create(
            model="gpt-4",
            messages=[{"role": "system", "content": "You are a text selector."}, {"role": "user", "content": summarizer_prompt}],
            temperature=0,
        )
        best_match = response.choices[0].message.content.strip()
        result = {"best_match": best_match}

    print("\n FINAL RESULT:", result)
    if not result:
        print(" No data extracted. See error logs or screenshots for details.")


if __name__ == "__main__":
    main()
