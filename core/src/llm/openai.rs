//! `OpenAI` LLM provider implementation

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::providers::LlmProviderTrait;
use crate::llm::{
    FinishReason, LlmMessage, LlmRequest, LlmResponse, LlmRole, LlmTool, LlmToolCall, LlmUsage,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// `OpenAI` API provider
pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
    organization: Option<String>,
}

impl OpenAiProvider {
    /// Create a new `OpenAI` provider
    pub fn new(api_key: String, model: String) -> GraphBitResult<Self> {
        let client = build_client()?;
        let base_url = "https://api.openai.com/v1".to_string();

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Create a new `OpenAI` provider with custom base URL
    pub fn with_base_url(api_key: String, model: String, base_url: String) -> GraphBitResult<Self> {
        let client = build_client()?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url,
            organization: None,
        })
    }

    /// Set organization ID
    pub fn with_organization(mut self, organization: String) -> Self {
        self.organization = Some(organization);
        self
    }

    /// Check if the model is a reasoning model (o-series)
    fn is_reasoning_model(&self) -> bool {
        let m = self.model.as_str();
        m.starts_with("o1") || m.starts_with("o3") || m.starts_with("o4")
    }

    /// Convert `GraphBit` message to `OpenAI` message format
    fn convert_message(message: &LlmMessage) -> OpenAiMessage {
        OpenAiMessage {
            role: match message.role {
                LlmRole::User => "user".to_string(),
                LlmRole::Assistant => "assistant".to_string(),
                LlmRole::System => "system".to_string(),
                LlmRole::Tool => "tool".to_string(),
            },
            content: message.content.clone(),
            tool_calls: if message.tool_calls.is_empty() {
                None
            } else {
                Some(
                    message
                        .tool_calls
                        .iter()
                        .map(|tc| OpenAiToolCall {
                            id: tc.id.clone(),
                            r#type: "function".to_string(),
                            function: OpenAiFunction {
                                name: tc.name.clone(),
                                arguments: tc.parameters.to_string(),
                            },
                        })
                        .collect(),
                )
            },
            tool_call_id: message.tool_call_id.clone(),
        }
    }

    /// Convert `GraphBit` tool to `OpenAI` tool format
    fn convert_tool(tool: &LlmTool) -> OpenAiTool {
        OpenAiTool {
            r#type: "function".to_string(),
            function: OpenAiFunctionDef {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.parameters.clone(),
            },
        }
    }

    /// Parse `OpenAI` response to `GraphBit` response
    fn parse_response(&self, response: OpenAiResponse) -> GraphBitResult<LlmResponse> {
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| GraphBitError::llm_provider("openai", "No choices in response"))?;

        let mut content = choice.message.content;
        if content.trim().is_empty()
            && !choice
                .message
                .tool_calls
                .as_ref()
                .unwrap_or(&vec![])
                .is_empty()
        {
            content = "I'll help you with that using the available tools.".to_string();
        }
        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .map(|tc| {
                // Production-grade argument parsing with error handling
                let parameters = if tc.function.arguments.trim().is_empty() {
                    serde_json::Value::Object(serde_json::Map::new())
                } else {
                    match serde_json::from_str(&tc.function.arguments) {
                        Ok(params) => params,
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse tool call arguments for {}: {e}. Arguments: '{}'",
                                tc.function.name,
                                tc.function.arguments
                            );
                            serde_json::json!({ "raw_arguments": tc.function.arguments })
                        }
                    }
                };

                LlmToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    parameters,
                }
            })
            .collect();

        let finish_reason = match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            Some(other) => FinishReason::Other(other.to_string()),
            None => FinishReason::Stop,
        };

        let usage = LlmUsage::new(
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
        );

        let mut resp = LlmResponse::new(content, &self.model)
            .with_tool_calls(tool_calls)
            .with_usage(usage)
            .with_finish_reason(finish_reason)
            .with_id(response.id);

        if let Some(fp) = response.system_fingerprint {
            resp = resp.with_metadata("system_fingerprint".to_string(), serde_json::Value::String(fp));
        }
        if let Some(tier) = response.service_tier {
            resp = resp.with_metadata("service_tier".to_string(), serde_json::Value::String(tier));
        }
        if let Some(details) = response.usage.completion_tokens_details {
            if let Ok(val) = serde_json::to_value(&details) {
                resp = resp.with_metadata("completion_tokens_details".to_string(), val);
            }
        }
        if let Some(details) = response.usage.prompt_tokens_details {
            if let Ok(val) = serde_json::to_value(&details) {
                resp = resp.with_metadata("prompt_tokens_details".to_string(), val);
            }
        }

        Ok(resp)
    }
}

