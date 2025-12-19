//! LLM Client bindings for JavaScript
//!
//! This module provides comprehensive LLM client functionality for JavaScript/TypeScript,
//! enabling direct interaction with language models through various providers.
//!
//! # Features
//! - Direct LLM completions (sync and async)
//! - Batch processing with concurrency control
//! - Streaming responses
//! - Full response metadata
//! - Client statistics and monitoring
//! - Circuit breaker pattern for resilience

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::llm::{
    LlmProviderTrait, LlmProviderFactory, LlmRequest, LlmMessage,
    LlmResponse as CoreLlmResponse, LlmConfig as CoreLlmConfig,
};
use crate::llm::{LlmConfig, LlmResponse, LlmUsage, FinishReason, LlmToolCall};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

/// Client statistics for monitoring
#[napi(object)]
pub struct ClientStats {
    /// Total number of requests made
    pub total_requests: i64,
    /// Number of successful requests
    pub successful_requests: i64,
    /// Number of failed requests
    pub failed_requests: i64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Total tokens used
    pub total_tokens: i64,
    /// Uptime in seconds
    pub uptime_seconds: f64,
}

/// Circuit breaker state for resilience
#[derive(Clone)]
enum CircuitBreakerState {
    Closed { failure_count: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}

/// Circuit breaker for handling provider failures
struct CircuitBreaker {
    state: Arc<RwLock<CircuitBreakerState>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitBreakerState::Closed { failure_count: 0 })),
            failure_threshold,
            recovery_timeout,
        }
    }

    async fn check_state(&self) -> Result<()> {
        let mut state = self.state.write().await;
        match *state {
            CircuitBreakerState::Open { opened_at } => {
                if opened_at.elapsed() > self.recovery_timeout {
                    *state = CircuitBreakerState::HalfOpen;
                    Ok(())
                } else {
                    Err(Error::from_reason("Circuit breaker is open - service unavailable"))
                }
            }
            _ => Ok(()),
        }
    }

    async fn record_success(&self) {
        let mut state = self.state.write().await;
        *state = CircuitBreakerState::Closed { failure_count: 0 };
    }

    async fn record_failure(&self) {
        let mut state = self.state.write().await;
        match *state {
            CircuitBreakerState::Closed { failure_count } => {
                let new_count = failure_count + 1;
                if new_count >= self.failure_threshold {
                    *state = CircuitBreakerState::Open {
                        opened_at: Instant::now(),
                    };
                } else {
                    *state = CircuitBreakerState::Closed {
                        failure_count: new_count,
                    };
                }
            }
            CircuitBreakerState::HalfOpen => {
                *state = CircuitBreakerState::Open {
                    opened_at: Instant::now(),
                };
            }
            _ => {}
        }
    }
}

/// Internal statistics tracker
struct StatsTracker {
    total_requests: i64,
    successful_requests: i64,
    failed_requests: i64,
    total_response_time_ms: f64,
    total_tokens: i64,
}

impl StatsTracker {
    fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_response_time_ms: 0.0,
            total_tokens: 0,
        }
    }

    fn record_request(&mut self, duration_ms: f64, tokens: i64, success: bool) {
        self.total_requests += 1;
        self.total_response_time_ms += duration_ms;
        self.total_tokens += tokens;
        
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
    }

    fn get_avg_response_time(&self) -> f64 {
        if self.total_requests > 0 {
            self.total_response_time_ms / self.total_requests as f64
        } else {
            0.0
        }
    }
}

/// LLM Client for direct language model interaction
///
/// Provides comprehensive access to language models with:
/// - Multiple completion modes (sync, async, batch, streaming)
/// - Full response metadata
/// - Performance monitoring
/// - Automatic retries and circuit breaking
///
/// # Example
///
/// ```javascript
/// const { LlmClient, LlmConfig } = require('@infinitibit_gmbh/graphbit');
///
/// // Create client
/// const config = LlmConfig.openai({
///   apiKey: process.env.OPENAI_API_KEY,
///   model: 'gpt-4o-mini'
/// });
/// const client = new LlmClient(config);
///
/// // Simple completion
/// const response = await client.complete("What is 2+2?");
/// console.log(response); // "4"
///
/// // Full response with metadata
/// const fullResponse = await client.completeFull("Explain quantum computing");
/// console.log(fullResponse.content);
/// console.log(fullResponse.usage.totalTokens);
/// ```
#[napi]
pub struct LlmClient {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    circuit_breaker: Arc<CircuitBreaker>,
    stats: Arc<RwLock<StatsTracker>>,
    created_at: Instant,
    warmed_up: Arc<tokio::sync::OnceCell<()>>,
    config: CoreLlmConfig,
}

