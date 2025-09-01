import os
import re
import requests
from dotenv import load_dotenv
import graphbit
from graphbit import Node, Workflow

# Load API keys
load_dotenv()
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
GOOGLE_CSE_ID = os.getenv("GOOGLE_CSE_ID")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

# Step 1: Google Search
def google_search(query, num_results=5):
    url = "https://www.googleapis.com/customsearch/v1"
    params = {"key": GOOGLE_API_KEY, "cx": GOOGLE_CSE_ID, "q": query}
    response = requests.get(url, params=params)
    response.raise_for_status()
    return response.json().get("items", [])[:num_results]

# Step 2: Extract page content & emails
def fetch_emails_from_url(url):
    try:
        html = requests.get(url, timeout=5).text
        emails = re.findall(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", html)
        return list(set(emails))  # unique
    except Exception:
        return []

# Step 3: Build Graphbit Agent
def marketing_agent(query):
    results = google_search(query)
    all_emails = []
    for item in results:
        url = item["link"]
        emails = fetch_emails_from_url(url)
        all_emails.extend(emails)
    all_emails = list(set(all_emails))

    # Graphbit summarizer for context (optional)
    agent = Node.agent(
        name="EmailSummary",
        prompt=f"Emails found for query '{query}': {all_emails}"
    )
    workflow = Workflow("Marketing Agent Workflow")
    workflow.add_node(agent)

    graphbit.init()
    executor = graphbit.Executor(graphbit.LlmConfig.openai(OPENAI_API_KEY))
    result = executor.execute(workflow)
    if result.is_success():
        return {"emails": all_emails, "summary": result.get_variable("node_result_1")}
    return {"emails": all_emails, "summary": "No summary"}

if __name__ == "__main__":
    query = "Find emails of digital marketing agencies in London"
    output = marketing_agent(query)
    print("Extracted Emails:", output["emails"])
    print("AI Summary:", output["summary"])
