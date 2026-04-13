use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::openai_compat::finish_reason::parse_openai_finish_reason;
use crate::llm::openai_compat::stream_tools::{
    StreamToolCallAccum as CompatStreamToolCallAccum,
    StreamToolCallDelta as CompatStreamToolCallDelta,
    assistant_text_for_tool_calls as compat_assistant_text_for_tool_calls,
    merge_stream_tool_call_deltas as compat_merge_stream_tool_call_deltas,
    render_stream_tool_call_delta_fragment as compat_render_stream_tool_call_delta_fragment,
    stream_tool_accum_to_llm_calls as compat_stream_tool_accum_to_llm_calls,
};
use crate::llm::{LlmResponse, LlmToolCall, LlmUsage};
use futures::stream::{Stream, StreamExt};
use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OpenAiStreamChunk {
    pub(crate) id: String,
    pub(crate) choices: Vec<OpenAiStreamChoice>,
    #[serde(default)]
    pub(crate) usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OpenAiStreamChoice {
    pub(crate) delta: OpenAiDelta,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OpenAiUsage {
    pub(crate) prompt_tokens: u32,
    pub(crate) completion_tokens: u32,
}

type OpenAiDeltaToolCall = CompatStreamToolCallDelta;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OpenAiDelta {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) _role: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Vec<OpenAiDeltaToolCall>,
}

/// One tool-call slot in an SSE stream (`index`), merged across chunks.
pub(crate) type OpenAiStreamToolAccum = CompatStreamToolCallAccum;

pub(crate) fn merge_openai_stream_tool_deltas(
    acc: &mut HashMap<u32, OpenAiStreamToolAccum>,
    deltas: &[OpenAiDeltaToolCall],
) {
    compat_merge_stream_tool_call_deltas(acc, deltas);
}

/// Render user-facing incremental text for tool-call deltas.
pub(crate) fn render_tool_call_delta_fragment(
    deltas: &[OpenAiDeltaToolCall],
    acc: &HashMap<u32, OpenAiStreamToolAccum>,
    announced: &mut HashSet<u32>,
) -> String {
    compat_render_stream_tool_call_delta_fragment(deltas, acc, announced)
}

pub(crate) fn tool_accum_map_to_llm_calls(
    acc: &HashMap<u32, OpenAiStreamToolAccum>,
) -> Vec<LlmToolCall> {
    compat_stream_tool_accum_to_llm_calls(acc)
}

/// For streaming, keep textual content as-is; tool-call details are surfaced via
/// `LlmResponse.tool_calls` and consumed by higher layers.
pub(crate) fn stream_assistant_text_for_tool_calls(
    content: String,
    tool_calls: &[LlmToolCall],
) -> String {
    compat_assistant_text_for_tool_calls(content, tool_calls)
}

pub(crate) async fn execute_openai_advanced_stream<F>(
    client: &Client,
    url: &str,
    api_key: &str,
    request_json: &Value,
    customize_builder: F,
    model: String,
) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    execute_advanced_stream_inner(
        "openai",
        "OpenAI",
        client,
        url,
        api_key,
        request_json,
        customize_builder,
        model,
    )
    .await
}

pub(crate) async fn execute_advanced_stream_for_provider<F>(
    provider_name: &'static str,
    provider_display_name: &'static str,
    client: &Client,
    url: &str,
    api_key: &str,
    request_json: &Value,
    customize_builder: F,
    model: String,
) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    execute_advanced_stream_inner(
        provider_name,
        provider_display_name,
        client,
        url,
        api_key,
        request_json,
        customize_builder,
        model,
    )
    .await
}

