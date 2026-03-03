"""Iterative agent loop tests for Gemini provider."""
import os, sys
from dotenv import load_dotenv
load_dotenv()
sys.path.insert(0, os.path.dirname(__file__))
from test_helpers import TestRunner
from graphbit import LlmConfig

api_key = os.getenv("GEMINI_API_KEY")
if not api_key:
    print("ERROR: GEMINI_API_KEY not set"); sys.exit(1)

config = LlmConfig.gemini(api_key, "gemini-2.5-pro")
runner = TestRunner("Gemini (2.5-pro)", config, delay_between_tests=2.0)
success = runner.run_all_tests()
sys.exit(0 if success else 1)
