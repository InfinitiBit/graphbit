//! LLM-based memory extraction for intelligent memory decisions
//!
//! This module uses LLMs to intelligently decide what information should be
//! remembered and how it should be categorized.

use super::types::MemoryType;
use crate::llm::{LlmProvider, LlmRequest};
use crate::errors::GraphBitResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for memory extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    /// Enable extraction
    pub enabled: bool,
    /// Minimum importance threshold for extraction (0.0-1.0)
    pub min_importance: f32,
    /// Maximum tokens for extraction prompt
    pub max_tokens: u32,
    /// Temperature for LLM extraction
    pub temperature: f32,
    /// Extract from every N messages (1 = every message)
    pub extraction_frequency: usize,
}

impl ExtractionConfig {
    /// Create a new extraction configuration
    pub fn new() -> Self {
        Self {
            enabled: true,
            min_importance: 0.5,
            max_tokens: 500,
            temperature: 0.3, // Lower temperature for more consistent extraction
            extraction_frequency: 1,
        }
    }

    /// Create a conservative extraction config (extracts less)
    pub fn conservative() -> Self {
        Self {
            enabled: true,
            min_importance: 0.7,
            max_tokens: 300,
            temperature: 0.2,
            extraction_frequency: 3, // Every 3rd message
        }
    }

    /// Create an aggressive extraction config (extracts more)
    pub fn aggressive() -> Self {
        Self {
            enabled: true,
            min_importance: 0.3,
            max_tokens: 700,
            temperature: 0.4,
            extraction_frequency: 1,
        }
    }

    /// Disable extraction
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            min_importance: 1.0,
            max_tokens: 0,
            temperature: 0.0,
            extraction_frequency: 0,
        }
    }
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of memory extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    /// Whether anything should be remembered
    pub should_remember: bool,
    /// What to remember (if should_remember is true)
    pub content: Option<String>,
    /// Suggested memory type
    pub memory_type: Option<MemoryType>,
    /// Importance score (0.0-1.0)
    pub importance: f32,
    /// Suggested tags
    pub tags: Vec<String>,
    /// Reasoning for the decision
    pub reasoning: Option<String>,
}

impl ExtractionResult {
    /// Create a result indicating nothing should be remembered
    pub fn skip() -> Self {
        Self {
            should_remember: false,
            content: None,
            memory_type: None,
            importance: 0.0,
            tags: Vec::new(),
            reasoning: None,
        }
    }

    /// Create a result with extracted memory
    pub fn remember(
        content: String,
        memory_type: MemoryType,
        importance: f32,
        tags: Vec<String>,
    ) -> Self {
        Self {
            should_remember: true,
            content: Some(content),
            memory_type: Some(memory_type),
            importance: importance.clamp(0.0, 1.0),
            tags,
            reasoning: None,
        }
    }

    /// Add reasoning
    pub fn with_reasoning(mut self, reasoning: String) -> Self {
        self.reasoning = Some(reasoning);
        self
    }
}

/// Memory extractor using LLM
pub struct MemoryExtractor {
    /// LLM provider for extraction
    llm_provider: Arc<LlmProvider>,
    /// Extraction configuration
    config: ExtractionConfig,
    /// Message counter for frequency control
    message_count: usize,
}

impl MemoryExtractor {
    /// Create a new memory extractor
    pub fn new(llm_provider: Arc<LlmProvider>, config: ExtractionConfig) -> Self {
        Self {
            llm_provider,
            config,
            message_count: 0,
        }
    }

    /// Extract memories from a conversation message
    pub async fn extract(&mut self, message: &str, context: Option<&str>) -> GraphBitResult<ExtractionResult> {
        if !self.config.enabled {
            return Ok(ExtractionResult::skip());
        }

        // Check extraction frequency
        self.message_count += 1;
        if self.message_count % self.config.extraction_frequency != 0 {
            return Ok(ExtractionResult::skip());
        }

        // Build extraction prompt
        let prompt = self.build_extraction_prompt(message, context);

        // Call LLM
        let request = LlmRequest::new(&prompt)
            .with_max_tokens(self.config.max_tokens)
            .with_temperature(self.config.temperature);

        let response = self.llm_provider.complete(request).await?;

        // Parse the response
        self.parse_extraction_response(&response.content)
    }

