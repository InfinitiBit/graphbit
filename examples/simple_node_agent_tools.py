#!/usr/bin/env python3
"""Simple Node Agent with Multiple Tool Calls - 50 line example"""
import os
import datetime
import graphbit
from graphbit import Node, Workflow, LlmConfig, Executor

def get_city_temperature(city: str) -> str:
    print(f"🌡️ Getting temperature for {city}")
    temps = {"paris": "22°C", "london": "18°C", "tokyo": "25°C"}
    result = temps.get(city.lower(), "21°C")
    print(f"🌡️ Result: {result}")
    return result

def get_current_time() -> str:
    print("🕐 Getting current time")
    time_str = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"🕐 Result: {time_str}")
    return time_str

def extract_output(context):
    variables = context.variables()
    if variables:
        for key, value in variables:
            value_str = str(value).strip()
            if value_str and value_str.lower() not in ["null", "none"]:
                return value_str.strip('"')
    return "No output captured"

def main():
    graphbit.init()
    api_key = os.getenv("OPENAI_API_KEY")

    config = LlmConfig.openai(api_key, "gpt-4o-mini")
    temp = get_city_temperature('paris')
    time = get_current_time()

    workflow = Workflow("Multi-Tool Demo")
    agent = Node.agent("Assistant", f"Provide a friendly response about Paris temperature ({temp}) and current time ({time})")
    workflow.add_node(agent)

    executor = Executor(config)
    print("Executing agent...")
    result = executor.execute(workflow)
    print(f"Completed in {result.execution_time_ms()}ms")
    print(f"Response: {extract_output(result)}")

if __name__ == "__main__":
    main()
