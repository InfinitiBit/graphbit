//! Validation utilities for GraphBit Node.js bindings

use napi::bindgen_prelude::*;

/// Validate that a string is not empty
pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            format!("{} cannot be empty", field_name),
        ));
    }
    Ok(())
}

/// Validate that a number is positive
pub fn validate_positive_number(value: i32, field_name: &str) -> Result<()> {
    if value <= 0 {
        return Err(Error::new(
            Status::InvalidArg,
            format!("{} must be positive, got: {}", field_name, value),
        ));
    }
    Ok(())
}

/// Validate that an optional number is positive if present
pub fn validate_optional_positive_number(
    value: Option<i32>,
    field_name: &str,
) -> Result<Option<usize>> {
    if let Some(val) = value {
        validate_positive_number(val, field_name)?;
        Ok(Some(val as usize))
    } else {
        Ok(None)
    }
}

/// Validate API key for different providers with enhanced requirements
pub fn validate_api_key(api_key: &str, provider: &str) -> Result<()> {
    if api_key.is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            format!("{} API key cannot be empty", provider),
        ));
    }

    let min_length = match provider.to_lowercase().as_str() {
        "openai" => 20,
        "anthropic" => 15,
        "deepseek" => 15,
        "huggingface" => 10,
        "perplexity" => 15,
        _ => 8,
    };

    if api_key.len() < min_length {
        return Err(Error::new(
            Status::InvalidArg,
            format!(
                "{} API key too short (minimum {} characters)",
                provider, min_length
            ),
        ));
    }

    // Additional validation for specific providers
    match provider.to_lowercase().as_str() {
        "openai" => {
            if !api_key.starts_with("sk-") {
                return Err(Error::new(
                    Status::InvalidArg,
                    "OpenAI API key must start with 'sk-'",
                ));
            }
        }
        "anthropic" => {
            if !api_key.starts_with("sk-ant-") {
                return Err(Error::new(
                    Status::InvalidArg,
                    "Anthropic API key must start with 'sk-ant-'",
                ));
            }
        }
        "huggingface" => {
            if !api_key.starts_with("hf_") {
                return Err(Error::new(
                    Status::InvalidArg,
                    "HuggingFace API key must start with 'hf_'",
                ));
            }
        }
        _ => {} // No specific format requirements for other providers
    }

    Ok(())
}

/// Validate URL format
pub fn validate_url(url: &str, field_name: &str) -> Result<()> {
    if url.is_empty() {
        return Err(Error::new(
            Status::InvalidArg,
            format!("{} cannot be empty", field_name),
        ));
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(Error::new(
            Status::InvalidArg,
            format!("{} must be a valid HTTP/HTTPS URL", field_name),
        ));
    }

    Ok(())
}

/// Validate model name format
pub fn validate_model_name(model: &str, provider: &str) -> Result<()> {
    if model.is_empty() {
        return Err(Error::new(Status::InvalidArg, "Model name cannot be empty"));
    }

    // Provider-specific model validation
    match provider.to_lowercase().as_str() {
        "openai" => {
            let valid_models = [
                "gpt-4",
                "gpt-4-turbo",
                "gpt-3.5-turbo",
                "gpt-4o",
                "gpt-4o-mini",
            ];
            if !valid_models.iter().any(|&m| model.starts_with(m)) {
                return Err(Error::new(
                    Status::InvalidArg,
                    format!(
                        "Invalid OpenAI model: {}. Must be one of: {}",
                        model,
                        valid_models.join(", ")
                    ),
                ));
            }
        }
        "anthropic" => {
            if !model.starts_with("claude-") {
                return Err(Error::new(
                    Status::InvalidArg,
                    format!(
                        "Invalid Anthropic model: {}. Must start with 'claude-'",
                        model
                    ),
                ));
            }
        }
        _ => {} // No specific validation for other providers
    }

    Ok(())
}
