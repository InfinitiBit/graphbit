//! Character-based text splitter.

use crate::errors::{GraphBitError, GraphBitResult};

use super::config::{SplitterStrategy, TextChunk, TextSplitterConfig};
use super::TextSplitterTrait;

/// Character-based text splitter
pub struct CharacterSplitter {
    pub(super) config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
}

impl CharacterSplitter {
    /// Create a new character splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
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

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Character {
                chunk_size,
                chunk_overlap,
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
        })
    }
}

impl TextSplitterTrait for CharacterSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        let char_to_byte: Vec<(usize, usize)> = text
            .char_indices()
            .enumerate()
            .map(|(char_idx, (byte_idx, _))| (char_idx, byte_idx))
            .collect();

        let total_chars = text.chars().count();
        let mut char_to_byte_map = char_to_byte;
        char_to_byte_map.push((total_chars, text.len()));

        let mut char_start = 0;

        while char_start < total_chars {
            let mut char_end = (char_start + self.chunk_size).min(total_chars);

            let mut byte_start = char_to_byte_map[char_start].1;
            let mut byte_end = char_to_byte_map[char_end].1;

            if self.config.preserve_word_boundaries && char_end < total_chars {
                let chunk_text = &text[byte_start..byte_end];
                if let Some(last_space_pos) = chunk_text.rfind(char::is_whitespace) {
                    byte_end = byte_start
                        + last_space_pos
                        + chunk_text[last_space_pos..]
                            .chars()
                            .next()
                            .unwrap()
                            .len_utf8();

                    for i in (char_start..char_end).rev() {
                        if char_to_byte_map[i + 1].1 == byte_end {
                            char_end = i + 1;
                            break;
                        }
                    }
                }
            }

            let chunk_content = &text[byte_start..byte_end];
            let content = if self.config.trim_whitespace {
                chunk_content.trim()
            } else {
                chunk_content
            };

            if !content.is_empty() {
                if self.config.trim_whitespace {
                    let trim_start = chunk_content.len() - chunk_content.trim_start().len();
                    let trim_end = chunk_content.trim_end().len() - content.len();
                    byte_start += trim_start;
                    byte_end -= trim_end;
                }

                chunks.push(TextChunk::new(
                    content.to_string(),
                    byte_start,
                    byte_end,
                    chunk_index,
                ));
                chunk_index += 1;
            }

            let next_char_start = if self.chunk_overlap > 0 && char_end < total_chars {
                char_end.saturating_sub(self.chunk_overlap)
            } else {
                char_end
            };

            char_start = next_char_start.max(char_start + 1);
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
