//! Circuit breaker for error recovery and cascading failure prevention.

use chrono;
use serde::{Deserialize, Serialize};

use super::{DEFAULT_FAILURE_WINDOW_MS, DEFAULT_RECOVERY_TIMEOUT_MS};

/// Circuit breaker configuration for error recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Time in milliseconds to wait before trying again when circuit is open
    pub recovery_timeout_ms: u64,
    /// Number of successful calls needed to close the circuit
    pub success_threshold: u32,
    /// Time window for counting failures in milliseconds
    pub failure_window_ms: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_ms: DEFAULT_RECOVERY_TIMEOUT_MS,
            success_threshold: 3,
            failure_window_ms: DEFAULT_FAILURE_WINDOW_MS,
        }
    }
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open {
        /// Timestamp when the circuit was opened
        opened_at: chrono::DateTime<chrono::Utc>,
    },
    /// Circuit is half-open, testing if service has recovered
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Configuration
    pub config: CircuitBreakerConfig,
    /// Current state
    pub state: CircuitBreakerState,
    /// Failure count in current window
    pub failure_count: u32,
    /// Success count when half-open
    pub success_count: u32,
    /// Last failure time
    pub last_failure: Option<chrono::DateTime<chrono::Utc>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure: None,
        }
    }

    /// Check if a request should be allowed
    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open { opened_at } => {
                let now = chrono::Utc::now();
                let elapsed = now.signed_duration_since(opened_at).num_milliseconds() as u64;

                if elapsed >= self.config.recovery_timeout_ms {
                    self.state = CircuitBreakerState::HalfOpen;
                    self.success_count = 0;
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    /// Record a successful operation
    #[inline]
    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitBreakerState::Open { .. } => {
                self.failure_count = 0;
                self.success_count = 0;
            }
        }
    }

    /// Record a failed operation
    #[inline]
    pub fn record_failure(&mut self) {
        self.last_failure = Some(chrono::Utc::now());

        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitBreakerState::Open {
                        opened_at: chrono::Utc::now(),
                    };
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open {
                    opened_at: chrono::Utc::now(),
                };
                self.failure_count = 1;
                self.success_count = 0;
            }
            CircuitBreakerState::Open { .. } => {}
        }
    }
}