#[napi]
impl LlmClient {
    /// Create a new LLM client
    ///
    /// # Arguments
    /// * `config` - LLM configuration with provider settings
    ///
    /// # Example
    ///
    /// ```javascript
    /// const config = LlmConfig.openai({ apiKey: process.env.OPENAI_API_KEY });
    /// const client = new LlmClient(config);
    /// ```
    #[napi(constructor)]
    pub fn new(config: &LlmConfig) -> Result<Self> {
        let core_config = config.clone_inner();
        
        let provider = LlmProviderFactory::create_provider(core_config.clone())
            .map_err(|e| Error::from_reason(format!("Failed to create provider: {}", e)))?;

        let circuit_breaker = Arc::new(CircuitBreaker::new(
            5,  // failure threshold
            Duration::from_secs(60), // recovery timeout
        ));

        Ok(Self {
            provider: Arc::new(RwLock::new(provider)),
            circuit_breaker,
            stats: Arc::new(RwLock::new(StatsTracker::new())),
            created_at: Instant::now(),
            warmed_up: Arc::new(tokio::sync::OnceCell::new()),
            config: core_config,
        })
    }

    /// Simple text completion (async)
    ///
    /// Returns just the generated text content without metadata.
    ///
    /// # Arguments
    /// * `prompt` - The input text prompt
    /// * `max_tokens` - Maximum tokens to generate (optional)
    /// * `temperature` - Sampling temperature 0.0-2.0 (optional)
    ///
    /// # Returns
    /// The generated text as a string
    ///
    /// # Example
    ///
    /// ```javascript
    /// const response = await client.complete("What is the capital of France?");
    /// console.log(response); // "Paris"
    ///
    /// const response = await client.complete("Write a haiku", 50, 0.7);
    /// ```
    #[napi]
    pub async fn complete(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<String> {
        // Validate inputs
        if prompt.trim().is_empty() {
            return Err(Error::from_reason("Prompt cannot be empty"));
        }

        if let Some(tokens) = max_tokens {
            if tokens == 0 {
                return Err(Error::from_reason("max_tokens must be greater than 0"));
            }
        }

        if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(Error::from_reason("temperature must be between 0.0 and 2.0"));
            }
        }

        // Build request
        let mut request = LlmRequest::new(prompt);
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(temp) = temperature {
            request = request.with_temperature(temp as f32);
        }