#[async_trait]
impl LlmProviderTrait for OpenAiProvider {
    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: LlmRequest) -> GraphBitResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let messages: Vec<OpenAiMessage> =
            request.messages.iter().map(Self::convert_message).collect();

        let tools: Option<Vec<OpenAiTool>> = if request.tools.is_empty() {
            None
        } else {
            Some(request.tools.iter().map(Self::convert_tool).collect())
        };

        // Extract typed OpenAI parameters from extra_params
        let mut extra = request.extra_params;

        let frequency_penalty = extract_f32(&mut extra, "frequency_penalty");
        let presence_penalty = extract_f32(&mut extra, "presence_penalty");
        let n = extract_u32(&mut extra, "n");
        let stop = extra.remove("stop");
        let seed = extra.remove("seed").and_then(|v| v.as_i64());
        let logit_bias = extra.remove("logit_bias");
        let response_format = extra.remove("response_format");
        let parallel_tool_calls = extra.remove("parallel_tool_calls").and_then(|v| v.as_bool());
        let logprobs = extra.remove("logprobs").and_then(|v| v.as_bool());
        let top_logprobs = extract_u32(&mut extra, "top_logprobs");
        let user = extract_string(&mut extra, "user");
        let service_tier = extract_string(&mut extra, "service_tier");
        let store = extra.remove("store").and_then(|v| v.as_bool());
        let request_metadata = extra.remove("metadata");
        let reasoning_effort = extract_string(&mut extra, "reasoning_effort");
        let max_completion_tokens_param = extract_u32(&mut extra, "max_completion_tokens");
        let tool_choice_override = extra.remove("tool_choice");
        let modalities = extra.remove("modalities");
        let audio = extra.remove("audio");

        // For o-series reasoning models, use max_completion_tokens instead of max_tokens
        let (max_tokens, max_completion_tokens) = if let Some(mct) = max_completion_tokens_param {
            // Explicit max_completion_tokens takes priority
            (None, Some(mct))
        } else if self.is_reasoning_model() {
            // Reasoning models require max_completion_tokens
            (None, request.max_tokens)
        } else {
            (request.max_tokens, None)
        };

        // Determine tool_choice: explicit override > auto when tools present > None
        let tool_choice = if let Some(override_val) = tool_choice_override {
            Some(override_val)
        } else if tools.is_some() {
            Some(serde_json::Value::String("auto".to_string()))
        } else {
            None
        };

        let body = OpenAiRequest {
            model: self.model.clone(),
            messages,
            frequency_penalty,
            logit_bias,
            logprobs,
            top_logprobs,
            max_tokens,
            max_completion_tokens,
            n,
            presence_penalty,
            response_format,
            seed,
            service_tier,
            stop,
            store,
            temperature: request.temperature,
            top_p: request.top_p,
            tools: tools.clone(),
            tool_choice,
            parallel_tool_calls,
            user,
            reasoning_effort,
            request_metadata,
            modalities,
            audio,
        };

        // Serialize and merge any remaining extra parameters
        let mut request_json = serde_json::to_value(&body)?;
        if let serde_json::Value::Object(ref mut map) = request_json {
            for (key, value) in extra {
                map.insert(key, value);
            }
        }

        let mut req_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_json);

        if let Some(org) = &self.organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| GraphBitError::llm_provider("openai", format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GraphBitError::llm_provider(
                "openai",
                format!("API error: {error_text}"),
            ));
        }

        let openai_response: OpenAiResponse = response.json().await.map_err(|e| {
            GraphBitError::llm_provider("openai", format!("Failed to parse response: {e}"))
        })?;

        self.parse_response(openai_response)
    }

    fn supports_function_calling(&self) -> bool {
        let m = self.model.as_str();
        // o1-preview and o1-mini do not support function calling
        if m == "o1-preview" || m == "o1-mini" || m.starts_with("o1-preview-") || m.starts_with("o1-mini-") {
            return false;
        }
        // All GPT and o-series models support function calling
        m.starts_with("gpt-")
            || m.starts_with("o1")
            || m.starts_with("o3")
            || m.starts_with("o4")
            || m.starts_with("chatgpt-")
    }

    fn max_context_length(&self) -> Option<u32> {
        let m = self.model.as_str();
        // GPT-4.1 series: 1M tokens
        if m.starts_with("gpt-4.1") {
            Some(1_000_000)
        }
        // O-series reasoning models: 200K tokens
        else if m.starts_with("o1") || m.starts_with("o3") || m.starts_with("o4") {
            Some(200_000)
        }
        // GPT-4o series: 128K tokens
        else if m.starts_with("gpt-4o") {
            Some(128_000)
        }
        // GPT-4.5 preview: 128K tokens
        else if m.starts_with("gpt-4.5") {
            Some(128_000)
        }
        // GPT-4 Turbo: 128K tokens
        else if m == "gpt-4-turbo" || m.starts_with("gpt-4-turbo-") {
            Some(128_000)
        }
        // GPT-4 32K
        else if m == "gpt-4-32k" || m.starts_with("gpt-4-32k-") {
            Some(32_768)
        }
        // GPT-4 base: 8K tokens
        else if m == "gpt-4" || m.starts_with("gpt-4-0") {
            Some(8_192)
        }
        // GPT-3.5 Turbo: 16K tokens
        else if m.starts_with("gpt-3.5-turbo") {
            Some(16_384)
        }
        // ChatGPT models
        else if m.starts_with("chatgpt-") {
            Some(128_000)
        }
        else {
            None
        }
    }

    fn cost_per_token(&self) -> Option<(f64, f64)> {
        let m = self.model.as_str();
        // GPT-4.1 nano: $0.10/$0.40 per 1M tokens
        if m.starts_with("gpt-4.1-nano") {
            Some((0.000_000_1, 0.000_000_4))
        }
        // GPT-4.1 mini: $0.40/$1.60 per 1M tokens
        else if m.starts_with("gpt-4.1-mini") {
            Some((0.000_000_4, 0.000_001_6))
        }
        // GPT-4.1: $2.00/$8.00 per 1M tokens
        else if m.starts_with("gpt-4.1") {
            Some((0.000_002, 0.000_008))
        }
        // GPT-4o mini: $0.15/$0.60 per 1M tokens
        else if m.starts_with("gpt-4o-mini") {
            Some((0.000_000_15, 0.000_000_6))
        }
        // GPT-4o: $2.50/$10.00 per 1M tokens
        else if m.starts_with("gpt-4o") {
            Some((0.000_002_5, 0.000_01))
        }
        // GPT-4.5 preview: $75.00/$150.00 per 1M tokens
        else if m.starts_with("gpt-4.5") {
            Some((0.000_075, 0.000_15))
        }
        // GPT-4 Turbo: $10.00/$30.00 per 1M tokens
        else if m == "gpt-4-turbo" || m.starts_with("gpt-4-turbo-") {
            Some((0.000_01, 0.000_03))
        }
        // GPT-4 32K: $60.00/$120.00 per 1M tokens
        else if m == "gpt-4-32k" || m.starts_with("gpt-4-32k-") {
            Some((0.000_06, 0.000_12))
        }
        // GPT-4 base: $30.00/$60.00 per 1M tokens
        else if m == "gpt-4" || m.starts_with("gpt-4-0") {
            Some((0.000_03, 0.000_06))
        }
        // GPT-3.5 Turbo: $0.50/$1.50 per 1M tokens
        else if m.starts_with("gpt-3.5-turbo") {
            Some((0.000_000_5, 0.000_001_5))
        }
        // o4-mini: $1.10/$4.40 per 1M tokens
        else if m.starts_with("o4-mini") {
            Some((0.000_001_1, 0.000_004_4))
        }
        // o3-pro: $20.00/$80.00 per 1M tokens
        else if m.starts_with("o3-pro") {
            Some((0.000_02, 0.000_08))
        }
        // o3-mini: $1.10/$4.40 per 1M tokens
        else if m.starts_with("o3-mini") {
            Some((0.000_001_1, 0.000_004_4))
        }
        // o3: $10.00/$40.00 per 1M tokens
        else if m.starts_with("o3") {
            Some((0.000_01, 0.000_04))
        }
        // o1-mini: $3.00/$12.00 per 1M tokens
        else if m == "o1-mini" || m.starts_with("o1-mini-") {
            Some((0.000_003, 0.000_012))
        }
        // o1-preview: $15.00/$60.00 per 1M tokens
        else if m == "o1-preview" || m.starts_with("o1-preview-") {
            Some((0.000_015, 0.000_06))
        }
        // o1: $15.00/$60.00 per 1M tokens
        else if m.starts_with("o1") {
            Some((0.000_015, 0.000_06))
        }
        else {
            None
        }
    }
}

