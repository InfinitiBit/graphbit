//! Production-grade LLM client for GraphBit Python bindings
//!
//! This module provides a robust, high-performance LLM client with:
//! - Connection pooling and reuse
//! - Circuit breaker pattern for resilience
//! - Automatic retry with exponential backoff
//! - Comprehensive timeout handling
//! - Request/response validation
//! - Performance monitoring and metrics

use futures::{Stream, StreamExt};
use graphbit_core::errors::GraphBitResult;
use graphbit_core::llm::{LlmMessage, LlmProviderTrait, LlmRequest};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;
use tracing::{debug, info, instrument, warn};

use super::config::LlmConfig;
use super::response::PyLlmResponse;
use crate::errors::{timeout_error, to_py_error, validation_error};
use crate::runtime::get_runtime;

/// Client configuration for production environments
#[derive(Debug, Clone)]
pub(crate) struct ClientConfig {
    /// Request timeout in seconds
    pub request_timeout: Duration,
    /// Maximum retries for failed requests
    pub max_retries: u32,
    /// Base delay for exponential backoff
    pub base_retry_delay: Duration,
    /// Maximum delay for exponential backoff
    pub max_retry_delay: Duration,
    /// Enable circuit breaker
    pub circuit_breaker_enabled: bool,
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker recovery timeout
    pub circuit_breaker_recovery_timeout: Duration,
    /// Enable debug output
    pub debug: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            request_timeout: Duration::from_secs(120), // Increased to match `Ollama`'s capabilities
            max_retries: 3,
            base_retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_secs(5),
            circuit_breaker_enabled: true,
            circuit_breaker_threshold: 5,
            circuit_breaker_recovery_timeout: Duration::from_secs(60),
            debug: false,
        }
    }
}

