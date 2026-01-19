
import sys
import os
from pathlib import Path

# Setup path to import _license_notice directly
PYTHON_SRC = Path(__file__).parent.parent / "python" / "python-src" / "graphbit"
sys.path.append(str(PYTHON_SRC))

# Mock environment for enterprise check
os.environ["AWS_EXECUTION_ENV"] = "true"
os.environ["KUBERNETES_SERVICE_HOST"] = "10.0.0.1" # Trigger "multiple indicators" if needed, logic says >= 2

print("--- START VERIFICATION ---")

try:
    import _license_notice
    # It runs on import, but we can call explicitly to be sure and see output
    print("\n[Explicit Call Check]")
    _license_notice.check_license_compliance()
except ImportError as e:
    print(f"Import Error: {e}")
except Exception as e:
    print(f"Error: {e}")

print("--- END VERIFICATION ---")
