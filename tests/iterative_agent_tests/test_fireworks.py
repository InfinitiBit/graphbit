"""Iterative agent loop tests for Fireworks provider."""
import os, sys
from dotenv import load_dotenv
load_dotenv()
sys.path.insert(0, os.path.dirname(__file__))
from test_helpers import TestRunner
from graphbit import LlmConfig

api_key = os.getenv("FIREWORKS_API_KEY")
if not api_key:
    # Fireworks key might not be in .env, check if env var set
    print("ERROR: FIREWORKS_API_KEY not set"); sys.exit(1)

config = LlmConfig.fireworks(api_key, "accounts/fireworks/models/kimi-k2-instruct-0905")
runner = TestRunner("Fireworks (kimi-k2-instruct-0905)", config, delay_between_tests=1.0)
success = runner.run_all_tests()
sys.exit(0 if success else 1)
