//! Validation bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::validation::ValidationError as CoreValidationError;
use graphbit_core::validation::ValidationResult as CoreValidationResult;

/// Validation error details
#[napi(object)]
pub struct ValidationError {
    /// Field path that failed validation
    pub field_path: String,
    /// Error message
    pub message: String,
    /// Expected value or type
    pub expected: Option<String>,
    /// Actual value that failed validation
    pub actual: Option<String>,
    /// Error code for programmatic handling
    pub error_code: String,
}

impl From<CoreValidationError> for ValidationError {
    fn from(error: CoreValidationError) -> Self {
        Self {
            field_path: error.field_path,
            message: error.message,
            expected: error.expected,
            actual: error.actual,
            error_code: error.error_code,
        }
    }
}

/// Validation result
#[napi(object)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    #[napi(ts_type = "Record<string, any>")]
    pub metadata: serde_json::Value,
}

impl From<CoreValidationResult> for ValidationResult {
    fn from(result: CoreValidationResult) -> Self {
        Self {
            is_valid: result.is_valid,
            errors: result.errors.into_iter().map(ValidationError::from).collect(),
            metadata: serde_json::to_value(&result.metadata).unwrap_or(serde_json::Value::Null),
        }
    }
}

/// Validate JSON against a schema
#[napi]
pub fn validate_json(data: String, schema: String) -> Result<ValidationResult> {
    let schema_value: serde_json::Value = serde_json::from_str(&schema)
        .map_err(|e| Error::from_reason(format!("Invalid JSON schema: {}", e)))?;

    let validator = graphbit_core::validation::TypeValidator::new();
    let result = validator.validate_against_schema(&data, &schema_value);

    Ok(ValidationResult::from(result))
}

