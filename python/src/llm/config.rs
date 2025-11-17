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
    #[pyo3(signature = (api_key, deployment_name, endpoint, api_version=None))]
    fn azure_openai(
        api_key: String,
        deployment_name: String,
        endpoint: String,
        api_version: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "Azure OpenAI")?;

        Ok(Self {
            inner: CoreLlmConfig::azure_openai(
                api_key,
                deployment_name,
                endpoint,
                api_version.unwrap_or_else(|| "2024-10-21".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None, base_url=None))]
    fn bytedance(
        api_key: String,
        model: Option<String>,
        base_url: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "ByteDance")?;

        let config = if let Some(base_url) = base_url {
            CoreLlmConfig::bytedance_with_base_url(
                api_key,
                model.unwrap_or_else(|| "seed-1-6-250915".to_string()),
                base_url,
            )
        } else {
            CoreLlmConfig::bytedance(
                api_key,
                model.unwrap_or_else(|| "seed-1-6-250915".to_string()),
            )
        };

        Ok(Self { inner: config })
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

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn openrouter(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenRouter")?;

        Ok(Self {
            inner: CoreLlmConfig::openrouter(
                api_key,
                model.unwrap_or_else(|| "openai/gpt-4o-mini".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None, site_url=None, site_name=None))]
    fn openrouter_with_site(
        api_key: String,
        model: Option<String>,
        site_url: Option<String>,
        site_name: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "OpenRouter")?;

        Ok(Self {
            inner: CoreLlmConfig::openrouter_with_site(
                api_key,
                model.unwrap_or_else(|| "openai/gpt-4o-mini".to_string()),
                site_url,
                site_name,
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn fireworks(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "Fireworks")?;

        Ok(Self {
            inner: CoreLlmConfig::fireworks(
                api_key,
                model.unwrap_or_else(|| {
                    "accounts/fireworks/models/llama-v3p1-8b-instruct".to_string()
                }),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None, organization=None))]
    fn ai21(
        api_key: String,
        model: Option<String>,
        organization: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "AI21")?;

        let config = if let Some(organization) = organization {
            CoreLlmConfig::ai21_with_organization(
                api_key,
                model.unwrap_or_else(|| "jamba-mini".to_string()),
                organization,
            )
        } else {
            CoreLlmConfig::ai21(api_key, model.unwrap_or_else(|| "jamba-mini".to_string()))
        };

        Ok(Self { inner: config })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None, version=None))]
    fn replicate(
        api_key: String,
        model: Option<String>,
        version: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "Replicate")?;

        let config = if let Some(version) = version {
            CoreLlmConfig::replicate_with_version(
                api_key,
                model.unwrap_or_else(|| "openai/gpt-5".to_string()),
                version,
            )
        } else {
            CoreLlmConfig::replicate(api_key, model.unwrap_or_else(|| "openai/gpt-5".to_string()))
        };

        Ok(Self { inner: config })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn togetherai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "TogetherAI")?;

        Ok(Self {
            inner: CoreLlmConfig::togetherai(
                api_key,
                model.unwrap_or_else(|| "openai/gpt-oss-20b".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn xai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "xAI")?;

        Ok(Self {
            inner: CoreLlmConfig::xai(api_key, model.unwrap_or_else(|| "grok-4".to_string())),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn mistralai(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "MistralAI")?;

        Ok(Self {
            inner: CoreLlmConfig::mistralai(
                api_key,
                model.unwrap_or_else(|| "mistral-large-latest".to_string()),
            ),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, model=None))]
    fn huggingface_python(api_key: String, model: Option<String>) -> PyResult<Self> {
        validate_api_key(&api_key, "HuggingFace")?;

        Python::with_gil(|py| {
            // Import the Python HuggingFace class
            let hf_module = py
                .import("graphbit.providers.huggingface.llm")
                .map_err(|e| {
                    pyo3::exceptions::PyImportError::new_err(format!(
                        "Failed to import HuggingFace module: {e}"
                    ))
                })?;
            let hf_class = hf_module.getattr("HuggingfaceLLM").map_err(|e| {
                pyo3::exceptions::PyAttributeError::new_err(format!(
                    "Failed to get HuggingfaceLLM class: {e}"
                ))
            })?;

            // Create instance with API key (token parameter)
            let hf_instance = hf_class.call1((api_key,)).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "Failed to create HuggingfaceLLM instance: {e}"
                ))
            })?;

            let config = CoreLlmConfig::PythonBridge {
                python_instance: std::sync::Arc::new(hf_instance.into()),
                model: model.unwrap_or_else(|| "microsoft/DialoGPT-medium".to_string()),
            };

            Ok(Self { inner: config })
        })
    }

    fn provider(&self) -> String {
        self.inner.provider_name().to_string()
    }

    fn model(&self) -> String {
        self.inner.model_name().to_string()
    }
}
