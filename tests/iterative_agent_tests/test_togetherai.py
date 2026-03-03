"""Iterative agent loop tests for TogetherAI provider."""
import os, sys
from dotenv import load_dotenv
load_dotenv()
sys.path.insert(0, os.path.dirname(__file__))
from test_helpers import TestRunner
from graphbit import LlmConfig

api_key = os.getenv("TOGETHERAI_API_KEY")
if not api_key:
    print("ERROR: TOGETHERAI_API_KEY not set"); sys.exit(1)

# Use a model that supports function calling
config = LlmConfig.togetherai(api_key, "meta-llama/Llama-3.3-70B-Instruct-Turbo")
runner = TestRunner("TogetherAI (Llama-3.3-70B)", config, delay_between_tests=1.0)
success = runner.run_all_tests()
sys.exit(0 if success else 1)
