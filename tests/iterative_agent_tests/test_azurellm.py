"""Iterative agent loop tests for Azure LLM provider."""
import os, sys
from dotenv import load_dotenv
load_dotenv()
sys.path.insert(0, os.path.dirname(__file__))
from test_helpers import TestRunner
from graphbit import LlmConfig

api_key = os.getenv("AZURELLM_API_KEY")
endpoint = os.getenv("AZURELLM_ENDPOINT")
if not api_key or not endpoint:
    print("ERROR: AZURELLM_API_KEY or AZURELLM_ENDPOINT not set"); sys.exit(1)

config = LlmConfig.azurellm(api_key, "gpt-4o", endpoint)
runner = TestRunner("Azure LLM (gpt-4o)", config, delay_between_tests=1.0)
success = runner.run_all_tests()
sys.exit(0 if success else 1)
