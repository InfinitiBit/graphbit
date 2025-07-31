#!/usr/bin/env python3
"""
Test script for the Resume Screening Agent

This script demonstrates how to use the resume screening agent with sample files.
"""

import os
import sys
from pathlib import Path

# Add the current directory to the path so we can import the agent
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from resume_screener_agent import ResumeScreenerAgent


def test_resume_screener():
    """Test the resume screener with sample files"""
    
    # Get the directory where this script is located
    script_dir = Path(__file__).parent
    
    # Define paths to sample files
    requirements_file = script_dir / "sample_job_requirements.txt"
    resumes_folder = script_dir / "sample_resumes"
    
    # Check if sample files exist
    if not requirements_file.exists():
        print(f"❌ Requirements file not found: {requirements_file}")
        return
    
    if not resumes_folder.exists():
        print(f"❌ Resumes folder not found: {resumes_folder}")
        return
    
    print("🧪 Testing Resume Screening Agent")
    print("=" * 50)
    
    try:
        # Create the resume screener
        print("🔧 Initializing Resume Screener...")
        screener = ResumeScreenerAgent()
        
        # Screen the resumes
        print(f"📋 Requirements file: {requirements_file}")
        print(f"📁 Resumes folder: {resumes_folder}")
        print()
        
        results = screener.screen_resumes(
            requirements_file=str(requirements_file),
            resume_folder=str(resumes_folder)
        )
        
        # Display results
        print("\n" + "=" * 50)
        print("🏆 FINAL RANKING RESULTS")
        print("=" * 50)
        
        for i, result in enumerate(results, 1):
            print(f"{i}. {result}")
        
        print(f"\n✅ Test completed successfully! Ranked {len(results)} candidates.")
        
        # Save results to a test output file
        output_file = script_dir / "test_results.json"
        import json
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2)
        print(f"💾 Test results saved to: {output_file}")
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()


def test_without_api():
    """Test the file reading functionality without API calls"""
    print("📁 Testing file reading functionality...")
    
    script_dir = Path(__file__).parent
    requirements_file = script_dir / "sample_job_requirements.txt"
    resumes_folder = script_dir / "sample_resumes"
    
    try:
        # Create screener without API calls
        screener = ResumeScreenerAgent()
        
        # Test reading requirements
        requirements = screener.read_job_requirements(str(requirements_file))
        print(f"✅ Read requirements file: {len(requirements)} characters")
        
        # Test reading resumes
        resumes = screener.read_resume_files(str(resumes_folder))
        print(f"✅ Read {len(resumes)} resume files:")
        for filename, content in resumes:
            print(f"  - {filename}: {len(content)} characters")
        
        print("✅ File reading test completed successfully!")
        
    except Exception as e:
        print(f"❌ File reading test failed: {e}")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Test Resume Screening Agent")
    parser.add_argument("--files-only", action="store_true", 
                       help="Test only file reading functionality (no API calls)")
    
    args = parser.parse_args()
    
    if args.files_only:
        test_without_api()
    else:
        test_resume_screener() 