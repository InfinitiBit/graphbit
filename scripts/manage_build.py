#!/usr/bin/env python3
"""
GraphBit Build Wrapper Script
Wraps build commands (cargo, maturin) to ensure license notices are displayed
ONLY after successful completion.
"""

import sys
import subprocess
import argparse
import shutil
from pathlib import Path

# Add python directory to path to import build_notice
REPO_ROOT = Path(__file__).parent.parent
PYTHON_DIR = REPO_ROOT / "python"
sys.path.append(str(PYTHON_DIR))

try:
    import build_notice
except ImportError:
    print("Error: Could not import build_notice.py from python directory", file=sys.stderr)
    sys.exit(1)

def run_command(command, args):
    """Run a command with arguments"""
    full_cmd = [command] + args
    
    # Check if executable exists
    if not shutil.which(command):
         print(f"Error: Command '{command}' not found in PATH.", file=sys.stderr)
         return 1

    print(f"Running: {' '.join(full_cmd)}")
    try:
        # Run command in the REPO_ROOT by default, or PYTHON_DIR if it's a python command?
        # cargo runs best from root or crate dir. maturin depends.
        # Let's run from current working directory, assuming user runs from root.
        result = subprocess.run(full_cmd, cwd=Path.cwd(), check=False)
        return result.returncode
    except KeyboardInterrupt:
        print("\nAborted.", file=sys.stderr)
        return 130
    except Exception as e:
         print(f"Error executing command: {e}", file=sys.stderr)
         return 1

def clean_artifacts():
    """Remove generated artifacts (*.pyd, *.so, *.dll) from python-src"""
    src_dir = PYTHON_DIR / "python-src"
    if not src_dir.exists():
        return

    print(f"Cleaning artifacts in {src_dir}...")
    patterns = ["**/*.pyd", "**/*.so", "**/*.dll"]
    count = 0
    for pattern in patterns:
        for path in src_dir.glob(pattern):
            try:
                # Add write permission if needed (some build artifacts are read-only)
                path.chmod(0o777)
                path.unlink()
                print(f"Removed: {path}")
                count += 1
            except Exception as e:
                print(f"Failed to remove {path}: {e}", file=sys.stderr)
    
    if count == 0:
        print("No artifacts found to clean.")
    else:
        print(f"Removed {count} artifacts.")

def main():
    parser = argparse.ArgumentParser(
        description="GraphBit Build Wrapper - Runs command and shows license notice on success."
    )
    parser.add_argument(
        "command", 
        help="Command to run (e.g., cargo, maturin, clean)"
    )
    parser.add_argument(
        "args", 
        nargs=argparse.REMAINDER, 
        help="Arguments passed to the command"
    )
    
    args = parser.parse_args()

    if args.command == "clean":
        clean_artifacts()
        sys.exit(0)

    # Auto-clean before build or develop
    if args.command == "maturin" and args.args and args.args[0] in ["build", "develop"]:
        print("Cleaning up old artifacts before build...")
        clean_artifacts()
    
    # Run the requested command
    ret_code = run_command(args.command, args.args)
    
    # Upon success, show license notice
    if ret_code == 0:
        print("\n" + "="*80)
        build_notice.show_build_license_notice()
        print("="*80 + "\n")
    
    sys.exit(ret_code)

if __name__ == "__main__":
    main()
