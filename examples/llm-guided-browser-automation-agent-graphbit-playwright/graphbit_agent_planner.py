import graphbit
import openai
import json
import os
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("GraphbitAgent")

openai.api_key = os.getenv("OPENAI_API_KEY")

def get_plan_from_graphbit(prompt: str) -> list:
    logger.info("Initializing Graphbit workflow...")
    graphbit.init()
    workflow = graphbit.Workflow("llm-plan")
    node = graphbit.Node.agent("Planner", (
        "Convert this prompt into a JSON plan using actions like 'goto', 'wait_for', 'click', 'extract_text'. "
        "Only output the JSON list.\n\nPrompt: " + prompt
    ))
    workflow.add_node(node)

    # Use the agent node's prompt as the LLM system message
    planner_prompt = node.prompt
    logger.info(f"Calling GPT-4 with planner prompt:\n{planner_prompt}")

    response = openai.ChatCompletion.create(
        model="gpt-4",
        messages=[
            {"role": "system", "content": "You are a browser automation planner. Convert tasks to JSON steps."},
            {"role": "user", "content": planner_prompt}
        ],
        temperature=0
    )

    content = response['choices'][0]['message']['content']
    logger.info("GPT-4 raw output:\n" + content)
    return json.loads(content)