async fn execute_advanced_stream_inner<F>(
    provider_name: &'static str,
    provider_display_name: &'static str,
    client: &Client,
    url: &str,
    api_key: &str,
    request_json: &Value,
    customize_builder: F,
    model: String,
) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    const CONNECTION_TIMEOUT: Duration = Duration::from_secs(60);
    const ERROR_BODY_TIMEOUT: Duration = Duration::from_secs(10);
    const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);
    const MAX_CONSECUTIVE_PARSE_ERRORS: u32 = 5;

    let response = timeout(
        CONNECTION_TIMEOUT,
        customize_builder(
            client
                .post(url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(request_json),
        )
        .send(),
    )
    .await
    .map_err(|_| {
        GraphBitError::llm_provider(
            provider_name,
            format!(
                "Connection timeout after {:?} - {} did not respond. \
                 Check network connectivity and {} status.",
                CONNECTION_TIMEOUT, provider_display_name, provider_display_name
            ),
        )
    })?
    .map_err(|e| GraphBitError::llm_provider(provider_name, format!("Request failed: {e}")))?;

    if !response.status().is_success() {
        let error_text = timeout(ERROR_BODY_TIMEOUT, response.text())
            .await
            .unwrap_or_else(|_| Ok(format!("Error body read timeout after {:?}", ERROR_BODY_TIMEOUT)))
            .unwrap_or_else(|_| "Unknown error (failed to read body)".to_string());

        return Err(GraphBitError::llm_provider(
            provider_name,
            format!("API error: {error_text}"),
        ));
    }

    let byte_stream = response.bytes_stream();
    let provider_name_clone = provider_name;
    let provider_display_name_clone = provider_display_name;
    let stream = futures::stream::unfold(
        (
            byte_stream,
            String::new(),
            false,
            0u32,
            0u32,
            HashMap::<u32, OpenAiStreamToolAccum>::new(),
            HashSet::<u32>::new(),
        ),
        move |(
            mut byte_stream,
            mut buffer,
            timeout_occurred,
            mut consecutive_parse_errors,
            mut total_parse_errors,
            mut tool_call_accum,
            mut announced_tool_indices,
        )| {
            let model = model.clone();
            let provider_name = provider_name_clone;
            let provider_display_name = provider_display_name_clone;
            async move {
                if timeout_occurred {
                    return None;
                }

                loop {
                    if buffer.find('\n').is_none() {
                        let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                            Ok(Some(result)) => result,
                            Ok(None) => {
                                let tail_line = buffer.trim().to_string();
                                if let Some(data) = tail_line.strip_prefix("data: ")
                                    && data.trim() != "[DONE]"
                                    && let Ok(stream_chunk) =
                                        serde_json::from_str::<OpenAiStreamChunk>(data)
                                {
                                    let OpenAiStreamChunk { id, choices, usage } = stream_chunk;
                                    let mut tool_fragment = String::new();
                                    if let Some(choice) = choices.first() {
                                        merge_openai_stream_tool_deltas(
                                            &mut tool_call_accum,
                                            &choice.delta.tool_calls,
                                        );
                                        tool_fragment = render_tool_call_delta_fragment(
                                            &choice.delta.tool_calls,
                                            &tool_call_accum,
                                            &mut announced_tool_indices,
                                        );
                                    }
                                    let streamed_tool_calls =
                                        tool_accum_map_to_llm_calls(&tool_call_accum);
                                    let usage = usage
                                        .map(|u| LlmUsage::new(u.prompt_tokens, u.completion_tokens));
                                    let finish_reason = choices
                                        .first()
                                        .and_then(|c| c.finish_reason.as_deref())
                                        .map(|reason| parse_openai_finish_reason(Some(reason)));
                                    let content = choices
                                        .first()
                                        .and_then(|c| c.delta.content.as_ref())
                                        .cloned()
                                        .unwrap_or_default();
                                    let streamed_content = format!("{tool_fragment}{content}");

                                    if !streamed_content.is_empty()
                                        || usage.is_some()
                                        || finish_reason.is_some()
                                    {
                                        let text = if streamed_content.is_empty() {
                                            stream_assistant_text_for_tool_calls(
                                                String::new(),
                                                &streamed_tool_calls,
                                            )
                                        } else {
                                            streamed_content
                                        };
                                        let mut response = LlmResponse::new(text, &model)
                                            .with_id(id)
                                            .with_tool_calls(streamed_tool_calls);
                                        if let Some(usage) = usage {
                                            response = response.with_usage(usage);
                                        }
                                        if let Some(finish_reason) = finish_reason {
                                            response = response.with_finish_reason(finish_reason);
                                        }
                                        return Some((
                                            Ok(response),
                                            (
                                                byte_stream,
                                                String::new(),
                                                false,
                                                consecutive_parse_errors,
                                                total_parse_errors,
                                                tool_call_accum,
                                                announced_tool_indices,
                                            ),
                                        ));
                                    }
                                }

                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "Stream ended with {} total parse errors. Some data may have been lost.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "Stream chunk timeout after {:?} - {} stopped responding. \
                                     Response may be incomplete.",
                                    CHUNK_TIMEOUT,
                                    provider_display_name
                                );
                                return Some((
                                    Err(GraphBitError::llm_provider(
                                        provider_name,
                                        format!(
                                            "Stream timeout after {:?} - response may be incomplete",
                                            CHUNK_TIMEOUT
                                        ),
                                    )),
                                    (
                                        byte_stream,
                                        buffer,
                                        true,
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                        tool_call_accum,
                                        announced_tool_indices,
                                    ),
                                ));
                            }
                        };

                        let chunk = match chunk_result {
                            Ok(c) => c,
                            Err(e) => {
                                return Some((
                                    Err(GraphBitError::llm_provider(provider_name, format!("Stream error: {e}"))),
                                    (
                                        byte_stream,
                                        buffer,
                                        false,
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                        tool_call_accum,
                                        announced_tool_indices,
                                    ),
                                ));
                            }
                        };

                        buffer.push_str(&String::from_utf8_lossy(&chunk));
                    }

                    while let Some(newline_pos) = buffer.find('\n') {
                        let line: String = buffer.drain(..=newline_pos).collect();
                        let line = line.trim();

                        if line.is_empty() || line.starts_with(':') {
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            if data.trim() == "[DONE]" {
                                if total_parse_errors > 0 {
                                    tracing::warn!(
                                        "Stream completed with {} total parse errors. Some data may have been lost.",
                                        total_parse_errors
                                    );
                                }
                                return None;
                            }

                            match serde_json::from_str::<OpenAiStreamChunk>(data) {
                                Ok(stream_chunk) => {
                                    consecutive_parse_errors = 0;

                                    let OpenAiStreamChunk { id, choices, usage } = stream_chunk;
                                    let mut tool_fragment = String::new();
                                    if let Some(choice) = choices.first() {
                                        merge_openai_stream_tool_deltas(
                                            &mut tool_call_accum,
                                            &choice.delta.tool_calls,
                                        );
                                        tool_fragment = render_tool_call_delta_fragment(
                                            &choice.delta.tool_calls,
                                            &tool_call_accum,
                                            &mut announced_tool_indices,
                                        );
                                    }
                                    let streamed_tool_calls = tool_accum_map_to_llm_calls(&tool_call_accum);
                                    let usage =
                                        usage.map(|u| LlmUsage::new(u.prompt_tokens, u.completion_tokens));
                                    let finish_reason = choices
                                        .first()
                                        .and_then(|c| c.finish_reason.as_deref())
                                        .map(|reason| parse_openai_finish_reason(Some(reason)));

                                    if let Some(choice) = choices.first() {
                                        let content = choice.delta.content.clone().unwrap_or_default();
                                        let streamed_content = format!("{tool_fragment}{content}");
                                        if !streamed_content.is_empty() {
                                            let mut response = LlmResponse::new(streamed_content, &model)
                                                .with_id(id.clone())
                                                .with_tool_calls(streamed_tool_calls.clone());
                                            if let Some(usage) = usage.clone() {
                                                response = response.with_usage(usage);
                                            }
                                            if let Some(finish_reason) = finish_reason.clone() {
                                                response = response.with_finish_reason(finish_reason);
                                            }
                                            return Some((
                                                Ok(response),
                                                (
                                                    byte_stream,
                                                    buffer,
                                                    false,
                                                    consecutive_parse_errors,
                                                    total_parse_errors,
                                                    tool_call_accum,
                                                    announced_tool_indices,
                                                ),
                                            ));
                                        }
                                    }

                                    if usage.is_some() || finish_reason.is_some() {
                                        let text = stream_assistant_text_for_tool_calls(
                                            String::new(),
                                            &streamed_tool_calls,
                                        );
                                        let mut response = LlmResponse::new(text, &model)
                                            .with_id(id)
                                            .with_tool_calls(streamed_tool_calls);
                                        if let Some(usage) = usage {
                                            response = response.with_usage(usage);
                                        }
                                        if let Some(finish_reason) = finish_reason {
                                            response = response.with_finish_reason(finish_reason);
                                        }
                                        return Some((
                                            Ok(response),
                                            (
                                                byte_stream,
                                                buffer,
                                                false,
                                                consecutive_parse_errors,
                                                total_parse_errors,
                                                tool_call_accum,
                                                announced_tool_indices,
                                            ),
                                        ));
                                    }
                                }
                                Err(e) => {
                                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(data) {
                                        if let Some(response) =
                                            usage_response_from_fallback_json(value, &model, &tool_call_accum)
                                        {
                                            return Some((
                                                Ok(response),
                                                (
                                                    byte_stream,
                                                    buffer,
                                                    false,
                                                    consecutive_parse_errors,
                                                    total_parse_errors,
                                                    tool_call_accum,
                                                    announced_tool_indices,
                                                ),
                                            ));
                                        }
                                    }

                                    consecutive_parse_errors += 1;
                                    total_parse_errors += 1;

                                    tracing::warn!(
                                        "Failed to parse stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                        consecutive_parse_errors,
                                        total_parse_errors,
                                        e,
                                        if data.len() > 200 { &data[..200] } else { data }
                                    );

                                    if consecutive_parse_errors >= MAX_CONSECUTIVE_PARSE_ERRORS {
                                        return Some((
                                            Err(GraphBitError::llm_provider(
                                                provider_name,
                                                format!(
                                                    "Stream corrupted: {} consecutive parse errors. \
                                                     Last error: {}. Data may be incomplete.",
                                                    consecutive_parse_errors, e
                                                ),
                                            )),
                                            (
                                                byte_stream,
                                                buffer,
                                                true,
                                                consecutive_parse_errors,
                                                total_parse_errors,
                                                tool_call_accum,
                                                announced_tool_indices,
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    );

    Ok(Box::new(Box::pin(stream)))
}

fn usage_response_from_fallback_json(
    value: Value,
    model: &str,
    tool_call_accum: &HashMap<u32, OpenAiStreamToolAccum>,
) -> Option<LlmResponse> {
    let prompt_tokens = value
        .get("usage")
        .and_then(|u| u.get("prompt_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let completion_tokens = value
        .get("usage")
        .and_then(|u| u.get("completion_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    if prompt_tokens == 0 && completion_tokens == 0 {
        return None;
    }

    let id = value
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let finish_reason = value
        .get("choices")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first())
        .and_then(|c| c.get("finish_reason"))
        .and_then(|v| v.as_str())
        .map(|r| parse_openai_finish_reason(Some(r)));

    let streamed_tool_calls = tool_accum_map_to_llm_calls(tool_call_accum);
    let text = stream_assistant_text_for_tool_calls(String::new(), &streamed_tool_calls);
    let mut response = LlmResponse::new(text, model)
        .with_usage(LlmUsage::new(prompt_tokens, completion_tokens))
        .with_tool_calls(streamed_tool_calls);
    if !id.is_empty() {
        response = response.with_id(id);
    }
    if let Some(finish_reason) = finish_reason {
        response = response.with_finish_reason(finish_reason);
    }

    Some(response)
}
