#!/usr/bin/env python3
"""
Extract text-only questions from GAIA validation set for benchmark integration.

This script:
1. Downloads GAIA validation set from HuggingFace
2. Filters for text-only questions (no file attachments)
3. Categorizes by level and complexity
4. Maps to benchmark scenarios
5. Outputs selected questions for review
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Optional

from dotenv import load_dotenv

# Load environment variables
load_dotenv()

def load_gaia_validation_set():
    """Load GAIA validation set using HuggingFace datasets library."""
    try:
        from datasets import load_dataset
    except ImportError:
        print("ERROR: HuggingFace datasets library not installed.")
        print("Install with: pip install datasets")
        return None
    
    print("Loading GAIA validation set...")
    try:
        # Load the 2023_all validation split
        dataset = load_dataset("gaia-benchmark/GAIA", "2023_all", split="validation")
        print(f"✓ Loaded {len(dataset)} questions from GAIA validation set")
        return dataset
    except Exception as e:
        print(f"ERROR loading dataset: {e}")
        print("\nNote: GAIA dataset may require authentication.")
        print("1. Create HuggingFace account at https://huggingface.co")
        print("2. Accept dataset terms at https://huggingface.co/datasets/gaia-benchmark/GAIA")
        print("3. Login with: huggingface-cli login")
        return None

def filter_text_only_questions(dataset) -> List[Dict]:
    """Filter questions that don't require file attachments or external tools."""
    text_only = []
    
    for item in dataset:
        # Check if question has file attachments
        has_files = False
        if 'file_name' in item and item['file_name']:
            has_files = True
        if 'file_path' in item and item['file_path']:
            has_files = True
        
        # Skip questions with files
        if has_files:
            continue
        
        # Check if question text suggests tool requirements
        question = item.get('Question', '')
        question_lower = question.lower()
        
        # Skip if mentions images, files, or specific websites
        tool_indicators = [
            'image', 'picture', 'photo', 'screenshot',
            'file', 'pdf', 'spreadsheet', 'csv',
            'website', 'browse', 'search for',
            'download', 'attached', 'provided file'
        ]
        
        requires_tools = any(indicator in question_lower for indicator in tool_indicators)
        if requires_tools:
            continue
        
        # This is a text-only question
        text_only.append({
            'question': item.get('Question', ''),
            'answer': item.get('Final answer', ''),
            'level': item.get('Level', 0),
            'task_id': item.get('task_id', ''),
            'annotator_metadata': item.get('Annotator Metadata', {})
        })
    
    print(f"✓ Found {len(text_only)} text-only questions")
    return text_only

def categorize_by_level(questions: List[Dict]) -> Dict[int, List[Dict]]:
    """Categorize questions by difficulty level."""
    by_level = {1: [], 2: [], 3: []}
    
    for q in questions:
        level = q['level']
        if level in by_level:
            by_level[level].append(q)
    
    print(f"\nLevel distribution:")
    print(f"  Level 1: {len(by_level[1])} questions")
    print(f"  Level 2: {len(by_level[2])} questions")
    print(f"  Level 3: {len(by_level[3])} questions")
    
    return by_level