/// Circuit breaker state for resilience
#[derive(Debug, Clone)]
enum CircuitBreakerState {
    Closed { failure_count: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}

/// Circuit breaker implementation
#[derive(Debug)]
struct CircuitBreaker {
    state: Arc<RwLock<CircuitBreakerState>>,
    config: ClientConfig,
}

impl CircuitBreaker {
    fn new(config: ClientConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitBreakerState::Closed {
                failure_count: 0,
            })),
            config,
        }
    }

    async fn can_execute(&self) -> bool {
        if !self.config.circuit_breaker_enabled {
            return true;
        }

        let mut state = self.state.write().await;
        match *state {
            CircuitBreakerState::Closed { .. } => true,
            CircuitBreakerState::Open { opened_at } => {
                if opened_at.elapsed() > self.config.circuit_breaker_recovery_timeout {
                    *state = CircuitBreakerState::HalfOpen;
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    async fn record_success(&self) {
        if !self.config.circuit_breaker_enabled {
            return;
        }

        let mut state = self.state.write().await;
        *state = CircuitBreakerState::Closed { failure_count: 0 };
    }

    async fn record_failure(&self) {
        if !self.config.circuit_breaker_enabled {
            return;
        }

        let mut state = self.state.write().await;
        match *state {
            CircuitBreakerState::Closed { failure_count } => {
                let new_count = failure_count + 1;
                if new_count >= self.config.circuit_breaker_threshold {
                    *state = CircuitBreakerState::Open {
                        opened_at: Instant::now(),
                    };
                    if self.config.debug {
                        warn!("Circuit breaker opened due to {} failures", new_count);
                    }
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
                if self.config.debug {
                    warn!("Circuit breaker reopened after failed recovery attempt");
                }
            }
            _ => {}
        }
    }
}

/// Client statistics for monitoring
#[derive(Debug, Clone)]
pub(crate) struct ClientStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub circuit_breaker_state: String,
    pub uptime: Duration,
}

/// Production-grade LLM client with resilience patterns
#[pyclass]
pub struct LlmClient {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    circuit_breaker: Arc<CircuitBreaker>,
    config: ClientConfig,
    stats: Arc<RwLock<ClientStats>>,
    created_at: Instant,
    /// Cache for warmup to avoid repeated initialization overhead
    warmed_up: Arc<tokio::sync::OnceCell<()>>,
}

#[pymethods]
impl LlmClient {
    #[new]
    #[pyo3(signature = (config, debug=None))]
    fn new(config: LlmConfig, debug: Option<bool>) -> PyResult<Self> {
        let mut client_config = ClientConfig {
            debug: debug.unwrap_or(false),
            ..Default::default()
        };

        // Optimize timeout based on provider type
        match &config.inner {
            graphbit_core::llm::providers::LlmConfig::Ollama { .. } => {
                // `Ollama` needs more time for local inference
                client_config.request_timeout = Duration::from_secs(180);
            }
            graphbit_core::llm::providers::LlmConfig::OpenAI { .. }
            | graphbit_core::llm::providers::LlmConfig::Anthropic { .. }
            | graphbit_core::llm::providers::LlmConfig::AzureLlm { .. }
            | graphbit_core::llm::providers::LlmConfig::Perplexity { .. }
            | graphbit_core::llm::providers::LlmConfig::Fireworks { .. }
            | graphbit_core::llm::providers::LlmConfig::ByteDance { .. }
            | graphbit_core::llm::providers::LlmConfig::Ai21 { .. }
            | graphbit_core::llm::providers::LlmConfig::MistralAI { .. }
            | graphbit_core::llm::providers::LlmConfig::Xai { .. } => {
                // Cloud APIs are typically faster
                client_config.request_timeout = Duration::from_secs(60);
            }
            _ => {
                // Keep default for other providers
            }
        }

        let provider =
            graphbit_core::llm::LlmProviderFactory::create_provider(config.inner.clone())
                .map_err(to_py_error)?;

        let circuit_breaker = Arc::new(CircuitBreaker::new(client_config.clone()));

        let stats = Arc::new(RwLock::new(ClientStats {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            circuit_breaker_state: "Closed".to_string(),
            uptime: Duration::from_secs(0),
        }));

        if client_config.debug {
            info!("Created LLM client with config: {:?}", client_config);
        }

        Ok(Self {
            provider: Arc::new(RwLock::new(provider)),
            circuit_breaker,
            config: client_config,
            stats,
            created_at: Instant::now(),
            warmed_up: Arc::new(tokio::sync::OnceCell::new()),
        })
    }

    /// High-performance async completion with full resilience
    #[instrument(skip(self, py), fields(prompt_len = prompt.len()))]
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_async<'a>(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // Validate input
        if prompt.is_empty() {
            return Err(validation_error(
                "prompt",
                Some(&prompt),
                "Prompt cannot be empty",
            ));
        }

        // Validate max_tokens
        let validated_max_tokens = if let Some(tokens) = max_tokens {
            if tokens <= 0 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens must be greater than 0",
                ));
            }
            if tokens > u32::MAX as i64 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens is too large",
                ));
            }
            Some(tokens as u32)
        } else {
            None
        };

        // Validate temperature
        let validated_temperature = if let Some(temp) = temperature {
            if temp < 0.0 || temp > 2.0 {
                return Err(validation_error(
                    "temperature",
                    Some(&temp.to_string()),
                    "temperature must be between 0.0 and 2.0",
                ));
            }
            Some(temp as f32)
        } else {
            None
        };

        let provider = Arc::clone(&self.provider);
        let circuit_breaker = Arc::clone(&self.circuit_breaker);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Self::execute_with_resilience(
                provider,
                circuit_breaker,
                stats,
                config,
                prompt,
                validated_max_tokens,
                validated_temperature,
            )
            .await
        })
    }

    /// Synchronous completion with resilience (use sparingly)
    #[instrument(skip(self, py), fields(prompt_len = prompt.len()))]
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'_>,
    ) -> PyResult<String> {
        // Validate input
        if prompt.is_empty() {
            return Err(validation_error(
                "prompt",
                Some(&prompt),
                "Prompt cannot be empty",
            ));
        }

        // Validate max_tokens
        let validated_max_tokens = if let Some(tokens) = max_tokens {
            if tokens <= 0 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens must be greater than 0",
                ));
            }
            if tokens > u32::MAX as i64 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens is too large",
                ));
            }
            Some(tokens as u32)
        } else {
            None
        };

        // Validate temperature
        let validated_temperature = if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(validation_error(
                    "temperature",
                    Some(&temp.to_string()),
                    "temperature must be between 0.0 and 2.0",
                ));
            }
            Some(temp as f32)
        } else {
            None
        };

        let provider = Arc::clone(&self.provider);
        let circuit_breaker = Arc::clone(&self.circuit_breaker);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        // CRITICAL FIX: Release GIL during async execution for true parallelism
        py.allow_threads(|| {
            get_runtime().block_on(async move {
                Self::execute_with_resilience(
                    provider,
                    circuit_breaker,
                    stats,
                    config,
                    prompt,
                    validated_max_tokens,
                    validated_temperature,
                )
                .await
            })
        })
    }

    /// Ultra-fast batch processing with controlled concurrency
    #[instrument(skip(self, py), fields(batch_size = prompts.len()))]
    #[pyo3(signature = (prompts, max_tokens=None, temperature=None, max_concurrency=None))]
    fn complete_batch<'a>(
        &self,
        prompts: Vec<String>,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        max_concurrency: Option<usize>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // Validate batch size
        if prompts.is_empty() {
            return Err(validation_error(
                "prompts",
                None,
                "Prompts list cannot be empty",
            ));
        }

        if prompts.len() > 1000 {
            return Err(validation_error(
                "prompts",
                Some(&prompts.len().to_string()),
                "Batch size cannot exceed 1000",
            ));
        }

        // Validate max_tokens
        let validated_max_tokens = if let Some(tokens) = max_tokens {
            if tokens <= 0 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens must be greater than 0",
                ));
            }
            if tokens > u32::MAX as i64 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens is too large",
                ));
            }
            Some(tokens as u32)
        } else {
            None
        };

        // Validate temperature
        let validated_temperature = if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(validation_error(
                    "temperature",
                    Some(&temp.to_string()),
                    "temperature must be between 0.0 and 2.0",
                ));
            }
            Some(temp as f32)
        } else {
            None
        };

        // Validate max_concurrency
        if let Some(concurrency) = max_concurrency {
            if concurrency == 0 {
                return Err(validation_error(
                    "max_concurrency",
                    Some(&concurrency.to_string()),
                    "max_concurrency must be greater than 0",
                ));
            }
        }

        let provider = Arc::clone(&self.provider);
        let circuit_breaker = Arc::clone(&self.circuit_breaker);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Pre-allocate with exact capacity for better memory performance
            let mut requests = Vec::with_capacity(prompts.len());
            for prompt in prompts {
                if prompt.is_empty() {
                    continue; // Skip empty prompts
                }

                let mut req = LlmRequest::new(prompt);
                if let Some(tokens) = validated_max_tokens {
                    req = req.with_max_tokens(tokens);
                }
                if let Some(temp) = validated_temperature {
                    req = req.with_temperature(temp);
                }
                requests.push(req);
            }

            // Adaptive concurrency based on system capabilities
            let concurrency = max_concurrency.unwrap_or_else(|| {
                let cpu_count = num_cpus::get();
                (cpu_count * 2).clamp(5, 20) // Between 5 and 20
            });

            if config.debug {
                info!(
                    "Processing batch of {} requests with concurrency {}",
                    requests.len(),
                    concurrency
                );
            }

            // High-performance streaming with buffered execution and resilience
            let results = futures::stream::iter(requests)
                .map(|request| {
                    let provider = Arc::clone(&provider);
                    let circuit_breaker = Arc::clone(&circuit_breaker);
                    let stats = Arc::clone(&stats);
                    let config = config.clone();

                    async move {
                        Self::execute_request_with_resilience(
                            provider,
                            circuit_breaker,
                            stats,
                            config,
                            request,
                        )
                        .await
                    }
                })
                .buffer_unordered(concurrency)
                .collect::<Vec<_>>()
                .await;

            // Pre-allocate response vector for efficiency
            let mut responses = Vec::with_capacity(results.len());
            for res in results {
                match res {
                    Ok(response) => responses.push(response.content),
                    Err(e) => {
                        if config.debug {
                            warn!("Batch request failed: {}", e);
                        }
                        responses.push(format!("Error: {}", e));
                    }
                }
            }

            Ok(responses)
        })
    }

    /// Optimized chat completion with message validation
    #[instrument(skip(self, py), fields(message_count = messages.len()))]
    #[pyo3(signature = (messages, max_tokens=None, temperature=None))]
    fn chat_optimized<'a>(
        &self,
        messages: Vec<(String, String)>,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // Validate messages
        if messages.is_empty() {
            return Err(validation_error(
                "messages",
                None,
                "Messages list cannot be empty",
            ));
        }

        // Validate max_tokens
        let validated_max_tokens = if let Some(tokens) = max_tokens {
            if tokens <= 0 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens must be greater than 0",
                ));
            }
            if tokens > u32::MAX as i64 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens is too large",
                ));
            }
            Some(tokens as u32)
        } else {
            None
        };

        // Validate temperature
        let validated_temperature = if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(validation_error(
                    "temperature",
                    Some(&temp.to_string()),
                    "temperature must be between 0.0 and 2.0",
                ));
            }
            Some(temp as f32)
        } else {
            None
        };

        let provider = Arc::clone(&self.provider);
        let circuit_breaker = Arc::clone(&self.circuit_breaker);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Pre-allocate with exact capacity and validate messages
            let mut llm_messages = Vec::with_capacity(messages.len());
            for (role, content) in messages {
                if content.is_empty() {
                    continue; // Skip empty messages
                }

                let message = match role.as_str() {
                    "system" => LlmMessage::system(content),
                    "assistant" => LlmMessage::assistant(content),
                    "user" => LlmMessage::user(content),
                    _ => LlmMessage::user(content),
                };
                llm_messages.push(message);
            }

            if llm_messages.is_empty() {
                return Err(validation_error(
                    "messages",
                    None,
                    "No valid messages found",
                ));
            }

            let mut request = LlmRequest::with_messages(llm_messages);
            if let Some(tokens) = validated_max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = validated_temperature {
                request = request.with_temperature(temp);
            }

            Self::execute_request_with_resilience(provider, circuit_breaker, stats, config, request)
                .await
                .map(|response| response.content)
        })
    }

    /// Stream completion with TRUE real-time token streaming
    /// Returns an async iterator that yields chunks as they arrive from the LLM
    ///
    /// Usage in Python:
    ///     async for chunk in client.stream("prompt"):
    ///         print(chunk, end='', flush=True)
    #[instrument(skip(self), fields(prompt_len = prompt.len()))]
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn stream(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
    ) -> PyResult<StreamIterator> {
        // Validate input
        if prompt.is_empty() {
            return Err(validation_error(
                "prompt",
                Some(&prompt),
                "Prompt cannot be empty",
            ));
        }

        // Validate max_tokens
        let validated_max_tokens = if let Some(tokens) = max_tokens {
            if tokens <= 0 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens must be greater than 0",
                ));
            }
            if tokens > u32::MAX as i64 {
                return Err(validation_error(
                    "max_tokens",
                    Some(&tokens.to_string()),
                    "max_tokens is too large",
                ));
            }
            Some(tokens as u32)
        } else {
            None
        };

        // Validate temperature
        let validated_temperature = if let Some(temp) = temperature {
            if !(0.0..=2.0).contains(&temp) {
                return Err(validation_error(
                    "temperature",
                    Some(&temp.to_string()),
                    "temperature must be between 0.0 and 2.0",
                ));
            }
            Some(temp as f32)
        } else {
            None
        };

        Ok(StreamIterator::new(
            Arc::clone(&self.provider),
            self.config.clone(),
            prompt,
            validated_max_tokens,
            validated_temperature,
        ))
    }

    /// Stream completion (alias for backward compatibility)
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_stream<'a>(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let iterator = self.stream(prompt, max_tokens, temperature)?;
        Ok(Bound::new(py, iterator)?.into_any())
    }

    /// Get comprehensive client statistics
    fn get_stats<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let stats = get_runtime().block_on(async {
            let mut stats = self.stats.read().await.clone();
            stats.uptime = self.created_at.elapsed();

            // Get circuit breaker state
            let cb_state = self.circuit_breaker.state.read().await;
            stats.circuit_breaker_state = match *cb_state {
                CircuitBreakerState::Closed { failure_count } => {
                    format!("Closed (failures: {})", failure_count)
                }
                CircuitBreakerState::Open { .. } => "Open".to_string(),
                CircuitBreakerState::HalfOpen => "HalfOpen".to_string(),
            };

            stats
        });

        let dict = PyDict::new(py);
        dict.set_item("total_requests", stats.total_requests)?;
        dict.set_item("successful_requests", stats.successful_requests)?;
        dict.set_item("failed_requests", stats.failed_requests)?;
        dict.set_item(
            "success_rate",
            if stats.total_requests > 0 {
                stats.successful_requests as f64 / stats.total_requests as f64
            } else {
                0.0
            },
        )?;
        dict.set_item("average_response_time_ms", stats.average_response_time_ms)?;
        dict.set_item("circuit_breaker_state", stats.circuit_breaker_state)?;
        dict.set_item("uptime_seconds", stats.uptime.as_secs())?;

        Ok(dict)
    }

    /// Warmup the client with a test request
    fn warmup<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let warmed_up = Arc::clone(&self.warmed_up);
        let provider = Arc::clone(&self.provider);
        let debug = self.config.debug; // Capture debug flag before async move

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            warmed_up
                .get_or_init(|| async {
                    if debug {
                        info!("Warming up LLM client...");
                    }
                    let test_request = LlmRequest::new("Hello".to_string());
                    let guard = provider.read().await;

                    if let Err(e) = guard.complete(test_request).await {
                        if debug {
                            warn!(
                                "Warmup request failed (this is expected for some providers): {}",
                                e
                            );
                        }
                    }

                    if debug {
                        info!("LLM client warmup completed");
                    }
                })
                .await;

            Ok("Client warmed up successfully".to_string())
        })
    }

    /// Reset client statistics
    fn reset_stats(&self) -> PyResult<()> {
        get_runtime().block_on(async {
            let mut stats = self.stats.write().await;
            stats.total_requests = 0;
            stats.successful_requests = 0;
            stats.failed_requests = 0;
            stats.average_response_time_ms = 0.0;
        });
        Ok(())
    }

    /// Complete with full response object (synchronous)
    #[instrument(skip(self, py), fields(prompt_len = prompt.len()))]
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_full(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'_>,
    ) -> PyResult<PyLlmResponse> {
        // Validate input
        if prompt.is_empty() {
            return Err(validation_error(
                "prompt",
                Some(&prompt),
                "Prompt cannot be empty",
            ));
        }

        let validated_max_tokens = max_tokens
            .map(|t| {
                if t <= 0 {
                    Err(validation_error(
                        "max_tokens",
                        Some(&t.to_string()),
                        "max_tokens must be positive",
                    ))
                } else if t > 100_000 {
                    Err(validation_error(
                        "max_tokens",
                        Some(&t.to_string()),
                        "max_tokens cannot exceed 100,000",
                    ))
                } else {
                    Ok(t as u32)
                }
            })
            .transpose()?;

        let validated_temperature = temperature
            .map(|t| {
                if !(0.0..=2.0).contains(&t) {
                    Err(validation_error(
                        "temperature",
                        Some(&t.to_string()),
                        "temperature must be between 0.0 and 2.0",
                    ))
                } else {
                    Ok(t as f32)
                }
            })
            .transpose()?;

        // CRITICAL FIX: Release GIL during async execution for true parallelism
        let response = py.allow_threads(|| {
            get_runtime().block_on(Self::execute_with_resilience_full(
                Arc::clone(&self.provider),
                Arc::clone(&self.circuit_breaker),
                Arc::clone(&self.stats),
                self.config.clone(),
                prompt,
                validated_max_tokens,
                validated_temperature,
            ))
        })?;

        Ok(PyLlmResponse::from(response))
    }

    /// Complete with full response object (asynchronous)
    #[instrument(skip(self, py), fields(prompt_len = prompt.len()))]
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_full_async<'a>(
        &self,
        prompt: String,
        max_tokens: Option<i64>,
        temperature: Option<f64>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        // Validate input
        if prompt.is_empty() {
            return Err(validation_error(
                "prompt",
                Some(&prompt),
                "Prompt cannot be empty",
            ));
        }

        let validated_max_tokens = max_tokens
            .map(|t| {
                if t <= 0 {
                    Err(validation_error(
                        "max_tokens",
                        Some(&t.to_string()),
                        "max_tokens must be positive",
                    ))
                } else if t > 100_000 {
                    Err(validation_error(
                        "max_tokens",
                        Some(&t.to_string()),
                        "max_tokens cannot exceed 100,000",
                    ))
                } else {
                    Ok(t as u32)
                }
            })
            .transpose()?;

        let validated_temperature = temperature
            .map(|t| {
                if !(0.0..=2.0).contains(&t) {
                    Err(validation_error(
                        "temperature",
                        Some(&t.to_string()),
                        "temperature must be between 0.0 and 2.0",
                    ))
                } else {
                    Ok(t as f32)
                }
            })
            .transpose()?;

        let provider = Arc::clone(&self.provider);
        let circuit_breaker = Arc::clone(&self.circuit_breaker);
        let stats = Arc::clone(&self.stats);
        let config = self.config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let response = Self::execute_with_resilience_full(
                provider,
                circuit_breaker,
                stats,
                config,
                prompt,
                validated_max_tokens,
                validated_temperature,
            )
            .await?;

            Ok(PyLlmResponse::from(response))
        })
    }
}

