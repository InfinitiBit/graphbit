use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::LlmUsage;
use serde_json::{Map, Value};

pub(crate) const TOOL_ONLY_FALLBACK_TEXT: &str =
    "I'll help you with that using the available tools.";

/// Return the first choice or a standardized provider error.
pub(crate) fn first_choice_or_error<T>(provider_name: &str, choices: Vec<T>) -> GraphBitResult<T> {
    choices
        .into_iter()
        .next()
        .ok_or_else(|| GraphBitError::llm_provider(provider_name, "No choices in response"))
}

/// True when a response includes at least one tool call.
pub(crate) fn has_tool_calls<T>(tool_calls: Option<&Vec<T>>) -> bool {
    tool_calls.map(|calls| !calls.is_empty()).unwrap_or(false)
}

/// Apply provider fallback text when content is empty and tool calls exist.
pub(crate) fn fallback_content_if_tool_only(
    content: String,
    has_tool_calls: bool,
    fallback_text: &str,
) -> String {
    if content.trim().is_empty() && has_tool_calls {
        fallback_text.to_string()
    } else {
        content
    }
}

/// Parse tool arguments and preserve raw payload on JSON errors.
pub(crate) fn parse_tool_arguments_empty_object_or_raw_fallback<F>(
    arguments: &str,
    on_parse_error: F,
) -> Value
where
    F: FnOnce(&str, &serde_json::Error),
{
    if arguments.trim().is_empty() {
        return Value::Object(Map::new());
    }

    match serde_json::from_str(arguments) {
        Ok(value) => value,
        Err(e) => {
            on_parse_error(arguments, &e);
            serde_json::json!({ "raw_arguments": arguments })
        }
    }
}

/// Parse tool arguments with a shared OpenAI-style warning and fallback behavior.
pub(crate) fn parse_tool_arguments_openai_style(tool_name: &str, arguments: &str) -> Value {
    parse_tool_arguments_empty_object_or_raw_fallback(arguments, |raw_arguments, e| {
        tracing::warn!(
            "Failed to parse tool call arguments for {}: {e}. Arguments: '{}'",
            tool_name,
            raw_arguments
        );
    })
}

/// Build usage from prompt/completion token counts.
pub(crate) fn usage_from_prompt_completion(prompt_tokens: u32, completion_tokens: u32) -> LlmUsage {
    LlmUsage::new(prompt_tokens, completion_tokens)
}
