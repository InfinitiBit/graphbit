#!/usr/bin/env python3
"""
Script to create sample PDF resumes for testing the resume screening agent.
This script converts the text resume files to PDF format.
"""

import os
from pathlib import Path

def create_pdf_from_text(text_file: str, pdf_file: str):
    """Create a PDF file from text content"""
    try:
        from reportlab.lib.pagesizes import letter
        from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer
        from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
        from reportlab.lib.units import inch
        
        # Read the text content
        with open(text_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Create PDF document
        doc = SimpleDocTemplate(pdf_file, pagesize=letter)
        styles = getSampleStyleSheet()
        
        # Create custom style for better formatting
        custom_style = ParagraphStyle(
            'CustomStyle',
            parent=styles['Normal'],
            fontSize=10,
            spaceAfter=6,
            spaceBefore=6
        )
        
        # Split content into paragraphs
        paragraphs = []
        lines = content.split('\n')
        
        for line in lines:
            line = line.strip()
            if line:
                # Handle headers (all caps lines)
                if line.isupper() and len(line) > 3:
                    para = Paragraph(line, styles['Heading1'])
                else:
                    para = Paragraph(line, custom_style)
                paragraphs.append(para)
                paragraphs.append(Spacer(1, 6))
        
        # Build PDF
        doc.build(paragraphs)
        print(f"✅ Created PDF: {pdf_file}")
        
    except ImportError:
        print("⚠️  ReportLab not available. Creating simple text-based PDF...")
        create_simple_pdf(text_file, pdf_file)
    except Exception as e:
        print(f"❌ Error creating PDF {pdf_file}: {e}")

def create_simple_pdf(text_file: str, pdf_file: str):
    """Create a simple PDF using fpdf if reportlab is not available"""
    try:
        from fpdf import FPDF
        
        # Read the text content
        with open(text_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Create PDF
        pdf = FPDF()
        pdf.add_page()
        pdf.set_font("Arial", size=10)
        
        # Add content
        lines = content.split('\n')
        for line in lines:
            line = line.strip()
            if line:
                # Handle headers (all caps lines)
                if line.isupper() and len(line) > 3:
                    pdf.set_font("Arial", 'B', 12)
                    pdf.cell(0, 10, txt=line, ln=True)
                    pdf.set_font("Arial", size=10)
                else:
                    pdf.cell(0, 8, txt=line, ln=True)
        
        pdf.output(pdf_file)
        print(f"✅ Created PDF: {pdf_file}")
        
    except ImportError:
        print("⚠️  FPDF not available. Skipping PDF creation.")
        print("Install reportlab or fpdf: pip install reportlab fpdf")
    except Exception as e:
        print(f"❌ Error creating PDF {pdf_file}: {e}")

def main():
    """Create sample PDF resumes"""
    script_dir = Path(__file__).parent
    sample_resumes_dir = script_dir / "sample_resumes"
    
    # Create sample_resumes directory if it doesn't exist
    sample_resumes_dir.mkdir(exist_ok=True)
    
    # Define the text resume files and their PDF counterparts
    resume_files = [
        ("john_doe_resume.txt", "john_doe_resume.pdf"),
        ("jane_smith_resume.txt", "jane_smith_resume.pdf"),
        ("mike_johnson_resume.txt", "mike_johnson_resume.pdf")
    ]
    
    print("📄 Creating sample PDF resumes...")
    
    for text_file, pdf_file in resume_files:
        text_path = sample_resumes_dir / text_file
        pdf_path = sample_resumes_dir / pdf_file
        
        if text_path.exists():
            create_pdf_from_text(str(text_path), str(pdf_path))
        else:
            print(f"⚠️  Text file not found: {text_path}")
    
    print("\n✅ PDF creation completed!")
    print("📁 Sample PDF resumes are now available in the sample_resumes/ directory")

if __name__ == "__main__":
    main() 