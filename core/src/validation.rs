//! Type validation system for `GraphBit`
//!
//! This module provides comprehensive validation of LLM responses and data
//! against JSON schemas and custom validation rules.

use crate::errors::GraphBitResult;
use regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of a validation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    /// Validation metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Create a failed validation result
    pub fn failure(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            metadata: HashMap::with_capacity(4),
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Add an error to the result
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
    }

    /// Merge another validation result into this one
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.metadata.extend(other.metadata);
    }
}

/// A validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ValidationError {
    /// Create a new validation error
    pub fn new(
        field_path: impl Into<String>,
        message: impl Into<String>,
        error_code: impl Into<String>,
    ) -> Self {
        Self {
            field_path: field_path.into(),
            message: message.into(),
            expected: None,
            actual: None,
            error_code: error_code.into(),
        }
    }

    /// Add expected value information
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Add actual value information
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }
}

/// A validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Name of the rule
    pub name: String,
    /// Description of what the rule validates
    pub description: String,
    /// JSON schema for validation (if applicable)
    pub schema: Option<serde_json::Value>,
    /// Custom validation function name
    pub validator_function: Option<String>,
    /// Rule configuration
    pub config: HashMap<String, serde_json::Value>,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            schema: None,
            validator_function: None,
            config: HashMap::with_capacity(4),
        }
    }

    /// Add JSON schema validation
    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Add custom validator function
    pub fn with_validator(mut self, validator_function: impl Into<String>) -> Self {
        self.validator_function = Some(validator_function.into());
        self
    }

    /// Add configuration
    pub fn with_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.insert(key, value);
        self
    }
}

/// Type validator for validating data against schemas and rules
pub struct TypeValidator {
    /// Registered custom validators
    custom_validators: HashMap<String, Box<dyn CustomValidator>>,
}

impl TypeValidator {
    /// Create a new type validator
    pub fn new() -> Self {
        Self {
            custom_validators: HashMap::with_capacity(8),
        }
    }

    /// Register a custom validator
    pub fn register_validator(
        &mut self,
        name: String,
        validator: Box<dyn CustomValidator>,
    ) -> GraphBitResult<()> {
        self.custom_validators.insert(name, validator);
        Ok(())
    }

    /// Validate data against a JSON schema
    pub fn validate_against_schema(
        &self,
        data: &str,
        schema: &serde_json::Value,
    ) -> ValidationResult {
        // Parse the data as JSON
        let json_data = match serde_json::from_str::<serde_json::Value>(data) {
            Ok(value) => value,
            Err(e) => {
                return ValidationResult::failure(vec![ValidationError::new(
                    "root",
                    format!("Invalid JSON: {}", e),
                    "INVALID_JSON",
                )]);
            }
        };

        self.validate_json_against_schema(&json_data, schema, "root")
    }

