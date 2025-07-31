#!/usr/bin/env python3
"""
Resume Screening Agent using GraphBit

This agent screens resumes against job requirements and provides:
1. Requirement alignment scores
2. Candidate caliber scores  
3. Overall ranking
4. Formatted output with candidate names and file names
"""

import os
import json
import uuid
import glob
from pathlib import Path
from typing import List, Dict, Any, Tuple
import graphbit

# Try to import PDF processing libraries
try:
    import PyPDF2
    PDF_AVAILABLE = True
except ImportError:
    try:
        import pypdf
        PDF_AVAILABLE = True
    except ImportError:
        PDF_AVAILABLE = False

try:
    import pdfplumber
    PDFPLUMBER_AVAILABLE = True
except ImportError:
    PDFPLUMBER_AVAILABLE = False


class ResumeScreenerAgent:
    """Resume screening agent using GraphBit workflow"""
    
    def __init__(self, llm_config=None):
        """Initialize the resume screener agent"""
        graphbit.init()
        
        # Use provided config or default to OpenAI
        if llm_config is None:
            api_key = os.getenv("OPENAI_API_KEY")
            if not api_key:
                raise ValueError("OPENAI_API_KEY environment variable is required")
            llm_config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4o-mini")
        
        self.llm_config = llm_config
        self.executor = graphbit.Executor.new_memory_optimized(llm_config, timeout_seconds=300)
        
    def read_job_requirements(self, requirements_file: str) -> str:
        """Read job requirements from file"""
        try:
            with open(requirements_file, 'r', encoding='utf-8') as f:
                return f.read().strip()
        except Exception as e:
            raise Exception(f"Failed to read requirements file: {e}")
    
    def extract_text_from_pdf(self, file_path: str) -> str:
        """Extract text content from PDF file"""
        if not PDF_AVAILABLE and not PDFPLUMBER_AVAILABLE:
            raise Exception("PDF processing libraries not available. Install PyPDF2 or pdfplumber.")
        
        try:
            # Try pdfplumber first (better text extraction)
            if PDFPLUMBER_AVAILABLE:
                with pdfplumber.open(file_path) as pdf:
                    text = ""
                    for page in pdf.pages:
                        page_text = page.extract_text()
                        if page_text:
                            text += page_text + "\n"
                    return text.strip()
            
            # Fallback to PyPDF2
            elif PDF_AVAILABLE:
                with open(file_path, 'rb') as file:
                    if hasattr(PyPDF2, 'PdfReader'):
                        # PyPDF2 v3+
                        reader = PyPDF2.PdfReader(file)
                        text = ""
                        for page in reader.pages:
                            text += page.extract_text() + "\n"
                        return text.strip()
                    else:
                        # PyPDF2 v2
                        reader = PyPDF2.PdfFileReader(file)
                        text = ""
                        for page_num in range(reader.numPages):
                            page = reader.getPage(page_num)
                            text += page.extractText() + "\n"
                        return text.strip()
            
        except Exception as e:
            raise Exception(f"Failed to extract text from PDF {file_path}: {e}")
    
    def read_resume_files(self, resume_folder: str) -> List[Tuple[str, str]]:
        """Read all resume files from folder and return (filename, content) pairs"""
        resume_files = []
        supported_extensions = ['.pdf', '.txt', '.docx', '.doc']
        
        # Find all resume files
        for ext in supported_extensions:
            pattern = os.path.join(resume_folder, f"*{ext}")
            resume_files.extend(glob.glob(pattern))
        
        if not resume_files:
            raise Exception(f"No resume files found in {resume_folder}")
        
        # Read file contents
        resumes = []
        for file_path in resume_files:
            try:
                filename = os.path.basename(file_path)
                file_ext = os.path.splitext(filename)[1].lower()
                
                if file_ext == '.pdf':
                    # Extract text from PDF
                    content = self.extract_text_from_pdf(file_path)
                    if not content.strip():
                        print(f"Warning: No text extracted from PDF {filename}")
                        continue
                else:
                    # Read as text for other formats
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                
                resumes.append((filename, content))
                print(f"✅ Loaded {filename} ({len(content)} characters)")
                
            except Exception as e:
                print(f"⚠️  Warning: Could not read {file_path}: {e}")
                continue
        
        return resumes
    
    def create_workflow(self, job_requirements: str, resumes: List[Tuple[str, str]]) -> graphbit.Workflow:
        """Create the resume screening workflow"""
        workflow = graphbit.Workflow("Resume Screening Pipeline")
        agent_id = str(uuid.uuid4())
        
        # Create nodes for the workflow
        nodes = {}
        
        # 1. Job Requirements Analyzer
        requirements_analyzer = graphbit.Node.agent(
            name="Job Requirements Analyzer",
            prompt=f"""
            Analyze the following job requirements and extract key criteria for evaluation:
            
            JOB REQUIREMENTS:
            {job_requirements}
            
            Provide a structured analysis including:
            1. Required skills and qualifications
            2. Experience requirements
            3. Education requirements
            4. Key responsibilities
            5. Preferred qualifications
            
            Format your response as a JSON object with these categories.
            """,
            agent_id=agent_id
        )
        nodes['requirements_analyzer'] = workflow.add_node(requirements_analyzer)
        
        # 2. Resume Analyzer (will be called for each resume)
        resume_analyzer = graphbit.Node.agent(
            name="Resume Analyzer",
            prompt="""
            Analyze the following resume against the job requirements:
            
            RESUME CONTENT:
            {resume_content}
            
            RESUME FILENAME:
            {resume_filename}
            
            JOB REQUIREMENTS ANALYSIS:
            {requirements_analysis}
            
            Provide a comprehensive analysis including:
            1. Skills match (0-100 score)
            2. Experience alignment (0-100 score)
            3. Education fit (0-100 score)
            4. Overall requirement alignment (0-100 score)
            5. Candidate caliber assessment (0-100 score)
            6. Key strengths and weaknesses
            7. Extracted candidate name
            
            Format your response as a JSON object with these fields.
            """,
            agent_id=agent_id
        )
        nodes['resume_analyzer'] = workflow.add_node(resume_analyzer)
        
        # 3. Resume Ranking and Scoring
        ranking_agent = graphbit.Node.agent(
            name="Resume Ranking Agent",
            prompt="""
            Rank and score the analyzed resumes:
            
            RESUME ANALYSES:
            {resume_analyses}
            
            Provide:
            1. Overall ranking (1st, 2nd, 3rd, etc.)
            2. Final scores for each candidate
            3. Summary of top candidates
            4. Formatted output list with format: {{candidate_name-filename}}
            
            Format your response as a JSON object with ranking, scores, and formatted_list fields.
            """,
            agent_id=agent_id
        )
        nodes['ranking_agent'] = workflow.add_node(ranking_agent)
        
        # Connect nodes
        workflow.connect(nodes['requirements_analyzer'], nodes['resume_analyzer'])
        workflow.connect(nodes['resume_analyzer'], nodes['ranking_agent'])
        
        workflow.validate()
        return workflow
    
    def extract_candidate_name(self, resume_content: str, filename: str) -> str:
        """Extract candidate name from resume content"""
        # Simple heuristic to extract name from filename or content
        name_from_filename = os.path.splitext(filename)[0].replace('_', ' ').replace('-', ' ')
        
        # Try to find name patterns in content
        lines = resume_content.split('\n')
        for line in lines[:10]:  # Check first 10 lines
            line = line.strip()
            if line and len(line.split()) <= 4 and not any(word.lower() in ['resume', 'cv', 'curriculum', 'vitae'] for word in line.split()):
                return line
        
        return name_from_filename
    
    def analyze_resume(self, resume_content: str, filename: str, requirements_analysis: str) -> Dict[str, Any]:
        """Analyze a single resume against requirements"""
        # Create a simple workflow for single resume analysis
        workflow = graphbit.Workflow("Single Resume Analysis")
        agent_id = str(uuid.uuid4())
        
        analyzer = graphbit.Node.agent(
            name="Resume Analyzer",
            prompt=f"""
            Analyze the following resume against the job requirements:
            
            RESUME CONTENT:
            {resume_content}
            
            RESUME FILENAME:
            {filename}
            
            JOB REQUIREMENTS ANALYSIS:
            {requirements_analysis}
            
            Provide a comprehensive analysis including:
            1. Skills match (0-100 score)
            2. Experience alignment (0-100 score) 
            3. Education fit (0-100 score)
            4. Overall requirement alignment (0-100 score)
            5. Candidate caliber assessment (0-100 score)
            6. Key strengths and weaknesses
            7. Extracted candidate name
            
            Format your response as a JSON object with these fields.
            """,
            agent_id=agent_id
        )
        
        workflow.add_node(analyzer)
        workflow.validate()
        
        result = self.executor.execute(workflow)
        
        if result.is_failed():
            raise Exception(f"Resume analysis failed: {result.state()}")
        
        # Extract the analysis result
        variables = result.variables()
        if variables:
            for _, value in variables:
                try:
                    # Try to parse as JSON
                    analysis = json.loads(str(value))
                    return analysis
                except json.JSONDecodeError:
                    # If not JSON, return as string
                    return {"raw_analysis": str(value)}
        
        return {"error": "No analysis result found"}
    
    def screen_resumes(self, requirements_file: str, resume_folder: str) -> List[str]:
        """Main method to screen resumes and return ranked list"""
        print("🔍 Starting Resume Screening Process...")
        
        # 1. Read job requirements
        print("📋 Reading job requirements...")
        job_requirements = self.read_job_requirements(requirements_file)
        print(f"✅ Loaded requirements from {requirements_file}")
        
        # 2. Read resume files
        print("📁 Reading resume files...")
        resumes = self.read_resume_files(resume_folder)
        print(f"✅ Found {len(resumes)} resume files")
        
        # 3. Analyze job requirements
        print("🎯 Analyzing job requirements...")
        requirements_workflow = graphbit.Workflow("Requirements Analysis")
        agent_id = str(uuid.uuid4())
        
        req_analyzer = graphbit.Node.agent(
            name="Requirements Analyzer",
            prompt=f"""
            Analyze the following job requirements and extract key criteria for evaluation:
            
            JOB REQUIREMENTS:
            {job_requirements}
            
            Provide a structured analysis including:
            1. Required skills and qualifications
            2. Experience requirements
            3. Education requirements
            4. Key responsibilities
            5. Preferred qualifications
            
            Format your response as a JSON object with these categories.
            """,
            agent_id=agent_id
        )
        
        requirements_workflow.add_node(req_analyzer)
        requirements_workflow.validate()
        
        req_result = self.executor.execute(requirements_workflow)
        if req_result.is_failed():
            raise Exception(f"Requirements analysis failed: {req_result.state()}")
        
        # Extract requirements analysis
        req_variables = req_result.variables()
        requirements_analysis = ""
        if req_variables:
            for _, value in req_variables:
                requirements_analysis = str(value)
                break
        
        # 4. Analyze each resume
        print("📄 Analyzing resumes...")
        resume_analyses = []
        
        for filename, content in resumes:
            print(f"  Analyzing {filename}...")
            try:
                analysis = self.analyze_resume(content, filename, requirements_analysis)
                analysis['filename'] = filename
                resume_analyses.append(analysis)
            except Exception as e:
                print(f"  ⚠️  Error analyzing {filename}: {e}")
                continue
        
        # 5. Rank and score all resumes
        print("🏆 Ranking and scoring resumes...")
        ranking_workflow = graphbit.Workflow("Resume Ranking")
        agent_id = str(uuid.uuid4())
        
        ranking_agent = graphbit.Node.agent(
            name="Ranking Agent",
            prompt=f"""
            Rank and score the analyzed resumes:
            
            RESUME ANALYSES:
            {json.dumps(resume_analyses, indent=2)}
            
            Provide:
            1. Overall ranking (1st, 2nd, 3rd, etc.)
            2. Final scores for each candidate
            3. Summary of top candidates
            4. Formatted output list with format: {{candidate_name-filename}}
            
            Format your response as a JSON object with ranking, scores, and formatted_list fields.
            The formatted_list should be an array of strings in the format: "candidate_name-filename"
            """,
            agent_id=agent_id
        )
        
        ranking_workflow.add_node(ranking_agent)
        ranking_workflow.validate()
        
        ranking_result = self.executor.execute(ranking_workflow)
        if ranking_result.is_failed():
            raise Exception(f"Ranking failed: {ranking_result.state()}")
        
        # Extract final ranking
        ranking_variables = ranking_result.variables()
        if ranking_variables:
            for _, value in ranking_variables:
                try:
                    ranking_data = json.loads(str(value))
                    if 'formatted_list' in ranking_data:
                        return ranking_data['formatted_list']
                    else:
                        # Fallback: create formatted list from analyses
                        formatted_list = []
                        for analysis in resume_analyses:
                            candidate_name = analysis.get('candidate_name', analysis.get('filename', 'Unknown'))
                            filename = analysis.get('filename', 'unknown.pdf')
                            formatted_list.append(f"{candidate_name}-{filename}")
                        return formatted_list
                except json.JSONDecodeError:
                    # If not JSON, create fallback list
                    formatted_list = []
                    for analysis in resume_analyses:
                        candidate_name = analysis.get('candidate_name', analysis.get('filename', 'Unknown'))
                        filename = analysis.get('filename', 'unknown.pdf')
                        formatted_list.append(f"{candidate_name}-{filename}")
                    return formatted_list
        
        # Final fallback
        formatted_list = []
        for analysis in resume_analyses:
            candidate_name = analysis.get('candidate_name', analysis.get('filename', 'Unknown'))
            filename = analysis.get('filename', 'unknown.pdf')
            formatted_list.append(f"{candidate_name}-{filename}")
        
        return formatted_list


