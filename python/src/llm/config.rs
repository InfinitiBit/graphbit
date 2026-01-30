//! LLM configuration for GraphBit Python bindings

use crate::validation::validate_api_key;
use graphbit_core::llm::providers::{register_python_instance, unregister_python_instance};
use graphbit_core::llm::LlmConfig as CoreLlmConfig;
use pyo3::prelude::*;
use uuid::Uuid;

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
    fn azurellm(
        api_key: String,
        deployment_name: String,
        endpoint: String,
        api_version: Option<String>,
    ) -> PyResult<Self> {
        validate_api_key(&api_key, "Azure LLM")?;

        Ok(Self {
            inner: CoreLlmConfig::azurellm(
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
    fn huggingface(api_key: String, model: Option<String>) -> PyResult<Self> {
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

            let py_object: PyObject = hf_instance.into();
            let instance_arc = std::sync::Arc::new(py_object);
            let instance_id = Uuid::new_v4().to_string();

            // Register the instance globally
            register_python_instance(instance_id.clone(), instance_arc.clone());

            let config = CoreLlmConfig::PythonBridge {
                python_instance: Some(instance_arc),
                model: model.unwrap_or_else(|| "microsoft/DialoGPT-medium".to_string()),
                instance_id: Some(instance_id),
            };

            Ok(Self { inner: config })
        })
    }

    #[staticmethod]
    #[pyo3(signature = (api_key=None, model=None))]
    fn litellm(api_key: Option<String>, model: Option<String>) -> PyResult<Self> {
        // API key is optional for LiteLLM as it can use environment variables
        if let Some(ref key) = api_key {
            validate_api_key(key, "LiteLLM")?;
        }

        Python::with_gil(|py| {
            // Import the Python LiteLLM class
            let litellm_module = py.import("graphbit.providers.litellm.llm").map_err(|e| {
                pyo3::exceptions::PyImportError::new_err(format!(
                    "Failed to import LiteLLM module: {e}"
                ))
            })?;
            let litellm_class = litellm_module.getattr("LiteLLMLLM").map_err(|e| {
                pyo3::exceptions::PyAttributeError::new_err(format!(
                    "Failed to get LiteLLMLLM class: {e}"
                ))
            })?;

            // Create instance with optional API key
            let litellm_instance = if let Some(key) = api_key {
                litellm_class.call1((key,)).map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Failed to create LiteLLMLLM instance: {e}"
                    ))
                })?
            } else {
                litellm_class.call0().map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Failed to create LiteLLMLLM instance: {e}"
                    ))
                })?
            };

            let config = CoreLlmConfig::PythonBridge {
                python_instance: Some(std::sync::Arc::new(litellm_instance.into())),
                model: model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
                instance_id: Some(Uuid::new_v4().to_string()),
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

    /// Cleanup the Python instance from the global registry
    ///
    /// Call this method when you're done using a HuggingFace or other PythonBridge
    /// configuration to free memory and Python resources.
    ///
    /// Returns True if an instance was removed, False otherwise.
    ///
    /// Example:
    ///     config = LlmConfig.huggingface(api_key="...", model="...")
    ///     # ... use config ...
    ///     config.cleanup()  # Frees resources
    fn cleanup(&self) -> bool {
        if let CoreLlmConfig::PythonBridge {
            instance_id: Some(ref id),
            ..
        } = self.inner
        {
            unregister_python_instance(id)
        } else {
            false
        }
    }
    /// Context manager entry point
    ///
    /// Enables `with` statement usage for automatic resource cleanup:
    ///
    /// Example:
    ///     with LlmConfig.huggingface(api_key="...", model="...") as config:
    ///         # use config
    ///     # cleanup() called automatically
    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Context manager exit point
    ///
    /// Automatically calls `cleanup()` when exiting the `with` block.
    /// Returns False to not suppress any exceptions.
    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    fn __exit__(
        &self,
        _exc_type: Option<PyObject>,
        _exc_val: Option<PyObject>,
        _exc_tb: Option<PyObject>,
    ) -> bool {
        self.cleanup();
        false
    }
}
