//! Retry configuration and error classification.

use serde::{Deserialize, Serialize};

use crate::errors::GraphBitError;

use super::DEFAULT_TIMEOUT_MS;

/// Retry configuration for node execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (0 means no retries)
    pub max_attempts: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Backoff multiplier for exponential backoff (e.g., 2.0 for doubling)
    pub backoff_multiplier: f64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Jitter factor to add randomness (0.0 to 1.0)
    pub jitter_factor: f64,
    /// Types of errors that should trigger retries
    pub retryable_errors: Vec<RetryableErrorType>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: DEFAULT_TIMEOUT_MS,
            jitter_factor: 0.1,
            retryable_errors: vec![
                RetryableErrorType::NetworkError,
                RetryableErrorType::TimeoutError,
                RetryableErrorType::TemporaryUnavailable,
                RetryableErrorType::InternalServerError,
            ],
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: DEFAULT_TIMEOUT_MS,
            jitter_factor: 0.1,
            retryable_errors: vec![
                RetryableErrorType::NetworkError,
                RetryableErrorType::TimeoutError,
                RetryableErrorType::TemporaryUnavailable,
                RetryableErrorType::InternalServerError,
            ],
        }
    }

    /// Configure exponential backoff
    pub fn with_exponential_backoff(
        mut self,
        initial_delay_ms: u64,
        multiplier: f64,
        max_delay_ms: u64,
    ) -> Self {
        self.initial_delay_ms = initial_delay_ms;
        self.backoff_multiplier = multiplier;
        self.max_delay_ms = max_delay_ms;
        self
    }

    /// Set jitter factor
    #[inline]
    pub fn with_jitter(mut self, jitter_factor: f64) -> Self {
        self.jitter_factor = jitter_factor.clamp(0.0, 1.0);
        self
    }

    /// Set retryable error types
    #[inline]
    pub fn with_retryable_errors(mut self, errors: Vec<RetryableErrorType>) -> Self {
        self.retryable_errors = errors;
        self
    }

    /// Calculate delay for a given attempt with exponential backoff and jitter
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0;
        }

        let base_delay = (self.initial_delay_ms as f64
            * self.backoff_multiplier.powi(attempt as i32 - 1))
        .min(self.max_delay_ms as f64);

        let jitter = if self.jitter_factor > 0.0 {
            let max_jitter = base_delay * self.jitter_factor;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(-max_jitter..=max_jitter)
        } else {
            0.0
        };

        ((base_delay + jitter).max(0.0) as u64).min(self.max_delay_ms)
    }

    /// Check if an error should trigger a retry
    #[inline]
    pub fn should_retry(&self, error: &GraphBitError, attempt: u32) -> bool {
        if attempt >= self.max_attempts {
            return false;
        }

        let error_type = RetryableErrorType::from_error(error);
        self.retryable_errors.contains(&error_type)
    }
}

/// Types of errors that can potentially be retried
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RetryableErrorType {
    /// Network connectivity issues
    NetworkError,
    /// Request timeout errors
    TimeoutError,
    /// Rate limiting from external services
    RateLimitError,
    /// Temporary service unavailability
    TemporaryUnavailable,
    /// Internal server errors (5xx)
    InternalServerError,
    /// Authentication/authorization that might be temporary
    AuthenticationError,
    /// Resource conflicts that might resolve
    ResourceConflict,
    /// All other errors (use with caution)
    Other,
}

impl RetryableErrorType {
    /// Determine retry type from error
    pub fn from_error(error: &GraphBitError) -> Self {
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("timeout") || error_str.contains("timed out") {
            Self::TimeoutError
        } else if error_str.contains("network") || error_str.contains("connection") {
            Self::NetworkError
        } else if error_str.contains("rate limit") || error_str.contains("too many requests") {
            Self::RateLimitError
        } else if error_str.contains("unavailable") || error_str.contains("service") {
            Self::TemporaryUnavailable
        } else if error_str.contains("internal server error") || error_str.contains("500") {
            Self::InternalServerError
        } else if error_str.contains("auth") || error_str.contains("unauthorized") {
            Self::AuthenticationError
        } else if error_str.contains("conflict") || error_str.contains("409") {
            Self::ResourceConflict
        } else {
            Self::Other
        }
    }
}
