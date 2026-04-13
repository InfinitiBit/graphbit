use crate::llm::FinishReason;

/// Map OpenAI-shape finish_reason values into [`FinishReason`].
pub(crate) fn parse_openai_finish_reason(reason: Option<&str>) -> FinishReason {
    match reason {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCalls,
        Some("content_filter") => FinishReason::ContentFilter,
        Some(other) => FinishReason::Other(other.to_string()),
        None => FinishReason::Stop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_finish_reasons() {
        assert!(matches!(
            parse_openai_finish_reason(Some("stop")),
            FinishReason::Stop
        ));
        assert!(matches!(
            parse_openai_finish_reason(Some("length")),
            FinishReason::Length
        ));
        assert!(matches!(
            parse_openai_finish_reason(Some("tool_calls")),
            FinishReason::ToolCalls
        ));
        assert!(matches!(
            parse_openai_finish_reason(Some("content_filter")),
            FinishReason::ContentFilter
        ));
    }

    #[test]
    fn parses_unknown_and_none_finish_reasons() {
        assert!(matches!(
            parse_openai_finish_reason(Some("rate_limited")),
            FinishReason::Other(v) if v == "rate_limited"
        ));
        assert!(matches!(
            parse_openai_finish_reason(None),
            FinishReason::Stop
        ));
    }
}