def main():
    """Main function to run the resume screener"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Resume Screening Agent")
    parser.add_argument("--requirements", "-r", required=True, help="Path to job requirements file")
    parser.add_argument("--resumes", "-f", required=True, help="Path to folder containing resumes")
    parser.add_argument("--output", "-o", help="Output file for results (optional)")
    
    args = parser.parse_args()
    
    # Validate inputs
    if not os.path.exists(args.requirements):
        print(f"❌ Requirements file not found: {args.requirements}")
        return
    
    if not os.path.exists(args.resumes):
        print(f"❌ Resume folder not found: {args.resumes}")
        return
    
    try:
        # Create and run the resume screener
        screener = ResumeScreenerAgent()
        results = screener.screen_resumes(args.requirements, args.resumes)
        
        print("\n" + "="*60)
        print("🏆 FINAL RANKING RESULTS")
        print("="*60)
        
        for i, result in enumerate(results, 1):
            print(f"{i}. {result}")
        
        # Save results if output file specified
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as f:
                json.dump(results, f, indent=2)
            print(f"\n💾 Results saved to: {args.output}")
        
        print(f"\n✅ Resume screening completed! Ranked {len(results)} candidates.")
        
    except Exception as e:
        print(f"❌ Error during resume screening: {e}")
        return


if __name__ == "__main__":
    main() 