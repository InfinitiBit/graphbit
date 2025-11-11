//! E2B deployment functionality for GraphBit CLI

use super::{CliError, to_py_cli_error};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
use std::process::Command;

/// Deploy GraphBit project to E2B sandbox
#[pyfunction]
#[pyo3(signature = (project_dir=None, template=None, env_file=None, timeout=300))]
pub fn deploy_to_e2b(
    py: Python<'_>,
    project_dir: Option<String>,
    template: Option<String>,
    env_file: Option<String>,
    timeout: u64,
) -> PyResult<Bound<'_, PyDict>> {
    let result = PyDict::new(py);
    
    // Determine project directory
    let project_path = project_dir.unwrap_or_else(|| ".".to_string());
    let path = Path::new(&project_path);
    
    if !path.exists() {
        return Err(to_py_cli_error(CliError::InvalidPath(
            format!("Project directory not found: {}", project_path),
        )));
    }
    
    // Validate project structure
    if let Err(err) = super::run::validate_project_structure(path) {
        return Err(to_py_cli_error(err));
    }
    
    // Check E2B API key
    let api_key = get_e2b_api_key(env_file.as_deref())?;
    
    // Deploy to E2B
    match deploy_project(path, &api_key, template.as_deref(), timeout) {
        Ok(deployment_result) => {
            result.set_item("success", true)?;
            result.set_item("project_path", project_path)?;
            result.set_item("sandbox_id", deployment_result.sandbox_id)?;
            result.set_item("sandbox_url", deployment_result.sandbox_url)?;
            result.set_item("deployment_time", deployment_result.deployment_time)?;
            
            if let Some(logs) = deployment_result.logs {
                result.set_item("logs", logs)?;
            }
        }
        Err(err) => {
            return Err(to_py_cli_error(err));
        }
    }
    
    Ok(result)
}

/// Deployment result structure
struct DeploymentResult {
    sandbox_id: String,
    sandbox_url: String,
    deployment_time: f64,
    logs: Option<String>,
}

/// Deploy project to E2B
fn deploy_project(
    project_path: &Path,
    api_key: &str,
    template: Option<&str>,
    _timeout: u64,
) -> Result<DeploymentResult, CliError> {
    let start_time = std::time::Instant::now();
    
    // Create deployment script
    let deploy_script = create_deployment_script(project_path, api_key, template)?;
    
    // Write deployment script to temporary file
    let temp_script_path = project_path.join("_deploy_temp.py");
    std::fs::write(&temp_script_path, deploy_script)
        .map_err(|e| CliError::IoError(e))?;
    
    // Execute deployment script
    let mut cmd = Command::new("python");
    cmd.arg(&temp_script_path);
    cmd.env("E2B_API_KEY", api_key);
    cmd.current_dir(project_path);
    
    let output = cmd.output().map_err(|e| CliError::IoError(e))?;
    
    // Clean up temporary script
    let _ = std::fs::remove_file(&temp_script_path);
    
    let deployment_time = start_time.elapsed().as_secs_f64();
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(CliError::TemplateError(
            format!("Deployment failed: {}", error_msg),
        ));
    }
    
    // Parse deployment output
    let output_str = String::from_utf8_lossy(&output.stdout);
    let deployment_info = parse_deployment_output(&output_str)?;
    
    Ok(DeploymentResult {
        sandbox_id: deployment_info.sandbox_id,
        sandbox_url: deployment_info.sandbox_url,
        deployment_time,
        logs: Some(output_str.to_string()),
    })
}

/// Deployment information from output
struct DeploymentInfo {
    sandbox_id: String,
    sandbox_url: String,
}

/// Parse deployment script output
fn parse_deployment_output(output: &str) -> Result<DeploymentInfo, CliError> {
    let mut sandbox_id = None;
    let mut sandbox_url = None;
    
    for line in output.lines() {
        if line.starts_with("SANDBOX_ID:") {
            sandbox_id = Some(line.strip_prefix("SANDBOX_ID:").unwrap().trim().to_string());
        } else if line.starts_with("SANDBOX_URL:") {
            sandbox_url = Some(line.strip_prefix("SANDBOX_URL:").unwrap().trim().to_string());
        }
    }
    
    match (sandbox_id, sandbox_url) {
        (Some(id), Some(url)) => Ok(DeploymentInfo {
            sandbox_id: id,
            sandbox_url: url,
        }),
        _ => Err(CliError::TemplateError(
            "Failed to parse deployment output".to_string(),
        )),
    }
}

