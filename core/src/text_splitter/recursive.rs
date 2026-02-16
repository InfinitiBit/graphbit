//! Recursive text splitter with multiple separators.

use crate::errors::{GraphBitError, GraphBitResult};

use super::config::{SplitterStrategy, TextChunk, TextSplitterConfig};
use super::TextSplitterTrait;

/// Recursive text splitter that tries multiple separators
pub struct RecursiveSplitter {
    pub(super) config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    separators: Vec<String>,
}

impl RecursiveSplitter {
    /// Create a new recursive splitter with default separators
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        let default_separators = vec![
            "\n\n".to_string(),
            "\n".to_string(),
            ". ".to_string(),
            "! ".to_string(),
            "? ".to_string(),
            "; ".to_string(),
            ": ".to_string(),
            " - ".to_string(),
            " ".to_string(),
            "".to_string(),
        ];

        Self::with_separators(chunk_size, chunk_overlap, default_separators)
    }

    /// Create a recursive splitter with custom separators
    pub fn with_separators(
        chunk_size: usize,
        chunk_overlap: usize,
        separators: Vec<String>,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        if separators.is_empty() {
            return Err(GraphBitError::validation(
                "text_splitter",
                "At least one separator must be provided",
            ));
        }

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Recursive {
                chunk_size,
                chunk_overlap,
                separators: Some(separators.clone()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            separators,
        })
    }

    fn recursive_split(&self, text: &str, separators: &[String]) -> Vec<String> {
        if separators.is_empty() || text.len() <= self.chunk_size {
            return vec![text.to_string()];
        }

        let separator = &separators[0];
        let remaining_separators = &separators[1..];

        if separator.is_empty() {
            return self.split_by_characters(text);
        }

        let parts: Vec<&str> = text.split(separator).collect();

        let mut result = Vec::new();
        let mut current_chunk = String::new();

        for part in parts {
            let potential_chunk = if current_chunk.is_empty() {
                part.to_string()
            } else {
                format!("{current_chunk}{separator}{part}")
            };

            if potential_chunk.len() <= self.chunk_size {
                current_chunk = potential_chunk;
            } else {
                if !current_chunk.is_empty() {
                    result.push(current_chunk);
                    current_chunk = String::new();
                }

                if part.len() > self.chunk_size {
                    let sub_parts = self.recursive_split(part, remaining_separators);
                    result.extend(sub_parts);
                } else {
                    current_chunk = part.to_string();
                }
            }
        }

        if !current_chunk.is_empty() {
            result.push(current_chunk);
        }

        result
    }

    fn split_by_characters(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = (start + self.chunk_size).min(text.len());
            result.push(text[start..end].to_string());
            start = end;
        }

        result
    }
}

impl TextSplitterTrait for RecursiveSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let parts = self.recursive_split(text, &self.separators);
        let mut chunks = Vec::new();
        let mut position = 0;

        for (chunk_index, part) in parts.iter().enumerate() {
            let content = if self.config.trim_whitespace {
                part.trim().to_string()
            } else {
                part.clone()
            };

            if !content.is_empty() {
                if let Some(start_pos) = text[position..].find(&content) {
                    let actual_start = position + start_pos;
                    let actual_end = actual_start + content.len();

                    chunks.push(TextChunk::new(
                        content,
                        actual_start,
                        actual_end,
                        chunk_index,
                    ));

                    position = actual_end;
                }
            }
        }

        if self.chunk_overlap > 0 && chunks.len() > 1 {
            let mut overlapped_chunks = Vec::new();

            for i in 0..chunks.len() {
                let mut chunk_content = chunks[i].content.clone();

                if i > 0 {
                    let prev_chunk = &chunks[i - 1];
                    let overlap_start = prev_chunk.content.len().saturating_sub(self.chunk_overlap);
                    let overlap_text = &prev_chunk.content[overlap_start..];
                    chunk_content = format!("{overlap_text}{chunk_content}");
                }

                if i < chunks.len() - 1 {
                    let next_chunk = &chunks[i + 1];
                    let overlap_end = self.chunk_overlap.min(next_chunk.content.len());
                    let overlap_text = &next_chunk.content[..overlap_end];
                    chunk_content = format!("{chunk_content}{overlap_text}");
                }

                overlapped_chunks.push(TextChunk::new(
                    chunk_content,
                    chunks[i].start_index,
                    chunks[i].end_index,
                    i,
                ));
            }

            return Ok(overlapped_chunks);
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
        if self.separators.is_empty() {
            return Err(GraphBitError::validation(
                "text_splitter",
                "At least one separator must be provided",
            ));
        }
        Ok(())
    }
}