    /// Build the extraction prompt
    fn build_extraction_prompt(&self, message: &str, context: Option<&str>) -> String {
        let context_str = context
            .map(|c| format!("\n\nPrevious Context:\n{}", c))
            .unwrap_or_default();

        format!(
            r#"Analyze the following message and determine if it contains information worth remembering for future interactions.

Message: "{}"{}

Decide:
1. Should this be remembered? (yes/no)
2. If yes, what exactly should be remembered? (extract the key information)
3. What type of memory is this?
   - working: Temporary context for current session
   - factual: Long-term fact, preference, or setting
   - episodic: Record of this specific interaction
   - semantic: General pattern or insight
4. How important is this? (0.0-1.0, where 1.0 is critical)
5. What tags would help categorize this?

Respond in this exact format:
REMEMBER: yes/no
CONTENT: [what to remember, or "none"]
TYPE: working/factual/episodic/semantic
IMPORTANCE: [0.0-1.0]
TAGS: [comma-separated tags, or "none"]
REASONING: [brief explanation]

Only remember information that is:
- Factual and verifiable
- Relevant for future interactions
- Not already common knowledge
- Has importance >= {}

Be selective - not everything needs to be remembered."#,
            message, context_str, self.config.min_importance
        )
    }

    /// Parse the LLM extraction response
    fn parse_extraction_response(&self, response: &str) -> GraphBitResult<ExtractionResult> {
        let mut should_remember = false;
        let mut content: Option<String> = None;
        let mut memory_type: Option<MemoryType> = None;
        let mut importance = 0.0;
        let mut tags = Vec::new();
        let mut reasoning: Option<String> = None;

        for line in response.lines() {
            let line = line.trim();
            
            if let Some(value) = line.strip_prefix("REMEMBER:") {
                should_remember = value.trim().to_lowercase() == "yes";
            } else if let Some(value) = line.strip_prefix("CONTENT:") {
                let content_str = value.trim();
                if content_str != "none" && !content_str.is_empty() {
                    content = Some(content_str.to_string());
                }
            } else if let Some(value) = line.strip_prefix("TYPE:") {
                memory_type = match value.trim().to_lowercase().as_str() {
                    "working" => Some(MemoryType::Working),
                    "factual" => Some(MemoryType::Factual),
                    "episodic" => Some(MemoryType::Episodic),
                    "semantic" => Some(MemoryType::Semantic),
                    _ => None,
                };
            } else if let Some(value) = line.strip_prefix("IMPORTANCE:") {
                if let Ok(imp) = value.trim().parse::<f32>() {
                    importance = imp.clamp(0.0, 1.0);
                }
            } else if let Some(value) = line.strip_prefix("TAGS:") {
                let tags_str = value.trim();
                if tags_str != "none" && !tags_str.is_empty() {
                    tags = tags_str
                        .split(',')
                        .map(|t| t.trim().to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                }
            } else if let Some(value) = line.strip_prefix("REASONING:") {
                let reasoning_str = value.trim();
                if !reasoning_str.is_empty() {
                    reasoning = Some(reasoning_str.to_string());
                }
            }
        }

        // Validate extraction
        if !should_remember {
            return Ok(ExtractionResult::skip());
        }

        if importance < self.config.min_importance {
            return Ok(ExtractionResult::skip());
        }

        if content.is_none() || memory_type.is_none() {
            return Ok(ExtractionResult::skip());
        }

        let mut result = ExtractionResult::remember(
            content.unwrap(),
            memory_type.unwrap(),
            importance,
            tags,
        );

        if let Some(r) = reasoning {
            result = result.with_reasoning(r);
        }

        Ok(result)
    }

    /// Extract multiple memories from a conversation
    pub async fn extract_batch(
        &mut self,
        messages: &[String],
        context: Option<&str>,
    ) -> GraphBitResult<Vec<ExtractionResult>> {
        let mut results = Vec::with_capacity(messages.len());

        for message in messages {
            let result = self.extract(message, context).await?;
            if result.should_remember {
                results.push(result);
            }
        }

        Ok(results)
    }

    /// Update extraction configuration
    pub fn update_config(&mut self, config: ExtractionConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &ExtractionConfig {
        &self.config
    }

    /// Reset message counter
    pub fn reset_counter(&mut self) {
        self.message_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_config() {
        let config = ExtractionConfig::new();
        assert!(config.enabled);
        assert_eq!(config.min_importance, 0.5);

        let conservative = ExtractionConfig::conservative();
        assert_eq!(conservative.min_importance, 0.7);

        let aggressive = ExtractionConfig::aggressive();
        assert_eq!(aggressive.min_importance, 0.3);
    }

    #[test]
    fn test_extraction_result() {
        let skip = ExtractionResult::skip();
        assert!(!skip.should_remember);

        let remember = ExtractionResult::remember(
            "Test content".to_string(),
            MemoryType::Factual,
            0.8,
            vec!["test".to_string()],
        );
        assert!(remember.should_remember);
        assert_eq!(remember.importance, 0.8);
    }
}

