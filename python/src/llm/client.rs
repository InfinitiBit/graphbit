//! LLM client for GraphBit Python bindings

use futures::StreamExt;
use graphbit_core::llm::{LlmMessage, LlmProviderTrait, LlmRequest};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::config::LlmConfig;
use crate::errors::to_py_error;
use crate::runtime::get_runtime;

#[pyclass]
pub struct LlmClient {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
}

#[pymethods]
impl LlmClient {
    #[new]
    fn new(config: LlmConfig) -> PyResult<Self> {
        let provider =
            graphbit_core::llm::LlmProviderFactory::create_provider(config.inner.clone())
                .map_err(to_py_error)?;

        Ok(Self {
            provider: Arc::new(RwLock::new(provider)),
        })
    }

    /// Pure async - maximum performance
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_async<'a>(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider_guard = provider.read().await;
            let response = provider_guard
                .complete(request)
                .await
                .map_err(to_py_error)?;

            Ok(response.content)
        })
    }

    /// Optimized sync with minimal overhead
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> PyResult<String> {
        let provider = Arc::clone(&self.provider);

        // Direct runtime access - no spawn_blocking overhead
        get_runtime().block_on(async move {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let provider_guard = provider.read().await;
            let response = provider_guard
                .complete(request)
                .await
                .map_err(to_py_error)?;

            Ok(response.content)
        })
    }

    /// Ultra-fast batch processing
    #[pyo3(signature = (prompts, max_tokens=None, temperature=None, max_concurrency=None))]
    fn complete_batch<'a>(
        &self,
        prompts: Vec<String>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        max_concurrency: Option<usize>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let requests: Vec<LlmRequest> = prompts
                .into_iter()
                .map(|prompt| {
                    let mut req = LlmRequest::new(prompt);
                    if let Some(tokens) = max_tokens {
                        req = req.with_max_tokens(tokens);
                    }
                    if let Some(temp) = temperature {
                        req = req.with_temperature(temp);
                    }
                    req
                })
                .collect();

            let concurrency = max_concurrency.unwrap_or(10); // Higher default

            // Maximum performance streaming
            let results = futures::stream::iter(requests)
                .map(|request| {
                    let provider = Arc::clone(&provider);
                    async move {
                        let guard = provider.read().await;
                        guard.complete(request).await
                    }
                })
                .buffer_unordered(concurrency)
                .collect::<Vec<_>>()
                .await;

            let responses: Vec<String> = results
                .into_iter()
                .map(|res| match res {
                    Ok(response) => response.content,
                    Err(e) => format!("Error: {}", e),
                })
                .collect();

            Ok(responses)
        })
    }

    /// Fast chat completion
    #[pyo3(signature = (messages, max_tokens=None, temperature=None))]
    fn chat_optimized<'a>(
        &self,
        messages: Vec<(String, String)>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let llm_messages: Vec<LlmMessage> = messages
                .into_iter()
                .map(|(role, content)| match role.as_str() {
                    "system" => LlmMessage::system(content),
                    "assistant" => LlmMessage::assistant(content),
                    _ => LlmMessage::user(content),
                })
                .collect();

            let mut request = LlmRequest::with_messages(llm_messages);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let guard = provider.read().await;
            let response = guard.complete(request).await.map_err(to_py_error)?;

            Ok(response.content)
        })
    }

    /// High-performance streaming
    #[pyo3(signature = (prompt, max_tokens=None, temperature=None))]
    fn complete_stream<'a>(
        &self,
        prompt: String,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            let guard = provider.read().await;

            // Try streaming with pre-allocated buffer
            match guard.stream(request.clone()).await {
                Ok(mut stream) => {
                    let mut content = String::with_capacity(1024);
                    while let Some(chunk_result) = stream.next().await {
                        if let Ok(chunk) = chunk_result {
                            content.push_str(&chunk.content);
                        }
                    }
                    Ok(content)
                }
                Err(_) => {
                    // Fast fallback to regular completion
                    let response = guard.complete(request).await.map_err(to_py_error)?;
                    Ok(response.content)
                }
            }
        })
    }

    fn get_stats<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("runtime", "zero-overhead-async")?;
        Ok(dict)
    }

    fn warmup<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let warmup_request = LlmRequest::new("".to_string()).with_max_tokens(1);
            let guard = provider.read().await;
            let _ = guard.complete(warmup_request).await;
            Ok(true)
        })
    }
}
