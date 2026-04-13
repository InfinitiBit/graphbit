use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::LlmResponse;
use futures::stream::{Stream, StreamExt};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Execute a simple OpenAI-style SSE text stream.
///
/// This helper centralizes the shared streaming skeleton used by providers that:
/// - emit `data: {json}` lines
/// - terminate with `data: [DONE]`
/// - stream text via a chunk type where a provider callback extracts `(id, content)`
pub(crate) async fn execute_openai_style_text_stream<Chunk, FH, FE>(
    provider_name: &'static str,
    provider_display_name: &'static str,
    client: &Client,
    url: &str,
    api_key: &str,
    request_json: &Value,
    customize_builder: FH,
    model: String,
    parse_tail_line: bool,
    extract_chunk_content: FE,
) -> GraphBitResult<Box<dyn Stream<Item = GraphBitResult<LlmResponse>> + Unpin + Send>>
where
    Chunk: DeserializeOwned + Send + 'static,
    FH: FnOnce(RequestBuilder) -> RequestBuilder,
    FE: Fn(&Chunk) -> Option<(String, String)> + Send + Sync + 'static,
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
    let extract_chunk_content = Arc::new(extract_chunk_content);

    let stream = futures::stream::unfold(
        (byte_stream, String::new(), false, 0u32, 0u32),
        move |(
            mut byte_stream,
            mut buffer,
            timeout_occurred,
            mut consecutive_parse_errors,
            mut total_parse_errors,
        )| {
            let model = model.clone();
            let extract_chunk_content = Arc::clone(&extract_chunk_content);
            async move {
                if timeout_occurred {
                    return None;
                }

                loop {
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

                            match serde_json::from_str::<Chunk>(data) {
                                Ok(stream_chunk) => {
                                    consecutive_parse_errors = 0;
                                    if let Some((id, content)) = extract_chunk_content(&stream_chunk)
                                        && !content.is_empty()
                                    {
                                        let response = LlmResponse::new(content, &model).with_id(id);
                                        return Some((
                                            Ok(response),
                                            (
                                                byte_stream,
                                                buffer,
                                                false,
                                                consecutive_parse_errors,
                                                total_parse_errors,
                                            ),
                                        ));
                                    }
                                }
                                Err(e) => {
                                    consecutive_parse_errors += 1;
                                    total_parse_errors += 1;

                                    tracing::warn!(
                                        "Failed to parse {} stream chunk (consecutive: {}, total: {}): {}, data: {}",
                                        provider_display_name,
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
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    let chunk_result = match timeout(CHUNK_TIMEOUT, byte_stream.next()).await {
                        Ok(Some(result)) => result,
                        Ok(None) => {
                            if parse_tail_line {
                                let tail_line = buffer.trim().to_string();
                                if let Some(data) = tail_line.strip_prefix("data: ")
                                    && data.trim() != "[DONE]"
                                    && let Ok(stream_chunk) = serde_json::from_str::<Chunk>(data)
                                    && let Some((id, content)) =
                                        extract_chunk_content(&stream_chunk)
                                    && !content.is_empty()
                                {
                                    let response = LlmResponse::new(content, &model).with_id(id);
                                    return Some((
                                        Ok(response),
                                        (
                                            byte_stream,
                                            String::new(),
                                            false,
                                            consecutive_parse_errors,
                                            total_parse_errors,
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
                                ),
                            ));
                        }
                    };

                    let chunk = match chunk_result {
                        Ok(c) => c,
                        Err(e) => {
                            return Some((
                                Err(GraphBitError::llm_provider(
                                    provider_name,
                                    format!("Stream error: {e}"),
                                )),
                                (
                                    byte_stream,
                                    buffer,
                                    false,
                                    consecutive_parse_errors,
                                    total_parse_errors,
                                ),
                            ));
                        }
                    };

                    buffer.push_str(&String::from_utf8_lossy(&chunk));
                }
            }
        },
    );

    Ok(Box::new(Box::pin(stream)))
}
