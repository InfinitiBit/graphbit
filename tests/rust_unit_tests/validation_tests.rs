use graphbit_core::validation::{CustomValidator, TypeValidator, ValidationError, ValidationRule, ValidationResult};
use graphbit_core::*;
use std::collections::HashMap;

#[test]
fn test_validation_result() {
    let success = ValidationResult::success();
    assert!(success.is_valid);

    let errors = vec![ValidationError::new(
        "test_field",
        "test error",
        "TEST_ERROR",
    )];
    let failure = ValidationResult::failure(errors);
    assert!(!failure.is_valid);
    assert_eq!(failure.errors.len(), 1);
}

#[test]
fn test_validation_error() {
    let error = ValidationError::new("test_field", "test error", "TEST_ERROR");
    assert_eq!(error.field_path, "test_field");
    assert_eq!(error.message, "test error");
    assert_eq!(error.error_code, "TEST_ERROR");
}

#[test]
fn test_validation_result_merging() {
    let mut result1 = ValidationResult::success();
    let result2 = ValidationResult::success();
    result1.merge(result2);
    assert!(result1.is_valid);

    let mut result1 = ValidationResult::success();
    let result2 =
        ValidationResult::failure(vec![ValidationError::new("field", "error", "TEST_ERROR")]);
    result1.merge(result2);
    assert!(!result1.is_valid);
}

#[test]
fn test_validation_error_with_expected_and_actual() {
    let error = ValidationError::new("test_field", "test error", "TEST_ERROR")
        .with_expected("string")
        .with_actual("number");

    assert_eq!(error.field_path, "test_field");
    assert_eq!(error.message, "test error");
    assert_eq!(error.error_code, "TEST_ERROR");
    assert_eq!(error.expected, Some("string".to_string()));
    assert_eq!(error.actual, Some("number".to_string()));
}

#[test]
fn test_validation_result_with_metadata() {
    let result = ValidationResult::success().with_metadata(
        "test_key".to_string(),
        serde_json::Value::String("test_value".to_string()),
    );

    assert!(result.is_valid);
    assert_eq!(
        result.metadata.get("test_key"),
        Some(&serde_json::Value::String("test_value".to_string()))
    );
}

#[test]
fn test_validation_result_add_error() {
    let mut result = ValidationResult::success();
    assert!(result.is_valid);

    let error = ValidationError::new("field", "error message", "ERROR_CODE");
    result.add_error(error);

    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].field_path, "field");
}

#[test]
fn test_validation_rule_builder() {
    let rule = ValidationRule::new("test_rule", "Test rule description")
        .with_schema(serde_json::json!({"type": "string"}))
        .with_validator("custom_validator".to_string())
        .with_config(
            "param1".to_string(),
            serde_json::Value::String("value1".to_string()),
        );

    assert_eq!(rule.name, "test_rule");
    assert_eq!(rule.description, "Test rule description");
    assert!(rule.schema.is_some());
    assert_eq!(
        rule.validator_function,
        Some("custom_validator".to_string())
    );
    assert_eq!(
        rule.config.get("param1"),
        Some(&serde_json::Value::String("value1".to_string()))
    );
}

#[test]
fn test_type_validator_creation() {
    let _validator = TypeValidator::new();
    // Should create successfully

    let _default_validator = TypeValidator::default();
    // Should create successfully with default
}

#[test]
fn test_json_schema_validation_basic_types() {
    let validator = TypeValidator::new();

    // Test string validation
    let string_schema = serde_json::json!({"type": "string"});
    let result = validator.validate_against_schema("\"hello\"", &string_schema);
    assert!(result.is_valid);

    let result = validator.validate_against_schema("123", &string_schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "TYPE_MISMATCH"));

    // Test number validation
    let number_schema = serde_json::json!({"type": "number"});
    let result = validator.validate_against_schema("42", &number_schema);
    assert!(result.is_valid);

    let result = validator.validate_against_schema("\"not a number\"", &number_schema);
    assert!(!result.is_valid);

    // Test boolean validation
    let boolean_schema = serde_json::json!({"type": "boolean"});
    let result = validator.validate_against_schema("true", &boolean_schema);
    assert!(result.is_valid);

    let result = validator.validate_against_schema("\"not a boolean\"", &boolean_schema);
    assert!(!result.is_valid);
}

#[test]
fn test_json_schema_validation_string_constraints() {
    let validator = TypeValidator::new();

    // Test string length constraints
    let schema = serde_json::json!({
        "type": "string",
        "minLength": 5,
        "maxLength": 10
    });

    // Valid string
    let result = validator.validate_against_schema("\"hello\"", &schema);
    assert!(result.is_valid);

    // Too short
    let result = validator.validate_against_schema("\"hi\"", &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "STRING_TOO_SHORT"));

    // Too long
    let result = validator.validate_against_schema("\"this is way too long\"", &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "STRING_TOO_LONG"));

    // Test pattern validation
    let pattern_schema = serde_json::json!({
        "type": "string",
        "pattern": "^[a-z]+$"
    });

    let result = validator.validate_against_schema("\"hello\"", &pattern_schema);
    assert!(result.is_valid);

    let result = validator.validate_against_schema("\"Hello123\"", &pattern_schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "PATTERN_MISMATCH"));
}

