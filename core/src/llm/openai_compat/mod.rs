//! Shared helpers for OpenAI-shape providers.
//!
//! This module centralizes reusable pieces used by providers that expose
//! OpenAI-compatible request/response and SSE streaming semantics.

pub(crate) mod finish_reason;
pub(crate) mod http;
pub(crate) mod complete;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod sse;
pub(crate) mod stream_tools;
