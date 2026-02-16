//! Token-based text splitter.

use regex::Regex;

use crate::errors::{GraphBitError, GraphBitResult};

use super::config::{SplitterStrategy, TextChunk, TextSplitterConfig};
use super::TextSplitterTrait;

/// Token-based text splitter
pub struct TokenSplitter {
    pub(super) config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    token_pattern: Regex,
}

impl TokenSplitter {
    /// Create a new token splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        Self::with_pattern(chunk_size, chunk_overlap, r"\b\w+\b|\s+|[^\w\s]+")
    }

    /// Create a token splitter with custom token pattern
    pub fn with_pattern(
        chunk_size: usize,
        chunk_overlap: usize,
        pattern: &str,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        if chunk_overlap >= chunk_size {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk overlap must be less than chunk size",
            ));
        }

        let token_pattern = Regex::new(pattern).map_err(|e| {
            GraphBitError::validation("text_splitter", format!("Invalid regex pattern: {e}"))
        })?;

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Token {
                chunk_size,
                chunk_overlap,
                token_pattern: Some(pattern.to_string()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            token_pattern,
        })
    }
}

impl TextSplitterTrait for TokenSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let tokens: Vec<&str> = self
            .token_pattern
            .find_iter(text)
            .map(|m| m.as_str())
            .collect();

        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut chunk_index = 0;
        let mut i = 0;

        while i < tokens.len() {
            let chunk_end = (i + self.chunk_size).min(tokens.len());
            let chunk_tokens = &tokens[i..chunk_end];

            let start_pos = tokens[i].as_ptr() as usize - text.as_ptr() as usize;
            let last_token = tokens[chunk_end - 1];
            let end_pos = last_token.as_ptr() as usize - text.as_ptr() as usize + last_token.len();

            let content = chunk_tokens.join("");
            let content = if self.config.trim_whitespace {
                content.trim().to_string()
            } else {
                content
            };

            if !content.is_empty() {
                let mut chunk = TextChunk::new(content, start_pos, end_pos, chunk_index);
                chunk.metadata.insert(
                    "token_count".to_string(),
                    serde_json::Value::Number(chunk_tokens.len().into()),
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            i = if self.chunk_overlap > 0 && chunk_end < tokens.len() {
                chunk_end.saturating_sub(self.chunk_overlap)
            } else {
                chunk_end
            };
        }

        Ok(chunks)
    }

    fn config(&self) -> &TextSplitterConfig {
        &self.config
    }

    fn validate_config(&self) -> GraphBitResult<()> {
        if self.chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }
        Ok(())
    }
}
