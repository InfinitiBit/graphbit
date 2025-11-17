#!/usr/bin/env python3
"""
GraphBit CLI - Standalone version
A command-line interface for GraphBit agent development and deployment.
"""

import os
from pathlib import Path
from typing import Optional
import typer
from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.progress import Progress, SpinnerColumn, TextColumn
import time

# Initialize Rich console
console = Console()

# Create the main Typer app
app = typer.Typer(
    name="graphbit",
    help="GraphBit CLI - Build, run, and deploy AI agents with ease",
    rich_markup_mode="rich",
    no_args_is_help=True,
)

# Check if CLI functionality is available
CLI_AVAILABLE = True
try:
    import graphbit
    from graphbit import init_project, run_agent, deploy_to_e2b
except ImportError as e:
    CLI_AVAILABLE = False
    console.print(f"⚠️ Warning: GraphBit CLI functionality not fully available: {e}", style="yellow")

@app.command()
def init(
    project_name: str = typer.Argument(..., help="Name of the project to create"),
    target_dir: Optional[str] = typer.Option(None, "--dir", "-d", help="Target directory (default: current directory)"),
    template: Optional[str] = typer.Option("basic", "--template", "-t", help="Project template to use"),
):
    """Initialize a new GraphBit project with standardized structure."""
    
    if not CLI_AVAILABLE:
        console.print("❌ CLI functionality not available. Please install GraphBit with CLI support.", style="red")
        raise typer.Exit(1)
    
    console.print(f"Initializing GraphBit project: [bold blue]{project_name}[/bold blue]")
    
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console,
        transient=True,
    ) as progress:
        task = progress.add_task("Creating project structure...", total=None)
        
        try:
            # Call the Rust backend function
            result = init_project(project_name, target_dir, template)
            
            # Display success message
            console.print("✅ Project created successfully!")
            
            # Create a nice panel with project info
            panel_content = f"""✅ Successfully created GraphBit project!

Project: [bold blue]{project_name}[/bold blue]
Location: [bold green]{result.get('project_path', './'+project_name)}[/bold green]
Template: [bold yellow]{template}[/bold yellow]
Files created: [bold cyan]{result.get('files_created', 0)}[/bold cyan]

Next steps:
1. [bold]cd {project_name}[/bold]
2. [bold]cp .env.example .env[/bold]
3. [bold]Edit .env with your API keys[/bold]
4. [bold]pip install -r requirements.txt[/bold]
5. [bold]python main.py[/bold]"""
            
            console.print(Panel(panel_content, title="✅ Project Created", border_style="green"))
            
        except Exception as e:
            console.print(f"❌ Error creating project: {e}", style="red")
            raise typer.Exit(1)

@app.command()
def run(
    file_path: Optional[str] = typer.Argument(None, help="Path to the Python file (default: main.py)"),
    config: Optional[str] = typer.Option(None, "--config", "-c", help="Configuration file path"),
    env_file: Optional[str] = typer.Option(None, "--env", "-e", help="Environment file path"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Enable verbose output"),
):
    """Run a GraphBit agent or workflow locally."""
    
    if not CLI_AVAILABLE:
        console.print("❌ CLI functionality not available. Please install GraphBit with CLI support.", style="red")
        raise typer.Exit(1)
    
    # Default to main.py if no file specified
    if file_path is None:
        file_path = "main.py"
    
    console.print(f"Running GraphBit file: [bold blue]{file_path}[/bold blue]")
    
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console,
        transient=True,
    ) as progress:
        task = progress.add_task("Executing...", total=None)
        
        try:
            start_time = time.time()
            
            # Call the Rust backend function
            result = run_agent(file_path, config, env_file, verbose)
            
            end_time = time.time()
            execution_time = end_time - start_time
            
            # Display results
            console.print("✅ Execution completed!")
            
            # Create results panel
            panel_content = f"""✅ Execution completed successfully!

File: [bold blue]{file_path}[/bold blue]
Exit Code: [bold green]{result.get('exit_code', 0)}[/bold green]
Execution Time: [bold cyan]{execution_time:.2f}s[/bold cyan]

Output:
{result.get('output', 'No output captured')}"""
            
            console.print(Panel(panel_content, title="✅ Execution Results", border_style="green"))
            
        except Exception as e:
            console.print(f"❌ Error running agent: {e}", style="red")
            raise typer.Exit(1)

