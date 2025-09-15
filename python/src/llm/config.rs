//! LLM configuration for GraphBit Python bindings

use crate::validation::validate_api_key;
use graphbit_core::llm::LlmConfig as CoreLlmConfig;
use pyo3::prelude::*;

/// Configuration for LLM providers and models
#[pyclass]
#[derive(Clone)]
pub struct LlmConfig {
    pub(crate) inner: CoreLlmConfig,
}

#[pymethods]
impl LlmConfig {
    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn openai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenAI")?;

        Ok(Self {
            inner: CoreLlmConfig::openai(
                api_key,
                model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn anthropic(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "Anthropic")?;

        Ok(Self {
            inner: CoreLlmConfig::anthropic(
                api_key,
                model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn deepseek(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "DeepSeek")?;

        Ok(Self {
            inner: CoreLlmConfig::deepseek(
                api_key,
                model.unwrap_or_else(|| "deepseek-chat".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None, base_url=None))]
    fn huggingface(
        api_key: String,
        model: Option<String>,
        base_url: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        let mut config =
            CoreLlmConfig::huggingface(api_key, model.unwrap_or_else(|| "gpt2".to_string()));

        // Set custom base URL if provided
        if let CoreLlmConfig::HuggingFace {
            base_url: ref mut url,
            ..
        } = config
        {
            *url = base_url;
        }

        Ok(Self { inner: config })
    }

    #[staticmethod]
    #[pyo3(signature = (model=None))]
    fn ollama(model: Option<String>) -> Self {
        Self {
            inner: CoreLlmConfig::ollama(model.unwrap_or_else(|| "llama3.2".to_string())),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn perplexity(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "Perplexity")?;

        Ok(Self {
            inner: CoreLlmConfig::perplexity(api_key, model.unwrap_or_else(|| "sonar".to_string())),
        })
    }

    fn provider(&self) -> String {
        self.inner.provider_name().to_string()
    }

    fn model(&self) -> String {
        self.inner.model_name().to_string()
    }
}