impl LlmClient {
    /// Core execution method with full resilience patterns
    async fn execute_with_resilience(
        provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
        circuit_breaker: Arc<CircuitBreaker>,
        stats: Arc<RwLock<ClientStats>>,
        config: ClientConfig,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> PyResult<String> {
        let mut request = LlmRequest::new(prompt);
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(temp) = temperature {
            request = request.with_temperature(temp);
        }

        let response = Self::execute_request_with_resilience(
            provider,
            circuit_breaker,
            stats,
            config,
            request,
        )
        .await?;

        Ok(response.content)
    }

    /// Core execution method with full resilience patterns (returns full response)
    async fn execute_with_resilience_full(
        provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
        circuit_breaker: Arc<CircuitBreaker>,
        stats: Arc<RwLock<ClientStats>>,
        config: ClientConfig,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> PyResult<graphbit_core::llm::LlmResponse> {
        let mut request = LlmRequest::new(prompt);
        if let Some(tokens) = max_tokens {
            request = request.with_max_tokens(tokens);
        }
        if let Some(temp) = temperature {
            request = request.with_temperature(temp);
        }

        Self::execute_request_with_resilience(provider, circuit_breaker, stats, config, request)
            .await
    }

    /// Execute a single request with retry logic and circuit breaker
    async fn execute_request_with_resilience(
        provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
        circuit_breaker: Arc<CircuitBreaker>,
        stats: Arc<RwLock<ClientStats>>,
        config: ClientConfig,
        request: LlmRequest,
    ) -> Result<graphbit_core::llm::LlmResponse, PyErr> {
        let start_time = Instant::now();

        // Update stats
        {
            let mut stats_guard = stats.write().await;
            stats_guard.total_requests += 1;
        }

        // Check circuit breaker
        if !circuit_breaker.can_execute().await {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Circuit breaker is open, request rejected",
            ));
        }

        let mut last_error = None;
        let mut delay = config.base_retry_delay;

        for attempt in 0..=config.max_retries {
            if config.debug {
                debug!("Executing LLM request (attempt {})", attempt + 1);
            }

            let result = timeout(config.request_timeout, async {
                let guard = provider.read().await;
                guard.complete(request.clone()).await
            })
            .await;

            match result {
                Ok(Ok(response)) => {
                    // Success - update stats and circuit breaker
                    let duration = start_time.elapsed();

                    {
                        let mut stats_guard = stats.write().await;
                        stats_guard.successful_requests += 1;

                        // Update average response time (simple moving average)
                        let total_requests = stats_guard.total_requests as f64;
                        stats_guard.average_response_time_ms =
                            (stats_guard.average_response_time_ms * (total_requests - 1.0)
                                + duration.as_millis() as f64)
                                / total_requests;
                    }

                    circuit_breaker.record_success().await;
                    if config.debug {
                        info!("LLM request completed successfully in {:?}", duration);
                    }

                    return Ok(response);
                }
                Ok(Err(e)) => {
                    if config.debug {
                        warn!("LLM request failed (attempt {}): {}", attempt + 1, e);
                    }
                    last_error = Some(to_py_error(e));
                    circuit_breaker.record_failure().await;
                }
                Err(_) => {
                    if config.debug {
                        warn!(
                            "LLM request timed out (attempt {}) after {:?}",
                            attempt + 1,
                            config.request_timeout
                        );
                    }
                    let timeout_err = timeout_error(
                        "llm_request",
                        config.request_timeout.as_millis() as u64,
                        "Request timed out",
                    );
                    last_error = Some(timeout_err);
                    circuit_breaker.record_failure().await;
                }
            }

            // Don't wait after the last attempt
            if attempt < config.max_retries {
                tokio::time::sleep(delay).await;
                delay = (delay * 2).min(config.max_retry_delay);
            }
        }

        // Update failure stats
        {
            let mut stats_guard = stats.write().await;
            stats_guard.failed_requests += 1;
        }

        Err(last_error.unwrap_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("All retry attempts failed")
        }))
    }
}

