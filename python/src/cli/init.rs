//! Project initialization functionality for GraphBit CLI

use super::{CliError, ensure_dir, to_py_cli_error, write_file};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;

/// Initialize a new GraphBit project with E2B-compatible structure
#[pyfunction]
#[pyo3(signature = (project_name, target_dir=None, template=None))]
pub(crate) fn init_project(
    py: Python<'_>,
    project_name: String,
    target_dir: Option<String>,
    template: Option<String>,
) -> PyResult<Bound<'_, PyDict>> {
    let result = PyDict::new(py);
    
    // Determine project path
    let base_dir = target_dir.unwrap_or_else(|| ".".to_string());
    let project_path = Path::new(&base_dir).join(&project_name);
    
    // Check if project already exists
    if project_path.exists() {
        return Err(to_py_cli_error(CliError::ProjectExists(
            project_path.display().to_string(),
        )));
    }
    
    // Create project structure
    match create_project_structure(&project_path, &project_name, template.as_deref()) {
        Ok(created_files) => {
            result.set_item("success", true)?;
            result.set_item("project_path", project_path.display().to_string())?;
            result.set_item("created_files", created_files)?;
            result.set_item("message", format!("Successfully created GraphBit project '{}'", project_name))?;
        }
        Err(err) => {
            return Err(to_py_cli_error(err));
        }
    }
    
    Ok(result)
}

/// Create the complete project structure
fn create_project_structure(
    project_path: &Path,
    project_name: &str,
    _template: Option<&str>,
) -> Result<Vec<String>, CliError> {
    let mut created_files = Vec::new();
    
    // Create main project directory
    ensure_dir(project_path)?;
    created_files.push(project_path.display().to_string());
    
    // Create minimal subdirectories (only if needed)
    // No subdirectories - keep it simple and flat
    
    // Create configuration files
    create_config_files(project_path, project_name, &mut created_files)?;
    
    // Create comprehensive template files (all patterns included)
    create_comprehensive_template(project_path, &mut created_files)?;
    
    Ok(created_files)
}

/// Create configuration files
fn create_config_files(
    project_path: &Path,
    project_name: &str,
    created_files: &mut Vec<String>,
) -> Result<(), CliError> {
    // .env.example
    let env_example = project_path.join(".env.example");
    write_file(&env_example, &get_env_example_content())?;
    created_files.push(env_example.display().to_string());
    
    // .gitignore
    let gitignore = project_path.join(".gitignore");
    write_file(&gitignore, &get_gitignore_content())?;
    created_files.push(gitignore.display().to_string());
    
    // requirements.txt
    let requirements = project_path.join("requirements.txt");
    write_file(&requirements, &get_requirements_content())?;
    created_files.push(requirements.display().to_string());
    
    // README.md
    let readme = project_path.join("README.md");
    write_file(&readme, &get_readme_content(project_name))?;
    created_files.push(readme.display().to_string());
    
    Ok(())
}

/// Create comprehensive template with all patterns
fn create_comprehensive_template(
    project_path: &Path,
    created_files: &mut Vec<String>,
) -> Result<(), CliError> {
    // Main entry point - user chooses which pattern to run
    let main_file = project_path.join("main.py");
    write_file(&main_file, &get_comprehensive_main_content())?;
    created_files.push(main_file.display().to_string());

    // Agent pattern
    let agent_file = project_path.join("agent.py");
    write_file(&agent_file, &get_simple_agent_content())?;
    created_files.push(agent_file.display().to_string());

    // Workflow pattern
    let workflow_file = project_path.join("workflow.py");
    write_file(&workflow_file, &get_workflow_content())?;
    created_files.push(workflow_file.display().to_string());

    // Workflow with tools pattern
    let workflow_tools_file = project_path.join("workflow_with_tools.py");
    write_file(&workflow_tools_file, &get_workflow_with_tools_content())?;
    created_files.push(workflow_tools_file.display().to_string());

    // Tools module
    let tools_file = project_path.join("tools.py");
    write_file(&tools_file, &get_tools_content())?;
    created_files.push(tools_file.display().to_string());



    Ok(())
}





// Template content functions

fn get_env_example_content() -> String {
    r#"# GraphBit Environment Variables

# E2B API Key (required for deployment)
# Get your API key from https://e2b.dev/dashboard
E2B_API_KEY=your_e2b_api_key_here

# LLM API Keys (choose the ones you need)
OPENAI_API_KEY=your_openai_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# GraphBit Tracer Configuration (optional)
# Get your API key from GraphBit platform
GRAPHBIT_TRACING_API_KEY=your_graphbit_tracing_api_key_here
GRAPHBIT_TRACEABLE_PROJECT=your_project_name_here
"#.to_string()
}