#[test]
fn test_json_schema_validation_number_constraints() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "number",
        "minimum": 0,
        "maximum": 100
    });

    // Valid number
    let result = validator.validate_against_schema("50", &schema);
    assert!(result.is_valid);

    // Too small
    let result = validator.validate_against_schema("-10", &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "NUMBER_TOO_SMALL"));

    // Too large
    let result = validator.validate_against_schema("150", &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "NUMBER_TOO_LARGE"));
}

#[test]
fn test_json_schema_validation_object_properties() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"}
        },
        "required": ["name"]
    });

    // Valid object
    let result = validator.validate_against_schema(r#"{"name": "John", "age": 30}"#, &schema);
    assert!(result.is_valid);

    // Missing required property
    let result = validator.validate_against_schema(r#"{"age": 30}"#, &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "MISSING_REQUIRED_PROPERTY"));

    // Invalid property type
    let result = validator.validate_against_schema(r#"{"name": "John", "age": "thirty"}"#, &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "TYPE_MISMATCH"));
}

#[test]
fn test_json_schema_validation_array_items() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "array",
        "items": {"type": "string"}
    });

    // Valid array
    let result = validator.validate_against_schema(r#"["hello", "world"]"#, &schema);
    assert!(result.is_valid);

    // Invalid item type
    let result = validator.validate_against_schema(r#"["hello", 123]"#, &schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "TYPE_MISMATCH"));
}

#[test]
fn test_invalid_json_input() {
    let validator = TypeValidator::new();
    let schema = serde_json::json!({"type": "string"});

    let result = validator.validate_against_schema("invalid json", &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "INVALID_JSON"));
}

#[test]
fn test_invalid_schema() {
    let validator = TypeValidator::new();
    let invalid_schema = serde_json::Value::String("not an object".to_string());

    let result = validator.validate_against_schema("\"test\"", &invalid_schema);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "INVALID_SCHEMA"));
}

// Mock custom validator for testing
struct MockValidator {
    name: String,
    should_pass: bool,
}

impl MockValidator {
    fn new(name: &str, should_pass: bool) -> Self {
        Self {
            name: name.to_string(),
            should_pass,
        }
    }
}

impl CustomValidator for MockValidator {
    fn validate(
        &self,
        _data: &str,
        _config: &HashMap<String, serde_json::Value>,
    ) -> GraphBitResult<ValidationResult> {
        if self.should_pass {
            Ok(ValidationResult::success())
        } else {
            Ok(ValidationResult::failure(vec![ValidationError::new(
                "test_field",
                "Mock validation failed",
                "MOCK_ERROR",
            )]))
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "Mock validator for testing"
    }
}

#[test]
fn test_custom_validator_registration_and_usage() {
    let mut validator = TypeValidator::new();

    // Register a custom validator
    let custom_validator = Box::new(MockValidator::new("mock_validator", true));
    validator
        .register_validator("mock_validator".to_string(), custom_validator)
        .unwrap();

    // Create a rule that uses the custom validator
    let rule = ValidationRule::new("test_rule", "Test rule with custom validator")
        .with_validator("mock_validator");

    let result = validator.validate_against_rule("test data", &rule);
    assert!(result.is_valid);
}

#[test]
fn test_custom_validator_failure() {
    let mut validator = TypeValidator::new();

    // Register a custom validator that fails
    let custom_validator = Box::new(MockValidator::new("failing_validator", false));
    validator
        .register_validator("failing_validator".to_string(), custom_validator)
        .unwrap();

    let rule = ValidationRule::new("test_rule", "Test rule with failing validator")
        .with_validator("failing_validator");

    let result = validator.validate_against_rule("test data", &rule);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "MOCK_ERROR"));
}

#[test]
fn test_unknown_custom_validator() {
    let validator = TypeValidator::new();

    let rule = ValidationRule::new("test_rule", "Test rule with unknown validator")
        .with_validator("unknown_validator");

    let result = validator.validate_against_rule("test data", &rule);
    assert!(!result.is_valid);
    assert!(result
        .errors
        .iter()
        .any(|e| e.error_code == "UNKNOWN_VALIDATOR"));
}

// Additional tests to achieve 100% coverage for validation.rs

#[test]
fn test_validation_result_merge_with_metadata() {
    let mut result1 = ValidationResult::success()
        .with_metadata("key1".to_string(), serde_json::Value::String("value1".to_string()));

    let result2 = ValidationResult::success()
        .with_metadata("key2".to_string(), serde_json::Value::String("value2".to_string()));

    result1.merge(result2);

    assert!(result1.is_valid);
    assert_eq!(result1.metadata.len(), 2);
    assert_eq!(result1.metadata.get("key1"), Some(&serde_json::Value::String("value1".to_string())));
    assert_eq!(result1.metadata.get("key2"), Some(&serde_json::Value::String("value2".to_string())));
}

