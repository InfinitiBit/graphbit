//! Local execution functionality for GraphBit CLI

use super::{CliError, to_py_cli_error};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
use std::process::Command;

/// Run a GraphBit agent or workflow locally
#[pyfunction]
#[pyo3(signature = (file_path, config_file=None, env_file=None, verbose=false))]
pub fn run_agent(
    py: Python<'_>,
    file_path: String,
    config_file: Option<String>,
    env_file: Option<String>,
    verbose: bool,
) -> PyResult<Bound<'_, PyDict>> {
    let result = PyDict::new(py);
    
    // Validate file path
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(to_py_cli_error(CliError::InvalidPath(
            format!("File not found: {}", file_path),
        )));
    }
    
    if !path.extension().map_or(false, |ext| ext == "py") {
        return Err(to_py_cli_error(CliError::InvalidPath(
            "Only Python files (.py) are supported".to_string(),
        )));
    }
    
    // Execute the file
    match execute_python_file(&file_path, config_file.as_deref(), env_file.as_deref(), verbose) {
        Ok(execution_result) => {
            result.set_item("success", true)?;
            result.set_item("file_path", file_path)?;
            result.set_item("output", execution_result.output)?;
            result.set_item("exit_code", execution_result.exit_code)?;
            result.set_item("execution_time", execution_result.execution_time)?;
            
            if !execution_result.error.is_empty() {
                result.set_item("error", execution_result.error)?;
            }
        }
        Err(err) => {
            return Err(to_py_cli_error(err));
        }
    }
    
    Ok(result)
}

/// Execution result structure
struct ExecutionResult {
    output: String,
    error: String,
    exit_code: i32,
    execution_time: f64,
}

/// Execute a Python file with optional configuration
fn execute_python_file(
    file_path: &str,
    config_file: Option<&str>,
    env_file: Option<&str>,
    verbose: bool,
) -> Result<ExecutionResult, CliError> {
    let start_time = std::time::Instant::now();
    
    // Build command
    let mut cmd = Command::new("python");
    cmd.arg(file_path);
    
    // Set environment variables if env_file is provided
    if let Some(env_path) = env_file {
        if Path::new(env_path).exists() {
            // Load .env file and set environment variables
            if let Ok(env_content) = std::fs::read_to_string(env_path) {
                for line in env_content.lines() {
                    if let Some((key, value)) = parse_env_line(line) {
                        cmd.env(key, value);
                    }
                }
            }
        }
    } else {
        // Try to load default .env file
        if Path::new(".env").exists() {
            if let Ok(env_content) = std::fs::read_to_string(".env") {
                for line in env_content.lines() {
                    if let Some((key, value)) = parse_env_line(line) {
                        cmd.env(key, value);
                    }
                }
            }
        }
    }
    
    // Add config file as environment variable if provided
    if let Some(config_path) = config_file {
        cmd.env("GRAPHBIT_CONFIG_FILE", config_path);
    }
    
    // Set verbose mode
    if verbose {
        cmd.env("GRAPHBIT_VERBOSE", "true");
    }
    
    // Execute command
    let output = cmd.output().map_err(|e| CliError::IoError(e))?;
    
    let execution_time = start_time.elapsed().as_secs_f64();
    
    Ok(ExecutionResult {
        output: String::from_utf8_lossy(&output.stdout).to_string(),
        error: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
        execution_time,
    })
}

/// Parse a line from .env file
pub fn parse_env_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    
    // Skip comments and empty lines
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    
    // Split on first '=' character
    if let Some(eq_pos) = line.find('=') {
        let key = line[..eq_pos].trim().to_string();
        let value = line[eq_pos + 1..].trim();
        
        // Remove quotes if present
        let value = if (value.starts_with('"') && value.ends_with('"')) ||
                      (value.starts_with('\'') && value.ends_with('\'')) {
            value[1..value.len()-1].to_string()
        } else {
            value.to_string()
        };
        
        Some((key, value))
    } else {
        None
    }
}

/// Validate GraphBit project structure
pub fn validate_project_structure(project_dir: &Path) -> Result<(), CliError> {
    // Check for main.py (required for simplified structure)
    let main_py_path = project_dir.join("main.py");
    if !main_py_path.exists() {
        return Err(CliError::InvalidPath(
            "Missing main.py file - this doesn't appear to be a GraphBit project".to_string(),
        ));
    }

    // Check for requirements.txt
    let requirements_path = project_dir.join("requirements.txt");
    if !requirements_path.exists() {
        return Err(CliError::InvalidPath(
            "Missing requirements.txt file".to_string(),
        ));
    }

    Ok(())
}