/// Python async iterator for true real-time streaming
/// Each iterator owns its own stream - no shared state between different iterators
///
/// IMPORTANT: Uses OnceCell for initialization to avoid holding mutex across await points
/// which would cause deadlocks in concurrent scenarios.
#[pyclass]
pub struct StreamIterator {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    // OnceCell for one-time lazy initialization - thread-safe without holding locks
    initialized: Arc<tokio::sync::OnceCell<()>>,
    // Stream is created once per iterator and consumed
    // Mutex is ONLY held briefly for polling, never across provider.stream() call
    stream: Arc<
        tokio::sync::Mutex<
            Option<
                Pin<Box<dyn Stream<Item = GraphBitResult<graphbit_core::llm::LlmResponse>> + Send>>,
            >,
        >,
    >,
}

impl StreamIterator {
    fn new(
        provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
        _config: ClientConfig, // Unused but kept for API compatibility
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Self {
        Self {
            provider,
            prompt,
            max_tokens,
            temperature,
            initialized: Arc::new(tokio::sync::OnceCell::new()),
            stream: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}

#[pymethods]
impl StreamIterator {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);
        let prompt = self.prompt.clone();
        let max_tokens = self.max_tokens;
        let temperature = self.temperature;
        let stream = Arc::clone(&self.stream);
        let initialized = Arc::clone(&self.initialized);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Phase 1: Initialize stream if needed (using OnceCell - no lock held during init)
            // This ensures initialization happens exactly once, without holding mutex
            let init_result = initialized
                .get_or_try_init(|| async {
                    // Build request
                    let mut request = LlmRequest::new(prompt.clone());
                    if let Some(tokens) = max_tokens {
                        request = request.with_max_tokens(tokens);
                    }
                    if let Some(temp) = temperature {
                        request = request.with_temperature(temp);
                    }

                    // Get stream from provider (NO MUTEX HELD during this await!)
                    let guard = provider.read().await;
                    let provider_stream = guard.stream(request).await.map_err(to_py_error)?;
                    drop(guard); // Explicitly drop read lock before acquiring mutex

                    // Now briefly hold mutex just to store the stream
                    let mut stream_guard = stream.lock().await;
                    *stream_guard = Some(Box::pin(provider_stream));
                    drop(stream_guard); // Explicitly release mutex

                    Ok::<(), PyErr>(())
                })
                .await;

            // Check if initialization failed
            init_result?;

            // Phase 2: Get next chunk (brief mutex hold for polling only)
            let mut stream_guard = stream.lock().await;
            if let Some(ref mut s) = *stream_guard {
                // Poll for next item - this is quick, no network I/O here
                // The actual network I/O happens inside the stream's internal state
                if let Some(result) = s.next().await {
                    match result {
                        Ok(response) => Ok(response.content),
                        Err(e) => {
                            // Always propagate errors - don't swallow them
                            Err(to_py_error(e))
                        }
                    }
                } else {
                    // Stream ended
                    Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(
                        "Stream ended",
                    ))
                }
            } else {
                // Should never happen after OnceCell initialization
                Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Stream not initialized",
                ))
            }
        })
    }

    /// Stream and print the response in real-time
    /// Returns the complete response as a string after streaming
    ///
    /// Usage in Python:
    ///     response = await chunks.streaming_response()
    ///     # Chunks are printed in real-time, then full response is returned
    fn streaming_response<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);
        let prompt = self.prompt.clone();
        let max_tokens = self.max_tokens;
        let temperature = self.temperature;
        let stream = Arc::clone(&self.stream);
        let initialized = Arc::clone(&self.initialized);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Phase 1: Initialize stream if needed (using OnceCell - no lock held during init)
            let init_result = initialized
                .get_or_try_init(|| async {
                    // Build request
                    let mut request = LlmRequest::new(prompt.clone());
                    if let Some(tokens) = max_tokens {
                        request = request.with_max_tokens(tokens);
                    }
                    if let Some(temp) = temperature {
                        request = request.with_temperature(temp);
                    }

                    // Get stream from provider (NO MUTEX HELD during this await!)
                    let guard = provider.read().await;
                    let provider_stream = guard.stream(request).await.map_err(to_py_error)?;
                    drop(guard); // Explicitly drop read lock before acquiring mutex

                    // Now briefly hold mutex just to store the stream
                    let mut stream_guard = stream.lock().await;
                    *stream_guard = Some(Box::pin(provider_stream));
                    drop(stream_guard); // Explicitly release mutex

                    Ok::<(), PyErr>(())
                })
                .await;

            // Check if initialization failed
            init_result?;

            // Phase 2: Collect and print chunks in real-time
            let mut full_response = String::new();

            // Hold mutex for streaming - but now we're just polling, not doing heavy I/O
            let mut stream_guard = stream.lock().await;
            if let Some(ref mut s) = *stream_guard {
                while let Some(result) = s.next().await {
                    match result {
                        Ok(response) => {
                            // Print chunk immediately (to stdout) - handle errors gracefully
                            print!("{}", response.content);
                            use std::io::Write;
                            let _ = std::io::stdout().flush(); // Don't panic if flush fails

                            // Accumulate for final response
                            full_response.push_str(&response.content);
                        }
                        Err(e) => {
                            // Always propagate errors
                            return Err(to_py_error(e));
                        }
                    }
                }
            }

            Ok(full_response)
        })
    }
}
