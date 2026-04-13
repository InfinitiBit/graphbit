use crate::errors::{GraphBitError, GraphBitResult};
use reqwest::Client;
use std::time::Duration;

/// Shared HTTP policy defaults for OpenAI-shape providers.
#[derive(Debug, Clone)]
pub(crate) struct OpenAiCompatHttpPolicy {
    pub(crate) timeout: Duration,
    pub(crate) pool_max_idle_per_host: usize,
    pub(crate) pool_idle_timeout: Duration,
    pub(crate) tcp_keepalive: Duration,
}

impl Default for OpenAiCompatHttpPolicy {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(60),
            pool_max_idle_per_host: 10,
            pool_idle_timeout: Duration::from_secs(30),
            tcp_keepalive: Duration::from_secs(60),
        }
    }
}

/// Build a reqwest client with shared timeout and pooling defaults.
pub(crate) fn build_http_client(
    provider_name: &str,
    policy: Option<OpenAiCompatHttpPolicy>,
) -> GraphBitResult<Client> {
    let p = policy.unwrap_or_default();
    Client::builder()
        .timeout(p.timeout)
        .pool_max_idle_per_host(p.pool_max_idle_per_host)
        .pool_idle_timeout(p.pool_idle_timeout)
        .tcp_keepalive(p.tcp_keepalive)
        .build()
        .map_err(|e| {
            GraphBitError::llm_provider(
                provider_name,
                format!("Failed to create HTTP client: {e}"),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_builds_client() {
        let client = build_http_client("openai", None).expect("client should build");
        let _ = client;
    }

    #[test]
    fn custom_policy_builds_client() {
        let policy = OpenAiCompatHttpPolicy {
            timeout: Duration::from_secs(30),
            pool_max_idle_per_host: 4,
            pool_idle_timeout: Duration::from_secs(10),
            tcp_keepalive: Duration::from_secs(15),
        };
        let client = build_http_client("deepseek", Some(policy)).expect("client should build");
        let _ = client;
    }
}
