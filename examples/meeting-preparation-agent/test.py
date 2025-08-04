"""Testing the agent connections separately."""

import os

import graphbit
from graphbit import Node, Workflow

# from tools import search_news, search_web


api_key = os.getenv("OPENAI_API_KEY")
os.environ["OLLAMA_MODEL"] = "tinyllama"
model = os.getenv("OLLAMA_MODEL", "tinyllama")

# company_name = "Tesla"

# company_info = search_web(company_name)
# # industry_info = search_news(company_name)
# print(company_info)
# print("\n\n")

# company_research_agent = Node.agent(
#     name="company_research_agent",
#     prompt= (
#     "You are an assistant preparing for a business meeting."
#     "Gather all the key details about the following company for meeting preparation: "
#     f"Company Information: {company_info}"
#     "Please provide a comprehensive report suitable for a business meeting."
#     )
# )

# summarizer_agent = Node.agent(
#     name="summarizer_agent",
#     prompt= (
#     "Summarize the company research information from company_research_id for meeting preparation."
#     )
# )

# company_research_agent = Node.agent(
#     name="ip_threat_analysis",
#     prompt= (
#         "You've developed a software application with novel architecture and logic, but the patenting process "
#         "will take time. Analyze and outline the key intellectual property threats during the pre-patent period, "
#         "including unauthorized use, reverse engineering, and replication."
#     )
# )

# summarizer_agent = Node.agent(
#     name="summarizer_agent",
#     prompt= (
#         "Provide a comprehensive summary on the analysis. "
#     )
# )

company_research_agent = Node.agent(
    name="company_research_agent", prompt=("You are an assistant preparing for a business meeting. " "Analyze the company information." "Company Information: Tesla"), agent_id="company_research_agent"
)

summarizer_agent = Node.agent(name="summarizer_agent", prompt=("Summarize the information of the company."), agent_id="summarizer_agent")

llm_config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4o")
executor = graphbit.Executor.new_memory_optimized(llm_config, timeout_seconds=300)


workflow = Workflow("Meeting Preparation Workflow")

company_research_id = workflow.add_node(company_research_agent)
summarizer_id = workflow.add_node(summarizer_agent)

workflow.connect(company_research_id, summarizer_id)

print("Connected nodes successfully")

workflow.validate()

result = executor.execute(workflow)

print(f"Workflow execution completed. Success: {result.is_success()}")
print(f"Result state: {result.state()}")

if result.is_success():
    print("\nAll variables: \n", result.get_all_variables())