#[test]
fn test_validation_result_merge_with_errors() {
    let mut result1 = ValidationResult::success();
    let error1 = ValidationError::new("field1", "error1", "ERROR1");
    result1.add_error(error1);

    let mut result2 = ValidationResult::success();
    let error2 = ValidationError::new("field2", "error2", "ERROR2");
    result2.add_error(error2);

    result1.merge(result2);

    assert!(!result1.is_valid);
    assert_eq!(result1.errors.len(), 2);
    assert_eq!(result1.errors[0].field_path, "field1");
    assert_eq!(result1.errors[1].field_path, "field2");
}

#[test]
fn test_invalid_schema_validation() {
    let validator = TypeValidator::new();

    // Test with non-object schema
    let invalid_schema = serde_json::Value::String("not an object".to_string());
    let result = validator.validate_against_schema("\"test\"", &invalid_schema);

    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "INVALID_SCHEMA"));
}

#[test]
fn test_null_type_validation() {
    let validator = TypeValidator::new();

    let null_schema = serde_json::json!({"type": "null"});
    let result = validator.validate_against_schema("null", &null_schema);
    assert!(result.is_valid);

    let result = validator.validate_against_schema("\"not null\"", &null_schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "TYPE_MISMATCH"));
}

#[test]
fn test_object_required_properties_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"}
        },
        "required": ["name", "age"]
    });

    // Valid object
    let valid_data = r#"{"name": "John", "age": 30}"#;
    let result = validator.validate_against_schema(valid_data, &schema);
    assert!(result.is_valid);

    // Missing required property
    let invalid_data = r#"{"name": "John"}"#;
    let result = validator.validate_against_schema(invalid_data, &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "MISSING_REQUIRED_PROPERTY"));
}

#[test]
fn test_nested_object_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "email": {"type": "string", "pattern": "^[^@]+@[^@]+\\.[^@]+$"}
                }
            }
        }
    });

    // Valid nested object
    let valid_data = r#"{"user": {"name": "John", "email": "john@example.com"}}"#;
    let result = validator.validate_against_schema(valid_data, &schema);
    assert!(result.is_valid);

    // Invalid nested object (bad email pattern)
    let invalid_data = r#"{"user": {"name": "John", "email": "invalid-email"}}"#;
    let result = validator.validate_against_schema(invalid_data, &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "PATTERN_MISMATCH"));
}

#[test]
fn test_array_items_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "array",
        "items": {"type": "string"}
    });

    // Valid array
    let valid_data = r#"["hello", "world"]"#;
    let result = validator.validate_against_schema(valid_data, &schema);
    assert!(result.is_valid);

    // Invalid array item
    let invalid_data = r#"["hello", 123]"#;
    let result = validator.validate_against_schema(invalid_data, &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "TYPE_MISMATCH"));
}

#[test]
fn test_invalid_regex_pattern() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "string",
        "pattern": "[invalid regex pattern"  // Missing closing bracket
    });

    let result = validator.validate_against_schema("\"test\"", &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "INVALID_REGEX_PATTERN"));
}

#[test]
fn test_number_constraints_validation() {
    let validator = TypeValidator::new();

    let schema = serde_json::json!({
        "type": "number",
        "minimum": 10,
        "maximum": 100
    });

    // Valid number
    let result = validator.validate_against_schema("50", &schema);
    assert!(result.is_valid);

    // Too small
    let result = validator.validate_against_schema("5", &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "NUMBER_TOO_SMALL"));

    // Too large
    let result = validator.validate_against_schema("150", &schema);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "NUMBER_TOO_LARGE"));
}

#[test]
fn test_custom_validator_error_handling() {
    let mut validator = TypeValidator::new();

    // Register a custom validator that returns an error
    let custom_validator = Box::new(ErrorValidator);
    validator.register_validator("error_validator".to_string(), custom_validator).unwrap();

    let rule = ValidationRule::new("test_rule", "Test rule with error validator")
        .with_validator("error_validator");

    let result = validator.validate_against_rule("test data", &rule);
    assert!(!result.is_valid);
    assert!(result.errors.iter().any(|e| e.error_code == "CUSTOM_VALIDATION_ERROR"));
}

// Helper struct for testing custom validator errors
struct ErrorValidator;

impl CustomValidator for ErrorValidator {
    fn validate(
        &self,
        _data: &str,
        _config: &HashMap<String, serde_json::Value>,
    ) -> GraphBitResult<ValidationResult> {
        Err(GraphBitError::validation("test_field", "Test error"))
    }

    fn name(&self) -> &str {
        "error_validator"
    }

    fn description(&self) -> &str {
        "A validator that always returns an error"
    }
}