    /// Validate JSON value against schema
    #[allow(clippy::only_used_in_recursion)]
    fn validate_json_against_schema(
        &self,
        data: &serde_json::Value,
        schema: &serde_json::Value,
        path: &str,
    ) -> ValidationResult {
        let mut result = ValidationResult::success();

        // Extract schema properties
        let schema_obj = match schema.as_object() {
            Some(obj) => obj,
            None => {
                result.add_error(ValidationError::new(
                    path,
                    "Schema must be an object",
                    "INVALID_SCHEMA",
                ));
                return result;
            }
        };

        // Check type
        if let Some(expected_type) = schema_obj.get("type").and_then(|t| t.as_str()) {
            let actual_type = match data {
                serde_json::Value::Null => "null",
                serde_json::Value::Bool(_) => "boolean",
                serde_json::Value::Number(_) => "number",
                serde_json::Value::String(_) => "string",
                serde_json::Value::Array(_) => "array",
                serde_json::Value::Object(_) => "object",
            };

            if expected_type != actual_type {
                result.add_error(
                    ValidationError::new(path, "Type mismatch".to_string(), "TYPE_MISMATCH")
                        .with_expected(expected_type)
                        .with_actual(actual_type),
                );
                return result;
            }
        }

        // Validate object properties
        if let (Some(data_obj), Some(properties)) = (
            data.as_object(),
            schema_obj.get("properties").and_then(|p| p.as_object()),
        ) {
            // Check required properties
            if let Some(required) = schema_obj.get("required").and_then(|r| r.as_array()) {
                for req_prop in required {
                    if let Some(prop_name) = req_prop.as_str() {
                        if !data_obj.contains_key(prop_name) {
                            result.add_error(ValidationError::new(
                                format!("{}.{}", path, prop_name),
                                format!("Required property '{}' is missing", prop_name),
                                "MISSING_REQUIRED_PROPERTY",
                            ));
                        }
                    }
                }
            }

            // Validate each property
            for (prop_name, prop_schema) in properties {
                if let Some(prop_value) = data_obj.get(prop_name) {
                    let prop_path = format!("{}.{}", path, prop_name);
                    let prop_result =
                        self.validate_json_against_schema(prop_value, prop_schema, &prop_path);
                    result.merge(prop_result);
                }
            }
        }

        // Validate array items
        if let (Some(data_array), Some(items_schema)) = (data.as_array(), schema_obj.get("items")) {
            for (index, item) in data_array.iter().enumerate() {
                let item_path = format!("{}[{}]", path, index);
                let item_result = self.validate_json_against_schema(item, items_schema, &item_path);
                result.merge(item_result);
            }
        }

        // Validate string constraints
        if let Some(data_str) = data.as_str() {
            if let Some(min_length) = schema_obj.get("minLength").and_then(|l| l.as_u64()) {
                if (data_str.len() as u64) < min_length {
                    result.add_error(ValidationError::new(
                        path,
                        format!("String too short (min: {})", min_length),
                        "STRING_TOO_SHORT",
                    ));
                }
            }

            if let Some(max_length) = schema_obj.get("maxLength").and_then(|l| l.as_u64()) {
                if (data_str.len() as u64) > max_length {
                    result.add_error(ValidationError::new(
                        path,
                        format!("String too long (max: {})", max_length),
                        "STRING_TOO_LONG",
                    ));
                }
            }

            if let Some(pattern) = schema_obj.get("pattern").and_then(|p| p.as_str()) {
                // Use regex for proper pattern matching
                match regex::Regex::new(pattern) {
                    Ok(re) => {
                        if !re.is_match(data_str) {
                            result.add_error(ValidationError::new(
                                path,
                                format!("String does not match pattern: {}", pattern),
                                "PATTERN_MISMATCH",
                            ));
                        }
                    }
                    Err(e) => {
                        result.add_error(ValidationError::new(
                            path,
                            format!("Invalid regex pattern {}: {}", pattern, e),
                            "INVALID_REGEX_PATTERN",
                        ));
                    }
                }
            }
        }

        // Validate number constraints
        if let Some(data_num) = data.as_f64() {
            if let Some(minimum) = schema_obj.get("minimum").and_then(|m| m.as_f64()) {
                if data_num < minimum {
                    result.add_error(ValidationError::new(
                        path,
                        format!("Number too small (min: {})", minimum),
                        "NUMBER_TOO_SMALL",
                    ));
                }
            }

            if let Some(maximum) = schema_obj.get("maximum").and_then(|m| m.as_f64()) {
                if data_num > maximum {
                    result.add_error(ValidationError::new(
                        path,
                        format!("Number too large (max: {})", maximum),
                        "NUMBER_TOO_LARGE",
                    ));
                }
            }
        }

        result
    }

    /// Validate data against a validation rule
    pub fn validate_against_rule(&self, data: &str, rule: &ValidationRule) -> ValidationResult {
        let mut result = ValidationResult::success();

        // Schema validation
        if let Some(schema) = &rule.schema {
            let schema_result = self.validate_against_schema(data, schema);
            result.merge(schema_result);
        }

        // Custom validator
        if let Some(validator_name) = &rule.validator_function {
            if let Some(validator) = self.custom_validators.get(validator_name) {
                match validator.validate(data, &rule.config) {
                    Ok(custom_result) => result.merge(custom_result),
                    Err(e) => {
                        result.add_error(ValidationError::new(
                            "custom_validator",
                            format!("Custom validation failed: {}", e),
                            "CUSTOM_VALIDATION_ERROR",
                        ));
                    }
                }
            } else {
                result.add_error(ValidationError::new(
                    "validator",
                    format!("Unknown validator: {}", validator_name),
                    "UNKNOWN_VALIDATOR",
                ));
            }
        }

        result
    }
}

impl Default for TypeValidator {
    fn default() -> Self {
        Self {
            custom_validators: HashMap::with_capacity(8),
        }
    }
}

/// Trait for custom validators
pub trait CustomValidator: Send + Sync {
    /// Validate data with custom logic
    fn validate(
        &self,
        data: &str,
        config: &HashMap<String, serde_json::Value>,
    ) -> GraphBitResult<ValidationResult>;

    /// Get validator name
    fn name(&self) -> &str;

    /// Get validator description
    fn description(&self) -> &str;
}