fn get_gitignore_content() -> String {
    r#"# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
*.egg-info/
.installed.cfg
*.egg

# Environment variables
.env
.env.local

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# GraphBit specific
outputs/
*.log
"#.to_string()
}

fn get_requirements_content() -> String {
    r#"# GraphBit framework
graphbit

# GraphBit tracing and observability
graphbit-tracer

# E2B integration
e2b-code-interpreter

# Environment management
python-dotenv

# Optional: Additional packages for common use cases
# pandas
# numpy
# matplotlib
# requests
"#.to_string()
}

fn get_readme_content(project_name: &str) -> String {
    format!(r#"# {project_name}

A GraphBit project for agentic workflow automation with E2B deployment support.

## Setup

1. **Install dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

2. **Configure environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

3. **Get E2B API key:**
   - Sign up at [E2B Dashboard](https://e2b.dev/dashboard)
   - Copy your API key to `.env`

## Usage

### Local Development
```bash
# Run the example workflow
python main.py

# Or use GraphBit CLI
graphbit run workflows/example_workflow.py
```

### E2B Deployment
```bash
# Deploy to E2B sandbox
graphbit deploy
```

## Project Structure

- `main.py` - Main entry point (run this!)
- `agent.py` - Simple GraphBit agent example
- `.env` - Environment variables (API keys)
- `requirements.txt` - Python dependencies

## Learn More

- [GraphBit Documentation](https://github.com/graphbit-ai/graphbit)
- [E2B Documentation](https://e2b.dev/docs)
"#, project_name = project_name)
}

fn get_simple_agent_content() -> String {
    r#""""Simple GraphBit agent example with optional GraphBit Tracer integration."""

import asyncio
import os
from graphbit import LlmConfig, Executor, Workflow, Node, tool
from graphbit_tracer import AutoTracer


# Define a simple tool
@tool(_description="Add two numbers together")
def add_numbers(a: float, b: float) -> float:
    """Add two numbers and return the result."""
    return a + b


async def run_agent():
    """Run a simple GraphBit agent with optional tracer integration."""
    print("=" * 50)
    print("GraphBit Agent Example")
    print("=" * 50)

    # Initialize tracer
    tracer = await AutoTracer.create()

    # Configure LLM
    api_key = os.getenv("OPENAI_API_KEY")


    llm_config = LlmConfig.openai(api_key, "gpt-4o-mini")

    # Create workflow with tool
    workflow = Workflow("SimpleAgent")
    agent_node = Node.agent(
        name="SimpleAgent",
        prompt="Calculate 15 + 27 using the add_numbers tool and explain the result briefly.",
        agent_id="simple_agent",
        tools=[add_numbers],
    )
    workflow.add_node(agent_node)
    workflow.validate()

    # Execute with tracing
    executor = Executor(llm_config)
    traced_executor = tracer.wrap_executor(executor, llm_config)
    result = await traced_executor.execute(workflow)

    # Send traces to API and cleanup
    results = await tracer.send_to_api()
    await tracer.shutdown()

    return result


if __name__ == "__main__":
    asyncio.run(run_agent())
"#.to_string()
}


fn get_comprehensive_main_content() -> String {
    r#""""Main entry point for GraphBit project.

Choose which pattern to run by uncommenting the desired import and function call.
All patterns are included - implement the one you need!
"""

import os
from dotenv import load_dotenv
import asyncio

# Load environment variables from .env file
load_dotenv()

# Choose your pattern - uncomment ONE of these:

# 1. Simple Agent Pattern (direct LLM interaction)
# from agent import run_agent

# 2. Workflow Pattern (multi-step processes)
# from workflow import run_workflow

# 3. Workflow with Tools Pattern (agents with external capabilities)
# from workflow_with_tools import run_workflow_with_tools

def main():
    """Main function - choose your pattern and uncomment the corresponding line."""
    # Check environment
    if not os.getenv("OPENAI_API_KEY"):
        print("⚠️  OPENAI_API_KEY not found in environment")
        print("Copy .env.example to .env and add your API keys")
        return

    print("✅ GraphBit Project Ready!")
    print("Choose your pattern in main.py:")
    print("   1. Agent: Uncomment 'from agent import run_agent' and 'asyncio.run(run_agent())'")
    print("   2. Workflow: Uncomment 'from workflow import run_workflow' and 'asyncio.run(run_workflow())'")
    print("   3. Tools: Uncomment 'from workflow_with_tools import run_workflow_with_tools' and 'asyncio.run(run_workflow_with_tools())'")
    print("   Note: All patterns include optional GraphBit Tracer integration")

    # Uncomment ONE of these based on your chosen pattern:
    # All patterns are now async and include optional GraphBit Tracer integration


    # result = asyncio.run(run_agent())
    # result = asyncio.run(run_workflow())
    # result = asyncio.run(run_workflow_with_tools())

    # print(f"Result: {result}")

if __name__ == "__main__":
    main()
"#.to_string()
}



/// Get workflow content
fn get_workflow_content() -> String {
    r#""""GraphBit workflow implementation with optional GraphBit Tracer integration."""

import asyncio
import os
from graphbit import LlmConfig, Executor, Workflow, Node
from graphbit_tracer import AutoTracer


async def run_workflow():
    """Run a multi-node workflow with optional tracer integration."""
    print("=" * 50)
    print("GraphBit Workflow Example")
    print("=" * 50)

    # Initialize tracer
    tracer = await AutoTracer.create()

    # Configure LLM
    api_key = os.getenv("OPENAI_API_KEY")

    llm_config = LlmConfig.openai(api_key, "gpt-4o-mini")

    # Create multi-node workflow
    workflow = Workflow("ContentWorkflow")

    # Node 1: Generate ideas
    idea_node = Node.agent(
        name="IdeaGenerator",
        prompt="Generate 3 creative ideas for a blog post about technology trends.",
        agent_id="idea_generator"
    )
    workflow.add_node(idea_node)

    # Node 2: Select best idea
    selector_node = Node.agent(
        name="IdeaSelector",
        prompt="From these ideas:\n{{IdeaGenerator}}\n\nChoose the most interesting one and explain why in 2 sentences.",
        agent_id="idea_selector"
    )
    workflow.add_node(selector_node)

    workflow.validate()

    # Execute with tracing
    executor = Executor(llm_config)
    traced_executor = tracer.wrap_executor(executor, llm_config)
    result = await traced_executor.execute(workflow)

    # Send traces to API and cleanup
    results = await tracer.send_to_api()
    await tracer.shutdown()

    return result


if __name__ == "__main__":
    asyncio.run(run_workflow())
"#.to_string()
}

/// Get workflow with tools content
fn get_workflow_with_tools_content() -> String {
    r#""""GraphBit workflow with tools implementation and optional GraphBit Tracer integration."""

import asyncio
import os
from graphbit import LlmConfig, Executor, Workflow, Node, tool
from graphbit_tracer import AutoTracer


# Define tools
@tool(_description="Calculate the sum of two numbers")
def add_numbers(a: float, b: float) -> float:
    """Add two numbers and return the result."""
    return a + b


@tool(_description="Calculate the product of two numbers")
def multiply_numbers(a: float, b: float) -> float:
    """Multiply two numbers and return the result."""
    return a * b


async def run_workflow_with_tools():
    """Run a workflow with tools and optional tracer integration."""
    print("=" * 50)
    print("GraphBit Workflow with Tools Example")
    print("=" * 50)

    # Initialize tracer
    tracer = await AutoTracer.create()

    # Configure LLM
    api_key = os.getenv("OPENAI_API_KEY")

    llm_config = LlmConfig.openai(api_key, "gpt-4o-mini")

    # Create workflow with tools
    workflow = Workflow("MathWorkflow")

    # Node with math tools
    math_node = Node.agent(
        name="MathAgent",
        prompt="Calculate (15 + 27) * 2 using the available tools. Show your work step by step.",
        agent_id="math_agent",
        tools=[add_numbers, multiply_numbers],
    )
    workflow.add_node(math_node)

    workflow.validate()

    # Execute with tracing
    executor = Executor(llm_config)
    traced_executor = tracer.wrap_executor(executor, llm_config)
    result = await traced_executor.execute(workflow)

    # Send traces to API and cleanup
    results = await tracer.send_to_api()
    await tracer.shutdown()

    return result


if __name__ == "__main__":
    asyncio.run(run_workflow_with_tools())
"#.to_string()
}

/// Get tools module content
fn get_tools_content() -> String {
    r#""""Tools for GraphBit agents."""

from graphbit import tool

# TODO: Define your tools here
# @tool(_description="Description of what your tool does")
# def your_tool(param: str) -> str:
#     """Your tool implementation."""
#     return "result"
"#.to_string()
}