def select_questions_for_scenarios(by_level: Dict[int, List[Dict]]) -> Dict[str, List[Dict]]:
    """Select appropriate questions for each benchmark scenario."""
    
    # Target counts for each scenario
    selection = {
        'simple_task': [],           # 1 Level 1 question
        'sequential_tasks': [],       # 3 Level 1-2 questions
        'parallel_tasks': [],         # 4 Level 1 questions
        'complex_workflow': [],       # 5 Level 2-3 questions
        'memory_intensive': [],       # 1 Level 2 question (longest context)
        'concurrent_tasks': []        # 8 Level 1 questions
    }
    
    # Simple Task: 1 straightforward Level 1 question
    if by_level[1]:
        selection['simple_task'] = [by_level[1][0]]
    
    # Sequential Tasks: 3 questions that could logically flow
    # Use mix of Level 1 and Level 2
    available_seq = by_level[1][1:4] if len(by_level[1]) > 3 else []
    if len(available_seq) < 3 and by_level[2]:
        available_seq.extend(by_level[2][:3-len(available_seq)])
    selection['sequential_tasks'] = available_seq[:3]
    
    # Parallel Tasks: 4 independent Level 1 questions
    start_idx = 4
    selection['parallel_tasks'] = by_level[1][start_idx:start_idx+4]
    
    # Concurrent Tasks: 8 independent Level 1 questions
    start_idx = 8
    selection['concurrent_tasks'] = by_level[1][start_idx:start_idx+8]
    
    # Complex Workflow: 5 Level 2-3 questions (mix)
    complex_pool = by_level[2][:4] if len(by_level[2]) >= 4 else by_level[2]
    if by_level[3]:
        complex_pool.append(by_level[3][0])
    selection['complex_workflow'] = complex_pool[:5]
    
    # Memory Intensive: 1 Level 2 question with longest text
    # Find longest question by character count
    if by_level[2]:
        longest = max(by_level[2], key=lambda q: len(q['question']))
        selection['memory_intensive'] = [longest]
    
    print(f"\n✓ Question selection complete:")
    for scenario, questions in selection.items():
        print(f"  {scenario}: {len(questions)} questions")
    
    return selection

def save_selected_questions(selection: Dict[str, List[Dict]], output_file: str):
    """Save selected questions to JSON file for review."""
    output = {
        'metadata': {
            'source': 'GAIA 2023 validation set',
            'filter': 'text-only (no file attachments)',
            'total_selected': sum(len(qs) for qs in selection.values())
        },
        'scenarios': {}
    }
    
    for scenario, questions in selection.items():
        output['scenarios'][scenario] = [
            {
                'question': q['question'],
                'answer': q['answer'],
                'level': q['level'],
                'task_id': q['task_id'],
                'length': len(q['question'])
            }
            for q in questions
        ]
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)
    
    print(f"\n✓ Saved selection to: {output_file}")
    print(f"\nReview the file to verify question appropriateness.")

def print_sample_questions(selection: Dict[str, List[Dict]]):
    """Print sample questions for quick review."""
    print("\n" + "="*80)
    print("SAMPLE QUESTIONS")
    print("="*80)
    
    for scenario, questions in selection.items():
        if not questions:
            continue
        
        print(f"\n[{scenario.upper().replace('_', ' ')}]")
        print(f"Count: {len(questions)}")
        print(f"\nFirst question:")
        q = questions[0]
        print(f"  Level: {q['level']}")
        print(f"  Question: {q['question'][:200]}...")
        print(f"  Answer: {q['answer']}")

def main():
    """Main extraction workflow."""
    print("="*80)
    print("GAIA Text-Only Question Extractor")
    print("="*80)
    
    # Step 1: Load dataset
    dataset = load_gaia_validation_set()
    if dataset is None:
        return
    
    # Step 2: Filter text-only questions
    text_only = filter_text_only_questions(dataset)
    
    if len(text_only) < 22:  # Minimum needed for all scenarios
        print(f"\n⚠️  WARNING: Only found {len(text_only)} text-only questions.")
        print(f"   Need at least 22 for all scenarios.")
        print(f"   Proceeding with available questions...")
    
    # Step 3: Categorize by level
    by_level = categorize_by_level(text_only)
    
    # Step 4: Select questions for scenarios
    selection = select_questions_for_scenarios(by_level)
    
    # Step 5: Save and display
    output_file = "gaia_selected_questions.json"
    save_selected_questions(selection, output_file)
    print_sample_questions(selection)
    
    print("\n" + "="*80)
    print("NEXT STEPS:")
    print("="*80)
    print("1. Review gaia_selected_questions.json")
    print("2. Verify questions are appropriate for each scenario")
    print("3. Run: python update_prompts_with_gaia.py")
    print("   (This will update frameworks/common.py)")

if __name__ == "__main__":
    main()
