//! Core type definitions for `GraphBit`
//!
//! This module contains all the fundamental types used throughout the
//! `GraphBit` agentic workflow automation framework.

mod circuit_breaker;
mod concurrency;
mod context;
mod execution;
mod ids;
mod message;
mod retry;

// Re-export constants
/// Default timeout for operations (30 seconds)
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
/// Default recovery timeout for circuit breakers (1 minute)
pub const DEFAULT_RECOVERY_TIMEOUT_MS: u64 = 60_000;
/// Default failure window for circuit breakers (5 minutes)
pub const DEFAULT_FAILURE_WINDOW_MS: u64 = 300_000;

// Re-export all types so the public API is unchanged
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState};
pub use concurrency::{
    ConcurrencyConfig, ConcurrencyManager, ConcurrencyPermits, ConcurrencyStats, TaskInfo,
};
pub use context::{WorkflowContext, WorkflowState};
pub use execution::{
    AgentCapability, NodeExecutionResult, WorkflowExecutionStats,
};
pub use ids::{AgentId, NodeId, WorkflowId};
pub use message::{AgentMessage, MessageContent};
pub use retry::{RetryConfig, RetryableErrorType};
