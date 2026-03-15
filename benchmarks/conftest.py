import sys
from pathlib import Path

# Allow `from frameworks.xxx import ...` when running pytest from repo root
sys.path.insert(0, str(Path(__file__).parent))
