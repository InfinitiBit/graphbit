# Resume Screening Agent using GraphBit

This is a comprehensive resume screening agent built with GraphBit that automatically analyzes resumes against job requirements and provides ranked results.

## Features

- **Job Requirements Analysis**: Extracts and analyzes key criteria from job requirement files
- **Resume Processing**: Reads and processes multiple resume files (PDF, TXT, DOCX, DOC)
- **PDF Support**: Full PDF text extraction using pdfplumber and PyPDF2
- **Intelligent Scoring**: Provides scores for:
  - Skills match (0-100)
  - Experience alignment (0-100)
  - Education fit (0-100)
  - Overall requirement alignment (0-100)
  - Candidate caliber assessment (0-100)
- **Ranking System**: Ranks candidates based on comprehensive analysis
- **Formatted Output**: Returns results in the format `{candidate_name-filename}`

## Requirements

- Python 3.8+
- GraphBit library
- OpenAI API key (or other supported LLM provider)
- PDF processing libraries (automatically installed)

## Installation

1. Install GraphBit and dependencies:
```bash
pip install -r requirements.txt
```

2. Set up your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
```

## Usage

### Command Line Interface

```bash
python resume_screener_agent.py --requirements path/to/requirements.txt --resumes path/to/resumes/folder
```

#### Options:
- `--requirements`, `-r`: Path to job requirements file (required)
- `--resumes`, `-f`: Path to folder containing resume files (required)
- `--output`, `-o`: Output file for results (optional)

### Example Usage

```bash
# Basic usage with PDF resumes
python resume_screener_agent.py -r sample_job_requirements.txt -f sample_resumes/

# With output file
python resume_screener_agent.py -r sample_job_requirements.txt -f sample_resumes/ -o results.json
```

### Programmatic Usage

```python
from resume_screener_agent import ResumeScreenerAgent

# Create the agent
screener = ResumeScreenerAgent()

# Screen resumes
results = screener.screen_resumes(
    requirements_file="path/to/requirements.txt",
    resume_folder="path/to/resumes/"
)

# Print results
for i, result in enumerate(results, 1):
    print(f"{i}. {result}")
```

## File Formats

### Job Requirements File
- **Format**: Plain text (.txt)
- **Content**: Detailed job description with requirements, skills, experience, education, etc.

### Resume Files
- **Primary Format**: PDF (recommended for real-world usage)
- **Supported Formats**: PDF, TXT, DOCX, DOC
- **Location**: All resume files should be in a single folder
- **Naming**: Files should be named descriptively (e.g., `john_doe_resume.pdf`)

## PDF Support

The agent includes robust PDF processing capabilities:

- **Text Extraction**: Uses pdfplumber (primary) and PyPDF2 (fallback)
- **Multi-page Support**: Handles multi-page PDF resumes
- **Error Handling**: Graceful handling of corrupted or unreadable PDFs
- **Format Preservation**: Maintains text structure and formatting

### PDF Processing Libraries

The agent automatically detects and uses available PDF libraries:
- `pdfplumber` (preferred - better text extraction)
- `PyPDF2` (fallback)
- `pypdf` (alternative)

## Sample Files

This directory includes sample files for testing:

- `sample_job_requirements.txt`: Sample job requirements for a Senior Software Engineer position
- `sample_resumes/`: Folder containing sample resumes:
  - `john_doe_resume.txt/pdf`: Senior developer with strong experience
  - `jane_smith_resume.txt/pdf`: Mid-level developer with good experience
  - `mike_johnson_resume.txt/pdf`: Junior developer with limited experience

### Creating Sample PDF Resumes

To create sample PDF resumes from the text files:

```bash
python create_sample_pdfs.py
```

This will generate PDF versions of the text resumes for testing.

## Workflow Process

1. **Requirements Analysis**: The agent reads and analyzes the job requirements file to extract key criteria
2. **Resume Processing**: Reads all resume files from the specified folder (with PDF text extraction)
3. **Individual Analysis**: Analyzes each resume against the requirements using AI
4. **Scoring**: Provides comprehensive scores for different aspects
5. **Ranking**: Ranks all candidates based on the analysis
6. **Output**: Returns formatted results with candidate names and filenames

## Output Format

The agent returns a list of strings in the format:
```
{candidate_name-filename}
```

Example output:
```
1. John Doe-john_doe_resume.pdf
2. Jane Smith-jane_smith_resume.pdf
3. Mike Johnson-mike_johnson_resume.pdf
```

## Configuration

### LLM Configuration

You can customize the LLM configuration:

```python
import graphbit

# Use OpenAI
config = graphbit.LlmConfig.openai(
    api_key="your-api-key",
    model="gpt-4o-mini"
)

# Use other providers
config = graphbit.LlmConfig.anthropic(
    api_key="your-api-key",
    model="claude-3-sonnet-20240229"
)

screener = ResumeScreenerAgent(llm_config=config)
```

### Supported LLM Providers

- OpenAI (GPT-4, GPT-3.5)
- Anthropic (Claude)
- Ollama (local models)
- And other providers supported by GraphBit

## Error Handling

The agent includes comprehensive error handling for:
- Missing or invalid files
- Network issues with LLM providers
- Malformed resume content
- API rate limits
- PDF processing errors
- Corrupted or unreadable PDFs

## Performance Considerations

- **Large Resume Folders**: The agent processes resumes sequentially to avoid overwhelming the LLM
- **PDF Processing**: Uses efficient PDF text extraction with fallback options
- **Timeout**: Default timeout is 300 seconds per workflow execution
- **Memory**: Uses memory-optimized executor for large workloads
- **Rate Limiting**: Built-in delays between API calls

## Troubleshooting

### Common Issues

1. **API Key Not Set**: Ensure `OPENAI_API_KEY` environment variable is set
2. **File Not Found**: Check file paths and permissions
3. **No Resumes Found**: Ensure resume files have supported extensions
4. **Network Issues**: Check internet connection and API access
5. **PDF Processing Errors**: Install required PDF libraries: `pip install pdfplumber PyPDF2`

### PDF-Specific Issues

- **No Text Extracted**: Some PDFs may be image-based or have security restrictions
- **Encoding Issues**: The agent handles various PDF encodings automatically
- **Large PDFs**: The agent processes PDFs efficiently but may take longer for very large files

### Debug Mode

For debugging, you can modify the agent to print more detailed information:

```python
# Add debug prints in the analyze_resume method
print(f"Analyzing {filename}...")
print(f"Content length: {len(content)} characters")
```

## Contributing

To extend the resume screening agent:

1. **Add New File Formats**: Modify the `read_resume_files` method
2. **Custom Scoring**: Update the analysis prompts in `analyze_resume`
3. **Additional Criteria**: Extend the requirements analysis
4. **Export Formats**: Add new output formats
5. **PDF Processing**: Enhance PDF text extraction capabilities

## License

This project is part of the GraphBit examples and follows the same license as GraphBit.

## Support

For issues and questions:
- Check the GraphBit documentation
- Review the sample files for reference
- Ensure all dependencies are properly installed
- Test with the provided sample files first 