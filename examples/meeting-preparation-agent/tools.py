"""The tools that will be used for the meeting preparation agent."""

import os

import requests

# import asyncio
from dotenv import load_dotenv

from graphbit import LlmClient as gb_lct
from graphbit import LlmConfig as gb_lcg

load_dotenv()

OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
GOOGLE_CSE_ID = os.getenv("GOOGLE_CSE_ID")
# NEWSAPI_KEY = os.getenv("NEWSAPI_KEY")


# -- 1. Web Search Tool using Google Custom Search API --
# https://www.googleapis.com/customsearch/v1
def search_web(query):
    """Search web for results."""
    url = "https://www.googleapis.com/customsearch/v1"
    params = {
        "key": GOOGLE_API_KEY,
        "cx": GOOGLE_CSE_ID,
        "q": query,
        "num": 5,
    }
    response = requests.get(url, params=params)
    data = response.json()
    items = data.get("items", [])
    results = [f"{item['title']}: {item.get('snippet', '')}" for item in items]
    return "\n".join(results) if results else "No web results found."


# -- 2. News/Trend Tool using NewsAPI --
# https://newsapi.org/v2/everything
def search_news(company_name):
    """Search news for results."""
    url = "https://newsapi.org/v2/everything"
    params = {"q": company_name, "apiKey": os.getenv("NEWSAPI_KEY"), "pageSize": 5, "language": "en"}
    response = requests.get(url, params=params)
    data = response.json()
    if "articles" in data:
        articles = [f"{a['title']}: {a['description']}" for a in data["articles"] if a.get("description")]
        return "\n".join(articles)
    return "No news articles found."


# -- 3. LLM Summariser Tool using OpenAI --
async def llm_summarise(text, prompt="Summarize the following for a business meeting:"):
    """Summarise the text for a business meeting."""
    config = gb_lcg.openai(api_key=OPENAI_API_KEY, model="gpt-4o")

    llm_client = gb_lct(config=config)

    messages = [("system", "You are a business analyst."), ("user", f"{prompt}\n\n{text}")]

    result = await llm_client.chat_optimized(messages, max_tokens=400, temperature=0.4)
    return result.strip()


# print("Testing web search for 'Tesla':")
# print("-" * 50)
# print(search_web("Tesla"))

# print("\nTesting LLM summariser for 'Tesla, electric vehicles, and innovation':")
# print("-" * 50)
# test_text = (
#     "Tesla, Inc. is an American electric vehicle and clean energy company based in Austin, Texas. "
#     "Tesla designs and manufactures electric vehicles, battery energy storage, solar panels, and other clean energy products. "
#     "It is known for its innovation in battery technology and its mission to accelerate the world's transition to sustainable energy."
# )
# print(asyncio.run(llm_summarise(test_text)))

# print("\nTesting news search for 'Tesla':")
# print("-" * 50)
# print(search_news("Tesla"))
