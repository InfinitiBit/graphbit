//! JavaScript / Node.js bindings for the `GraphBit` memory layer via NAPI-RS.

#![allow(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::OnceLock;

// Re-use the tokio runtime across calls.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime for graphbit-js")
    })
}

// ---------------------------------------------------------------------------
// Data objects exposed to JS
// ---------------------------------------------------------------------------

/// A single message in a conversation.
#[napi(object)]
pub struct JsMessage {
    pub role: String,
    pub content: String,
}

/// Scope filter for memory isolation.
#[napi(object)]
pub struct JsScope {
    pub user_id: Option<String>,
    pub agent_id: Option<String>,
    pub run_id: Option<String>,
}

/// A stored memory fact.
#[napi(object)]
pub struct JsMemory {
    pub id: String,
    pub content: String,
    pub user_id: Option<String>,
    pub agent_id: Option<String>,
    pub run_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: String,
}

/// A memory with its similarity score.
#[napi(object)]
pub struct JsScoredMemory {
    pub memory: JsMemory,
    pub score: f64,
}

/// A historical record of a memory mutation.
#[napi(object)]
pub struct JsMemoryHistory {
    pub memory_id: String,
    pub old_content: String,
    pub new_content: String,
    pub action: String,
    pub timestamp: String,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn core_memory_to_js(m: graphbit_core::memory::Memory) -> JsMemory {
    JsMemory {
        id: m.id.to_string(),
        content: m.content,
        user_id: m.scope.user_id,
        agent_id: m.scope.agent_id,
        run_id: m.scope.run_id,
        created_at: m.created_at.to_rfc3339(),
        updated_at: m.updated_at.to_rfc3339(),
        metadata: serde_json::to_string(&m.metadata).unwrap_or_else(|_| "{}".to_string()),
    }
}

fn core_scored_to_js(s: graphbit_core::memory::ScoredMemory) -> JsScoredMemory {
    JsScoredMemory {
        memory: core_memory_to_js(s.memory),
        score: s.score,
    }
}

fn core_history_to_js(h: graphbit_core::memory::MemoryHistory) -> JsMemoryHistory {
    JsMemoryHistory {
        memory_id: h.memory_id.to_string(),
        old_content: h.old_content,
        new_content: h.new_content,
        action: h.action.to_string(),
        timestamp: h.timestamp.to_rfc3339(),
    }
}

fn js_scope_to_core(scope: Option<JsScope>) -> graphbit_core::memory::MemoryScope {
    match scope {
        Some(s) => graphbit_core::memory::MemoryScope {
            user_id: s.user_id,
            agent_id: s.agent_id,
            run_id: s.run_id,
        },
        None => graphbit_core::memory::MemoryScope::default(),
    }
}

fn to_napi_error(e: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(e.to_string())
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Configuration for the memory subsystem.
#[napi]
pub struct JsMemoryConfig {
    inner: graphbit_core::memory::MemoryConfig,
}

#[napi]
impl JsMemoryConfig {
    /// Create a new memory configuration.
    ///
    /// `llm_provider` - one of "openai", "anthropic", etc.
    /// `llm_api_key` - API key for the LLM provider.
    /// `llm_model` - model name (e.g. "gpt-4o-mini").
    /// `embedding_provider` - one of "openai", "huggingface".
    /// `embedding_api_key` - API key for the embedding provider.
    /// `embedding_model` - model name (e.g. "text-embedding-3-small").
    #[napi(constructor)]
    pub fn new(
        llm_provider: String,
        llm_api_key: String,
        llm_model: Option<String>,
        embedding_api_key: String,
        embedding_model: Option<String>,
        db_path: Option<String>,
        similarity_threshold: Option<f64>,
    ) -> Result<Self> {
        let llm_config = build_llm_config(
            &llm_provider,
            &llm_api_key,
            &llm_model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        )?;

        let embedding_config = graphbit_core::embeddings::EmbeddingConfig {
            provider: graphbit_core::embeddings::EmbeddingProvider::OpenAI,
            api_key: embedding_api_key,
            model: embedding_model.unwrap_or_else(|| "text-embedding-3-small".to_string()),
            base_url: None,
            timeout_seconds: None,
            max_batch_size: None,
            extra_params: HashMap::new(),
        };

        let mut config = graphbit_core::memory::MemoryConfig::new(llm_config, embedding_config);

        if let Some(path) = db_path {
            config = config.with_db_path(path);
        }
        if let Some(threshold) = similarity_threshold {
            config = config.with_similarity_threshold(threshold);
        }

        Ok(Self { inner: config })
    }
}

fn build_llm_config(
    provider: &str,
    api_key: &str,
    model: &str,
) -> Result<graphbit_core::llm::LlmConfig> {
    use graphbit_core::llm::LlmConfig;
    match provider.to_lowercase().as_str() {
        "openai" => Ok(LlmConfig::openai(api_key, model)),
        "anthropic" => Ok(LlmConfig::anthropic(api_key, model)),
        "deepseek" => Ok(LlmConfig::deepseek(api_key, model)),
        other => Err(napi::Error::from_reason(format!(
            "Unsupported LLM provider for JS bindings: {other}"
        ))),
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// High-level memory client for JavaScript consumers.
#[napi]
pub struct JsMemoryClient {
    service: Arc<graphbit_core::memory::MemoryService>,
}

#[napi]
impl JsMemoryClient {
    /// Create a new client. This opens the database and loads existing memories.
    #[napi(factory)]
    pub async fn create(config: &JsMemoryConfig) -> Result<Self> {
        let cfg = config.inner.clone();
        let service = graphbit_core::memory::MemoryService::new(cfg)
            .await
            .map_err(to_napi_error)?;
        Ok(Self {
            service: Arc::new(service),
        })
    }

    /// Extract and store facts from conversation messages.
    #[napi]
    pub async fn add(
        &self,
        messages: Vec<JsMessage>,
        scope: Option<JsScope>,
    ) -> Result<Vec<JsMemory>> {
        let llm_messages: Vec<graphbit_core::llm::LlmMessage> = messages
            .into_iter()
            .map(|m| match m.role.as_str() {
                "system" => graphbit_core::llm::LlmMessage::system(m.content),
                "assistant" => graphbit_core::llm::LlmMessage::assistant(m.content),
                _ => graphbit_core::llm::LlmMessage::user(m.content),
            })
            .collect();

        let core_scope = js_scope_to_core(scope);
        let memories = self
            .service
            .add(&llm_messages, &core_scope)
            .await
            .map_err(to_napi_error)?;

        Ok(memories.into_iter().map(core_memory_to_js).collect())
    }

    /// Search for memories similar to a query.
    #[napi]
    pub async fn search(
        &self,
        query: String,
        scope: Option<JsScope>,
        top_k: Option<u32>,
    ) -> Result<Vec<JsScoredMemory>> {
        let core_scope = js_scope_to_core(scope);
        let results = self
            .service
            .search(&query, &core_scope, top_k.unwrap_or(10) as usize)
            .await
            .map_err(to_napi_error)?;

        Ok(results.into_iter().map(core_scored_to_js).collect())
    }

    /// Get a single memory by ID.
    #[napi]
    pub async fn get(&self, memory_id: String) -> Result<Option<JsMemory>> {
        let id = graphbit_core::memory::MemoryId::from_string(&memory_id)
            .map_err(|e| to_napi_error(format!("Invalid memory ID: {e}")))?;
        let mem = self.service.get(&id).await.map_err(to_napi_error)?;
        Ok(mem.map(core_memory_to_js))
    }

    /// Get all memories matching a scope.
    #[napi]
    pub async fn get_all(&self, scope: Option<JsScope>) -> Result<Vec<JsMemory>> {
        let core_scope = js_scope_to_core(scope);
        let memories = self
            .service
            .get_all(&core_scope)
            .await
            .map_err(to_napi_error)?;
        Ok(memories.into_iter().map(core_memory_to_js).collect())
    }

    /// Update a memory's content.
    #[napi]
    pub async fn update(&self, memory_id: String, content: String) -> Result<JsMemory> {
        let id = graphbit_core::memory::MemoryId::from_string(&memory_id)
            .map_err(|e| to_napi_error(format!("Invalid memory ID: {e}")))?;
        let mem = self
            .service
            .update(&id, &content)
            .await
            .map_err(to_napi_error)?;
        Ok(core_memory_to_js(mem))
    }

    /// Delete a single memory.
    #[napi]
    pub async fn delete(&self, memory_id: String) -> Result<()> {
        let id = graphbit_core::memory::MemoryId::from_string(&memory_id)
            .map_err(|e| to_napi_error(format!("Invalid memory ID: {e}")))?;
        self.service.delete(&id).await.map_err(to_napi_error)
    }

    /// Delete all memories matching a scope.
    #[napi]
    pub async fn delete_all(&self, scope: Option<JsScope>) -> Result<()> {
        let core_scope = js_scope_to_core(scope);
        self.service
            .delete_all(&core_scope)
            .await
            .map_err(to_napi_error)
    }

    /// Get mutation history for a memory.
    #[napi]
    pub async fn history(&self, memory_id: String) -> Result<Vec<JsMemoryHistory>> {
        let id = graphbit_core::memory::MemoryId::from_string(&memory_id)
            .map_err(|e| to_napi_error(format!("Invalid memory ID: {e}")))?;
        let entries = self
            .service
            .history(&id)
            .await
            .map_err(to_napi_error)?;
        Ok(entries.into_iter().map(core_history_to_js).collect())
    }
}