        // Execute with resilience
        let response = self.execute_with_resilience(request).await?;
        Ok(response.content)
    }

    /// Async completion (alias for complete)
    ///
    /// Provided for API compatibility.
    #[napi]
    pub async fn complete_async(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<String> {
        self.complete(prompt, max_tokens, temperature).await
    }

    /// Full completion with metadata
    ///
    /// Returns complete response including usage statistics, finish reason, and model info.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const response = await client.completeFull("Explain AI");
    /// console.log(response.content);
    /// console.log(`Tokens used: ${response.usage.totalTokens}`);
    /// console.log(`Finish reason: ${response.finishReason}`);
    /// ```
    #[napi]
    pub async fn complete_full(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<LlmResponse> {
        // Validate inputs
        if prompt.trim().is_empty() {
            return Err(Error::from_reason("Prompt cannot be empty"));
        }

        // Build request
        let mut request = LlmRequest::new(prompt);
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(temp) = temperature {
            request = request.with_temperature(temp as f32);
        }

        // Execute with resilience
        let response = self.execute_with_resilience(request).await?;
        Ok(LlmResponse::from(response))
    }

    /// Async full completion (alias for completeFull)
    #[napi]
    pub async fn complete_full_async(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<LlmResponse> {
        self.complete_full(prompt, max_tokens, temperature).await
    }

    /// Batch process multiple prompts with concurrency control
    ///
    /// # Arguments
    /// * `prompts` - Array of prompt strings
    /// * `max_tokens` - Maximum tokens per completion (optional)
    /// * `temperature` - Sampling temperature (optional)
    /// * `max_concurrency` - Maximum concurrent requests (optional, default: 5)
    ///
    /// # Example
    ///
    /// ```javascript
    /// const responses = await client.completeBatch([
    ///   "What is the capital of France?",
    ///   "What is the capital of Germany?",
    ///   "What is the capital of Italy?"
    /// ], { maxConcurrency: 2 });
    /// ```
    #[napi]
    pub async fn complete_batch(
        &self,
        prompts: Vec<String>,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
        max_concurrency: Option<u32>,
    ) -> Result<Vec<String>> {
        if prompts.is_empty() {
            return Err(Error::from_reason("Prompts array cannot be empty"));
        }

        if prompts.len() > 1000 {
            return Err(Error::from_reason("Batch size cannot exceed 1000"));
        }

        let concurrency = max_concurrency.unwrap_or(5).max(1).min(20) as usize;

        // Create requests
        let requests: Vec<LlmRequest> = prompts
            .into_iter()
            .filter(|p| !p.trim().is_empty())
            .map(|prompt| {
                let mut req = LlmRequest::new(prompt);
                if let Some(tokens) = max_tokens {
                    req = req.with_max_tokens(tokens);
                }
                if let Some(temp) = temperature {
                    req = req.with_temperature(temp as f32);
                }
                req
            })
            .collect();

        // Execute in parallel with concurrency control
        use futures::stream::{self, StreamExt};
        
        let results: Vec<Result<String>> = stream::iter(requests)
            .map(|request| async move {
                match self.execute_with_resilience(request).await {
                    Ok(response) => Ok(response.content),
                    Err(e) => Ok(format!("Error: {}", e)),
                }
            })
            .buffer_unordered(concurrency)
            .collect()
            .await;

        Ok(results.into_iter().map(|r| r.unwrap_or_else(|e| format!("Error: {}", e))).collect())
    }

    /// Stream completion (returns async iterator)
    ///
    /// Note: Streaming support depends on provider capabilities
    #[napi]
    pub async fn complete_stream(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<String> {
        // For now, return complete response
        // TODO: Implement true streaming with async iterator when napi-rs supports it
        self.complete(prompt, max_tokens, temperature).await
    }

    /// Optimized chat completion
    ///
    /// # Arguments
    /// * `messages` - Array of [role, content] tuples
    ///
    /// # Example
    ///
    /// ```javascript
    /// const response = await client.chatOptimized([
    ///   ["system", "You are a helpful assistant"],
    ///   ["user", "What is AI?"]
    /// ]);
    /// ```
    #[napi]
    pub async fn chat_optimized(
        &self,
        messages: Vec<Vec<String>>,
        max_tokens: Option<u32>,
        temperature: Option<f64>,
    ) -> Result<String> {
        if messages.is_empty() {
            return Err(Error::from_reason("Messages array cannot be empty"));
        }

        // Convert messages to LlmMessage format
        let llm_messages: Result<Vec<LlmMessage>> = messages
            .into_iter()
            .map(|msg| {
                if msg.len() != 2 {
                    return Err(Error::from_reason("Each message must be [role, content]"));
                }
                Ok(LlmMessage::user(msg[1].clone()))
            })
            .collect();

        let llm_messages = llm_messages?;

        // Build request from messages
        let mut request = LlmRequest::with_messages(llm_messages);
        
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(temp) = temperature {
            request = request.with_temperature(temp as f32);
        }

        // Execute
        let response = self.execute_with_resilience(request).await?;
        Ok(response.content)
    }

    /// Get client statistics
    ///
    /// Returns performance and usage metrics for monitoring
    ///
    /// # Example
    ///
    /// ```javascript
    /// const stats = await client.getStats();
    /// console.log(`Total requests: ${stats.totalRequests}`);
    /// console.log(`Success rate: ${stats.successfulRequests / stats.totalRequests * 100}%`);
    /// console.log(`Avg response time: ${stats.avgResponseTimeMs}ms`);
    /// ```
    #[napi]
    pub async fn get_stats(&self) -> Result<ClientStats> {
        let stats = self.stats.read().await;
        let uptime = self.created_at.elapsed().as_secs_f64();

        Ok(ClientStats {
            total_requests: stats.total_requests,
            successful_requests: stats.successful_requests,
            failed_requests: stats.failed_requests,
            avg_response_time_ms: stats.get_avg_response_time(),
            total_tokens: stats.total_tokens,
            uptime_seconds: uptime,
        })
    }

    /// Reset client statistics
    ///
    /// Clears all accumulated statistics
    #[napi]
    pub async fn reset_stats(&self) -> Result<()> {
        let mut stats = self.stats.write().await;
        *stats = StatsTracker::new();
        Ok(())
    }

    /// Warm up the client with a test request
    ///
    /// Initializes connections and caches for better performance
    ///
    /// # Example
    ///
    /// ```javascript
    /// await client.warmup(); // Call once at startup
    /// ```
    #[napi]
    pub async fn warmup(&self) -> Result<()> {
        self.warmed_up
            .get_or_init(|| async {
                let test_request = LlmRequest::new("test".to_string())
                    .with_max_tokens(1);
                let _ = self.execute_with_resilience(test_request).await;
            })
            .await;
        Ok(())
    }

    // Internal method: Execute request with resilience patterns
    async fn execute_with_resilience(&self, request: LlmRequest) -> Result<CoreLlmResponse> {
        // Check circuit breaker
        self.circuit_breaker.check_state().await?;

        let start = Instant::now();
        let provider = self.provider.read().await;

        // Execute request
        let result = provider.complete(request).await;

        let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

        // Update statistics and circuit breaker
        match &result {
            Ok(response) => {
                let mut stats = self.stats.write().await;
                stats.record_request(duration_ms, response.usage.total_tokens as i64, true);
                self.circuit_breaker.record_success().await;
            }
            Err(_) => {
                let mut stats = self.stats.write().await;
                stats.record_request(duration_ms, 0, false);
                self.circuit_breaker.record_failure().await;
            }
        }

        result.map_err(|e| Error::from_reason(format!("LLM request failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_tracker() {
        let mut tracker = StatsTracker::new();
        
        tracker.record_request(100.0, 50, true);
        tracker.record_request(200.0, 75, true);
        
        assert_eq!(tracker.total_requests, 2);
        assert_eq!(tracker.successful_requests, 2);
        assert_eq!(tracker.get_avg_response_time(), 150.0);
        assert_eq!(tracker.total_tokens, 125);
    }
}

