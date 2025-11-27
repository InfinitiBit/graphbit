//! Validation bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::validation::ValidationResult as CoreValidationResult;

/// Validation result
#[napi(object)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

impl From<CoreValidationResult> for ValidationResult {
    fn from(result: CoreValidationResult) -> Self {
        Self {
            valid: result.is_valid,
            errors: result.errors.iter().map(|e| e.message.clone()).collect(),
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

