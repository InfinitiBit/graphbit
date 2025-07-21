import requests
import graphbit
from graphbit import Node, Workflow
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# Get API key and Search Engine ID from environment variables
GOOGLE_API_KEY = os.getenv("GOOGLE_API_KEY")
GOOGLE_CSE_ID = os.getenv("GOOGLE_CSE_ID")
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")

def google_search(query):
    url = "https://www.googleapis.com/customsearch/v1"
    params = {"key": GOOGLE_API_KEY, "cx": GOOGLE_CSE_ID, "q": query}
    response = requests.get(url, params=params)
    try:
        response.raise_for_status()
    except requests.exceptions.HTTPError:
        print("Error details:", response.text)
        raise
    return response.json()

def process_search_results(results, max_snippets=3):
    items = results.get("items", [])[:max_snippets]
    snippets = [
        f"{item['title']} ({item['link']}): {item['snippet']}"
        for item in items
    ]
    return "\n\n".join(snippets)

# Run Google Search outside the workflow
search_results = google_search("Infinitibit Limited")
#print(search_results)

# Process search results and extract snippets
snippets_text = process_search_results(search_results)
#print(snippets_text)

# Build workflow
agent = Node.agent(
    name="Summarizer",
    prompt=f"Summarize these search results: {snippets_text}"
)
workflow = Workflow("Google Search Workflow")
node_id = workflow.add_node(agent)

# Initialize Graphbit and LLM config
graphbit.init()
llm_config = graphbit.LlmConfig.openai(OPENAI_API_KEY)
executor = graphbit.Executor(llm_config)

# Run workflow with snippets as input
result = executor.execute(workflow)

# Print the result
if result.is_success():
    print("Summary:", result.get_variable("node_result_1"))
    #print("All variables:", result.get_all_variables())
    #print("Workflow state:", result.state())
else:
    print("Workflow failed:", result.state())


