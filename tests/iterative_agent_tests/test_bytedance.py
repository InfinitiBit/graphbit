"""Iterative agent loop tests for ByteDance provider."""
import os, sys
from dotenv import load_dotenv
load_dotenv()
sys.path.insert(0, os.path.dirname(__file__))
from test_helpers import TestRunner
from graphbit import LlmConfig

api_key = os.getenv("BYTEDANCE_API_KEY")
if not api_key:
    print("ERROR: BYTEDANCE_API_KEY not set"); sys.exit(1)

config = LlmConfig.bytedance(api_key, "seed-1-6-250915")
runner = TestRunner("ByteDance (seed-1-6-250915)", config, delay_between_tests=1.0)
success = runner.run_all_tests()
sys.exit(0 if success else 1)