@app.command()
def deploy(
    project_dir: Optional[str] = typer.Option(None, "--dir", "-d", help="Project directory (default: current directory)"),
    template: Optional[str] = typer.Option("python", "--template", "-t", help="E2B template to use"),
    env_file: Optional[str] = typer.Option(None, "--env", "-e", help="Environment file path"),
    timeout: int = typer.Option(300, "--timeout", help="Deployment timeout in seconds"),
):
    """Deploy GraphBit agent/workflow to E2B platform."""
    
    if not CLI_AVAILABLE:
        console.print("❌ CLI functionality not available. Please install GraphBit with CLI support.", style="red")
        raise typer.Exit(1)
    
    console.print("Deploying GraphBit project to E2B...")
    
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console,
        transient=True,
    ) as progress:
        task = progress.add_task("Deploying to E2B...", total=None)
        
        try:
            # Call the Rust backend function
            result = deploy_to_e2b(project_dir, template, env_file, timeout)
            
            # Display results
            console.print("✅ Deployment completed!")
            
            # Create results panel
            panel_content = f"""✅ Successfully deployed to E2B!

Sandbox ID: [bold blue]{result.get('sandbox_id', 'N/A')}[/bold blue]
Template: [bold yellow]{template}[/bold yellow]
Status: [bold green]{result.get('status', 'Deployed')}[/bold green]

Access your deployed agent at:
[bold cyan]{result.get('url', 'Check E2B dashboard')}[/bold cyan]"""
            
            console.print(Panel(panel_content, title="✅ Deployment Complete", border_style="green"))
            
        except Exception as e:
            console.print(f"❌ Error deploying to E2B: {e}", style="red")
            raise typer.Exit(1)

@app.command()
def info(
    project_dir: Optional[str] = typer.Option(None, "--dir", "-d", help="Project directory (default: current directory)"),
):
    """Display information about the current GraphBit project."""
    
    project_path = Path(project_dir) if project_dir else Path.cwd()
    console.print(f"GraphBit Project Information: [bold blue]{project_path.absolute()}[/bold blue]")
    
    # Create project structure table
    table = Table(title="Project Structure")
    table.add_column("Component", style="cyan", no_wrap=True)
    table.add_column("Count", style="magenta")
    table.add_column("Status", style="green")
    
    # Check for different components
    components = {
        "Agents": (list(project_path.glob("**/agent*.py")), "❌ Missing"),
        "Workflows": (list(project_path.glob("**/workflow*.py")), "❌ Missing"),
        "Tools": (list(project_path.glob("**/tool*.py")), "❌ Missing"),
        "Data Files": (list(project_path.glob("data/*")), "❌ Missing"),
        "Requirements": (list(project_path.glob("requirements.txt")), "❌ Missing"),
        "Environment": (list(project_path.glob(".env*")), "❌ Missing"),
        "Main Script": (list(project_path.glob("main.py")), "❌ Missing"),
    }
    
    for component, (files, default_status) in components.items():
        count = len(files)
        status = "✅ Found" if count > 0 else default_status
        table.add_row(component, str(count), status)
    
    console.print(table)
    
    # Check environment variables
    env_file = project_path / ".env"
    if env_file.exists():
        console.print("\nEnvironment Configuration:")
        
        # Check for common API keys
        api_keys = ["OPENAI_API_KEY", "ANTHROPIC_API_KEY", "E2B_API_KEY"]
        
        try:
            with open(env_file, 'r') as f:
                env_content = f.read()
            
            for key in api_keys:
                if key in env_content and not env_content.split(key + "=")[1].split('\n')[0].strip() == "":
                    console.print(f"  ✅ {key}: Configured")
                else:
                    console.print(f"  ❌ {key}: Not configured")
        except Exception as e:
            console.print(f"  ⚠️ Error reading .env file: {e}")

@app.command()
def version():
    """Show GraphBit version information."""
    try:
        import graphbit
        version = getattr(graphbit, '__version__', '0.5.1')
        console.print(f"GraphBit CLI version: [bold green]{version}[/bold green]")
    except ImportError:
        console.print("GraphBit CLI version: [bold yellow]Unknown (GraphBit not installed)[/bold yellow]")

def main():
    """Main entry point for the CLI."""
    app()

if __name__ == "__main__":
    main()