// --- Helper functions ---

/// Build an optimized HTTP client with connection pooling
fn build_client() -> GraphBitResult<Client> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| {
            GraphBitError::llm_provider("openai", format!("Failed to create HTTP client: {e}"))
        })
}

/// Extract an f32 value from extra_params
fn extract_f32(extra: &mut HashMap<String, serde_json::Value>, key: &str) -> Option<f32> {
    extra.remove(key).and_then(|v| v.as_f64().map(|f| f as f32))
}

/// Extract a u32 value from extra_params
fn extract_u32(extra: &mut HashMap<String, serde_json::Value>, key: &str) -> Option<u32> {
    extra.remove(key).and_then(|v| v.as_u64().map(|n| n as u32))
}

/// Extract a String value from extra_params
fn extract_string(extra: &mut HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
    extra.remove(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s),
        _ => None,
    })
}

// --- OpenAI API types ---

/// Request body for the OpenAI Chat Completions API
#[derive(Debug, Serialize)]
struct OpenAiRequest {
    /// ID of the model to use
    model: String,
    /// Messages comprising the conversation
    messages: Vec<OpenAiMessage>,
    /// Penalize new tokens based on their frequency in the text so far (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    /// Modify the likelihood of specified tokens appearing (token ID to bias -100 to 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<serde_json::Value>,
    /// Whether to return log probabilities of the output tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,
    /// Number of most likely tokens to return at each position (0-20, requires logprobs)
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<u32>,
    /// Maximum number of tokens to generate (legacy, use max_completion_tokens for o-series)
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    /// Upper bound for tokens including visible output and reasoning tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    /// How many chat completion choices to generate for each input message
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    /// Penalize new tokens based on whether they appear in the text so far (-2.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    /// Output format control (ResponseFormatText, ResponseFormatJSONSchema, ResponseFormatJSONObject)
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<serde_json::Value>,
    /// Seed for deterministic sampling (best effort)
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
    /// Processing type: "auto", "default", "flex", or "priority"
    #[serde(skip_serializing_if = "Option::is_none")]
    service_tier: Option<String>,
    /// Up to 4 sequences where the API will stop generating further tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<serde_json::Value>,
    /// Whether to store the output for model distillation or evals
    #[serde(skip_serializing_if = "Option::is_none")]
    store: Option<bool>,
    /// Sampling temperature (0 to 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// List of tools (functions) the model may call
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAiTool>>,
    /// Controls which function is called: "none", "auto", "required", or specific function
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<serde_json::Value>,
    /// Whether to enable parallel function calling during tool use
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,
    /// End-user identifier for abuse detection and caching
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    /// Reasoning effort for o-series models: "low", "medium", "high"
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    /// Key-value metadata to attach to the request
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    request_metadata: Option<serde_json::Value>,
    /// Output types, e.g. ["text", "audio"] for audio-capable models
    #[serde(skip_serializing_if = "Option::is_none")]
    modalities: Option<serde_json::Value>,
    /// Audio output parameters (format, voice) when audio modality is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    #[serde(deserialize_with = "deserialize_nullable_content")]
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiToolCall {
    id: String,
    r#type: String,
    function: OpenAiFunction,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiTool {
    r#type: String,
    function: OpenAiFunctionDef,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    id: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
    #[serde(default)]
    system_fingerprint: Option<String>,
    #[serde(default)]
    service_tier: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    #[serde(default)]
    completion_tokens_details: Option<OpenAiCompletionTokensDetails>,
    #[serde(default)]
    prompt_tokens_details: Option<OpenAiPromptTokensDetails>,
}

/// Detailed breakdown of completion token usage (includes reasoning tokens for o-series)
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiCompletionTokensDetails {
    #[serde(default)]
    reasoning_tokens: Option<u32>,
    #[serde(default)]
    accepted_prediction_tokens: Option<u32>,
    #[serde(default)]
    rejected_prediction_tokens: Option<u32>,
}

/// Detailed breakdown of prompt token usage
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiPromptTokensDetails {
    #[serde(default)]
    cached_tokens: Option<u32>,
}

/// Custom deserializer for nullable content field
/// `OpenAI` returns null for content when tool calls are made
fn deserialize_nullable_content<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
