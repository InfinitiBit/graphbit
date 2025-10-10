import os

from graphbit import Executor, LlmConfig, Node, Workflow, tool


@tool(_description="Get current weather information for any city")
def get_weather(location: str) -> dict:
    return {"location": location, "temperature": 25, "condition": "cloudy"}


@tool(_description="Perform mathematical calculations and return results")
def calculate(expression: str) -> str:
    return f"Result: {eval(expression)}"


def main(model: str):
    agent_id = model
    api_key = os.getenv("MISTRALAI_API_KEY")
    llm_config = LlmConfig.mistralai(api_key, model)
    # llm_client = LlmClient(llm_config)
    executor = Executor(llm_config)

    workflow = Workflow(f"Tool Test: {agent_id}")
    agent = Node.agent(
        name=agent_id,
        system_prompt="Use the tools I have provided. to answer my queries.",
        prompt="What's the weather of paris and what is 2324+2342? use the provided tools and provide the exact output without modifying the value.",
        agent_id=agent_id,
        tools=[get_weather, calculate],
    )
    workflow.add_node(agent)
    workflow.validate()

    result = executor.execute(workflow)
    content = result.get_node_output(agent_id)
    # content = llm_client.complete("What's the weather of paris and what is 2324+2342? use the provided tools and provide the exact output without modifying the value.", 1000, 0.7)
    print(f"{agent_id}: ", content)
    print("--------------------------------------------------")


models = ["mistral-small-latest"]


def without_tool_calling(model: str):
    agent_id = model
    api_key = os.getenv("MISTRALAI_API_KEY")
    llm_config = LlmConfig.mistralai(api_key, model)
    executor = Executor(llm_config)
    workflow = Workflow(f"Tool Test: {agent_id}")
    agent = Node.agent(
        name=agent_id,
        system_prompt="You are a helpful assistant.",
        prompt="What's the weather of paris and what is 2324+2342?",
        agent_id=agent_id,
    )
    workflow.add_node(agent)
    workflow.validate()

    result = executor.execute(workflow)
    content = result.get_node_output(agent_id)
    print(f"{agent_id}: ", content)
    print("--------------------------------------------------")
    print()


if __name__ == "__main__":
    for model in models:
        without_tool_calling(model)
        main(model)
