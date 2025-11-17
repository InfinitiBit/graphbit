//! CLI module for GraphBit
//!
//! This module provides command-line interface functionality for GraphBit,
//! including project initialization, local execution, and E2B deployment.

use pyo3::prelude::*;
use std::fs;
use std::path::Path;

mod init;
mod run;
mod deploy;

pub(crate) use init::init_project;
pub(crate) use run::run_agent;
pub(crate) use deploy::deploy_to_e2b;

/// CLI error types
#[derive(Debug)]
pub(crate) enum CliError {
    IoError(std::io::Error),
    ProjectExists(String),
    InvalidPath(String),
    TemplateError(String),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::IoError(err) => write!(f, "IO error: {}", err),
            CliError::ProjectExists(path) => write!(f, "Project already exists at: {}", path),
            CliError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            CliError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

/// Convert CLI error to Python exception
pub(crate) fn to_py_cli_error(err: CliError) -> PyErr {
    use pyo3::exceptions::{PyFileExistsError, PyIOError, PyValueError};
    
    match err {
        CliError::IoError(io_err) => PyIOError::new_err(io_err.to_string()),
        CliError::ProjectExists(path) => PyFileExistsError::new_err(format!("Project already exists at: {}", path)),
        CliError::InvalidPath(path) => PyValueError::new_err(format!("Invalid path: {}", path)),
        CliError::TemplateError(msg) => PyValueError::new_err(format!("Template error: {}", msg)),
    }
}

/// Utility function to create directory if it doesn't exist
pub(crate) fn ensure_dir(path: &Path) -> Result<(), CliError> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Utility function to write file with content
pub(crate) fn write_file(path: &Path, content: &str) -> Result<(), CliError> {
    fs::write(path, content)?;
    Ok(())
}
