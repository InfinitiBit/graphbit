//! Sentence-based text splitter.

use regex::Regex;

use crate::errors::{GraphBitError, GraphBitResult};

use super::config::{SplitterStrategy, TextChunk, TextSplitterConfig};
use super::TextSplitterTrait;

/// Sentence-based text splitter
pub struct SentenceSplitter {
    pub(super) config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    sentence_pattern: Regex,
}

impl SentenceSplitter {
    /// Create a new sentence splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        let default_endings = vec![
            r"[.!?]+[\s\n]+",
            r"[。！？]+[\s\n]*",
            r"[\n\r]+",
        ];

        Self::with_endings(chunk_size, chunk_overlap, default_endings)
    }

    /// Create a sentence splitter with custom sentence endings
    pub fn with_endings(
        chunk_size: usize,
        chunk_overlap: usize,
        endings: Vec<&str>,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        let pattern = format!("({})", endings.join("|"));
        let sentence_pattern = Regex::new(&pattern).map_err(|e| {
            GraphBitError::validation("text_splitter", format!("Invalid regex pattern: {e}"))
        })?;

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Sentence {
                chunk_size,
                chunk_overlap,
                sentence_endings: Some(endings.iter().map(ToString::to_string).collect()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            sentence_pattern,
        })
    }
}

impl TextSplitterTrait for SentenceSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let mut sentences = Vec::new();
        let mut last_end = 0;

        for mat in self.sentence_pattern.find_iter(text) {
            let sentence_end = mat.end();
            let sentence = &text[last_end..sentence_end];
            if !sentence.trim().is_empty() {
                sentences.push((sentence, last_end, sentence_end));
            }
            last_end = sentence_end;
        }

        if last_end < text.len() {
            let sentence = &text[last_end..];
            if !sentence.trim().is_empty() {
                sentences.push((sentence, last_end, text.len()));
            }
        }

        let mut chunks = Vec::new();
        let mut chunk_index = 0;
        let mut i = 0;

        while i < sentences.len() {
            let mut current_size = 0;
            let mut j = i;

            while j < sentences.len() && current_size < self.chunk_size {
                current_size += sentences[j].0.len();
                j += 1;
            }

            if j == i {
                j = i + 1;
            }

            let start_pos = sentences[i].1;
            let end_pos = sentences[j - 1].2;
            let content: String = sentences[i..j].iter().map(|(s, _, _)| *s).collect();

            let content = if self.config.trim_whitespace {
                content.trim().to_string()
            } else {
                content
            };

            if !content.is_empty() {
                let mut chunk = TextChunk::new(content, start_pos, end_pos, chunk_index);
                chunk.metadata.insert(
                    "sentence_count".to_string(),
                    serde_json::Value::Number((j - i).into()),
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            let next_i = if self.chunk_overlap > 0 && j < sentences.len() {
                j.saturating_sub(self.chunk_overlap)
            } else {
                j
            };

            i = next_i.max(i + 1);
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
