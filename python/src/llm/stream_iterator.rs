/// Python async iterator for streaming LLM responses
#[pyclass]
struct StreamIterator {
    provider: Arc<RwLock<Box<dyn LlmProviderTrait>>>,
    config: ClientConfig,
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[pymethods]
impl StreamIterator {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let provider = Arc::clone(&self.provider);
        let config = self.config.clone();
        let prompt = self.prompt.clone();
        let max_tokens = self.max_tokens;
        let temperature = self.temperature;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Build request on first call
            let mut request = LlmRequest::new(prompt);
            if let Some(tokens) = max_tokens {
                request = request.with_max_tokens(tokens);
            }
            if let Some(temp) = temperature {
                request = request.with_temperature(temp);
            }

            // Get stream from provider
            let guard = provider.read().await;
            let mut stream = guard.stream(request).await.map_err(to_py_error)?;

            // Get next chunk
            if let Some(result) = stream.next().await {
                match result {
                    Ok(response) => Ok(response.content),
                    Err(e) => {
                        if config.debug {
                            warn!("Stream chunk error: {}", e);
                        }
                        Err(to_py_error(e))
                    }
                }
            } else {
                // Stream ended
                Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(
                    "Stream ended",
                ))
            }
        })
    }
}
