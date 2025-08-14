"""The agents that will be used for the meeting preparation agent."""

import os

from tools import search_news, search_web

import graphbit
from graphbit import Node

company_name = "Tesla"

# Get company information
print("Getting company information...")
company_info = search_web(company_name)
print("Getting industry information...")
industry_info = search_news(company_name)

company_research_agent = Node.agent(
    name="company_research_agent",
    prompt=f"""
    You are an assistant preparing for a business meeting.
    Summarize the key details about the following company for meeting preparation:

    Company Information:
    {company_info}

    Please provide a comprehensive summary suitable for a business meeting.
    """,
    agent_id="company_research_agent",
)

industry_trend_agent = Node.agent(
    name="Industry Trend Agent",
    prompt=f"""
    You are an assistant preparing for a business meeting.
    Summarize the latest industry trends and challenges related to the following company.

    Industry News and Information:
    {industry_info}

    Please provide a comprehensive summary of key industry trends, recent developments, and challenges that are relevant for a business meeting.
    """,
    agent_id="industry_trend_agent",
)

# ip_threat_analysis = graphbit.Node.agent(
#     name="ip_threat_analysis", prompt=("You are an assistant preparing for a business meeting. " "Analyze the company information." "Company Information: Tesla"), agent_id="ip_threat_analysis"
# )

# summarizer_agent = graphbit.Node.agent(name="summarizer_agent", prompt=("Summarize the information of the company."), agent_id="summarizer_agent")

sales_strategy_agent = Node.agent(
    name="Sales Strategy Agent",
    prompt="""
    You are an assistant preparing for a business meeting.
    Based on the company and industry summaries, generate a concise and actionable sales strategy tailored for this meeting.

    Please provide:
    - Key talking points for the meeting
    - Specific recommended actions to increase the chances of a successful outcome

    Ensure your response is comprehensive, relevant, and suitable for a business meeting context.
    """,
    agent_id="sales_strategy_agent",
)


# Check if API key is available
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    print("Warning: OPENAI_API_KEY not set. Please set it to run the workflow.")
    print("You can set it with: export OPENAI_API_KEY=your_api_key_here")
    exit(1)

llm_config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4o")
executor = graphbit.Executor.new_memory_optimized(llm_config, timeout_seconds=300)

workflow = graphbit.Workflow("Meeting Preparation Workflow")

# Add all agent nodes to the workflow
company_research_id = workflow.add_node(company_research_agent)

# ip_threat_analysis_id = workflow.add_node(ip_threat_analysis)
industry_trend_id = workflow.add_node(industry_trend_agent)
sales_strategy_id = workflow.add_node(sales_strategy_agent)
# summarizer_id = workflow.add_node(summarizer_agent)

# Set up dependencies: sales_strategy_agent depends on both company_research_agent and industry_trend_agent
workflow.connect(company_research_id, sales_strategy_id)
workflow.connect(industry_trend_id, sales_strategy_id)

# workflow.connect(ip_threat_analysis_id, summarizer_id)

print("Connected nodes successfully")

workflow.validate()

# Execute the workflow
print("Executing workflow...")
result = executor.execute(workflow)

print(f"Workflow execution completed. Success: {result.is_success()}")
print(f"Result state: {result.state()}")

if result.is_success():
    # All variables
    print("\nAll variables: \n", result.get_all_variables())

    # Get the output from the sales strategy agent (final node)
    # output = result.get_variable("node_result_2")
    # print(f"\n\nSales strategy agent output:")
    # print(output)

    # Try to get outputs from the individual agents
    # company_summary = result.get_variable("node_result_3")
    # industry_summary = result.get_variable("node_result_1")

    # if output:
    #     # Clean up the output (remove escaped quotes and newlines)
    #     import json
    #     try:
    #         # Try to parse as JSON to handle escaped characters
    #         cleaned_output = json.loads(output)
    #     except:
    #         # If not valid JSON, just use the raw output
    #         cleaned_output = output.replace('\\n', '\n').replace('\\"', '"')

    #     try:
    #         # Try to parse as JSON to handle escaped characters
    #         cleaned_company_summary_output = json.loads(company_summary)
    #     except:
    #         # If not valid JSON, just use the raw output
    #         cleaned_company_summary_output = output.replace('\\n', '\n').replace('\\"', '"')

    #     try:
    #         # Try to parse as JSON to handle escaped characters
    #         cleaned_industry_summary_output = json.loads(industry_summary)
    #     except:
    #         # If not valid JSON, just use the raw output
    #         cleaned_industry_summary_output = output.replace('\\n', '\n').replace('\\"', '"')

    #     # print("Sales Strategy:")
    #     # print(cleaned_output)

    #     # print("\n" + "="*50)
    #     # print("Company Research Summary:")
    #     # print(cleaned_company_summary_output)

    #     # print("\n" + "="*50)
    #     # print("Industry Trends Summary:")
    #     # print(cleaned_industry_summary_output)
    # else:
    #     print("No output available from sales_strategy_agent")
    #     # Try to get any available variables
    #     print("Available variables:")
    #     # Note: We might need to check what variables are actually available
else:
    print(f"Workflow failed: {result.state()}")
