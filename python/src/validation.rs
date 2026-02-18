//! Validation utilities for GraphBit Python bindings

use pyo3::prelude::*;

/// Validate API key for different providers.
///
/// Performs three tiers of validation:
/// 1. Empty check
/// 2. Minimum length check (provider-specific)
/// 3. Prefix/format check (provider-specific)
///
/// Also rejects keys containing whitespace or control characters, which
/// are common copy-paste artefacts.
pub(crate) fn validate_api_key(api_key: &str, provider: &str) -> PyResult<()> {
    // ── Tier 0: empty ────────────────────────────────────────────────────────
    if api_key.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{provider} API key cannot be empty"
        )));
    }

    // ── Tier 0b: whitespace / control-character guard ────────────────────────
    // Catches copy-paste artefacts such as leading/trailing spaces or newlines.
    if api_key.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{provider} API key contains invalid characters (whitespace or control characters). \
             Please ensure the key was copied correctly without surrounding spaces or newlines."
        )));
    }

    // ── Provider-specific rules ───────────────────────────────────────────────
    // Each entry: (min_length, expected_prefix, format_description)
    // `expected_prefix` is `None` for providers that don't have a fixed prefix.
    let (min_length, expected_prefix, format_desc) = match provider.to_lowercase().as_str() {
        // OpenAI: "sk-" followed by alphanumeric characters (legacy and project keys)
        "openai" => (
            20,
            Some("sk-"),
            "OpenAI API keys start with 'sk-' followed by alphanumeric characters \
             (e.g. sk-proj-... or sk-...)",
        ),

        // Anthropic: "sk-ant-" prefix
        "anthropic" => (
            20,
            Some("sk-ant-"),
            "Anthropic API keys start with 'sk-ant-' followed by alphanumeric characters",
        ),

        // HuggingFace: "hf_" prefix
        "huggingface" => (
            10,
            Some("hf_"),
            "HuggingFace API keys start with 'hf_' followed by alphanumeric characters",
        ),

        // Perplexity: "pplx-" prefix
        "perplexity" => (
            10,
            Some("pplx-"),
            "Perplexity API keys start with 'pplx-' followed by alphanumeric characters",
        ),

        // xAI (Grok): "xai-" prefix
        "xai" => (
            10,
            Some("xai-"),
            "xAI API keys start with 'xai-' followed by alphanumeric characters",
        ),

        // DeepSeek uses OpenAI-compatible format with "sk-" prefix
        "deepseek" => (
            20,
            Some("sk-"),
            "DeepSeek API keys start with 'sk-' followed by alphanumeric characters",
        ),

        // OpenRouter: "sk-or-" prefix
        "openrouter" => (
            20,
            Some("sk-or-"),
            "OpenRouter API keys start with 'sk-or-' followed by alphanumeric characters",
        ),

        // Replicate: "r8_" prefix
        "replicate" => (
            20,
            Some("r8_"),
            "Replicate API tokens start with 'r8_' followed by alphanumeric characters",
        ),

        // Gemini: "AIza" prefix (Google API key format)
        "gemini" => (
            20,
            Some("AIza"),
            "Google Gemini API keys start with 'AIza' followed by alphanumeric characters",
        ),

        // MistralAI: alphanumeric, no fixed prefix
        "mistralai" => (
            20,
            None,
            "MistralAI API keys are alphanumeric strings of at least 20 characters",
        ),

        // Fireworks: alphanumeric, no fixed prefix
        "fireworks" => (
            20,
            None,
            "Fireworks API keys are alphanumeric strings of at least 20 characters",
        ),

        // AI21: alphanumeric, no fixed prefix
        "ai21" => (
            20,
            None,
            "AI21 API keys are alphanumeric strings of at least 20 characters",
        ),

        // TogetherAI: alphanumeric hex, no fixed prefix
        "togetherai" => (
            20,
            None,
            "TogetherAI API keys are alphanumeric strings of at least 20 characters",
        ),

        // ByteDance: generic, no fixed prefix
        "bytedance" => (
            8,
            None,
            "ByteDance API keys are alphanumeric strings of at least 8 characters",
        ),

        // Azure LLM / Azure Embedding: hex strings or UUIDs, no fixed prefix
        "azure llm" | "azure" => (
            8,
            None,
            "Azure API keys are typically hex strings or UUIDs of at least 8 characters",
        ),

        // LiteLLM: generic, no fixed prefix
        "litellm" => (
            8,
            None,
            "LiteLLM API keys are alphanumeric strings of at least 8 characters",
        ),

        // Fallback for any provider not listed above
        _ => (8, None, "API key must be at least 8 characters"),
    };

    // ── Tier 1: minimum length ────────────────────────────────────────────────
    if api_key.len() < min_length {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "{provider} API key is too short (minimum {min_length} characters). {format_desc}"
        )));
    }

    // ── Tier 2: prefix / format check ────────────────────────────────────────
    if let Some(prefix) = expected_prefix {
        if !api_key.starts_with(prefix) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "{provider} API key has an invalid format. {format_desc}"
            )));
        }
    }

    Ok(())
}