/// Create deployment script content
fn create_deployment_script(
    _project_path: &Path,
    api_key: &str,
    template: Option<&str>,
) -> Result<String, CliError> {
    let template_name = template.unwrap_or("python");
    
    let script = format!(r#"#!/usr/bin/env python3
"""
Temporary deployment script for GraphBit project.
This script is auto-generated and will be deleted after deployment.
"""

import os
import sys
import json
import zipfile
import tempfile
from pathlib import Path

def main():
    try:
        # Import E2B SDK
        from e2b_code_interpreter import Sandbox
    except ImportError:
        print("ERROR: e2b-code-interpreter package not found")
        print("Please install it with: pip install e2b-code-interpreter")
        sys.exit(1)
    
    # Set API key
    os.environ['E2B_API_KEY'] = '{api_key}'
    
    print("🚀 Starting E2B deployment...")
    
    # Create sandbox
    print("📦 Creating E2B sandbox...")
    try:
        sandbox = Sandbox.create(template='{template_name}')
        print(f"✅ Sandbox created: {{sandbox.id}}")
        
        # Upload project files
        print("📤 Uploading project files...")
        upload_project_files(sandbox)
        
        # Install dependencies
        print("📦 Installing dependencies...")
        install_dependencies(sandbox)
        
        # Run initial setup
        print("🔧 Running project setup...")
        setup_result = run_project_setup(sandbox)
        
        # Output deployment information
        print(f"SANDBOX_ID:{{sandbox.id}}")
        print(f"SANDBOX_URL:https://{{sandbox.id}}.e2b.dev")
        
        print("✅ Deployment completed successfully!")
        print(f"🌐 Access your sandbox at: https://{{sandbox.id}}.e2b.dev")
        
    except Exception as e:
        print(f"❌ Deployment failed: {{str(e)}}")
        sys.exit(1)

def upload_project_files(sandbox):
    """Upload all project files to the sandbox."""
    project_root = Path('.')
    
    # Files and directories to upload
    items_to_upload = [
        'agents/',
        'workflows/', 
        'tools/',
        'data/',
        'main.py',
        'requirements.txt'
    ]
    
    for item in items_to_upload:
        item_path = project_root / item
        if item_path.exists():
            if item_path.is_file():
                # Upload file
                with open(item_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                sandbox.filesystem.write(f'/home/user/{{item}}', content)
                print(f"  📄 Uploaded {{item}}")
            elif item_path.is_dir():
                # Upload directory recursively
                upload_directory(sandbox, item_path, f'/home/user/{{item}}')
                print(f"  📁 Uploaded {{item}}")

def upload_directory(sandbox, local_dir, remote_dir):
    """Recursively upload a directory."""
    for item in local_dir.rglob('*'):
        if item.is_file() and not item.name.startswith('.'):
            relative_path = item.relative_to(local_dir)
            remote_path = f'{{remote_dir}}/{{relative_path}}'
            
            try:
                with open(item, 'r', encoding='utf-8') as f:
                    content = f.read()
                sandbox.filesystem.write(remote_path, content)
            except UnicodeDecodeError:
                # Skip binary files
                pass

def install_dependencies(sandbox):
    """Install Python dependencies in the sandbox."""
    # Install requirements
    result = sandbox.run_code("""
import subprocess
import sys

# Install requirements
try:
    result = subprocess.run([sys.executable, '-m', 'pip', 'install', '-r', 'requirements.txt'], 
                          capture_output=True, text=True, cwd='/home/user')
    print("STDOUT:", result.stdout)
    if result.stderr:
        print("STDERR:", result.stderr)
    print("Return code:", result.returncode)
except Exception as e:
    print(f"Error installing requirements: {{e}}")
""")
    
    if result.error:
        raise Exception(f"Failed to install dependencies: {{result.error}}")

def run_project_setup(sandbox):
    """Run initial project setup."""
    # Test if main.py can be imported
    result = sandbox.run_code("""
import sys
sys.path.append('/home/user')

try:
    # Test import
    import main
    print("✅ Project setup successful - main.py can be imported")
    
    # Try to run main function if it exists
    if hasattr(main, 'main'):
        print("🚀 Running main function...")
        main.main()
    else:
        print("ℹ️  No main() function found in main.py")
        
except Exception as e:
    print(f"⚠️  Setup warning: {{e}}")
    print("This might be normal if the project requires specific configuration.")
""")
    
    return result

if __name__ == "__main__":
    main()
"#, api_key = api_key, template_name = template_name);
    
    Ok(script)
}

/// Get E2B API key from environment or .env file
fn get_e2b_api_key(env_file: Option<&str>) -> PyResult<String> {
    // First try environment variable
    if let Ok(api_key) = std::env::var("E2B_API_KEY") {
        if !api_key.is_empty() {
            return Ok(api_key);
        }
    }
    
    // Try to load from .env file
    let env_path = env_file.unwrap_or(".env");
    if Path::new(env_path).exists() {
        if let Ok(env_content) = std::fs::read_to_string(env_path) {
            for line in env_content.lines() {
                if let Some((key, value)) = super::run::parse_env_line(line) {
                    if key == "E2B_API_KEY" && !value.is_empty() {
                        return Ok(value);
                    }
                }
            }
        }
    }
    
    Err(to_py_cli_error(CliError::TemplateError(
        "E2B_API_KEY not found. Please set it in your environment or .env file".to_string(),
    )))
}
