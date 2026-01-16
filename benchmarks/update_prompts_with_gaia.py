#!/usr/bin/env python3
"""
Update benchmark prompts in common.py with GAIA questions.

This script reads the selected GAIA questions and updates the prompt constants
in frameworks/common.py while preserving all other functionality.
"""

import json
import json
import re
from pathlib import Path
from typing import List, Dict

from dotenv import load_dotenv

# Load environment variables
load_dotenv()

def load_selected_questions(json_file: str = "gaia_selected_questions.json") -> Dict:
    """Load the selected GAIA questions from JSON."""
    with open(json_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data['scenarios']

def format_question_for_prompt(question: str) -> str:
    """Format question for benchmark prompt (ensure proper escaping)."""
    # Escape special characters if needed
    return question

def generate_new_prompts(scenarios: Dict) -> Dict[str, str]:
    """Generate new prompt constants for common.py."""
    
    prompts = {}
    
    # Simple Task - single question
    if scenarios['simple_task']:
        q = scenarios['simple_task'][0]
        prompts['SIMPLE_TASK_PROMPT'] = f'"""{q["question"]}"""'
    
    # Sequential Tasks - list of 3 questions
    if scenarios['sequential_tasks']:
        tasks_list = [f'"{q["question"]}"' for q in scenarios['sequential_tasks']]
        prompts['SEQUENTIAL_TASKS'] = f'[\n    {",\n    ".join(tasks_list)}\n]'
    
    # Parallel Tasks - list of 4 questions
    if scenarios['parallel_tasks']:
        tasks_list = [f'"{q["question"]}"' for q in scenarios['parallel_tasks']]
        prompts['PARALLEL_TASKS'] = f'[\n    {",\n    ".join(tasks_list)}\n]'
    
    # Complex Workflow - list of dicts with task/prompt/depends_on
    # We'll create a logical dependency structure
    if scenarios['complex_workflow']:
        questions = scenarios['complex_workflow']
        workflow_steps = []
        
        # Step 1: No dependencies
        workflow_steps.append(f'''    {{
        "task": "step_1",
        "prompt": "{questions[0]["question"]}",
        "depends_on": []
    }}''')
        
        # Step 2: Depends on step 1
        workflow_steps.append(f'''    {{
        "task": "step_2",
        "prompt": "{questions[1]["question"]}",
        "depends_on": ["step_1"]
    }}''')
        
        # Step 3: Depends on step 1
        workflow_steps.append(f'''    {{
        "task": "step_3",
        "prompt": "{questions[2]["question"]}",
        "depends_on": ["step_1"]
    }}''')
        
        # Step 4: Depends on steps 2 and 3
        workflow_steps.append(f'''    {{
        "task": "step_4",
        "prompt": "{questions[3]["question"]}",
        "depends_on": ["step_2", "step_3"]
    }}''')
        
        # Step 5: Depends on step 4
        workflow_steps.append(f'''    {{
        "task": "step_5",
        "prompt": "{questions[4]["question"]}",
        "depends_on": ["step_4"]
    }}''')
        
        prompts['COMPLEX_WORKFLOW_STEPS'] = f'[\n{",\n".join(workflow_steps)}\n]'
    
    # Memory Intensive - single long question
    if scenarios['memory_intensive']:
        q = scenarios['memory_intensive'][0]
        prompts['MEMORY_INTENSIVE_PROMPT'] = f'"""{q["question"]}"""'
    
    # Concurrent Tasks - list of 8 questions
    if scenarios['concurrent_tasks']:
        tasks_list = [f'"{q["question"]}"' for q in scenarios['concurrent_tasks']]
        prompts['CONCURRENT_TASK_PROMPTS'] = f'[\n    {",\n    ".join(tasks_list)}\n]'
    
    return prompts

def update_common_py(prompts: Dict[str, str], common_file: str = "frameworks/common.py"):
    """Update the common.py file with new prompts."""
    
    # Read current file
    with open(common_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Create backup
    backup_file = common_file + ".backup_before_gaia"
    with open(backup_file, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"✓ Created backup: {backup_file}")
    
    # Replace each prompt constant
    for const_name, new_value in prompts.items():
        # Find the pattern for this constant
        # Pattern: CONST_NAME = """...""" or [...] or {...}
        
        if const_name == 'SIMPLE_TASK_PROMPT' or const_name == 'MEMORY_INTENSIVE_PROMPT':
            # String constant with triple quotes
            pattern = rf'{const_name}\s*=\s*""".*?"""'
            replacement = f'{const_name} = {new_value}'
            content = re.sub(pattern, replacement, content, flags=re.DOTALL)
        
        elif const_name in ['SEQUENTIAL_TASKS', 'PARALLEL_TASKS', 'CONCURRENT_TASK_PROMPTS']:
            # List constants
            pattern = rf'{const_name}\s*=\s*\[.*?\]'
            replacement = f'{const_name} = {new_value}'
            content = re.sub(pattern, replacement, content, flags=re.DOTALL)
        
        elif const_name == 'COMPLEX_WORKFLOW_STEPS':
            # List of dict constants
            pattern = rf'{const_name}\s*=\s*\[.*?\]\s*(?=\n\n|\n#|\nclass|\ndef|$)'
            replacement = f'{const_name} = {new_value}\n'
            content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    
    # Write updated content
    with open(common_file, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"✓ Updated: {common_file}")
    print("\nUpdated prompts:")
    for const_name in prompts.keys():
        print(f"  - {const_name}")

def add_gaia_comment(common_file: str = "frameworks/common.py"):
    """Add comment at top of file documenting GAIA source."""
    with open(common_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    gaia_comment = '''# GAIA Benchmark Questions
# Source: Curated from GAIA validation set (public examples)
# Paper: https://arxiv.org/abs/2311.12983
# Dataset: https://huggingface.co/datasets/gaia-benchmark/GAIA
#
# Questions are text-only (no tool/file requirements) and mapped to scenarios:
# - Simple Task: Level 1 factual question
# - Sequential: Level 1-2 multi-step questions
# - Parallel: Level 1 independent questions
# - Complex: Level 2-3 questions with dependencies
# - Memory: Level 2 long-context question
# - Concurrent: Level 1 high-volume questions
#

'''
    
    # Find the first import or constant definition
    first_code_match = re.search(r'(import |from |class |def |[A-Z_]+ =)', content)
    if first_code_match:
        insert_pos = first_code_match.start()
        content = content[:insert_pos] + gaia_comment + content[insert_pos:]
    
    with open(common_file, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print("✓ Added GAIA source documentation comment")

def main():
    """Main update workflow."""
    print("="*80)
    print("Updating Benchmark Prompts with GAIA Questions")
    print("="*80)
    
    # Step 1: Load selected questions
    print("\n1. Loading selected GAIA questions...")
    scenarios = load_selected_questions()
    total_questions = sum(len(qs) for qs in scenarios.values())
    print(f"   Loaded {total_questions} questions across {len(scenarios)} scenarios")
    
    # Step 2: Generate new prompts
    print("\n2. Generating new prompt constants...")
    prompts = generate_new_prompts(scenarios)
    print(f"   Generated {len(prompts)} prompt constants")
    
    # Step 3: Update common.py
    print("\n3. Updating frameworks/common.py...")
    update_common_py(prompts)
    
    # Step 4: Add documentation comment
    print("\n4. Adding GAIA source documentation...")
    add_gaia_comment()
    
    print("\n" + "="*80)
    print("SUCCESS: Benchmark prompts updated with GAIA questions!")
    print("="*80)
    print("\nNext steps:")
    print("1. Review frameworks/common.py to verify changes")
    print("2. Run quick test: python run_benchmark.py --frameworks graphbit --scenarios simple_task --num-runs 1")
    print("3. If successful, run full benchmark")
    print("\nBackup saved to: frameworks/common.py.backup_before_gaia")
    print("(Restore with: mv frameworks/common.py.backup_before_gaia frameworks/common.py)")

if __name__ == "__main__":
    main()
